use ibapi::Client;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, warn, error};
use anyhow::{Result, Context};

#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    pub host: String,
    pub port: u16,
    pub client_id: i32,
    pub max_retries: u32,
    pub initial_retry_delay: Duration,
    pub max_retry_delay: Duration,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        ConnectionConfig {
            host: "127.0.0.1".to_string(),
            port: 7500,
            client_id: 1,
            max_retries: 5,
            initial_retry_delay: Duration::from_secs(1),
            max_retry_delay: Duration::from_secs(30),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Error(String),
}

pub struct ConnectionManager {
    config: ConnectionConfig,
    status: ConnectionStatus,
    client: Option<Client>,
}

impl ConnectionManager {
    pub fn new(config: ConnectionConfig) -> Self {
        ConnectionManager {
            config,
            status: ConnectionStatus::Disconnected,
            client: None,
        }
    }

    pub fn with_default_config() -> Self {
        Self::new(ConnectionConfig::default())
    }

    pub async fn connect(&mut self) -> Result<()> {
        self.status = ConnectionStatus::Connecting;
        let connection_url = format!("{}:{}", self.config.host, self.config.port);
        
        info!("Attempting to connect to TWS at {}", connection_url);
        
        match self.connect_with_retry(&connection_url).await {
            Ok(client) => {
                info!("Successfully connected to TWS");
                self.client = Some(client);
                self.status = ConnectionStatus::Connected;
                Ok(())
            }
            Err(e) => {
                error!("Failed to connect after {} retries: {}", self.config.max_retries, e);
                self.status = ConnectionStatus::Error(e.to_string());
                Err(e)
            }
        }
    }

    async fn connect_with_retry(&self, connection_url: &str) -> Result<Client> {
        let mut retry_delay = self.config.initial_retry_delay;
        let mut last_error = None;
        
        for attempt in 1..=self.config.max_retries {
            info!("Connection attempt {} of {}", attempt, self.config.max_retries);
            
            match Client::connect(connection_url, self.config.client_id) {
                Ok(client) => {
                    info!("Connection successful on attempt {}", attempt);
                    return Ok(client);
                }
                Err(e) => {
                    warn!("Connection attempt {} failed: {}", attempt, e);
                    last_error = Some(e);
                    
                    if attempt < self.config.max_retries {
                        info!("Retrying in {:?}", retry_delay);
                        sleep(retry_delay).await;
                        
                        // Exponential backoff with max delay
                        retry_delay = std::cmp::min(
                            retry_delay * 2,
                            self.config.max_retry_delay
                        );
                    }
                }
            }
        }
        
        Err(anyhow::anyhow!(
            "Failed to connect after {} attempts. Last error: {:?}",
            self.config.max_retries,
            last_error
        ))
    }

    pub async fn disconnect(&mut self) {
        if let Some(_client) = self.client.take() {
            info!("Disconnecting from TWS");
            // Note: ibapi Client doesn't have explicit disconnect, it disconnects on drop
            self.status = ConnectionStatus::Disconnected;
        }
    }

    pub fn is_connected(&self) -> bool {
        matches!(self.status, ConnectionStatus::Connected)
    }

    pub fn get_status(&self) -> &ConnectionStatus {
        &self.status
    }

    pub fn get_client(&self) -> Option<&Client> {
        self.client.as_ref()
    }

    pub fn get_client_mut(&mut self) -> Option<&mut Client> {
        self.client.as_mut()
    }

    pub async fn health_check(&mut self) -> Result<bool> {
        if !self.is_connected() {
            return Ok(false);
        }
        
        // Try to get server time as a health check
        if let Some(client) = &self.client {
            match client.server_time() {
                Ok(_) => Ok(true),
                Err(e) => {
                    warn!("Health check failed: {}", e);
                    self.status = ConnectionStatus::Error(format!("Health check failed: {}", e));
                    Ok(false)
                }
            }
        } else {
            Ok(false)
        }
    }

    pub async fn reconnect(&mut self) -> Result<()> {
        info!("Attempting to reconnect to TWS");
        self.disconnect().await;
        sleep(Duration::from_secs(1)).await;
        self.connect().await
    }
}