	.build_version macos, 11, 0
	.section	__TEXT,__literal16,16byte_literals
	.p2align	4, 0x0
lCPI0_0:
	.quad	2
	.quad	3
lCPI0_1:
	.quad	0
	.quad	1
lCPI0_2:
	.quad	0
	.quad	-2
lCPI0_3:
	.quad	-4
	.quad	-6
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_w2_hand
	.p2align	2
_w2_hand:
	.cfi_startproc
Lloh0:
	adrp	x8, lCPI0_0@PAGE
Lloh1:
	ldr	q1, [x8, lCPI0_0@PAGEOFF]
	movi.2d	v0, #0000000000000000
	mov	w8, #6
	dup.2d	v2, x8
	mov	w8, #3
	dup.2d	v3, x8
	mov	w8, #16
	dup.2d	v4, x8
	movi.2d	v5, #0x000000000000ff
	movi.2d	v6, #0000000000000000
Lloh2:
	adrp	x8, lCPI0_1@PAGE
Lloh3:
	ldr	q7, [x8, lCPI0_1@PAGEOFF]
	add	x8, x0, #3
	mov	w9, #992
	movi.2d	v16, #0000000000000000
	movi.2d	v18, #0000000000000000
	movi.2d	v19, #0000000000000000
	movi.2d	v17, #0000000000000000
	movi.2d	v21, #0000000000000000
	movi.2d	v20, #0000000000000000
