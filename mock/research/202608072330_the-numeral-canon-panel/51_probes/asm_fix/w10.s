	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind
	.p2align	2
__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind:
LBB0_1:
	b	LBB0_1

	.globl	_w10_flat8
	.p2align	2
_w10_flat8:
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	add	x12, x0, #3
	mov	w13, #1000
LBB1_1:
	ldur	x14, [x12, #-3]
	ldur	x15, [x12, #-2]
	ldur	x16, [x12, #-1]
	and	x14, x14, #0x3ff
	ubfx	x15, x15, #2, #10
	ldr	x17, [x12], #5
	ubfx	x16, x16, #4, #10
	ubfx	x17, x17, #6, #10
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

	.globl	_w10_loop8
	.p2align	2
_w10_loop8:
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	add	x12, x0, #2
	mov	w13, #1000
LBB2_1:
	ldurb	w14, [x12, #-1]
	ldrb	w15, [x12]
	ldrb	w16, [x12, #1]
	ldrb	w17, [x12, #2]
	orr	w0, w14, w15, lsl #8
	orr	w15, w15, w16, lsl #8
	orr	x16, x16, x17, lsl #8
	ldurb	w17, [x12, #-2]
	bfi	x17, x14, #8, #2
	ubfx	x14, x0, #2, #10
	ubfx	x15, x15, #4, #10
	add	x8, x17, x8
	add	x9, x14, x9
	add	x10, x15, x10
	add	x11, x11, x16, lsr #6
	add	x12, x12, #5
	subs	x13, x13, #4
	b.ne	LBB2_1
	add	x8, x9, x8
	add	x8, x10, x8
	add	x0, x11, x8
	ret

	.globl	_w10_wide
_w10_wide = _w10_flat8
.subsections_via_symbols
