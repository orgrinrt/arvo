	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind
	.p2align	2
__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind:
LBB0_1:
	b	LBB0_1

	.globl	_w40_flat8
	.p2align	2
_w40_flat8:
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	add	x12, x0, #9
	mov	w13, #1000
LBB1_1:
	ldur	w14, [x12, #-9]
	ldur	w15, [x12, #-4]
	ldur	w16, [x12, #1]
	ldur	w17, [x12, #6]
	ldurb	w0, [x12, #-5]
	ldrb	w1, [x12]
	ldrb	w2, [x12, #5]
	orr	x14, x14, x0, lsl #32
	ldrb	w0, [x12, #10]
	orr	x15, x15, x1, lsl #32
	orr	x16, x16, x2, lsl #32
	orr	x17, x17, x0, lsl #32
	add	x8, x14, x8
	add	x9, x15, x9
	add	x10, x16, x10
	add	x11, x17, x11
	add	x12, x12, #20
	subs	x13, x13, #4
	b.ne	LBB1_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret

	.globl	_w40_loop8
	.p2align	2
_w40_loop8:
	mov	x8, x0
	mov	x0, #0
	add	x8, x8, #2
	mov	w9, #1000
LBB2_1:
	ldur	w10, [x8, #-2]
	ldrb	w11, [x8, #2]
	orr	x10, x10, x11, lsl #32
	add	x0, x10, x0
	add	x8, x8, #5
	subs	x9, x9, #1
	b.ne	LBB2_1
	ret

	.globl	_w40_wide
	.p2align	2
_w40_wide:
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	add	x12, x0, #10
	mov	w13, #1000
LBB3_1:
	ldur	x14, [x12, #-10]
	ldur	x15, [x12, #-5]
	ldr	x16, [x12]
	and	x14, x14, #0xffffffffff
	and	x15, x15, #0xffffffffff
	ldur	x17, [x12, #5]
	and	x16, x16, #0xffffffffff
	and	x17, x17, #0xffffffffff
	add	x8, x14, x8
	add	x9, x15, x9
	add	x10, x16, x10
	add	x11, x17, x11
	add	x12, x12, #20
	subs	x13, x13, #4
	b.ne	LBB3_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret

.subsections_via_symbols
