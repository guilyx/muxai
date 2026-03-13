use std::collections::HashMap;
use std::future::Future;
use std::io::Write;
use std::pin::Pin;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use wait_timeout::ChildExt;

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
    pub env: HashMap<String, String>,
    pub timeout: Duration,
}

impl CliProvider {
    pub fn cursor() -> Self {
        Self {
            provider_name: ProviderName::Cursor,
            command: "cursor-agent".to_string(),
            args: Vec::new(),
            env: HashMap::new(),
            timeout: Duration::from_secs(30),
        }
    }

    pub fn claude() -> Self {
        Self {
            provider_name: ProviderName::Claude,
            command: "claude".to_string(),
            args: Vec::new(),
            env: HashMap::new(),
            timeout: Duration::from_secs(30),
        }
    }

    pub fn vibe() -> Self {
        Self {
            provider_name: ProviderName::Vibe,
            command: "vibe".to_string(),
            args: Vec::new(),
            env: HashMap::new(),
            timeout: Duration::from_secs(30),
        }
    }

    fn classify_error(&self, message: String, operation: &str) -> MuxaiError {
        let lowered = message.to_lowercase();
        let (code, temporary) = if lowered.contains("unauthorized") || lowered.contains("auth") {
            (ErrorCode::Auth, false)
        } else if lowered.contains("rate limit") || lowered.contains("too many requests") {
            (ErrorCode::RateLimit, true)
        } else if lowered.contains("timeout") {
            (ErrorCode::Timeout, true)
        } else {
            (ErrorCode::ProviderExec, true)
        };

        MuxaiError {
            code,
            message,
            provider: Some(self.provider_name.clone()),
            operation: operation.to_string(),
            temporary,
        }
    }

    fn build_prompt(&self, request: &Request) -> String {
        let mut lines = Vec::<String>::new();
        if let Some(system_prompt) = &request.system_prompt {
            lines.push("[system]".to_string());
            lines.push(system_prompt.clone());
            lines.push(String::new());
        }
        for message in &request.messages {
            let mut header = format!("[{:?}]", message.role).to_lowercase();
            if let Some(name) = &message.name {
                header.push_str(&format!("({name})"));
            }
            lines.push(header);
            lines.push(message.content.clone());
            lines.push(String::new());
        }
        lines.join("\n").trim().to_string()
    }
}

impl Provider for CliProvider {
    fn name(&self) -> ProviderName {
        self.provider_name.clone()
    }

    fn run(&self, request: Request) -> Result<Response, MuxaiError> {
        let started = Instant::now();
        let mut child = Command::new(&self.command);
        child
            .args(&self.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        if !self.env.is_empty() {
            child.envs(self.env.clone());
        }

        let mut child = child.spawn().map_err(|error| MuxaiError {
            code: ErrorCode::ProviderExec,
            message: error.to_string(),
            provider: Some(self.provider_name.clone()),
            operation: "CliProvider::run".to_string(),
            temporary: false,
        })?;

        if let Some(stdin) = child.stdin.as_mut() {
            let prompt = self.build_prompt(&request);
            stdin
                .write_all(prompt.as_bytes())
                .map_err(|error| MuxaiError {
                    code: ErrorCode::ProviderExec,
                    message: error.to_string(),
                    provider: Some(self.provider_name.clone()),
                    operation: "CliProvider::run".to_string(),
                    temporary: false,
                })?;
        }

        let status = child
            .wait_timeout(self.timeout)
            .map_err(|error| MuxaiError {
                code: ErrorCode::ProviderExec,
                message: error.to_string(),
                provider: Some(self.provider_name.clone()),
                operation: "CliProvider::run".to_string(),
                temporary: false,
            })?;

        if status.is_none() {
            let _ = child.kill();
            let _ = child.wait();
            return Err(MuxaiError {
                code: ErrorCode::Timeout,
                message: format!("provider command timed out after {:?}", self.timeout),
                provider: Some(self.provider_name.clone()),
                operation: "CliProvider::run".to_string(),
                temporary: true,
            });
        }

        let output = child.wait_with_output().map_err(|error| MuxaiError {
            code: ErrorCode::ProviderExec,
            message: error.to_string(),
            provider: Some(self.provider_name.clone()),
            operation: "CliProvider::run".to_string(),
            temporary: false,
        })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(self.classify_error(
                if stderr.trim().is_empty() {
                    format!("provider exited with status {}", output.status)
                } else {
                    stderr
                },
                "CliProvider::run",
            ));
        }

        let raw = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(Response {
            provider: self.provider_name.clone(),
            content: raw.trim().to_string(),
            raw,
            finish_reason: FinishReason::Stop,
            usage: Usage::default(),
            tool_calls: Vec::<ToolCall>::new(),
            duration: started.elapsed(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Message, Role};

    #[test]
    fn cli_provider_passes_request_prompt() {
        let provider = CliProvider {
            provider_name: ProviderName::Cursor,
            command: "sh".to_string(),
            args: vec!["-c".to_string(), "cat".to_string()],
            env: HashMap::new(),
            timeout: Duration::from_secs(2),
        };
        let response = provider
            .run(Request {
                messages: vec![Message {
                    role: Role::User,
                    content: "hello".to_string(),
                    name: None,
                }],
                ..Default::default()
            })
            .expect("provider run should succeed");

        assert!(response.content.contains("hello"));
    }

    #[test]
    fn cli_provider_classifies_auth_error() {
        let provider = CliProvider {
            provider_name: ProviderName::Cursor,
            command: "sh".to_string(),
            args: vec![
                "-c".to_string(),
                "echo unauthorized 1>&2; exit 1".to_string(),
            ],
            env: HashMap::new(),
            timeout: Duration::from_secs(2),
        };
        let err = provider
            .run(Request::default())
            .expect_err("provider run should fail");
        assert_eq!(err.code, ErrorCode::Auth);
    }

    #[test]
    fn cli_provider_times_out() {
        let provider = CliProvider {
            provider_name: ProviderName::Cursor,
            command: "sh".to_string(),
            args: vec!["-c".to_string(), "sleep 2".to_string()],
            env: HashMap::new(),
            timeout: Duration::from_millis(50),
        };
        let err = provider
            .run(Request::default())
            .expect_err("provider run should timeout");
        assert_eq!(err.code, ErrorCode::Timeout);
    }
}
