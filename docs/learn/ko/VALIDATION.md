# 교재 검증 기록

[목차](README.md) · [학습자 최종 평가](assessment.md)

검증일: 2026-09-17. 기준: `v0.1.0` / `117b141ad2505a26c5175405520003a5ea0ab32d`. 환경: macOS arm64, Rust 1.98.1, Python 3.11. 이 기록은 교재 제작 시 실행 결과이며 학습자가 작성한 코드의 성공을 대신하지 않는다.

## 자동 검사

- 34개 checkpoint를 빈 Git 실습 폴더에 순서대로 적용했다. 각 단계의 파일 내용을 독립적으로 읽은 해당 Git commit과 비교하여 일치함을 확인했다.
- 최종 313개 release 파일이 일치한다. 여기에는 Rust 파일 226개와 manifest·lockfile·문서·배포 파일이 포함된다.
- 모든 단계의 지정 테스트 명령이 종료 코드 0을 반환했다. 03장은 빈 라이브러리 기반이라 행동 테스트가 아직 없으며 이후 단계부터 실제 기능을 검사한다.
- 최종 checkpoint에서 `cargo test --workspace --locked`를 실행하여 실패 0을 확인했다. 기본 실행에서 ignored로 표시되는 두 worker fixture는 부모 프로세스 테스트가 필요한 설정과 함께 실행하는 하위 프로세스용이다.
- 최종 저장소의 `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo doc --workspace --no-deps --locked`가 통과했다.
- helper는 신규 snapshot 복원, 동일 파일 비교, 기존 폴더 덮어쓰기 거절, 변경 파일 검출, 손상 patch를 쓰기 전에 거절하는 동작을 확인했다.
- Rust Book 링크 46개가 HTTP 200을 반환했다. 교재 내부 상대 경로와 코드 fence 짝도 모두 유효함을 확인했다.

## 직접 실행

- 소유권 예제: `request: report`, `shared owners: 2` 출력.
- 데이터 계약 예제: 빈 식별자 거절과 Unknown의 재시도 금지 확인.
- async Port 예제: `answer: report`와 handle drop 뒤 worker 완료 확인.
- 독립 text Host: fallback, 예산, 저장 outcome, 이벤트 재연결, 동일 요청 재사용, SQLite reopen 확인.
- 독립 tool Host: 두 직렬 도구, 고정 시스템 입력, 모델 공개 인자 분리, 추가 호출 없는 replay 확인.
- 독립 resume Host: 새로운 Host/검토자로 저장된 wait 재개, 고정 입력과 한 번의 효과, command replay 확인.

## 장별 검사 명령

아래 모든 명령은 해당 단계에서 통과했다. 최종 소스에서 이전 단계의 테스트만 실행한 것으로 대체하지 않았다.

