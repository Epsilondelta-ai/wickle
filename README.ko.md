# Wickle

[English](README.md) | **한국어** | [日本語](README.ja.md) | [简体中文](README.zh-CN.md) | [Español](README.es.md) | [Français](README.fr.md) | [Deutsch](README.de.md) | [Русский](README.ru.md)

**엡실론델타가 만드는 확장 가능한 에이전트 엔진.**

<p align="center">
  <img src="assets/mascot/wickle.png" alt="쳇바퀴를 달리는 귀여운 고슴도치, Wickle 마스코트" width="420" />
</p>

Wickle은 엡실론델타가 Rust로 개발하는 에이전트 엔진입니다. 모델의 판단과 도구 호출을 반복하는 실행 루프를 애플리케이션에서 사용하는 라이브러리로 제공하는 것을 목표로 합니다.

Agent Profile로 에이전트의 동작을 구성하고, 어댑터를 통해 다양한 모델과 도구를 연결하는 구조로 설계하고 있습니다.

**개발 상태:** 모델·도구의 순차 호출, 시스템 입력 분리, 결과 저장, 이벤트 재조회, 취소와 승인·입력·외부 효과 확인 대기의 재개를 지원합니다. 같은 재개 명령은 중복 소비하지 않으며, 외부 효과는 Host의 검증을 거쳐 확정합니다. Lifecycle Hook으로 제한된 문맥·인자 변환과 저장 완료 후 관찰을 지원합니다. 어댑터의 Tool·Hook·ContextSource export를 고정된 조립 정보와 구간별 인스턴스로 연결하며, 실행 구간이 끝나면 자원을 해제합니다. [어댑터 런타임](docs/adapters.md). 실행 소유권과 저장된 입력을 확인하고 외부 효과를 조회하여 중단된 실행을 복구할 수 있습니다. [실행 복구](docs/recovery.md). [에이전트 실행](docs/agents.md) · [데이터 계약](docs/contracts.md).

읽기 전용 [ContextSource](docs/context-sources.md)는 Run·모델 단계별 조회, 자료 묶음 저장, 재사용 시 현재 접근 권한 검사를 지원합니다.

[Skills](docs/skills.md)는 등록된 도구를 통해 고정 버전의 지침 전체를 로드합니다. [Artifacts](docs/artifacts.md)는 scope가 적용된 원문, 제한된 preview와 원천 근거를 보존합니다.

[문맥 선택·압축](docs/context-compaction.md)은 원본 대화를 보존하면서 제한된 preview와 검증된 요약을 적용합니다. 모델 기반 압축도 같은 Run 예산을 사용합니다.

[출력 검증](docs/verification.md)은 JSON 스키마, 버전이 고정된 검증 기준, 한도 내 보완과 고정된 후보의 승인을 지원합니다. 모델 기반 검증도 같은 Run 예산을 사용합니다.

[모델 제공자 가이드](docs/model-providers.md)에서 OpenAI, Azure OpenAI, Anthropic, AWS Bedrock, Gemini API, Vertex AI, xAI 어댑터와 공통 연결 방법을 확인할 수 있습니다. 각 제공자의 인증, API 버전과 동작 차이는 별도로 처리합니다.

[MCP 도구 연결](docs/mcp.md)은 검토한 도구를 stdio로 연결하는 방법을 설명합니다.
