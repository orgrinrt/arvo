	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind
	.p2align	2
__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind:
LBB0_1:
	b	LBB0_1

	.globl	_chain13_eager
	.p2align	2
_chain13_eager:
	add	w8, w1, w0
	add	w9, w2, w3
	add	w8, w8, w9
	and	w0, w8, #0x1fff
	ret

	.globl	_cmp13
	.p2align	2
_cmp13:
	add	w8, w1, w0
	and	w8, w8, #0x1fff
	cmp	w8, w2
	cset	w0, lo
	ret

	.globl	_exact16
	.p2align	2
_exact16:
	add	w8, w1, w0
	and	w0, w8, #0xffff
	ret

	.globl	_exact64
	.p2align	2
_exact64:
	add	x0, x1, x0
	ret

	.globl	_exact8
	.p2align	2
_exact8:
	add	w8, w1, w0
	and	w0, w8, #0xff
	ret

	.globl	_hr13
	.p2align	2
_hr13:
	add	w8, w1, w0
	and	w0, w8, #0x1fff
	ret

	.globl	_hr60
	.p2align	2
_hr60:
	add	x8, x2, x0
	and	x0, x8, #0xfffffffffffffff
	mov	x1, #0
	ret

	.globl	_low13
	.p2align	2
_low13:
	add	w8, w1, w0
	and	w0, w8, #0x7
	ret

	.globl	_mix13_eager
	.p2align	2
_mix13_eager:
	add	w8, w1, w0
	neg	w9, w0
	madd	w8, w8, w2, w9
	and	w0, w8, #0x1fff
	ret

	.globl	_sub13
	.p2align	2
_sub13:
	add	w8, w1, w0
	and	w0, w8, #0x1fff
	ret

	.globl	_sub60
	.p2align	2
_sub60:
	add	x8, x1, x0
	and	x0, x8, #0xfffffffffffffff
	ret

	.globl	_mix13_lazy
_mix13_lazy = _mix13_eager
	.globl	_chain13_lazy
_chain13_lazy = _chain13_eager
.subsections_via_symbols
