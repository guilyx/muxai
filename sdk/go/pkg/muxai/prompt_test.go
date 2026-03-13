package muxai

import (
	"strings"
	"testing"
)

func TestBuildPrompt(t *testing.T) {
	prompt := BuildPrompt(Request{
		SystemPrompt: "be concise",
		Messages: []Message{
			{Role: RoleUser, Content: "hello"},
			{Role: RoleAssistant, Content: "hi"},
		},
		Tools: []ToolDefinition{
			{Name: "search", Description: "search docs"},
		},
	})

	if prompt == "" {
		t.Fatal("prompt should not be empty")
	}
	if !strings.Contains(prompt, "[system]") || !strings.Contains(prompt, "[user]") || !strings.Contains(prompt, "[tools]") {
		t.Fatalf("prompt missing expected sections: %q", prompt)
	}
}
