	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind
	.p2align	2
__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind:
LBB0_1:
	b	LBB0_1

	.section	__TEXT,__literal8,8byte_literals
	.p2align	3, 0x0
lCPI1_0:
	.short	1
	.short	2
	.short	4
	.short	8
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_lane_path
	.p2align	2
_lane_path:
	cbz	x1, LBB1_3
	lsl	x9, x1, #2
	sub	x8, x9, #4
	cmp	x8, #12
	b.hs	LBB1_4
	mov	x8, #-1
	mov	x13, x0
	b	LBB1_13
LBB1_3:
	mov	x8, #-1
	mov	x0, x8
	ret
LBB1_4:
	lsr	x10, x8, #2
	add	x10, x10, #1
	dup.4s	v0, w2
	adrp	x11, lCPI1_0@PAGE
	cmp	x8, #60
	b.hs	LBB1_6
	mov	x12, #0
	mov	x8, #-1
	b	LBB1_10
LBB1_6:
	and	x13, x10, #0xc
	and	x12, x10, #0x7ffffffffffffff0
	movi.2d	v1, #0000000000000000
	add	x8, x0, #32
	and	x14, x10, #0x7ffffffffffffff0
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
	movi.2d	v4, #0000000000000000
LBB1_7:
	ldp	q5, q6, [x8, #-32]
	ldp	q7, q16, [x8], #64
	cmhi.4s	v5, v5, v0
	xtn.4h	v5, v5
	cmhi.4s	v6, v6, v0
	xtn.4h	v6, v6
	cmhi.4s	v7, v7, v0
	xtn.4h	v7, v7
	cmhi.4s	v16, v16, v0
	xtn.4h	v16, v16
	orr.8b	v1, v1, v5
	orr.8b	v2, v2, v6
	orr.8b	v3, v3, v7
	orr.8b	v4, v4, v16
	subs	x14, x14, #16
	b.ne	LBB1_7
	orr.8b	v1, v2, v1
	orr.8b	v1, v3, v1
	orr.8b	v1, v4, v1
	shl.4h	v1, v1, #15
	cmlt.4h	v1, v1, #0
	ldr	d2, [x11, lCPI1_0@PAGEOFF]
	and.8b	v1, v1, v2
	addv.4h	h1, v1
	fmov	w8, s1
	tst	w8, #0xf
	csetm	x8, eq
	cmp	x10, x12
	b.eq	LBB1_15
	cbz	x13, LBB1_16
LBB1_10:
	cmn	x8, #1
	cset	w8, ne
	and	x14, x10, #0x7ffffffffffffffc
	add	x13, x0, x14, lsl #2
	dup.4h	v1, w8
	add	x8, x0, x12, lsl #2
	sub	x12, x12, x14
LBB1_11:
	ldr	q2, [x8], #16
	cmhi.4s	v2, v2, v0
	xtn.4h	v2, v2
	orr.8b	v1, v1, v2
	adds	x12, x12, #4
	b.ne	LBB1_11
	ldr	d0, [x11, lCPI1_0@PAGEOFF]
	shl.4h	v1, v1, #15
	cmlt.4h	v1, v1, #0
	and.8b	v0, v1, v0
	addv.4h	h0, v0
	fmov	w8, s0
	tst	w8, #0xf
	csetm	x8, eq
	cmp	x10, x14
	b.eq	LBB1_15
LBB1_13:
	add	x9, x0, x9
LBB1_14:
	ldr	w10, [x13], #4
	cmp	w2, w10
	csel	x8, xzr, x8, lo
	cmp	x13, x9
	b.ne	LBB1_14
LBB1_15:
	mov	x0, x8
	ret
LBB1_16:
	add	x13, x0, x12, lsl #2
	b	LBB1_13

	.globl	_scalar_path
	.p2align	2
_scalar_path:
	cbz	x1, LBB2_3
	lsl	x8, x1, #2
	sub	x10, x8, #4
	cmp	x10, #12
	b.hs	LBB2_4
	mov	w12, #1
	mov	x11, x0
	b	LBB2_13
LBB2_3:
	mov	w12, #1
	and	w0, w12, #0x1
	ret
LBB2_4:
	lsr	x9, x10, #2
	add	x9, x9, #1
	dup.4s	v0, w2
	cmp	x10, #252
	b.hs	LBB2_6
	mov	x10, #0
	mov	w12, #1
	b	LBB2_10
LBB2_6:
	and	x11, x9, #0x3c
	and	x10, x9, #0x7fffffffffffffc0
	add	x12, x0, #128
	movi.16b	v1, #1
	and	x13, x9, #0x7fffffffffffffc0
	movi.16b	v2, #1
	movi.16b	v3, #1
	movi.16b	v4, #1
LBB2_7:
	ldp	q6, q5, [x12, #-96]
	ldp	q7, q16, [x12, #-128]
	ldp	q18, q17, [x12, #-32]
	ldp	q19, q20, [x12, #-64]
	ldp	q22, q21, [x12, #32]
	ldp	q23, q24, [x12]
	ldp	q26, q25, [x12, #96]
	cmhs.4s	v16, v0, v16
	cmhs.4s	v7, v0, v7
	uzp1.8h	v7, v7, v16
	cmhs.4s	v6, v0, v6
	cmhs.4s	v5, v0, v5
	ldp	q16, q27, [x12, #64]
	uzp1.8h	v5, v6, v5
	uzp1.16b	v5, v7, v5
	cmhs.4s	v6, v0, v20
	cmhs.4s	v7, v0, v19
	uzp1.8h	v6, v7, v6
	cmhs.4s	v7, v0, v18
	cmhs.4s	v17, v0, v17
	uzp1.8h	v7, v7, v17
	uzp1.16b	v6, v6, v7
	cmhs.4s	v7, v0, v24
	cmhs.4s	v17, v0, v23
	uzp1.8h	v7, v17, v7
	cmhs.4s	v17, v0, v22
	cmhs.4s	v18, v0, v21
	uzp1.8h	v17, v17, v18
	uzp1.16b	v7, v7, v17
	cmhs.4s	v17, v0, v27
	cmhs.4s	v16, v0, v16
	uzp1.8h	v16, v16, v17
	cmhs.4s	v17, v0, v26
	cmhs.4s	v18, v0, v25
	uzp1.8h	v17, v17, v18
	uzp1.16b	v16, v16, v17
	and.16b	v1, v1, v5
	and.16b	v2, v2, v6
	and.16b	v3, v3, v7
	and.16b	v4, v4, v16
	add	x12, x12, #256
	subs	x13, x13, #64
	b.ne	LBB2_7
	and.16b	v1, v2, v1
	and.16b	v1, v3, v1
	and.16b	v1, v4, v1
	shl.16b	v1, v1, #7
	cmlt.16b	v1, v1, #0
	uminv.16b	b1, v1
	fmov	w12, s1
	cmp	x9, x10
	b.eq	LBB2_15
	cbz	x11, LBB2_16
LBB2_10:
	and	x13, x9, #0x7ffffffffffffffc
	add	x11, x0, x13, lsl #2
	movi.2d	v1, #0xffffffffffffffff
	mov.h	v1[0], w12
	add	x12, x0, x10, lsl #2
	sub	x10, x10, x13
LBB2_11:
	ldr	q2, [x12], #16
	cmhs.4s	v2, v0, v2
	xtn.4h	v2, v2
	and.8b	v1, v1, v2
	adds	x10, x10, #4
	b.ne	LBB2_11
	shl.4h	v0, v1, #15
	cmlt.4h	v0, v0, #0
	uminv.4h	h0, v0
	fmov	w12, s0
	cmp	x9, x13
	b.eq	LBB2_15
LBB2_13:
	add	x8, x0, x8
LBB2_14:
	ldr	w9, [x11], #4
	cmp	w2, w9
	cset	w9, hs
	and	w12, w12, w9
	cmp	x11, x8
	b.ne	LBB2_14
LBB2_15:
	and	w0, w12, #0x1
	ret
LBB2_16:
	add	x11, x0, x10, lsl #2
	b	LBB2_13

.subsections_via_symbols
