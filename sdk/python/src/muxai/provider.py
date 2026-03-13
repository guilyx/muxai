from __future__ import annotations

import abc
from collections.abc import AsyncIterator

from .types import Event, EventType, ProviderName, Request, Response


class Provider(abc.ABC):
    @property
    @abc.abstractmethod
    def name(self) -> ProviderName:
        raise NotImplementedError

    @abc.abstractmethod
    def run(self, request: Request) -> Response:
        raise NotImplementedError

    @abc.abstractmethod
    async def run_async(self, request: Request) -> Response:
        raise NotImplementedError

    async def run_events(self, request: Request) -> AsyncIterator[Event]:
        yield Event(type=EventType.STARTED, provider=self.name)
        response = await self.run_async(request)
        yield Event(type=EventType.DONE, provider=self.name, response=response)
