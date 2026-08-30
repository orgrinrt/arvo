	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind
	.p2align	2
__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind:
LBB0_1:
	b	LBB0_1

	.globl	_w31_gather
	.p2align	2
_w31_gather:
	stp	x20, x19, [sp, #-16]!
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	mov	w12, #93
	mov	w13, #1000
	mov	w14, #5
LBB1_1:
	sub	x15, x12, #93
	sub	x16, x12, #62
	sub	x17, x12, #31
	add	x1, x0, x15, lsr #3
	add	x2, x0, x16, lsr #3
	add	x3, x0, x17, lsr #3
	add	x4, x0, x12, lsr #3
	ldr	w5, [x1]
	ldr	w6, [x2]
	ldr	w7, [x3]
	ldr	w19, [x4]
	ldrb	w1, [x1, #4]
	ldrb	w2, [x2, #4]
	ldrb	w3, [x3, #4]
	orr	x1, x5, x1, lsl #32
	orr	x2, x6, x2, lsl #32
	orr	x3, x7, x3, lsl #32
	ldrb	w4, [x4, #4]
	orr	x4, x19, x4, lsl #32
	and	x15, x15, #0x4
	and	x16, x16, #0x7
	and	x17, x17, #0x6
	and	x5, x12, x14
	lsr	x15, x1, x15
	lsr	x16, x2, x16
	lsr	x17, x3, x17
	lsr	x1, x4, x5
	and	x15, x15, #0x7fffffff
	and	x16, x16, #0x7fffffff
	and	x17, x17, #0x7fffffff
	and	x1, x1, #0x7fffffff
	add	x8, x15, x8
	add	x9, x16, x9
	add	x10, x17, x10
	add	x11, x1, x11
	add	x12, x12, #124
	subs	x13, x13, #4
	b.ne	LBB1_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ldp	x20, x19, [sp], #16
	ret

	.globl	_w31_hand
	.p2align	2
_w31_hand:
	stp	x20, x19, [sp, #-16]!
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	mov	w12, #93
	mov	w13, #1000
	mov	w14, #5
LBB2_1:
	sub	x15, x12, #93
	sub	x16, x12, #62
	sub	x17, x12, #31
	and	x1, x15, #0x4
	and	x2, x16, #0x7
	and	x3, x17, #0x6
	add	x15, x0, x15, lsr #3
	add	x16, x0, x16, lsr #3
	add	x17, x0, x17, lsr #3
	add	x4, x0, x12, lsr #3
	ldr	w5, [x15]
	ldr	w6, [x16]
	ldr	w7, [x17]
	ldr	w19, [x4]
	ldrb	w15, [x15, #4]
	ldrb	w16, [x16, #4]
	ldrb	w17, [x17, #4]
	orr	x15, x5, x15, lsl #32
	ldrb	w4, [x4, #4]
	orr	x16, x6, x16, lsl #32
	orr	x17, x7, x17, lsl #32
	orr	x4, x19, x4, lsl #32
	and	x5, x12, x14
	lsr	x15, x15, x1
	lsr	x16, x16, x2
	lsr	x17, x17, x3
	lsr	x1, x4, x5
	and	x15, x15, #0x7fffffff
	and	x16, x16, #0x7fffffff
	and	x17, x17, #0x7fffffff
	and	x1, x1, #0x7fffffff
	add	x8, x15, x8
	add	x9, x16, x9
	add	x10, x17, x10
	add	x11, x1, x11
	add	x12, x12, #124
	subs	x13, x13, #4
	b.ne	LBB2_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ldp	x20, x19, [sp], #16
	ret

	.globl	_w31_native
	.p2align	2
_w31_native:
	add	x8, x0, #32
	movi.2d	v0, #0000000000000000
	mov	w9, #992
	movi.2d	v1, #0000000000000000
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
LBB3_1:
	ldp	q4, q5, [x8, #-32]
	ldp	q6, q7, [x8], #64
	bic.4s	v4, #128, lsl #24
	bic.4s	v5, #128, lsl #24
	bic.4s	v6, #128, lsl #24
	bic.4s	v7, #128, lsl #24
	uaddw.2d	v0, v0, v4
	uaddw2.2d	v0, v0, v4
	uaddw.2d	v1, v1, v5
	uaddw2.2d	v1, v1, v5
	uaddw.2d	v2, v2, v6
	uaddw2.2d	v2, v2, v6
	uaddw.2d	v3, v3, v7
	uaddw2.2d	v3, v3, v7
	subs	x9, x9, #16
	b.ne	LBB3_1
	add.2d	v0, v1, v0
	add.2d	v1, v3, v2
	add.2d	v0, v1, v0
	addp.2d	d0, v0
	movi.2d	v1, #0000000000000000
	mov.d	v1[0], v0[0]
	ldr	q0, [x0, #3968]
	bic.4s	v0, #128, lsl #24
	uaddw.2d	v1, v1, v0
	uaddw2.2d	v0, v1, v0
	ldr	q1, [x0, #3984]
	bic.4s	v1, #128, lsl #24
	uaddw.2d	v0, v0, v1
	uaddw2.2d	v0, v0, v1
	addp.2d	d0, v0
	fmov	x0, d0
	ret

	.globl	_w31_typed
	.p2align	2
_w31_typed:
	mov	x9, #0
	mov	x8, #0
	mov	w10, #1000
LBB4_1:
	add	x11, x0, x9, lsr #3
	ldr	w12, [x11]
	ldrb	w11, [x11, #4]
	orr	x11, x12, x11, lsl #32
	and	x12, x9, #0x7
	lsr	x11, x11, x12
	and	x11, x11, #0x7fffffff
	add	x8, x11, x8
	add	x9, x9, #31
	subs	x10, x10, #1
	b.ne	LBB4_1
	mov	x0, x8
	ret

	.globl	_w31_wide
	.p2align	2
_w31_wide:
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	mov	w12, #93
	mov	w13, #1000
	mov	w14, #5
LBB5_1:
	sub	x15, x12, #93
	sub	x16, x12, #62
	sub	x17, x12, #31
	lsr	x1, x15, #3
	lsr	x2, x16, #3
	lsr	x3, x17, #3
	lsr	x4, x12, #3
	ldr	x1, [x0, x1]
	ldr	x2, [x0, x2]
	ldr	x3, [x0, x3]
	ldr	x4, [x0, x4]
	and	x15, x15, #0x4
	and	x16, x16, #0x7
	and	x17, x17, #0x6
	and	x5, x12, x14
	lsr	x15, x1, x15
	lsr	x16, x2, x16
	lsr	x17, x3, x17
	lsr	x1, x4, x5
	and	x15, x15, #0x7fffffff
	and	x16, x16, #0x7fffffff
	and	x17, x17, #0x7fffffff
	and	x1, x1, #0x7fffffff
	add	x8, x15, x8
	add	x9, x16, x9
	add	x10, x17, x10
	add	x11, x1, x11
	add	x12, x12, #124
	subs	x13, x13, #4
	b.ne	LBB5_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret

.subsections_via_symbols
