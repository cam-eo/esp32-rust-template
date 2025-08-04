# ESP32 Template API Documentation

This document describes the API and usage patterns for the ESP32 embedded development template.

## Core Modules

### Peripherals

#### LED Controller

The `LedController` provides a high-level interface for managing multiple LEDs.

```rust
use esp32_template::peripherals::LedController;

// Create a new LED controller
let mut led_controller = LedController::new(pins.gpio2, pins.gpio4)?;

// Set individual LEDs
led_controller.set_led1(true)?;
led_controller.set_led2(false)?;

// Set both LEDs to the same state
led_controller.set_state(true)?;

// Toggle LEDs
led_controller.toggle_led1()?;
led_controller.toggle_led2()?;

// Get current LED states
let led1_state = led_controller.get_led1_state()?;
let led2_state = led_controller.get_led2_state()?;
```

#### Button Controller

The `ButtonController` provides debounced button input handling.

```rust
use esp32_template::peripherals::ButtonController;

// Create a new button controller
let mut button_controller = ButtonController::new(pins.gpio5)?;

// Check if button is pressed (with debouncing)
let is_pressed = button_controller.is_pressed()?;

// Check raw button state (no debouncing)
let raw_state = button_controller.is_pressed_raw()?;

// Wait for button press with timeout
let pressed = button_controller.wait_for_press(5000)?; // 5 second timeout

// Configure debounce time
button_controller.set_debounce_time(100); // 100ms debounce
```

### Tasks

#### WiFi Task

The `WifiTask` provides WiFi connectivity management.

```rust
use esp32_template::tasks::WifiTask;

// Create a new WiFi task
let mut wifi_task = WifiTask::new(
    "your_ssid".to_string(),
    "your_password".to_string()
);

// Initialize WiFi
wifi_task.init()?;

// Connect to network
wifi_task.connect()?;

// Check connection status
let status = wifi_task.get_status()?;

// Get IP address
let ip = wifi_task.get_ip()?;

// Disconnect
wifi_task.disconnect()?;
```

#### Sensor Task

The `SensorTask` provides sensor data management.

```rust
use esp32_template::tasks::SensorTask;

// Create a new sensor task
let mut sensor_task = SensorTask::new();

// Start sensor task
sensor_task.start()?;

// Read individual sensors
let temperature = sensor_task.read_temperature()?;
let humidity = sensor_task.read_humidity()?;
let pressure = sensor_task.read_pressure()?;

// Read all sensors at once
let (temp, hum, press) = sensor_task.read_all_sensors()?;

// Get current values without reading from hardware
let (temp, hum, press) = sensor_task.get_current_values();

// Stop sensor task
sensor_task.stop()?;
```

### Utilities

#### Error Handling

```rust
use esp32_template::utils::error_handler;

// Handle errors with consistent logging
let result = some_operation();
let handled_result = handle_error(result, "operation context");

// Handle errors with custom recovery
let result = some_operation();
let recovered_result = handle_error_with_recovery(
    result,
    "operation context",
    || fallback_operation()
);

// Retry with exponential backoff
let result = retry_with_backoff(
    || some_operation(),
    3,    // max attempts
    1000  // initial delay in ms
);
```

#### Time Utilities

```rust
use esp32_template::utils::time_utils;

// Get system uptime
let uptime_ms = get_uptime_ms();
let uptime_s = get_uptime_s();

// Format uptime as string
let uptime_str = format_uptime(); // "HH:MM:SS"

// Sleep functions
sleep_ms(1000); // sleep for 1 second
sleep_us(1000); // sleep for 1 millisecond

// Timer functionality
let mut timer = Timer::new(5000); // 5 second timer
if timer.has_expired() {
    println!("Timer expired!");
}
let remaining = timer.remaining_ms();
timer.reset();
```

#### Math Utilities

```rust
use esp32_template::utils::math_utils;

// Map value from one range to another
let mapped = map_range(50.0, 0.0, 100.0, 0.0, 1.0); // 0.5

// Clamp value between min and max
let clamped = clamp(150.0, 0.0, 100.0); // 100.0

// Linear interpolation
let interpolated = lerp(0.0, 100.0, 0.5); // 50.0

// Smooth step interpolation
let smooth = smooth_step(0.0, 100.0, 50.0);

// Angle conversions
let radians = degrees_to_radians(180.0);
let degrees = radians_to_degrees(std::f32::consts::PI);

// Statistical functions
let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
let avg = average(&values)?;
let std_dev = standard_deviation(&values)?;

// Range validation
let validated = validate_range(50.0, 0.0, 100.0, "temperature")?;
```

