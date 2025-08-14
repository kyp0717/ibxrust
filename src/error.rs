use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Connection error: {0}")]
    Connection(String),
    
    #[error("Market data error: {0}")]
    MarketData(String),
    
    #[error("Order error: {0}")]
    Order(String),
    
    #[error("Position error: {0}")]
    Position(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("TWS API error: {0}")]
    TwsApi(#[from] ibapi::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Environment variable error: {0}")]
    Env(#[from] std::env::VarError),
    
    #[error("Other error: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, Error>;