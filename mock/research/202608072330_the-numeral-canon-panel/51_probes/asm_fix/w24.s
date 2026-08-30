	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind
	.p2align	2
__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind:
LBB0_1:
	b	LBB0_1

	.globl	_w24_flat8
	.p2align	2
_w24_flat8:
	stp	d11, d10, [sp, #-32]!
	stp	d9, d8, [sp, #16]
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
	add	x8, x8, #48
	ld3.16b	{ v16, v17, v18 }, [x9]
	ushll2.8h	v19, v17, #0
	ushll2.4s	v20, v19, #0
	ushll.4s	v19, v19, #0
	ushll.8h	v21, v17, #0
	ushll2.4s	v22, v21, #0
	ushll.4s	v21, v21, #0
	ushll.2d	v23, v21, #8
	ushll2.2d	v21, v21, #8
	ushll.2d	v24, v22, #8
	ushll.2d	v25, v19, #8
	ushll2.2d	v22, v22, #8
	ushll2.2d	v19, v19, #8
	ushll.2d	v26, v20, #8
	ushll2.2d	v20, v20, #8
	ushll.8h	v27, v16, #0
	ushll.4s	v28, v27, #0
	ushll.2d	v29, v28, #0
	ushll2.2d	v28, v28, #0
	ushll2.4s	v27, v27, #0
	ushll.2d	v30, v27, #0
	ushll2.8h	v31, v16, #0
	ushll.4s	v8, v31, #0
	ushll.2d	v9, v8, #0
	ushll2.2d	v27, v27, #0
	ushll2.2d	v8, v8, #0
	ushll2.4s	v31, v31, #0
	ushll.2d	v10, v31, #0
	ushll2.2d	v31, v31, #0
	orr.16b	v20, v20, v31
	orr.16b	v26, v26, v10
	orr.16b	v19, v19, v8
	orr.16b	v22, v22, v27
	orr.16b	v25, v25, v9
	orr.16b	v24, v24, v30
	orr.16b	v21, v21, v28
	orr.16b	v23, v23, v29
	ushll.8h	v27, v18, #0
	ushll.4s	v28, v27, #0
	ushll2.4s	v27, v27, #0
	ushll2.8h	v16, v18, #0
	ushll.4s	v17, v16, #0
	ushll2.4s	v16, v16, #0
	ushll2.2d	v18, v16, #16
	ushll.2d	v16, v16, #16
	ushll2.2d	v29, v17, #16
	ushll2.2d	v30, v27, #16
	ushll.2d	v17, v17, #16
	ushll.2d	v27, v27, #16
	ushll2.2d	v31, v28, #16
	ushll.2d	v28, v28, #16
	orr.16b	v23, v23, v28
	orr.16b	v21, v21, v31
	orr.16b	v24, v24, v27
	orr.16b	v17, v25, v17
	orr.16b	v22, v22, v30
	orr.16b	v19, v19, v29
	orr.16b	v16, v26, v16
	orr.16b	v18, v20, v18
	add.2d	v5, v18, v5
	add.2d	v7, v16, v7
	add.2d	v4, v19, v4
	add.2d	v1, v22, v1
	add.2d	v6, v17, v6
	add.2d	v3, v24, v3
	add.2d	v0, v21, v0
	add.2d	v2, v23, v2
	cmp	x8, #2976
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
	add	x8, x0, #2976
	ld3.8b	{ v2, v3, v4 }, [x8]
	ushll.8h	v0, v3, #0
	ushll.4s	v5, v0, #0
	ushll2.4s	v0, v0, #0
	ushll2.2d	v6, v0, #8
	ushll2.2d	v7, v5, #8
	ushll.2d	v0, v0, #8
	ushll.2d	v5, v5, #8
	ushll.8h	v16, v2, #0
	ushll2.4s	v17, v16, #0
	ushll2.2d	v18, v17, #0
	ushll.4s	v16, v16, #0
	ushll2.2d	v19, v16, #0
	ushll.2d	v17, v17, #0
	ushll.2d	v16, v16, #0
	orr.16b	v5, v5, v16
	orr.16b	v0, v0, v17
	orr.16b	v7, v7, v19
	orr.16b	v6, v6, v18
	ushll.8h	v2, v4, #0
	ushll2.4s	v3, v2, #0
	ushll.4s	v2, v2, #0
	ushll.2d	v4, v2, #16
	ushll.2d	v16, v3, #16
	ushll2.2d	v2, v2, #16
	ushll2.2d	v3, v3, #16
	orr.16b	v3, v6, v3
	orr.16b	v2, v7, v2
	orr.16b	v0, v0, v16
	orr.16b	v4, v5, v4
	add.2d	v0, v4, v0
	add.2d	v2, v2, v3
	add.2d	v0, v0, v2
	add.2d	v0, v0, v1
	addp.2d	d0, v0
	fmov	x0, d0
	ldp	d9, d8, [sp, #16]
	ldp	d11, d10, [sp], #32
	ret

	.globl	_w24_loop8
	.p2align	2
_w24_loop8:
	stp	d11, d10, [sp, #-32]!
	stp	d9, d8, [sp, #16]
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
	add	x8, x8, #48
	ld3.16b	{ v16, v17, v18 }, [x9]
	ushll.8h	v19, v16, #0
	ushll.4s	v20, v19, #0
	ushll.2d	v21, v20, #0
	ushll2.2d	v20, v20, #0
	ushll2.4s	v19, v19, #0
	ushll.2d	v22, v19, #0
	ushll2.8h	v23, v16, #0
	ushll.4s	v24, v23, #0
	ushll.2d	v25, v24, #0
	ushll2.2d	v19, v19, #0
	ushll2.2d	v24, v24, #0
	ushll2.4s	v23, v23, #0
	ushll.2d	v26, v23, #0
	ushll2.2d	v23, v23, #0
	ushll2.8h	v27, v17, #0
	ushll2.4s	v28, v27, #0
	ushll.4s	v27, v27, #0
	ushll.8h	v29, v17, #0
	ushll2.4s	v30, v29, #0
	ushll.4s	v29, v29, #0
	ushll.2d	v31, v29, #8
	ushll2.2d	v29, v29, #8
	ushll.2d	v8, v30, #8
	ushll.2d	v9, v27, #8
	ushll2.2d	v30, v30, #8
	ushll2.2d	v27, v27, #8
	ushll.2d	v10, v28, #8
	ushll2.2d	v28, v28, #8
	orr.16b	v23, v28, v23
	orr.16b	v26, v10, v26
	orr.16b	v24, v27, v24
	orr.16b	v19, v30, v19
	orr.16b	v25, v9, v25
	orr.16b	v22, v8, v22
	orr.16b	v20, v29, v20
	orr.16b	v21, v31, v21
	ushll.8h	v27, v18, #0
	ushll.4s	v28, v27, #0
	ushll2.4s	v27, v27, #0
	ushll2.8h	v16, v18, #0
	ushll.4s	v17, v16, #0
	ushll2.4s	v16, v16, #0
	ushll2.2d	v18, v16, #16
	ushll.2d	v16, v16, #16
	ushll2.2d	v29, v17, #16
	ushll2.2d	v30, v27, #16
	ushll.2d	v17, v17, #16
	ushll.2d	v27, v27, #16
	ushll2.2d	v31, v28, #16
	ushll.2d	v28, v28, #16
	orr.16b	v21, v28, v21
	orr.16b	v20, v31, v20
	orr.16b	v22, v27, v22
	orr.16b	v17, v17, v25
	orr.16b	v19, v30, v19
	orr.16b	v24, v29, v24
	orr.16b	v16, v16, v26
	orr.16b	v18, v18, v23
	add.2d	v5, v18, v5
	add.2d	v7, v16, v7
	add.2d	v4, v24, v4
	add.2d	v1, v19, v1
	add.2d	v6, v17, v6
	add.2d	v3, v22, v3
	add.2d	v0, v20, v0
	add.2d	v2, v21, v2
	cmp	x8, #2976
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
	add	x8, x0, #2976
	ld3.8b	{ v2, v3, v4 }, [x8]
	ushll.8h	v0, v2, #0
	ushll2.4s	v5, v0, #0
	ushll2.2d	v6, v5, #0
	ushll.4s	v0, v0, #0
	ushll2.2d	v7, v0, #0
	ushll.2d	v5, v5, #0
	ushll.2d	v0, v0, #0
	ushll.8h	v16, v3, #0
	ushll.4s	v17, v16, #0
	ushll2.4s	v16, v16, #0
	ushll2.2d	v18, v16, #8
	ushll2.2d	v19, v17, #8
	ushll.2d	v16, v16, #8
	ushll.2d	v17, v17, #8
	orr.16b	v0, v17, v0
	orr.16b	v5, v16, v5
	orr.16b	v7, v19, v7
	orr.16b	v6, v18, v6
	ushll.8h	v2, v4, #0
	ushll2.4s	v3, v2, #0
	ushll.4s	v2, v2, #0
	ushll.2d	v4, v2, #16
	ushll.2d	v16, v3, #16
	ushll2.2d	v2, v2, #16
	ushll2.2d	v3, v3, #16
	orr.16b	v3, v3, v6
	orr.16b	v2, v2, v7
	orr.16b	v5, v16, v5
	orr.16b	v0, v4, v0
	add.2d	v0, v0, v5
	add.2d	v2, v2, v3
	add.2d	v0, v0, v2
	add.2d	v0, v0, v1
	addp.2d	d0, v0
	fmov	x0, d0
	ldp	d9, d8, [sp, #16]
	ldp	d11, d10, [sp], #32
	ret

	.globl	_w24_wide
	.p2align	2
_w24_wide:
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	add	x12, x0, #6
	mov	w13, #1000
LBB3_1:
	ldur	x14, [x12, #-6]
	ldur	x15, [x12, #-3]
	ldr	x16, [x12]
	and	x14, x14, #0xffffff
	and	x15, x15, #0xffffff
	ldur	x17, [x12, #3]
	and	x16, x16, #0xffffff
	and	x17, x17, #0xffffff
	add	x8, x14, x8
	add	x9, x15, x9
	add	x10, x16, x10
	add	x11, x17, x11
	add	x12, x12, #12
	subs	x13, x13, #4
	b.ne	LBB3_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret

.subsections_via_symbols