## Configuration

### ESP-IDF Configuration

Edit `config/sdkconfig.defaults` to customize ESP-IDF settings:

```bash
# Main task stack size
CONFIG_ESP_MAIN_TASK_STACK_SIZE=8000

# FreeRTOS tick frequency
CONFIG_FREERTOS_HZ=1000

# WiFi configuration
CONFIG_ESP32_WIFI_STATIC_RX_BUFFER_NUM=10
CONFIG_ESP32_WIFI_DYNAMIC_RX_BUFFER_NUM=32

# Logging configuration
CONFIG_LOG_DEFAULT_LEVEL_INFO=y
CONFIG_LOG_MAXIMUM_LEVEL_VERBOSE=y
```

### Rust Configuration

Edit `Cargo.toml` to customize Rust settings:

```toml
[package]
name = "your-project-name"
version = "0.1.0"

[dependencies]
esp-idf-hal = "0.45.2"
esp-idf-svc = "0.51.0"
log = "0.4"
anyhow = "1.0.98"

[features]
wifi = ["esp-idf-svc/wifi"]
bluetooth = ["esp-idf-svc/bt"]
embassy = ["esp-idf-svc/embassy-time-driver", "esp-idf-svc/embassy-sync"]
```

## Error Handling Patterns

### Result Types

All functions that can fail return `Result<T, anyhow::Error>`:

```rust
fn some_operation() -> Result<()> {
    // Your code here
    Ok(())
}
```

### Error Propagation

Use the `?` operator for error propagation:

```rust
fn main() -> Result<()> {
    let peripherals = Peripherals::take()?;
    let led_controller = LedController::new(pins.gpio2, pins.gpio4)?;
    led_controller.set_state(true)?;
    Ok(())
}
```

### Custom Error Handling

```rust
match some_operation() {
    Ok(value) => {
        info!("Operation successful: {:?}", value);
    }
    Err(e) => {
        error!("Operation failed: {:?}", e);
        // Handle error appropriately
    }
}
```

## Logging

The template uses the `log` crate for logging:

```rust
use log::{info, warn, error, debug, trace};

info!("Application started");
warn!("High temperature detected");
error!("Failed to initialize peripheral");
debug!("Debug information");
trace!("Trace information");
```

## Best Practices

### 1. Error Handling

- Always use `Result` types for functions that can fail
- Use the `?` operator for error propagation
- Provide meaningful error messages
- Use the error handling utilities for consistent logging

### 2. Resource Management

- Use `Peripherals::take()` to acquire hardware resources
- Properly initialize peripherals before use
- Handle initialization failures gracefully

### 3. Logging

- Use appropriate log levels
- Include context in log messages
- Use structured logging for complex data

### 4. Performance

- Use release builds for production
- Minimize heap allocations
- Use appropriate stack sizes for tasks

### 5. Testing

- Write unit tests for utility functions
- Use integration tests for hardware-dependent code
- Mock hardware interfaces for testing

## Examples

### Basic LED Blink

```rust
use esp32_template::peripherals::LedController;

fn main() -> Result<()> {
    let peripherals = Peripherals::take()?;
    let mut led_controller = LedController::new(pins.gpio2, pins.gpio4)?;

    loop {
        led_controller.set_state(true)?;
        sleep_ms(1000);
        led_controller.set_state(false)?;
        sleep_ms(1000);
    }
}
```

### Button-Controlled LED

```rust
use esp32_template::peripherals::{LedController, ButtonController};

fn main() -> Result<()> {
    let peripherals = Peripherals::take()?;
    let mut led_controller = LedController::new(pins.gpio2, pins.gpio4)?;
    let mut button_controller = ButtonController::new(pins.gpio5)?;

    let mut led_state = false;
    let mut last_button_state = false;

    loop {
        let button_pressed = button_controller.is_pressed()?;

        if button_pressed && !last_button_state {
            led_state = !led_state;
            led_controller.set_state(led_state)?;
        }

        last_button_state = button_pressed;
        sleep_ms(50);
    }
}
```

### WiFi Connection

```rust
use esp32_template::tasks::WifiTask;

fn main() -> Result<()> {
    let mut wifi_task = WifiTask::new(
        "your_ssid".to_string(),
        "your_password".to_string()
    );

    wifi_task.init()?;
    wifi_task.connect()?;

    info!("Connected to WiFi!");

    // Your application code here

    Ok(())
}
```
