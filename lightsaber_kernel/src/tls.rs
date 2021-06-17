use core::{
    alloc::Layout,
    mem,
    ptr
};

use alloc::alloc::{
    Allocator,
    Global,
    alloc_zeroed
};

use spin::Once;

use lightsaber_util::libcore_ext::{
    io,
    linker::LinkerSymbol
};

use crate::{architecture, ALLOCATOR};

pub(in self) static THREAD_LOCAL_STORAGE: Once<ThreadLocalStorage> = Once::new();

#[repr(C)]
pub(in self) struct ThreadControlBlock {
    self_ptr: *const Self
}

#[repr(C)]
pub(in self) struct ThreadLocalStorage {
    pointer: usize,
    tcb_offset: usize
}

impl ThreadLocalStorage {
    #[inline]
    pub(in self) fn new(pointer: usize, tcb_offset: usize) -> Self {
        Self {
            pointer,
            tcb_offset
        }
    }
}

pub fn initialize_tls() {
    extern "C" {
        static __tdata_start: LinkerSymbol;
        static __tdata_end: LinkerSymbol;
        static __tbss_start: LinkerSymbol;
        static __tbss_end: LinkerSymbol;
    }

    let total_size: usize = unsafe {
        __tbss_end.as_usize() - __tdata_start.as_usize()
    };
    let tdata_size: usize = unsafe {
        __tdata_end.as_usize() - __tdata_start.as_usize()
    };

    let total_tls_size = total_size + mem::size_of::<ThreadControlBlock>();
    let tls_layout = unsafe {
        Layout::from_size_align_unchecked(total_tls_size, mem::align_of::<ThreadControlBlock>())
    };

    let tls_rawptr = unsafe {
        alloc_zeroed(tls_layout)
    };
    let tls_offset = tls_rawptr as usize;

    unsafe {
        ptr::copy(__tdata_start.as_ptr(), tls_rawptr, tdata_size);
        ptr::write_bytes(
            (tls_offset + tdata_size) as *mut u8,
            0,
           total_tls_size - tdata_size
        );
    }

    let tcb_ptr = ((tls_rawptr as u64) + (total_size as u64)) as *mut u64;
    let tcb_offset = tcb_ptr as usize;

    unsafe {
        io::wrmsr(io::INTEL_ARCHITECTURE_32BIT_FS_BASE, tcb_offset as u64);
        *tcb_ptr = tcb_offset as u64;
    }

    THREAD_LOCAL_STORAGE.call_once(move || ThreadLocalStorage::new(tls_offset, tcb_offset));

    #[cfg(target_arch = "x86_64")]
    unsafe {
        architecture::amd64::gdt::PROCESSOR_CONTROL_REGION.fs_offset = tcb_offset as usize;
    }
}
