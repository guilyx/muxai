import { spawn } from "node:child_process";

import { MuxaiError } from "../errors.js";
import type { Provider } from "../provider.js";
import type { Event, Message, ProviderName, Request, Response } from "../types.js";

export class CliProvider implements Provider {
  readonly name: ProviderName;
  readonly #command: string;
  readonly #args: string[];
  readonly #env?: Record<string, string | undefined>;
  readonly #timeoutMs: number;

  constructor(args: {
    name: ProviderName;
    command: string;
    cliArgs?: string[];
    env?: Record<string, string | undefined>;
    timeoutMs?: number;
  }) {
    this.name = args.name;
    this.#command = args.command;
    this.#args = args.cliArgs ?? [];
    this.#env = args.env;
    this.#timeoutMs = args.timeoutMs ?? 30_000;
  }

  async run(request: Request): Promise<Response> {
    const prompt = formatPrompt(request);
    return await new Promise<Response>((resolve, reject) => {
      const child = spawn(this.#command, this.#args, {
        env: { ...process.env, ...this.#env },
      });
      let stdout = "";
      let stderr = "";
      let finished = false;
      const timeout = setTimeout(() => {
        if (finished) {
          return;
        }
        child.kill("SIGKILL");
        reject(
          new MuxaiError({
            code: "timeout_error",
            message: `provider command timed out after ${this.#timeoutMs}ms`,
            provider: this.name,
            operation: "CliProvider.run",
            temporary: true,
          }),
        );
      }, this.#timeoutMs);

      child.stdout.on("data", (chunk) => {
        stdout += String(chunk);
      });
      child.stderr.on("data", (chunk) => {
        stderr += String(chunk);
      });
      child.on("error", (error) => {
        clearTimeout(timeout);
        finished = true;
        reject(
          new MuxaiError({
            code: "provider_exec_error",
            message: error.message,
            provider: this.name,
            operation: "CliProvider.run",
          }),
        );
      });
      child.on("close", (code) => {
        clearTimeout(timeout);
        finished = true;
        if (code && code !== 0) {
          reject(this.#classifyProcessError(stderr.trim() || `provider exited with code ${code}`));
          return;
        }
        resolve({
          provider: this.name,
          content: stdout.trim(),
          raw: stdout,
        });
      });

      child.stdin.write(prompt);
      child.stdin.end();
    });
  }

  async *runEvents(request: Request): AsyncIterable<Event> {
    yield { type: "started", provider: this.name };
    try {
      const response = await this.run(request);
      yield { type: "done", provider: this.name, response };
    } catch (error) {
      yield {
        type: "error",
        provider: this.name,
        error: error instanceof Error ? error.message : String(error),
      };
      throw error;
    }
  }

  #classifyProcessError(message: string): MuxaiError {
    const lowered = message.toLowerCase();
    if (lowered.includes("unauthorized") || lowered.includes("auth")) {
      return new MuxaiError({
        code: "auth_error",
        message,
        provider: this.name,
        operation: "CliProvider.run",
        temporary: false,
      });
    }
    if (lowered.includes("rate limit") || lowered.includes("too many requests")) {
      return new MuxaiError({
        code: "rate_limit_error",
        message,
        provider: this.name,
        operation: "CliProvider.run",
        temporary: true,
      });
    }
    return new MuxaiError({
      code: "provider_exec_error",
      message,
      provider: this.name,
      operation: "CliProvider.run",
      temporary: true,
    });
  }
}

function formatPrompt(request: Request): string {
  const lines: string[] = [];
  if (request.systemPrompt) {
    lines.push("[system]", request.systemPrompt, "");
  }
  for (const message of request.messages) {
    lines.push(formatMessage(message));
  }
  return lines.join("\n").trim();
}

function formatMessage(message: Message): string {
  const maybeName = message.name ? `(${message.name})` : "";
  return `[${message.role}]${maybeName}\n${message.content}\n`;
}
