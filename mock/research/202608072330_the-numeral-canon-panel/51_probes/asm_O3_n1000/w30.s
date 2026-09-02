	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_w30_hand
	.p2align	2
_w30_hand:
	.cfi_startproc
	mov	x9, #0
	mov	x8, #0
	mov	w10, #1000
LBB0_1:
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
	b.ne	LBB0_1
	mov	x0, x8
	ret
	.cfi_endproc

	.globl	_w30_typed
_w30_typed = _w30_hand
.subsections_via_symbols