| 장 | 검사 |
| --- | --- |
| [03](03-workspace.md) | `cargo test --workspace --locked` |
| [04](04-contracts.md) | `cargo test -p wickle --test contracts --locked` |
| [05](05-policy.md) | `cargo test -p wickle --test policy --locked` |
| [06](06-state.md) | `cargo test -p wickle --test state --locked` |
| [07](07-budget.md) | `cargo test -p wickle --test budget --locked` |
| [08](08-model.md) | `cargo test -p wickle --test model_protocol --locked`<br>`cargo test -p wickle --test model_execution --locked` |
| [09](09-schema.md) | `cargo test -p wickle --test tool_schema --locked` |
| [10](10-context.md) | `cargo test -p wickle --test context_projection --locked` |
| [11](11-binding.md) | `cargo test -p wickle --test input_binding --locked` |
| [12](12-catalog.md) | `cargo test -p wickle-model-router --test catalog --locked` |
| [13](13-sqlite.md) | `cargo test -p wickle-state-sqlite --test state_store --locked` |
| [14](14-routing.md) | `cargo test -p wickle-model-router --test routed_execution --locked` |
| [15](15-agent.md) | `cargo test -p wickle --test agent_runtime --locked` |
| [16](16-tools.md) | `cargo test -p wickle --test tool_execution --locked` |
| [17](17-resume.md) | `cargo test -p wickle --test agent_resume --locked` |
| [18](18-hooks.md) | `cargo test -p wickle --test agent_hooks --locked` |
| [19](19-adapters.md) | `cargo test -p wickle-adapter-runtime --test runtime_lifecycle --locked` |
| [20](20-sources.md) | `cargo test -p wickle-adapter-runtime --test context_sources --locked`<br>`cargo test -p wickle --test context_sources --locked` |
| [21](21-skills.md) | `cargo test -p wickle --test skills --locked`<br>`cargo test -p wickle --test artifacts --locked` |
| [22](22-compaction.md) | `cargo test -p wickle --test agent_tool_loop --locked` |
| [23](23-verification.md) | `cargo test -p wickle --test verification --locked` |
| [24](24-recovery.md) | `cargo test -p wickle-state-sqlite --test agent_recovery --locked`<br>`cargo test -p wickle --test agent_recovery --locked` |
| [25](25-openai.md) | `cargo test -p wickle-model-openai --test responses --locked` |
| [26](26-azure.md) | `cargo test -p wickle-model-azure-openai --test responses --locked` |
| [27](27-anthropic.md) | `cargo test -p wickle-model-anthropic --test messages --locked` |
| [28](28-bedrock.md) | `cargo test -p wickle-model-bedrock --test bedrock --locked` |
| [29](29-gemini.md) | `cargo test -p wickle-model-gemini --test generate --locked` |
| [30](30-vertex.md) | `cargo test -p wickle-model-vertex --test vertex --locked` |
| [31](31-xai.md) | `cargo test -p wickle-model-xai --test responses --locked` |
| [32](32-mcp.md) | `cargo test -p wickle-mcp --test stdio --locked` |
| [33](33-events.md) | `cargo test -p wickle-state-sqlite --test host_contract --locked` |
| [34](34-evidence.md) | `cargo test -p wickle-model-router --test catalog --locked` |
| [35](35-integration.md) | `cargo test -p wickle-state-sqlite --test state_store --locked` |
| [36](36-release.md) | `cargo test --workspace --locked` |

## 검증 범위의 한계

유료 모델 API 호출은 이 교재 검증에 포함하지 않았다. provider 검사는 local fixture이며 실제 계정 가용성이나 모델 답변 품질을 증명하지 않는다. 새 Linux/Windows 실행, Rust 1.85 재실행, 하드웨어 전원 차단 실험도 이 제작 과정에서 수행하지 않았다. 초심자에게 실제 강의를 진행한 학습 효과 평가는 별도로 필요하다.

중간 검사에서 Azure 명령의 crate 이름 오기를 발견하여 수정한 뒤 해당 checkpoint에서 재실행했다. package 검사 첫 시도는 기능 assertion이 아니라 링크 단계의 디스크 공간 부족으로 중단되었다. 누적된 제작용 빌드 캐시를 정리하고 개발 debug/incremental 산출물을 줄여 재검증했으며 최종 결과는 아래와 같다.

## 최종 패키징 재검사: 통과

```sh
CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_DEV_INCREMENTAL=false python3 scripts/check-package.py --allow-dirty
```

종료 코드 0. 13개 라이브러리의 추출 package 소비자와, 동일 core 소스를 사용하는 두 독립 업무 workspace가 모두 통과했다. 마지막 출력은 다음과 같다.

```text
Independent package and business consumers: passed (13 libraries; two separate business workspaces using identical core sources)
```

이 설정은 빌드 산출물 용량을 줄이며 소스나 dependency 버전을 바꾸지 않는다. 별도로 교재에 포함된 `lab.py snapshot 36`만 실행하여 최종 313개 파일이 동일하게 복원됨도 확인했다.
