	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind
	.p2align	2
__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind:
LBB0_1:
	b	LBB0_1

	.globl	_w5_flat8
	.p2align	2
_w5_flat8:
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	mov	w12, #15
	mov	w13, #1000
	mov	w14, #5
LBB1_1:
	sub	x15, x12, #15
	sub	x16, x12, #10
	sub	x17, x12, #5
	lsr	x1, x15, #3
	lsr	x2, x16, #3
	lsr	x3, x17, #3
	lsr	x4, x12, #3
	ldr	x1, [x0, x1]
	ldr	x2, [x0, x2]
	ldr	x3, [x0, x3]
	ldr	x4, [x0, x4]
	and	x15, x15, #0x4
	and	x16, x16, x14
	and	x17, x17, #0x6
	and	x5, x12, #0x7
	lsr	x15, x1, x15
	lsr	x16, x2, x16
	lsr	x17, x3, x17
	lsr	x1, x4, x5
	and	x15, x15, #0x1f
	and	x16, x16, #0x1f
	and	x17, x17, #0x1f
	and	x1, x1, #0x1f
	add	x8, x15, x8
	add	x9, x16, x9
	add	x10, x17, x10
	add	x11, x1, x11
	add	x12, x12, #20
	subs	x13, x13, #4
	b.ne	LBB1_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret

	.globl	_w5_loop8
	.p2align	2
_w5_loop8:
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	mov	w12, #15
	mov	w13, #1000
	mov	w14, #5
LBB2_1:
	sub	x15, x12, #15
	sub	x16, x12, #10
	sub	x17, x12, #5
	lsr	x1, x15, #3
	lsr	x2, x16, #3
	lsr	x3, x17, #3
	lsr	x4, x12, #3
	ldrh	w1, [x0, x1]
	ldrh	w2, [x0, x2]
	ldrh	w3, [x0, x3]
	ldrh	w4, [x0, x4]
	and	x15, x15, #0x4
	and	x16, x16, x14
	and	x17, x17, #0x6
	and	x5, x12, #0x7
	lsr	x15, x1, x15
	lsr	x16, x2, x16
	lsr	x17, x3, x17
	lsr	x1, x4, x5
	and	x15, x15, #0x1f
	and	x16, x16, #0x1f
	and	x17, x17, #0x1f
	and	x1, x1, #0x1f
	add	x8, x15, x8
	add	x9, x16, x9
	add	x10, x17, x10
	add	x11, x1, x11
	add	x12, x12, #20
	subs	x13, x13, #4
	b.ne	LBB2_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret

	.globl	_w5_wide
_w5_wide = _w5_flat8
.subsections_via_symbols
