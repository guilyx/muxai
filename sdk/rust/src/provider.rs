use std::future::Future;
use std::pin::Pin;
use std::process::Command;
use std::time::Duration;

use crate::errors::{ErrorCode, MuxaiError};
use crate::types::{
    Event, EventType, FinishReason, ProviderName, Request, Response, ToolCall, Usage,
};

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

pub trait Provider: Send + Sync {
    fn name(&self) -> ProviderName;
    fn run(&self, request: Request) -> Result<Response, MuxaiError>;

    fn run_async<'a>(&'a self, request: Request) -> BoxFuture<'a, Result<Response, MuxaiError>> {
        Box::pin(async move { self.run(request) })
    }

    fn run_events<'a>(&'a self, request: Request) -> BoxFuture<'a, Result<Vec<Event>, MuxaiError>> {
        Box::pin(async move {
            let provider = self.name();
            let response = self.run_async(request).await?;
            Ok(vec![
                Event {
                    event_type: EventType::Started,
                    provider: provider.clone(),
                    delta: None,
                    response: None,
                    error: None,
                },
                Event {
                    event_type: EventType::Done,
                    provider,
                    delta: None,
                    response: Some(response),
                    error: None,
                },
            ])
        })
    }
}

pub struct CliProvider {
    pub provider_name: ProviderName,
    pub command: String,
    pub args: Vec<String>,
}

impl CliProvider {
    pub fn cursor() -> Self {
        Self {
            provider_name: ProviderName::Cursor,
            command: "cursor-agent".to_string(),
            args: Vec::new(),
        }
    }

    pub fn claude() -> Self {
        Self {
            provider_name: ProviderName::Claude,
            command: "claude".to_string(),
            args: Vec::new(),
        }
    }

    pub fn vibe() -> Self {
        Self {
            provider_name: ProviderName::Vibe,
            command: "vibe".to_string(),
            args: Vec::new(),
        }
    }
}

impl Provider for CliProvider {
    fn name(&self) -> ProviderName {
        self.provider_name.clone()
    }

    fn run(&self, _request: Request) -> Result<Response, MuxaiError> {
        let mut child = Command::new(&self.command);
        child.args(&self.args);
        let output = child.output().map_err(|error| MuxaiError {
            code: ErrorCode::ProviderExec,
            message: error.to_string(),
            provider: Some(self.provider_name.clone()),
            operation: "CliProvider::run".to_string(),
            temporary: false,
        })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(MuxaiError {
                code: ErrorCode::ProviderExec,
                message: if stderr.trim().is_empty() {
                    format!("provider exited with status {}", output.status)
                } else {
                    stderr
                },
                provider: Some(self.provider_name.clone()),
                operation: "CliProvider::run".to_string(),
                temporary: true,
            });
        }

        let raw = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(Response {
            provider: self.provider_name.clone(),
            content: raw.trim().to_string(),
            raw,
            finish_reason: FinishReason::Stop,
            usage: Usage::default(),
            tool_calls: Vec::<ToolCall>::new(),
            duration: Duration::default(),
        })
    }
}
