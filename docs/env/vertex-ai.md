# Google Cloud Vertex AI의 Gemini

[환경변수 안내](README.md) · [전체 예제](../../.env.example)

Google Cloud 프로젝트, 모델을 호출할 위치, Application Default Credentials(ADC)를 준비합니다. 여기서는 ADC 경로를 설정합니다. Wickle의 Vertex 어댑터와 실제 연결 검사 runner는 아직 완성되지 않았으며, 코어가 아래 `.env`를 자동으로 읽지는 않습니다.

```dotenv
GOOGLE_CLOUD_PROJECT=
GOOGLE_CLOUD_LOCATION=
VERTEX_API_VERSION=v1

VERTEX_MODEL_1_ID=
VERTEX_MODEL_1_VERSION=

# 특정 credential/federation 설정 파일을 선택할 때만 설정합니다.
# GOOGLE_APPLICATION_CREDENTIALS=
# Host에서 별도 quota project를 지정해야 할 때만 설정합니다.
# GOOGLE_CLOUD_QUOTA_PROJECT=
# 일반적인 location별 endpoint 대신 사용할 origin이 있을 때만 설정합니다.
# VERTEX_ENDPOINT=
```

1. Google Cloud Console에서 사용할 프로젝트를 선택합니다. **프로젝트 ID**를 `GOOGLE_CLOUD_PROJECT`에 복사하고, 결제 연결과 `aiplatform.googleapis.com` 활성화를 확인합니다. 호출 주체에는 `roles/aiplatform.user` 또는 필요한 추론 권한을 담은 별도 역할이 필요합니다. [프로젝트·권한 준비](https://docs.cloud.google.com/vertex-ai/generative-ai/docs/start/quickstart).
2. Console의 모델 카탈로그에서 Gemini 모델을 선택한 뒤 [모델 ID·release 목록](https://docs.cloud.google.com/vertex-ai/generative-ai/docs/learn/model-versions)과 [지원 위치](https://docs.cloud.google.com/vertex-ai/generative-ai/docs/learn/locations)를 확인합니다. 지원되는 리전 또는 `global`을 `GOOGLE_CLOUD_LOCATION`에 넣습니다. 같은 모델이라도 모든 위치에서 사용할 수 있다고 가정하지 않습니다.
3. 모델 요청에 사용할 정확한 Google 모델 ID를 `VERTEX_MODEL_1_ID`에 복사하고, 확인한 release 식별자를 `VERTEX_MODEL_1_VERSION`에 기록합니다. 표시 이름·프로젝트 ID·endpoint ID를 모델 ID 대신 넣지 않습니다. 별도 release 값이 없으면 공식적으로 고정 release를 식별한다고 확인한 전체 모델 ID를 사용할 수 있습니다. [모델 버전과 수명주기](https://docs.cloud.google.com/vertex-ai/generative-ai/docs/learn/model-versions).

CLI에서 접근 가능한 프로젝트를 조회하고 로컬 ADC를 준비할 수도 있습니다. API 활성화 명령의 프로젝트 ID는 실제 값으로 바꿉니다.

```sh
gcloud projects list --format='table(projectId,name)'
gcloud services enable aiplatform.googleapis.com --project '<프로젝트 ID>'
gcloud auth application-default login
```

`gcloud projects list`의 프로젝트 ID와 Console의 선택값이 같은지 확인합니다. `gcloud auth login`의 CLI 로그인과 애플리케이션이 사용하는 ADC 설정은 구분되므로, 로컬 라이브러리용으로는 `application-default login`을 사용합니다. [프로젝트 조회](https://docs.cloud.google.com/sdk/gcloud/reference/projects/list), [ADC 준비](https://docs.cloud.google.com/vertex-ai/generative-ai/docs/start/gcp-auth).

로컬 ADC 또는 실행 환경에 연결된 service account를 사용할 때는 `GOOGLE_APPLICATION_CREDENTIALS`를 설정하지 않습니다. 특정 credential/federation 파일을 의도적으로 사용할 경우에만 그 파일 경로를 넣고 파일은 저장소 밖에 보관합니다. ADC는 지정 파일, 로컬 ADC, 연결된 실행 환경의 identity 순으로 인증 정보를 찾습니다. [ADC 검색 순서](https://docs.cloud.google.com/docs/authentication/application-default-credentials).

별도 quota project가 필요하면 ADC의 quota project를 설정합니다. `GOOGLE_CLOUD_QUOTA_PROJECT`를 읽어 적용하는 부분은 Host 인증 라이브러리의 연결 책임이며, 환경변수 작성만으로 모든 SDK 설정이 바뀌는 것은 아닙니다. [quota project 설정](https://docs.cloud.google.com/docs/quotas/set-quota-project).

```sh
gcloud auth application-default set-quota-project '<quota 프로젝트 ID>'
```

같은 프로젝트·ADC·위치에서 다른 모델이나 버전도 검사하려면 다음 번호를 추가합니다.

```dotenv
VERTEX_MODEL_2_ID=
VERTEX_MODEL_2_VERSION=
```

번호는 양의 정수이며 ID를 채운 각 번호가 독립된 모델·버전 검사 대상입니다. 미확인 VERSION은 준비 중에는 비워 두고 실제 검사 전에 확인합니다. 미확인을 통과로 기록하지 않습니다. 프로젝트·인증 계정·리전·endpoint가 다르면 별도의 `.env` 파일을 사용합니다.

`VERTEX_API_VERSION=v1`은 API 계약 버전입니다. 모델 버전과는 별개이며, 일반 endpoint는 선택한 location에 맞춰 구성합니다. [공식 호출 예제](https://docs.cloud.google.com/vertex-ai/generative-ai/docs/start/quickstart). AI Studio 키를 준비한 경우에는 [Gemini API 가이드](gemini.md)의 경로를 사용합니다.
