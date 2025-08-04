// Utility functions and helpers module
pub mod error_handler;
pub mod time_utils;
pub mod math_utils;

// Re-export commonly used utilities
pub use error_handler::handle_error;
pub use time_utils::get_uptime_ms;
pub use math_utils::map_range; 