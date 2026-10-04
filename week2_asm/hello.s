# program to print "hello world"-like message
# to run use the cmd:
#    clang -nostdlib -no-pie hello.s -o hello && ./hello
.global _start

.section .rodata
msg:
    .asciz "Hello rusty world\n"  # null terminated string because of .asciZ - ascii + zero
msg_length = . - msg              # to calculate length of msg - current address - address where msg is located

.section .text
_start:
    mov $1, %eax            # syscall write
    movq $1, %rdi            # stdout as output
    leaq msg(%rip), %rsi    # read from
    movq $msg_length, %rdx   # how many bytes to read
    syscall  

    # successful exit
    mov $60, %eax           # syscall exit
    xorq %rdi, %rdi          # exit code - 0
    syscall
