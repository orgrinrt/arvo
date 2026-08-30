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
	.quad	-6
	.quad	-2
lCPI1_1:
	.quad	0
	.quad	-4
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_w30_flat8
	.p2align	2
_w30_flat8:
	movi.2d	v0, #0000000000000000
	mov	w8, #1000
Lloh0:
	adrp	x9, lCPI1_0@PAGE
Lloh1:
	ldr	q1, [x9, lCPI1_0@PAGEOFF]
Lloh2:
	adrp	x9, lCPI1_1@PAGE
Lloh3:
	ldr	q2, [x9, lCPI1_1@PAGEOFF]
	mov	w9, #1073741823
	dup.2d	v3, x9
	movi.2d	v4, #0000000000000000
LBB1_1:
	add	x9, x0, #7
	ldur	q5, [x0, #3]
	ldr	d6, [x0], #15
	ld1.d	{ v6 }[1], [x9]
	ushl.2d	v5, v5, v1
	ushl.2d	v6, v6, v2
	and.16b	v6, v6, v3
	and.16b	v5, v5, v3
	add.2d	v4, v5, v4
	add.2d	v0, v6, v0
	subs	x8, x8, #4
	b.ne	LBB1_1
	add.2d	v0, v4, v0
	addp.2d	d0, v0
	fmov	x0, d0
	ret
	.loh AdrpLdr	Lloh2, Lloh3
	.loh AdrpAdrp	Lloh0, Lloh2
	.loh AdrpLdr	Lloh0, Lloh1

	.globl	_w30_loop8
	.p2align	2
_w30_loop8:
	mov	x9, #0
	mov	x8, #0
	mov	w10, #1000
LBB2_1:
	add	x11, x0, x9, lsr #3
	ldr	w12, [x11]
	ldrb	w11, [x11, #4]
	orr	x11, x12, x11, lsl #32
	and	x12, x9, #0x6
	lsr	x11, x11, x12
	and	x11, x11, #0x3fffffff
	add	x8, x11, x8
	add	x9, x9, #30
	subs	x10, x10, #1
	b.ne	LBB2_1
	mov	x0, x8
	ret

	.globl	_w30_wide
_w30_wide = _w30_flat8
.subsections_via_symbols
