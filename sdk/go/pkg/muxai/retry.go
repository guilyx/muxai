package muxai

import "time"

func deterministicBackoff(attempt int, baseDelay, maxDelay time.Duration) time.Duration {
	if attempt <= 0 {
		return baseDelay
	}
	if baseDelay <= 0 {
		baseDelay = 50 * time.Millisecond
	}
	if maxDelay <= 0 {
		maxDelay = 3 * time.Second
	}

	delay := baseDelay
	for i := 0; i < attempt; i++ {
		delay *= 2
		if delay > maxDelay {
			delay = maxDelay
			break
		}
	}

	// Deterministic jitter based on attempt.
	jitterStep := time.Duration((attempt*97)%31) * time.Millisecond
	delay += jitterStep
	if delay > maxDelay {
		return maxDelay
	}
	return delay
}

func isRetryable(err error) bool {
	if err == nil {
		return false
	}
	if IsTemporary(err) || IsCode(err, ErrorCodeRateLimit) || IsCode(err, ErrorCodeTransient) {
		return true
	}
	return false
}
