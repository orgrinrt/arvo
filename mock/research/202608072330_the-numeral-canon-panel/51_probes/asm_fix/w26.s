	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind
	.p2align	2
__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind:
LBB0_1:
	b	LBB0_1

	.globl	_w26_flat8
	.p2align	2
_w26_flat8:
	mov	x8, #0
	mov	x9, #0
	mov	x10, #0
	mov	x11, #0
	add	x12, x0, #6
	mov	w13, #1000
LBB1_1:
	ldur	x14, [x12, #-6]
	ldur	x15, [x12, #-3]
	ldr	x16, [x12]
	and	x14, x14, #0x3ffffff
	ubfx	x15, x15, #2, #26
	ldur	x17, [x12, #3]
	ubfx	x16, x16, #4, #26
	lsr	w17, w17, #6
	add	x8, x14, x8
	add	x9, x15, x9
	add	x10, x16, x10
	add	x11, x17, x11
	add	x12, x12, #13
	subs	x13, x13, #4
	b.ne	LBB1_1
	add	x8, x9, x8
	add	x9, x11, x10
	add	x0, x9, x8
	ret

	.globl	_w26_loop8
	.p2align	2
_w26_loop8:
	mov	x9, #0
	mov	x8, #0
	mov	w10, #1000
LBB2_1:
	lsr	x11, x9, #3
	ldr	w11, [x0, x11]
	and	x12, x9, #0x6
	lsr	x11, x11, x12
	and	x11, x11, #0x3ffffff
	add	x8, x11, x8
	add	x9, x9, #26
	subs	x10, x10, #1
	b.ne	LBB2_1
	mov	x0, x8
	ret

	.globl	_w26_wide
_w26_wide = _w26_flat8
.subsections_via_symbols
