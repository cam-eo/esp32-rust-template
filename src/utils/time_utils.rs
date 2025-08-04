use anyhow::Result;

/// Get system uptime in milliseconds
pub fn get_uptime_ms() -> u64 {
    esp_idf_hal::sys::esp_timer_get_time() / 1000
}

/// Get system uptime in seconds
pub fn get_uptime_s() -> u64 {
    esp_idf_hal::sys::esp_timer_get_time() / 1_000_000
}

/// Convert milliseconds to ticks
pub fn ms_to_ticks(ms: u32) -> u32 {
    ms * (esp_idf_hal::sys::CONFIG_FREERTOS_HZ as u32) / 1000
}

/// Convert ticks to milliseconds
pub fn ticks_to_ms(ticks: u32) -> u32 {
    ticks * 1000 / (esp_idf_hal::sys::CONFIG_FREERTOS_HZ as u32)
}

/// Sleep for specified milliseconds
pub fn sleep_ms(ms: u32) {
    esp_idf_hal::delay::FreeRtos::delay_ms(ms);
}

/// Sleep for specified microseconds
pub fn sleep_us(us: u32) {
    esp_idf_hal::delay::FreeRtos::delay_us(us);
}

/// Format uptime as human readable string
pub fn format_uptime() -> String {
    let uptime_s = get_uptime_s();
    let hours = uptime_s / 3600;
    let minutes = (uptime_s % 3600) / 60;
    let seconds = uptime_s % 60;
    
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

/// Check if specified time has elapsed since start time
pub fn has_elapsed(start_time_ms: u64, duration_ms: u32) -> bool {
    let current_time = get_uptime_ms();
    current_time >= start_time_ms + duration_ms as u64
}

/// Create a timer that expires after specified duration
pub struct Timer {
    start_time: u64,
    duration_ms: u32,
}

impl Timer {
    /// Create a new timer with specified duration
    pub fn new(duration_ms: u32) -> Self {
        Self {
            start_time: get_uptime_ms(),
            duration_ms,
        }
    }

    /// Check if timer has expired
    pub fn has_expired(&self) -> bool {
        has_elapsed(self.start_time, self.duration_ms)
    }

    /// Get remaining time in milliseconds
    pub fn remaining_ms(&self) -> u32 {
        let elapsed = get_uptime_ms() - self.start_time;
        if elapsed >= self.duration_ms as u64 {
            0
        } else {
            self.duration_ms - elapsed as u32
        }
    }

    /// Reset timer
    pub fn reset(&mut self) {
        self.start_time = get_uptime_ms();
    }

    /// Reset timer with new duration
    pub fn reset_with_duration(&mut self, duration_ms: u32) {
        self.duration_ms = duration_ms;
        self.start_time = get_uptime_ms();
    }
} 