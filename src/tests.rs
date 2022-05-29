use std::{thread, time, sync};

use crate::Semaphore;

#[test]
fn check_limit_not_exceeded() {
    let originl_sem = Semaphore::new(3, time::Duration::from_secs(1));
    let shared_sem = sync::Arc::new(
        sync::Mutex::new(originl_sem)
    );

    let shared_done_count = sync::Arc::new(sync::Mutex::new(0));

    for _ in 0..15 {
        let cloned_sem = shared_sem.clone();
        let cloned_done_state = shared_done_count.clone();
        thread::spawn(move || {
            let mut local_sem = cloned_sem.lock().unwrap();

            let calculated_delay = local_sem.calc_delay();
            drop(local_sem);
            dbg!(&calculated_delay);

            if let Some(delay) = calculated_delay {
                thread::sleep(delay);
            }

            let mut local_done_count = cloned_done_state.lock().unwrap();
            *local_done_count += 1;

        });
    }

    // Add some millis because of working freeze
    let one_second = time::Duration::from_secs(1) + time::Duration::from_millis(50);

    // Maximum 6 threads should be completed (3 with no delay at 3 adter a second)
    thread::sleep(one_second);
    let cloned_done_count = shared_done_count.clone();
    let current_done = cloned_done_count.lock().unwrap();

    assert_eq!(*current_done, 6);

    // Let other thread to write there again
    drop(current_done);

    // Now 12 thread should be completed, because 2 seconds passed
    // And another 6 threads should be completed
    thread::sleep(one_second * 2);
    let cloned_done_count = shared_done_count.clone();
    let current_done = cloned_done_count.lock().unwrap();

    assert_eq!(*current_done, 12);
}
