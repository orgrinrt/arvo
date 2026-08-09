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
	.globl	_w1_flat8
	.p2align	2
_w1_flat8:
Lloh0:
	adrp	x8, lCPI1_0@PAGE
Lloh1:
	ldr	q1, [x8, lCPI1_0@PAGEOFF]
	movi.2d	v0, #0000000000000000
	mov	w8, #1000
	mov	w9, #2
	dup.2d	v2, x9
	mov	w9, #6
	dup.2d	v3, x9
	mov	w9, #7
	dup.2d	v4, x9
	mov	w9, #4
	dup.2d	v5, x9
	mov	w9, #1
	dup.2d	v6, x9
	mov	w9, #8
	dup.2d	v7, x9
	movi.2d	v16, #0000000000000000
	movi.2d	v17, #0000000000000000
	movi.2d	v18, #0000000000000000
LBB1_1:
	add.2d	v19, v1, v2
	add.2d	v20, v1, v3
	ldr	x9, [x0], #1
	dup.2d	v21, x9
	and.16b	v22, v1, v4
	and.16b	v19, v19, v4
	eor.16b	v23, v22, v5
	and.16b	v20, v20, v4
	neg.2d	v22, v22
	ushl.2d	v22, v21, v22
	neg.2d	v19, v19
	ushl.2d	v19, v21, v19
	neg.2d	v23, v23
	ushl.2d	v23, v21, v23
	neg.2d	v20, v20
	ushl.2d	v20, v21, v20
	and.16b	v21, v22, v6
	and.16b	v19, v19, v6
	and.16b	v22, v23, v6
	and.16b	v20, v20, v6
	add.2d	v0, v21, v0
	add.2d	v16, v19, v16
	add.2d	v17, v22, v17
	add.2d	v18, v20, v18
	add.2d	v1, v1, v7
	subs	x8, x8, #8
	b.ne	LBB1_1
	add.2d	v0, v16, v0
	add.2d	v0, v17, v0
	add.2d	v0, v18, v0
	addp.2d	d0, v0
	fmov	x0, d0
	ret
	.loh AdrpLdr	Lloh0, Lloh1

	.section	__TEXT,__literal16,16byte_literals
	.p2align	4, 0x0
lCPI2_0:
	.quad	6
	.quad	7
lCPI2_1:
	.quad	4
	.quad	5
lCPI2_2:
	.quad	2
	.quad	3
lCPI2_3:
	.quad	0
	.quad	1
lCPI2_4:
	.quad	0
	.quad	-1
lCPI2_5:
	.quad	-4
	.quad	-5
lCPI2_6:
	.quad	-2
	.quad	-3
lCPI2_7:
	.quad	-6
	.quad	-7
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_w1_loop8
	.p2align	2
