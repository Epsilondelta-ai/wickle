# Azure Foundry OpenAI GPT 환경 설정

Azure에서는 리소스의 인증·endpoint를 공통으로 설정하고, 각 모델 슬롯에 실제 배포 대상을 지정합니다. 같은 모델의 두 버전을 테스트할 때도 배포별 실제 버전을 확인해야 합니다.

## 콘솔에서 확인할 값

1. [Azure Portal](https://portal.azure.com/)에서 사용할 구독과 OpenAI 리소스를 엽니다. **Keys and endpoint**에서 endpoint와 `KEY1` 또는 `KEY2`를 확인합니다. endpoint의 리소스 주소를 `AZURE_OPENAI_ENDPOINT`에, 선택한 키를 `AZURE_OPENAI_API_KEY`에 넣습니다. [키와 endpoint 확인 방법](https://learn.microsoft.com/en-us/connectors/azureopenai/#get-your-credentials)
2. [Foundry](https://ai.azure.com/)의 해당 리소스 배포 목록에서 사용할 배포를 엽니다. 배포 이름과 **Details**의 모델 이름·현재 모델 버전을 각각 복사합니다. 같은 배포 이름으로 모델 버전이 변경될 수 있으므로 자동 업그레이드 설정도 확인합니다. [배포 버전 확인](https://learn.microsoft.com/en-us/azure/foundry/foundry-models/concepts/model-versions)

| 설정 | 확인할 값 |
| --- | --- |
| `AZURE_OPENAI_MODEL_1_ID` | 배포의 underlying model name. 관리 API로 확인할 때는 `properties.model.name` |
| `AZURE_OPENAI_MODEL_1_VERSION` | 배포의 현재 model version. 관리 API의 `properties.model.version` |
| `AZURE_OPENAI_MODEL_1_DEPLOYMENT` | 실제 호출할 deployment name. 관리 API 배포 객체의 `name` |

추론 요청의 `model`에는 배포 이름이 전달됩니다. 모델 이름·버전은 배포가 가리키는 대상을 확인하기 위한 별도 값입니다. `.env`의 `_VERSION`을 바꾸는 것만으로 Azure 배포가 변경되지는 않습니다. [모델과 배포 관리](https://learn.microsoft.com/en-us/azure/foundry/openai/how-to/working-with-models)

## 복사할 `.env` 블록

```dotenv
# 공통 리소스 주소. 예: https://<resource>.openai.azure.com
# 여기에 /openai/v1/ 또는 deployment 경로를 붙이지 않습니다.
AZURE_OPENAI_ENDPOINT=
AZURE_OPENAI_AUTH_MODE=api_key
AZURE_OPENAI_API_KEY=

# 공통 추론 API 계약. v1에서는 dated api-version을 비워둡니다.
AZURE_OPENAI_API_MODE=v1
AZURE_OPENAI_API_VERSION=

# 첫 번째 배포
AZURE_OPENAI_MODEL_1_ID=
AZURE_OPENAI_MODEL_1_VERSION=
AZURE_OPENAI_MODEL_1_DEPLOYMENT=

# 두 번째 배포: 다른 모델 또는 같은 모델의 다른 버전
AZURE_OPENAI_MODEL_2_ID=
AZURE_OPENAI_MODEL_2_VERSION=
AZURE_OPENAI_MODEL_2_DEPLOYMENT=
```

`v1` 경로에서는 리소스 주소 뒤에 `/openai/v1/`을 붙여 호출하며, 날짜 형태의 `api-version`은 필요하지 않습니다. 별도로 구현된 dated API 경로를 사용할 때만 `AZURE_OPENAI_API_MODE=dated`와 해당 경로가 요구하는 정확한 `AZURE_OPENAI_API_VERSION`을 함께 설정합니다. 리소스·배포를 관리하는 Azure 관리 API 버전을 여기에 넣지 않습니다. [Azure 추론 API 버전](https://learn.microsoft.com/en-us/azure/foundry/openai/api-version-lifecycle?view=foundry-classic)

## Microsoft Entra ID를 사용하는 경우

`AZURE_OPENAI_AUTH_MODE=entra`로 바꾸고 API key는 비워둡니다. 로컬에서는 `az login`으로 사용할 계정에 로그인하고, 해당 리소스의 모델 호출 권한을 확인합니다. Azure에서 실행할 때는 관리 ID를 사용할 수도 있습니다. 서비스 주체를 선택한 경우에만 아래 tenant·client·secret을 설정합니다. 토큰 발급·갱신은 Host의 인증 제공자가 처리합니다. [Entra 인증 설정](https://learn.microsoft.com/en-us/azure/foundry/foundry-models/how-to/configure-entra-id)

```dotenv
AZURE_OPENAI_AUTH_MODE=entra
AZURE_OPENAI_API_KEY=

# 서비스 주체를 사용하는 경우에만 주석을 해제하고 입력
# AZURE_TENANT_ID=
# AZURE_CLIENT_ID=
# AZURE_CLIENT_SECRET=

# 선택한 리소스의 인증 안내에 맞는 scope
# Foundry v1 공식 예제의 값:
# AZURE_OPENAI_ENTRA_SCOPE=https://ai.azure.com/.default
```

같은 모델의 버전 두 개를 비교하려면 각 버전을 대상으로 하는 **서로 다른 배포**를 준비하고 `_1_DEPLOYMENT`, `_2_DEPLOYMENT`에 넣습니다. 두 배포의 모델 이름이 같아도 됩니다. 추가 배포는 `_3_ID`, `_3_VERSION`, `_3_DEPLOYMENT`처럼 번호를 늘립니다.

ID가 채워진 각 슬롯은 독립 테스트 대상입니다. 배포 이름도 함께 입력하고, 버전을 모르면 준비 중에만 비워둔 뒤 테스트 전에 Details에서 확인하세요. 다른 리소스·계정이나 다른 공통 API 설정을 사용할 때는 별도 `.env` 파일로 나눕니다.

이 문서는 모델별 실제 연결 테스트용 설정 규약이며 `.env` 저장만으로 호출이 실행되지 않습니다. 코어 라이브러리는 `.env`를 직접 읽지 않습니다. [공통 설정 안내](README.md)에서 준비 및 실행 범위를 확인하세요.
