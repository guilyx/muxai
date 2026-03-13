export type ProviderName = "cursor" | "claude" | "vibe";

export type Role = "system" | "user" | "assistant" | "tool";

export interface Message {
  role: Role;
  content: string;
  name?: string;
}

export interface Request {
  messages: Message[];
  systemPrompt?: string;
  maxTurns?: number;
  metadata?: Record<string, string>;
}

export interface Response {
  provider: ProviderName;
  content: string;
  raw: string;
}
