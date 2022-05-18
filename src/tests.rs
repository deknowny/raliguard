use std::{thread, time, sync};

use crate::Semaphore;

#[test]
fn check_limit_not_exceeded() {
    let originl_sem = Semaphore::new(3, 1);
    let shared_sem = sync::Arc::new(
        sync::Mutex::new(originl_sem)
    );

    let shared_done_count = sync::Arc::new(sync::Mutex::new(0));

    for _ in 0..10 {
        let cloned_sem = shared_sem.clone();
        let cloned_done_state = shared_done_count.clone();
        thread::spawn(move || {
            let mut local_sem = cloned_sem.lock().unwrap();

            let calculated_delay = local_sem.calc_delay();
            drop(local_sem);

            if let Some(delay) = calculated_delay {
                dbg!(&delay);
                thread::sleep(delay);
            }

            let mut local_done_count = cloned_done_state.lock().unwrap();
            *local_done_count += 1;

        });
    }

    let one_second = time::Duration::new(1, 0);

    // Maximum 3 threads should be completed
    thread::sleep(one_second);
    let cloned_done_count = shared_done_count.clone();
    let current_done = cloned_done_count.lock().unwrap();

    assert_eq!(*current_done <= 3, true);

    // Let other thread to write there again
    drop(current_done);

    // Now 9 thread should be completed, because 2 seconds passed
    thread::sleep(one_second * 2);
    let cloned_done_count = shared_done_count.clone();
    let current_done = cloned_done_count.lock().unwrap();

    assert_eq!(*current_done <= 9, true);
}
