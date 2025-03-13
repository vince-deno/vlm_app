// --- Main traits

use std::{ path::PathBuf, time::Instant};

use crate::cli::cl::{DefaultVLMCli, VLMCli};

pub trait DefaultVLMTaskExecutor: Send + Sync {
    fn execute_simple_task_default(&self) -> Result<(), String> {
        println!("Executing simple task (default)...");
        Ok(())
    }
}

pub trait DefaultVLMGenericTaskExecutor {
    fn execute_task_default<F, T>(&self, task: F) -> Result<T, String>
    where
        F: FnOnce() -> T,
    {
        let start = Instant::now();
        let result = task();
        println!("Task executed in (default): {:?}", start.elapsed());
        Ok(result)
    }
}

impl<T: DefaultVLMCli> VLMCli for T {
    fn run(&self) -> Result<(), std::fmt::Error> {
        self.run_default()
    }
    fn parse_args(&self) -> Result<(), std::fmt::Error> {
        self.parse_args_default()
    }
    fn help(&self) {
        self.help_default()
    }
    fn pattern(pattern: String) -> Self
    where
        Self: Sized,
    {
        Self::pattern_default(pattern)
    }
    fn file(file: PathBuf) -> Self
    where
        Self: Sized,
    {
        Self::file_default(file)
    }
    fn count(&self, count: usize) {
        let _ = Self::default().count_default(count);
    }
    fn path(&self, path: PathBuf) {
        let _ = Self::default().path_default(path);
    }
    fn json(&self, json: bool) {
        let _ = Self::default().json_default(json);
    }
}

pub trait VLMTaskExecutor: Send + Sync {
    fn execute_simple_task(&self) -> Result<(), String>;
}

impl<T: DefaultVLMTaskExecutor> VLMTaskExecutor for T {
    fn execute_simple_task(&self) -> Result<(), String> {
        self.execute_simple_task_default()
    }
}

pub trait VLMGenericTaskExecutor: VLMTaskExecutor {
    fn execute_task<F, T>(&self, task: F) -> Result<T, String>
    where
        F: FnOnce() -> T;
}

impl<T: DefaultVLMGenericTaskExecutor + VLMTaskExecutor> VLMGenericTaskExecutor for T {
    fn execute_task<F, U>(&self, task: F) -> Result<U, String>
    where
        F: FnOnce() -> U,
    {
        self.execute_task_default(task)
    }
}


