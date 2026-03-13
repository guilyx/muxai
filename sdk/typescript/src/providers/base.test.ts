import assert from "node:assert/strict";
import test from "node:test";

import { MuxaiError } from "../errors.js";
import { CliProvider } from "./base.js";

test("CliProvider succeeds and returns output", async () => {
  const provider = new CliProvider({
    name: "cursor",
    command: "sh",
    cliArgs: ["-c", "cat"],
  });

  const response = await provider.run({
    messages: [{ role: "user", content: "hello" }],
  });

  assert.match(response.content, /hello/);
});

test("CliProvider classifies auth failures", async () => {
  const provider = new CliProvider({
    name: "cursor",
    command: "sh",
    cliArgs: ["-c", "echo unauthorized >&2; exit 1"],
  });

  await assert.rejects(
    provider.run({ messages: [{ role: "user", content: "hello" }] }),
    (error: unknown) =>
      error instanceof MuxaiError && error.code === "auth_error",
  );
});

test("CliProvider enforces timeout", async () => {
  const provider = new CliProvider({
    name: "cursor",
    command: "sh",
    cliArgs: ["-c", "sleep 2; echo done"],
    timeoutMs: 20,
  });

  await assert.rejects(
    provider.run({ messages: [{ role: "user", content: "hello" }] }),
    (error: unknown) =>
      error instanceof MuxaiError && error.code === "timeout_error",
  );
});
