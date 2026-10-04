# 2 numbers sum - read 2 numbers using scanf and write result using printf
# scanf and printf are available from libc because we use entrypoint main which happens after libc initialization
# to run use the cmd:
#    clang -no-pie sum.s -o sum && ./sum

.global main

.section .rodata
    fmt_input: .asciz "%d %d"
    fmt_output: .asciz "Result is %d\n"

.section .bss       # for uninitialized variables
    a:  .zero 4
    b:  .zero 4

.section .text

main:
    push %rbp
    mov %rsp, %rbp
    sub $16, %rsp      # align rsp for scanf

    lea fmt_input(%rip), %rdi
    lea a(%rip), %rsi
    lea b(%rip), %rdx
    xor %eax, %eax
    call scanf@PLT

    mov a(%rip), %esi   # esi to store output because we use 4 bytes for each number
    add  b(%rip), %esi

    lea fmt_output(%rip), %rdi      # register rdi for output string, register rsi for sum result
    xor %eax, %eax
    call printf@PLT

    xor %eax, %eax      # return 0
    leave 
    ret
    
    .section .note.GNU-stack,"",@progbits