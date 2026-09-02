	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_w32_hand
	.p2align	2
_w32_hand:
	.cfi_startproc
	stp	d15, d14, [sp, #-64]!
	.cfi_def_cfa_offset 64
	stp	d13, d12, [sp, #16]
	stp	d11, d10, [sp, #32]
	stp	d9, d8, [sp, #48]
	.cfi_offset b8, -8
	.cfi_offset b9, -16
	.cfi_offset b10, -24
	.cfi_offset b11, -32
	.cfi_offset b12, -40
	.cfi_offset b13, -48
	.cfi_offset b14, -56
	.cfi_offset b15, -64
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
	add	x8, x8, #64
	ld4.16b	{ v16, v17, v18, v19 }, [x9]
	ushll2.8h	v20, v16, #0
	ushll2.4s	v21, v20, #0
	ushll2.2d	v22, v21, #0
	ushll.2d	v21, v21, #0
	ushll.4s	v20, v20, #0
	ushll2.2d	v23, v20, #0
	ushll.8h	v24, v16, #0
	ushll2.4s	v25, v24, #0
	ushll2.2d	v26, v25, #0
	ushll.2d	v20, v20, #0
	ushll.2d	v25, v25, #0
	ushll.4s	v24, v24, #0
	ushll2.2d	v27, v24, #0
	ushll.2d	v24, v24, #0
	ushll.8h	v28, v17, #0
	ushll.4s	v29, v28, #0
	ushll2.4s	v28, v28, #0
	ushll2.8h	v30, v17, #0
	ushll.4s	v31, v30, #0
	ushll2.4s	v30, v30, #0
	ushll2.2d	v8, v30, #8
	ushll.2d	v30, v30, #8
	ushll2.2d	v9, v31, #8
	ushll2.2d	v10, v28, #8
	ushll.2d	v31, v31, #8
	ushll.2d	v28, v28, #8
	ushll2.2d	v11, v29, #8
	ushll.2d	v29, v29, #8
	orr.16b	v24, v29, v24
	orr.16b	v27, v11, v27
	orr.16b	v25, v28, v25
	orr.16b	v20, v31, v20
	orr.16b	v26, v10, v26
	orr.16b	v23, v9, v23
	orr.16b	v21, v30, v21
	orr.16b	v22, v8, v22
	ushll2.8h	v28, v18, #0
	ushll2.4s	v29, v28, #0
	ushll.4s	v28, v28, #0
	ushll.8h	v30, v18, #0
	ushll2.4s	v31, v30, #0
	ushll.4s	v30, v30, #0
	ushll.2d	v8, v30, #16
	ushll2.2d	v30, v30, #16
	ushll.2d	v9, v31, #16
	ushll.2d	v10, v28, #16
	ushll2.2d	v31, v31, #16
	ushll2.2d	v28, v28, #16
	ushll.2d	v11, v29, #16
	ushll2.2d	v29, v29, #16
	ushll.8h	v12, v19, #0
	ushll.4s	v13, v12, #0
	ushll2.4s	v12, v12, #0
	ushll2.8h	v16, v19, #0
	ushll.4s	v17, v16, #0
	ushll2.4s	v16, v16, #0
	ushll2.2d	v18, v16, #24
	ushll.2d	v16, v16, #24
	ushll2.2d	v19, v17, #24
	ushll2.2d	v14, v12, #24
	ushll.2d	v17, v17, #24
	ushll.2d	v12, v12, #24
	ushll2.2d	v15, v13, #24
	ushll.2d	v13, v13, #24
	orr.16b	v8, v13, v8
	orr.16b	v24, v8, v24
	orr.16b	v30, v15, v30
	orr.16b	v27, v30, v27
	orr.16b	v30, v12, v9
	orr.16b	v25, v30, v25
	orr.16b	v17, v17, v10
	orr.16b	v17, v17, v20
	orr.16b	v20, v14, v31
	orr.16b	v20, v20, v26
	orr.16b	v19, v19, v28
	orr.16b	v19, v19, v23
	orr.16b	v16, v16, v11
	orr.16b	v16, v16, v21
	orr.16b	v18, v18, v29
	orr.16b	v18, v18, v22
	add.2d	v5, v18, v5
	add.2d	v7, v16, v7
	add.2d	v4, v19, v4
	add.2d	v1, v20, v1
	add.2d	v6, v17, v6
	add.2d	v3, v25, v3
	add.2d	v0, v27, v0
	add.2d	v2, v24, v2
	cmp	x8, #3968
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
	add	x8, x0, #3968
	ld4.8b	{ v2, v3, v4, v5 }, [x8]
	ushll.8h	v0, v2, #0
	ushll.4s	v6, v0, #0
	ushll.2d	v7, v6, #0
	ushll2.4s	v0, v0, #0
	ushll.2d	v16, v0, #0
	ushll2.2d	v6, v6, #0
	ushll2.2d	v0, v0, #0
	ushll.8h	v17, v3, #0
	ushll2.4s	v18, v17, #0
	ushll.4s	v17, v17, #0
	ushll.2d	v19, v17, #8
	ushll.2d	v20, v18, #8
	ushll2.2d	v17, v17, #8
	ushll2.2d	v18, v18, #8
	orr.16b	v0, v18, v0
	orr.16b	v6, v17, v6
	orr.16b	v16, v20, v16
	orr.16b	v7, v19, v7
	ushll.8h	v17, v4, #0
	ushll.4s	v18, v17, #0
	ushll2.4s	v17, v17, #0
	ushll2.2d	v19, v17, #16
	ushll2.2d	v20, v18, #16
	ushll.2d	v17, v17, #16
	ushll.2d	v18, v18, #16
	ushll.8h	v2, v5, #0
	ushll2.4s	v3, v2, #0
	ushll.4s	v2, v2, #0
	ushll.2d	v4, v2, #24
	ushll.2d	v5, v3, #24
	ushll2.2d	v2, v2, #24
	ushll2.2d	v3, v3, #24
	orr.16b	v3, v3, v19
	orr.16b	v0, v3, v0
	orr.16b	v2, v2, v20
	orr.16b	v2, v2, v6
	orr.16b	v3, v5, v17
	orr.16b	v3, v3, v16
	orr.16b	v4, v4, v18
	orr.16b	v4, v4, v7
	add.2d	v3, v4, v3
	add.2d	v0, v2, v0
	add.2d	v0, v3, v0
	add.2d	v0, v0, v1
	addp.2d	d0, v0
	fmov	x0, d0
	ldp	d9, d8, [sp, #48]
	ldp	d11, d10, [sp, #32]
	ldp	d13, d12, [sp, #16]
	ldp	d15, d14, [sp], #64
	.cfi_def_cfa_offset 0
	.cfi_restore b8
	.cfi_restore b9
	.cfi_restore b10
	.cfi_restore b11
	.cfi_restore b12
	.cfi_restore b13
	.cfi_restore b14
	.cfi_restore b15
	ret
	.cfi_endproc

	.globl	_w32_typed
_w32_typed = _w32_hand
.subsections_via_symbols
