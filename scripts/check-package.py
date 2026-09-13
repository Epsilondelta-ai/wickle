#!/usr/bin/env python3
"""Verify the core dependency boundary and consume extracted Cargo packages."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile


ROOT = Path(__file__).resolve().parent.parent
# Changes to core dependencies require an explicit boundary review.
CORE_DEPENDENCIES = {
    "futures-util", "getrandom", "jsonschema", "serde", "serde_json", "sha2", "thiserror",
    "tokio", "tokio-util",
}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--allow-dirty", action="store_true")
    args = parser.parse_args()
    toolchain = subprocess.check_output(
        ["rustup", "show", "active-toolchain"], cwd=ROOT, text=True
    ).split()[0]
    env = {**os.environ, "RUSTUP_TOOLCHAIN": toolchain}

    def metadata(directory):
        return json.loads(subprocess.check_output(
            ["cargo", "metadata", "--format-version", "1", "--locked"],
            cwd=directory, env=env, text=True,
        ))

    workspace = metadata(ROOT)
    core = next(p for p in workspace["packages"] if p["name"] == "wickle")
    catalog = next(p for p in workspace["packages"] if p["name"] == "wickle-model-router")
    sqlite = next(p for p in workspace["packages"] if p["name"] == "wickle-state-sqlite")
    adapters = next(p for p in workspace["packages"] if p["name"] == "wickle-adapter-runtime")
    openai = next(p for p in workspace["packages"] if p["name"] == "wickle-model-openai")
    responses = next(p for p in workspace["packages"] if p["name"] == "wickle-model-responses")
    azure = next(p for p in workspace["packages"] if p["name"] == "wickle-model-azure-openai")
    anthropic = next(p for p in workspace["packages"] if p["name"] == "wickle-model-anthropic")
    libraries = [core, catalog, sqlite, adapters, responses, openai, azure, anthropic]
    versions = {d["name"]: d["req"] for d in core["dependencies"] if d["kind"] is None}
    for dep in core["dependencies"]:
        if dep["kind"] != "dev":
            if dep["name"] not in CORE_DEPENDENCIES or dep.get("path"):
                raise RuntimeError(f"Unapproved core dependency: {dep['name']}")
    print("Core dependency boundary: passed", flush=True)

    command = ["cargo", "package", "-p", "wickle", "--locked"]
    if args.allow_dirty:
        command.append("--allow-dirty")
    subprocess.run(command, cwd=ROOT, env=env, check=True)
    package_name = f"wickle-{core['version']}"
    archive = Path(workspace["target_directory"]) / "package" / f"{package_name}.crate"
    print(f"Package SHA-256: {hashlib.sha256(archive.read_bytes()).hexdigest()}", flush=True)

    with tempfile.TemporaryDirectory(prefix="wickle-consumer-") as temporary:
        base = Path(temporary).resolve()
        if base.is_relative_to(ROOT):
            raise RuntimeError("The consumer must be outside the source repository")
        # The archive is produced by cargo package above, not supplied externally.
        subprocess.run(["tar", "-xzf", str(archive), "-C", str(base)], check=True)
        # Cargo 1.85 resolves unpublished path dependencies through the registry
        # when packaging. Stage the sources so the temporary patch and its lock
        # update cannot alter the checkout. The consumer verifies the archive.
        staged = base / "staged"
        staged.mkdir()
        for filename in ["Cargo.toml", "Cargo.lock"]:
            shutil.copyfile(ROOT / filename, staged / filename)
        for package in libraries:
            manifest = Path(package["manifest_path"])
            destination = staged / manifest.parent.relative_to(ROOT)
            destination.mkdir(parents=True)
            shutil.copyfile(manifest, destination / "Cargo.toml")
            shutil.copytree(manifest.parent / "src", destination / "src")
        package_paths = {core["name"]: base / package_name}
        for package in [catalog, sqlite, adapters, responses, openai, azure, anthropic]:
            patches = [argument for name, path in package_paths.items()
                       for argument in ["--config", f'patch.crates-io.{name}.path={json.dumps(str(path))}']]
            subprocess.run(
                ["cargo", "package", "-p", package["name"], "--allow-dirty",
                 "--offline", "--no-verify", *patches],
                cwd=staged, env={**env, "CARGO_TARGET_DIR": str(staged / "target")}, check=True,
            )
            name = f"{package['name']}-{package['version']}"
            packaged = staged / "target/package" / f"{name}.crate"
            print(f"{package['name']} package SHA-256: {hashlib.sha256(packaged.read_bytes()).hexdigest()}", flush=True)
            subprocess.run(["tar", "-xzf", str(packaged), "-C", str(base)], check=True)
            package_paths[package["name"]] = base / name
        consumer = base / "consumer"
        (consumer / "src").mkdir(parents=True)
        library_dependencies = ''.join(
            f'{name} = {{ path = "../{path.name}" }}\n'
            for name, path in package_paths.items()
        )
        (consumer / "Cargo.toml").write_text(
            '[package]\nname = "wickle-package-consumer"\nversion = "0.0.0"\n'
            'edition = "2024"\npublish = false\n\n[workspace]\n\n'
            f'[dependencies]\n{library_dependencies}'
            f'serde_json = "{versions["serde_json"]}"\n'
            f'futures-util = {{ version = "{versions["futures-util"]}", default-features = false, features = ["std", "async-await"] }}\n'
            f'tokio = {{ version = "{versions["tokio"]}", features = ["rt", "macros", "net", "io-util"] }}\n'
            f'\n[patch.crates-io]\n{library_dependencies}',
            encoding="utf-8",
        )
        shutil.copyfile(ROOT / "tests/support/consumer.rs", consumer / "src/main.rs")
        examples = sorted((ROOT / "tests/support").glob("*_consumer.rs"))
        if examples:
            (consumer / "src/bin").mkdir()
            for example in examples:
                shutil.copyfile(example, consumer / "src/bin" / example.name)
        # Keep consumer builds independent of the checkout and its build cache.
        env["CARGO_TARGET_DIR"] = str(base / "target")
        # This throwaway consumer does not reuse incremental compiler state.
        env["CARGO_INCREMENTAL"] = "0"
        # Preserve the verified transitive versions when adding the consumer.
        # A fresh lockfile would select newer compatible entries in the local cache.
        shutil.copyfile(ROOT / "Cargo.lock", consumer / "Cargo.lock")
        subprocess.run(["cargo", "metadata", "--format-version", "1", "--offline"],
                       cwd=consumer, env=env, stdout=subprocess.DEVNULL, check=True)
        resolved = metadata(consumer)
        allowed_registry = {(p["name"], p["version"], p["source"])
                            for p in workspace["packages"] if p["source"] is not None}
        for package in resolved["packages"]:
            if package["name"] in package_paths:
                expected = package_paths[package["name"]] / "Cargo.toml"
                if Path(package["manifest_path"]).resolve() != expected:
                    raise RuntimeError(f"Consumer substituted a package: {package['name']}")
            if package["source"] is None:
                manifest = Path(package["manifest_path"]).resolve()
                if not manifest.is_relative_to(base):
                    raise RuntimeError(f"Consumer depends on an external path: {manifest}")
            elif (package["name"], package["version"], package["source"]) not in allowed_registry:
                raise RuntimeError(f"Consumer resolved an unpinned dependency: {package['name']} {package['version']}")
        subprocess.run(["cargo", "run", "--locked", "--offline", "--bin", "wickle-package-consumer"],
                       cwd=consumer, env=env, check=True)
        for example in examples:
            subprocess.run(["cargo", "run", "--locked", "--offline", "--bin", example.stem],
                           cwd=consumer, env=env, check=True)
    print("Independent package consumers: passed (core, model catalog, SQLite store, adapter runtime, Responses codec, OpenAI, Azure and Anthropic adapters)", flush=True)


if __name__ == "__main__":
    main()
