use anyhow::Result;
use log::{error, warn, info};

/// Handle errors with consistent logging and recovery strategies
pub fn handle_error<T>(result: Result<T>, context: &str) -> Result<T> {
    match result {
        Ok(value) => Ok(value),
        Err(e) => {
            error!("Error in {}: {:?}", context, e);
            Err(e)
        }
    }
}

/// Handle errors with custom recovery strategy
pub fn handle_error_with_recovery<T, F>(
    result: Result<T>,
    context: &str,
    recovery_fn: F,
) -> Result<T>
where
    F: FnOnce() -> Result<T>,
{
    match result {
        Ok(value) => Ok(value),
        Err(e) => {
            warn!("Error in {}: {:?}, attempting recovery...", context, e);
            match recovery_fn() {
                Ok(value) => {
                    info!("Recovery successful for {}", context);
                    Ok(value)
                }
                Err(recovery_error) => {
                    error!("Recovery failed for {}: {:?}", context, recovery_error);
                    Err(recovery_error)
                }
            }
        }
    }
}

/// Retry operation with exponential backoff
pub fn retry_with_backoff<T, F>(
    mut operation: F,
    max_attempts: u32,
    initial_delay_ms: u32,
) -> Result<T>
where
    F: FnMut() -> Result<T>,
{
    let mut attempt = 0;
    let mut delay_ms = initial_delay_ms;

    loop {
        match operation() {
            Ok(value) => return Ok(value),
            Err(e) => {
                attempt += 1;
                if attempt >= max_attempts {
                    return Err(e);
                }

                warn!("Attempt {} failed, retrying in {}ms: {:?}", attempt, delay_ms, e);
                esp_idf_hal::delay::FreeRtos::delay_ms(delay_ms);
                delay_ms *= 2; // Exponential backoff
            }
        }
    }
}

/// Safe division with error handling
pub fn safe_divide(numerator: f32, denominator: f32) -> Result<f32> {
    if denominator.abs() < f32::EPSILON {
        Err(anyhow::anyhow!("Division by zero"))
    } else {
        Ok(numerator / denominator)
    }
}

/// Validate range with error handling
pub fn validate_range(value: f32, min: f32, max: f32, name: &str) -> Result<f32> {
    if value < min || value > max {
        Err(anyhow::anyhow!(
            "{} value {} is out of range [{}, {}]",
            name,
            value,
            min,
            max
        ))
    } else {
        Ok(value)
    }
} 