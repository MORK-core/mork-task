use mork_hal::context::{HALContext, HALContextTrait};
use alloc::sync::Arc;
use mork_common::constants::MAX_THREAD_PIRO;
use mork_common::types::SyncUnsafeCell;
use mork_hal::idle_thread;
use mork_hal::mm::PageTableImpl;
use crate::task_state::ThreadStateEnum;

#[allow(dead_code)]
pub struct TaskContext {
    pub hal_context: HALContext,
    pub prio: usize,
    pub vspace: Option<Arc<SyncUnsafeCell<PageTableImpl>>>,
    pub state: ThreadStateEnum
}
impl TaskContext {
    pub fn new_user_thread() -> Self {
        let mut hal_context = HALContext::new();
        hal_context.set_user_flag(true);
        hal_context.set_interrupt_enable(true);
        Self {
            hal_context,
            prio: MAX_THREAD_PIRO - 1,
            vspace: None,
            state: ThreadStateEnum::ThreadStateInactive,
        }
    }

    pub fn new_idle_thread(stack_ptr: usize) -> Self {
        let mut hal_context = HALContext::new();
        hal_context.set_stack(stack_ptr);
        hal_context.set_next_ip(idle_thread as usize);
        hal_context.set_interrupt_enable(true);
        hal_context.set_user_flag(false);
        Self {
            hal_context,
            prio: MAX_THREAD_PIRO,
            vspace: None,
            state: ThreadStateEnum::ThreadStateIdleThreadState
        }
    }
}