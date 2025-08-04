// Peripheral drivers module
pub mod led;
pub mod button;

// Re-export commonly used peripherals
pub use led::LedController;
pub use button::ButtonController; 