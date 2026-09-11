	.syntax	unified
	.eabi_attribute	67, "2.09"
	.eabi_attribute	6, 12
	.eabi_attribute	7, 77
	.eabi_attribute	8, 0
	.eabi_attribute	9, 1
	.eabi_attribute	34, 0
	.eabi_attribute	17, 1
	.eabi_attribute	20, 1
	.eabi_attribute	21, 1
	.eabi_attribute	23, 3
	.eabi_attribute	24, 1
	.eabi_attribute	25, 1
	.eabi_attribute	38, 1
	.eabi_attribute	14, 0
	.file	"arm.c282f498cbbce42f-cgu.0"
	.section	.text.observe_arm,"ax",%progbits
	.globl	observe_arm
	.p2align	1
	.type	observe_arm,%function
	.code	16
	.thumb_func
observe_arm:
	.fnstart
	.save	{r7, lr}
	push	{r7, lr}
	.setfp	r7, sp
	add	r7, sp, #0
	movs	r0, #2
	pop	{r7, pc}
.Lfunc_end0:
	.size	observe_arm, .Lfunc_end0-observe_arm
	.cantunwind
	.fnend

	.ident	"rustc version 1.98.0-nightly (57d06900f 2026-05-27)"
	.section	".note.GNU-stack","",%progbits
	.eabi_attribute	30, 2
