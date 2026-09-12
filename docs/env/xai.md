# xAI Grok 환경 설정

xAI 직접 API의 인증 정보와 모델별 테스트 대상을 준비합니다. 키와 endpoint는 공통으로 한 번 설정하고, 테스트할 모델이나 릴리스마다 슬롯을 추가합니다.

## 콘솔에서 확인할 값

1. [xAI Console](https://console.x.ai/)에 로그인해 API를 사용할 계정·팀을 선택합니다. API 사용에 필요한 크레딧을 확인한 뒤 **API Keys**에서 키를 만들고 `XAI_API_KEY`에 넣습니다. [공식 시작 안내](https://docs.x.ai/developers/quickstart)
2. Console에서 사용할 수 있는 모델과 [공식 모델 목록](https://docs.x.ai/developers/models)을 대조하고 정확한 API model ID를 복사합니다. 공개 문서에 있는 모델이라도 해당 계정·키에서 사용할 수 있는지 따로 확인합니다.
3. 버전별 테스트에는 공식 모델 페이지에서 확인한 릴리스 ID를 사용합니다. xAI는 모델 기본 이름이나 `-latest` 이름이 바뀔 수 있는 alias이고, 날짜가 포함된 릴리스 ID는 특정 릴리스를 가리킬 수 있다고 안내합니다. 이름에 임의로 날짜를 붙이지 말고 공개된 정확한 ID를 복사하세요. [xAI 모델 alias와 릴리스](https://docs.x.ai/developers/models#model-aliases)

## 복사할 `.env` 블록

```dotenv
# 공통 인증과 endpoint
XAI_API_KEY=
XAI_BASE_URL=https://api.x.ai/v1

# 첫 번째 모델: 실제 사용 가능한 ID와 확인한 릴리스
XAI_MODEL_1_ID=
XAI_MODEL_1_VERSION=

# 두 번째 모델 또는 같은 모델의 다른 릴리스
XAI_MODEL_2_ID=
XAI_MODEL_2_VERSION=
```

| 설정 | 입력할 값 |
| --- | --- |
| `XAI_API_KEY` | 선택한 xAI 계정·팀에서 만든 API key |
| `XAI_BASE_URL` | `/v1`을 포함한 xAI API 주소 |
| `XAI_MODEL_1_ID` | 실제 요청의 `model`에 전달할 정확한 모델 ID |
| `XAI_MODEL_1_VERSION` | 공식 자료로 확인한 해당 모델의 릴리스 식별자 |

`_VERSION`은 테스트 대상의 릴리스를 기록하는 값입니다. 실제 호출 버전은 `_ID`에 넣는 모델 ID로 선택하므로 `_VERSION`만 바꿔 alias를 특정 버전으로 고정할 수는 없습니다. 공급자가 전체 릴리스 ID로만 버전을 구분한다면, 확인한 그 ID를 `_ID`와 `_VERSION`에 동일하게 적습니다. `/v1`은 API 경로 버전이며 모델 릴리스와 별개입니다.

두 모델은 각각 `_1_ID`와 `_2_ID`에 넣습니다. 같은 모델의 두 버전은 각 슬롯에 실제로 존재하는 서로 다른 릴리스 ID를 넣습니다. 더 필요하면 `XAI_MODEL_3_ID`, `XAI_MODEL_3_VERSION`처럼 양의 정수 번호를 늘립니다.

ID가 채워진 슬롯마다 독립 테스트 대상이 됩니다. 사용하지 않는 슬롯은 ID를 비워둡니다. 릴리스를 아직 확인하지 못했다면 준비 중에는 `_VERSION`을 비워두고 실제 테스트 전에 확인하세요. 별도 계정이나 endpoint를 사용할 때는 별도 `.env` 파일로 준비합니다.

이 문서는 모델별 실제 연결 테스트용 설정 규약입니다. `.env` 저장만으로 호출이 실행되지 않으며 코어 라이브러리가 파일을 직접 읽지 않습니다. 준비 및 실행 범위는 [공통 설정 안내](README.md)를 따릅니다.
