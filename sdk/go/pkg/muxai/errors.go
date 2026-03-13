package muxai

import (
	"errors"
	"fmt"
)

type ErrorCode string

const (
	ErrorCodeConfig        ErrorCode = "config_error"
	ErrorCodeAuth          ErrorCode = "auth_error"
	ErrorCodeRateLimit     ErrorCode = "rate_limit_error"
	ErrorCodeTransient     ErrorCode = "transient_error"
	ErrorCodeProviderExec  ErrorCode = "provider_exec_error"
	ErrorCodeProviderParse ErrorCode = "provider_parse_error"
	ErrorCodeTimeout       ErrorCode = "timeout_error"
	ErrorCodeCanceled      ErrorCode = "canceled_error"
)

type Error struct {
	Code      ErrorCode
	Provider  ProviderName
	Operation string
	Temporary bool
	Err       error
}

func (e *Error) Error() string {
	if e == nil {
		return "<nil>"
	}

	base := string(e.Code)
	if e.Provider != "" {
		base = fmt.Sprintf("%s (%s)", base, e.Provider)
	}
	if e.Operation != "" {
		base = fmt.Sprintf("%s during %s", base, e.Operation)
	}
	if e.Err != nil {
		return fmt.Sprintf("%s: %v", base, e.Err)
	}
	return base
}

func (e *Error) Unwrap() error {
	if e == nil {
		return nil
	}
	return e.Err
}

func WrapError(code ErrorCode, provider ProviderName, op string, err error, temporary bool) error {
	if err == nil {
		return nil
	}
	return &Error{
		Code:      code,
		Provider:  provider,
		Operation: op,
		Temporary: temporary,
		Err:       err,
	}
}

func IsCode(err error, code ErrorCode) bool {
	var muxErr *Error
	if !errors.As(err, &muxErr) {
		return false
	}
	return muxErr.Code == code
}

func IsTemporary(err error) bool {
	var muxErr *Error
	if errors.As(err, &muxErr) {
		return muxErr.Temporary
	}
	return false
}
