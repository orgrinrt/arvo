	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind
	.p2align	2
__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind:
LBB0_1:
	b	LBB0_1

	.globl	_w6_gather
	.p2align	2
_w6_gather:
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	add	x12, x0, #2
	mov	w13, #1000
LBB1_1:
	ldurb	w14, [x12, #-2]
	ldurb	w15, [x12, #-1]
	ldrb	w16, [x12], #3
	orr	w17, w14, w15, lsl #8
	orr	w15, w15, w16, lsl #8
	and	x14, x14, #0x3f
	ubfx	x17, x17, #6, #6
	ubfx	x15, x15, #4, #6
	add	x8, x8, x14
	add	x9, x17, x9
	add	x10, x15, x10
	add	x11, x11, x16, lsr #2
	subs	x13, x13, #4
	b.ne	LBB1_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret

	.globl	_w6_native
	.p2align	2
_w6_native:
	add	x8, x0, #32
	movi.2d	v0, #0000000000000000
	mov	w9, #960
	movi.16b	v1, #63
	movi.16b	v2, #1
	movi.2d	v3, #0000000000000000
	movi.2d	v4, #0000000000000000
	movi.2d	v5, #0000000000000000
LBB2_1:
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
	b.ne	LBB2_1
	add.2d	v0, v3, v0
	add.2d	v1, v5, v4
	add.2d	v0, v1, v0
	addp.2d	d0, v0
	movi.2d	v1, #0000000000000000
	mov.d	v1[0], v0[0]
	movi.16b	v0, #63
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
	and	x9, x9, #0x3f
	ldrb	w10, [x0, #993]
	and	x10, x10, #0x3f
	ldrb	w11, [x0, #994]
	and	x11, x11, #0x3f
	ldrb	w12, [x0, #995]
	and	x12, x12, #0x3f
	add	x10, x10, x11
	add	x10, x10, x12
	ldrb	w11, [x0, #996]
	and	x11, x11, #0x3f
	ldrb	w12, [x0, #997]
	and	x12, x12, #0x3f
	add	x10, x10, x11
	add	x10, x10, x12
	ldrb	w11, [x0, #998]
	and	x11, x11, #0x3f
	ldrb	w12, [x0, #999]
	and	x12, x12, #0x3f
	add	x10, x10, x11
	add	x10, x10, x12
	add	x8, x8, x9
	add	x0, x8, x10
	ret

	.globl	_w6_wide
	.p2align	2
_w6_wide:
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	add	x12, x0, #1
	mov	w13, #1000
LBB3_1:
	ldur	x14, [x12, #-1]
	ldr	x15, [x12]
	and	x16, x14, #0x3f
	ubfx	x14, x14, #6, #6
	ldur	x17, [x12, #1]
	ubfx	x15, x15, #4, #6
	ubfx	x17, x17, #2, #6
	add	x8, x16, x8
	add	x9, x14, x9
	add	x10, x15, x10
	add	x11, x17, x11
	add	x12, x12, #3
	subs	x13, x13, #4
	b.ne	LBB3_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret

	.globl	_w6_hand
_w6_hand = _w6_gather
	.globl	_w6_typed
_w6_typed = _w6_gather
.subsections_via_symbols
