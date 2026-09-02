	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind
	.p2align	2
__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind:
LBB0_1:
	b	LBB0_1

	.globl	_w6_flat8
	.p2align	2
_w6_flat8:
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	add	x12, x0, #1
	mov	w13, #1000
LBB1_1:
	ldur	x14, [x12, #-1]
	ldr	x15, [x12]
	and	x16, x14, #0x3f
	ubfx	x14, x14, #6, #6
	ldur	x17, [x12, #1]
	ubfx	x15, x15, #4, #6
	ubfx	x17, x17, #2, #6
	add	x8, x16, x8
	add	x9, x14, x9
	add	x10, x15, x10
	add	x11, x17, x11
	add	x12, x12, #3
	subs	x13, x13, #4
	b.ne	LBB1_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret

	.globl	_w6_loop8
	.p2align	2
_w6_loop8:
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	add	x12, x0, #2
	mov	w13, #1000
LBB2_1:
	ldurb	w14, [x12, #-2]
	ldurb	w15, [x12, #-1]
	ldrb	w16, [x12], #3
	orr	w17, w14, w15, lsl #8
	orr	w15, w15, w16, lsl #8
	and	x14, x14, #0x3f
	ubfx	x17, x17, #6, #6
	ubfx	x15, x15, #4, #6
	add	x8, x8, x14
	add	x9, x17, x9
	add	x10, x15, x10
	add	x11, x11, x16, lsr #2
	subs	x13, x13, #4
	b.ne	LBB2_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret

	.globl	_w6_wide
_w6_wide = _w6_flat8
.subsections_via_symbols
