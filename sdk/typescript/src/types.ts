export type ProviderName = "cursor" | "claude" | "vibe";

export type Role = "system" | "user" | "assistant" | "tool";

export type FinishReason =
  | "stop"
  | "tool_call"
  | "length"
  | "error"
  | "incomplete";

export interface Message {
  role: Role;
  content: string;
  name?: string;
}

export interface ToolDefinition {
  name: string;
  description?: string;
}

export interface ToolCall {
  name: string;
  arguments: string;
}

export interface Usage {
  inputTokens: number;
  outputTokens: number;
  totalTokens: number;
}

export interface Request {
  messages: Message[];
  systemPrompt?: string;
  tools?: ToolDefinition[];
  maxTurns?: number;
  metadata?: Record<string, string>;
}

export interface Response {
  provider: ProviderName;
  content: string;
  raw: string;
  finishReason?: FinishReason;
  toolCalls?: ToolCall[];
  usage?: Usage;
  durationMs?: number;
}

export type EventType = "started" | "delta" | "done" | "error";

export interface Event {
  type: EventType;
  provider: ProviderName;
  delta?: string;
  response?: Response;
  error?: string;
}
