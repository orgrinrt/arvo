	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind
	.p2align	2
__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind:
LBB0_1:
	b	LBB0_1

	.globl	_w16_gather
	.p2align	2
_w16_gather:
	mov	x8, #0
	movi.2d	v0, #0000000000000000
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
	movi.2d	v1, #0000000000000000
	movi.2d	v6, #0000000000000000
	movi.2d	v4, #0000000000000000
	movi.2d	v7, #0000000000000000
	movi.2d	v5, #0000000000000000
LBB1_1:
	add	x9, x0, x8
	ld2.16b	{ v16, v17 }, [x9]
	ushll2.8h	v18, v16, #0
	ushll2.4s	v19, v18, #0
	ushll2.2d	v20, v19, #0
	ushll.2d	v19, v19, #0
	ushll.4s	v18, v18, #0
	ushll2.2d	v21, v18, #0
	ushll.8h	v22, v16, #0
	ushll2.4s	v23, v22, #0
	ushll2.2d	v24, v23, #0
	ushll.2d	v18, v18, #0
	ushll.2d	v23, v23, #0
	ushll.4s	v22, v22, #0
	ushll2.2d	v25, v22, #0
	ushll.2d	v22, v22, #0
	ushll.8h	v26, v17, #0
	ushll.4s	v27, v26, #0
	ushll2.4s	v26, v26, #0
	ushll2.8h	v16, v17, #0
	ushll.4s	v17, v16, #0
	ushll2.4s	v16, v16, #0
	ushll2.2d	v28, v16, #8
	ushll.2d	v16, v16, #8
	ushll2.2d	v29, v17, #8
	ushll2.2d	v30, v26, #8
	ushll.2d	v17, v17, #8
	ushll.2d	v26, v26, #8
	ushll2.2d	v31, v27, #8
	ushll.2d	v27, v27, #8
	orr.16b	v22, v27, v22
	orr.16b	v25, v31, v25
	orr.16b	v23, v26, v23
	orr.16b	v17, v17, v18
	orr.16b	v18, v30, v24
	orr.16b	v21, v29, v21
	orr.16b	v16, v16, v19
	orr.16b	v19, v28, v20
	add.2d	v5, v19, v5
	add.2d	v7, v16, v7
	add.2d	v4, v21, v4
	add.2d	v1, v18, v1
	add.2d	v6, v17, v6
	add.2d	v3, v23, v3
	add.2d	v0, v25, v0
	add.2d	v2, v22, v2
	add	x8, x8, #32
	cmp	x8, #1984
	b.ne	LBB1_1
	add.2d	v2, v2, v6
	add.2d	v3, v3, v7
	add.2d	v2, v2, v3
	add.2d	v0, v0, v4
	add.2d	v1, v1, v5
	add.2d	v0, v0, v1
	add.2d	v0, v2, v0
	addp.2d	d0, v0
	movi.2d	v1, #0000000000000000
	mov.d	v1[0], v0[0]
	add	x8, x0, #1984
	ld2.8b	{ v2, v3 }, [x8]
	ushll.8h	v0, v2, #0
	ushll.4s	v4, v0, #0
	ushll.2d	v5, v4, #0
	ushll2.4s	v0, v0, #0
	ushll.2d	v6, v0, #0
	ushll2.2d	v4, v4, #0
	ushll2.2d	v0, v0, #0
	ushll.8h	v2, v3, #0
	ushll2.4s	v3, v2, #0
	ushll.4s	v2, v2, #0
	ushll.2d	v7, v2, #8
	ushll.2d	v16, v3, #8
	ushll2.2d	v2, v2, #8
	ushll2.2d	v3, v3, #8
	orr.16b	v0, v3, v0
	orr.16b	v2, v2, v4
	orr.16b	v3, v16, v6
	orr.16b	v4, v7, v5
	add.2d	v3, v4, v3
	add.2d	v0, v2, v0
	add.2d	v0, v3, v0
	add.2d	v0, v0, v1
	addp.2d	d0, v0
	fmov	x0, d0
	ret

	.globl	_w16_hand
	.p2align	2
_w16_hand:
	mov	x8, #0
	movi.2d	v0, #0000000000000000
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
	movi.2d	v1, #0000000000000000
	movi.2d	v6, #0000000000000000
	movi.2d	v4, #0000000000000000
	movi.2d	v7, #0000000000000000
	movi.2d	v5, #0000000000000000
