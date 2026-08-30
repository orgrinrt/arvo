	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind
	.p2align	2
__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind:
LBB0_1:
	b	LBB0_1

	.globl	_w21_gather
	.p2align	2
_w21_gather:
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	mov	w12, #63
	mov	w13, #1000
	mov	w14, #5
LBB1_1:
	sub	x15, x12, #63
	sub	x16, x12, #42
	sub	x17, x12, #21
	lsr	x1, x15, #3
	lsr	x2, x16, #3
	lsr	x3, x17, #3
	lsr	x4, x12, #3
	ldr	w1, [x0, x1]
	ldr	w2, [x0, x2]
	ldr	w3, [x0, x3]
	ldr	w4, [x0, x4]
	and	x15, x15, #0x4
	and	x16, x16, x14
	and	x17, x17, #0x6
	and	x5, x12, #0x7
	lsr	x15, x1, x15
	lsr	x16, x2, x16
	lsr	x17, x3, x17
	lsr	x1, x4, x5
	and	x15, x15, #0x1fffff
	and	x16, x16, #0x1fffff
	and	x17, x17, #0x1fffff
	and	x1, x1, #0x1fffff
	add	x8, x15, x8
	add	x9, x16, x9
	add	x10, x17, x10
	add	x11, x1, x11
	add	x12, x12, #84
	subs	x13, x13, #4
	b.ne	LBB1_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret

	.globl	_w21_hand
	.p2align	2
_w21_hand:
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	mov	w12, #63
	mov	w13, #1000
	mov	w14, #5
LBB2_1:
	sub	x15, x12, #63
	sub	x16, x12, #42
	sub	x17, x12, #21
	lsr	x1, x15, #3
	lsr	x2, x16, #3
	lsr	x3, x17, #3
	lsr	x4, x12, #3
	and	x15, x15, #0x4
	and	x16, x16, x14
	and	x17, x17, #0x6
	and	x5, x12, #0x7
	ldr	w1, [x0, x1]
	ldr	w2, [x0, x2]
	ldr	w3, [x0, x3]
	ldr	w4, [x0, x4]
	lsr	x15, x1, x15
	lsr	x16, x2, x16
	lsr	x17, x3, x17
	lsr	x1, x4, x5
	and	x15, x15, #0x1fffff
	and	x16, x16, #0x1fffff
	and	x17, x17, #0x1fffff
	and	x1, x1, #0x1fffff
	add	x8, x15, x8
	add	x9, x16, x9
	add	x10, x17, x10
	add	x11, x1, x11
	add	x12, x12, #84
	subs	x13, x13, #4
	b.ne	LBB2_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret

	.globl	_w21_native
	.p2align	2
_w21_native:
	add	x8, x0, #32
	movi.2d	v0, #0000000000000000
	mov	w9, #992
	movi.4s	v1, #31, msl #16
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
	movi.2d	v4, #0000000000000000
LBB3_1:
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
	b.ne	LBB3_1
	add.2d	v0, v2, v0
	add.2d	v1, v4, v3
	add.2d	v0, v1, v0
	addp.2d	d0, v0
	movi.2d	v1, #0000000000000000
	mov.d	v1[0], v0[0]
	ldr	q0, [x0, #3968]
	movi.4s	v2, #31, msl #16
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

	.globl	_w21_typed
	.p2align	2
_w21_typed:
	mov	x9, #0
	mov	x8, #0
	mov	w10, #1000
LBB4_1:
	lsr	x11, x9, #3
	ldr	w11, [x0, x11]
	and	x12, x9, #0x7
	lsr	x11, x11, x12
	and	x11, x11, #0x1fffff
	add	x8, x11, x8
	add	x9, x9, #21
	subs	x10, x10, #1
	b.ne	LBB4_1
	mov	x0, x8
	ret

	.globl	_w21_wide
	.p2align	2
_w21_wide:
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	mov	w12, #63
	mov	w13, #1000
	mov	w14, #5
LBB5_1:
	sub	x15, x12, #63
	sub	x16, x12, #42
	sub	x17, x12, #21
	lsr	x1, x15, #3
	lsr	x2, x16, #3
	lsr	x3, x17, #3
	lsr	x4, x12, #3
	ldr	x1, [x0, x1]
	ldr	x2, [x0, x2]
	ldr	x3, [x0, x3]
	ldr	x4, [x0, x4]
	and	x15, x15, #0x4
	and	x16, x16, x14
	and	x17, x17, #0x6
	and	x5, x12, #0x7
	lsr	x15, x1, x15
	lsr	x16, x2, x16
	lsr	x17, x3, x17
	lsr	x1, x4, x5
	and	x15, x15, #0x1fffff
	and	x16, x16, #0x1fffff
	and	x17, x17, #0x1fffff
	and	x1, x1, #0x1fffff
	add	x8, x15, x8
	add	x9, x16, x9
	add	x10, x17, x10
	add	x11, x1, x11
	add	x12, x12, #84
	subs	x13, x13, #4
	b.ne	LBB5_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret

.subsections_via_symbols