_w1_loop8:
	stp	d11, d10, [sp, #-32]!
	stp	d9, d8, [sp, #16]
Lloh2:
	adrp	x8, lCPI2_0@PAGE
Lloh3:
	ldr	q0, [x8, lCPI2_0@PAGEOFF]
Lloh4:
	adrp	x8, lCPI2_1@PAGE
Lloh5:
	ldr	q1, [x8, lCPI2_1@PAGEOFF]
	movi.2d	v2, #0000000000000000
	mov	w8, #7
	dup.2d	v3, x8
	mov	w8, #1
	dup.2d	v4, x8
	mov	w8, #16
	dup.2d	v5, x8
Lloh6:
	adrp	x8, lCPI2_2@PAGE
Lloh7:
	ldr	q7, [x8, lCPI2_2@PAGEOFF]
	movi.2d	v6, #0000000000000000
Lloh8:
	adrp	x8, lCPI2_3@PAGE
Lloh9:
	ldr	q17, [x8, lCPI2_3@PAGEOFF]
	add	x8, x0, #1
	mov	w9, #992
	movi.2d	v18, #0000000000000000
	movi.2d	v16, #0000000000000000
	movi.2d	v19, #0000000000000000
	movi.2d	v20, #0000000000000000
	movi.2d	v21, #0000000000000000
	movi.2d	v22, #0000000000000000
LBB2_1:
	sub	x10, x8, #1
	ld1r.8b	{ v23 }, [x10]
	ld1r.8b	{ v24 }, [x8]
	ushll.8h	v23, v23, #0
	ushll.4s	v25, v23, #0
	ushll.2d	v26, v25, #0
	ushll2.2d	v25, v25, #0
	ushll2.4s	v23, v23, #0
	ushll.2d	v27, v23, #0
	ushll2.2d	v23, v23, #0
	ushll.8h	v24, v24, #0
	ushll.4s	v28, v24, #0
	ushll.2d	v29, v28, #0
	ushll2.2d	v28, v28, #0
	ushll2.4s	v24, v24, #0
	ushll.2d	v30, v24, #0
	ushll2.2d	v24, v24, #0
	and.16b	v31, v17, v3
	and.16b	v8, v7, v3
	and.16b	v9, v1, v3
	and.16b	v10, v0, v3
	neg.2d	v10, v10
	ushl.2d	v23, v23, v10
	neg.2d	v9, v9
	ushl.2d	v27, v27, v9
	neg.2d	v8, v8
	ushl.2d	v25, v25, v8
	neg.2d	v31, v31
	ushl.2d	v26, v26, v31
	ushl.2d	v24, v24, v10
	ushl.2d	v30, v30, v9
	ushl.2d	v28, v28, v8
	ushl.2d	v29, v29, v31
	and.16b	v26, v26, v4
	and.16b	v25, v25, v4
	and.16b	v27, v27, v4
	and.16b	v23, v23, v4
	and.16b	v29, v29, v4
	and.16b	v28, v28, v4
	and.16b	v30, v30, v4
	and.16b	v24, v24, v4
	add.2d	v19, v23, v19
	add.2d	v16, v27, v16
	add.2d	v18, v25, v18
	add.2d	v6, v26, v6
	add.2d	v22, v24, v22
	add.2d	v21, v30, v21
	add.2d	v2, v28, v2
	add.2d	v20, v29, v20
	add.2d	v1, v1, v5
	add.2d	v7, v7, v5
	add.2d	v17, v17, v5
	add.2d	v0, v0, v5
	add	x8, x8, #2
	subs	x9, x9, #16
	b.ne	LBB2_1
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
Lloh10:
	adrp	x8, lCPI2_4@PAGE
Lloh11:
	ldr	q5, [x8, lCPI2_4@PAGEOFF]
	ushl.2d	v0, v0, v5
Lloh12:
	adrp	x8, lCPI2_5@PAGE
Lloh13:
	ldr	q5, [x8, lCPI2_5@PAGEOFF]
	ushl.2d	v2, v2, v5
Lloh14:
	adrp	x8, lCPI2_6@PAGE
Lloh15:
	ldr	q5, [x8, lCPI2_6@PAGEOFF]
	ushl.2d	v4, v4, v5
Lloh16:
	adrp	x8, lCPI2_7@PAGE
Lloh17:
	ldr	q5, [x8, lCPI2_7@PAGEOFF]
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
	ret
	.loh AdrpLdr	Lloh8, Lloh9
	.loh AdrpAdrp	Lloh6, Lloh8
	.loh AdrpLdr	Lloh6, Lloh7
	.loh AdrpLdr	Lloh4, Lloh5
	.loh AdrpAdrp	Lloh2, Lloh4
	.loh AdrpLdr	Lloh2, Lloh3
	.loh AdrpLdr	Lloh16, Lloh17
	.loh AdrpAdrp	Lloh14, Lloh16
	.loh AdrpLdr	Lloh14, Lloh15
	.loh AdrpAdrp	Lloh12, Lloh14
	.loh AdrpLdr	Lloh12, Lloh13
	.loh AdrpAdrp	Lloh10, Lloh12
	.loh AdrpLdr	Lloh10, Lloh11

	.globl	_w1_wide
_w1_wide = _w1_flat8
.subsections_via_symbols
