use std::time::{SystemTime, UNIX_EPOCH, Duration, Instant};


#[derive(Debug)]
pub struct Semaphore {
    pub access_times: u64,
    pub per_period: u64,

    boundary_second: u64,
    current_block_access: u64
}


impl Semaphore {
    pub fn new(access_times: u64, per_period: u64) -> Self {
        Semaphore {
            access_times,
            per_period,
            boundary_second: 0,
            current_block_access: 0
        }
    }

    pub fn calc_delay(&mut self) -> Option<Duration> {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        let timestamp = since_the_epoch.as_secs();

        // Boundary second should be moved forward if it's outdated
        if timestamp >= self.boundary_second {
            self.boundary_second = timestamp + self.per_period;
            self.current_block_access = 0;
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
