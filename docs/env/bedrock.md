# AWS Bedrock의 Claude

[환경변수 안내](README.md) · [전체 예제](../../.env.example)

AWS 계정의 인증 정보, 호출을 시작할 리전, 해당 리전에서 사용할 Claude 모델을 준비합니다. 이 경로는 AWS SDK의 credential provider chain을 사용합니다. 아래는 설정 준비용이며 Wickle의 Bedrock 어댑터와 실제 연결 검사 runner는 아직 완성되지 않았습니다.

```dotenv
AWS_REGION=

BEDROCK_MODEL_1_ID=
BEDROCK_MODEL_1_VERSION=
# 모델 호출에 inference profile을 사용할 때만 설정합니다.
# BEDROCK_MODEL_1_INFERENCE_PROFILE_ID=

# 이름 있는 AWS profile을 사용할 때만 설정합니다.
# AWS_PROFILE=
# 임시 access key를 직접 사용할 때만 세 값을 함께 설정합니다.
# AWS_ACCESS_KEY_ID=
# AWS_SECRET_ACCESS_KEY=
# AWS_SESSION_TOKEN=
# SDK의 기본 리전별 endpoint를 바꿔야 할 때만 설정합니다.
# BEDROCK_ENDPOINT=
```

1. AWS Console에서 계정과 리전을 선택한 뒤 **Amazon Bedrock → Model catalog**에서 Claude 모델의 ID·지원 리전·호출 방식을 확인합니다. 선택한 호출 리전을 `AWS_REGION`에 넣습니다. [지원 모델](https://docs.aws.amazon.com/bedrock/latest/userguide/models-supported.html).
2. 계정의 모델 접근 조건과 호출 권한을 확인합니다. Bedrock Runtime의 Claude는 최초 사용 정보 제출 등 모델별 접근 준비가 필요할 수 있습니다. 모델 목록 조회 성공만으로 추론 권한까지 확인된 것은 아닙니다. [모델 접근 설정](https://docs.aws.amazon.com/bedrock/latest/userguide/model-access.html).
3. 인증 방식을 준비합니다. 로컬 IAM Identity Center를 사용한다면 아래처럼 profile을 만들고 로그인합니다. `bedrock-dev`는 직접 정하는 profile 이름이며, 로그인 후 `.env`에 `AWS_PROFILE=bedrock-dev`를 넣습니다. [AWS CLI SSO 설정](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-sso.html).

```sh
aws configure sso --profile bedrock-dev
aws sso login --profile bedrock-dev
```

기존 shared profile이나 AWS 실행 환경에 연결된 role도 사용할 수 있습니다. profile·role을 쓸 때는 access key 변수를 주석으로 둡니다. 환경변수의 access key가 profile보다 먼저 선택되므로 셸에 남아 있는 다른 계정의 키도 확인해야 합니다. 임시 키를 직접 사용한다면 access key·secret key·session token 세 값을 함께 준비합니다. [Rust SDK 인증 정보 검색 순서](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/credproviders.html).

모델 목록은 CLI로도 확인할 수 있습니다. `<선택한 리전>`과 profile 이름을 실제 값으로 바꿉니다. role이나 기본 profile을 사용한다면 `--profile` 옵션을 생략합니다.

```sh
aws bedrock list-foundation-models \
  --by-provider Anthropic \
  --region '<선택한 리전>' \
  --profile bedrock-dev \
  --output json

aws bedrock list-inference-profiles \
  --region '<선택한 리전>' \
  --profile bedrock-dev \
  --output json
```

첫 목록의 foundation model ID를 `BEDROCK_MODEL_1_ID`에 복사합니다. Console과 공식 모델 문서에서 확인한 release를 `BEDROCK_MODEL_1_VERSION`에 기록합니다. 공식적으로 고정 release를 식별하는 전체 model ID를 버전 식별자로 사용할 수 있지만, 임의의 날짜나 suffix를 만들지는 않습니다. [foundation model 목록](https://docs.aws.amazon.com/cli/latest/reference/bedrock/list-foundation-models.html), [제공 경로별 모델 ID](https://platform.claude.com/docs/en/about-claude/models/model-ids-and-versions).

Inference profile로 호출한다면 그 ID 또는 ARN을 같은 번호의 `BEDROCK_MODEL_1_INFERENCE_PROFILE_ID`에 넣고, 아래 명령으로 실제 연결된 모델과 리전을 확인합니다. `BEDROCK_MODEL_1_ID`에는 기반 foundation model을 유지합니다. 호출 시 profile이 `modelId` 대상이 되며, `AWS_REGION`은 요청의 출발 리전입니다. [profile 조회](https://docs.aws.amazon.com/cli/latest/reference/bedrock/get-inference-profile.html), [profile로 추론하기](https://docs.aws.amazon.com/bedrock/latest/userguide/inference-profiles-use.html).

```sh
aws bedrock get-inference-profile \
  --inference-profile-identifier '<확인한 profile ID 또는 ARN>' \
  --region '<선택한 리전>' \
  --profile bedrock-dev \
  --output json
```

같은 계정·인증·출발 리전에서 다른 모델이나 버전을 검사하려면 다음 번호를 추가합니다.

```dotenv
BEDROCK_MODEL_2_ID=
BEDROCK_MODEL_2_VERSION=
# BEDROCK_MODEL_2_INFERENCE_PROFILE_ID=
```

번호는 양의 정수이며 ID를 채운 각 번호가 독립된 검사 대상입니다. VERSION은 준비 중 미확인이면 비워 두되 실제 검사 전에 확인하고, 미확인을 통과로 기록하지 않습니다. 계정·role/profile·출발 리전·endpoint가 다르면 별도의 `.env` 파일을 사용합니다. 이 설정에는 직접 Anthropic API의 키나 `ANTHROPIC_API_VERSION`을 넣지 않습니다.
