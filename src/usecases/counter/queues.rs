use std::sync::{Arc, Mutex};

use crate::creator::core::{TaskQueue, TaskRequest};

/// Simple in-memory task queue using Arc<Mutex>
#[derive(Clone)]
pub struct SimpleTaskQueue {
    queue: Arc<Mutex<Vec<TaskRequest>>>,
}

impl SimpleTaskQueue {
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn get_queue(&self) -> Arc<Mutex<Vec<TaskRequest>>> {
        self.queue.clone()
    }
}

impl Default for SimpleTaskQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskQueue for SimpleTaskQueue {
    fn push(&self, task: TaskRequest) {
        if let Ok(mut queue) = self.queue.try_lock() {
            queue.push(task);
        }
    }

    fn pop(&self) -> Option<TaskRequest> {
        if let Ok(mut queue) = self.queue.try_lock() {
            queue.pop()
        } else {
            None
        }
    }
}

/// Mock task queue for testing
#[allow(dead_code)]
pub struct MockTaskQueue {
    tasks: Vec<TaskRequest>,
    should_fail: bool,
}

impl MockTaskQueue {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            should_fail: false,
        }
    }

    #[allow(dead_code)]
    pub fn with_tasks(tasks: Vec<TaskRequest>) -> Self {
        Self {
            tasks,
            should_fail: false,
        }
    }

    #[allow(dead_code)]
    pub fn new_failing() -> Self {
        Self {
            tasks: Vec::new(),
            should_fail: true,
        }
    }
}

impl Default for MockTaskQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskQueue for MockTaskQueue {
    fn push(&self, _task: TaskRequest) {
        if self.should_fail {
            // Simulate failure
        }
    }

    fn pop(&self) -> Option<TaskRequest> {
        if self.should_fail {
            None
        } else {
            self.tasks.last().cloned()
        }
    }
}
