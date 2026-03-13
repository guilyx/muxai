from __future__ import annotations

import asyncio
import time
from dataclasses import dataclass

from .errors import ErrorCode, MuxaiError
from .provider import Provider
from .types import ProviderName, Request, Response


@dataclass(frozen=True)
class ClientConfig:
    default_provider: ProviderName
    timeout_seconds: float = 30.0
    max_retries: int = 2
    base_delay_seconds: float = 0.1


class Client:
    def __init__(self, providers: list[Provider], config: ClientConfig) -> None:
        if not providers:
            raise MuxaiError(
                code=ErrorCode.CONFIG,
                message="at least one provider is required",
                operation="Client.__init__",
            )
        self._providers = {provider.name: provider for provider in providers}
        self._config = config

        if config.default_provider not in self._providers:
            raise MuxaiError(
                code=ErrorCode.CONFIG,
                message=f"default provider {config.default_provider.value!r} not registered",
                operation="Client.__init__",
                provider=config.default_provider,
            )

    def run(self, request: Request, provider: ProviderName | None = None) -> Response:
        target = provider or self._config.default_provider
        adapter = self._providers.get(target)
        if adapter is None:
            raise MuxaiError(
                code=ErrorCode.CONFIG,
                message=f"provider {target.value!r} not registered",
                operation="Client.run",
                provider=target,
            )

        last_error: Exception | None = None
        for attempt in range(self._config.max_retries + 1):
            try:
                return adapter.run(request)
            except MuxaiError as exc:
                last_error = exc
                if not exc.temporary or attempt == self._config.max_retries:
                    raise
                time.sleep(self._config.base_delay_seconds * (2**attempt))
            except Exception as exc:  # pragma: no cover - defensive safety
                last_error = exc
                raise MuxaiError(
                    code=ErrorCode.PROVIDER_EXEC,
                    message=str(exc),
                    operation="Client.run",
                    provider=target,
                    temporary=False,
                ) from exc

        raise MuxaiError(
            code=ErrorCode.TRANSIENT,
            message=str(last_error),
            operation="Client.run",
            provider=target,
            temporary=True,
        )

    async def run_async(self, request: Request, provider: ProviderName | None = None) -> Response:
        target = provider or self._config.default_provider
        adapter = self._providers.get(target)
        if adapter is None:
            raise MuxaiError(
                code=ErrorCode.CONFIG,
                message=f"provider {target.value!r} not registered",
                operation="Client.run_async",
                provider=target,
            )

        async def _run_once() -> Response:
            return await adapter.run_async(request)

        try:
            return await asyncio.wait_for(_run_once(), timeout=self._config.timeout_seconds)
        except asyncio.TimeoutError as exc:
            raise MuxaiError(
                code=ErrorCode.TIMEOUT,
                message="provider call timed out",
                operation="Client.run_async",
                provider=target,
                temporary=True,
            ) from exc
