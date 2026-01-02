use std::time::{SystemTime, UNIX_EPOCH}; // get current time

/// Gets the current Unix timestamp as a `u32`.
/// 
/// This function retrieves the current time since the Unix epoch (January 1, 1970)
/// and converts it to a `u32` value. This is sufficient for timestamps until year 2106.
/// 
/// # Returns
/// 
/// Current Unix timestamp as `u32`.
/// 
/// # Panics
/// 
/// This function will panic if:
/// - The system time is before the Unix epoch
/// - The timestamp cannot be converted to `u32` (unlikely before 2106)
pub fn get_time() -> u32{
    let a = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    a.try_into().unwrap()
}