# Provider configuration

Prepare credentials and one environment-variable block per model/version using
[the model test `.env` guide](env/README.md). It links separate guides for OpenAI,
Azure OpenAI, Anthropic, Bedrock, Gemini API, Vertex AI, and xAI.

[`.env.example`](../.env.example) contains shared authentication/connection settings
and the first numbered model slot for each provider. Add further slots for more
models or versions. No provider enable-list variable is required.

Provider adapters and model-specific live tests are still being implemented.
Configuration files supply Host/test settings; Wickle core does not load `.env`
or put credentials into profiles, prompts, tool arguments, or run snapshots.
