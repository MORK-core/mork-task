use mork_hal::context::{HALContext, HALContextTrait};
use alloc::sync::Arc;
use mork_common::constants::MAX_THREAD_PIRO;
use mork_common::types::SyncUnsafeCell;
use mork_hal::idle_thread;
use mork_hal::mm::PageTableImpl;
use crate::task_state::ThreadStateEnum;

#[allow(dead_code)]
pub struct TaskContext {
    hal_context: HALContext,
    pub prio: usize,
    pub vspace: Option<Arc<SyncUnsafeCell<PageTableImpl>>>,
    state: ThreadStateEnum
}
impl TaskContext {
    pub fn new() -> Self {
        Self {
            hal_context: HALContext::new(),
            prio: MAX_THREAD_PIRO - 1,
            vspace: None,
            state: ThreadStateEnum::ThreadStateInactive,
        }
    }

    pub fn new_idle_thread(stack_ptr: usize) -> Self {
        let mut hal_context = HALContext::new();
        hal_context.set_stack(stack_ptr);
        hal_context.set_next_ip(idle_thread as usize);
        hal_context.configure_idle();
        Self {
            hal_context,
            prio: MAX_THREAD_PIRO,
            vspace: None,
            state: ThreadStateEnum::ThreadStateIdleThreadState
        }

    }
}