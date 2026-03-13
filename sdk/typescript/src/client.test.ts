import test from "node:test";
import assert from "node:assert/strict";

import { Client } from "./client.js";
import { MuxaiError } from "./errors.js";
import type { Provider } from "./provider.js";

class FakeProvider implements Provider {
  readonly name: "cursor" | "claude" | "vibe";
  readonly #content: string;

  constructor(name: "cursor" | "claude" | "vibe", content = "ok") {
    this.name = name;
    this.#content = content;
  }

  async run() {
    return {
      provider: this.name,
      content: this.#content,
      raw: this.#content,
    };
  }
}

test("Client.run resolves via default provider", async () => {
  const client = new Client([new FakeProvider("cursor")], {
    defaultProvider: "cursor",
  });
  const response = await client.run({
    messages: [{ role: "user", content: "hello" }],
  });
  assert.equal(response.content, "ok");
});

test("Client.run throws config error for missing provider", async () => {
  const client = new Client([new FakeProvider("claude")], {
    defaultProvider: "claude",
  });
  await assert.rejects(
    client.run(
      { messages: [{ role: "user", content: "hello" }] },
      "cursor",
    ),
    (error: unknown) =>
      error instanceof MuxaiError && error.code === "config_error",
  );
});
