use crate::architecture::amd64::interrupts::idt::stack_frame::InterruptErrorStack;

#[no_mangle]
pub(in super) unsafe extern "C" fn __interrupt_device_unavailable(stack: *mut InterruptErrorStack) {
    fn __priv_inner(_stack: &mut InterruptErrorStack) {
        panic!("Device unavailable. (INT 7 (0x7), DEVICE_UNAVAILABLE_EXCEPTION)");
    }

    __priv_inner(&mut *stack);
}

global_asm!(r#".intel_syntax noprefix
.global device_unavailable
.type device_unavailable, @function
.section .text.device_unavailable, "ax", @progbits
device_unavailable:
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
call __interrupt_device_unavailable
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
.size device_unavailable, . - device_unavailable
.text
.att_syntax prefix"#
);

extern "C" {
    pub fn device_unavailable();
}