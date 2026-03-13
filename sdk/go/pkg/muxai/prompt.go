package muxai

import (
	"strings"
)

func BuildPrompt(req Request) string {
	var b strings.Builder

	if req.SystemPrompt != "" {
		b.WriteString("[system]\n")
		b.WriteString(req.SystemPrompt)
		b.WriteString("\n\n")
	}

	for _, m := range req.Messages {
		b.WriteString("[")
		b.WriteString(string(m.Role))
		b.WriteString("]")
		if m.Name != "" {
			b.WriteString("(")
			b.WriteString(m.Name)
			b.WriteString(")")
		}
		b.WriteString("\n")
		b.WriteString(m.Content)
		b.WriteString("\n\n")
	}

	if len(req.Tools) > 0 {
		b.WriteString("[tools]\n")
		for _, tool := range req.Tools {
			b.WriteString("- ")
			b.WriteString(tool.Name)
			if tool.Description != "" {
				b.WriteString(": ")
				b.WriteString(tool.Description)
			}
			b.WriteString("\n")
		}
	}

	return strings.TrimSpace(b.String())
}
