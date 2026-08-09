	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind
	.p2align	2
__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind:
LBB0_1:
	b	LBB0_1

	.globl	_group_promising
	.p2align	2
_group_promising:
	cbz	x1, LBB1_6
	mov	w10, #0
	mov	w8, #0
	mov	w11, #0
	mov	x9, #0
LBB1_2:
	mov	x12, x10
LBB1_3:
	ldr	w13, [x0, x9, lsl #2]
	tbz	w11, #0, LBB1_5
	add	w14, w13, w10
	cmp	w12, w13
	csel	w12, w14, w12, hs
	cset	w11, hs
	cinc	x9, x9, hs
	csel	w10, w14, w10, hs
	cmp	x9, x1
	b.lo	LBB1_3
	b	LBB1_7
LBB1_5:
	add	w8, w8, #1
	add	x9, x9, #1
	mov	w11, #1
	mov	x10, x13
	cmp	x9, x1
	b.lo	LBB1_2
	b	LBB1_7
LBB1_6:
	mov	w8, #0
LBB1_7:
	mov	x0, x8
	ret

	.globl	_group_silent
	.p2align	2
_group_silent:
	cbz	x1, LBB2_5
	mov	w9, #0
	mov	w8, #0
	mov	w12, #0
	mov	x10, #0
	mov	w11, #0
LBB2_2:
	ldr	w13, [x0, x10, lsl #2]
	tbz	w12, #0, LBB2_4
	add	w14, w13, w9
	cmp	w11, w13
	csel	w11, w14, w11, hs
	cset	w12, hs
	cinc	x10, x10, hs
	csel	w9, w14, w9, hs
	cmp	x10, x1
	b.lo	LBB2_2
	b	LBB2_6
LBB2_4:
	cmp	w13, #0
	csel	w11, wzr, w11, eq
	cset	w12, eq
	cinc	w8, w8, eq
	csel	w9, wzr, w9, eq
	add	x10, x10, #1
	cmp	x10, x1
	b.lo	LBB2_2
	b	LBB2_6
LBB2_5:
	mov	w8, #0
LBB2_6:
	mov	x0, x8
	ret

.subsections_via_symbols
