from __future__ import annotations

import asyncio
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
            )
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
            raise MuxaiError(
                code=ErrorCode.PROVIDER_EXEC,
                message=stderr,
                provider=self.name,
                operation="Provider.run",
                temporary=True,
            )

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
        stdout, stderr = await process.communicate(self._prompt(request).encode())

        if process.returncode != 0:
            message = stderr.decode().strip() or "provider command failed"
            raise MuxaiError(
                code=ErrorCode.PROVIDER_EXEC,
                message=message,
                provider=self.name,
                operation="Provider.run_async",
                temporary=True,
            )

        raw = stdout.decode()
        return Response(provider=self.name, content=raw.strip(), raw=raw)
