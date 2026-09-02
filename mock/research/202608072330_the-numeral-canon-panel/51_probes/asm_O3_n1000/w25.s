	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_w25_hand
	.p2align	2
_w25_hand:
	.cfi_startproc
	mov	x9, #0
	mov	x8, #0
	mov	w10, #1000
LBB0_1:
	lsr	x11, x9, #3
	ldr	w11, [x0, x11]
	and	x12, x9, #0x7
	lsr	x11, x11, x12
	and	x11, x11, #0x1ffffff
	add	x8, x11, x8
	add	x9, x9, #25
	subs	x10, x10, #1
	b.ne	LBB0_1
	mov	x0, x8
	ret
	.cfi_endproc

	.globl	_w25_typed
_w25_typed = _w25_hand
.subsections_via_symbols
