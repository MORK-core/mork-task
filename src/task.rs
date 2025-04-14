use alloc::boxed::Box;
use mork_hal::context::{HALContext, HALContextTrait};
use mork_capability::cap::{Cap, CapType, PageTableCap, ThreadCap};
use mork_capability::cnode::{CapIndex, CapNode};
use mork_common::constants::{CNodeSlot, MAX_THREAD_PIRO};
use mork_common::mork_kernel_log;
use mork_common::syscall::ipc_buffer::IPCBuffer;
use mork_hal::idle_thread;
use mork_mm::page_table::PageTable;
use crate::task_state::ThreadStateEnum;
use crate::TIME_SLICE;

#[allow(dead_code)]
#[repr(C, align(4096))]
pub struct TaskContext {
    pub hal_context: HALContext,
    pub prio: usize,
    pub cspace: Option<Box<CapNode>>,
    pub ipc_buffer: Option<CapIndex>,
    pub is_queued: bool,
    pub time_slice: usize,
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
            cspace: None,
            ipc_buffer: None,
            is_queued: false,
            time_slice: TIME_SLICE,
            state: ThreadStateEnum::ThreadStateInactive,
        }
    }

    pub fn init_cspace(&mut self) {
        self.cspace = Some(Box::new(CapNode::new()));
        let thread_cap = ThreadCap::new(self.get_ptr());
        let cap_node = self.cspace.as_mut().unwrap();
        cap_node[CNodeSlot::CapInitThread as usize] = thread_cap.into_cap();
        cap_node[CNodeSlot::CapInitVSpace as usize] = Cap::default();
    }

    pub fn init_vspace(&mut self, page_table: Box<PageTable>) {
        let vspace_cap = PageTableCap::new(page_table.get_ptr());
        self.cspace.as_mut().unwrap()[CNodeSlot::CapInitVSpace as usize] = vspace_cap.into_cap();
        Box::leak(page_table);
    }

    pub fn get_vspace(&self) -> Option<&PageTable> {
        self.get_page_table_cap()
            .map(|cap| &*PageTable::from_cap(cap))
    }

    pub fn get_vspace_mut(&mut self) -> Option<&mut PageTable> {
        self.get_page_table_cap()
            .map(|cap| PageTable::from_cap(cap))
    }

    pub fn get_ipc_buffer(&self) -> Option<&IPCBuffer> {
        self.get_ipc_buffer_ptr()
            .map(|ptr| unsafe { &*(ptr as *const IPCBuffer) })
    }

    pub fn get_ipc_buffer_mut(&mut self) -> Option<&mut IPCBuffer> {
        self.get_ipc_buffer_ptr()
            .map(|ptr| unsafe { &mut *(ptr as *mut IPCBuffer) })
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
            cspace: None,
            ipc_buffer: None,
            is_queued: false,
            time_slice: TIME_SLICE,
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

    fn get_ipc_buffer_ptr(&self) -> Option<usize> {
        let cspace = self.cspace.as_ref()?;
        let cap = &cspace[self.ipc_buffer?];
        if cap.get_type() != CapType::Frame {
            return None;
        }
        Some(unsafe { (cap.frame_cap.base_ptr() << 12) as usize })
    }

    fn get_page_table_cap(&self) -> Option<&PageTableCap> {
        let cspace = self.cspace.as_ref()?;
        let cap = &cspace[CNodeSlot::CapInitVSpace as usize];
        if cap.get_type() != CapType::PageTable {
            mork_kernel_log!(warn, "error type: {:?}", cap.get_type());
            return None;
        }
        Some(unsafe { &cap.page_table_cap })
    }
}