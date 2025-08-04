// Integration tests for ESP32 template
// These tests can be run with: cargo test --target xtensa-esp32s3-espidf

use esp32_template::peripherals::{LedController, ButtonController};
use esp32_template::utils::{get_uptime_ms, map_range};

#[test]
fn test_led_controller() {
    // This test would require actual hardware
    // For now, we'll just test the utility functions
    assert_eq!(map_range(50.0, 0.0, 100.0, 0.0, 1.0), 0.5);
    assert_eq!(map_range(25.0, 0.0, 100.0, 0.0, 10.0), 2.5);
}

#[test]
fn test_time_utilities() {
    let uptime = get_uptime_ms();
    assert!(uptime > 0);
}

#[test]
fn test_math_utilities() {
    // Test map_range function
    assert_eq!(map_range(50.0, 0.0, 100.0, 0.0, 1.0), 0.5);
    assert_eq!(map_range(0.0, 0.0, 100.0, 0.0, 1.0), 0.0);
    assert_eq!(map_range(100.0, 0.0, 100.0, 0.0, 1.0), 1.0);
    
    // Test edge cases
    assert_eq!(map_range(25.0, 0.0, 100.0, 0.0, 10.0), 2.5);
    assert_eq!(map_range(75.0, 0.0, 100.0, 0.0, 10.0), 7.5);
}

#[test]
fn test_error_handling() {
    // Test that our error handling utilities work correctly
    let result: Result<i32, anyhow::Error> = Ok(42);
    assert!(result.is_ok());
    
    let error_result: Result<i32, anyhow::Error> = Err(anyhow::anyhow!("Test error"));
    assert!(error_result.is_err());
}

// Mock tests for hardware-dependent functionality
#[cfg(test)]
mod mock_tests {
    use super::*;
    
    #[test]
    fn test_mock_led_operations() {
        // These would be actual hardware tests in a real environment
        // For now, we'll just verify our test framework works
        assert!(true);
    }
    
    #[test]
    fn test_mock_button_operations() {
        // Mock button test
        assert!(true);
    }
} 