use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::errors::{ErrorCode, MuxaiError};
use crate::provider::Provider;
use crate::types::{Event, ProviderName, Request, Response};

#[derive(Clone, Debug)]
pub struct ClientConfig {
    pub default_provider: ProviderName,
    pub timeout: Duration,
    pub max_retries: usize,
    pub base_delay: Duration,
    pub max_delay: Duration,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            default_provider: ProviderName::Cursor,
            timeout: Duration::from_secs(30),
            max_retries: 2,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(2),
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

    pub fn run_default(&self, request: Request) -> Result<Response, MuxaiError> {
        self.run(None, request)
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
                    thread::sleep(self.retry_delay(attempts));
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

    pub async fn run_events(
        &self,
        provider: Option<ProviderName>,
        request: Request,
    ) -> Result<Vec<Event>, MuxaiError> {
        let selected = provider.unwrap_or_else(|| self.config.default_provider.clone());
        let adapter = self.providers.get(&selected).ok_or_else(|| MuxaiError {
            code: ErrorCode::Config,
            message: "provider is not registered".to_string(),
            provider: Some(selected.clone()),
            operation: "Client::run_events".to_string(),
            temporary: false,
        })?;
        adapter.run_events(request).await
    }

    fn retry_delay(&self, attempt: usize) -> Duration {
        let mut delay = self.config.base_delay * (2u32.pow(attempt as u32));
        let jitter_ms = ((attempt * 97) % 31) as u64;
        delay += Duration::from_millis(jitter_ms);
        if delay > self.config.max_delay {
            return self.config.max_delay;
        }
        delay
    }
}
