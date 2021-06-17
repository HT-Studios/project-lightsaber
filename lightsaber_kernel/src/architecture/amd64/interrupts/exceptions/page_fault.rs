use x86_64::registers::control::Cr2;

use crate::architecture::amd64::interrupts::idt::stack_frame::InterruptErrorStack;

#[no_mangle]
pub(in super) unsafe extern "C" fn __interrupt_page_fault(interrupt_stack: *mut InterruptErrorStack) {
    fn __priv_inner(stack: &mut InterruptErrorStack) {
        let accessed_address = Cr2::read();

        panic!("Page fault; accessed address: {:?}, stack: {:#?}. (INT 14 (0xE), PAGE_FAULT)", accessed_address, stack);
    }

    __priv_inner(&mut *interrupt_stack);
}

global_asm!(r#".intel_syntax noprefix
.global page_fault
.type page_fault, @function
.section .text.page_fault, "ax", @progbits
page_fault:
    test QWORD PTR [rsp + 16], 0x3
    jz 1f
    swapgs
    1:
    xchg [rsp], rax

    push rcx
    push rdx
    push rdi
    push rsi
    push r8
    push r9
    push r10
    push r11

    push rbx
    push rbp
    push r12
    push r13
    push r14
    push r15

    push fs
    mov rcx, 0x18
    mov fs, cx
    push rax

call map_pti
mov rdi, rsp
call __interrupt_page_fault
call unmap_pti
add rsp, 8

    pop fs

    pop r15
    pop r14
    pop r13
    pop r12
    pop rbp
    pop rbx

    pop r11
    pop r10
    pop r9
    pop r8
    pop rsi
    pop rdi
    pop rdx
    pop rcx
    pop rax

    test QWORD PTR [rsp + 16], 0x3
    jz 1f
    swapgs
    1:
    iretq
.size page_fault, . - page_fault
.text
.att_syntax prefix"#
);

extern "C" {
    pub fn page_fault();
}