from __future__ import annotations

from dataclasses import dataclass
from enum import Enum

from .types import ProviderName


class ErrorCode(str, Enum):
    CONFIG = "config_error"
    AUTH = "auth_error"
    RATE_LIMIT = "rate_limit_error"
    TRANSIENT = "transient_error"
    PROVIDER_EXEC = "provider_exec_error"
    PROVIDER_PARSE = "provider_parse_error"
    TIMEOUT = "timeout_error"
    CANCELED = "canceled_error"


@dataclass
class MuxaiError(Exception):
    code: ErrorCode
    message: str
    provider: ProviderName | None = None
    operation: str = "unknown"
    temporary: bool = False

    def __str__(self) -> str:
        provider = f" ({self.provider.value})" if self.provider else ""
        return f"{self.code.value}{provider} during {self.operation}: {self.message}"


def is_code(error: Exception, code: ErrorCode) -> bool:
    return isinstance(error, MuxaiError) and error.code == code


def is_temporary(error: Exception) -> bool:
    return isinstance(error, MuxaiError) and error.temporary
