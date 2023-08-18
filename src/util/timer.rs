/// A timer that can be used to time multiple functions.
/// Timings are stored in nanoseconds.
pub struct Timer {
    times: Vec<u128>,
}

impl Timer {
    /// Creates a new timer with no times recorded.
    pub fn new() -> Self {
        Self { times: Vec::new() }
    }

    /// Time a function and return the elapsed time in nanoseconds and the result of the function call as a tuple.
    ///
    /// # Arguments
    /// * `function` - The function to time.
    ///
    /// # Examples
    /// ```
    /// let mut timer = Timer::new();
    ///
    /// let (time, result) = timer.time(|| {
    ///     let mut sum = 0;
    ///     while sum != 1_000_000 {
    ///         sum += 1;
    ///     }
    ///
    ///     sum
    /// });
    ///
    /// println!("Function took {} nanoseconds.", time);
    /// println!("Result of function call: {}", result);
    /// ```
    pub fn time<F, R>(&mut self, function: F) -> (u128, R)
    where
        F: FnOnce() -> R,
    {
        let start = std::time::Instant::now();
        let result = function();
        let elapsed = start.elapsed().as_nanos();

        self.times.push(elapsed);

        (elapsed, result)
    }

    /// Get the total time elapsed by all timers.
    ///
    /// # Examples
    /// ```
    /// let mut timer = Timer::new();
    ///
    /// let (time, result) = timer.time(|| {
    ///     let mut sum = 0;
    ///     while sum != 1_000_000 {
    ///         sum += 1;
    ///     }
    ///
    ///     sum
    /// });
    /// println!("Function took {} nanoseconds.", time);
    ///
    /// let (time, result) = timer.time(|| {
    ///     let mut sum = 0;
    ///     while sum != 5_000_000 {
    ///         sum += 1;
    ///     }
    ///
    ///     sum
    /// });
    /// println!("Function took {} nanoseconds.", time);
    ///
    /// // Prints the time it took for both functions to run in nanoseconds.
    /// println!("Total Time: {} nanoseconds.", timer.total_time());
    pub fn total_time(&self) -> u128 {
        self.times.iter().sum()
    }
}

/// Format a time in nanoseconds into a human-readable string.
///
/// # Arguments
/// * `nanos` - The time in nanoseconds.
///
/// # Examples
/// ```
/// let nanos = 1_000_000_000;
///
/// println!("{} nanoseconds is {}.", nanos, format_time(nanos));
/// ```
pub fn format_time(nanos: u128) -> String {
    let mut nanos = nanos;
    let mut result = String::new();

    let years = nanos / 31_557_600_000_000_000;
    nanos -= years * 31_557_600_000_000_000;
    if years > 0 {
        result.push_str(&format!("{} year", years));
        if years > 1 {
            result.push('s');
        }
    }

    let months = nanos / 2_629_800_000_000_000;
    nanos -= months * 2_629_800_000_000_000;
    if months > 0 {
        if !result.is_empty() {
            result.push_str(", ");
        }
        result.push_str(&format!("{} month", months));
        if months > 1 {
            result.push('s');
        }
    }

    let days = nanos / 86_400_000_000_000;
    nanos -= days * 86_400_000_000_000;
    if days > 0 {
        if !result.is_empty() {
            result.push_str(", ");
        }
        result.push_str(&format!("{} day", days));
        if days > 1 {
            result.push('s');
        }
    }

    let hours = nanos / 3_600_000_000_000;
    nanos -= hours * 3_600_000_000_000;
    if hours > 0 {
        if !result.is_empty() {
            result.push_str(", ");
        }
        result.push_str(&format!("{} hour", hours));
        if hours > 1 {
            result.push('s');
        }
    }

    let minutes = nanos / 60_000_000_000;
    nanos -= minutes * 60_000_000_000;
    if minutes > 0 {
        if !result.is_empty() {
            result.push_str(", ");
        }
        result.push_str(&format!("{} minute", minutes));
        if minutes > 1 {
            result.push('s');
        }
    }

    let seconds = nanos / 1_000_000_000;
    nanos -= seconds * 1_000_000_000;
    if seconds > 0 {
        if !result.is_empty() {
            result.push_str(", ");
        }
        result.push_str(&format!("{} second", seconds));
        if seconds > 1 {
            result.push('s');
        }
    }

    let milliseconds = nanos / 1_000_000;
    nanos -= milliseconds * 1_000_000;
    if milliseconds > 0 {
        if !result.is_empty() {
            result.push_str(", ");
        }
        result.push_str(&format!("{} millisecond", milliseconds));
        if milliseconds > 1 {
            result.push('s');
        }
    }

    let microseconds = nanos / 1_000;
    nanos -= microseconds * 1_000;
    if microseconds > 0 {
        if !result.is_empty() {
            result.push_str(", ");
        }
        result.push_str(&format!("{} microsecond", microseconds));
        if microseconds > 1 {
            result.push('s');
        }
    }

    if nanos > 0 {
        if !result.is_empty() {
            result.push_str(", ");
        }
        result.push_str(&format!("{} nanosecond", nanos));
        if nanos > 1 {
            result.push('s');
        }
    }

    // Add an "and" to the final unit if there are multiple units.
    if result.matches(',').count() > 1 {
        let last_comma = result.rfind(',').unwrap();
        result.insert_str(last_comma + 1, " and");
    }

    result
}
