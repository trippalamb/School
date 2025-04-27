@ (Tripp) Milton Lamb
@ mal0004@uah.edu
@ CS513-01 Spring 2025
@ This program prompts the user for an integer between 0 and 10 inclusive and then prints
@ "Hello World." that number of times and exits. It also provides validation for the input.

.data
    greeting:       .asciz  "Welcome! Please enter an integer between 0 and 10 (inclusive).\n"
    hello:          .asciz  "Hello World.\n"
    too_high_err:   .asciz  "Your number must be 10 or less.\n"
    too_low_err:    .asciz  "Your number must be 0 or more.\n"
    format:         .asciz  "%d"
    input:          .word   0

.text
.global main

main:

    push    {lr}
    
    ldr     r0, =greeting
    bl      printf

    ldr     r0, =format
    ldr     r1, =input
    bl      scanf

    ldr     r4, =input
    ldr     r4, [r4]

    cmp     r4, #10
    bgt     too_high

    cmp     r4, #0
    blt     too_low

    beq     done

loop:


    ldr     r0, =hello
    bl      printf


    sub     r4, r4, #1
    cmp     r4, #0
    bne     loop

done:
    pop     {lr}
    bx      lr

too_high:
    ldr     r0, =too_high_err
    bl      printf
    b       done

too_low:
    ldr     r0, =too_low_err
    bl      printf
    b       done
