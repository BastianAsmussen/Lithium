use std::time::Duration;

/// A timer that can be used to time multiple functions.
///
/// # Fields
///
/// * `times` - A vector of all the times recorded by the timer.
#[derive(Debug, Clone)]
pub struct Timer {
    times: Vec<Duration>,
}

impl Timer {
    /// Creates a new timer with no times recorded.
    ///
    /// # Returns
    ///
    /// * `Timer` - The new timer.
    pub const fn new() -> Self {
        Self { times: Vec::new() }
    }

    /// Time a function and return the elapsed time in nanoseconds and the result of the function call as a tuple.
    ///
    /// # Arguments
    /// * `function` - The function to time.
    ///
    /// # Returns
    ///
    /// * (`R`, `Duration`) - The result of the function call and the elapsed time in nanoseconds.
    pub fn time<F, R>(&mut self, function: F) -> (R, Duration)
    where
        F: FnOnce() -> R,
    {
        let start = std::time::Instant::now();
        let result = function();
        let elapsed = start.elapsed();

        self.times.push(elapsed);

        (result, elapsed)
    }

    /// Get the total time elapsed by all timers.
    ///
    /// # Returns
    ///
    /// * `Duration` - The total time elapsed by all timers.
    pub fn total_time(&self) -> Duration {
        self.times.iter().sum()
    }
}

/// Format a time into a human-readable string.
///
/// # Arguments
/// * `duration` - The time to format.
///
/// # Returns
///
/// * `String` - The formatted time.
pub fn format_time(duration: Duration) -> String {
    let mut nanos = duration.as_nanos();
    let mut result = String::new();

    if nanos >= 1_000_000_000 {
        let seconds = nanos / 1_000_000_000;
        nanos -= seconds * 1_000_000_000;
        result.push_str(&format!("{seconds}s, "));
    }

    if nanos >= 1_000_000 {
        let milliseconds = nanos / 1_000_000;
        nanos -= milliseconds * 1_000_000;
        result.push_str(&format!("{milliseconds}ms, "));
    }

    if nanos >= 1_000 {
        let microseconds = nanos / 1_000;
        nanos -= microseconds * 1_000;
        result.push_str(&format!("{microseconds}μs, "));
    }

    if nanos > 0 {
        result.push_str(&format!("{nanos}ns"));
    }

    // If there is a trailing comma, remove it.
    if result.ends_with(", ") {
        result.pop();
        result.pop();
    }

    // If there is no time, return 0ns.
    if result.is_empty() {
        result.push_str("0ns");
    }

    result
}
