# Wickle 0.1.0을 직접 만드는 Rust·에이전트 강의

컴퓨터공학 전공생이 Rust와 에이전트를 처음 배우며 실제 Wickle 0.1.0을 구현하는 한국어 교재다. **[00장부터 시작한다](00-start.md).** Rust 주 교재는 [The Rust Programming Language](https://doc.rust-lang.org/book/)이며 [Book 대응표](rust-book-map.md)에 선행 읽기를 정리했다.

38개 강의 문서(00–36장과 02B 아키텍처), 34개 단계별 전체 구현 문서, 34개 누적 정답 패치로 구성된다. 각 구현 장에는 개념, 작성 순서, 실제 코드, 아키텍처와 패턴의 장단점, 행동 테스트, 결함 실험, 해설이 있다. 코드에는 생략된 구현이 없으며 패치만으로 Git history 없이 최종 release를 복원할 수 있다. 이해 여부는 패치 적용 성공이 아니라 장별 실험과 최종 평가로 확인한다.

기준: `v0.1.0` / `117b141ad2505a26c5175405520003a5ea0ab32d`. 이 release의 엔진·13개 crate·실행 가능한 Host 예제를 학습한다. 아직 없는 SaaS 관리 화면이나 운영 서비스를 구현했다고 간주하지 않는다. 기본 실습에 유료 모델 호출은 없다.

## 준비 강의

| 순서 | 강의 | 전체 구현 |
| --- | --- | --- |
| 00 | [학습 방법과 개발 환경](00-start.md) | 본문 실습 |
| 01 | [Rust 기초: 소유권·타입·오류·모듈](01-rust.md) | 본문 실습 |
| 02 | [Future·Stream·Port와 실행 소유권](02-async.md) | 본문 실습 |
| 02b | [소프트웨어 아키텍처와 디자인 패턴](02b-architecture.md) | 본문 실습 |

## 기반 계약과 저장

| 순서 | 강의 | 전체 구현 |
| --- | --- | --- |
| 03 | [Cargo workspace와 첫 라이브러리](03-workspace.md) | [코드·테스트](implementation/03-workspace.md) |
| 04 | [Rust 타입으로 실행 계약 표현하기](04-contracts.md) | [코드·테스트](implementation/04-contracts.md) |
| 05 | [Scope와 현재 권한 검사](05-policy.md) | [코드·테스트](implementation/05-policy.md) |
| 06 | [원자적 저장소와 실행 소유권](06-state.md) | [코드·테스트](implementation/06-state.md) |
| 07 | [예산 예약·시간·취소](07-budget.md) | [코드·테스트](implementation/07-budget.md) |
| 08 | [모델 스트림과 물리 호출](08-model.md) | [코드·테스트](implementation/08-model.md) |
| 09 | [모델 입력과 시스템 입력 분리](09-schema.md) | [코드·테스트](implementation/09-schema.md) |
| 10 | [프롬프트 고정과 문맥 투영](10-context.md) | [코드·테스트](implementation/10-context.md) |
| 11 | [도구 실행 인자를 확정하고 저장하기](11-binding.md) | [코드·테스트](implementation/11-binding.md) |
| 12 | [모델 카탈로그·버전·옵션](12-catalog.md) | [코드·테스트](implementation/12-catalog.md) |
| 13 | [SQLite에 실행 상태 영속화하기](13-sqlite.md) | [코드·테스트](implementation/13-sqlite.md) |
| 14 | [정확한 모델 선택과 제한된 fallback](14-routing.md) | [코드·테스트](implementation/14-routing.md) |

## 실행·확장·복구

| 순서 | 강의 | 전체 구현 |
| --- | --- | --- |
| 15 | [Agent와 RunHandle로 첫 실행 완성](15-agent.md) | [코드·테스트](implementation/15-agent.md) |
| 16 | [직렬 Tool Loop와 외부 효과 원장](16-tools.md) | [코드·테스트](implementation/16-tools.md) |
| 17 | [승인·입력·외부 결과를 기다리고 재개](17-resume.md) | [코드·테스트](implementation/17-resume.md) |
| 18 | [Hook 변환과 결과 관찰](18-hooks.md) | [코드·테스트](implementation/18-hooks.md) |
| 19 | [어댑터 조립과 자원 수명](19-adapters.md) | [코드·테스트](implementation/19-adapters.md) |
| 20 | [검색과 메모리를 ContextSource로 연결](20-sources.md) | [코드·테스트](implementation/20-sources.md) |
| 21 | [Skill·Artifact·Evidence 구현](21-skills.md) | [코드·테스트](implementation/21-skills.md) |
| 22 | [문맥 선택·미리보기·압축](22-compaction.md) | [코드·테스트](implementation/22-compaction.md) |
| 23 | [출력 검증과 제한된 보완 루프](23-verification.md) | [코드·테스트](implementation/23-verification.md) |
| 24 | [프로세스 장애와 불확실한 효과 복구](24-recovery.md) | [코드·테스트](implementation/24-recovery.md) |

## 모델 제공자와 MCP

| 순서 | 강의 | 전체 구현 |
| --- | --- | --- |
| 25 | [OpenAI Responses 어댑터](25-openai.md) | [코드·테스트](implementation/25-openai.md) |
| 26 | [Azure 배포·인증과 공통 Responses codec](26-azure.md) | [코드·테스트](implementation/26-azure.md) |
| 27 | [Anthropic Messages와 재현 가능한 복구 검사](27-anthropic.md) | [코드·테스트](implementation/27-anthropic.md) |
| 28 | [Bedrock 인증·바이너리 이벤트 프레임](28-bedrock.md) | [코드·테스트](implementation/28-bedrock.md) |
| 29 | [Gemini 스트림·도구 순서·서명](29-gemini.md) | [코드·테스트](implementation/29-gemini.md) |
| 30 | [Vertex AI 프로젝트·지역·토큰 경계](30-vertex.md) | [코드·테스트](implementation/30-vertex.md) |
| 31 | [xAI와 일곱 제공 경로 통합](31-xai.md) | [코드·테스트](implementation/31-xai.md) |
| 32 | [MCP stdio 도구와 프로세스 관리](32-mcp.md) | [코드·테스트](implementation/32-mcp.md) |

## 통합과 완성

| 순서 | 강의 | 전체 구현 |
| --- | --- | --- |
| 33 | [영속 이벤트와 외부 메모리 갱신](33-events.md) | [코드·테스트](implementation/33-events.md) |
| 34 | [모델 버전별 지원 근거 검증](34-evidence.md) | [코드·테스트](implementation/34-evidence.md) |
| 35 | [독립 Host 통합과 저장소 캐시 검증](35-integration.md) | [코드·테스트](implementation/35-integration.md) |
| 36 | [0.1.0 완성·패키징·최종 평가](36-release.md) | [코드·테스트](implementation/36-release.md) |

## 함께 사용하는 자료

- [Rust Book 선행 읽기와 Wickle 적용표](rust-book-map.md)
- [전체 파일·공개 API 찾아보기](code-atlas.md)
- [하나의 요청으로 전체 흐름 추적하기](walkthrough.md)
- [독립 Host 전체 실행 코드](host-example.md)
- [기술 용어집](glossary.md)
- [최종 구현 평가와 실행 명령](assessment.md)
- [교재 검증 결과와 한계](VALIDATION.md)
- [Checkpoint 도구](lab.py), [버전·검사 manifest](course.json)

## 막히지 않고 이어가기

이전 장의 결과에서 다음 장을 구현한다. 특정 단계의 참조 환경이 필요하면 00장의 `COURSE` 설정 후 `python3 "$COURSE/lab.py" snapshot 15 --dest ../wickle-answer-15`처럼 **새 폴더**에 복원한다. 교재의 정답은 기존 실습 폴더를 덮어쓰지 않는다.

전체 소스의 MIT 라이선스는 [프로젝트 LICENSE](../../../LICENSE)를 따른다. Rust Book 본문을 이 교재에 복제하지 않으며 링크로 연결한다. 모델/API 설명은 0.1.0 코드의 계약을 설명하며 현재 서비스 판매·가용성의 보증이 아니다.
