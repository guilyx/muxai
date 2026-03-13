use std::fmt::{Display, Formatter};

use crate::types::ProviderName;

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

impl Display for MuxaiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} during {}: {}",
            self.code, self.operation, self.message
        )
    }
}

impl std::error::Error for MuxaiError {}
