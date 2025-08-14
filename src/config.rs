use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use anyhow::{Result, Context};
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub tws: TwsConfig,
    pub trading: TradingConfig,
    pub ui: UiConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwsConfig {
    pub host: String,
    pub port: u16,
    pub client_id: i32,
    pub paper_trading: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingConfig {
    pub default_position_size: i32,
    pub max_position_size: i32,
    pub require_confirmation: bool,
    pub auto_close_on_target: bool,
    pub target_profit_percent: f64,
    pub stop_loss_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub refresh_rate_ms: u64,
    pub show_colors: bool,
    pub clear_screen: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub file_output: bool,
    pub log_dir: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            tws: TwsConfig {
                host: "127.0.0.1".to_string(),
                port: 7500,
                client_id: 1,
                paper_trading: true,
            },
            trading: TradingConfig {
                default_position_size: 100,
                max_position_size: 1000,
                require_confirmation: true,
                auto_close_on_target: false,
                target_profit_percent: 2.0,
                stop_loss_percent: 1.0,
            },
            ui: UiConfig {
                refresh_rate_ms: 1000,
                show_colors: true,
                clear_screen: true,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                file_output: true,
                log_dir: "logs".to_string(),
            },
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        // Load .env file if it exists
        dotenv().ok();
        
        let mut config = AppConfig::default();
        
        // Override with environment variables if they exist
        if let Ok(host) = env::var("TWS_HOST") {
            config.tws.host = host;
        }
        
        if let Ok(port) = env::var("TWS_PORT") {
            config.tws.port = port.parse()
                .context("Invalid TWS_PORT value")?;
        }
        
        if let Ok(client_id) = env::var("TWS_CLIENT_ID") {
            config.tws.client_id = client_id.parse()
                .context("Invalid TWS_CLIENT_ID value")?;
        }
        
        if let Ok(paper) = env::var("PAPER_TRADING") {
            config.tws.paper_trading = paper.to_lowercase() == "true" || paper == "1";
        }
        
        if let Ok(position_size) = env::var("DEFAULT_POSITION_SIZE") {
            config.trading.default_position_size = position_size.parse()
                .context("Invalid DEFAULT_POSITION_SIZE value")?;
        }
        
        if let Ok(max_position) = env::var("MAX_POSITION_SIZE") {
            config.trading.max_position_size = max_position.parse()
                .context("Invalid MAX_POSITION_SIZE value")?;
        }
        
        if let Ok(confirmation) = env::var("REQUIRE_CONFIRMATION") {
            config.trading.require_confirmation = confirmation.to_lowercase() == "true" || confirmation == "1";
        }
        
        if let Ok(refresh_rate) = env::var("UI_REFRESH_RATE_MS") {
            config.ui.refresh_rate_ms = refresh_rate.parse()
                .context("Invalid UI_REFRESH_RATE_MS value")?;
        }
        
        if let Ok(colors) = env::var("SHOW_COLORS") {
            config.ui.show_colors = colors.to_lowercase() == "true" || colors == "1";
        }
        
        if let Ok(log_level) = env::var("LOG_LEVEL") {
            config.logging.level = log_level;
        }
        
        if let Ok(log_dir) = env::var("LOG_DIR") {
            config.logging.log_dir = log_dir;
        }
        
        info!("Configuration loaded: {:?}", config);
        
        Ok(config)
    }
    
    pub fn validate(&self) -> Result<()> {
        // Validate configuration values
        if self.tws.port == 0 {
            return Err(anyhow::anyhow!("TWS port cannot be 0"));
        }
        
        if self.trading.max_position_size < self.trading.default_position_size {
            return Err(anyhow::anyhow!("Max position size must be >= default position size"));
        }
        
        if self.trading.target_profit_percent <= 0.0 {
            return Err(anyhow::anyhow!("Target profit percent must be positive"));
        }
        
        if self.trading.stop_loss_percent <= 0.0 {
            return Err(anyhow::anyhow!("Stop loss percent must be positive"));
        }
        
        if self.ui.refresh_rate_ms == 0 {
            return Err(anyhow::anyhow!("UI refresh rate must be positive"));
        }
        
        Ok(())
    }
    
    pub fn connection_url(&self) -> String {
        format!("{}:{}", self.tws.host, self.tws.port)
    }
}