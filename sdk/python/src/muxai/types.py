from __future__ import annotations

from dataclasses import dataclass, field
from enum import Enum


class ProviderName(str, Enum):
    CURSOR = "cursor"
    CLAUDE = "claude"
    VIBE = "vibe"


class Role(str, Enum):
    SYSTEM = "system"
    USER = "user"
    ASSISTANT = "assistant"
    TOOL = "tool"


@dataclass(frozen=True)
class Message:
    role: Role
    content: str
    name: str | None = None


@dataclass(frozen=True)
class Request:
    messages: list[Message]
    system_prompt: str | None = None
    max_turns: int = 1
    metadata: dict[str, str] = field(default_factory=dict)


@dataclass(frozen=True)
class Response:
    provider: ProviderName
    content: str
    raw: str
