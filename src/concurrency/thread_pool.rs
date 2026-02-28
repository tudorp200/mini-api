use crossbeam::queue::SegQueue;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};

use crate::concurrency::worker::{Job, Worker};
use std::time::{Duration, Instant};
use std::fmt;


#[derive(Debug)]
pub enum ThreadPoolError {
    ShutdownTimeout,
    ThreadJoinError(String),
}

impl fmt::Display for ThreadPoolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThreadPoolError::ShutdownTimeout => {
                write!(f, "Thread pool shutdown timed out before all workers finished")
            }
            ThreadPoolError::ThreadJoinError(msg) => {
                write!(f, "Failed to join worker thread: {}", msg)
            }
        }
    }
}

impl std::error::Error for ThreadPoolError {}
pub struct ThreadPool {
    workers: Vec<Worker>,
    job_queue: Arc<SegQueue<Job>>,
    job_signal: Arc<(Mutex<bool>, Condvar)>,
    running: Arc<AtomicBool>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let job_queue = Arc::new(SegQueue::new());
        let job_signal = Arc::new((Mutex::new(false), Condvar::new()));
        let mut workers = Vec::with_capacity(size);
        let running = Arc::new(AtomicBool::new(true));

        for id in 0..size {
            workers.push(Worker::new(
                id,
                Arc::clone(&job_queue),
                Arc::clone(&job_signal),
                Arc::clone(&running),
            ));
        }

        ThreadPool {
            workers,
            job_queue,
            job_signal,
            running,
        }
    }

    pub fn execute<F>(&self, f: F) -> Result<(), ThreadPoolError>
    where
        F: FnOnce() -> Result<(), Box<dyn std::error::Error>> + Send + 'static,
    {
        let job = Job::Task(Box::new(f));
        // Push this job to our queue
        self.job_queue.push(job);
        // Signal that a new job is available
        let (lock, cvar) = &*self.job_signal;
        let mut job_available = lock.lock().unwrap();
        *job_available = true;
        cvar.notify_all();
        Ok(())
    }

    pub fn shutdown(&mut self, timeout: Duration) -> Result<(), ThreadPoolError> {
        let start = Instant::now();
        self.running.store(false, Ordering::SeqCst);

        let (lock, cvar) = &*self.job_signal;
        match lock.try_lock() {
            Ok(mut job_available) => {
                *job_available = true;
                cvar.notify_all();
            }
            Err(_) => {
                // We couldn't acquire the lock, but we've set running to false,
                // so workers will eventually notice
                println!("Warning: Couldn't acquire lock to notify workers. They will exit on their next timeout check.");
            }
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                
                let remaining = timeout
                    .checked_sub(start.elapsed())
                    .unwrap_or(Duration::ZERO);

                if remaining.is_zero() {
                    return Err(ThreadPoolError::ShutdownTimeout);
                }

                if thread.join().is_err() {
                    return Err(ThreadPoolError::ThreadJoinError(format!(
                        "Worker {} failed to join",
                        worker.id
                    )));
                }
            }
        }

        if start.elapsed() > timeout {
            Err(ThreadPoolError::ShutdownTimeout)
        } else {
            Ok(())
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        if !self.workers.is_empty() {
            let _ = self.shutdown(Duration::from_secs(2));
        }
    }
}

