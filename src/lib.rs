#![no_std]

extern crate alloc;

pub const TIME_SLICE: usize = 10;

pub mod task;
pub mod task_state;
pub mod schedule;