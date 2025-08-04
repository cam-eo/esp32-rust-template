use esp_idf_hal::gpio::{Gpio2, Gpio4, PinDriver};
use anyhow::Result;
use log::error;

/// LED Controller for managing multiple LEDs
pub struct LedController {
    led1: PinDriver<'static, Gpio2, esp_idf_hal::gpio::Output>,
    led2: PinDriver<'static, Gpio4, esp_idf_hal::gpio::Output>,
}

impl LedController {
    /// Create a new LED controller with two LEDs
    pub fn new(
        led1_pin: Gpio2,
        led2_pin: Gpio4,
    ) -> Result<Self> {
        let led1 = PinDriver::output(led1_pin)
            .map_err(|e| {
                error!("Failed to configure LED1 pin: {:?}", e);
                anyhow::anyhow!("LED1 pin configuration failed")
            })?;

        let led2 = PinDriver::output(led2_pin)
            .map_err(|e| {
                error!("Failed to configure LED2 pin: {:?}", e);
                anyhow::anyhow!("LED2 pin configuration failed")
            })?;

        Ok(Self { led1, led2 })
    }

    /// Set the state of LED1
    pub fn set_led1(&mut self, state: bool) -> Result<()> {
        match state {
            true => self.led1.set_high(),
            false => self.led1.set_low(),
        }
        .map_err(|e| {
            error!("Failed to set LED1 state: {:?}", e);
            anyhow::anyhow!("LED1 state setting failed")
        })
    }

    /// Set the state of LED2
    pub fn set_led2(&mut self, state: bool) -> Result<()> {
        match state {
            true => self.led2.set_high(),
            false => self.led2.set_low(),
        }
        .map_err(|e| {
            error!("Failed to set LED2 state: {:?}", e);
            anyhow::anyhow!("LED2 state setting failed")
        })
    }

    /// Set both LEDs to the same state
    pub fn set_state(&mut self, state: bool) -> Result<()> {
        self.set_led1(state)?;
        self.set_led2(state)?;
        Ok(())
    }

    /// Toggle LED1
    pub fn toggle_led1(&mut self) -> Result<()> {
        self.led1.toggle()
            .map_err(|e| {
                error!("Failed to toggle LED1: {:?}", e);
                anyhow::anyhow!("LED1 toggle failed")
            })
    }

    /// Toggle LED2
    pub fn toggle_led2(&mut self) -> Result<()> {
        self.led2.toggle()
            .map_err(|e| {
                error!("Failed to toggle LED2: {:?}", e);
                anyhow::anyhow!("LED2 toggle failed")
            })
    }

    /// Get the current state of LED1
    pub fn get_led1_state(&self) -> Result<bool> {
        self.led1.is_high()
            .map_err(|e| {
                error!("Failed to read LED1 state: {:?}", e);
                anyhow::anyhow!("LED1 state reading failed")
            })
    }

    /// Get the current state of LED2
    pub fn get_led2_state(&self) -> Result<bool> {
        self.led2.is_high()
            .map_err(|e| {
                error!("Failed to read LED2 state: {:?}", e);
                anyhow::anyhow!("LED2 state reading failed")
            })
    }
} 