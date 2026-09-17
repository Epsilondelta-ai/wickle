# 최종 구현 평가

[목차](README.md) · [36장](36-release.md) · [실행 흐름](walkthrough.md)

모든 명령은 36장까지 완성한 실습 workspace 루트에서 실행한다. Wickle 0.1.0과 동등한 구현을 목표로 한다. byte 동일성은 누락을 찾는 보조 검사이고, 동작 검증과 설명 능력이 최종 기준이다.

## 1. 자동 검증

```sh
cargo fmt --all -- --check
cargo check --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo test --workspace --doc --locked
cargo doc --workspace --no-deps --locked
python3 "$COURSE/lab.py" compare 36 --work .
```

`compare` 차이에는 공백·학습용 주석도 포함될 수 있다. 참조에 없는 추가 파일은 별도 확인한다. 동일한 구현을 작성하는 경로라면 모든 참조 파일이 일치해야 한다. 다른 구현을 택했다면 어떤 계약을 같은 방식으로 만족하는지 설명한다.

최소 지원 Rust를 직접 확인하려면 1.85.0 toolchain을 설치한 뒤 다음을 별도로 실행한다. 이 결과를 실행하지 않고 통과로 기록하지 않는다.

```sh
cargo +1.85.0 check -p wickle --lib --no-default-features --locked
cargo +1.85.0 test --workspace --locked
```

## 2. 직접 소비자 실행과 패키징

[Host 실습](walkthrough.md)의 text/fallback, tool loop, resume 세 프로그램을 직접 실행한다. 그 다음 release 파일만 사용하는 package consumer 검사를 실행한다.

```sh
python3 scripts/check-package.py --allow-dirty
```

학습 작업이 아직 commit되지 않아도 이 명령은 검사한다. 내부에서 `.crate`를 추출하고 저장소 밖의 Rust Host들을 빌드한다. SQLite reopen, 승인 재개, 별도 프로세스 복구, MCP 로컬 서버, 일곱 모델 adapter의 로컬 fixture와 외부 이벤트 소비를 검사한다. stdout의 각 consumer 요약과 최종 종료 코드 0을 확인한다. dependency 다운로드 후에는 유료 모델 API 호출을 하지 않는다.

## 3. 행동 평가표

| 평가 시나리오 | 관찰할 증거 | 실패 조건 |
| --- | --- | --- |
| 동일 request 재전송 | 같은 Run ID와 동일 model/tool 호출 수 | 새 업무 실행이 추가됨 |
| current grant 철회 | deny와 외부 callback 0회 | 과거 허용만으로 실행 |
| 모델이 hidden workspace를 공급 | schema 거절, executor 0회 | 값이 같으니 허용 |
| 승인 중 resolver 값 변경 | 원래 BoundToolInput 유지 | 승인 대상이 새 값으로 바뀜 |
| 마지막 예산 슬롯 경쟁 | reservation 한 개만 commit | 한도 밖 두 호출 |
| 잘린 model stream | 오류, 부분 text만 보존 | partial tool을 실행 |
| Applied + schema 실패 | effect/receipt는 유지 | NotApplied로 바꾸고 재시도 |
| 쓰기 후 crash | reconcile 또는 Unknown wait | 근거 없는 write 재전송 |
| lease 교체 후 늦은 worker | 오래된 generation 거절 | stale commit 성공 |
| 빈 context 조회 | 이전 slot 자료 제거 | 과거 Ready 자료 재사용 |
| observer 실패 | 별도 보고, 기존 outcome 유지 | 업무를 다시 실행 |
| 압축 후보 무효 | 원본 보존, 후보 미채택 | 필수 요청·pair 손실 |
| verifier 미완료 | verified 성공 아님 | timeout을 품질 통과로 처리 |
| 외부 memory Accepted | 아직 미적용 상태 | 새 Run이 적용 완료라고 믿음 |
| DB 재개방 | 저장된 동일 결과·이벤트 | in-memory 값에만 의존 |

각 행을 통과시킨 테스트 이름이나 consumer 출력과 자신의 설명을 남긴다. 표의 문구가 코드에 등장하는지만 검사하는 테스트는 이 성질을 증명하지 못한다.

## 4. 구술·설계 평가

다음 질문을 코드 위치와 tradeoff를 포함해 답한다.

1. core가 adapter를 호출하는데 Cargo 의존 방향은 왜 반대인가?
2. Arc, Mutex, CAS, lease, fencing은 각각 어떤 문제를 해결하는가?
3. StateStore의 commit을 일반 CRUD 여러 호출로 분해하면 무엇을 잃는가?
4. 원본 transcript와 ModelRequest를 왜 같은 구조체로 두지 않는가?
5. Command/Strategy/Observer/Factory/Facade가 실제로 나타나는 곳은 어디인가?
6. 이 구현을 Event Sourcing, Saga, exactly-once 시스템이라고 단정하면 왜 부정확한가?
7. SQLite 전체 checkpoint 방식의 장점과 성장 비용은 무엇이며 언제 다른 store가 필요한가?
8. 모델 버전과 deployment version, contract test와 live evidence는 왜 구분하는가?
9. context 요약·verifier 결과가 구조적으로 유효한 것과 사실인 것은 어떻게 다른가?
10. 모델을 더 똑똑하게 바꿔도 Host 정책과 복구 원장이 필요한 이유는 무엇인가?

설명은 [아키텍처 장](02b-architecture.md)의 내용을 자신의 요청 예제로 재구성한다. 정답 문장을 암기하기보다 하나의 경계를 제거했을 때의 반례를 만든다.

## 5. 제출물

완성 코드, 자동 검사 결과, 직접 consumer 실행 결과, 위 시나리오에 대한 관찰 표, 설계 질문에 대한 답변을 함께 보관한다. 통과하지 못한 검사와 미실행 항목은 명시한다. 외부 API smoke를 추가하면 별도 계정·모델·target·날짜·기능의 근거를 기록하며 다른 제공자로 일반화하지 않는다.
