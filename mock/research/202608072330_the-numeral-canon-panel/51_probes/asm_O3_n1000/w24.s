	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_w24_hand
	.p2align	2
_w24_hand:
	.cfi_startproc
	stp	d11, d10, [sp, #-32]!
	.cfi_def_cfa_offset 32
	stp	d9, d8, [sp, #16]
	.cfi_offset b8, -8
	.cfi_offset b9, -16
	.cfi_offset b10, -24
	.cfi_offset b11, -32
	mov	x8, #0
	movi.2d	v0, #0000000000000000
	movi.2d	v2, #0000000000000000
	movi.2d	v3, #0000000000000000
	movi.2d	v1, #0000000000000000
	movi.2d	v6, #0000000000000000
	movi.2d	v4, #0000000000000000
	movi.2d	v7, #0000000000000000
	movi.2d	v5, #0000000000000000
LBB0_1:
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
	b.ne	LBB0_1
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
	.cfi_def_cfa_offset 0
	.cfi_restore b8
	.cfi_restore b9
	.cfi_restore b10
	.cfi_restore b11
	ret
	.cfi_endproc

	.globl	_w24_typed
_w24_typed = _w24_hand
.subsections_via_symbols
