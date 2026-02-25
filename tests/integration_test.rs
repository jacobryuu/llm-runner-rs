use llm_runner_rs::{settings::Settings, error::LlmError};

#[test]
fn test_default_settings() {
    let settings = Settings::default();
    assert_eq!(settings.server.host, "127.0.0.1");
    assert_eq!(settings.server.port, 8080);
    assert_eq!(settings.inference.max_tokens, 256);
}

#[test]
fn test_error_status_codes() {
    let err = LlmError::ValidationError("test".to_string());
    assert_eq!(err.status_code(), 400);

    let err = LlmError::RateLimitExceeded;
    assert_eq!(err.status_code(), 429);

    let err = LlmError::Timeout;
    assert_eq!(err.status_code(), 408);
}
