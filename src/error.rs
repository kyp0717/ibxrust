use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Connection error: {0}")]
    ConnectionError(String),
    
    #[error("TWS API error: {0}")]
    TwsApiError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Trading error: {0}")]
    TradingError(String),
    
    #[error("Market data error: {0}")]
    MarketDataError(String),
    
    #[error("Order error: {0}")]
    OrderError(String),
    
    #[error("Position error: {0}")]
    PositionError(String),
    
    #[error("Invalid symbol: {0}")]
    InvalidSymbol(String),
    
    #[error("Insufficient funds")]
    InsufficientFunds,
    
    #[error("Market closed")]
    MarketClosed,
    
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    
    #[error("User cancelled operation")]
    UserCancelled,
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("Timeout error: operation timed out after {0} seconds")]
    TimeoutError(u64),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<ibapi::Error> for AppError {
    fn from(err: ibapi::Error) -> Self {
        AppError::TwsApiError(err.to_string())
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Unknown(err.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;

pub trait ErrorHandler {
    fn handle_error(&self, error: &AppError) -> ErrorAction;
}

#[derive(Debug, Clone)]
pub enum ErrorAction {
    Retry,
    RetryWithDelay(u64),
    Abort,
    Continue,
    Reconnect,
    AskUser,
}

pub struct DefaultErrorHandler;

impl ErrorHandler for DefaultErrorHandler {
    fn handle_error(&self, error: &AppError) -> ErrorAction {
        match error {
            AppError::ConnectionError(_) => ErrorAction::Reconnect,
            AppError::TwsApiError(_) => ErrorAction::RetryWithDelay(5),
            AppError::RateLimitExceeded => ErrorAction::RetryWithDelay(60),
            AppError::MarketClosed => ErrorAction::Abort,
            AppError::InsufficientFunds => ErrorAction::Abort,
            AppError::UserCancelled => ErrorAction::Abort,
            AppError::TimeoutError(_) => ErrorAction::Retry,
            _ => ErrorAction::Abort,
        }
    }
}

#[macro_export]
macro_rules! log_error {
    ($err:expr) => {
        tracing::error!("Error occurred: {}", $err);
    };
    ($err:expr, $msg:expr) => {
        tracing::error!("{}: {}", $msg, $err);
    };
}

#[macro_export]
macro_rules! handle_error {
    ($result:expr) => {
        match $result {
            Ok(val) => val,
            Err(e) => {
                $crate::log_error!(e);
                return Err(e.into());
            }
        }
    };
    ($result:expr, $handler:expr) => {
        match $result {
            Ok(val) => val,
            Err(e) => {
                $crate::log_error!(e);
                let action = $handler.handle_error(&e);
                match action {
                    ErrorAction::Continue => continue,
                    _ => return Err(e.into()),
                }
            }
        }
    };
}