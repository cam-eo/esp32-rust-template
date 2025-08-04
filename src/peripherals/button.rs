use esp_idf_hal::gpio::{Gpio5, PinDriver, Pull};
use anyhow::Result;
use log::error;

/// Button Controller with debouncing
pub struct ButtonController {
    button: PinDriver<'static, Gpio5, esp_idf_hal::gpio::Input>,
    last_state: bool,
    debounce_time: u32,
    last_press_time: u32,
}

impl ButtonController {
    /// Create a new button controller
    pub fn new(button_pin: Gpio5) -> Result<Self> {
        let mut button = PinDriver::input(button_pin)
            .map_err(|e| {
                error!("Failed to configure button pin: {:?}", e);
                anyhow::anyhow!("Button pin configuration failed")
            })?;

        // Enable pull-up resistor
        button.set_pull(Pull::Up)
            .map_err(|e| {
                error!("Failed to enable pull-up on button: {:?}", e);
                anyhow::anyhow!("Button pull-up configuration failed")
            })?;

        Ok(Self {
            button,
            last_state: false,
            debounce_time: 50, // 50ms debounce
            last_press_time: 0,
        })
    }

    /// Check if button is currently pressed (with debouncing)
    pub fn is_pressed(&mut self) -> Result<bool> {
        let current_state = self.button.is_low()
            .map_err(|e| {
                error!("Failed to read button state: {:?}", e);
                anyhow::anyhow!("Button state reading failed")
            })?;

        // Simple debouncing logic
        if current_state != self.last_state {
            // State changed, update last state
            self.last_state = current_state;
            return Ok(current_state);
        }

        Ok(false) // No state change
    }

    /// Check if button is currently pressed (raw reading, no debouncing)
    pub fn is_pressed_raw(&self) -> Result<bool> {
        self.button.is_low()
            .map_err(|e| {
                error!("Failed to read button state: {:?}", e);
                anyhow::anyhow!("Button state reading failed")
            })
    }

    /// Wait for button press with timeout
    pub fn wait_for_press(&mut self, timeout_ms: u32) -> Result<bool> {
        let start_time = esp_idf_hal::sys::esp_timer_get_time() / 1000; // Convert to ms
        
        loop {
            if self.is_pressed()? {
                return Ok(true);
            }

            let current_time = esp_idf_hal::sys::esp_timer_get_time() / 1000;
            if current_time - start_time > timeout_ms {
                return Ok(false);
            }

            esp_idf_hal::delay::FreeRtos::delay_ms(10);
        }
    }

    /// Set debounce time in milliseconds
    pub fn set_debounce_time(&mut self, time_ms: u32) {
        self.debounce_time = time_ms;
    }

    /// Get current debounce time
    pub fn get_debounce_time(&self) -> u32 {
        self.debounce_time
    }
} 