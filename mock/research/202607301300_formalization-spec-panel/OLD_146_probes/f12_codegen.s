	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_loop_by_hand
	.p2align	2
_loop_by_hand:
	.cfi_startproc
	cmp	x1, x3
	csel	x8, x1, x3, lo
	cbz	x8, LBB0_14
	cmp	x8, #4
	b.hs	LBB0_3
	mov	x9, #0
	b	LBB0_12
LBB0_3:
	cmp	x8, #16
	b.hs	LBB0_5
	mov	x9, #0
	b	LBB0_9
LBB0_5:
	and	x10, x8, #0xc
	and	x9, x8, #0x1ffffffffffffff0
	add	x11, x2, #32
	add	x12, x0, #32
	and	x13, x8, #0x1ffffffffffffff0
LBB0_6:
	ldp	q0, q1, [x12, #-32]
	ldp	q2, q3, [x12], #64
	shl.4s	v0, v0, #5
	shl.4s	v1, v1, #5
	shl.4s	v2, v2, #5
	shl.4s	v3, v3, #5
	stp	q0, q1, [x11, #-32]
	stp	q2, q3, [x11], #64
	subs	x13, x13, #16
	b.ne	LBB0_6
	cmp	x8, x9
	b.eq	LBB0_14
	cbz	x10, LBB0_12
LBB0_9:
	mov	x11, x9
	and	x9, x8, #0x1ffffffffffffffc
	sub	x10, x11, x9
	lsl	x12, x11, #2
	add	x11, x0, x12
	add	x12, x2, x12
LBB0_10:
	ldr	q0, [x11], #16
	shl.4s	v0, v0, #5
	str	q0, [x12], #16
	adds	x10, x10, #4
	b.ne	LBB0_10
	cmp	x8, x9
	b.eq	LBB0_14
LBB0_12:
	sub	x8, x8, x9
	lsl	x10, x9, #2
	add	x9, x2, x10
	add	x10, x0, x10
LBB0_13:
	ldr	w11, [x10], #4
	lsl	w11, w11, #5
	str	w11, [x9], #4
	subs	x8, x8, #1
	b.ne	LBB0_13
LBB0_14:
	ret
	.cfi_endproc

	.globl	_scalar_by_hand
	.p2align	2
_scalar_by_hand:
	.cfi_startproc
	lsl	w0, w0, #5
	ret
	.cfi_endproc

	.globl	_loop_via_conversion
_loop_via_conversion = _loop_by_hand
	.globl	_scalar_via_conversion
_scalar_via_conversion = _scalar_by_hand
.subsections_via_symbols
