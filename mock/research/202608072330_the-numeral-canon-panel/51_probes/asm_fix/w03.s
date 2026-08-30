	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind
	.p2align	2
__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind:
LBB0_1:
	b	LBB0_1

	.globl	_w3_flat8
	.p2align	2
_w3_flat8:
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	mov	w12, #9
	mov	w13, #5
LBB1_1:
	sub	x14, x12, #9
	sub	x15, x12, #3
	lsr	x16, x14, #3
	lsr	x17, x15, #3
	lsr	x1, x12, #3
	ldr	x16, [x0, x16]
	ldr	x17, [x0, x17]
	ldr	x1, [x0, x1]
	and	x14, x14, #0x4
	orr	x2, x14, #0x3
	and	x15, x15, #0x6
	and	x3, x12, x13
	lsr	x14, x16, x14
	lsr	x16, x16, x2
	lsr	x15, x17, x15
	lsr	x17, x1, x3
	and	x14, x14, #0x7
	and	x16, x16, #0x7
	and	x15, x15, #0x7
	and	x17, x17, #0x7
	add	x8, x14, x8
	add	x9, x16, x9
	add	x10, x15, x10
	add	x11, x17, x11
	add	x12, x12, #12
	cmp	x12, #3009
	b.ne	LBB1_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret

	.globl	_w3_loop8
	.p2align	2
_w3_loop8:
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	mov	w12, #9
	mov	w13, #5
LBB2_1:
	sub	x14, x12, #9
	sub	x15, x12, #3
	lsr	x16, x14, #3
	lsr	x17, x15, #3
	lsr	x1, x12, #3
	ldrh	w16, [x0, x16]
	ldrh	w17, [x0, x17]
	ldrh	w1, [x0, x1]
	and	x14, x14, #0x4
	orr	x2, x14, #0x3
	and	x15, x15, #0x6
	and	x3, x12, x13
	lsr	x14, x16, x14
	lsr	x16, x16, x2
	lsr	x15, x17, x15
	lsr	x17, x1, x3
	and	x14, x14, #0x7
	and	x16, x16, #0x7
	and	x15, x15, #0x7
	and	x17, x17, #0x7
	add	x8, x14, x8
	add	x9, x16, x9
	add	x10, x15, x10
	add	x11, x17, x11
	add	x12, x12, #12
	cmp	x12, #3009
	b.ne	LBB2_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret

	.globl	_w3_wide
_w3_wide = _w3_flat8
.subsections_via_symbols
