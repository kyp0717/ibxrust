use ibxrust::{
    AppConfig, 
    ConnectionManager, 
    ConnectionConfig,
    logging::{Logger, log_session_start, log_session_end},
};
use ibapi::prelude::*;
use tracing::{info, error, warn};
use anyhow::Result;
use clap::Parser;
use std::io::{self, Write};
use futures::StreamExt;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Stock ticker symbol to trade
    #[arg(short, long)]
    symbol: Option<String>,
    
    /// TWS connection port
    #[arg(short, long, default_value = "7500")]
    port: u16,
    
    /// Enable paper trading mode
    #[arg(long, default_value = "true")]
    paper: bool,
    
    /// Configuration file path
    #[arg(short, long)]
    config: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();
    
    // Load configuration
    let mut config = AppConfig::load()?;
    
    // Override config with command line args
    if args.port != 7500 {
        config.tws.port = args.port;
    }
    config.tws.paper_trading = args.paper;
    
    // Validate configuration
    config.validate()?;
    
    // Initialize logging
    if config.logging.file_output {
        Logger::init_with_file(&config.logging)?;
    } else {
        Logger::init(&config.logging)?;
    }
    
    log_session_start();
    
    // Display mode
    if config.tws.paper_trading {
        warn!("PAPER TRADING MODE ENABLED");
    } else {
        warn!("LIVE TRADING MODE - USE WITH CAUTION");
    }
    
    // Get ticker symbol
    let symbol = match args.symbol {
        Some(s) => s.to_uppercase(),
        None => {
            print!("Enter ticker symbol: ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            input.trim().to_uppercase()
        }
    };
    
    info!("Trading symbol: {}", symbol);
    
    // Create connection manager
    let conn_config = ConnectionConfig {
        host: config.tws.host.clone(),
        port: config.tws.port,
        client_id: config.tws.client_id,
        max_retries: 5,
        initial_retry_delay: std::time::Duration::from_secs(1),
        max_retry_delay: std::time::Duration::from_secs(30),
    };
    
    let mut connection_manager = ConnectionManager::new(conn_config);
    
    // Connect to TWS
    info!("Connecting to TWS at {}", config.connection_url());
    
    match connection_manager.connect().await {
        Ok(_) => {
            info!("Successfully connected to TWS");
            
            // Perform health check
            if let Ok(healthy) = connection_manager.health_check().await {
                if healthy {
                    info!("Connection health check passed");
                } else {
                    warn!("Connection health check failed");
                }
            }
            
            // Get client reference
            if let Some(client) = connection_manager.get_client() {
                // Example: Get current price for the symbol
                let contract = Contract::stock(&symbol);
                
                info!("Requesting market data for {}", symbol);
                
                // Request real-time bars (this is just an example)
                match client.realtime_bars(&contract, BarSize::Sec5, WhatToShow::Trades, false).await {
                    Ok(mut bars_stream) => {
                        info!("Receiving real-time data for {}", symbol);
                        
                        // Just show a few bars as example
                        let mut count = 0;
                        while let Some(bar) = bars_stream.next().await {
                            match bar {
                                Ok(bar) => {
                                    info!("Price: ${:.2} | Volume: {} | Time: {:?}", 
                                        bar.close, bar.volume, bar.time);
                                    count += 1;
                                    if count >= 5 {
                                        break;
                                    }
                                }
                                Err(e) => {
                                    error!("Error receiving bar: {}", e);
                                    break;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to request market data: {}", e);
                    }
                }
            }
            
            // Disconnect
            connection_manager.disconnect().await;
        }
        Err(e) => {
            error!("Failed to connect to TWS: {}", e);
            error!("Please ensure TWS or IB Gateway is running and API connections are enabled");
            error!("Check that the port {} is correct", config.tws.port);
        }
    }
    
    log_session_end(0.0);
    
    Ok(())
}