use std::backtrace::Backtrace;
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum RaindropMcpError {
    #[error("Raindrop API error: {0}")]
    RaindropApi(
        String,
        #[source] Option<Box<dyn std::error::Error + Send + Sync>>,
    ),

    #[error("MCP protocol error: {0}")]
    McpProtocol(
        String,
        #[source] Option<Box<dyn std::error::Error + Send + Sync>>,
    ),

    #[error("HTTP request error: {0}")]
    HttpRequest(#[from] reqwest::Error),

    #[error("JSON serialization error: {0}")]
    JsonSerialization(#[from] serde_json::Error),

    #[error("Environment variable error: {0}")]
    EnvironmentVariable(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Rate limit exceeded: {message}")]
    RateLimitExceeded {
        message: String,
        retry_after: Option<u64>,
    },

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl RaindropMcpError {
    /// Returns true if this error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            RaindropMcpError::HttpRequest(_)
                | RaindropMcpError::ServiceUnavailable(_)
                | RaindropMcpError::Timeout(_)
                | RaindropMcpError::RateLimitExceeded { .. }
        )
    }

    /// Returns the retry delay in seconds if applicable
    pub fn retry_after(&self) -> Option<u64> {
        match self {
            RaindropMcpError::RateLimitExceeded { retry_after, .. } => *retry_after,
            RaindropMcpError::ServiceUnavailable(_) => Some(5), // Default 5 seconds
            RaindropMcpError::Timeout(_) => Some(2),            // Default 2 seconds
            _ => None,
        }
    }

    /// Logs the error with full context and backtrace
    pub fn log_error(&self) {
        let backtrace = Backtrace::capture();
        error!(
            error = %self,
            backtrace = ?backtrace,
            retryable = self.is_retryable(),
            retry_after = ?self.retry_after(),
            "RaindropMcpError occurred"
        );
    }

    /// Convert to MCP error code for protocol compliance
    pub fn to_mcp_error_code(&self) -> i32 {
        match self {
            RaindropMcpError::InvalidParameter(_) => -32602, // Invalid params
            RaindropMcpError::NotFound(_) => -32601,         // Method not found
            RaindropMcpError::Unauthorized(_) => -32603,     // Internal error (auth)
            RaindropMcpError::RateLimitExceeded { .. } => -32604, // Rate limit
            RaindropMcpError::ServiceUnavailable(_) => -32605, // Service unavailable
            RaindropMcpError::Timeout(_) => -32606,          // Timeout
            _ => -32603,                                     // Generic internal error
        }
    }
}

pub type Result<T> = std::result::Result<T, RaindropMcpError>;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_error_display() {
        let err = RaindropMcpError::RaindropApi("API rate limit exceeded".to_string(), None);
        assert_eq!(
            err.to_string(),
            "Raindrop API error: API rate limit exceeded"
        );

        let err = RaindropMcpError::InvalidParameter("Missing required field".to_string());
        assert_eq!(err.to_string(), "Invalid parameter: Missing required field");

        let err = RaindropMcpError::NotFound("Collection not found".to_string());
        assert_eq!(err.to_string(), "Not found: Collection not found");

        let err = RaindropMcpError::Unauthorized("Invalid token".to_string());
        assert_eq!(err.to_string(), "Unauthorized: Invalid token");

        let err = RaindropMcpError::RateLimitExceeded {
            message: "Too many requests".to_string(),
            retry_after: Some(60),
        };
        assert_eq!(err.to_string(), "Rate limit exceeded: Too many requests");
    }

    #[test]
    fn test_error_from_io() {
        // Test IO error conversion
        let io_err =
            std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "Connection refused");

        let mcp_err: RaindropMcpError = io_err.into();
        match mcp_err {
            RaindropMcpError::Io(_) => {
                // Expected
            }
            _ => panic!("Expected Io error variant"),
        }
    }

    #[test]
    fn test_error_from_serde_json() {
        let json_str = "{invalid json}";
        let serde_err = serde_json::from_str::<serde_json::Value>(json_str).unwrap_err();

        let mcp_err: RaindropMcpError = serde_err.into();
        match mcp_err {
            RaindropMcpError::JsonSerialization(_) => {
                // Expected
            }
            _ => panic!("Expected JsonSerialization error variant"),
        }
    }

    #[test]
    fn test_result_type() {
        fn test_fn() -> Result<String> {
            Ok("success".to_string())
        }

        fn test_error_fn() -> Result<String> {
            Err(RaindropMcpError::Unknown("test error".to_string()))
        }

        assert!(test_fn().is_ok());
        assert!(test_error_fn().is_err());
    }

    #[test]
    fn test_retryable_errors() {
        let err = RaindropMcpError::ServiceUnavailable("Service is down".to_string());
        assert!(err.is_retryable());
        assert_eq!(err.retry_after(), Some(5));

        let err = RaindropMcpError::Timeout("Request timed out".to_string());
        assert!(err.is_retryable());
        assert_eq!(err.retry_after(), Some(2));

        let err = RaindropMcpError::RateLimitExceeded {
            message: "Too many requests".to_string(),
            retry_after: Some(30),
        };
        assert!(err.is_retryable());
        assert_eq!(err.retry_after(), Some(30));

        let err = RaindropMcpError::InvalidParameter("Bad input".to_string());
        assert!(!err.is_retryable());
        assert_eq!(err.retry_after(), None);
    }

    #[test]
    fn test_mcp_error_codes() {
        let err = RaindropMcpError::InvalidParameter("Bad param".to_string());
        assert_eq!(err.to_mcp_error_code(), -32602);

        let err = RaindropMcpError::NotFound("Not found".to_string());
        assert_eq!(err.to_mcp_error_code(), -32601);

        let err = RaindropMcpError::Unauthorized("Unauthorized".to_string());
        assert_eq!(err.to_mcp_error_code(), -32603);

        let err = RaindropMcpError::RateLimitExceeded {
            message: "Rate limited".to_string(),
            retry_after: None,
        };
        assert_eq!(err.to_mcp_error_code(), -32604);
    }
}
