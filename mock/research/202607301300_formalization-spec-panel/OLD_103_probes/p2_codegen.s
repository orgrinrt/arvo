	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind
	.p2align	2
__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind:
LBB0_1:
	b	LBB0_1

	.globl	_run_a
	.p2align	2
_run_a:
	stp	x24, x23, [sp, #-64]!
	stp	x22, x21, [sp, #16]
	stp	x20, x19, [sp, #32]
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	cbz	x1, LBB1_6
	mov	x20, x0
	mov	w19, #0
	mov	w22, #0
	add	x21, x0, x1, lsl #2
LBB1_2:
	mov	x23, x20
LBB1_3:
	ldr	w24, [x23], #4
	cmp	w22, w24
	cset	w0, hs
	bl	__RNvMCsjFdVLPp646J_7p1_arvoNtB2_4Bool3new
	tbz	w0, #0, LBB1_5
	add	w22, w24, w22
	mov	x20, x23
	cmp	x23, x21
	b.ne	LBB1_3
	b	LBB1_7
LBB1_5:
	add	w19, w19, #1
	add	x20, x20, #4
	mov	x22, x24
	cmp	x23, x21
	b.ne	LBB1_2
	b	LBB1_7
LBB1_6:
	mov	w19, #0
LBB1_7:
	mov	x0, x19
	ldp	x29, x30, [sp, #48]
	ldp	x20, x19, [sp, #32]
	ldp	x22, x21, [sp, #16]
	ldp	x24, x23, [sp], #64
	ret

	.globl	_run_b2
	.p2align	2
_run_b2:
	.cfi_startproc
	stp	x24, x23, [sp, #-64]!
	stp	x22, x21, [sp, #16]
	stp	x20, x19, [sp, #32]
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	.cfi_offset w23, -56
	.cfi_offset w24, -64
	cbz	x1, LBB2_4
	mov	x19, x0
	mov	w21, #0
	mov	w20, #0
	lsl	x22, x1, #2
LBB2_2:
	ldr	w23, [x19], #4
	cmp	w21, w23
	cset	w0, hs
	bl	__RNvMCsjFdVLPp646J_7p1_arvoNtB2_4Bool3new
	cmp	w0, #0
	csel	w8, w21, wzr, ne
	add	w21, w8, w23
	eor	w8, w0, #0x1
	add	w20, w20, w8
	subs	x22, x22, #4
	b.ne	LBB2_2
	mov	x0, x20
	ldp	x29, x30, [sp, #48]
	ldp	x20, x19, [sp, #32]
	ldp	x22, x21, [sp, #16]
	ldp	x24, x23, [sp], #64
	ret
LBB2_4:
	mov	w20, #0
	mov	x0, x20
	ldp	x29, x30, [sp, #48]
	ldp	x20, x19, [sp, #32]
	ldp	x22, x21, [sp, #16]
	ldp	x24, x23, [sp], #64
	ret
	.cfi_endproc

	.globl	_run_bare
	.p2align	2
_run_bare:
	cbz	x1, LBB3_4
	mov	w10, #0
	mov	w8, #0
	lsl	x9, x1, #2
LBB3_2:
	ldr	w11, [x0], #4
	cmp	w10, w11
	csel	w10, wzr, w10, lo
	cinc	w8, w8, lo
	add	w10, w10, w11
	subs	x9, x9, #4
	b.ne	LBB3_2
	mov	x0, x8
	ret
LBB3_4:
	mov	w8, #0
	mov	x0, x8
	ret

	.globl	_run_b1
_run_b1 = _run_a
.subsections_via_symbols
