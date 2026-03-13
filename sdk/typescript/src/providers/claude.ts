import type { Provider } from "../provider.js";
import { CliProvider } from "./base.js";

export function createClaudeProvider(): Provider {
  return new CliProvider({
    name: "claude",
    command: "claude",
  });
}