LBB0_1:
	add.2d	v22, v1, v1
	add.2d	v23, v7, v7
	and.16b	v23, v23, v2
	ldur	b24, [x8, #-3]
	dup.2s	v24, v24[0]
	and.16b	v22, v22, v2
	ushll.2d	v24, v24, #0
	and.16b	v24, v24, v5
	ldur	b25, [x8, #-2]
	dup.2s	v25, v25[0]
	ushll.2d	v25, v25, #0
	and.16b	v25, v25, v5
	ldur	b26, [x8, #-1]
	dup.2s	v26, v26[0]
	ushll.2d	v26, v26, #0
	and.16b	v26, v26, v5
	ldr	b27, [x8]
	dup.2s	v27, v27[0]
	ushll.2d	v27, v27, #0
	and.16b	v27, v27, v5
	neg.2d	v22, v22
	ushl.2d	v28, v24, v22
	neg.2d	v23, v23
	ushl.2d	v24, v24, v23
	ushl.2d	v29, v25, v22
	ushl.2d	v25, v25, v23
	ushl.2d	v30, v26, v22
	ushl.2d	v26, v26, v23
	ushl.2d	v22, v27, v22
	ushl.2d	v23, v27, v23
	and.16b	v24, v24, v3
	and.16b	v27, v28, v3
	and.16b	v25, v25, v3
	and.16b	v28, v29, v3
	and.16b	v26, v26, v3
	and.16b	v29, v30, v3
	and.16b	v23, v23, v3
	and.16b	v22, v22, v3
	add.2d	v16, v27, v16
	add.2d	v6, v24, v6
	add.2d	v19, v28, v19
	add.2d	v18, v25, v18
	add.2d	v21, v29, v21
	add.2d	v17, v26, v17
	add.2d	v0, v22, v0
	add.2d	v20, v23, v20
	add.2d	v1, v1, v4
	add.2d	v7, v7, v4
	add	x8, x8, #4
	subs	x9, x9, #16
	b.ne	LBB0_1
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
Lloh4:
	adrp	x8, lCPI0_2@PAGE
Lloh5:
	ldr	q3, [x8, lCPI0_2@PAGEOFF]
	ushl.2d	v4, v0, v3
Lloh6:
	adrp	x8, lCPI0_3@PAGE
Lloh7:
	ldr	q5, [x8, lCPI0_3@PAGEOFF]
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
	.loh AdrpLdr	Lloh2, Lloh3
	.loh AdrpLdr	Lloh0, Lloh1
	.loh AdrpLdr	Lloh6, Lloh7
	.loh AdrpAdrp	Lloh4, Lloh6
	.loh AdrpLdr	Lloh4, Lloh5
	.cfi_endproc

	.globl	_w2_native
	.p2align	2
_w2_native:
	.cfi_startproc
	add	x8, x0, #32
	movi.2d	v0, #0000000000000000
	mov	w9, #960
	movi.16b	v1, #3
	movi.16b	v2, #1
	movi.2d	v3, #0000000000000000
	movi.2d	v4, #0000000000000000
	movi.2d	v5, #0000000000000000
LBB1_1:
	ldp	q6, q7, [x8, #-32]
	ldp	q16, q17, [x8], #64
	and.16b	v6, v6, v1
	and.16b	v7, v7, v1
	and.16b	v16, v16, v1
	and.16b	v17, v17, v1
	movi.2d	v18, #0000000000000000
	udot.4s	v18, v6, v2
	uaddw.2d	v0, v0, v18
	uaddw2.2d	v0, v0, v18
	movi.2d	v6, #0000000000000000
	udot.4s	v6, v7, v2
	uaddw.2d	v3, v3, v6
	uaddw2.2d	v3, v3, v6
	movi.2d	v6, #0000000000000000
	udot.4s	v6, v16, v2
	uaddw.2d	v4, v4, v6
	uaddw2.2d	v4, v4, v6
	movi.2d	v6, #0000000000000000
	udot.4s	v6, v17, v2
	uaddw.2d	v5, v5, v6
	uaddw2.2d	v5, v5, v6
	subs	x9, x9, #64
	b.ne	LBB1_1
	add.2d	v0, v3, v0
	add.2d	v1, v5, v4
	add.2d	v0, v1, v0
	addp.2d	d0, v0
	movi.2d	v1, #0000000000000000
	mov.d	v1[0], v0[0]
	movi.16b	v0, #3
	ldp	q2, q3, [x0, #960]
	and.16b	v2, v2, v0
	movi.16b	v4, #1
	movi.2d	v5, #0000000000000000
	udot.4s	v5, v2, v4
	movi.2d	v2, #0000000000000000
	uaddw.2d	v1, v1, v5
	uaddw2.2d	v1, v1, v5
	and.16b	v0, v3, v0
	udot.4s	v2, v0, v4
	uaddw.2d	v0, v1, v2
	uaddw2.2d	v0, v0, v2
	addp.2d	d0, v0
	fmov	x8, d0
	ldrb	w9, [x0, #992]
	and	x9, x9, #0x3
	ldrb	w10, [x0, #993]
	and	x10, x10, #0x3
	ldrb	w11, [x0, #994]
	and	x11, x11, #0x3
	ldrb	w12, [x0, #995]
	and	x12, x12, #0x3
	add	x10, x10, x11
	add	x10, x10, x12
	ldrb	w11, [x0, #996]
	and	x11, x11, #0x3
	ldrb	w12, [x0, #997]
	and	x12, x12, #0x3
	add	x10, x10, x11
	add	x10, x10, x12
	ldrb	w11, [x0, #998]
	and	x11, x11, #0x3
	ldrb	w12, [x0, #999]
	and	x12, x12, #0x3
	add	x10, x10, x11
	add	x10, x10, x12
	add	x8, x8, x9
	add	x0, x8, x10
	ret
	.cfi_endproc

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
	.globl	_w2_typed
	.p2align	2
_w2_typed:
	.cfi_startproc
Lloh8:
	adrp	x8, lCPI2_0@PAGE
Lloh9:
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
Lloh10:
	adrp	x8, lCPI2_1@PAGE
Lloh11:
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
Lloh12:
	adrp	x8, lCPI2_2@PAGE
Lloh13:
	ldr	q3, [x8, lCPI2_2@PAGEOFF]
	ushl.2d	v4, v0, v3
Lloh14:
	adrp	x8, lCPI2_3@PAGE
Lloh15:
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
	.loh AdrpLdr	Lloh10, Lloh11
	.loh AdrpLdr	Lloh8, Lloh9
	.loh AdrpLdr	Lloh14, Lloh15
	.loh AdrpAdrp	Lloh12, Lloh14
	.loh AdrpLdr	Lloh12, Lloh13
	.cfi_endproc

.subsections_via_symbols
