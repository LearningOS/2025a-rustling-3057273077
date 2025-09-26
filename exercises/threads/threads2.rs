// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.



use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::thread;
use std::time::Duration;

fn main() {
    let jobs_completed = Arc::new(AtomicU32::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let jobs_completed = Arc::clone(&jobs_completed);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            jobs_completed.fetch_add(1, Ordering::SeqCst);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
        println!("jobs completed {}", jobs_completed.load(Ordering::SeqCst));
    }
}