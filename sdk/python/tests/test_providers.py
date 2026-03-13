from __future__ import annotations

import unittest

from muxai.errors import ErrorCode, MuxaiError
from muxai.providers.cursor import CursorProvider
from muxai.types import Message, Request, Role


class ProviderRuntimeTests(unittest.IsolatedAsyncioTestCase):
    def test_sync_provider_success(self) -> None:
        provider = CursorProvider(command="sh", args=["-c", "cat"])
        response = provider.run(Request(messages=[Message(role=Role.USER, content="hello")]))
        self.assertIn("hello", response.content)

    def test_sync_provider_auth_error_classification(self) -> None:
        provider = CursorProvider(
            command="sh",
            args=["-c", "echo unauthorized >&2; exit 1"],
        )
        with self.assertRaises(MuxaiError) as err:
            provider.run(Request(messages=[Message(role=Role.USER, content="x")]))
        self.assertEqual(err.exception.code, ErrorCode.AUTH)

    async def test_async_provider_timeout(self) -> None:
        provider = CursorProvider(
            command="sh",
            args=["-c", "sleep 2; echo done"],
            timeout_seconds=0.05,
        )
        with self.assertRaises(MuxaiError) as err:
            await provider.run_async(Request(messages=[Message(role=Role.USER, content="x")]))
        self.assertEqual(err.exception.code, ErrorCode.TIMEOUT)


if __name__ == "__main__":
    unittest.main()
