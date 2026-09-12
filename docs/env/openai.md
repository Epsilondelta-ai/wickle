# OpenAI GPT 환경 설정

OpenAI 직접 API의 인증 정보와 모델별 테스트 대상을 준비합니다. 키와 endpoint는 한 번 설정하고, 모델은 번호가 붙은 슬롯에 추가합니다. 아래 블록의 빈칸을 채워 `.env`에 넣으세요.

## 콘솔에서 확인할 값

1. [OpenAI Platform](https://platform.openai.com/)에서 사용할 조직과 프로젝트를 선택합니다. 해당 프로젝트의 **API Keys**에서 애플리케이션용 키를 만들고 `OPENAI_API_KEY`에 넣습니다. API 사용을 위한 프로젝트의 결제·사용 한도도 확인합니다. [공식 시작 안내](https://developers.openai.com/api/docs/quickstart)
2. [공식 모델 목록](https://developers.openai.com/api/docs/models)에서 테스트할 모델의 상세 페이지를 엽니다. 계정의 Playground 모델 선택 목록 또는 해당 키로 조회한 [Models API 목록](https://developers.openai.com/api/reference/resources/models/methods/list)과 대조해 접근 가능한 모델 ID를 확인합니다. 공개 모델 목록만으로 해당 프로젝트의 사용 권한이 확인되는 것은 아닙니다.
3. 버전을 고정하려면 상세 페이지의 **Snapshots**에 있는 정확한 ID를 확인합니다. 실제 요청 대상을 선택하는 값은 `OPENAI_MODEL_1_ID`입니다. `_VERSION`은 확인한 릴리스를 기록하는 값이며, 이것만 바꿔 alias의 호출 버전이 고정되지는 않습니다. OpenAI가 별도 버전 필드 없이 snapshot ID로 릴리스를 구분하는 경우에는 그 전체 ID를 두 칸에 동일하게 적습니다. [모델별 상세 페이지 진입](https://developers.openai.com/api/docs/models), [버전 고정과 호환성](https://developers.openai.com/api/reference/overview#backwards-compatibility)

## 복사할 `.env` 블록

```dotenv
# 공통 인증과 endpoint
OPENAI_API_KEY=
OPENAI_BASE_URL=https://api.openai.com/v1

# 첫 번째 모델: 계정에서 사용할 수 있는 정확한 ID와 확인한 릴리스
OPENAI_MODEL_1_ID=
OPENAI_MODEL_1_VERSION=

# 두 번째 모델 또는 같은 모델의 다른 릴리스
OPENAI_MODEL_2_ID=
OPENAI_MODEL_2_VERSION=

# 해당 키에 명시적인 조직·프로젝트 선택이 필요한 경우에만 설정
# OPENAI_ORG_ID=
# OPENAI_PROJECT_ID=
```

`OPENAI_BASE_URL`의 `/v1`은 API 경로 버전입니다. 모델 릴리스와는 별개입니다. 선택적으로 사용하는 조직·프로젝트 ID는 Platform 설정에서 확인합니다. [OpenAI 인증과 프로젝트 선택](https://developers.openai.com/api/reference/overview#authentication)

두 모델을 비교하려면 `_1_ID`와 `_2_ID`에 각 모델의 실제 ID를 넣습니다. 같은 모델의 두 버전을 비교하려면 각 슬롯에 서로 다른 **실제 snapshot ID**를 넣습니다. 더 추가할 때는 `OPENAI_MODEL_3_ID`, `OPENAI_MODEL_3_VERSION`처럼 양의 정수 번호를 늘립니다.

ID가 채워진 슬롯마다 독립적인 테스트 대상이 됩니다. 사용하지 않는 슬롯은 ID를 비워둡니다. 버전이 아직 확인되지 않았다면 준비 중에는 `_VERSION`을 비워두고 실제 테스트 전에 확인하세요. 날짜나 버전을 임의로 만들지 않습니다. 다른 계정·프로젝트·endpoint 조합은 별도 `.env` 파일로 준비합니다.

이 문서는 모델별 실제 연결 테스트용 설정 규약입니다. `.env`를 저장하는 것만으로 호출이 실행되지 않으며, 코어 라이브러리가 파일을 직접 읽지 않습니다. 준비 및 실행 범위는 [공통 설정 안내](README.md)를 따릅니다.
