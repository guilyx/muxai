//! muxai Rust SDK

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::process::Command;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub const VERSION: &str = "0.1.0";

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ProviderName {
    Cursor,
    Claude,
    Vibe,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Request {
    pub messages: Vec<Message>,
    pub system_prompt: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Response {
    pub provider: ProviderName,
    pub content: String,
    pub raw: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorCode {
    Config,
    Auth,
    RateLimit,
    Transient,
    ProviderExec,
    ProviderParse,
    Timeout,
    Canceled,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MuxaiError {
    pub code: ErrorCode,
    pub message: String,
    pub provider: Option<ProviderName>,
    pub operation: String,
    pub temporary: bool,
}

impl std::fmt::Display for MuxaiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} during {}: {}",
            self.code, self.operation, self.message
        )
    }
}

impl std::error::Error for MuxaiError {}

pub trait Provider: Send + Sync {
    fn name(&self) -> ProviderName;
    fn run(&self, request: Request) -> Result<Response, MuxaiError>;
    fn run_async(
        &self,
        request: Request,
    ) -> Pin<Box<dyn Future<Output = Result<Response, MuxaiError>> + Send + '_>> {
        Box::pin(async move { self.run(request) })
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

    fn run(&self, request: Request) -> Result<Response, MuxaiError> {
        let mut child = Command::new(&self.command);
        child.args(&self.args);
        let output = child.output().map_err(|error| MuxaiError {
            code: ErrorCode::ProviderExec,
            message: error.to_string(),
            provider: Some(self.provider_name.clone()),
            operation: "CliProvider::run".to_string(),
            temporary: false,
        })?;
        let raw = String::from_utf8_lossy(&output.stdout).to_string();
        let _ = request;
        Ok(Response {
            provider: self.provider_name.clone(),
            content: raw.trim().to_string(),
            raw,
        })
    }
}

#[derive(Clone, Debug)]
pub struct ClientConfig {
    pub default_provider: ProviderName,
    pub timeout: Duration,
    pub max_retries: usize,
    pub base_delay: Duration,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            default_provider: ProviderName::Cursor,
            timeout: Duration::from_secs(30),
            max_retries: 2,
            base_delay: Duration::from_millis(100),
        }
    }
}

pub struct Client {
    providers: HashMap<ProviderName, Arc<dyn Provider>>,
    config: ClientConfig,
}

impl Client {
    pub fn new(
        providers: Vec<Arc<dyn Provider>>,
        config: ClientConfig,
    ) -> Result<Self, MuxaiError> {
        if providers.is_empty() {
            return Err(MuxaiError {
                code: ErrorCode::Config,
                message: "at least one provider is required".to_string(),
                provider: None,
                operation: "Client::new".to_string(),
                temporary: false,
            });
        }

        let map = providers
            .into_iter()
            .map(|provider| (provider.name(), provider))
            .collect::<HashMap<_, _>>();

        if !map.contains_key(&config.default_provider) {
            return Err(MuxaiError {
                code: ErrorCode::Config,
                message: "default provider is not registered".to_string(),
                provider: Some(config.default_provider.clone()),
                operation: "Client::new".to_string(),
                temporary: false,
            });
        }

        Ok(Self {
            providers: map,
            config,
        })
    }

    pub fn run(
        &self,
        provider: Option<ProviderName>,
        request: Request,
    ) -> Result<Response, MuxaiError> {
        let selected = provider.unwrap_or_else(|| self.config.default_provider.clone());
        let adapter = self.providers.get(&selected).ok_or_else(|| MuxaiError {
            code: ErrorCode::Config,
            message: "provider is not registered".to_string(),
            provider: Some(selected.clone()),
            operation: "Client::run".to_string(),
            temporary: false,
        })?;

        let mut attempts = 0usize;
        loop {
            match adapter.run(request.clone()) {
                Ok(response) => return Ok(response),
                Err(err) if err.temporary && attempts < self.config.max_retries => {
                    let delay = self.config.base_delay * (2u32.pow(attempts as u32));
                    thread::sleep(delay);
                    attempts += 1;
                }
                Err(err) => return Err(err),
            }
        }
    }

    pub async fn run_async(
        &self,
        provider: Option<ProviderName>,
        request: Request,
    ) -> Result<Response, MuxaiError> {
        let selected = provider.unwrap_or_else(|| self.config.default_provider.clone());
        let adapter = self.providers.get(&selected).ok_or_else(|| MuxaiError {
            code: ErrorCode::Config,
            message: "provider is not registered".to_string(),
            provider: Some(selected.clone()),
            operation: "Client::run_async".to_string(),
            temporary: false,
        })?;

        let _ = self.config.timeout;
        adapter.run_async(request).await
    }
}

#[cfg(test)]
mod tests {
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
            .run(
                None,
                Request {
                    messages: vec![Message {
                        role: Role::User,
                        content: "hi".to_string(),
                    }],
                    system_prompt: None,
                },
            )
            .expect("run should succeed");
        assert_eq!(response.content, "ok");
    }
}
