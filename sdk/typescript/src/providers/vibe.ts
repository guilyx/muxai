import type { Provider } from "../provider.js";
import { CliProvider } from "./base.js";

export function createVibeProvider(): Provider {
  return new CliProvider({
    name: "vibe",
    command: "vibe",
  });
}
