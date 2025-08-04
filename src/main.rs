use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::sys::link_patches;
use esp_idf_svc::sys::EspError;
use log::{info, warn, error};
use anyhow::Result;

// Import modules
mod peripherals;
mod tasks;
mod utils;

use peripherals::led::LedController;
use peripherals::button::ButtonController;
use utils::error_handler::handle_error;

/// Main application entry point
fn main() -> Result<()> {
    // Setup ESP-IDF internals
    link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("ESP32 Template Application Starting...");

    // Initialize peripherals
    let peripherals = match Peripherals::take() {
        Ok(p) => p,
        Err(e) => {
            error!("Failed to acquire peripherals: {:?}", e);
            return Err(anyhow::anyhow!("Failed to acquire peripherals"));
        }
    };

    let pins = peripherals.pins;

    // Initialize LED controller
    let mut led_controller = match LedController::new(pins.gpio2, pins.gpio4) {
        Ok(controller) => {
            info!("LED controller initialized successfully");
            controller
        }
        Err(e) => {
            error!("Failed to initialize LED controller: {:?}", e);
            return Err(anyhow::anyhow!("LED controller initialization failed"));
        }
    };

    // Initialize button controller
    let mut button_controller = match ButtonController::new(pins.gpio5) {
        Ok(controller) => {
            info!("Button controller initialized successfully");
            controller
        }
        Err(e) => {
            error!("Failed to initialize button controller: {:?}", e);
            return Err(anyhow::anyhow!("Button controller initialization failed"));
        }
    };

    // Application state
    let mut led_state = false;
    let mut last_button_state = false;

    info!("Application initialized successfully. Starting main loop...");

    // Main application loop
    loop {
        // Read button state
        let button_pressed = match button_controller.is_pressed() {
            Ok(pressed) => pressed,
            Err(e) => {
                warn!("Failed to read button state: {:?}", e);
                false
            }
        };

        // Handle button press (rising edge detection)
        if button_pressed && !last_button_state {
            led_state = !led_state;
            
            match led_controller.set_state(led_state) {
                Ok(_) => {
                    if led_state {
                        info!("LED turned ON");
                    } else {
                        info!("LED turned OFF");
                    }
                }
                Err(e) => {
                    error!("Failed to set LED state: {:?}", e);
                }
            }
        }

        last_button_state = button_pressed;

        // Small delay to prevent busy waiting
        FreeRtos::delay_ms(50);
    }

    #[allow(unreachable_code)]
    Ok(())
} 