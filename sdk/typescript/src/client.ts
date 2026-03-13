import { MuxaiError } from "./errors.js";
import type { Provider } from "./provider.js";
import type { Event, ProviderName, Request, Response } from "./types.js";

export interface ClientConfig {
  defaultProvider: ProviderName;
  timeoutMs?: number;
  maxRetries?: number;
  maxDelayMs?: number;
}

export class Client {
  readonly #providers: Map<ProviderName, Provider>;
  readonly #config: Required<ClientConfig>;

  constructor(providers: Provider[], config: ClientConfig) {
    if (providers.length === 0) {
      throw new MuxaiError({
        code: "config_error",
        message: "at least one provider is required",
        operation: "Client.constructor",
      });
    }
    this.#providers = new Map(providers.map((provider) => [provider.name, provider]));
    this.#config = {
      defaultProvider: config.defaultProvider,
      timeoutMs: config.timeoutMs ?? 30_000,
      maxRetries: config.maxRetries ?? 2,
      maxDelayMs: config.maxDelayMs ?? 2_000,
    };
    if (!this.#providers.has(this.#config.defaultProvider)) {
      throw new MuxaiError({
        code: "config_error",
        message: `default provider '${this.#config.defaultProvider}' is not registered`,
        provider: this.#config.defaultProvider,
        operation: "Client.constructor",
      });
    }
  }

  async run(request: Request, provider?: ProviderName): Promise<Response> {
    const target = provider ?? this.#config.defaultProvider;
    const adapter = this.#providers.get(target);
    if (!adapter) {
      throw new MuxaiError({
        code: "config_error",
        message: `provider '${target}' is not registered`,
        provider: target,
        operation: "Client.run",
      });
    }

    let attempt = 0;
    while (attempt <= this.#config.maxRetries) {
      try {
        return await this.#runWithTimeout(adapter.run(request), target);
      } catch (error) {
        const muxErr =
          error instanceof MuxaiError
            ? error
            : new MuxaiError({
                code: "provider_exec_error",
                message: error instanceof Error ? error.message : String(error),
                provider: target,
                operation: "Client.run",
                temporary: false,
              });
        if (!muxErr.temporary || attempt === this.#config.maxRetries) {
          throw muxErr;
        }
        await sleep(this.#retryDelayMs(attempt));
      }
      attempt += 1;
    }

    throw new MuxaiError({
      code: "transient_error",
      message: "retry attempts exhausted",
      provider: target,
      operation: "Client.run",
      temporary: true,
    });
  }

  async runDefault(request: Request): Promise<Response> {
    return this.run(request, this.#config.defaultProvider);
  }

  async *runEvents(
    request: Request,
    provider?: ProviderName,
  ): AsyncIterable<Event> {
    const target = provider ?? this.#config.defaultProvider;
    const adapter = this.#providers.get(target);
    if (!adapter) {
      throw new MuxaiError({
        code: "config_error",
        message: `provider '${target}' is not registered`,
        provider: target,
        operation: "Client.runEvents",
      });
    }

    if (adapter.runEvents) {
      for await (const event of adapter.runEvents(request)) {
        yield event;
      }
      return;
    }

    yield { type: "started", provider: target };
    const response = await this.run(request, target);
    yield { type: "done", provider: target, response };
  }

  async #runWithTimeout(
    promise: Promise<Response>,
    provider: ProviderName,
  ): Promise<Response> {
    let timeoutId: ReturnType<typeof setTimeout> | undefined;
    try {
      return await Promise.race([
        promise,
        new Promise<Response>((_, reject) => {
          timeoutId = setTimeout(() => {
            reject(
              new MuxaiError({
                code: "timeout_error",
                message: "provider call timed out",
                provider,
                operation: "Client.run",
                temporary: true,
              }),
            );
          }, this.#config.timeoutMs);
        }),
      ]);
    } finally {
      if (timeoutId) {
        clearTimeout(timeoutId);
      }
    }
  }

  #retryDelayMs(attempt: number): number {
    let delay = 100 * 2 ** attempt;
    delay += (attempt * 97) % 31;
    if (delay > this.#config.maxDelayMs) {
      return this.#config.maxDelayMs;
    }
    return delay;
  }
}

function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
