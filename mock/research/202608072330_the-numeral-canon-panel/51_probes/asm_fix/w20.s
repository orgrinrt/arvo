	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind
	.p2align	2
__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind:
LBB0_1:
	b	LBB0_1

	.globl	_w20_flat8
	.p2align	2
_w20_flat8:
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	add	x12, x0, #7
	mov	w13, #1000
LBB1_1:
	ldur	x14, [x12, #-7]
	ldur	x15, [x12, #-5]
	ldur	x16, [x12, #-2]
	and	x14, x14, #0xfffff
	ubfx	x15, x15, #4, #20
	ldr	x17, [x12], #10
	and	x16, x16, #0xfffff
	ubfx	x17, x17, #4, #20
	add	x8, x14, x8
	add	x9, x15, x9
	add	x10, x16, x10
	add	x11, x17, x11
	subs	x13, x13, #4
	b.ne	LBB1_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret

	.globl	_w20_loop8
	.p2align	2
_w20_loop8:
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	add	x12, x0, #4
	mov	w13, #1000
LBB2_1:
	ldurb	w14, [x12, #-2]
	ldrb	w15, [x12, #3]
	ldurb	w16, [x12, #-1]
	ldrb	w17, [x12, #4]
	ldurh	w0, [x12, #-4]
	orr	x16, x14, x16, lsl #8
	ldurh	w1, [x12, #1]
	orr	x17, x15, x17, lsl #8
	ldrb	w2, [x12]
	orr	x16, x16, x2, lsl #16
	ldrb	w2, [x12, #5]
	orr	x17, x17, x2, lsl #16
	bfi	x0, x14, #16, #4
	bfi	x1, x15, #16, #4
	add	x8, x0, x8
	add	x9, x9, x16, lsr #4
	add	x10, x1, x10
	add	x11, x11, x17, lsr #4
	add	x12, x12, #10
	subs	x13, x13, #4
	b.ne	LBB2_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret

	.globl	_w20_wide
_w20_wide = _w20_flat8
.subsections_via_symbols
