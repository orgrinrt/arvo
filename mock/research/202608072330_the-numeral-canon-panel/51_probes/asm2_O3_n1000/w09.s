	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_w9_hand
	.p2align	2
_w9_hand:
	.cfi_startproc
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	mov	w12, #27
	mov	w13, #1000
	mov	w14, #5
LBB0_1:
	sub	x15, x12, #27
	sub	x16, x12, #18
	sub	x17, x12, #9
	and	x1, x15, #0x4
	and	x2, x16, x14
	and	x3, x17, #0x6
	and	x4, x12, #0x7
	lsr	x15, x15, #3
	lsr	x16, x16, #3
	lsr	x17, x17, #3
	lsr	x5, x12, #3
	ldrh	w15, [x0, x15]
	ldrh	w16, [x0, x16]
	ldrh	w17, [x0, x17]
	ldrh	w5, [x0, x5]
	lsr	x15, x15, x1
	lsr	x16, x16, x2
	lsr	x17, x17, x3
	lsr	x1, x5, x4
	and	x15, x15, #0x1ff
	and	x16, x16, #0x1ff
	and	x17, x17, #0x1ff
	and	x1, x1, #0x1ff
	add	x8, x15, x8
	add	x9, x16, x9
	add	x10, x17, x10
	add	x11, x1, x11
	add	x12, x12, #36
	subs	x13, x13, #4
	b.ne	LBB0_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret
	.cfi_endproc

	.globl	_w9_native
	.p2align	2
_w9_native:
	.cfi_startproc
	add	x8, x0, #32
	movi.2d	v0, #0000000000000000
	mov	w9, #992
	movi.2d	v1, #0000000000000000
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
LBB1_1:
	ldp	q4, q5, [x8, #-32]
	ldp	q6, q7, [x8], #64
	bic.8h	v4, #254, lsl #8
	bic.8h	v5, #254, lsl #8
	bic.8h	v6, #254, lsl #8
	bic.8h	v7, #254, lsl #8
	ushll2.4s	v16, v4, #0
	ushll.4s	v4, v4, #0
	uaddw.2d	v0, v0, v4
	uaddw2.2d	v0, v0, v4
	uaddw.2d	v0, v0, v16
	uaddw2.2d	v0, v0, v16
	ushll2.4s	v4, v5, #0
	ushll.4s	v5, v5, #0
	uaddw.2d	v1, v1, v5
	uaddw2.2d	v1, v1, v5
	uaddw.2d	v1, v1, v4
	uaddw2.2d	v1, v1, v4
	ushll2.4s	v4, v6, #0
	ushll.4s	v5, v6, #0
	uaddw.2d	v2, v2, v5
	uaddw2.2d	v2, v2, v5
	uaddw.2d	v2, v2, v4
	uaddw2.2d	v2, v2, v4
	ushll2.4s	v4, v7, #0
	ushll.4s	v5, v7, #0
	uaddw.2d	v3, v3, v5
	uaddw2.2d	v3, v3, v5
	uaddw.2d	v3, v3, v4
	uaddw2.2d	v3, v3, v4
	subs	x9, x9, #32
	b.ne	LBB1_1
	add.2d	v0, v1, v0
	add.2d	v1, v3, v2
	add.2d	v0, v1, v0
	addp.2d	d0, v0
	movi.2d	v1, #0000000000000000
	mov.d	v1[0], v0[0]
	ldr	q0, [x0, #1984]
	bic.8h	v0, #254, lsl #8
	ushll2.4s	v2, v0, #0
	ushll.4s	v0, v0, #0
	uaddw.2d	v1, v1, v0
	uaddw2.2d	v0, v1, v0
	uaddw.2d	v0, v0, v2
	uaddw2.2d	v0, v0, v2
	addp.2d	d0, v0
	fmov	x0, d0
	ret
	.cfi_endproc

	.globl	_w9_typed
	.p2align	2
_w9_typed:
	.cfi_startproc
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	mov	w12, #27
	mov	w13, #1000
	mov	w14, #5
LBB2_1:
	sub	x15, x12, #27
	sub	x16, x12, #18
	sub	x17, x12, #9
	lsr	x1, x15, #3
	lsr	x2, x16, #3
	lsr	x3, x17, #3
	lsr	x4, x12, #3
	ldrh	w1, [x0, x1]
	ldrh	w2, [x0, x2]
	ldrh	w3, [x0, x3]
	ldrh	w4, [x0, x4]
	and	x15, x15, #0x4
	and	x16, x16, x14
	and	x17, x17, #0x6
	and	x5, x12, #0x7
	lsr	x15, x1, x15
	lsr	x16, x2, x16
	lsr	x17, x3, x17
	lsr	x1, x4, x5
	and	x15, x15, #0x1ff
	and	x16, x16, #0x1ff
	and	x17, x17, #0x1ff
	and	x1, x1, #0x1ff
	add	x8, x15, x8
	add	x9, x16, x9
	add	x10, x17, x10
	add	x11, x1, x11
	add	x12, x12, #36
	subs	x13, x13, #4
	b.ne	LBB2_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret
	.cfi_endproc

.subsections_via_symbols
