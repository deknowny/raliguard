/// Implementation of rate limi semaphore
use std::time::{SystemTime, UNIX_EPOCH, Duration};


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
/// // Spawn 10 threads
/// for _ in 0..10 {
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
/// // So sleep 3 seconds
/// thread::sleep(time::Duration::new(1, 0));
/// let cloned_done_count = shared_done_count.clone();
/// let current_done = cloned_done_count.lock().unwrap();
///
/// // And then maximum 5 threads should be completed
/// // after 1 second sleeping
/// assert_eq!(*current_done <= 5, true);
/// ```
#[derive(Debug)]
pub struct Semaphore {
    pub access_times: u64,
    pub per_period: Duration,

    boundary: Duration,
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
    /// use std::time::Duration;
    /// use raliguard::Semaphore;
    ///
    /// // 5 executions per 1 second
    /// let semaphore = Semaphore::new(5, Duration::from_secs(1));
    ///
    /// // 2 executions per 7 seconds
    /// let semaphore = Semaphore::new(2, Duration::from_secs(7));
    /// ```
    pub fn new(access_times: u64, per_period: Duration) -> Self {
        Semaphore {
            access_times,
            per_period,
            boundary: Duration::from_secs(0),
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

        // Boundary second should be moved forward if it's outdated
        if since_the_epoch >= self.boundary {
            self.boundary = since_the_epoch + self.per_period;
            self.current_block_access = 1;
            return None;
        }

        self.current_block_access += 1;
        let delay = self.boundary - since_the_epoch;

        // Allowed access for current block gets it's maximum,
        // shoul move block forward
        if self.current_block_access == self.access_times {
            self.boundary += self.per_period;
            self.current_block_access = 0;
        }

        Some(delay)
    }
}
