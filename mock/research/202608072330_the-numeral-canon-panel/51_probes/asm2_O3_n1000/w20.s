	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_w20_hand
	.p2align	2
_w20_hand:
	.cfi_startproc
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	add	x12, x0, #7
	mov	w13, #1000
LBB0_1:
	ldur	w14, [x12, #-7]
	ldur	w15, [x12, #-5]
	ldur	w16, [x12, #-2]
	and	x14, x14, #0xfffff
	ubfx	x15, x15, #4, #20
	ldr	w17, [x12], #10
	and	x16, x16, #0xfffff
	ubfx	x17, x17, #4, #20
	add	x8, x8, x14
	add	x9, x9, x15
	add	x10, x10, x16
	add	x11, x11, x17
	subs	x13, x13, #4
	b.ne	LBB0_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret
	.cfi_endproc

	.globl	_w20_native
	.p2align	2
_w20_native:
	.cfi_startproc
	add	x8, x0, #32
	movi.2d	v0, #0000000000000000
	mov	w9, #992
	movi.4s	v1, #15, msl #16
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
	movi.2d	v4, #0000000000000000
LBB1_1:
	ldp	q5, q6, [x8, #-32]
	ldp	q7, q16, [x8], #64
	and.16b	v5, v5, v1
	and.16b	v6, v6, v1
	and.16b	v7, v7, v1
	and.16b	v16, v16, v1
	uaddw.2d	v0, v0, v5
	uaddw2.2d	v0, v0, v5
	uaddw.2d	v2, v2, v6
	uaddw2.2d	v2, v2, v6
	uaddw.2d	v3, v3, v7
	uaddw2.2d	v3, v3, v7
	uaddw.2d	v4, v4, v16
	uaddw2.2d	v4, v4, v16
	subs	x9, x9, #16
	b.ne	LBB1_1
	add.2d	v0, v2, v0
	add.2d	v1, v4, v3
	add.2d	v0, v1, v0
	addp.2d	d0, v0
	movi.2d	v1, #0000000000000000
	mov.d	v1[0], v0[0]
	ldr	q0, [x0, #3968]
	movi.4s	v2, #15, msl #16
	and.16b	v0, v0, v2
	uaddw.2d	v1, v1, v0
	uaddw2.2d	v0, v1, v0
	ldr	q1, [x0, #3984]
	and.16b	v1, v1, v2
	uaddw.2d	v0, v0, v1
	uaddw2.2d	v0, v0, v1
	addp.2d	d0, v0
	fmov	x0, d0
	ret
	.cfi_endproc

	.globl	_w20_typed
	.p2align	2
_w20_typed:
	.cfi_startproc
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	add	x12, x0, #4
	mov	w13, #1000
LBB2_1:
	ldurb	w14, [x12, #-2]
	ldrb	w15, [x12, #3]
	ldurb	w16, [x12, #-1]
	ldrb	w17, [x12, #4]
	ldurh	w0, [x12, #-4]
	orr	x16, x14, x16, lsl #8
	ldurh	w1, [x12, #1]
	orr	x17, x15, x17, lsl #8
	ldrb	w2, [x12]
	orr	x16, x16, x2, lsl #16
	ldrb	w2, [x12, #5]
	orr	x17, x17, x2, lsl #16
	bfi	x0, x14, #16, #4
	bfi	x1, x15, #16, #4
	add	x8, x0, x8
	add	x9, x9, x16, lsr #4
	add	x10, x1, x10
	add	x11, x11, x17, lsr #4
	add	x12, x12, #10
	subs	x13, x13, #4
	b.ne	LBB2_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret
	.cfi_endproc

.subsections_via_symbols
