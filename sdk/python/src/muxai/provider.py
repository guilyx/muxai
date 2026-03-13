from __future__ import annotations

import abc

from .types import ProviderName, Request, Response


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
