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
	cbz	x1, LBB1_6
	mov	x8, x0
	mov	w0, #0
	mov	w10, #0
	add	x9, x8, x1, lsl #2
LBB1_2:
	mov	x11, x8
LBB1_3:
	ldr	w12, [x11], #4
	cmp	w10, w12
	b.lo	LBB1_5
	add	w10, w12, w10
	mov	x8, x11
	cmp	x11, x9
	b.ne	LBB1_3
	b	LBB1_7
LBB1_5:
	add	w0, w0, #1
	add	x8, x8, #4
	mov	x10, x12
	cmp	x11, x9
	b.ne	LBB1_2
	b	LBB1_7
LBB1_6:
	mov	w0, #0
LBB1_7:
	ret

	.globl	_run_b2
	.p2align	2
_run_b2:
	.cfi_startproc
	cbz	x1, LBB2_4
	mov	w9, #0
	mov	w8, #0
	lsl	x10, x1, #2
LBB2_2:
	ldr	w11, [x0], #4
	cmp	w9, w11
	csel	w9, wzr, w9, lo
	add	w9, w9, w11
	cinc	w8, w8, lo
	subs	x10, x10, #4
	b.ne	LBB2_2
	mov	x0, x8
	ret
LBB2_4:
	mov	w8, #0
	mov	x0, x8
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