LBB2_1:
	add	x9, x0, x8
	ld2.16b	{ v16, v17 }, [x9]
	ushll2.8h	v18, v16, #0
	ushll2.4s	v19, v18, #0
	ushll2.2d	v20, v19, #0
	ushll.2d	v19, v19, #0
	ushll.4s	v18, v18, #0
	ushll2.2d	v21, v18, #0
	ushll.8h	v22, v16, #0
	ushll2.4s	v23, v22, #0
	ushll2.2d	v24, v23, #0
	ushll.2d	v18, v18, #0
	ushll.2d	v23, v23, #0
	ushll.4s	v22, v22, #0
	ushll2.2d	v25, v22, #0
	ushll.2d	v22, v22, #0
	ushll.8h	v26, v17, #0
	ushll.4s	v27, v26, #0
	ushll2.4s	v26, v26, #0
	ushll2.8h	v16, v17, #0
	ushll.4s	v17, v16, #0
	ushll2.4s	v16, v16, #0
	ushll2.2d	v28, v16, #8
	ushll.2d	v16, v16, #8
	ushll2.2d	v29, v17, #8
	ushll2.2d	v30, v26, #8
	ushll.2d	v17, v17, #8
	ushll.2d	v26, v26, #8
	ushll2.2d	v31, v27, #8
	ushll.2d	v27, v27, #8
	orr.16b	v22, v27, v22
	orr.16b	v25, v31, v25
	orr.16b	v23, v26, v23
	orr.16b	v17, v17, v18
	orr.16b	v18, v30, v24
	orr.16b	v21, v29, v21
	orr.16b	v16, v16, v19
	orr.16b	v19, v28, v20
	add.2d	v5, v19, v5
	add.2d	v7, v16, v7
	add.2d	v4, v21, v4
	add.2d	v1, v18, v1
	add.2d	v6, v17, v6
	add.2d	v3, v23, v3
	add.2d	v0, v25, v0
	add.2d	v2, v22, v2
	add	x8, x8, #32
	cmp	x8, #1984
	b.ne	LBB2_1
	add.2d	v2, v2, v6
	add.2d	v3, v3, v7
	add.2d	v2, v2, v3
	add.2d	v0, v0, v4
	add.2d	v1, v1, v5
	add.2d	v0, v0, v1
	add.2d	v0, v2, v0
	addp.2d	d0, v0
	movi.2d	v1, #0000000000000000
	mov.d	v1[0], v0[0]
	add	x8, x0, #1984
	ld2.8b	{ v2, v3 }, [x8]
	ushll.8h	v0, v2, #0
	ushll.4s	v4, v0, #0
	ushll.2d	v5, v4, #0
	ushll2.4s	v0, v0, #0
	ushll.2d	v6, v0, #0
	ushll2.2d	v4, v4, #0
	ushll2.2d	v0, v0, #0
	ushll.8h	v2, v3, #0
	ushll2.4s	v3, v2, #0
	ushll.4s	v2, v2, #0
	ushll.2d	v7, v2, #8
	ushll.2d	v16, v3, #8
	ushll2.2d	v2, v2, #8
	ushll2.2d	v3, v3, #8
	orr.16b	v0, v3, v0
	orr.16b	v2, v2, v4
	orr.16b	v3, v16, v6
	orr.16b	v4, v7, v5
	add.2d	v3, v4, v3
	add.2d	v0, v2, v0
	add.2d	v0, v3, v0
	add.2d	v0, v0, v1
	addp.2d	d0, v0
	fmov	x0, d0
	ret

	.globl	_w16_native
	.p2align	2
_w16_native:
	add	x8, x0, #32
	movi.2d	v0, #0000000000000000
	mov	w9, #992
	movi.2d	v1, #0000000000000000
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
LBB3_1:
	ldp	q4, q5, [x8, #-32]
	ldp	q6, q7, [x8], #64
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
	ushll2.4s	v2, v0, #0
	ushll.4s	v0, v0, #0
	uaddw.2d	v1, v1, v0
	uaddw2.2d	v0, v1, v0
	uaddw.2d	v0, v0, v2
	uaddw2.2d	v0, v0, v2
	addp.2d	d0, v0
	fmov	x0, d0
	ret

	.globl	_w16_wide
	.p2align	2
_w16_wide:
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	add	x12, x0, #4
	mov	w13, #1000
LBB4_1:
	ldurh	w14, [x12, #-4]
	ldurh	w15, [x12, #-2]
	ldrh	w16, [x12]
	add	x8, x14, x8
	add	x9, x15, x9
	ldrh	w14, [x12, #2]
	add	x10, x16, x10
	add	x11, x14, x11
	add	x12, x12, #8
	subs	x13, x13, #4
	b.ne	LBB4_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret

	.globl	_w16_typed
_w16_typed = _w16_hand
.subsections_via_symbols
