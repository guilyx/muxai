from __future__ import annotations

import unittest

from muxai.client import Client, ClientConfig
from muxai.errors import ErrorCode, MuxaiError
from muxai.provider import Provider
from muxai.types import Event, EventType, Message, ProviderName, Request, Response, Role


class FakeProvider(Provider):
    def __init__(self, name: ProviderName, content: str = "ok") -> None:
        self._name = name
        self._content = content

    @property
    def name(self) -> ProviderName:
        return self._name

    def run(self, request: Request) -> Response:
        return Response(provider=self._name, content=self._content, raw=self._content)

    async def run_async(self, request: Request) -> Response:
        return Response(provider=self._name, content=self._content, raw=self._content)

    async def run_events(self, request: Request):
        yield Event(type=EventType.STARTED, provider=self._name)
        yield Event(
            type=EventType.DONE,
            provider=self._name,
            response=Response(provider=self._name, content=self._content, raw=self._content),
        )


class FailingProvider(FakeProvider):
    def run(self, request: Request) -> Response:
        raise MuxaiError(
            code=ErrorCode.PROVIDER_EXEC,
            message="failed",
            provider=self.name,
            operation="run",
            temporary=False,
        )


class ClientTests(unittest.IsolatedAsyncioTestCase):
    def test_run_sync(self) -> None:
        client = Client(
            providers=[FakeProvider(ProviderName.CURSOR)],
            config=ClientConfig(default_provider=ProviderName.CURSOR),
        )
        response = client.run(Request(messages=[Message(role=Role.USER, content="hi")]))
        self.assertEqual(response.content, "ok")

    async def test_run_async(self) -> None:
        client = Client(
            providers=[FakeProvider(ProviderName.CLAUDE, content="async-ok")],
            config=ClientConfig(default_provider=ProviderName.CLAUDE),
        )
        response = await client.run_async(
            Request(messages=[Message(role=Role.USER, content="hello")])
        )
        self.assertEqual(response.content, "async-ok")

    async def test_run_events(self) -> None:
        client = Client(
            providers=[FakeProvider(ProviderName.CURSOR, content="stream-ok")],
            config=ClientConfig(default_provider=ProviderName.CURSOR),
        )
        got = []
        async for event in client.run_events(
            Request(messages=[Message(role=Role.USER, content="hello")])
        ):
            got.append(event.type)
        self.assertEqual(got, [EventType.STARTED, EventType.DONE])

    def test_unregistered_provider(self) -> None:
        client = Client(
            providers=[FakeProvider(ProviderName.CLAUDE)],
            config=ClientConfig(default_provider=ProviderName.CLAUDE),
        )
        with self.assertRaises(MuxaiError) as err:
            client.run(
                Request(messages=[Message(role=Role.USER, content="x")]),
                provider=ProviderName.CURSOR,
            )
        self.assertEqual(err.exception.code, ErrorCode.CONFIG)

    def test_error_passthrough(self) -> None:
        client = Client(
            providers=[FailingProvider(ProviderName.VIBE)],
            config=ClientConfig(default_provider=ProviderName.VIBE),
        )
        with self.assertRaises(MuxaiError):
            client.run(Request(messages=[Message(role=Role.USER, content="x")]))


if __name__ == "__main__":
    unittest.main()
