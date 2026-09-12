# 모델 테스트용 `.env` 설정

접근 가능한 프로바이더의 모델만 준비하면 됩니다. 인증 정보는 프로바이더별로 한 번 설정하고, 테스트할 모델·버전은 번호를 붙여 각각 추가합니다. 에이전트 검증에는 텍스트 응답과 Tool calling을 지원하는 모델을 준비합니다. 공급자를 켜는 별도 환경변수는 사용하지 않습니다.

**이 `.env`와 모델 번호 변수는 Wickle의 실제 연결 테스트 전용입니다. 라이브러리 사용자의 런타임 설정 규약이 아닙니다.**

이 문서는 테스트 연결 설정을 준비하기 위한 규약입니다. 실제 어댑터와 모델별 테스트는 구현 중이며, 모델 등록·버전 확인과 실제 호출 결과를 별도로 기록합니다.

## 프로바이더별 가이드

| 사용할 경로 | 설정 가이드 | 모델 변수 prefix |
| --- | --- | --- |
| OpenAI GPT | [OpenAI](openai.md) | `OPENAI_MODEL_<N>_` |
| Azure Foundry OpenAI GPT | [Azure OpenAI](azure-openai.md) | `AZURE_OPENAI_MODEL_<N>_` |
| Anthropic Claude 직접 API | [Claude](anthropic.md) | `ANTHROPIC_MODEL_<N>_` |
| AWS Bedrock Claude | [Bedrock](bedrock.md) | `BEDROCK_MODEL_<N>_` |
| Google AI Studio / Gemini API | [Gemini API](gemini.md) | `GEMINI_MODEL_<N>_` |
| Google Cloud Vertex AI Gemini | [Vertex AI](vertex-ai.md) | `VERTEX_MODEL_<N>_` |
| xAI Grok | [Grok](xai.md) | `XAI_MODEL_<N>_` |

## 파일 만들기

Wickle 저장소에서 [`.env.example`](../../.env.example)을 참고해 `.env`를 만듭니다. 기존 파일이 있으면 필요한 항목만 편집합니다.

```sh
if [ ! -e .env ]; then
  (umask 077; cp .env.example .env)
fi
```

`.env`와 `.env.*`는 Git에서 제외됩니다. 실제 키나 credential JSON을 문서·커밋에 넣지 않습니다. 테스트 실행기가 `.env`를 읽어 모델별 어댑터를 구성합니다. Core 라이브러리는 파일을 읽거나 이 환경변수에 의존하지 않고 구성된 어댑터를 전달받습니다. `.env`와 `.env.example`은 배포용 `.crate` 패키지에도 포함하지 않습니다. 실제 서비스의 설정 방식은 Host 애플리케이션이 결정합니다.

## 모델을 여러 개 추가하기

예를 들어 OpenAI 계정 하나에서 두 모델을 테스트하려면 다음처럼 작성합니다. 빈칸에는 콘솔이나 공식 모델 문서에서 확인한 실제 값을 넣습니다.

```dotenv
OPENAI_API_KEY=
OPENAI_BASE_URL=https://api.openai.com/v1

OPENAI_MODEL_1_ID=
OPENAI_MODEL_1_VERSION=

OPENAI_MODEL_2_ID=
OPENAI_MODEL_2_VERSION=
```

- 번호는 프로바이더별로 `1`, `2`, `3`처럼 붙입니다. 같은 모델의 서로 다른 버전도 번호를 나누어 작성합니다.
- 모델 ID가 입력된 각 항목이 독립 테스트 대상입니다. 사용할 수 없는 프로바이더와 아직 준비하지 않은 모델 항목은 비워 둡니다.
- `_VERSION`은 모델 release 정보입니다. API 버전·deployment 이름·SDK 버전과 구분합니다. 모르는 값은 준비 단계에서 비워 두고, 실제 테스트 전에 확인합니다. 임의 날짜나 버전을 만들어 넣지 않습니다.
- 공급자가 전체 모델 ID로 고정 release를 식별한다면, 공식적으로 확인한 그 ID를 `_VERSION`에도 사용할 수 있습니다. `latest` 같은 alias를 고정 버전으로 간주하지 않습니다.
- Azure deployment와 Bedrock inference profile처럼 모델마다 달라지는 호출 대상은 해당 번호의 항목으로 작성합니다. 각 가이드에 필요한 변수를 제시합니다.
- 다른 계정·endpoint·region·project를 사용해야 한다면 `.env.azure-dev`, `.env.bedrock-seoul`처럼 별도 파일로 나누어 같은 규약을 사용합니다. 공통 설정이 서로 다른 계정을 섞지 않습니다.
- 중복된 변수 이름을 추가하지 않습니다. 모델을 추가할 때는 해당 모델 블록의 번호를 바꾸고, 공통 API 키는 그대로 사용합니다.

이전 예제의 `OPENAI_MODEL_ID` / `OPENAI_MODEL_VERSION`처럼 번호 없는 모델 변수는 각각 `OPENAI_MODEL_1_ID` / `OPENAI_MODEL_1_VERSION` 형태로 옮깁니다. 다른 프로바이더도 같은 규칙입니다. 인증 변수 이름은 유지됩니다.

## 준비할 정보

| 정보 | 필요한 경우 |
| --- | --- |
| API 키 또는 클라우드 인증 방식 | 각 프로바이더의 호출 권한 |
| 모델 ID | 모든 모델 항목 |
| 정확한 모델 release/version | 버전별 지원 및 고정 검증 |
| Deployment 이름 | Azure OpenAI의 각 모델 항목 |
| Inference profile ID/ARN | Bedrock 호출에 필요한 모델 항목 |
| Region·project·endpoint | 해당 클라우드 리소스 또는 별도 연결 설정 |
| API version/mode | 선택한 제공 경로의 HTTP 계약 |

파일을 준비한 뒤에는 파일 경로와 작성한 프로바이더·모델 항목을 알려주면 됩니다. 키 값 자체를 메시지로 보낼 필요는 없습니다. 검증 결과는 `(제공 경로, 모델 ID, 모델 버전, API/배포 설정)`별로 기록하며, 설정 누락·호출 불가·미확인 버전을 성공으로 처리하지 않습니다.
