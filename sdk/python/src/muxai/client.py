from __future__ import annotations

import asyncio
import time
from collections.abc import AsyncIterator
from contextlib import suppress
from dataclasses import dataclass

from .errors import ErrorCode, MuxaiError
from .provider import Provider
from .types import Event, EventType, ProviderName, Request, Response, now_seconds


@dataclass(frozen=True)
class ClientConfig:
    default_provider: ProviderName
    timeout_seconds: float = 30.0
    max_retries: int = 2
    base_delay_seconds: float = 0.1
    max_delay_seconds: float = 2.0


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

    def run_default(self, request: Request) -> Response:
        return self.run(request=request, provider=self._config.default_provider)

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
                started = now_seconds()
                response = adapter.run(request)
                if response.duration_seconds == 0:
                    response = Response(
                        provider=response.provider,
                        content=response.content,
                        raw=response.raw,
                        finish_reason=response.finish_reason,
                        usage=response.usage,
                        tool_calls=response.tool_calls,
                        duration_seconds=now_seconds() - started,
                    )
                return response
            except MuxaiError as exc:
                last_error = exc
                if not exc.temporary or attempt == self._config.max_retries:
                    raise
                time.sleep(self._retry_delay_seconds(attempt))
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
        except asyncio.CancelledError as exc:
            raise MuxaiError(
                code=ErrorCode.CANCELED,
                message="provider call canceled",
                operation="Client.run_async",
                provider=target,
                temporary=False,
            ) from exc

    async def run_events(
        self, request: Request, provider: ProviderName | None = None
    ) -> AsyncIterator[Event]:
        target = provider or self._config.default_provider
        adapter = self._providers.get(target)
        if adapter is None:
            raise MuxaiError(
                code=ErrorCode.CONFIG,
                message=f"provider {target.value!r} not registered",
                operation="Client.run_events",
                provider=target,
            )

        queue: asyncio.Queue[Event | None] = asyncio.Queue()
        result_error: list[MuxaiError] = []

        async def _collect() -> None:
            try:
                async for event in adapter.run_events(request):
                    await queue.put(event)
            except MuxaiError as exc:
                result_error.append(exc)
                await queue.put(Event(type=EventType.ERROR, provider=target, error=str(exc)))
            finally:
                await queue.put(None)

        task = asyncio.create_task(_collect())
        try:
            while True:
                item = await asyncio.wait_for(queue.get(), timeout=self._config.timeout_seconds)
                if item is None:
                    break
                yield item
        except asyncio.TimeoutError as exc:
            task.cancel()
            with suppress(asyncio.CancelledError):
                await task
            raise MuxaiError(
                code=ErrorCode.TIMEOUT,
                message="provider event stream timed out",
                operation="Client.run_events",
                provider=target,
                temporary=True,
            ) from exc
        finally:
            if not task.done():
                task.cancel()
                with suppress(asyncio.CancelledError):
                    await task

        if result_error:
            raise result_error[0]

    def _retry_delay_seconds(self, attempt: int) -> float:
        delay = self._config.base_delay_seconds * (2**attempt)
        delay += ((attempt * 97) % 31) / 1000
        return min(delay, self._config.max_delay_seconds)
