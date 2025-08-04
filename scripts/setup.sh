#!/usr/bin/env bash
set -euo pipefail

# ---------------------------
# Colors and helpers
# ---------------------------
log() { echo -e "\033[1;34m[setup]\033[0m $*"; }
error() { echo -e "\033[1;31m[ERROR]\033[0m $*" >&2; }
success() { echo -e "\033[1;32m[SUCCESS]\033[0m $*"; }
warn() { echo -e "\033[1;33m[WARNING]\033[0m $*"; }

# ---------------------------
# Defaults
# ---------------------------
INSTALL_RUST=1
INSTALL_ESP_IDF=1
INSTALL_TOOLS=1
SETUP_WSL=0
SKIP_CONFIRM=0

# ---------------------------
# Helpers
# ---------------------------
usage() {
  cat <<'EOF'
Usage: setup.sh [options]

Options:
  --skip-rust        Skip Rust installation
  --skip-esp-idf     Skip ESP-IDF installation
  --skip-tools       Skip development tools installation
  --wsl              Setup for WSL2 environment
  --yes              Skip confirmation prompts
  -h, --help         Show this help and exit

Examples:
  ./scripts/setup.sh
  ./scripts/setup.sh --wsl
  ./scripts/setup.sh --yes
EOF
}

# ---------------------------
# Parse args
# ---------------------------
while [[ $# -gt 0 ]]; do
  case "$1" in
    --skip-rust) INSTALL_RUST=0; shift;;
    --skip-esp-idf) INSTALL_ESP_IDF=0; shift;;
    --skip-tools) INSTALL_TOOLS=0; shift;;
    --wsl) SETUP_WSL=1; shift;;
    --yes) SKIP_CONFIRM=1; shift;;
    -h|--help) usage; exit 0;;
    *) echo "Unknown option: $1" >&2; usage; exit 1;;
  esac
done

# ---------------------------
# Check OS
# ---------------------------
detect_os() {
  if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    if grep -q Microsoft /proc/version 2>/dev/null; then
      echo "wsl"
    else
      echo "linux"
    fi
  elif [[ "$OSTYPE" == "darwin"* ]]; then
    echo "macos"
  else
    echo "unknown"
  fi
}

OS=$(detect_os)
log "Detected OS: $OS"

# ---------------------------
# Confirmation
# ---------------------------
if [[ $SKIP_CONFIRM -eq 0 ]]; then
  echo "This script will set up your ESP32 development environment."
  echo "The following components will be installed:"
  [[ $INSTALL_RUST -eq 1 ]] && echo "  - Rust toolchain"
  [[ $INSTALL_ESP_IDF -eq 1 ]] && echo "  - ESP-IDF"
  [[ $INSTALL_TOOLS -eq 1 ]] && echo "  - Development tools (espflash, ldproxy)"
  [[ $SETUP_WSL -eq 1 ]] && echo "  - WSL2 USB configuration"
  echo
  read -p "Continue? (y/N): " -n 1 -r
  echo
  if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Setup cancelled."
    exit 0
  fi
fi

# ---------------------------
# Install Rust
# ---------------------------
if [[ $INSTALL_RUST -eq 1 ]]; then
  log "Installing Rust..."
  
  if command -v rustc &> /dev/null; then
    warn "Rust is already installed. Updating..."
    rustup update
  else
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
  fi
  
  success "Rust installed successfully"
fi

# ---------------------------
# Install ESP-IDF
# ---------------------------
if [[ $INSTALL_ESP_IDF -eq 1 ]]; then
  log "Installing ESP-IDF..."
  
  if command -v espup &> /dev/null; then
    warn "espup is already installed. Updating..."
    cargo install espup --locked --force
  else
    cargo install espup --locked
  fi
  
  espup install
  source ~/export-esp.sh
  
  success "ESP-IDF installed successfully"
fi

# ---------------------------
# Install development tools
# ---------------------------
if [[ $INSTALL_TOOLS -eq 1 ]]; then
  log "Installing development tools..."
  
  # Install espflash
  if ! command -v espflash &> /dev/null; then
    cargo install espflash
  else
    warn "espflash is already installed"
  fi
  
  # Install ldproxy
  if ! command -v ldproxy &> /dev/null; then
    cargo install ldproxy
  else
    warn "ldproxy is already installed"
  fi
  
  # Install additional tools on Linux/WSL
  if [[ "$OS" == "linux" || "$OS" == "wsl" ]]; then
    log "Installing system dependencies..."
    sudo apt update
    sudo apt install -y build-essential python3.12-venv pkg-config libssl-dev
  fi
  
  success "Development tools installed successfully"
fi

# ---------------------------
# WSL2 specific setup
# ---------------------------
if [[ $SETUP_WSL -eq 1 ]]; then
  log "Setting up WSL2 USB access..."
  
  # Add user to dialout group
  sudo usermod -a -G dialout "$USER"
  
  echo
  warn "WSL2 USB Setup Instructions:"
  echo "1. Install usbipd-win on Windows:"
  echo "   winget install usbipd"
  echo
  echo "2. List USB devices (in Windows PowerShell):"
  echo "   usbipd list"
  echo
  echo "3. Attach your ESP32 device to WSL2:"
  echo "   usbipd wsl attach --busid <BUSID>"
  echo
  echo "4. Restart WSL or log out and back in for group changes to take effect"
  echo
  success "WSL2 setup instructions provided"
fi

# ---------------------------
# Environment setup
# ---------------------------
log "Setting up environment..."

# Add ESP environment to shell profile
SHELL_PROFILE=""
if [[ -f "$HOME/.bashrc" ]]; then
  SHELL_PROFILE="$HOME/.bashrc"
elif [[ -f "$HOME/.zshrc" ]]; then
  SHELL_PROFILE="$HOME/.zshrc"
fi

if [[ -n "$SHELL_PROFILE" ]]; then
  if ! grep -q "export-esp.sh" "$SHELL_PROFILE"; then
    echo 'source ~/export-esp.sh' >> "$SHELL_PROFILE"
    log "Added ESP environment to $SHELL_PROFILE"
  else
    warn "ESP environment already configured in $SHELL_PROFILE"
  fi
fi

# ---------------------------
# Verification
# ---------------------------
log "Verifying installation..."

# Check Rust
if command -v rustc &> /dev/null; then
  success "Rust: $(rustc --version)"
else
  error "Rust not found"
fi

# Check ESP-IDF
if command -v espup &> /dev/null; then
  success "ESP-IDF: espup available"
else
  error "ESP-IDF not found"
fi

# Check tools
if command -v espflash &> /dev/null; then
  success "espflash: available"
else
  error "espflash not found"
fi

if command -v ldproxy &> /dev/null; then
  success "ldproxy: available"
else
  error "ldproxy not found"
fi

# ---------------------------
# Final instructions
# ---------------------------
echo
success "Setup completed successfully!"
echo
echo "Next steps:"
echo "1. Source the ESP environment: source ~/export-esp.sh"
echo "2. Build your project: cargo build"
echo "3. Flash your device: ./scripts/flash.sh"
echo
echo "For WSL2 users:"
echo "- Make sure to attach your ESP32 device using usbipd"
echo "- Restart WSL or log out and back in for group changes"
echo
echo "Happy coding! 🚀" 