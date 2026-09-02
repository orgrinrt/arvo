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
	.quad	0
	.quad	1
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_w2_flat8
	.p2align	2
_w2_flat8:
Lloh0:
	adrp	x8, lCPI1_0@PAGE
Lloh1:
	ldr	q0, [x8, lCPI1_0@PAGEOFF]
	add	x8, x0, #1
	movi.2d	v1, #0000000000000000
	mov	w9, #1000
	mov	w10, #6
	dup.2d	v2, x10
	mov	w10, #4
	dup.2d	v3, x10
	mov	w10, #3
	dup.2d	v4, x10
	mov	w10, #8
	dup.2d	v5, x10
	movi.2d	v6, #0000000000000000
	movi.2d	v7, #0000000000000000
	movi.2d	v16, #0000000000000000
LBB1_1:
	add.2d	v17, v0, v0
	sub	x10, x8, #1
	ld1r.2d	{ v18 }, [x10]
	ld1r.2d	{ v19 }, [x8]
	and.16b	v17, v17, v2
	eor.16b	v20, v17, v3
	neg.2d	v17, v17
	ushl.2d	v21, v18, v17
	neg.2d	v20, v20
	ushl.2d	v18, v18, v20
	ushl.2d	v17, v19, v17
	ushl.2d	v19, v19, v20
	and.16b	v20, v21, v4
	and.16b	v18, v18, v4
	and.16b	v17, v17, v4
	and.16b	v19, v19, v4
	add.2d	v1, v20, v1
	add.2d	v6, v18, v6
	add.2d	v7, v17, v7
	add.2d	v16, v19, v16
	add.2d	v0, v0, v5
	add	x8, x8, #2
	subs	x9, x9, #8
	b.ne	LBB1_1
	add.2d	v0, v6, v1
	add.2d	v0, v7, v0
	add.2d	v0, v16, v0
	addp.2d	d0, v0
	fmov	x0, d0
	ret
	.loh AdrpLdr	Lloh0, Lloh1

	.section	__TEXT,__literal16,16byte_literals
	.p2align	4, 0x0
lCPI2_0:
	.quad	2
	.quad	3
lCPI2_1:
	.quad	0
	.quad	1
lCPI2_2:
	.quad	0
	.quad	-2
lCPI2_3:
	.quad	-4
	.quad	-6
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_w2_loop8
	.p2align	2
_w2_loop8:
Lloh2:
	adrp	x8, lCPI2_0@PAGE
Lloh3:
	ldr	q1, [x8, lCPI2_0@PAGEOFF]
	movi.2d	v0, #0000000000000000
	mov	w8, #6
	dup.2d	v2, x8
	mov	w8, #3
	dup.2d	v3, x8
	mov	w8, #16
	dup.2d	v4, x8
	movi.2d	v5, #0x000000000000ff
	movi.2d	v6, #0000000000000000
Lloh4:
	adrp	x8, lCPI2_1@PAGE
Lloh5:
	ldr	q7, [x8, lCPI2_1@PAGEOFF]
	add	x8, x0, #3
	mov	w9, #992
	movi.2d	v16, #0000000000000000
	movi.2d	v18, #0000000000000000
	movi.2d	v19, #0000000000000000
	movi.2d	v17, #0000000000000000
	movi.2d	v21, #0000000000000000
	movi.2d	v20, #0000000000000000
