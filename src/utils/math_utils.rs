use anyhow::Result;

/// Map a value from one range to another
pub fn map_range(value: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
    let from_range = from_max - from_min;
    let to_range = to_max - to_min;
    let normalized = (value - from_min) / from_range;
    to_min + (normalized * to_range)
}

/// Clamp a value between min and max
pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    value.max(min).min(max)
}

/// Linear interpolation between two values
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Smooth step interpolation
pub fn smooth_step(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Convert degrees to radians
pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

/// Convert radians to degrees
pub fn radians_to_degrees(radians: f32) -> f32 {
    radians * 180.0 / std::f32::consts::PI
}

/// Calculate the average of a slice of values
pub fn average(values: &[f32]) -> Result<f32> {
    if values.is_empty() {
        return Err(anyhow::anyhow!("Cannot calculate average of empty slice"));
    }
    
    let sum: f32 = values.iter().sum();
    Ok(sum / values.len() as f32)
}

/// Calculate the standard deviation of a slice of values
pub fn standard_deviation(values: &[f32]) -> Result<f32> {
    if values.len() < 2 {
        return Err(anyhow::anyhow!("Need at least 2 values for standard deviation"));
    }
    
    let mean = average(values)?;
    let variance = values.iter()
        .map(|&x| (x - mean).powi(2))
        .sum::<f32>() / (values.len() - 1) as f32;
    
    Ok(variance.sqrt())
}

/// Round to specified number of decimal places
pub fn round_to_places(value: f32, places: u32) -> f32 {
    let multiplier = 10.0_f32.powi(places as i32);
    (value * multiplier).round() / multiplier
}

/// Check if two floating point numbers are approximately equal
pub fn approximately_equal(a: f32, b: f32, epsilon: f32) -> bool {
    (a - b).abs() < epsilon
}

/// Calculate percentage
pub fn percentage(value: f32, total: f32) -> Result<f32> {
    if total.abs() < f32::EPSILON {
        return Err(anyhow::anyhow!("Cannot calculate percentage with zero total"));
    }
    Ok((value / total) * 100.0)
}

/// Calculate the greatest common divisor of two integers
pub fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// Calculate the least common multiple of two integers
pub fn lcm(a: u32, b: u32) -> u32 {
    (a * b) / gcd(a, b)
}

/// Check if a number is prime
pub fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    
    let sqrt_n = (n as f32).sqrt() as u32;
    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
} 