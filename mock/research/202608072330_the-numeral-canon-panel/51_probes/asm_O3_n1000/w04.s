	.build_version macos, 11, 0
	.section	__TEXT,__literal16,16byte_literals
	.p2align	4, 0x0
lCPI0_0:
	.quad	0
	.quad	1
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_w4_hand
	.p2align	2
_w4_hand:
	.cfi_startproc
Lloh0:
	adrp	x8, lCPI0_0@PAGE
Lloh1:
	ldr	q0, [x8, lCPI0_0@PAGEOFF]
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
LBB0_1:
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
	b.ne	LBB0_1
	add.2d	v0, v6, v1
	add.2d	v0, v7, v0
	add.2d	v0, v16, v0
	addp.2d	d0, v0
	fmov	x0, d0
	ret
	.loh AdrpLdr	Lloh0, Lloh1
	.cfi_endproc

	.globl	_w4_typed
_w4_typed = _w4_hand
.subsections_via_symbols
