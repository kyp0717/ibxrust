pub mod config;
pub mod connection;
pub mod error;
pub mod logging;
pub mod trade;

// Re-export commonly used types
pub use config::AppConfig;
pub use connection::{ConnectionManager, ConnectionConfig, ConnectionStatus};
pub use error::{AppError, AppResult, ErrorHandler, ErrorAction, DefaultErrorHandler};
pub use trade::{Trade, Stage};