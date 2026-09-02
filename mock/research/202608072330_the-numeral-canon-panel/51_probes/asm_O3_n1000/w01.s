	.build_version macos, 11, 0
	.section	__TEXT,__literal16,16byte_literals
	.p2align	4, 0x0
lCPI0_0:
	.quad	6
	.quad	7
lCPI0_1:
	.quad	4
	.quad	5
lCPI0_2:
	.quad	2
	.quad	3
lCPI0_3:
	.quad	0
	.quad	1
lCPI0_4:
	.quad	0
	.quad	-1
lCPI0_5:
	.quad	-4
	.quad	-5
lCPI0_6:
	.quad	-2
	.quad	-3
lCPI0_7:
	.quad	-6
	.quad	-7
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_w1_hand
	.p2align	2
_w1_hand:
	.cfi_startproc
	stp	d11, d10, [sp, #-32]!
	.cfi_def_cfa_offset 32
	stp	d9, d8, [sp, #16]
	.cfi_offset b8, -8
	.cfi_offset b9, -16
	.cfi_offset b10, -24
	.cfi_offset b11, -32
Lloh0:
	adrp	x8, lCPI0_0@PAGE
Lloh1:
	ldr	q0, [x8, lCPI0_0@PAGEOFF]
Lloh2:
	adrp	x8, lCPI0_1@PAGE
Lloh3:
	ldr	q1, [x8, lCPI0_1@PAGEOFF]
	movi.2d	v2, #0000000000000000
	mov	w8, #7
	dup.2d	v3, x8
	mov	w8, #1
	dup.2d	v4, x8
	mov	w8, #16
	dup.2d	v5, x8
Lloh4:
	adrp	x8, lCPI0_2@PAGE
Lloh5:
	ldr	q7, [x8, lCPI0_2@PAGEOFF]
	movi.2d	v6, #0000000000000000
Lloh6:
	adrp	x8, lCPI0_3@PAGE
Lloh7:
	ldr	q17, [x8, lCPI0_3@PAGEOFF]
	add	x8, x0, #1
	mov	w9, #992
	movi.2d	v18, #0000000000000000
	movi.2d	v16, #0000000000000000
	movi.2d	v19, #0000000000000000
	movi.2d	v20, #0000000000000000
	movi.2d	v21, #0000000000000000
	movi.2d	v22, #0000000000000000
LBB0_1:
	and.16b	v23, v17, v3
	and.16b	v24, v7, v3
	and.16b	v25, v1, v3
	and.16b	v26, v0, v3
	sub	x10, x8, #1
	ld1r.8b	{ v27 }, [x10]
	ld1r.8b	{ v28 }, [x8]
	ushll.8h	v27, v27, #0
	ushll.4s	v29, v27, #0
	ushll.2d	v30, v29, #0
	ushll2.2d	v29, v29, #0
	ushll2.4s	v27, v27, #0
	ushll.2d	v31, v27, #0
	ushll2.2d	v27, v27, #0
	ushll.8h	v28, v28, #0
	ushll.4s	v8, v28, #0
	ushll.2d	v9, v8, #0
	ushll2.2d	v8, v8, #0
	ushll2.4s	v28, v28, #0
	ushll.2d	v10, v28, #0
	ushll2.2d	v28, v28, #0
	neg.2d	v26, v26
	ushl.2d	v27, v27, v26
	neg.2d	v25, v25
	ushl.2d	v31, v31, v25
	neg.2d	v24, v24
	ushl.2d	v29, v29, v24
	neg.2d	v23, v23
	ushl.2d	v30, v30, v23
	ushl.2d	v26, v28, v26
	ushl.2d	v25, v10, v25
	ushl.2d	v24, v8, v24
	ushl.2d	v23, v9, v23
	and.16b	v28, v30, v4
	and.16b	v29, v29, v4
	and.16b	v30, v31, v4
	and.16b	v27, v27, v4
	and.16b	v23, v23, v4
	and.16b	v24, v24, v4
	and.16b	v25, v25, v4
	and.16b	v26, v26, v4
	add.2d	v19, v27, v19
	add.2d	v16, v30, v16
	add.2d	v18, v29, v18
	add.2d	v6, v28, v6
	add.2d	v22, v26, v22
	add.2d	v21, v25, v21
	add.2d	v2, v24, v2
	add.2d	v20, v23, v20
	add.2d	v1, v1, v5
	add.2d	v7, v7, v5
	add.2d	v17, v17, v5
	add.2d	v0, v0, v5
	add	x8, x8, #2
	subs	x9, x9, #16
	b.ne	LBB0_1
	add.2d	v0, v2, v18
	add.2d	v1, v22, v19
	add.2d	v2, v20, v6
	add.2d	v3, v21, v16
	add.2d	v2, v2, v3
	add.2d	v0, v0, v1
	add.2d	v0, v2, v0
	addp.2d	d0, v0
	movi.2d	v1, #0000000000000000
	mov.d	v1[0], v0[0]
	add	x8, x0, #124
	ld1r.8b	{ v0 }, [x8]
	ushll.8h	v0, v0, #0
	ushll2.4s	v2, v0, #0
	ushll2.2d	v3, v2, #0
	ushll.4s	v0, v0, #0
	ushll2.2d	v4, v0, #0
	ushll.2d	v2, v2, #0
	ushll.2d	v0, v0, #0
Lloh8:
	adrp	x8, lCPI0_4@PAGE
Lloh9:
	ldr	q5, [x8, lCPI0_4@PAGEOFF]
	ushl.2d	v0, v0, v5
Lloh10:
	adrp	x8, lCPI0_5@PAGE
Lloh11:
	ldr	q5, [x8, lCPI0_5@PAGEOFF]
	ushl.2d	v2, v2, v5
Lloh12:
	adrp	x8, lCPI0_6@PAGE
Lloh13:
	ldr	q5, [x8, lCPI0_6@PAGEOFF]
	ushl.2d	v4, v4, v5
Lloh14:
	adrp	x8, lCPI0_7@PAGE
Lloh15:
	ldr	q5, [x8, lCPI0_7@PAGEOFF]
	mov	w8, #1
	dup.2d	v6, x8
	ushl.2d	v3, v3, v5
	and.16b	v3, v3, v6
	and.16b	v4, v4, v6
	and.16b	v2, v2, v6
	and.16b	v0, v0, v6
	add.2d	v0, v0, v2
	add.2d	v2, v4, v3
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
	.loh AdrpLdr	Lloh6, Lloh7
	.loh AdrpAdrp	Lloh4, Lloh6
	.loh AdrpLdr	Lloh4, Lloh5
	.loh AdrpLdr	Lloh2, Lloh3
	.loh AdrpAdrp	Lloh0, Lloh2
	.loh AdrpLdr	Lloh0, Lloh1
	.loh AdrpLdr	Lloh14, Lloh15
	.loh AdrpAdrp	Lloh12, Lloh14
	.loh AdrpLdr	Lloh12, Lloh13
	.loh AdrpAdrp	Lloh10, Lloh12
	.loh AdrpLdr	Lloh10, Lloh11
	.loh AdrpAdrp	Lloh8, Lloh10
	.loh AdrpLdr	Lloh8, Lloh9
	.cfi_endproc

	.globl	_w1_typed
_w1_typed = _w1_hand
.subsections_via_symbols
