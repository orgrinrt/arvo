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
	.quad	6
	.quad	7
lCPI1_1:
	.quad	4
	.quad	5
lCPI1_2:
	.quad	2
	.quad	3
lCPI1_3:
	.quad	0
	.quad	1
lCPI1_4:
	.quad	0
	.quad	-1
lCPI1_5:
	.quad	-4
	.quad	-5
lCPI1_6:
	.quad	-2
	.quad	-3
lCPI1_7:
	.quad	-6
	.quad	-7
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_w1_gather
	.p2align	2
_w1_gather:
	stp	d11, d10, [sp, #-32]!
	stp	d9, d8, [sp, #16]
Lloh0:
	adrp	x8, lCPI1_0@PAGE
Lloh1:
	ldr	q0, [x8, lCPI1_0@PAGEOFF]
Lloh2:
	adrp	x8, lCPI1_1@PAGE
Lloh3:
	ldr	q1, [x8, lCPI1_1@PAGEOFF]
	movi.2d	v2, #0000000000000000
	mov	w8, #7
	dup.2d	v3, x8
	mov	w8, #1
	dup.2d	v4, x8
	mov	w8, #16
	dup.2d	v5, x8
Lloh4:
	adrp	x8, lCPI1_2@PAGE
Lloh5:
	ldr	q7, [x8, lCPI1_2@PAGEOFF]
	movi.2d	v6, #0000000000000000
Lloh6:
	adrp	x8, lCPI1_3@PAGE
Lloh7:
	ldr	q17, [x8, lCPI1_3@PAGEOFF]
	add	x8, x0, #1
	mov	w9, #992
	movi.2d	v18, #0000000000000000
	movi.2d	v16, #0000000000000000
	movi.2d	v19, #0000000000000000
	movi.2d	v20, #0000000000000000
	movi.2d	v21, #0000000000000000
	movi.2d	v22, #0000000000000000
LBB1_1:
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
	b.ne	LBB1_1
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
	adrp	x8, lCPI1_4@PAGE
Lloh9:
	ldr	q5, [x8, lCPI1_4@PAGEOFF]
	ushl.2d	v0, v0, v5
Lloh10:
	adrp	x8, lCPI1_5@PAGE
Lloh11:
	ldr	q5, [x8, lCPI1_5@PAGEOFF]
	ushl.2d	v2, v2, v5
Lloh12:
	adrp	x8, lCPI1_6@PAGE
Lloh13:
	ldr	q5, [x8, lCPI1_6@PAGEOFF]
	ushl.2d	v4, v4, v5
Lloh14:
	adrp	x8, lCPI1_7@PAGE
Lloh15:
	ldr	q5, [x8, lCPI1_7@PAGEOFF]
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
	.globl	_w1_hand
	.p2align	2
_w1_hand:
	stp	d11, d10, [sp, #-32]!
	stp	d9, d8, [sp, #16]
Lloh16:
	adrp	x8, lCPI2_0@PAGE
Lloh17:
	ldr	q0, [x8, lCPI2_0@PAGEOFF]
Lloh18:
	adrp	x8, lCPI2_1@PAGE
Lloh19:
	ldr	q1, [x8, lCPI2_1@PAGEOFF]
	movi.2d	v2, #0000000000000000
	mov	w8, #7
	dup.2d	v3, x8
	mov	w8, #1
	dup.2d	v4, x8
	mov	w8, #16
	dup.2d	v5, x8
Lloh20:
	adrp	x8, lCPI2_2@PAGE
Lloh21:
	ldr	q7, [x8, lCPI2_2@PAGEOFF]
	movi.2d	v6, #0000000000000000
Lloh22:
	adrp	x8, lCPI2_3@PAGE
Lloh23:
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
Lloh24:
	adrp	x8, lCPI2_4@PAGE
Lloh25:
	ldr	q5, [x8, lCPI2_4@PAGEOFF]
	ushl.2d	v0, v0, v5
Lloh26:
	adrp	x8, lCPI2_5@PAGE
Lloh27:
	ldr	q5, [x8, lCPI2_5@PAGEOFF]
	ushl.2d	v2, v2, v5
Lloh28:
	adrp	x8, lCPI2_6@PAGE
Lloh29:
	ldr	q5, [x8, lCPI2_6@PAGEOFF]
	ushl.2d	v4, v4, v5
Lloh30:
	adrp	x8, lCPI2_7@PAGE
Lloh31:
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
	.loh AdrpLdr	Lloh22, Lloh23
	.loh AdrpAdrp	Lloh20, Lloh22
	.loh AdrpLdr	Lloh20, Lloh21
	.loh AdrpLdr	Lloh18, Lloh19
	.loh AdrpAdrp	Lloh16, Lloh18
	.loh AdrpLdr	Lloh16, Lloh17
	.loh AdrpLdr	Lloh30, Lloh31
	.loh AdrpAdrp	Lloh28, Lloh30
	.loh AdrpLdr	Lloh28, Lloh29
	.loh AdrpAdrp	Lloh26, Lloh28
	.loh AdrpLdr	Lloh26, Lloh27
	.loh AdrpAdrp	Lloh24, Lloh26
	.loh AdrpLdr	Lloh24, Lloh25

	.globl	_w1_native
	.p2align	2
_w1_native:
	add	x8, x0, #32
	movi.2d	v0, #0000000000000000
	mov	w9, #960
	movi.16b	v1, #1
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
	movi.2d	v17, #0000000000000000
	udot.4s	v17, v5, v1
	uaddw.2d	v0, v0, v17
	uaddw2.2d	v0, v0, v17
	movi.2d	v5, #0000000000000000
	udot.4s	v5, v6, v1
	uaddw.2d	v2, v2, v5
	uaddw2.2d	v2, v2, v5
	movi.2d	v5, #0000000000000000
	udot.4s	v5, v7, v1
	uaddw.2d	v3, v3, v5
	uaddw2.2d	v3, v3, v5
	movi.2d	v5, #0000000000000000
	udot.4s	v5, v16, v1
	uaddw.2d	v4, v4, v5
	uaddw2.2d	v4, v4, v5
	subs	x9, x9, #64
	b.ne	LBB3_1
	add.2d	v0, v2, v0
	add.2d	v1, v4, v3
	add.2d	v0, v1, v0
	addp.2d	d0, v0
	movi.2d	v1, #0000000000000000
	mov.d	v1[0], v0[0]
	movi.16b	v0, #1
	ldp	q2, q3, [x0, #960]
	and.16b	v2, v2, v0
	movi.2d	v4, #0000000000000000
	udot.4s	v4, v2, v0
	movi.2d	v2, #0000000000000000
	uaddw.2d	v1, v1, v4
	uaddw2.2d	v1, v1, v4
	and.16b	v3, v3, v0
	udot.4s	v2, v3, v0
	uaddw.2d	v0, v1, v2
	uaddw2.2d	v0, v0, v2
	addp.2d	d0, v0
	fmov	x8, d0
	ldrb	w9, [x0, #992]
	and	x9, x9, #0x1
	ldrb	w10, [x0, #993]
	and	x10, x10, #0x1
	ldrb	w11, [x0, #994]
	and	x11, x11, #0x1
	ldrb	w12, [x0, #995]
	and	x12, x12, #0x1
	add	x10, x10, x11
	add	x10, x10, x12
	ldrb	w11, [x0, #996]
	and	x11, x11, #0x1
	ldrb	w12, [x0, #997]
	and	x12, x12, #0x1
	add	x10, x10, x11
	add	x10, x10, x12
	ldrb	w11, [x0, #998]
	and	x11, x11, #0x1
	ldrb	w12, [x0, #999]
	and	x12, x12, #0x1
	add	x10, x10, x11
	add	x10, x10, x12
	add	x8, x8, x9
	add	x0, x8, x10
	ret

	.section	__TEXT,__literal16,16byte_literals
	.p2align	4, 0x0
lCPI4_0:
	.quad	6
	.quad	7
lCPI4_1:
	.quad	4
	.quad	5
lCPI4_2:
	.quad	2
	.quad	3
lCPI4_3:
	.quad	0
	.quad	1
lCPI4_4:
	.quad	0
	.quad	-1
lCPI4_5:
	.quad	-4
	.quad	-5
lCPI4_6:
	.quad	-2
	.quad	-3
lCPI4_7:
	.quad	-6
	.quad	-7
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_w1_typed
	.p2align	2
_w1_typed:
	stp	d11, d10, [sp, #-32]!
	stp	d9, d8, [sp, #16]
Lloh32:
	adrp	x8, lCPI4_0@PAGE
Lloh33:
	ldr	q0, [x8, lCPI4_0@PAGEOFF]
Lloh34:
	adrp	x8, lCPI4_1@PAGE
Lloh35:
	ldr	q1, [x8, lCPI4_1@PAGEOFF]
	movi.2d	v2, #0000000000000000
	mov	w8, #7
	dup.2d	v3, x8
	mov	w8, #1
	dup.2d	v4, x8
	mov	w8, #16
	dup.2d	v5, x8
Lloh36:
	adrp	x8, lCPI4_2@PAGE
Lloh37:
	ldr	q7, [x8, lCPI4_2@PAGEOFF]
	movi.2d	v6, #0000000000000000
Lloh38:
	adrp	x8, lCPI4_3@PAGE
Lloh39:
	ldr	q17, [x8, lCPI4_3@PAGEOFF]
	add	x8, x0, #1
	mov	w9, #992
	movi.2d	v18, #0000000000000000
	movi.2d	v16, #0000000000000000
	movi.2d	v19, #0000000000000000
	movi.2d	v20, #0000000000000000
	movi.2d	v21, #0000000000000000
	movi.2d	v22, #0000000000000000
LBB4_1:
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
	b.ne	LBB4_1
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
Lloh40:
	adrp	x8, lCPI4_4@PAGE
Lloh41:
	ldr	q5, [x8, lCPI4_4@PAGEOFF]
	ushl.2d	v0, v0, v5
Lloh42:
	adrp	x8, lCPI4_5@PAGE
Lloh43:
	ldr	q5, [x8, lCPI4_5@PAGEOFF]
	ushl.2d	v2, v2, v5
Lloh44:
	adrp	x8, lCPI4_6@PAGE
Lloh45:
	ldr	q5, [x8, lCPI4_6@PAGEOFF]
	ushl.2d	v4, v4, v5
Lloh46:
	adrp	x8, lCPI4_7@PAGE
Lloh47:
	ldr	q5, [x8, lCPI4_7@PAGEOFF]
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
	.loh AdrpLdr	Lloh38, Lloh39
	.loh AdrpAdrp	Lloh36, Lloh38
	.loh AdrpLdr	Lloh36, Lloh37
	.loh AdrpLdr	Lloh34, Lloh35
	.loh AdrpAdrp	Lloh32, Lloh34
	.loh AdrpLdr	Lloh32, Lloh33
	.loh AdrpLdr	Lloh46, Lloh47
	.loh AdrpAdrp	Lloh44, Lloh46
	.loh AdrpLdr	Lloh44, Lloh45
	.loh AdrpAdrp	Lloh42, Lloh44
	.loh AdrpLdr	Lloh42, Lloh43
	.loh AdrpAdrp	Lloh40, Lloh42
	.loh AdrpLdr	Lloh40, Lloh41

	.section	__TEXT,__literal16,16byte_literals
	.p2align	4, 0x0
lCPI5_0:
	.quad	0
	.quad	1
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_w1_wide
	.p2align	2
_w1_wide:
Lloh48:
	adrp	x8, lCPI5_0@PAGE
Lloh49:
	ldr	q1, [x8, lCPI5_0@PAGEOFF]
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
LBB5_1:
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
	b.ne	LBB5_1
	add.2d	v0, v16, v0
	add.2d	v0, v17, v0
	add.2d	v0, v18, v0
	addp.2d	d0, v0
	fmov	x0, d0
	ret
	.loh AdrpLdr	Lloh48, Lloh49

.subsections_via_symbols
