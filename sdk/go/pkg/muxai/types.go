package muxai

import "time"

type ProviderName string

const (
	ProviderCursor ProviderName = "cursor"
	ProviderClaude ProviderName = "claude"
	ProviderVibe   ProviderName = "vibe"
)

type Role string

const (
	RoleSystem    Role = "system"
	RoleUser      Role = "user"
	RoleAssistant Role = "assistant"
	RoleTool      Role = "tool"
)

type FinishReason string

const (
	FinishReasonStop       FinishReason = "stop"
	FinishReasonToolCall   FinishReason = "tool_call"
	FinishReasonLength     FinishReason = "length"
	FinishReasonError      FinishReason = "error"
	FinishReasonIncomplete FinishReason = "incomplete"
)

type Message struct {
	Role    Role
	Content string
	Name    string
}

type ToolDefinition struct {
	Name        string
	Description string
}

type ToolCall struct {
	Name      string
	Arguments string
}

type Usage struct {
	InputTokens  int
	OutputTokens int
	TotalTokens  int
}

type Request struct {
	SystemPrompt string
	Messages     []Message
	Tools        []ToolDefinition
	MaxTurns     int
	Metadata     map[string]string
}

type Response struct {
	Provider     ProviderName
	Content      string
	Raw          string
	FinishReason FinishReason
	Usage        Usage
	ToolCalls    []ToolCall
	Duration     time.Duration
}

type EventType string

const (
	EventTypeStarted EventType = "started"
	EventTypeDelta   EventType = "delta"
	EventTypeDone    EventType = "done"
)

type Event struct {
	Type     EventType
	Provider ProviderName
	Delta    string
	Response *Response
}
