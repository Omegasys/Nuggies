use std::sync::{Arc, Mutex};
use std::thread;
use std::collections::VecDeque;

use crate::runtime::job::{Job, JobType};

pub struct WorkerPool {
    queue: Arc<Mutex<VecDeque<Job>>>,
    max_workers: usize,
}

impl WorkerPool {
    pub fn new(max_workers: usize) -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            max_workers,
        }
    }

    pub fn submit(&self, job: Job) {
        self.queue.lock().unwrap().push_back(job);
    }

    pub fn run(&self) {
        let mut handles = vec![];

        for _ in 0..self.max_workers {
            let queue = Arc::clone(&self.queue);

            let handle = thread::spawn(move || {
                loop {
                    let job = {
                        let mut q = queue.lock().unwrap();
                        q.pop_front()
                    };

                    match job {
                        Some(job) => {
                            println!("[worker] executing job {:?}", job.job_type);

                            match job.job_type {
                                JobType::Install(pkg) => {
                                    println!("installing {}", pkg);
                                }
                                JobType::Remove(pkg) => {
                                    println!("removing {}", pkg);
                                }
                                JobType::Update => {
                                    println!("updating system");
                                }
                                JobType::GitHubInstall(owner, repo) => {
                                    println!("installing from github {}/{}", owner, repo);
                                }
                            }
                        }
                        None => break,
                    }
                }
            });

            handles.push(handle);
        }

        for h in handles {
            h.join().unwrap();
        }
    }
}
