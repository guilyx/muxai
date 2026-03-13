from __future__ import annotations

from dataclasses import dataclass, field
from enum import Enum
from time import monotonic


class ProviderName(str, Enum):
    CURSOR = "cursor"
    CLAUDE = "claude"
    VIBE = "vibe"


class Role(str, Enum):
    SYSTEM = "system"
    USER = "user"
    ASSISTANT = "assistant"
    TOOL = "tool"


class FinishReason(str, Enum):
    STOP = "stop"
    TOOL_CALL = "tool_call"
    LENGTH = "length"
    ERROR = "error"
    INCOMPLETE = "incomplete"


@dataclass(frozen=True)
class Message:
    role: Role
    content: str
    name: str | None = None


@dataclass(frozen=True)
class Request:
    messages: list[Message]
    system_prompt: str | None = None
    tools: list["ToolDefinition"] = field(default_factory=list)
    max_turns: int = 1
    metadata: dict[str, str] = field(default_factory=dict)


@dataclass(frozen=True)
class ToolDefinition:
    name: str
    description: str = ""


@dataclass(frozen=True)
class ToolCall:
    name: str
    arguments: str


@dataclass(frozen=True)
class Usage:
    input_tokens: int = 0
    output_tokens: int = 0
    total_tokens: int = 0


@dataclass(frozen=True)
class Response:
    provider: ProviderName
    content: str
    raw: str
    finish_reason: FinishReason = FinishReason.STOP
    usage: Usage = field(default_factory=Usage)
    tool_calls: list[ToolCall] = field(default_factory=list)
    duration_seconds: float = 0.0


class EventType(str, Enum):
    STARTED = "started"
    DELTA = "delta"
    DONE = "done"
    ERROR = "error"


@dataclass(frozen=True)
class Event:
    type: EventType
    provider: ProviderName
    delta: str = ""
    response: Response | None = None
    error: str | None = None


def now_seconds() -> float:
    return monotonic()
