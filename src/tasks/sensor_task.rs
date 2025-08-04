use anyhow::Result;
use log::{info, warn, error};
use esp_idf_hal::delay::FreeRtos;

/// Sensor Task for handling sensor operations in background
pub struct SensorTask {
    temperature: f32,
    humidity: f32,
    pressure: f32,
    is_active: bool,
}

impl SensorTask {
    /// Create a new sensor task
    pub fn new() -> Self {
        Self {
            temperature: 0.0,
            humidity: 0.0,
            pressure: 0.0,
            is_active: false,
        }
    }

    /// Start the sensor task
    pub fn start(&mut self) -> Result<()> {
        info!("Starting sensor task...");
        self.is_active = true;
        
        // Simulate sensor initialization
        self.temperature = 25.0;
        self.humidity = 50.0;
        self.pressure = 1013.25;
        
        info!("Sensor task started successfully");
        Ok(())
    }

    /// Stop the sensor task
    pub fn stop(&mut self) -> Result<()> {
        info!("Stopping sensor task...");
        self.is_active = false;
        info!("Sensor task stopped");
        Ok(())
    }

    /// Read temperature sensor
    pub fn read_temperature(&mut self) -> Result<f32> {
        if !self.is_active {
            return Err(anyhow::anyhow!("Sensor task not active"));
        }

        // Simulate sensor reading with some noise
        let noise = (esp_idf_hal::sys::esp_random() as f32 / u32::MAX as f32 - 0.5) * 2.0;
        self.temperature = 25.0 + noise;
        
        info!("Temperature: {:.1}°C", self.temperature);
        Ok(self.temperature)
    }

    /// Read humidity sensor
    pub fn read_humidity(&mut self) -> Result<f32> {
        if !self.is_active {
            return Err(anyhow::anyhow!("Sensor task not active"));
        }

        // Simulate sensor reading with some noise
        let noise = (esp_idf_hal::sys::esp_random() as f32 / u32::MAX as f32 - 0.5) * 5.0;
        self.humidity = 50.0 + noise;
        
        info!("Humidity: {:.1}%", self.humidity);
        Ok(self.humidity)
    }

    /// Read pressure sensor
    pub fn read_pressure(&mut self) -> Result<f32> {
        if !self.is_active {
            return Err(anyhow::anyhow!("Sensor task not active"));
        }

        // Simulate sensor reading with some noise
        let noise = (esp_idf_hal::sys::esp_random() as f32 / u32::MAX as f32 - 0.5) * 10.0;
        self.pressure = 1013.25 + noise;
        
        info!("Pressure: {:.1} hPa", self.pressure);
        Ok(self.pressure)
    }

    /// Read all sensors
    pub fn read_all_sensors(&mut self) -> Result<(f32, f32, f32)> {
        let temp = self.read_temperature()?;
        let humidity = self.read_humidity()?;
        let pressure = self.read_pressure()?;
        
        Ok((temp, humidity, pressure))
    }

    /// Get current sensor values (without reading from hardware)
    pub fn get_current_values(&self) -> (f32, f32, f32) {
        (self.temperature, self.humidity, self.pressure)
    }

    /// Check if sensor task is active
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    /// Run sensor task loop (for background operation)
    pub fn run_loop(&mut self) -> Result<()> {
        if !self.is_active {
            return Err(anyhow::anyhow!("Sensor task not active"));
        }

        loop {
            match self.read_all_sensors() {
                Ok((temp, humidity, pressure)) => {
                    // Process sensor data here
                    if temp > 30.0 {
                        warn!("High temperature detected: {:.1}°C", temp);
                    }
                    
                    if humidity < 20.0 {
                        warn!("Low humidity detected: {:.1}%", humidity);
                    }
                }
                Err(e) => {
                    error!("Failed to read sensors: {:?}", e);
                }
            }

            // Wait before next reading
            FreeRtos::delay_ms(5000); // 5 seconds
        }
    }
} 