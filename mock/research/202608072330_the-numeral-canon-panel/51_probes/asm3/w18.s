	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind
	.p2align	2
__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind:
LBB0_1:
	b	LBB0_1

	.section	__TEXT,__literal16,16byte_literals
	.p2align	4, 0x0
lCPI1_0:
	.long	4294967294
	.long	4294967290
	.long	0
	.long	4294967292
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_w18_gather
	.p2align	2
_w18_gather:
	add	x8, x0, #2
	movi.2d	v0, #0000000000000000
	mov	w9, #1000
Lloh0:
	adrp	x10, lCPI1_0@PAGE
Lloh1:
	ldr	q1, [x10, lCPI1_0@PAGEOFF]
	movi.4s	v2, #3, msl #16
	movi.2d	v3, #0000000000000000
LBB1_1:
	ldur	d4, [x8, #-2]
	ldr	d5, [x8], #9
	mov.d	v5[1], v4[0]
	ushl.4s	v4, v5, v1
	and.16b	v4, v4, v2
	uaddw2.2d	v0, v0, v4
	uaddw.2d	v3, v3, v4
	subs	x9, x9, #4
	b.ne	LBB1_1
	add.2d	v0, v3, v0
	addp.2d	d0, v0
	fmov	x0, d0
	ret
	.loh AdrpLdr	Lloh0, Lloh1

	.section	__TEXT,__literal16,16byte_literals
	.p2align	4, 0x0
lCPI2_0:
	.long	4294967294
	.long	4294967290
	.long	0
	.long	4294967292
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_w18_hand
	.p2align	2
_w18_hand:
	add	x8, x0, #2
	movi.2d	v0, #0000000000000000
	mov	w9, #1000
Lloh2:
	adrp	x10, lCPI2_0@PAGE
Lloh3:
	ldr	q1, [x10, lCPI2_0@PAGEOFF]
	movi.4s	v2, #3, msl #16
	movi.2d	v3, #0000000000000000
LBB2_1:
	ldur	d4, [x8, #-2]
	ldr	d5, [x8], #9
	mov.d	v5[1], v4[0]
	ushl.4s	v4, v5, v1
	and.16b	v4, v4, v2
	uaddw2.2d	v0, v0, v4
	uaddw.2d	v3, v3, v4
	subs	x9, x9, #4
	b.ne	LBB2_1
	add.2d	v0, v3, v0
	addp.2d	d0, v0
	fmov	x0, d0
	ret
	.loh AdrpLdr	Lloh2, Lloh3

	.globl	_w18_native
	.p2align	2
_w18_native:
	add	x8, x0, #32
	movi.2d	v0, #0000000000000000
	mov	w9, #992
	movi.4s	v1, #3, msl #16
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
	movi.4s	v2, #3, msl #16
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

	.globl	_w18_typed
	.p2align	2
_w18_typed:
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	add	x12, x0, #4
	mov	w13, #1000
LBB4_1:
	ldurb	w14, [x12, #-2]
	ldrb	w15, [x12]
	ldrb	w16, [x12, #2]
	ldurb	w17, [x12, #-1]
	ldrb	w0, [x12, #1]
	ldrb	w1, [x12, #3]
	orr	w17, w14, w17, lsl #8
	orr	w0, w15, w0, lsl #8
	orr	x1, x16, x1, lsl #8
	ldrb	w2, [x12, #4]
	orr	w15, w17, w15, lsl #16
	orr	w16, w0, w16, lsl #16
	orr	x17, x1, x2, lsl #16
	ldurh	w0, [x12, #-4]
	bfi	x0, x14, #16, #2
	ubfx	x14, x15, #2, #18
	ubfx	x15, x16, #4, #18
	add	x8, x0, x8
	add	x9, x14, x9
	add	x10, x15, x10
	add	x11, x11, x17, lsr #6
	add	x12, x12, #9
	subs	x13, x13, #4
	b.ne	LBB4_1
	add	x8, x9, x8
	add	x8, x10, x8
	add	x0, x11, x8
	ret

	.globl	_w18_wide
	.p2align	2
_w18_wide:
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	add	x12, x0, #4
	mov	w13, #1000
LBB5_1:
	ldur	x14, [x12, #-4]
	ldur	x15, [x12, #-2]
	ldr	x16, [x12]
	and	x14, x14, #0x3ffff
	ubfx	x15, x15, #2, #18
	ldur	x17, [x12, #2]
	ubfx	x16, x16, #4, #18
	ubfx	x17, x17, #6, #18
	add	x8, x14, x8
	add	x9, x15, x9
	add	x10, x16, x10
	add	x11, x17, x11
	add	x12, x12, #9
	subs	x13, x13, #4
	b.ne	LBB5_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret

.subsections_via_symbols
