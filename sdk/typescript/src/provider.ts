import type { Request, Response, ProviderName } from "./types.js";

export interface Provider {
  readonly name: ProviderName;
  run(request: Request): Promise<Response>;
}
