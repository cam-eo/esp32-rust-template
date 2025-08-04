use esp_idf_svc::wifi::{EspWifi, WifiConfiguration};
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use anyhow::Result;
use log::{info, warn, error};

/// WiFi Task for handling WiFi operations in background
pub struct WifiTask {
    wifi: Option<EspWifi<'static>>,
    ssid: String,
    password: String,
}

impl WifiTask {
    /// Create a new WiFi task
    pub fn new(ssid: String, password: String) -> Self {
        Self {
            wifi: None,
            ssid,
            password,
        }
    }

    /// Initialize WiFi
    pub fn init(&mut self) -> Result<()> {
        let nvs = EspDefaultNvsPartition::take()
            .map_err(|e| {
                error!("Failed to get NVS partition: {:?}", e);
                anyhow::anyhow!("NVS partition acquisition failed")
            })?;

        let sysloop = EspSystemEventLoop::take()
            .map_err(|e| {
                error!("Failed to get system event loop: {:?}", e);
                anyhow::anyhow!("System event loop acquisition failed")
            })?;

        let wifi = EspWifi::new(
            esp_idf_hal::peripherals::Peripherals::take()
                .map_err(|e| {
                    error!("Failed to acquire peripherals: {:?}", e);
                    anyhow::anyhow!("Peripheral acquisition failed")
                })?
                .modem,
            sysloop.clone(),
            Some(nvs),
        )
        .map_err(|e| {
            error!("Failed to create WiFi instance: {:?}", e);
            anyhow::anyhow!("WiFi instance creation failed")
        })?;

        self.wifi = Some(wifi);
        info!("WiFi initialized successfully");
        Ok(())
    }

    /// Connect to WiFi network
    pub fn connect(&mut self) -> Result<()> {
        if let Some(wifi) = &mut self.wifi {
            let wifi_configuration = WifiConfiguration::Client(esp_idf_svc::wifi::ClientConfiguration {
                ssid: self.ssid.clone().into(),
                password: self.password.clone().into(),
                ..Default::default()
            });

            wifi.set_configuration(&wifi_configuration)
                .map_err(|e| {
                    error!("Failed to set WiFi configuration: {:?}", e);
                    anyhow::anyhow!("WiFi configuration failed")
                })?;

            wifi.start()
                .map_err(|e| {
                    error!("Failed to start WiFi: {:?}", e);
                    anyhow::anyhow!("WiFi start failed")
                })?;

            info!("WiFi started, attempting to connect...");

            // Wait for connection
            loop {
                let status = wifi.get_status();
                match status {
                    esp_idf_svc::wifi::WifiStatus::Started => {
                        info!("WiFi started, waiting for connection...");
                    }
                    esp_idf_svc::wifi::WifiStatus::Connected => {
                        info!("WiFi connected successfully!");
                        break;
                    }
                    esp_idf_svc::wifi::WifiStatus::Failed => {
                        error!("WiFi connection failed");
                        return Err(anyhow::anyhow!("WiFi connection failed"));
                    }
                    _ => {
                        warn!("WiFi status: {:?}", status);
                    }
                }

                esp_idf_hal::delay::FreeRtos::delay_ms(100);
            }

            Ok(())
        } else {
            Err(anyhow::anyhow!("WiFi not initialized"))
        }
    }

    /// Disconnect from WiFi
    pub fn disconnect(&mut self) -> Result<()> {
        if let Some(wifi) = &mut self.wifi {
            wifi.stop()
                .map_err(|e| {
                    error!("Failed to stop WiFi: {:?}", e);
                    anyhow::anyhow!("WiFi stop failed")
                })?;

            info!("WiFi disconnected");
            Ok(())
        } else {
            Err(anyhow::anyhow!("WiFi not initialized"))
        }
    }

    /// Get WiFi status
    pub fn get_status(&self) -> Result<esp_idf_svc::wifi::WifiStatus> {
        if let Some(wifi) = &self.wifi {
            Ok(wifi.get_status())
        } else {
            Err(anyhow::anyhow!("WiFi not initialized"))
        }
    }

    /// Get IP address
    pub fn get_ip(&self) -> Result<Option<std::net::IpAddr>> {
        if let Some(wifi) = &self.wifi {
            Ok(wifi.get_ip_info().map(|info| info.subnet.gateway))
        } else {
            Err(anyhow::anyhow!("WiFi not initialized"))
        }
    }
} 