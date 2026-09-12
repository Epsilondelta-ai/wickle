# Preparing provider credentials

Use [`.env.example`](../.env.example) to prepare accounts, credentials, and model
targets for a Host application or live integration tests. Provider adapters and
the live-test runner are not implemented yet. These are configuration conventions
for their integration, not variables that Wickle core reads or a claim that a
provider/model combination has passed verification.

The Host loads configuration and creates provider clients. It passes configured
adapters and credential references into the library. Raw keys, cloud credentials,
and access tokens belong in Host connection bindings, outside agent profiles,
model-visible tool inputs, transcripts, and saved run data.

## Create your local file

From the repository directory, create the file only if it does not already exist:

```sh
if [ ! -e .env ]; then
  (umask 077; cp .env.example .env)
fi
```

Edit `.env` locally and fill only the providers you intend to use. Keep unused
sections blank. Leave optional cloud credential variables commented out unless
you need them; setting them to an empty string can interfere with an existing
credential provider chain. Use a dotenv parser in the Host or test runner when
loading this file; copying it does not export variables or make network calls.
Treat blank configuration values as absent and never log their contents.

`WICKLE_LIVE_PROVIDERS` declares the providers to run when a live runner is
available. Its empty default selects none. For example,
`WICKLE_LIVE_PROVIDERS=openai,anthropic` selects just those two paths. Credentials
alone must never opt a provider into a live call. A selected provider with missing
configuration must fail preflight before calling a service, rather than silently
falling back to another account or provider. Live calls can incur provider charges.

`.env` and `.env.*` are ignored by Git; `.env.example` is the public template.
Credential JSON files must be stored outside this repository. Share only the
configuration file path and nonsecret model/deployment details when asking for
help.

## Select a model and its version

The template deliberately leaves model selections empty. Obtain the identifiers
from the provider's console or model catalog for your account and region.

| Setting | Meaning |
| --- | --- |
| `*_MODEL_ID` | Provider model identifier, including a snapshot suffix if the provider uses one |
| `*_MODEL_VERSION` | Verified release identifier for that model; preserve the provider's exact value |
| Deployment or inference profile | Invocation target that can route to an underlying model |
| Endpoint, region, project | Where requests are sent and which cloud resource serves them |
| `*_API_VERSION` | HTTP protocol/schema version, independent of model release |

Do not extract a version from an arbitrary name. When a provider identifies an
immutable release only by its full model ID, that verified ID can also identify
the release. If the release is unknown, leave its version blank during preparation
and resolve it before a test requiring a pinned model. A deployment name, alias,
or written version string alone does not establish immutability. Check the
underlying model and deployment upgrade settings. Adapter implementation versions
come from the build and its dependency lockfile, not from these credentials.

## OpenAI GPT — `openai`

