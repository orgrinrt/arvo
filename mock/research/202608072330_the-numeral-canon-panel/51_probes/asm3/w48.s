	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind
	.p2align	2
__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind:
LBB0_1:
	b	LBB0_1

	.globl	_w48_gather
	.p2align	2
_w48_gather:
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	add	x12, x0, #11
	mov	w13, #1000
LBB1_1:
	ldur	w14, [x12, #-11]
	ldur	w15, [x12, #-5]
	ldur	w16, [x12, #1]
	ldur	w17, [x12, #7]
	ldurb	w0, [x12, #-7]
	ldurb	w1, [x12, #-1]
	ldrb	w2, [x12, #5]
	orr	x14, x14, x0, lsl #32
	ldrb	w0, [x12, #11]
	orr	x15, x15, x1, lsl #32
	orr	x16, x16, x2, lsl #32
	orr	x17, x17, x0, lsl #32
	ldurb	w0, [x12, #-6]
	ldrb	w1, [x12]
	ldrb	w2, [x12, #6]
	orr	x14, x14, x0, lsl #40
	ldrb	w0, [x12, #12]
	orr	x15, x15, x1, lsl #40
	orr	x16, x16, x2, lsl #40
	orr	x17, x17, x0, lsl #40
	add	x8, x14, x8
	add	x9, x15, x9
	add	x10, x16, x10
	add	x11, x17, x11
	add	x12, x12, #24
	subs	x13, x13, #4
	b.ne	LBB1_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret

	.globl	_w48_hand
	.p2align	2
_w48_hand:
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	add	x12, x0, #11
	mov	w13, #1000
LBB2_1:
	ldur	w14, [x12, #-11]
	ldur	w15, [x12, #-5]
	ldur	w16, [x12, #1]
	ldur	w17, [x12, #7]
	ldurb	w0, [x12, #-7]
	ldurb	w1, [x12, #-1]
	ldrb	w2, [x12, #5]
	orr	x14, x14, x0, lsl #32
	ldrb	w0, [x12, #11]
	orr	x15, x15, x1, lsl #32
	orr	x16, x16, x2, lsl #32
	orr	x17, x17, x0, lsl #32
	ldurb	w0, [x12, #-6]
	ldrb	w1, [x12]
	ldrb	w2, [x12, #6]
	orr	x14, x14, x0, lsl #40
	ldrb	w0, [x12, #12]
	orr	x15, x15, x1, lsl #40
	orr	x16, x16, x2, lsl #40
	orr	x17, x17, x0, lsl #40
	add	x8, x14, x8
	add	x9, x15, x9
	add	x10, x16, x10
	add	x11, x17, x11
	add	x12, x12, #24
	subs	x13, x13, #4
	b.ne	LBB2_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret

	.globl	_w48_native
	.p2align	2
_w48_native:
	add	x8, x0, #32
	movi.2d	v0, #0000000000000000
	mov	w9, #1000
	movi.2d	v1, #0x00ffffffffffff
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
	add.2d	v0, v5, v0
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	add.2d	v4, v16, v4
	subs	x9, x9, #8
	b.ne	LBB3_1
	add.2d	v0, v2, v0
	add.2d	v0, v3, v0
	add.2d	v0, v4, v0
	addp.2d	d0, v0
	fmov	x0, d0
	ret

	.globl	_w48_typed
	.p2align	2
_w48_typed:
	mov	x8, x0
	mov	x0, #0
	add	x8, x8, #2
	mov	w9, #1000
LBB4_1:
	ldur	w10, [x8, #-2]
	ldrb	w11, [x8, #2]
	ldrb	w12, [x8, #3]
	orr	x10, x10, x11, lsl #32
	orr	x10, x10, x12, lsl #40
	add	x0, x10, x0
	add	x8, x8, #6
	subs	x9, x9, #1
	b.ne	LBB4_1
	ret

	.globl	_w48_wide
	.p2align	2
_w48_wide:
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	add	x12, x0, #12
	mov	w13, #1000
LBB5_1:
	ldur	x14, [x12, #-12]
	ldur	x15, [x12, #-6]
	ldr	x16, [x12]
	and	x14, x14, #0xffffffffffff
	and	x15, x15, #0xffffffffffff
	ldur	x17, [x12, #6]
	and	x16, x16, #0xffffffffffff
	and	x17, x17, #0xffffffffffff
	add	x8, x14, x8
	add	x9, x15, x9
	add	x10, x16, x10
	add	x11, x17, x11
	add	x12, x12, #24
	subs	x13, x13, #4
	b.ne	LBB5_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret

.subsections_via_symbols
