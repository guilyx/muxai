from __future__ import annotations

import asyncio
import contextlib
import os
import subprocess
from dataclasses import dataclass, field

from ..errors import ErrorCode, MuxaiError
from ..provider import Provider
from ..types import Message, Request, Response


@dataclass
class CLIProvider(Provider):
    command: str
    args: list[str]
    env: dict[str, str] = field(default_factory=dict)
    timeout_seconds: float = 30.0

    def _prompt(self, request: Request) -> str:
        lines: list[str] = []
        if request.system_prompt:
            lines.extend(["[system]", request.system_prompt, ""])
        for message in request.messages:
            lines.append(self._format_message(message))
        return "\n".join(lines).strip()

    @staticmethod
    def _format_message(message: Message) -> str:
        header = f"[{message.role.value}]"
        if message.name:
            header = f"{header}({message.name})"
        return f"{header}\n{message.content}\n"

    def run(self, request: Request) -> Response:
        try:
            completed = subprocess.run(
                [self.command, *self.args],
                input=self._prompt(request),
                text=True,
                capture_output=True,
                check=False,
                env={**os.environ, **self.env} if self.env else None,
                timeout=self.timeout_seconds,
            )
        except subprocess.TimeoutExpired as exc:
            raise MuxaiError(
                code=ErrorCode.TIMEOUT,
                message=f"provider command timed out after {self.timeout_seconds}s",
                provider=self.name,
                operation="Provider.run",
                temporary=True,
            ) from exc
        except OSError as exc:
            raise MuxaiError(
                code=ErrorCode.PROVIDER_EXEC,
                message=str(exc),
                provider=self.name,
                operation="Provider.run",
                temporary=False,
            ) from exc

        if completed.returncode != 0:
            stderr = completed.stderr.strip() or "provider command failed"
            raise self._classify_process_error(stderr, "Provider.run")

        raw = completed.stdout
        return Response(provider=self.name, content=raw.strip(), raw=raw)

    async def run_async(self, request: Request) -> Response:
        process = await asyncio.create_subprocess_exec(
            self.command,
            *self.args,
            stdin=asyncio.subprocess.PIPE,
            stdout=asyncio.subprocess.PIPE,
            stderr=asyncio.subprocess.PIPE,
            env={**os.environ, **self.env} if self.env else None,
        )
        try:
            stdout, stderr = await asyncio.wait_for(
                process.communicate(self._prompt(request).encode()),
                timeout=self.timeout_seconds,
            )
        except asyncio.TimeoutError as exc:
            process.kill()
            with contextlib.suppress(Exception):
                await process.communicate()
            raise MuxaiError(
                code=ErrorCode.TIMEOUT,
                message=f"provider command timed out after {self.timeout_seconds}s",
                provider=self.name,
                operation="Provider.run_async",
                temporary=True,
            ) from exc

        if process.returncode != 0:
            message = stderr.decode().strip() or "provider command failed"
            raise self._classify_process_error(message, "Provider.run_async")

        raw = stdout.decode()
        return Response(provider=self.name, content=raw.strip(), raw=raw)

    def _classify_process_error(self, message: str, operation: str) -> MuxaiError:
        lowered = message.lower()
        if "unauthorized" in lowered or "auth" in lowered:
            code = ErrorCode.AUTH
            temporary = False
        elif "rate limit" in lowered or "too many requests" in lowered:
            code = ErrorCode.RATE_LIMIT
            temporary = True
        elif "timeout" in lowered:
            code = ErrorCode.TIMEOUT
            temporary = True
        else:
            code = ErrorCode.PROVIDER_EXEC
            temporary = True

        return MuxaiError(
            code=code,
            message=message,
            provider=self.name,
            operation=operation,
            temporary=temporary,
        )
