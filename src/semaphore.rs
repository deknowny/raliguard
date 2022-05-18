/// Implementation of rate limi semaphore
use std::time::{SystemTime, UNIX_EPOCH, Duration};


/// Use it to control execution frequency
#[derive(Debug)]
pub struct Semaphore {
    pub access_times: u64,
    pub per_period: u64,

    boundary_second: u64,
    current_block_access: u64
}


impl Semaphore {
    /// Create a new semaphore
    ///
    /// # Arguments:
    /// * `access_times` - how many times a code allowed to be executed
    /// * `per_period` - in which period code allowed to be executed
    ///
    /// # Returns:
    /// Duration you need to sleep
    ///
    /// # Examples:
    ///
    /// ```rust
    /// use raliguard::Semaphore;
    ///
    /// // 5 executions per 1 second
    /// let semaphore = Semaphore::new(5, 1);
    ///
    /// // 2 executions per 7 seconds
    /// let semaphore = Semaphore::new(2, 7);
    /// ```
    pub fn new(access_times: u64, per_period: u64) -> Self {
        Semaphore {
            access_times,
            per_period,
            boundary_second: 0,
            current_block_access: 0
        }
    }

    /// Calculate delay the task/thread should sleep
    ///
    /// Use with `std::sync::Arc` and `std::sync::Mutex`
    /// (or `tokio::sync::Mutex` in async style)
    pub fn calc_delay(&mut self) -> Option<Duration> {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        let timestamp = since_the_epoch.as_secs();

        // Boundary second should be moved forward if it's outdated
        if timestamp >= self.boundary_second {
            self.boundary_second = timestamp + self.per_period;
            self.current_block_access = 1;
            return None;
        }

        self.current_block_access += 1;
        let delay = Duration::from_secs(self.boundary_second) - since_the_epoch;

        // Allowed access for current block gets it's maximum,
        // shoul move block forward
        if self.current_block_access == self.access_times {
            self.boundary_second += self.per_period;
            self.current_block_access = 0;
        }

        Some(delay)
    }
}
