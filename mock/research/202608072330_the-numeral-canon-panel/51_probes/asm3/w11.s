	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind
	.p2align	2
__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind:
LBB0_1:
	b	LBB0_1

	.globl	_w11_gather
	.p2align	2
_w11_gather:
	stp	x20, x19, [sp, #-16]!
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	mov	w12, #33
	mov	w13, #1000
	mov	w14, #5
LBB1_1:
	sub	x15, x12, #33
	sub	x16, x12, #22
	sub	x17, x12, #11
	add	x1, x0, x15, lsr #3
	add	x2, x0, x16, lsr #3
	add	x3, x0, x17, lsr #3
	add	x4, x0, x12, lsr #3
	ldrh	w5, [x1]
	ldrh	w6, [x2]
	ldrh	w7, [x3]
	ldrh	w19, [x4]
	ldrb	w1, [x1, #2]
	ldrb	w2, [x2, #2]
	ldrb	w3, [x3, #2]
	orr	x1, x5, x1, lsl #16
	orr	x2, x6, x2, lsl #16
	orr	x3, x7, x3, lsl #16
	ldrb	w4, [x4, #2]
	orr	x4, x19, x4, lsl #16
	and	x15, x15, #0x4
	and	x16, x16, #0x7
	and	x17, x17, #0x6
	and	x5, x12, x14
	lsr	x15, x1, x15
	lsr	x16, x2, x16
	lsr	x17, x3, x17
	lsr	x1, x4, x5
	and	x15, x15, #0x7ff
	and	x16, x16, #0x7ff
	and	x17, x17, #0x7ff
	and	x1, x1, #0x7ff
	add	x8, x15, x8
	add	x9, x16, x9
	add	x10, x17, x10
	add	x11, x1, x11
	add	x12, x12, #44
	subs	x13, x13, #4
	b.ne	LBB1_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ldp	x20, x19, [sp], #16
	ret

	.globl	_w11_hand
	.p2align	2
_w11_hand:
	stp	x20, x19, [sp, #-16]!
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	mov	w12, #33
	mov	w13, #1000
	mov	w14, #5
LBB2_1:
	sub	x15, x12, #33
	sub	x16, x12, #22
	sub	x17, x12, #11
	and	x1, x15, #0x4
	and	x2, x16, #0x7
	and	x3, x17, #0x6
	add	x15, x0, x15, lsr #3
	add	x16, x0, x16, lsr #3
	add	x17, x0, x17, lsr #3
	add	x4, x0, x12, lsr #3
	ldrh	w5, [x15]
	ldrh	w6, [x16]
	ldrh	w7, [x17]
	ldrh	w19, [x4]
	ldrb	w15, [x15, #2]
	ldrb	w16, [x16, #2]
	ldrb	w17, [x17, #2]
	orr	x15, x5, x15, lsl #16
	ldrb	w4, [x4, #2]
	orr	x16, x6, x16, lsl #16
	orr	x17, x7, x17, lsl #16
	orr	x4, x19, x4, lsl #16
	and	x5, x12, x14
	lsr	x15, x15, x1
	lsr	x16, x16, x2
	lsr	x17, x17, x3
	lsr	x1, x4, x5
	and	x15, x15, #0x7ff
	and	x16, x16, #0x7ff
	and	x17, x17, #0x7ff
	and	x1, x1, #0x7ff
	add	x8, x15, x8
	add	x9, x16, x9
	add	x10, x17, x10
	add	x11, x1, x11
	add	x12, x12, #44
	subs	x13, x13, #4
	b.ne	LBB2_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ldp	x20, x19, [sp], #16
	ret

	.globl	_w11_native
	.p2align	2
_w11_native:
	add	x8, x0, #32
	movi.2d	v0, #0000000000000000
	mov	w9, #992
	movi.2d	v1, #0000000000000000
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
LBB3_1:
	ldp	q4, q5, [x8, #-32]
	ldp	q6, q7, [x8], #64
	bic.8h	v4, #248, lsl #8
	bic.8h	v5, #248, lsl #8
	bic.8h	v6, #248, lsl #8
	bic.8h	v7, #248, lsl #8
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
	b.ne	LBB3_1
	add.2d	v0, v1, v0
	add.2d	v1, v3, v2
	add.2d	v0, v1, v0
	addp.2d	d0, v0
	movi.2d	v1, #0000000000000000
	mov.d	v1[0], v0[0]
	ldr	q0, [x0, #1984]
	bic.8h	v0, #248, lsl #8
	ushll2.4s	v2, v0, #0
	ushll.4s	v0, v0, #0
	uaddw.2d	v1, v1, v0
	uaddw2.2d	v0, v1, v0
	uaddw.2d	v0, v0, v2
	uaddw2.2d	v0, v0, v2
	addp.2d	d0, v0
	fmov	x0, d0
	ret

	.globl	_w11_typed
	.p2align	2
_w11_typed:
	stp	x20, x19, [sp, #-16]!
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	mov	w12, #33
	mov	w13, #1000
	mov	w14, #5
LBB4_1:
	sub	x15, x12, #33
	sub	x16, x12, #22
	sub	x17, x12, #11
	add	x1, x0, x15, lsr #3
	add	x2, x0, x16, lsr #3
	add	x3, x0, x17, lsr #3
	add	x4, x0, x12, lsr #3
	ldrh	w5, [x1]
	ldrh	w6, [x2]
	ldrh	w7, [x3]
	ldrh	w19, [x4]
	ldrb	w1, [x1, #2]
	ldrb	w2, [x2, #2]
	ldrb	w3, [x3, #2]
	orr	x1, x5, x1, lsl #16
	orr	x2, x6, x2, lsl #16
	orr	x3, x7, x3, lsl #16
	ldrb	w4, [x4, #2]
	orr	x4, x19, x4, lsl #16
	and	x15, x15, #0x4
	and	x16, x16, #0x7
	and	x17, x17, #0x6
	and	x5, x12, x14
	lsr	x15, x1, x15
	lsr	x16, x2, x16
	lsr	x17, x3, x17
	lsr	x1, x4, x5
	and	x15, x15, #0x7ff
	and	x16, x16, #0x7ff
	and	x17, x17, #0x7ff
	and	x1, x1, #0x7ff
	add	x8, x15, x8
	add	x9, x16, x9
	add	x10, x17, x10
	add	x11, x1, x11
	add	x12, x12, #44
	subs	x13, x13, #4
	b.ne	LBB4_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ldp	x20, x19, [sp], #16
	ret

	.globl	_w11_wide
	.p2align	2
_w11_wide:
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	mov	w12, #33
	mov	w13, #1000
	mov	w14, #5
LBB5_1:
	sub	x15, x12, #33
	sub	x16, x12, #22
	sub	x17, x12, #11
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
	and	x15, x15, #0x7ff
	and	x16, x16, #0x7ff
	and	x17, x17, #0x7ff
	and	x1, x1, #0x7ff
	add	x8, x15, x8
	add	x9, x16, x9
	add	x10, x17, x10
	add	x11, x1, x11
	add	x12, x12, #44
	subs	x13, x13, #4
	b.ne	LBB5_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret

.subsections_via_symbols
