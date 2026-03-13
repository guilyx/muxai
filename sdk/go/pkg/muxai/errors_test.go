package muxai

import (
	"errors"
	"testing"
)

func TestWrapAndInspectError(t *testing.T) {
	base := errors.New("boom")
	err := WrapError(ErrorCodeProviderExec, ProviderCursor, "Run", base, true)
	if err == nil {
		t.Fatal("expected wrapped error")
	}
	if !IsCode(err, ErrorCodeProviderExec) {
		t.Fatalf("expected provider exec code, got %v", err)
	}
	if !IsTemporary(err) {
		t.Fatalf("expected temporary true, got false")
	}
}

func TestIsCodeFalseOnPlainError(t *testing.T) {
	if IsCode(errors.New("plain"), ErrorCodeConfig) {
		t.Fatal("plain errors should not match muxai error code")
	}
}
