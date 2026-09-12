#!/usr/bin/env python3
"""Verify the core dependency boundary and consume an extracted Cargo package."""

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
    "futures-util", "jsonschema", "serde", "serde_json", "sha2", "thiserror",
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
        consumer = base / "consumer"
        (consumer / "src").mkdir(parents=True)
        (consumer / "Cargo.toml").write_text(
            '[package]\nname = "wickle-package-consumer"\nversion = "0.0.0"\n'
            'edition = "2024"\npublish = false\n\n[workspace]\n\n'
            f'[dependencies]\nwickle = {{ path = "../{package_name}" }}\n'
            f'serde_json = "{versions["serde_json"]}"\n'
            f'tokio = {{ version = "{versions["tokio"]}", features = ["rt", "macros"] }}\n',
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
        subprocess.run(["cargo", "generate-lockfile", "--offline"],
                       cwd=consumer, env=env, check=True)
        resolved = metadata(consumer)
        for package in resolved["packages"]:
            if package["source"] is None:
                manifest = Path(package["manifest_path"]).resolve()
                if not manifest.is_relative_to(base):
                    raise RuntimeError(f"Consumer depends on an external path: {manifest}")
        subprocess.run(["cargo", "run", "--locked", "--offline", "--bin", "wickle-package-consumer"],
                       cwd=consumer, env=env, check=True)
        for example in examples:
            subprocess.run(["cargo", "run", "--locked", "--offline", "--bin", example.stem],
                           cwd=consumer, env=env, check=True)
    print("Independent package consumer: passed (profile validation and restore)", flush=True)


if __name__ == "__main__":
    main()