Create a project API key in the OpenAI dashboard and put it in `OPENAI_API_KEY`.
Use a standard application key. Fill `OPENAI_MODEL_ID` and
`OPENAI_MODEL_VERSION` for an accessible release; the default base URL already
includes `/v1`. `OPENAI_ORG_ID` and `OPENAI_PROJECT_ID` are optional selections
for keys that require explicit organization/project headers. See
[OpenAI authentication](https://developers.openai.com/api/reference/overview#authentication)
and [API key setup](https://developers.openai.com/api/docs/guides/production-best-practices#api-keys).

## Azure Foundry OpenAI GPT — `azure_openai`

Create or select an OpenAI model deployment in Foundry. Record its resource
origin in `AZURE_OPENAI_ENDPOINT`, deployment name in `AZURE_OPENAI_DEPLOYMENT`,
and underlying model name/release in `AZURE_OPENAI_MODEL_ID` and
`AZURE_OPENAI_MODEL_VERSION`. The API's `model` parameter uses the deployment
name. Verify the deployment's model version and upgrade policy separately. See
[Azure model deployments](https://learn.microsoft.com/en-us/azure/foundry/openai/how-to/working-with-models).

With `AZURE_OPENAI_API_MODE=v1`, the request base is the resource origin plus
`/openai/v1/`; leave `AZURE_OPENAI_API_VERSION` empty. Azure's v1 API does not
require a dated `api-version` parameter. If a separately implemented adapter uses
the dated API, select `dated` and fill the exact API version required by that
path. A value in the template does not enable an unimplemented protocol. See
[Azure API versions](https://learn.microsoft.com/en-us/azure/foundry/openai/api-version-lifecycle?view=foundry-classic).

Choose `AZURE_OPENAI_AUTH_MODE=api_key` and fill the resource key, or choose
`entra` and leave the API key empty. For Entra, use an identity with inference
permission on the resource, such as the local Azure CLI identity after `az login`
or a managed identity in Azure. A service principal instead supplies tenant,
client ID, and client secret to the Host's credential provider. The Host must
obtain and refresh tokens for the selected endpoint's scope; the current Foundry
v1 examples use `https://ai.azure.com/.default`. See
[Microsoft Entra setup](https://learn.microsoft.com/en-us/azure/foundry/foundry-models/how-to/configure-entra-id).

## Anthropic Claude — `anthropic`

Create an API key in the Claude Console and set `ANTHROPIC_API_KEY`. If your key
is identity-linked and not limited to one workspace, also supply
`ANTHROPIC_WORKSPACE_ID`. Select the direct API model ID and release in
`ANTHROPIC_MODEL_ID` and `ANTHROPIC_MODEL_VERSION`. See
[Claude authentication](https://platform.claude.com/docs/en/manage-claude/authentication).

`ANTHROPIC_API_VERSION=2023-06-01` selects the Messages API header version. It
does not identify a Claude model release. The origin is
`https://api.anthropic.com`; the adapter adds the Messages path. See
[Anthropic API versioning](https://platform.claude.com/docs/en/api/versioning).

## AWS Bedrock Claude — `bedrock_anthropic`

Select a Claude model available to your AWS account and source region, and meet
its model access requirements. Fill `AWS_REGION`, `BEDROCK_MODEL_ID`, and
`BEDROCK_MODEL_VERSION`. Confirm the identity has the relevant invocation and
streaming permissions. See
[Bedrock model access](https://docs.aws.amazon.com/bedrock/latest/userguide/model-access.html).

Use the AWS SDK credential chain. For an IAM Identity Center profile, configure
and authenticate it locally, then set `AWS_PROFILE` to its name:

```sh
aws configure sso
aws sso login --profile your-profile-name
```

Existing shared credentials or an attached cloud role can also supply credentials.
If using temporary access keys, supply the access key, secret key, and session
token together; leave them unset when using a profile or cloud role. See the
[Rust SDK credential chain](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/credproviders.html)
and [IAM Identity Center setup](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-sso.html).

If invocation requires an inference profile, fill `BEDROCK_INFERENCE_PROFILE_ID`
with its ID or ARN and retain the underlying foundation model in
`BEDROCK_MODEL_ID`. The adapter sends the profile target in Bedrock's `modelId`
field. `AWS_REGION` remains the source region. See
[invocation with inference profiles](https://docs.aws.amazon.com/bedrock/latest/userguide/inference-profiles-use.html).
Leave `BEDROCK_ENDPOINT` unset for normal SDK endpoint resolution. The SDK/API
operation determines the Bedrock protocol contract; the direct Anthropic API
version above is not a Bedrock setting.

## Google AI Studio Gemini — `google_ai`

Create a Gemini API key in Google AI Studio and set `GEMINI_API_KEY`, then fill
`GEMINI_MODEL_ID` and `GEMINI_MODEL_VERSION`. Use the key type and restrictions
currently required by the Gemini API. Google SDKs may also read `GOOGLE_API_KEY`,
which takes precedence when both are set; the Host should pass the selected
Gemini key explicitly to avoid using an unrelated key. See
[Gemini API keys](https://ai.google.dev/gemini-api/docs/api-key).

`GEMINI_BASE_URL` is the origin; `GEMINI_API_VERSION=v1` selects the stable API
path. Select `v1beta` only when the chosen adapter and feature require that
protocol. The API version is independent of the Gemini release. See
[Gemini API versions](https://ai.google.dev/gemini-api/docs/api-versions).

## Google Cloud Vertex AI Gemini — `vertex_gemini`

Select a Google Cloud project with billing, enable `aiplatform.googleapis.com`,
and grant the caller inference access. Fill `GOOGLE_CLOUD_PROJECT`,
`GOOGLE_CLOUD_LOCATION`, `VERTEX_MODEL_ID`, and `VERTEX_MODEL_VERSION`. Choose a
region or `global` that supports the selected model and your routing requirements.
`VERTEX_API_VERSION=v1` selects the API contract. The normal endpoint follows the
chosen location; `VERTEX_ENDPOINT` is an optional origin override. See the
[Google Cloud Gemini quickstart](https://docs.cloud.google.com/vertex-ai/generative-ai/docs/start/quickstart).

This path uses Application Default Credentials (ADC). For local development,
configure ADC with:

```sh
gcloud auth application-default login
```

Leave `GOOGLE_APPLICATION_CREDENTIALS` unset to use local ADC or an attached cloud
identity. Set it only when deliberately choosing an external credential or
federation configuration file. The Host can map `GOOGLE_CLOUD_QUOTA_PROJECT` to
an explicit quota project when needed; environment support depends on its chosen
authentication library. See [quota project configuration](https://docs.cloud.google.com/docs/quotas/set-quota-project).
An AI Studio API key does not configure this ADC path. See
[ADC setup](https://docs.cloud.google.com/docs/authentication/provide-credentials-adc)
and [credential lookup order](https://docs.cloud.google.com/docs/authentication/application-default-credentials).

## xAI Grok — `xai`

Create an API key in the xAI console, ensure the account can make API calls, and
set `XAI_API_KEY`. Fill `XAI_MODEL_ID` and `XAI_MODEL_VERSION` from the model
catalog available to that account. `XAI_BASE_URL=https://api.x.ai/v1` includes
the API path version. See the [xAI quickstart](https://docs.x.ai/developers/quickstart).

## What live verification must establish

Credential preparation is separate from a successful integration test. Once a
provider runner exists, record the selected endpoint/region, resolved model
release, deployment target, API contract, and adapter version alongside actual
results. Exercise a response and a tool-call/result round trip, including streaming
where supported. Record unavailable capabilities or unverified version metadata
as such. Never report skipped or unconfigured providers as passing, and never
include credentials in captured evidence.
