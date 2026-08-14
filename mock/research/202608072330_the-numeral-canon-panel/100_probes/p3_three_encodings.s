	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCs7jv63BkYSe1_18p3_three_encodings4arm0
	.p2align	2
__RNvCs7jv63BkYSe1_18p3_three_encodings4arm0:
	cbz	x1, LBB0_3
	cmp	x1, #8
	b.hs	LBB0_4
	mov	x8, #0
	mov	x9, #0
	b	LBB0_7
LBB0_3:
	mov	x8, #0
	mov	x0, x8
	ret
LBB0_4:
	and	x9, x1, #0xffffffffffffff8
	add	x8, x0, #32
	movi.2d	v0, #0000000000000000
	and	x10, x1, #0xffffffffffffff8
	movi.2d	v1, #0000000000000000
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
LBB0_5:
	ldp	q4, q5, [x8, #-32]
	ldp	q6, q7, [x8], #64
	add.2d	v0, v4, v0
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	subs	x10, x10, #8
	b.ne	LBB0_5
	add.2d	v0, v1, v0
	add.2d	v0, v2, v0
	add.2d	v0, v3, v0
	addp.2d	d0, v0
	fmov	x8, d0
	cmp	x1, x9
	b.eq	LBB0_9
LBB0_7:
	sub	x10, x1, x9
	add	x9, x0, x9, lsl #3
LBB0_8:
	ldr	x11, [x9], #8
	add	x8, x11, x8
	subs	x10, x10, #1
	b.ne	LBB0_8
LBB0_9:
	mov	x0, x8
	ret

	.globl	__RNvCs7jv63BkYSe1_18p3_three_encodings4arm1
	.p2align	2
__RNvCs7jv63BkYSe1_18p3_three_encodings4arm1:
	cbz	x1, LBB1_3
	cmp	x1, #8
	b.hs	LBB1_4
	mov	x8, #0
	mov	x9, #0
	b	LBB1_7
LBB1_3:
	mov	x8, #0
	mov	x0, x8
	ret
LBB1_4:
	and	x9, x1, #0xffffffffffffff8
	add	x8, x0, #32
	movi.2d	v0, #0000000000000000
	movi.2d	v1, #0x0000000000ffff
	and	x10, x1, #0xffffffffffffff8
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
	movi.2d	v4, #0000000000000000
LBB1_5:
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
	subs	x10, x10, #8
	b.ne	LBB1_5
	add.2d	v0, v2, v0
	add.2d	v0, v3, v0
	add.2d	v0, v4, v0
	addp.2d	d0, v0
	fmov	x8, d0
	cmp	x1, x9
	b.eq	LBB1_9
LBB1_7:
	sub	x10, x1, x9
	add	x9, x0, x9, lsl #3
LBB1_8:
	ldrh	w11, [x9], #8
	add	x8, x11, x8
	subs	x10, x10, #1
	b.ne	LBB1_8
LBB1_9:
	mov	x0, x8
	ret

	.globl	__RNvCs7jv63BkYSe1_18p3_three_encodings4arm2
	.p2align	2
__RNvCs7jv63BkYSe1_18p3_three_encodings4arm2:
	cbz	x1, LBB2_3
	cmp	x1, #8
	b.hs	LBB2_4
	mov	x8, #0
	mov	x9, #0
	b	LBB2_7
LBB2_3:
	mov	x8, #0
	mov	x0, x8
	ret
LBB2_4:
	and	x9, x1, #0xffffffffffffff8
	add	x8, x0, #32
	movi.2d	v0, #0000000000000000
	and	x10, x1, #0xffffffffffffff8
	movi.2d	v1, #0000000000000000
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
LBB2_5:
	ldp	q4, q5, [x8, #-32]
	ldp	q6, q7, [x8], #64
	eor.16b	v0, v4, v0
	eor.16b	v1, v5, v1
	eor.16b	v2, v6, v2
	eor.16b	v3, v7, v3
	subs	x10, x10, #8
	b.ne	LBB2_5
	eor.16b	v0, v1, v0
	eor3.16b	v0, v2, v0, v3
	ext.16b	v1, v0, v0, #8
	eor.8b	v0, v0, v1
	fmov	x8, d0
	cmp	x1, x9
	b.eq	LBB2_9
LBB2_7:
	sub	x10, x1, x9
	add	x9, x0, x9, lsl #3
LBB2_8:
	ldr	x11, [x9], #8
	eor	x8, x11, x8
	subs	x10, x10, #1
	b.ne	LBB2_8
LBB2_9:
	mov	x0, x8
	ret

	.globl	__RNvCs7jv63BkYSe1_18p3_three_encodings4arm3
	.p2align	2
__RNvCs7jv63BkYSe1_18p3_three_encodings4arm3:
	cbz	x1, LBB3_3
	cmp	x1, #4
	b.hs	LBB3_4
	mov	x9, #0
	mov	w8, #1
	b	LBB3_7
LBB3_3:
	mov	w8, #1
	mov	x0, x8
	ret
LBB3_4:
	and	x9, x1, #0xffffffffffffffc
	add	x10, x0, #16
	mov	w8, #1
	and	x11, x1, #0xffffffffffffffc
	mov	w12, #1
	mov	w13, #1
	mov	w14, #1
LBB3_5:
	ldp	x15, x16, [x10, #-16]
	ldp	x17, x2, [x10], #32
	orr	x15, x15, #0x1
	orr	x16, x16, #0x1
	orr	x17, x17, #0x1
	orr	x2, x2, #0x1
	mul	x8, x15, x8
	mul	x12, x16, x12
	mul	x13, x17, x13
	mul	x14, x2, x14
	subs	x11, x11, #4
	b.ne	LBB3_5
	mul	x8, x12, x8
	mul	x8, x13, x8
	mul	x8, x14, x8
	cmp	x1, x9
	b.eq	LBB3_9
LBB3_7:
	sub	x10, x1, x9
	add	x9, x0, x9, lsl #3
LBB3_8:
	ldr	x11, [x9], #8
	orr	x11, x11, #0x1
	mul	x8, x11, x8
	subs	x10, x10, #1
	b.ne	LBB3_8
LBB3_9:
	mov	x0, x8
	ret

	.globl	__RNvCs7jv63BkYSe1_18p3_three_encodings4arm4
	.p2align	2
__RNvCs7jv63BkYSe1_18p3_three_encodings4arm4:
	.cfi_startproc
	cbz	x1, LBB4_3
	cmp	x1, #8
	b.hs	LBB4_4
	mov	x8, #0
	mov	x9, #0
	b	LBB4_7
LBB4_3:
	mov	x8, #0
	mov	x0, x8
	ret
LBB4_4:
	and	x9, x1, #0xffffffffffffff8
	add	x8, x0, #32
	movi.2d	v0, #0000000000000000
	and	x10, x1, #0xffffffffffffff8
	movi.2d	v1, #0000000000000000
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
	movi.2d	v4, #0000000000000000
LBB4_5:
	ldp	q5, q6, [x8, #-32]
	ldp	q7, q16, [x8], #64
	xar.2d	v5, v5, v0, #57
	xar.2d	v6, v6, v0, #57
	xar.2d	v7, v7, v0, #57
	xar.2d	v16, v16, v0, #57
	add.2d	v1, v5, v1
	add.2d	v2, v6, v2
	add.2d	v3, v7, v3
	add.2d	v4, v16, v4
	subs	x10, x10, #8
	b.ne	LBB4_5
	add.2d	v0, v2, v1
	add.2d	v0, v3, v0
	add.2d	v0, v4, v0
	addp.2d	d0, v0
	fmov	x8, d0
	cmp	x1, x9
	b.eq	LBB4_9
LBB4_7:
	sub	x10, x1, x9
	add	x9, x0, x9, lsl #3
LBB4_8:
	ldr	x11, [x9], #8
	ror	x11, x11, #57
	add	x8, x11, x8
	subs	x10, x10, #1
	b.ne	LBB4_8
LBB4_9:
	mov	x0, x8
	ret
	.cfi_endproc

	.globl	_e1_named
	.p2align	2
_e1_named:
	b	__RNvCs7jv63BkYSe1_18p3_three_encodings4arm0

	.globl	_e3_direct
	.p2align	2
_e3_direct:
	b	__RNvCs7jv63BkYSe1_18p3_three_encodings4arm4

	.globl	_e4_consumer
	.p2align	2
_e4_consumer:
	b	__RNvCs7jv63BkYSe1_18p3_three_encodings4arm4

	.globl	_e2_weighted
_e2_weighted = _e1_named
.subsections_via_symbols
