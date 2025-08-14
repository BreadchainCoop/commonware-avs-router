use std::sync::{Arc, Mutex};

use crate::creator::types::TaskQueue;
use crate::ingress::TaskRequest;

/// Simplified concrete implementation of TaskQueue using std::sync::Mutex
///
/// This is a basic, working implementation that uses a Vec as the underlying storage
/// with standard mutex synchronization. It's simpler and more reliable than the
/// previous async implementation.
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
        if let Ok(mut queue) = self.queue.lock() {
            queue.push(task);
        }
    }

    fn pop(&self) -> Option<TaskRequest> {
        if let Ok(mut queue) = self.queue.lock() {
            queue.pop()
        } else {
            None
        }
    }
}
