import type { ProviderName } from "./types.js";

export type ErrorCode =
  | "config_error"
  | "auth_error"
  | "rate_limit_error"
  | "transient_error"
  | "provider_exec_error"
  | "provider_parse_error"
  | "timeout_error"
  | "canceled_error";

export class MuxaiError extends Error {
  readonly code: ErrorCode;
  readonly provider?: ProviderName;
  readonly operation: string;
  readonly temporary: boolean;

  constructor(args: {
    code: ErrorCode;
    message: string;
    provider?: ProviderName;
    operation: string;
    temporary?: boolean;
  }) {
    super(args.message);
    this.name = "MuxaiError";
    this.code = args.code;
    this.provider = args.provider;
    this.operation = args.operation;
    this.temporary = args.temporary ?? false;
  }
}
