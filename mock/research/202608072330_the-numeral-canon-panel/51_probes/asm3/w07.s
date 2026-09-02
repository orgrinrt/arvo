	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind
	.p2align	2
__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind:
LBB0_1:
	b	LBB0_1

	.globl	_w7_gather
	.p2align	2
_w7_gather:
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	mov	w12, #21
	mov	w13, #1000
	mov	w14, #5
LBB1_1:
	sub	x15, x12, #21
	sub	x16, x12, #14
	sub	x17, x12, #7
	lsr	x1, x15, #3
	lsr	x2, x16, #3
	lsr	x3, x17, #3
	lsr	x4, x12, #3
	ldrh	w1, [x0, x1]
	ldrh	w2, [x0, x2]
	ldrh	w3, [x0, x3]
	ldrh	w4, [x0, x4]
	and	x15, x15, #0x4
	and	x16, x16, #0x7
	and	x17, x17, #0x6
	and	x5, x12, x14
	lsr	x15, x1, x15
	lsr	x16, x2, x16
	lsr	x17, x3, x17
	lsr	x1, x4, x5
	and	x15, x15, #0x7f
	and	x16, x16, #0x7f
	and	x17, x17, #0x7f
	and	x1, x1, #0x7f
	add	x8, x15, x8
	add	x9, x16, x9
	add	x10, x17, x10
	add	x11, x1, x11
	add	x12, x12, #28
	subs	x13, x13, #4
	b.ne	LBB1_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret

	.globl	_w7_hand
	.p2align	2
_w7_hand:
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	mov	w12, #21
	mov	w13, #1000
	mov	w14, #5
LBB2_1:
	sub	x15, x12, #21
	sub	x16, x12, #14
	sub	x17, x12, #7
	and	x1, x15, #0x4
	and	x2, x16, #0x7
	and	x3, x17, #0x6
	and	x4, x12, x14
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
	and	x15, x15, #0x7f
	and	x16, x16, #0x7f
	and	x17, x17, #0x7f
	and	x1, x1, #0x7f
	add	x8, x15, x8
	add	x9, x16, x9
	add	x10, x17, x10
	add	x11, x1, x11
	add	x12, x12, #28
	subs	x13, x13, #4
	b.ne	LBB2_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret

	.globl	_w7_native
	.p2align	2
_w7_native:
	add	x8, x0, #32
	movi.2d	v0, #0000000000000000
	mov	w9, #960
	movi.16b	v1, #127
	movi.16b	v2, #1
	movi.2d	v3, #0000000000000000
	movi.2d	v4, #0000000000000000
	movi.2d	v5, #0000000000000000
LBB3_1:
	ldp	q6, q7, [x8, #-32]
	ldp	q16, q17, [x8], #64
	and.16b	v6, v6, v1
	and.16b	v7, v7, v1
	and.16b	v16, v16, v1
	and.16b	v17, v17, v1
	movi.2d	v18, #0000000000000000
	udot.4s	v18, v6, v2
	uaddw.2d	v0, v0, v18
	uaddw2.2d	v0, v0, v18
	movi.2d	v6, #0000000000000000
	udot.4s	v6, v7, v2
	uaddw.2d	v3, v3, v6
	uaddw2.2d	v3, v3, v6
	movi.2d	v6, #0000000000000000
	udot.4s	v6, v16, v2
	uaddw.2d	v4, v4, v6
	uaddw2.2d	v4, v4, v6
	movi.2d	v6, #0000000000000000
	udot.4s	v6, v17, v2
	uaddw.2d	v5, v5, v6
	uaddw2.2d	v5, v5, v6
	subs	x9, x9, #64
	b.ne	LBB3_1
	add.2d	v0, v3, v0
	add.2d	v1, v5, v4
	add.2d	v0, v1, v0
	addp.2d	d0, v0
	movi.2d	v1, #0000000000000000
	mov.d	v1[0], v0[0]
	movi.16b	v0, #127
	ldp	q2, q3, [x0, #960]
	and.16b	v2, v2, v0
	movi.16b	v4, #1
	movi.2d	v5, #0000000000000000
	udot.4s	v5, v2, v4
	movi.2d	v2, #0000000000000000
	uaddw.2d	v1, v1, v5
	uaddw2.2d	v1, v1, v5
	and.16b	v0, v3, v0
	udot.4s	v2, v0, v4
	uaddw.2d	v0, v1, v2
	uaddw2.2d	v0, v0, v2
	addp.2d	d0, v0
	fmov	x8, d0
	ldrb	w9, [x0, #992]
	and	x9, x9, #0x7f
	ldrb	w10, [x0, #993]
	and	x10, x10, #0x7f
	ldrb	w11, [x0, #994]
	and	x11, x11, #0x7f
	ldrb	w12, [x0, #995]
	and	x12, x12, #0x7f
	add	x10, x10, x11
	add	x10, x10, x12
	ldrb	w11, [x0, #996]
	and	x11, x11, #0x7f
	ldrb	w12, [x0, #997]
	and	x12, x12, #0x7f
	add	x10, x10, x11
	add	x10, x10, x12
	ldrb	w11, [x0, #998]
	and	x11, x11, #0x7f
	ldrb	w12, [x0, #999]
	and	x12, x12, #0x7f
	add	x10, x10, x11
	add	x10, x10, x12
	add	x8, x8, x9
	add	x0, x8, x10
	ret

	.globl	_w7_wide
	.p2align	2
_w7_wide:
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	mov	w12, #21
	mov	w13, #1000
	mov	w14, #5
LBB4_1:
	sub	x15, x12, #21
	sub	x16, x12, #14
	sub	x17, x12, #7
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
	and	x15, x15, #0x7f
	and	x16, x16, #0x7f
	and	x17, x17, #0x7f
	and	x1, x1, #0x7f
	add	x8, x15, x8
	add	x9, x16, x9
	add	x10, x17, x10
	add	x11, x1, x11
	add	x12, x12, #28
	subs	x13, x13, #4
	b.ne	LBB4_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret

	.globl	_w7_typed
_w7_typed = _w7_gather
.subsections_via_symbols
