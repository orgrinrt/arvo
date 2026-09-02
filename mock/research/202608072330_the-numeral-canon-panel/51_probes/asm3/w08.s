	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind
	.p2align	2
__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind:
LBB0_1:
	b	LBB0_1

	.globl	_w8_gather
	.p2align	2
_w8_gather:
	add	x8, x0, #32
	movi.2d	v0, #0000000000000000
	mov	w9, #960
	movi.16b	v1, #1
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
	movi.2d	v4, #0000000000000000
LBB1_1:
	ldp	q5, q6, [x8, #-32]
	ldp	q7, q16, [x8], #64
	movi.2d	v17, #0000000000000000
	udot.4s	v17, v5, v1
	uaddw.2d	v0, v0, v17
	uaddw2.2d	v0, v0, v17
	movi.2d	v5, #0000000000000000
	udot.4s	v5, v6, v1
	uaddw.2d	v2, v2, v5
	uaddw2.2d	v2, v2, v5
	movi.2d	v5, #0000000000000000
	udot.4s	v5, v7, v1
	uaddw.2d	v3, v3, v5
	uaddw2.2d	v3, v3, v5
	movi.2d	v5, #0000000000000000
	udot.4s	v5, v16, v1
	uaddw.2d	v4, v4, v5
	uaddw2.2d	v4, v4, v5
	subs	x9, x9, #64
	b.ne	LBB1_1
	add.2d	v0, v2, v0
	add.2d	v1, v4, v3
	add.2d	v0, v1, v0
	addp.2d	d0, v0
	movi.2d	v1, #0000000000000000
	mov.d	v1[0], v0[0]
	movi.16b	v0, #1
	movi.2d	v2, #0000000000000000
	ldp	q3, q4, [x0, #960]
	udot.4s	v2, v3, v0
	movi.2d	v3, #0000000000000000
	uaddw.2d	v1, v1, v2
	uaddw2.2d	v1, v1, v2
	udot.4s	v3, v4, v0
	uaddw.2d	v0, v1, v3
	uaddw2.2d	v0, v0, v3
	addp.2d	d0, v0
	fmov	x8, d0
	ldrb	w9, [x0, #992]
	ldrb	w10, [x0, #993]
	ldrb	w11, [x0, #994]
	ldrb	w12, [x0, #995]
	add	x10, x10, x11
	add	x10, x10, x12
	ldrb	w11, [x0, #996]
	ldrb	w12, [x0, #997]
	add	x10, x10, x11
	add	x10, x10, x12
	ldrb	w11, [x0, #998]
	ldrb	w12, [x0, #999]
	add	x10, x10, x11
	add	x10, x10, x12
	add	x8, x8, x9
	add	x0, x8, x10
	ret

	.globl	_w8_native
	.p2align	2
_w8_native:
	add	x8, x0, #32
	movi.2d	v0, #0000000000000000
	mov	w9, #960
	movi.16b	v1, #1
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
	movi.2d	v4, #0000000000000000
LBB2_1:
	ldp	q5, q6, [x8, #-32]
	ldp	q7, q16, [x8], #64
	movi.2d	v17, #0000000000000000
	udot.4s	v17, v5, v1
	uaddw.2d	v0, v0, v17
	uaddw2.2d	v0, v0, v17
	movi.2d	v5, #0000000000000000
	udot.4s	v5, v6, v1
	uaddw.2d	v2, v2, v5
	uaddw2.2d	v2, v2, v5
	movi.2d	v5, #0000000000000000
	udot.4s	v5, v7, v1
	uaddw.2d	v3, v3, v5
	uaddw2.2d	v3, v3, v5
	movi.2d	v5, #0000000000000000
	udot.4s	v5, v16, v1
	uaddw.2d	v4, v4, v5
	uaddw2.2d	v4, v4, v5
	subs	x9, x9, #64
	b.ne	LBB2_1
	add.2d	v0, v2, v0
	add.2d	v1, v4, v3
	add.2d	v0, v1, v0
	addp.2d	d0, v0
	movi.2d	v1, #0000000000000000
	mov.d	v1[0], v0[0]
	movi.16b	v0, #1
	movi.2d	v2, #0000000000000000
	ldp	q3, q4, [x0, #960]
	udot.4s	v2, v3, v0
	movi.2d	v3, #0000000000000000
	uaddw.2d	v1, v1, v2
	uaddw2.2d	v1, v1, v2
	udot.4s	v3, v4, v0
	uaddw.2d	v0, v1, v3
	uaddw2.2d	v0, v0, v3
	addp.2d	d0, v0
	fmov	x8, d0
	ldrb	w9, [x0, #992]
	ldrb	w10, [x0, #993]
	ldrb	w11, [x0, #994]
	ldrb	w12, [x0, #995]
	add	x10, x10, x11
	add	x10, x10, x12
	ldrb	w11, [x0, #996]
	ldrb	w12, [x0, #997]
	add	x10, x10, x11
	add	x10, x10, x12
	ldrb	w11, [x0, #998]
	ldrb	w12, [x0, #999]
	add	x10, x10, x11
	add	x10, x10, x12
	add	x8, x8, x9
	add	x0, x8, x10
	ret

	.globl	_w8_wide
	.p2align	2
_w8_wide:
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	add	x12, x0, #3
	mov	w13, #1000
LBB3_1:
	ldurb	w14, [x12, #-3]
	ldurb	w15, [x12, #-2]
	ldurb	w16, [x12, #-1]
	add	x8, x14, x8
	add	x9, x15, x9
	ldrb	w14, [x12], #4
	add	x10, x16, x10
	add	x11, x14, x11
	subs	x13, x13, #4
	b.ne	LBB3_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret

	.globl	_w8_hand
_w8_hand = _w8_gather
	.globl	_w8_typed
_w8_typed = _w8_gather
.subsections_via_symbols
