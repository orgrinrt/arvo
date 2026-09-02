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
	.globl	_w4_gather
	.p2align	2
_w4_gather:
Lloh0:
	adrp	x8, lCPI1_0@PAGE
Lloh1:
	ldr	q0, [x8, lCPI1_0@PAGEOFF]
	add	x8, x0, #3
	movi.2d	v1, #0000000000000000
	mov	w9, #1000
	movi.2d	v2, #0x000000000000ff
	mov	w10, #4
	dup.2d	v3, x10
	mov	w10, #15
	dup.2d	v4, x10
	mov	w10, #8
	dup.2d	v5, x10
	movi.2d	v6, #0000000000000000
	movi.2d	v7, #0000000000000000
	movi.2d	v16, #0000000000000000
LBB1_1:
	shl.2d	v17, v0, #2
	ldur	b18, [x8, #-3]
	dup.2s	v18, v18[0]
	ldur	b19, [x8, #-2]
	dup.2s	v19, v19[0]
	ldur	b20, [x8, #-1]
	dup.2s	v20, v20[0]
	ldr	b21, [x8]
	dup.2s	v21, v21[0]
	ushll.2d	v18, v18, #0
	and.16b	v18, v18, v2
	ushll.2d	v19, v19, #0
	and.16b	v19, v19, v2
	ushll.2d	v20, v20, #0
	and.16b	v20, v20, v2
	ushll.2d	v21, v21, #0
	and.16b	v21, v21, v2
	and.16b	v17, v17, v3
	neg.2d	v17, v17
	ushl.2d	v18, v18, v17
	ushl.2d	v19, v19, v17
	ushl.2d	v20, v20, v17
	ushl.2d	v17, v21, v17
	and.16b	v18, v18, v4
	and.16b	v19, v19, v4
	and.16b	v20, v20, v4
	and.16b	v17, v17, v4
	add.2d	v1, v18, v1
	add.2d	v6, v19, v6
	add.2d	v7, v20, v7
	add.2d	v16, v17, v16
	add.2d	v0, v0, v5
	add	x8, x8, #4
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
	.quad	0
	.quad	1
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_w4_hand
	.p2align	2
_w4_hand:
Lloh2:
	adrp	x8, lCPI2_0@PAGE
Lloh3:
	ldr	q0, [x8, lCPI2_0@PAGEOFF]
	add	x8, x0, #3
	movi.2d	v1, #0000000000000000
	mov	w9, #1000
	mov	w10, #4
	dup.2d	v2, x10
	movi.2d	v3, #0x000000000000ff
	mov	w10, #15
	dup.2d	v4, x10
	mov	w10, #8
	dup.2d	v5, x10
	movi.2d	v6, #0000000000000000
	movi.2d	v7, #0000000000000000
	movi.2d	v16, #0000000000000000
LBB2_1:
	shl.2d	v17, v0, #2
	and.16b	v17, v17, v2
	ldur	b18, [x8, #-3]
	dup.2s	v18, v18[0]
	ldur	b19, [x8, #-2]
	dup.2s	v19, v19[0]
	ldur	b20, [x8, #-1]
	dup.2s	v20, v20[0]
	ldr	b21, [x8]
	dup.2s	v21, v21[0]
	ushll.2d	v18, v18, #0
	and.16b	v18, v18, v3
	ushll.2d	v19, v19, #0
	and.16b	v19, v19, v3
	ushll.2d	v20, v20, #0
	and.16b	v20, v20, v3
	ushll.2d	v21, v21, #0
	and.16b	v21, v21, v3
	neg.2d	v17, v17
	ushl.2d	v18, v18, v17
	ushl.2d	v19, v19, v17
	ushl.2d	v20, v20, v17
	ushl.2d	v17, v21, v17
	and.16b	v18, v18, v4
	and.16b	v19, v19, v4
	and.16b	v20, v20, v4
	and.16b	v17, v17, v4
	add.2d	v1, v18, v1
	add.2d	v6, v19, v6
	add.2d	v7, v20, v7
	add.2d	v16, v17, v16
	add.2d	v0, v0, v5
	add	x8, x8, #4
	subs	x9, x9, #8
	b.ne	LBB2_1
	add.2d	v0, v6, v1
	add.2d	v0, v7, v0
	add.2d	v0, v16, v0
	addp.2d	d0, v0
	fmov	x0, d0
	ret
	.loh AdrpLdr	Lloh2, Lloh3

	.globl	_w4_native
	.p2align	2
_w4_native:
	add	x8, x0, #32
	movi.2d	v0, #0000000000000000
	mov	w9, #960
	movi.16b	v1, #15
	movi.16b	v2, #1
	movi.2d	v3, #0000000000000000
	movi.2d	v4, #0000000000000000
	movi.2d	v5, #0000000000000000
LBB3_1:
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
	b.ne	LBB3_1
	add.2d	v0, v3, v0
	add.2d	v1, v5, v4
	add.2d	v0, v1, v0
	addp.2d	d0, v0
	movi.2d	v1, #0000000000000000
	mov.d	v1[0], v0[0]
	movi.16b	v0, #15
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
	and	x9, x9, #0xf
	ldrb	w10, [x0, #993]
	and	x10, x10, #0xf
	ldrb	w11, [x0, #994]
	and	x11, x11, #0xf
	ldrb	w12, [x0, #995]
	and	x12, x12, #0xf
	add	x10, x10, x11
	add	x10, x10, x12
	ldrb	w11, [x0, #996]
	and	x11, x11, #0xf
	ldrb	w12, [x0, #997]
	and	x12, x12, #0xf
	add	x10, x10, x11
	add	x10, x10, x12
	ldrb	w11, [x0, #998]
	and	x11, x11, #0xf
	ldrb	w12, [x0, #999]
	and	x12, x12, #0xf
	add	x10, x10, x11
	add	x10, x10, x12
	add	x8, x8, x9
	add	x0, x8, x10
	ret

	.section	__TEXT,__literal16,16byte_literals
	.p2align	4, 0x0
lCPI4_0:
	.quad	0
	.quad	1
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_w4_typed
	.p2align	2
_w4_typed:
Lloh4:
	adrp	x8, lCPI4_0@PAGE
Lloh5:
	ldr	q0, [x8, lCPI4_0@PAGEOFF]
	add	x8, x0, #3
	movi.2d	v1, #0000000000000000
	mov	w9, #1000
	movi.2d	v2, #0x000000000000ff
	mov	w10, #4
	dup.2d	v3, x10
	mov	w10, #15
	dup.2d	v4, x10
	mov	w10, #8
	dup.2d	v5, x10
	movi.2d	v6, #0000000000000000
	movi.2d	v7, #0000000000000000
	movi.2d	v16, #0000000000000000
LBB4_1:
	ldur	b17, [x8, #-3]
	dup.2s	v17, v17[0]
	ldur	b18, [x8, #-2]
	dup.2s	v18, v18[0]
	ldur	b19, [x8, #-1]
	dup.2s	v19, v19[0]
	ldr	b20, [x8]
	dup.2s	v20, v20[0]
	ushll.2d	v17, v17, #0
	and.16b	v17, v17, v2
	ushll.2d	v18, v18, #0
	and.16b	v18, v18, v2
	ushll.2d	v19, v19, #0
	and.16b	v19, v19, v2
	ushll.2d	v20, v20, #0
	and.16b	v20, v20, v2
	shl.2d	v21, v0, #2
	and.16b	v21, v21, v3
	neg.2d	v21, v21
	ushl.2d	v17, v17, v21
	ushl.2d	v18, v18, v21
	ushl.2d	v19, v19, v21
	ushl.2d	v20, v20, v21
	and.16b	v17, v17, v4
	and.16b	v18, v18, v4
	and.16b	v19, v19, v4
	and.16b	v20, v20, v4
	add.2d	v1, v17, v1
	add.2d	v6, v18, v6
	add.2d	v7, v19, v7
	add.2d	v16, v20, v16
	add.2d	v0, v0, v5
	add	x8, x8, #4
	subs	x9, x9, #8
	b.ne	LBB4_1
	add.2d	v0, v6, v1
	add.2d	v0, v7, v0
	add.2d	v0, v16, v0
	addp.2d	d0, v0
	fmov	x0, d0
	ret
	.loh AdrpLdr	Lloh4, Lloh5

	.section	__TEXT,__literal16,16byte_literals
	.p2align	4, 0x0
lCPI5_0:
	.quad	0
	.quad	1
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_w4_wide
	.p2align	2
_w4_wide:
Lloh6:
	adrp	x8, lCPI5_0@PAGE
Lloh7:
	ldr	q0, [x8, lCPI5_0@PAGEOFF]
	add	x8, x0, #3
	movi.2d	v1, #0000000000000000
	mov	w9, #1000
	mov	w10, #4
	dup.2d	v2, x10
	mov	w10, #15
	dup.2d	v3, x10
	mov	w10, #8
	dup.2d	v4, x10
	movi.2d	v5, #0000000000000000
	movi.2d	v6, #0000000000000000
	movi.2d	v7, #0000000000000000
LBB5_1:
	sub	x10, x8, #3
	sub	x11, x8, #2
	sub	x12, x8, #1
	ld1r.2d	{ v16 }, [x10]
	ld1r.2d	{ v17 }, [x11]
	ld1r.2d	{ v18 }, [x12]
	ld1r.2d	{ v19 }, [x8]
	shl.2d	v20, v0, #2
	and.16b	v20, v20, v2
	neg.2d	v20, v20
	ushl.2d	v16, v16, v20
	ushl.2d	v17, v17, v20
	ushl.2d	v18, v18, v20
	ushl.2d	v19, v19, v20
	and.16b	v16, v16, v3
	and.16b	v17, v17, v3
	and.16b	v18, v18, v3
	and.16b	v19, v19, v3
	add.2d	v1, v16, v1
	add.2d	v5, v17, v5
	add.2d	v6, v18, v6
	add.2d	v7, v19, v7
	add.2d	v0, v0, v4
	add	x8, x8, #4
	subs	x9, x9, #8
	b.ne	LBB5_1
	add.2d	v0, v5, v1
	add.2d	v0, v6, v0
	add.2d	v0, v7, v0
	addp.2d	d0, v0
	fmov	x0, d0
	ret
	.loh AdrpLdr	Lloh6, Lloh7

.subsections_via_symbols