LBB2_1:
	ldur	b22, [x8, #-3]
	dup.2s	v22, v22[0]
	ushll.2d	v22, v22, #0
	and.16b	v22, v22, v5
	ldur	b23, [x8, #-2]
	dup.2s	v23, v23[0]
	ushll.2d	v23, v23, #0
	ldur	b24, [x8, #-1]
	dup.2s	v24, v24[0]
	and.16b	v23, v23, v5
	ushll.2d	v24, v24, #0
	and.16b	v24, v24, v5
	ldr	b25, [x8]
	dup.2s	v25, v25[0]
	ushll.2d	v25, v25, #0
	and.16b	v25, v25, v5
	add.2d	v26, v1, v1
	add.2d	v27, v7, v7
	and.16b	v27, v27, v2
	and.16b	v26, v26, v2
	neg.2d	v26, v26
	ushl.2d	v28, v22, v26
	neg.2d	v27, v27
	ushl.2d	v22, v22, v27
	ushl.2d	v29, v23, v26
	ushl.2d	v23, v23, v27
	ushl.2d	v30, v24, v26
	ushl.2d	v24, v24, v27
	ushl.2d	v26, v25, v26
	ushl.2d	v25, v25, v27
	and.16b	v22, v22, v3
	and.16b	v27, v28, v3
	and.16b	v23, v23, v3
	and.16b	v28, v29, v3
	and.16b	v24, v24, v3
	and.16b	v29, v30, v3
	and.16b	v25, v25, v3
	and.16b	v26, v26, v3
	add.2d	v16, v27, v16
	add.2d	v6, v22, v6
	add.2d	v19, v28, v19
	add.2d	v18, v23, v18
	add.2d	v21, v29, v21
	add.2d	v17, v24, v17
	add.2d	v0, v26, v0
	add.2d	v20, v25, v20
	add.2d	v1, v1, v4
	add.2d	v7, v7, v4
	add	x8, x8, #4
	subs	x9, x9, #16
	b.ne	LBB2_1
	add.2d	v1, v18, v6
	add.2d	v2, v19, v16
	add.2d	v2, v21, v2
	add.2d	v1, v17, v1
	add.2d	v1, v20, v1
	add.2d	v0, v0, v2
	add.2d	v0, v1, v0
	addp.2d	d0, v0
	movi.2d	v1, #0000000000000000
	mov.d	v1[0], v0[0]
	ldr	b0, [x0, #248]
	dup.2s	v0, v0[0]
	ushll.2d	v0, v0, #0
	movi.2d	v2, #0x000000000000ff
	and.16b	v0, v0, v2
Lloh6:
	adrp	x8, lCPI2_2@PAGE
Lloh7:
	ldr	q3, [x8, lCPI2_2@PAGEOFF]
	ushl.2d	v4, v0, v3
Lloh8:
	adrp	x8, lCPI2_3@PAGE
Lloh9:
	ldr	q5, [x8, lCPI2_3@PAGEOFF]
	ushl.2d	v0, v0, v5
	mov	w8, #3
	dup.2d	v6, x8
	and.16b	v0, v0, v6
	and.16b	v4, v4, v6
	ldr	b7, [x0, #249]
	dup.2s	v7, v7[0]
	ushll.2d	v7, v7, #0
	and.16b	v2, v7, v2
	ushl.2d	v5, v2, v5
	ushl.2d	v2, v2, v3
	and.16b	v2, v2, v6
	and.16b	v3, v5, v6
	add.2d	v0, v3, v0
	add.2d	v2, v2, v4
	add.2d	v0, v2, v0
	add.2d	v0, v0, v1
	addp.2d	d0, v0
	fmov	x0, d0
	ret
	.loh AdrpLdr	Lloh4, Lloh5
	.loh AdrpLdr	Lloh2, Lloh3
	.loh AdrpLdr	Lloh8, Lloh9
	.loh AdrpAdrp	Lloh6, Lloh8
	.loh AdrpLdr	Lloh6, Lloh7

	.section	__TEXT,__literal16,16byte_literals
	.p2align	4, 0x0
lCPI3_0:
	.quad	0
	.quad	1
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_w2_wide
	.p2align	2
_w2_wide:
Lloh10:
	adrp	x8, lCPI3_0@PAGE
Lloh11:
	ldr	q0, [x8, lCPI3_0@PAGEOFF]
	add	x8, x0, #1
	movi.2d	v1, #0000000000000000
	mov	w9, #1000
	mov	w10, #6
	dup.2d	v2, x10
	mov	w10, #4
	dup.2d	v3, x10
	mov	w10, #3
	dup.2d	v4, x10
	mov	w10, #8
	dup.2d	v5, x10
	movi.2d	v6, #0000000000000000
	movi.2d	v7, #0000000000000000
	movi.2d	v16, #0000000000000000
LBB3_1:
	sub	x10, x8, #1
	ld1r.2d	{ v17 }, [x10]
	ld1r.2d	{ v18 }, [x8]
	add.2d	v19, v0, v0
	and.16b	v19, v19, v2
	eor.16b	v20, v19, v3
	neg.2d	v19, v19
	ushl.2d	v21, v17, v19
	neg.2d	v20, v20
	ushl.2d	v17, v17, v20
	ushl.2d	v19, v18, v19
	ushl.2d	v18, v18, v20
	and.16b	v20, v21, v4
	and.16b	v17, v17, v4
	and.16b	v19, v19, v4
	and.16b	v18, v18, v4
	add.2d	v1, v20, v1
	add.2d	v6, v17, v6
	add.2d	v7, v19, v7
	add.2d	v16, v18, v16
	add.2d	v0, v0, v5
	add	x8, x8, #2
	subs	x9, x9, #8
	b.ne	LBB3_1
	add.2d	v0, v6, v1
	add.2d	v0, v7, v0
	add.2d	v0, v16, v0
	addp.2d	d0, v0
	fmov	x0, d0
	ret
	.loh AdrpLdr	Lloh10, Lloh11

.subsections_via_symbols
