/// Implementation of rate limi semaphore
use std::time::{Duration, Instant};


/// Use it to control execution frequency
///
/// # Examples:
/// ```rust
/// use std::{thread, sync, time};
/// use raliguard::Semaphore;
///
///
/// // Create a semaphore with restriction `5 tasks per 1 second`
/// let originl_sem = Semaphore::new(5, time::Duration::from_secs(1));
///
/// // Make it sharable between treads (or you can share between tasks)
/// let shared_sem = sync::Arc::new(
///     sync::Mutex::new(originl_sem)
/// );
///
/// // This is a counter that increments when a thread completed
/// let shared_done_count = sync::Arc::new(sync::Mutex::new(0));
///
/// // Spawn 15 threads
/// for _ in 0..15 {
///     let cloned_sem = shared_sem.clone();
///     let cloned_done_state = shared_done_count.clone();
///     let thread = thread::spawn(move || {
///         let mut local_sem = cloned_sem.lock().unwrap();
///
///         // Get required delay
///         let calculated_delay = local_sem.calc_delay();
///         drop(local_sem);
///
///         // If delay exists, sleep it
///         if let Some(delay) = calculated_delay {
///             dbg!(&delay);
///             thread::sleep(delay);
///         }
///
///         // Mark the thread is done
///         let mut local_done_count = cloned_done_state.lock().unwrap();
///         *local_done_count += 1;
///
///     });
/// }
///
/// // So sleep 1 second (add some millis to let threads complete incrementing)
/// thread::sleep(time::Duration::from_secs(1) + time::Duration::from_millis(50));
/// let cloned_done_count = shared_done_count.clone();
/// let current_done = cloned_done_count.lock().unwrap();
///
/// // And then maximum 10 threads should be completed
/// // after 1 second sleeping
/// // (the first 5 with no delay and the another 5 after 1 second)
/// assert_eq!(*current_done == 10, true);
/// ```
#[derive(Debug, Clone)]
pub struct Semaphore {
    pub access_times: u64,
    pub per_period: Duration,

    boundary: Duration,
    current_block_access: u64,
    benchmark_stamp: Instant
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
    /// use std::time::Duration;
    /// use raliguard::Semaphore;
    ///
    /// // Allows 5 executions per 1 second
    /// let semaphore = Semaphore::new(5, Duration::from_secs(1));
    ///
    /// // Allows 2 executions per 7 seconds
    /// let semaphore = Semaphore::new(2, Duration::from_secs(7));
    /// ```
    pub fn new(access_times: u64, per_period: Duration) -> Self {
        Semaphore {
            access_times,
            per_period,
            boundary: Duration::from_secs(0),
            current_block_access: 0,
            benchmark_stamp: Instant::now(),
        }
    }

    /// Calculate delay the task/thread should sleep
    ///
    /// Use with `std::sync::Arc` and `std::sync::Mutex`
    /// (or `tokio::sync::Mutex` in async style)
    pub fn calc_delay(&mut self) -> Option<Duration> {
        let stamp = self.benchmark_stamp.elapsed();

        // Boundary second should be moved forward if it's outdated
        if stamp >= self.boundary {
            self.boundary = stamp + self.per_period;
            self.current_block_access = 1;
            return None;
        }

        // Add new hit
        self.current_block_access += 1;

        // Calc delay, should not be at all if it's the first block
        let delay = (self.boundary - stamp).checked_sub(self.per_period);

        // Allowed access for current block gets it's maximum,
        // should move block forward
        if self.current_block_access == self.access_times {
            self.boundary += self.per_period;
            self.current_block_access = 0;
        }

        delay
    }
}
