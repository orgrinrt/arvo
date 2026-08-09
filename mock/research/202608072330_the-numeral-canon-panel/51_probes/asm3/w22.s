	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind
	.p2align	2
__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind:
LBB0_1:
	b	LBB0_1

	.globl	_w22_gather
	.p2align	2
_w22_gather:
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	add	x12, x0, #5
	mov	w13, #1000
LBB1_1:
	ldur	w14, [x12, #-5]
	ldur	w15, [x12, #-3]
	ldr	w16, [x12]
	and	x14, x14, #0x3fffff
	ubfx	x15, x15, #6, #22
	ldur	w17, [x12, #3]
	ubfx	x16, x16, #4, #22
	ubfx	x17, x17, #2, #22
	add	x8, x8, x14
	add	x9, x9, x15
	add	x10, x10, x16
	add	x11, x11, x17
	add	x12, x12, #11
	subs	x13, x13, #4
	b.ne	LBB1_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret

	.globl	_w22_hand
	.p2align	2
_w22_hand:
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	add	x12, x0, #5
	mov	w13, #1000
LBB2_1:
	ldur	w14, [x12, #-5]
	ldur	w15, [x12, #-3]
	ldr	w16, [x12]
	and	x14, x14, #0x3fffff
	ubfx	x15, x15, #6, #22
	ldur	w17, [x12, #3]
	ubfx	x16, x16, #4, #22
	ubfx	x17, x17, #2, #22
	add	x8, x8, x14
	add	x9, x9, x15
	add	x10, x10, x16
	add	x11, x11, x17
	add	x12, x12, #11
	subs	x13, x13, #4
	b.ne	LBB2_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret

	.globl	_w22_native
	.p2align	2
_w22_native:
	add	x8, x0, #32
	movi.2d	v0, #0000000000000000
	mov	w9, #992
	movi.4s	v1, #63, msl #16
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
	uaddw.2d	v0, v0, v5
	uaddw2.2d	v0, v0, v5
	uaddw.2d	v2, v2, v6
	uaddw2.2d	v2, v2, v6
	uaddw.2d	v3, v3, v7
	uaddw2.2d	v3, v3, v7
	uaddw.2d	v4, v4, v16
	uaddw2.2d	v4, v4, v16
	subs	x9, x9, #16
	b.ne	LBB3_1
	add.2d	v0, v2, v0
	add.2d	v1, v4, v3
	add.2d	v0, v1, v0
	addp.2d	d0, v0
	movi.2d	v1, #0000000000000000
	mov.d	v1[0], v0[0]
	ldr	q0, [x0, #3968]
	movi.4s	v2, #63, msl #16
	and.16b	v0, v0, v2
	uaddw.2d	v1, v1, v0
	uaddw2.2d	v0, v1, v0
	ldr	q1, [x0, #3984]
	and.16b	v1, v1, v2
	uaddw.2d	v0, v0, v1
	uaddw2.2d	v0, v0, v1
	addp.2d	d0, v0
	fmov	x0, d0
	ret

	.globl	_w22_typed
	.p2align	2
_w22_typed:
	mov	x9, #0
	mov	x8, #0
	mov	w10, #1000
LBB4_1:
	lsr	x11, x9, #3
	ldr	w11, [x0, x11]
	and	x12, x9, #0x6
	lsr	x11, x11, x12
	and	x11, x11, #0x3fffff
	add	x8, x11, x8
	add	x9, x9, #22
	subs	x10, x10, #1
	b.ne	LBB4_1
	mov	x0, x8
	ret

	.section	__TEXT,__literal16,16byte_literals
	.p2align	4, 0x0
lCPI5_0:
	.quad	-2
	.quad	-4
lCPI5_1:
	.quad	-6
	.quad	0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_w22_wide
	.p2align	2
_w22_wide:
	add	x8, x0, #2
	movi.2d	v0, #0000000000000000
	mov	w9, #1000
Lloh0:
	adrp	x10, lCPI5_0@PAGE
Lloh1:
	ldr	q1, [x10, lCPI5_0@PAGEOFF]
Lloh2:
	adrp	x10, lCPI5_1@PAGE
Lloh3:
	ldr	q2, [x10, lCPI5_1@PAGEOFF]
	mov	w10, #4194303
	dup.2d	v3, x10
	movi.2d	v4, #0000000000000000
LBB5_1:
	add	x10, x8, #3
	ldur	q5, [x8, #-2]
	dup.2d	v6, v5[1]
	dup.2d	v5, v5[0]
	ld1.d	{ v5 }[0], [x8]
	ld1.d	{ v6 }[1], [x10]
	ushl.2d	v6, v6, v1
	ushl.2d	v5, v5, v2
	and.16b	v5, v5, v3
	and.16b	v6, v6, v3
	add.2d	v0, v6, v0
	add.2d	v4, v5, v4
	add	x8, x8, #11
	subs	x9, x9, #4
	b.ne	LBB5_1
	add.2d	v0, v4, v0
	addp.2d	d0, v0
	fmov	x0, d0
	ret
	.loh AdrpLdr	Lloh2, Lloh3
	.loh AdrpAdrp	Lloh0, Lloh2
	.loh AdrpLdr	Lloh0, Lloh1

.subsections_via_symbols
