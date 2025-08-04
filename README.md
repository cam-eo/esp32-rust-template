# ESP32 Embedded Development Template

A comprehensive template for ESP32 embedded development using Rust and ESP-IDF.

## Features

- **Rust + ESP-IDF**: Modern embedded development with Rust
- **ESP32-S3 Support**: Optimized for ESP32-S3 but compatible with other ESP32 variants
- **Structured Project**: Clean project structure with proper organization
- **Build Scripts**: Automated build and flash scripts
- **Development Tools**: Pre-configured for efficient development workflow
- **WSL2 Support**: Optimized for Windows with WSL2 development
- **Error Handling**: Robust error handling patterns
- **Logging**: Integrated logging system

## Project Structure

```
template/
├── src/
│   ├── main.rs              # Main application entry point
│   ├── peripherals/          # Peripheral drivers and abstractions
│   ├── tasks/               # FreeRTOS tasks and async code
│   └── utils/               # Utility functions and helpers
├── scripts/
│   ├── flash.sh             # Automated flash script
│   └── setup.sh             # Development environment setup
├── config/
│   └── sdkconfig.defaults   # ESP-IDF configuration defaults
├── docs/                    # Project documentation
├── tests/                   # Integration tests
├── Cargo.toml              # Rust dependencies and configuration
├── build.rs                # ESP-IDF build integration
├── rust-toolchain.toml     # Rust toolchain specification
└── README.md               # This file
```

## Quick Start

### Prerequisites

1. **Install Rust**: Follow the [official Rust installation guide](https://rustup.rs/)
2. **Install ESP-IDF**: Use `espup` for easy setup
3. **Install Development Tools**: See setup instructions below

### Setup Development Environment

#### Windows with WSL2 (Recommended)

1. **Install WSL2**:

   ```bash
   wsl --install
   ```

2. **Install Rust**:

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

3. **Install ESP-IDF**:

   ```bash
   cargo install espup --locked
   espup install
   source ~/export-esp.sh
   ```

4. **Install Additional Tools**:

   ```bash
   cargo install espflash
   cargo install ldproxy
   sudo apt install build-essential python3.12-venv
   ```

5. **Setup USB Access** (for flashing):
   ```bash
   sudo usermod -a -G dialout $USER
   # Restart WSL or log out and back in
   ```

#### Linux/macOS

1. **Install Rust**:

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

2. **Install ESP-IDF**:

   ```bash
   cargo install espup --locked
   espup install
   source ~/export-esp.sh
   ```

3. **Install Additional Tools**:
   ```bash
   cargo install espflash
   cargo install ldproxy
   ```

### Building and Flashing

1. **Build the project**:

   ```bash
   cargo build --release
   ```

2. **Flash to device**:

   ```bash
   ./scripts/flash.sh -r
   ```

3. **Flash with custom port**:
   ```bash
   ./scripts/flash.sh -p /dev/ttyUSB0 -r
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
```

### Rust Configuration

Edit `Cargo.toml` to customize Rust settings:

```toml
[package]
name = "your-project-name"
version = "0.1.0"
edition = "2021"

[dependencies]
esp-idf-hal = "0.45.2"
esp-idf-svc = "0.51.0"
log = "0.4"
anyhow = "1.0.98"
```

## Development Workflow

### 1. Project Setup

1. **Clone the template**:

   ```bash
   git clone <template-repo>
   cd template
   ```

2. **Customize project name**:

   - Update `Cargo.toml` package name
   - Update binary name in `Cargo.toml`
   - Update flash script binary path

3. **Configure your hardware**:
   - Update GPIO pins in `src/main.rs`
   - Add your peripherals in `src/peripherals/`
   - Configure ESP-IDF settings in `config/sdkconfig.defaults`

### 2. Development

1. **Write your application logic** in `src/main.rs`
2. **Add peripheral drivers** in `src/peripherals/`
3. **Create tasks** in `src/tasks/` for concurrent operations
4. **Add utilities** in `src/utils/` for common functions

### 3. Testing

1. **Unit tests**: Write tests alongside your code
2. **Integration tests**: Add tests in `tests/` directory
3. **Hardware tests**: Use the monitor mode for debugging

### 4. Deployment

1. **Build for release**:

   ```bash
   cargo build --release
   ```

2. **Flash to device**:
   ```bash
   ./scripts/flash.sh -r
   ```

## Common Patterns

### Error Handling

```rust
use anyhow::Result;

fn main() -> Result<()> {
    // Your code here
    Ok(())
}
```

### Peripheral Management

```rust
use esp_idf_hal::peripherals::Peripherals;

let peripherals = Peripherals::take()?;
let pins = peripherals.pins;
```

### Logging

```rust
use log::info;

info!("Application started");
```

### GPIO Configuration

```rust
use esp_idf_hal::gpio::*;

let mut led = PinDriver::output(pins.gpio2)?;
led.set_high()?;
```

## Troubleshooting

### Common Issues

1. **Build Errors**:

   - Ensure ESP-IDF environment is sourced: `source ~/export-esp.sh`
   - Check Rust toolchain: `rustup show`

2. **Flash Errors**:

   - Check device permissions: `ls -la /dev/ttyUSB*`
   - Verify device connection: `dmesg | tail`

3. **WSL2 USB Issues**:
   - Install usbipd-win on Windows
   - Forward USB device to WSL2
   - Add user to dialout group

### Debugging

1. **Serial Monitor**:

   ```bash
   espflash monitor /dev/ttyUSB0
   ```

2. **Logging**:

   - Use `log::info!`, `log::warn!`, `log::error!` macros
   - Configure log level in ESP-IDF settings

3. **Memory Issues**:
   - Monitor heap usage with `esp_heap_free_size()`
   - Check stack sizes in FreeRTOS configuration

## Contributing

1. Fork the template
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## License

This template is provided under the MIT License. See LICENSE file for details.

## Support

- [ESP-IDF Documentation](https://docs.espressif.com/projects/esp-idf/)
- [Rust Embedded Book](https://rust-embedded.github.io/book/)
- [ESP32 Rust Examples](https://github.com/esp-rs/esp-idf-template)
