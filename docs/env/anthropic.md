# Anthropic Claude 직접 API

[환경변수 안내](README.md) · [전체 예제](../../.env.example)

Claude Console에서 발급한 API 키와 직접 API의 모델 ID를 준비합니다. 아래 설정은 Host와 모델별 검사를 위한 규약입니다. Wickle 코어가 `.env`를 자동으로 읽지는 않으며, 제공 경로 어댑터와 실제 연결 검사 runner는 아직 완성되지 않았습니다.

```dotenv
ANTHROPIC_API_KEY=
ANTHROPIC_BASE_URL=https://api.anthropic.com
ANTHROPIC_API_VERSION=2023-06-01

ANTHROPIC_MODEL_1_ID=
ANTHROPIC_MODEL_1_VERSION=

# 키가 특정 workspace에 한정되지 않은 경우에만 설정합니다.
# ANTHROPIC_WORKSPACE_ID=
```

1. [Claude Console의 API keys](https://platform.claude.com/settings/keys)에서 키를 생성하고 `ANTHROPIC_API_KEY`에 넣습니다. 개인 개발에는 개인 키, 공동 서비스에는 서비스 계정 키를 사용합니다. 키 생성 시 선택한 workspace 범위도 확인합니다. [키 생성·인증 방식](https://platform.claude.com/docs/en/manage-claude/authentication).
2. 특정 workspace에 한정되지 않은 키는 `ANTHROPIC_WORKSPACE_ID`도 필요합니다. Console의 **Settings → Workspaces**에서 ID를 확인합니다. 이 값은 HTTP의 `anthropic-workspace-id` 헤더에 대응합니다. [workspace 선택](https://platform.claude.com/docs/en/manage-claude/authentication#select-a-workspace).
3. Console에서 사용할 모델을 고른 뒤 [Models API](https://platform.claude.com/docs/en/api/models/list)의 목록과 [모델 ID·버전 문서](https://platform.claude.com/docs/en/about-claude/models/model-ids-and-versions)를 확인합니다. 직접 API의 정확한 ID를 `ANTHROPIC_MODEL_1_ID`에 복사합니다. Bedrock의 모델 ID나 화면 표시 이름을 넣지 않습니다.
4. 해당 ID가 가리키는 release를 확인해 `ANTHROPIC_MODEL_1_VERSION`에 기록합니다. 별도 버전 필드가 없고 공식적으로 고정 release를 식별하는 전체 ID라면 그 ID를 사용할 수 있습니다. 이름에서 날짜를 추측하거나 목록의 `created_at`을 모델 버전으로 바꾸지 않습니다. [ID와 버전의 의미](https://platform.claude.com/docs/en/about-claude/models/model-ids-and-versions), [모델 목록의 필드](https://platform.claude.com/docs/en/api/models/list).

모델 목록을 직접 조회하려면 키와 API 버전을 셸 환경변수에도 준비한 뒤 실행합니다. `.env` 파일을 저장하는 것만으로 셸에 변수가 설정되지는 않습니다. workspace가 필요한 키는 마지막 헤더도 추가합니다.

```sh
curl --fail-with-body --silent --show-error \
  'https://api.anthropic.com/v1/models' \
  -H "x-api-key: $ANTHROPIC_API_KEY" \
  -H "anthropic-version: $ANTHROPIC_API_VERSION"
# workspace가 필요한 경우 위 명령에 추가:
# -H "anthropic-workspace-id: $ANTHROPIC_WORKSPACE_ID"
```

이 명령은 공식 모델 목록 조회이며 Wickle 연결 검사를 대신하지 않습니다. `has_more`가 참이면 `last_id`를 다음 요청의 `after_id`로 전달해 나머지 목록을 확인합니다. [Models API](https://platform.claude.com/docs/en/api/models/list).

같은 키·workspace에서 다른 모델이나 버전도 검사하려면 다음 번호를 추가합니다.

```dotenv
ANTHROPIC_MODEL_2_ID=
ANTHROPIC_MODEL_2_VERSION=
```

번호는 `1`, `2`처럼 양의 정수이며, ID를 채운 각 번호가 독립된 모델·버전 검사 대상입니다. 확인하지 못한 VERSION은 준비 중에는 비워 두고 실제 검사 전에 확인합니다. 미확인 상태를 통과로 기록하지 않습니다. 계정·workspace·endpoint가 다르면 별도의 `.env` 파일에 공통 설정과 모델 목록을 작성합니다.

`ANTHROPIC_API_VERSION`은 요청 헤더의 API 계약 버전입니다. 모델 release나 SDK 버전과는 별개입니다. [API 버전](https://platform.claude.com/docs/en/api/versioning).
