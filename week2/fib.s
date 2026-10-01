/*
    Program to calculate N fib number
    rax and rsi are used to get current and next numbers
    rdx is used as tmp variable to hold rax + rsi value

to run use the cmd:
    clang -no-pie fib.s -o fib && ./fib
*/

.global main

.section .rodata
    fmt_input: .asciz "%d"
    fmt_output: .asciz "Fib result is %d\n"

.section .bss
    n:   .zero 4

.section .text
main:
    push %rbp
    mov %rsp, %rbp
    sub $16, %rsp

    lea fmt_input(%rip), %rdi
    lea n(%rip), %rsi
    xor %eax, %eax
    call scanf@PLT

    mov n(%rip), %rcx
    dec %rcx

    # fib initialization
    xor %eax, %eax
    movq $1, %rsi

loop:
    cmp $0, %rcx
    je done
    dec %rcx
    leaq (%rax, %rsi, 1), %rdx      # similar to %rdx = %rax + %rsi * 1
    mov %rsi, %rax
    mov %rdx, %rsi
    jmp loop
done:
    lea fmt_output(%rip), %rdi
    mov %rax, %rsi
    xor %eax, %eax
    call printf@PLT
    leave
    ret

    .section .note.GNU-stack,"",@progbits
