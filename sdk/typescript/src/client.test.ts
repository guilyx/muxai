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

test("Client.runDefault resolves via configured provider", async () => {
  const client = new Client([new FakeProvider("claude", "default-ok")], {
    defaultProvider: "claude",
  });
  const response = await client.runDefault({
    messages: [{ role: "user", content: "hello" }],
  });
  assert.equal(response.content, "default-ok");
});

test("Client.runEvents yields started and done", async () => {
  const client = new Client([new FakeProvider("vibe", "stream-ok")], {
    defaultProvider: "vibe",
  });
  const events = [];
  for await (const event of client.runEvents({
    messages: [{ role: "user", content: "hello" }],
  })) {
    events.push(event.type);
  }
  assert.deepEqual(events, ["started", "done"]);
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
