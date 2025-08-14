use crate::error::{Error, Result};
use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub tws_host: String,
    pub tws_port: u16,
    pub client_id: i32,
    pub paper_trading: bool,
    pub log_level: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv().ok();
        
        let tws_host = env::var("TWS_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let tws_port = env::var("TWS_PORT")
            .unwrap_or_else(|_| "7500".to_string())
            .parse::<u16>()
            .map_err(|e| Error::Config(format!("Invalid TWS_PORT: {}", e)))?;
        let client_id = env::var("CLIENT_ID")
            .unwrap_or_else(|_| "100".to_string())
            .parse::<i32>()
            .map_err(|e| Error::Config(format!("Invalid CLIENT_ID: {}", e)))?;
        let paper_trading = env::var("PAPER_TRADING")
            .unwrap_or_else(|_| "true".to_string())
            .parse::<bool>()
            .unwrap_or(true);
        let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
        
        Ok(Config {
            tws_host,
            tws_port,
            client_id,
            paper_trading,
            log_level,
        })
    }
    
    pub fn connection_url(&self) -> String {
        format!("{}:{}", self.tws_host, self.tws_port)
    }
}