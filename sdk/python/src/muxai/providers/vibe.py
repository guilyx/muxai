from __future__ import annotations

from dataclasses import dataclass, field

from ..types import ProviderName
from .base import CLIProvider


@dataclass
class VibeProvider(CLIProvider):
    command: str = "vibe"
    args: list[str] = field(default_factory=list)
    env: dict[str, str] = field(default_factory=dict)

    @property
    def name(self) -> ProviderName:
        return ProviderName.VIBE
