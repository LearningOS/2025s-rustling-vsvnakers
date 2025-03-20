// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 })); // 使用 Arc 和 Mutex 包装共享状态
    let mut handles = vec![];

    for _ in 0..10 {
        let status_shared = Arc::clone(&status); // 克隆 Arc 引用
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            let mut status = status_shared.lock().unwrap(); // 加锁以访问共享状态
            status.jobs_completed += 1; // 更新共享状态
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap(); // 等待所有线程完成
    }

    // 打印最终的 jobs_completed 值
    println!("jobs completed {}", status.lock().unwrap().jobs_completed);
}
