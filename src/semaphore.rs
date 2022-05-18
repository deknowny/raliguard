use std::time::{SystemTime, UNIX_EPOCH, Duration};


#[derive(Debug)]
pub struct Semaphore {
    pub access_times: u64,
    pub per_period: u64,

    boundary_second: u64,
    current_block_access: u64
}


impl Semaphore {
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
        let delay = since_the_epoch - Duration::new(timestamp, 0);

        // Allowed access for current block gets it's maximum,
        // shoul move block forward
        if self.access_times == self.current_block_access {
            self.boundary_second += self.per_period;
            self.current_block_access = 0;
        }

        Some(delay)
    }
}
