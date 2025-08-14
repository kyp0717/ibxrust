use tracing::{Level, info};
use tracing_subscriber::{
    fmt,
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};
use std::fs;
use std::path::Path;
use anyhow::Result;

pub struct Logger;

impl Logger {
    pub fn init(config: &crate::config::LoggingConfig) -> Result<()> {
        // Create log directory if file output is enabled
        if config.file_output {
            fs::create_dir_all(&config.log_dir)?;
        }
        
        // Set up environment filter based on config
        let env_filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| {
                EnvFilter::new(&config.level)
            });
        
        // Create the subscriber
        let subscriber = tracing_subscriber::registry()
            .with(env_filter)
            .with(fmt::layer().with_target(true));
        
        // Initialize the subscriber
        subscriber.init();
        
        info!("Logging initialized with level: {}", config.level);
        
        Ok(())
    }
    
    pub fn init_with_file(config: &crate::config::LoggingConfig) -> Result<()> {
        // Create log directory if it doesn't exist
        fs::create_dir_all(&config.log_dir)?;
        
        // Generate log file name with timestamp
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let log_file = format!("{}/ibxrust_{}.log", config.log_dir, timestamp);
        
        // Create file appender
        let file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file)?;
        
        // Set up environment filter
        let env_filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| {
                EnvFilter::new(&config.level)
            });
        
        // Create the subscriber with both console and file output
        let subscriber = tracing_subscriber::registry()
            .with(env_filter)
            .with(
                fmt::layer()
                    .with_target(true)
                    .with_thread_names(true)
            )
            .with(
                fmt::layer()
                    .with_writer(file)
                    .with_ansi(false)
                    .with_target(true)
                    .with_thread_names(true)
            );
        
        // Initialize the subscriber
        subscriber.init();
        
        info!("Logging initialized with level: {} - Log file: {}", config.level, log_file);
        
        Ok(())
    }
}

#[macro_export]
macro_rules! log_trade {
    ($action:expr, $symbol:expr, $qty:expr, $price:expr) => {
        tracing::info!(
            target: "trading",
            action = $action,
            symbol = $symbol,
            quantity = $qty,
            price = $price,
            "Trade executed"
        );
    };
}

#[macro_export]
macro_rules! log_pnl {
    ($symbol:expr, $pnl:expr, $percentage:expr) => {
        if $pnl >= 0.0 {
            tracing::info!(
                target: "pnl",
                symbol = $symbol,
                pnl = $pnl,
                percentage = $percentage,
                "P&L Update - PROFIT"
            );
        } else {
            tracing::warn!(
                target: "pnl",
                symbol = $symbol,
                pnl = $pnl,
                percentage = $percentage,
                "P&L Update - LOSS"
            );
        }
    };
}

#[macro_export]
macro_rules! log_connection {
    ($status:expr) => {
        tracing::info!(
            target: "connection",
            status = $status,
            "Connection status changed"
        );
    };
    ($status:expr, $details:expr) => {
        tracing::info!(
            target: "connection",
            status = $status,
            details = $details,
            "Connection status changed"
        );
    };
}

pub fn log_session_start() {
    info!("{}", "=".repeat(50));
    info!("Interactive Brokers Trading App - Session Started");
    info!("Timestamp: {}", chrono::Local::now());
    info!("{}", "=".repeat(50));
}

pub fn log_session_end(final_pnl: f64) {
    info!("{}", "=".repeat(50));
    info!("Session Ended");
    info!("Final P&L: ${:.2}", final_pnl);
    info!("Timestamp: {}", chrono::Local::now());
    info!("{}", "=".repeat(50));
}