import { spawn } from "node:child_process";

import { MuxaiError } from "../errors.js";
import type { Provider } from "../provider.js";
import type { Message, ProviderName, Request, Response } from "../types.js";

export class CliProvider implements Provider {
  readonly name: ProviderName;
  readonly #command: string;
  readonly #args: string[];
  readonly #env?: Record<string, string | undefined>;

  constructor(args: {
    name: ProviderName;
    command: string;
    cliArgs?: string[];
    env?: Record<string, string | undefined>;
  }) {
    this.name = args.name;
    this.#command = args.command;
    this.#args = args.cliArgs ?? [];
    this.#env = args.env;
  }

  async run(request: Request): Promise<Response> {
    const prompt = formatPrompt(request);
    return await new Promise<Response>((resolve, reject) => {
      const child = spawn(this.#command, this.#args, {
        env: this.#env,
      });
      let stdout = "";
      let stderr = "";

      child.stdout.on("data", (chunk) => {
        stdout += String(chunk);
      });
      child.stderr.on("data", (chunk) => {
        stderr += String(chunk);
      });
      child.on("error", (error) => {
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
        if (code && code !== 0) {
          reject(
            new MuxaiError({
              code: "provider_exec_error",
              message: stderr.trim() || `provider exited with code ${code}`,
              provider: this.name,
              operation: "CliProvider.run",
              temporary: true,
            }),
          );
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
