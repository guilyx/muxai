import type { Event, ProviderName, Request, Response } from "./types.js";

export interface Provider {
  readonly name: ProviderName;
  run(request: Request): Promise<Response>;
  runEvents?(request: Request): AsyncIterable<Event>;
}
