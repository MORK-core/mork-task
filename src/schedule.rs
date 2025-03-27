use alloc::boxed::Box;
use alloc::collections::VecDeque;
use core::cmp::min;
use mork_common::constants::MAX_THREAD_PIRO;
use mork_common::types::Array;
use crate::task::TaskContext;

pub struct Scheduler {
    task_queues: Array<VecDeque<Box<TaskContext>>, {MAX_THREAD_PIRO }>,
    priority: usize,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            task_queues: Array::default(),
            priority: MAX_THREAD_PIRO,
        }
    }

    pub fn enqueue(&mut self, task: Box<TaskContext>) {
        assert!(task.prio < MAX_THREAD_PIRO);
        self.priority = min(self.priority, task.prio);
        self.task_queues[task.prio].push_back(task);
    }

    pub fn dequeue(&mut self) -> Option<Box<TaskContext>> {
        if self.priority > MAX_THREAD_PIRO {
            return None;
        }
        let task = self.task_queues[self.priority].pop_front()?;
        for i in task.prio..MAX_THREAD_PIRO {
            if self.task_queues[i].is_empty() {
                self.priority += 1;
            } else {
                break;
            }
        }
        Some(task)
    }
}