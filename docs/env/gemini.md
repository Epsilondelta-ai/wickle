# Google AI Studio의 Gemini API

[환경변수 안내](README.md) · [전체 예제](../../.env.example)

Google AI Studio에서 발급한 Gemini API 키와 해당 키로 사용할 모델을 준비합니다. 아래는 Host와 모델별 검사를 위한 설정 규약이며, Wickle의 Gemini 어댑터와 실제 연결 검사 runner는 아직 완성되지 않았습니다.

```dotenv
GEMINI_API_KEY=
GEMINI_BASE_URL=https://generativelanguage.googleapis.com
GEMINI_API_VERSION=v1

GEMINI_MODEL_1_ID=
GEMINI_MODEL_1_VERSION=
```

1. [Google AI Studio API Keys](https://aistudio.google.com/apikey)에서 사용할 프로젝트를 선택하고 키를 생성합니다. 기존 프로젝트가 보이지 않으면 **Dashboard → Projects → Import projects**에서 가져옵니다. 키는 해당 Google Cloud 프로젝트의 사용량·결제 설정과 연결됩니다. [프로젝트와 키 준비](https://ai.google.dev/gemini-api/docs/api-key).
2. 새로 생성한 **auth key**를 `GEMINI_API_KEY`에 넣습니다. 공식 문서는 기존 Standard key의 전환을 안내하므로, 오래된 키를 재사용한다면 Key Type과 현재 제한 조건도 확인합니다. Google SDK는 `GOOGLE_API_KEY`가 함께 설정되면 이를 우선할 수 있어, Host는 선택한 `GEMINI_API_KEY`를 명시적으로 전달해야 합니다. [키 유형·환경변수 우선순위](https://ai.google.dev/gemini-api/docs/api-key).
3. AI Studio의 모델 선택 화면과 [Models API](https://ai.google.dev/api/models)에서 사용할 모델의 ID·버전·지원 작업을 확인합니다. `name`이 `models/…` 형식이면 `models/` 뒤의 정확한 모델 식별자를 `GEMINI_MODEL_1_ID`에 넣습니다. `supportedGenerationMethods`에 필요한 생성 방식이 있는지도 확인합니다.
4. 공급자가 공개한 버전 정보를 `GEMINI_MODEL_1_VERSION`에 기록합니다. Models API의 `version` 값은 공급자 버전 metadata이며, 이 값만으로 ID가 고정 snapshot이라는 의미가 되지는 않습니다. 선택한 ID의 release·alias 의미를 함께 확인합니다. [모델 metadata](https://ai.google.dev/api/models#Model).

키를 셸 환경변수에도 준비했다면 공식 모델 목록을 아래처럼 조회할 수 있습니다. 이 명령의 `v1beta`는 목록 조회 경로이며, `.env`의 생성 API 버전을 자동으로 변경하지 않습니다. `.env` 저장만으로 셸 변수가 설정되지는 않습니다.

```sh
curl --fail-with-body --silent --show-error \
  'https://generativelanguage.googleapis.com/v1beta/models' \
  -H "x-goog-api-key: $GEMINI_API_KEY"
```

`nextPageToken`이 있으면 다음 요청에 `pageToken`을 넣어 나머지 모델도 확인합니다. 목록 조회는 Wickle의 실제 생성·Tool 호출 검사를 대신하지 않습니다. [모델 목록 API](https://ai.google.dev/api/models#method:-models.list).

같은 프로젝트·키·API 경로에서 두 번째 모델이나 버전을 검사하려면 다음 번호를 추가합니다.

```dotenv
GEMINI_MODEL_2_ID=
GEMINI_MODEL_2_VERSION=
```

번호는 양의 정수이며 ID를 채운 각 번호가 독립된 모델·버전 검사 대상입니다. VERSION은 준비 중에는 비워 둘 수 있지만 실제 검사 전에 확인해야 하며, 미확인을 통과로 기록하지 않습니다. 프로젝트·키·endpoint 또는 필요한 API 버전이 다르면 별도의 `.env` 파일을 사용합니다.

`GEMINI_API_VERSION=v1`은 안정 API 경로를 뜻합니다. 선택한 기능이 `v1beta`를 요구한다면 그 기능을 지원하는 어댑터와 함께 설정해야 합니다. 모델 release와 API 버전은 별개입니다. [API 버전](https://ai.google.dev/gemini-api/docs/api-versions). 이 경로의 인증은 AI Studio 키이며, ADC로 준비하는 [Vertex AI 경로](vertex-ai.md)는 별도 설정입니다.
