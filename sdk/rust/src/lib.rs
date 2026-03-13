//! muxai Rust SDK

pub mod client;
pub mod errors;
pub mod provider;
pub mod types;

pub use client::{Client, ClientConfig};
pub use errors::{ErrorCode, MuxaiError};
pub use provider::{BoxFuture, CliProvider, Provider};
pub use types::{
    Event, EventType, FinishReason, Message, ProviderName, Request, Response, Role, ToolCall,
    ToolDefinition, Usage,
};

pub const VERSION: &str = "0.1.0";

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    struct FakeProvider {
        name: ProviderName,
    }

    impl Provider for FakeProvider {
        fn name(&self) -> ProviderName {
            self.name.clone()
        }

        fn run(&self, _request: Request) -> Result<Response, MuxaiError> {
            Ok(Response {
                provider: self.name.clone(),
                content: "ok".to_string(),
                raw: "ok".to_string(),
                finish_reason: FinishReason::Stop,
                usage: Usage::default(),
                tool_calls: vec![],
                duration: std::time::Duration::default(),
            })
        }
    }

    #[test]
    fn run_sync_success() {
        let provider = Arc::new(FakeProvider {
            name: ProviderName::Cursor,
        });
        let client = Client::new(
            vec![provider],
            ClientConfig {
                default_provider: ProviderName::Cursor,
                ..Default::default()
            },
        )
        .expect("client should initialize");

        let response = client
            .run_default(Request {
                messages: vec![Message {
                    role: Role::User,
                    content: "hi".to_string(),
                    name: None,
                }],
                ..Default::default()
            })
            .expect("run should succeed");
        assert_eq!(response.content, "ok");
    }
}
