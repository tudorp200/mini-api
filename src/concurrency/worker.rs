use crossbeam::queue::SegQueue;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;
pub struct Worker {
    pub id : usize,
    pub thread :  Option<thread::JoinHandle<()>>
}

pub enum Job {
    Task(Box<dyn FnOnce() -> Result<(), Box<dyn std::error::Error>> + Send + 'static>),
    Shutdown,
}

impl Worker {
    pub fn new(
        id: usize,
        job_queue: Arc<SegQueue<Job>>,
        job_signal: Arc<(Mutex<bool>, Condvar)>,
        running: Arc<AtomicBool>,
    ) -> Worker {
        let thread = thread::spawn(move || loop {
            match job_queue.pop() {
                Some(Job::Task(task)) => if let Err(_) = task() {},
                Some(Job::Shutdown) => {
                    break;
                }
                None => {
                    let (lock, cvar) = &*job_signal;
                    let mut job_available = lock.lock().unwrap();
                    while !*job_available && running.load(Ordering::Relaxed) {
                        job_available = cvar
                            .wait_timeout(job_available, Duration::from_millis(100))
                            .unwrap()
                            .0;
                    }
                    *job_available = false;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}