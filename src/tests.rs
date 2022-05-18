use std::{thread, time, sync};

use crate::Semaphore;

#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

#[test]
fn check_limit_not_exceeded() {
    let originl_sem = Semaphore::new(2, 1);
    let shared_sem = sync::Arc::new(
        sync::Mutex::new(originl_sem)
    );

    let shared_done_count = sync::Arc::new(sync::Mutex::new(0));
    let mut running_threads = vec![];

    for _ in 0..20 {
        let cloned_sem = shared_sem.clone();
        let cloned_done_state = shared_done_count.clone();
        let task = thread::spawn(move || {
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
        running_threads.push(task);
    }

    // Maximum 3 threads should be completed
    thread::sleep(time::Duration::new(1, 0));
    let cloned_done_count = shared_done_count.clone();
    let current_done = cloned_done_count.lock().unwrap();

    dbg!(&current_done);
    assert_eq!(*current_done <= 3, true);
}
