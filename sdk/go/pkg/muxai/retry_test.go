package muxai

import (
	"errors"
	"testing"
	"time"
)

func TestDeterministicBackoffBounds(t *testing.T) {
	base := 10 * time.Millisecond
	max := 100 * time.Millisecond
	for attempt := 0; attempt < 10; attempt++ {
		got := deterministicBackoff(attempt, base, max)
		if got < base {
			t.Fatalf("delay below base: %s", got)
		}
		if got > max {
			t.Fatalf("delay above max: %s", got)
		}
	}
}

func TestIsRetryable(t *testing.T) {
	if !isRetryable(WrapError(ErrorCodeTransient, ProviderCursor, "Run", errors.New("x"), true)) {
		t.Fatal("transient errors should be retryable")
	}
	if !isRetryable(WrapError(ErrorCodeRateLimit, ProviderCursor, "Run", errors.New("x"), true)) {
		t.Fatal("rate limit errors should be retryable")
	}
	if isRetryable(errors.New("plain")) {
		t.Fatal("plain errors should not be retryable")
	}
}
