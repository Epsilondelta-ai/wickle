//! Anthropic Messages HTTP/SSE with Host-owned credentials and signed thinking replay.
#![forbid(unsafe_code)]
mod codec;
mod connection;
mod inspection;
mod model;
mod response;
pub use connection::{AnthropicConnection, AnthropicOptions};
pub use inspection::{AnthropicInspector, AnthropicSnapshot};
pub use model::AnthropicModel;
use wickle::{ContractError, ErrorCode};
fn error(code: ErrorCode, location: &str) -> ContractError {
    ContractError::new(code, format!("anthropic.{location}"))
}
