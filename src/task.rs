use alloc::boxed::Box;
use mork_hal::context::{HALContext, HALContextTrait};
use alloc::sync::Arc;
use mork_capability::cap::ThreadCap;
use mork_capability::cnode::CapNode;
use mork_common::constants::MAX_THREAD_PIRO;
use mork_common::syscall::ipc_buffer::IPCBuffer;
use mork_common::types::SyncUnsafeCell;
use mork_hal::idle_thread;
use mork_hal::mm::PageTableImpl;
use crate::task_state::ThreadStateEnum;

#[allow(dead_code)]
#[repr(C, align(4096))]
pub struct TaskContext {
    pub hal_context: HALContext,
    pub prio: usize,
    pub vspace: Option<Arc<SyncUnsafeCell<PageTableImpl>>>,
    pub cspace: Option<Arc<SyncUnsafeCell<CapNode>>>,
    pub ipc_buffer: Option<Box<IPCBuffer>>,

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
            cspace: None,
            ipc_buffer: None,
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
            cspace: None,
            ipc_buffer: None,
            state: ThreadStateEnum::ThreadStateIdleThreadState
        }
    }

    pub fn from_cap(cap: &ThreadCap) -> &mut Self {
        unsafe {
            &mut *((cap.base_ptr() << 12) as usize as *mut Self)
        }
    }

    pub fn get_ptr(&self) -> usize {
        self as *const _ as usize
    }
}