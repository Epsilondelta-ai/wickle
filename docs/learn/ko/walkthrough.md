# 요청 한 건으로 전체 엔진 추적하기

[목차](README.md) · [아키텍처](02b-architecture.md) · [직접 실행할 Host 전체 코드](host-example.md)

## 먼저 실제 Host를 실행한다

15장 이후 해당 checkpoint의 `tests/support/agent_consumer.rs`로 실행할 수 있다. 아래 명령은 36장까지 완료한 `wickle-lab` 폴더에서 수행한다. 교재의 실습 폴더명을 바꿨다면 path dependency의 `../wickle-lab` 부분도 실제 이름으로 바꾼다. `wickle-host`가 없는 상태에서 시작한다.

```sh
mkdir ../wickle-host
mkdir ../wickle-host/src
cp tests/support/agent_consumer.rs ../wickle-host/src/main.rs
cat > ../wickle-host/Cargo.toml <<'TOML'
[package]
name = "wickle-course-host"
version = "0.1.0"
edition = "2024"
publish = false

[workspace]

[dependencies]
wickle = { path = "../wickle-lab/crates/wickle" }
wickle-model-router = { path = "../wickle-lab/crates/wickle-model-router" }
wickle-state-sqlite = { path = "../wickle-lab/crates/wickle-state-sqlite" }
futures-util = { version = "=0.3.34", default-features = false, features = ["std", "async-await"] }
serde_json = "=1.0.151"
tokio = { version = "=1.53.1", features = ["rt", "macros", "sync", "time"] }
TOML
cp Cargo.lock ../wickle-host/Cargo.lock
cargo +1.98.1 run --manifest-path ../wickle-host/Cargo.toml
```

처음 Host dependency graph를 만들 때 복사한 lockfile을 Cargo가 Host 구성에 맞게 조정한다. 성공 후에는 `cargo +1.98.1 run --manifest-path ../wickle-host/Cargo.toml --locked`로 재현한다. 실제 output은 `agent consumer: pure construction, ... SQLite reopen`으로 끝나는 한 줄이다. 내부 assert들이 fallback, 호출 수, 저장 결과, 재요청을 확인한다. 두 모델은 synthetic이고 실제 DB 파일은 임시 디렉터리에 만든다.

이 smoke Host의 ExamplePolicy와 metadata inspector는 학습 fixture다. 외부 모델 metadata를 그대로 echo하는 inspector를 운영 코드로 복사하지 않는다. ExampleClock도 accounting을 결정적으로 만들기 위한 고정 시계이므로 timeout 정확성 검사를 이 예제로 주장하지 않는다.

## 조립 코드를 읽는 순서

1. `Catalog::resolve`: Profile이 참조한 구성요소의 승인된 metadata를 제공한다.
2. `routing_snapshot`: primary와 fallback, capabilities와 evidence를 구성한다. fixture 근거이지 vendor 사실 선언이 아니다.
3. `ExamplePolicy`: exact route의 연결을 허용한다. 실제 Host라면 현재 권한 DB를 확인할 수 있다.
4. `ExampleInspector`: 관찰 metadata를 만드는 테스트 대역이다.
5. `ExampleModel`: 첫 번째 모델은 RateLimited, 두 번째는 완료 text를 반환한다.
6. `Estimate`: 테스트 추정치이며 provider 보고 usage와 다르다.
7. `main`: concrete 객체를 만들고 AgentBindings로 주입한 뒤 start, events, outcome, replay, SQLite reopen 순으로 확인한다.

## 관찰할 숫자

| 시점 | 첫 모델 누적 호출 | 두 번째 모델 누적 호출 | 저장된 결과 |
| --- | --- | --- | --- |
| create_agent 직후 | 0 | 0 | 아직 Run 없음 |
| 최초 요청 완료 | 1 | 1 | 성공, model_calls=2, recovery_attempts=1 |
| 동일 request 재요청 | 1 | 1 | 같은 Run과 같은 outcome |
| DB reopen 후 | 1 | 1 | 같은 outcome, active_run_id 없음 |

events를 중간에 drop해도 실행이 끝나는 것을 확인한다. 마지막의 durable `run.finished`가 완료 근거다. 모델의 마지막 token이나 HTTP 완료와 저장 완료를 구별한다.

## 이제 도구·승인·복구를 포함한 하나의 사례를 종이에 추적한다

다음은 실제 enum 계약을 이해하기 위한 예시 identity이며 특정 테스트의 revision 숫자를 그대로 재현한 로그는 아니다. “보고서 R을 팀 workspace W에 게시해 줘”라는 요청을 사용한다.

| 단계 | 무엇을 고정·저장하는가 | 외부 행동 | 재시도/중단 후 처리 |
| --- | --- | --- | --- |
| admission | request ID, scope W, profile, prompt, system inputs | 아직 publish 없음 | 같은 digest면 원래 Run |
| model step | route, 입력, reservation, attempt A | publish 제안 생성 | 완료 응답은 재사용 |
| plan | 도구 call C와 모델 인자 | 아직 publish 없음 | 새 모델에게 계획을 다시 묻지 않음 |
| bind | C의 최종 report=R, workspace=W | resolver는 읽기만 | 기존 bound 값을 재사용 |
| approval wait | C의 정확한 target와 wait ID | 실행 없음 | reviewer가 바뀌어도 R/W 고정 |
| resume | command ID와 acceptance | 현재 policy 재확인 | 동일 command 소비 한 번 |
| dispatch | C의 attempt T와 idempotency key | 실제 게시 | 진입 뒤 crash이면 unknown 가능 |
| reconcile | T의 receipt와 확인된 effect | 게시 결과 조회 | 게시를 다시 보내지 않음 |
| settlement | effect, observation, paired message, event | 없음 | 이미 settle한 C 재실행 없음 |
| next model | 관찰 가능한 결과 projection | 사용자 설명 생성 | receipt와 hidden input은 제외 |
| finish | 검증된 outcome와 run.finished | 이후 observer | observer 실패가 결과를 뒤집지 않음 |

이 표에서 각 외부 행동 바로 앞의 저장·권한 경계를 지우면 어떤 장애가 발생하는지 설명해 본다. 이 과제가 상태 기계를 이해하는 가장 직접적인 방법이다.

## 실제 도구·승인 Host로 바꾸어 실행한다

방금 만든 Host의 main.rs만 바꾸면 같은 dependency로 두 consumer를 실행할 수 있다. 기존 파일을 보관하려면 이름을 바꾸어 복사한다. 이 Host는 학습자가 새로 만든 폴더이며 제품 소스는 수정하지 않는다.

```sh
cp tests/support/tool_loop_consumer.rs ../wickle-host/src/main.rs
cargo +1.98.1 run --manifest-path ../wickle-host/Cargo.toml --locked
cp tests/support/resume_consumer.rs ../wickle-host/src/main.rs
cargo +1.98.1 run --manifest-path ../wickle-host/Cargo.toml --locked
```

첫 프로그램은 `tool loop consumer:`로, 두 번째는 `resume consumer:`로 시작하는 검증 요약을 출력한다. 세부 조건은 각각 16장과 17장의 전체 구현 문서에서 읽는다. 프로세스를 실제 종료하는 recovery consumer와 MCP child, 전체 package consumer는 [최종 평가](assessment.md)의 명령으로 검사한다.
