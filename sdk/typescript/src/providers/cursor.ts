import type { Provider } from "../provider.js";
import { CliProvider } from "./base.js";

export function createCursorProvider(): Provider {
  return new CliProvider({
    name: "cursor",
    command: "cursor-agent",
  });
}
