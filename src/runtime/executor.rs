use crate::runtime::worker_pool::WorkerPool;
use crate::runtime::job::{Job, JobType};

pub struct Executor {
    pool: WorkerPool,
    counter: u64,
}

impl Executor {
    pub fn new(max_workers: usize) -> Self {
        Self {
            pool: WorkerPool::new(max_workers),
            counter: 0,
        }
    }

    pub fn install(&mut self, pkg: &str) {
        self.counter += 1;

        self.pool.submit(Job {
            id: self.counter,
            job_type: JobType::Install(pkg.to_string()),
        });
    }

    pub fn github_install(&mut self, owner: &str, repo: &str) {
        self.counter += 1;

        self.pool.submit(Job {
            id: self.counter,
            job_type: JobType::GitHubInstall(owner.to_string(), repo.to_string()),
        });
    }

    pub fn run(self) {
        self.pool.run();
    }
}
