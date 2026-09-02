	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind
	.p2align	2
__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind:
LBB0_1:
	b	LBB0_1

	.globl	_a2_sat_13_satur
	.p2align	2
_a2_sat_13_satur:
	mov	w8, #8191
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_a2_sat_13_wrap
	.p2align	2
_a2_sat_13_wrap:
	and	x0, x0, #0x1fff
	ret

	.globl	_a_sat_47
	.p2align	2
_a_sat_47:
	and	x0, x0, #0x7fffffffffff
	ret

	.globl	_b_unsat
	.p2align	2
_b_unsat:
	ldr	w8, [x0, #8]
	mov	x9, #-1
	lsl	x10, x9, x8
	cmp	w8, #63
	csinv	x8, x9, x10, hi
	ldr	x9, [x0]
	and	x0, x8, x9
	ret

	.globl	_c_dispatch
	.p2align	2
_c_dispatch:
	cmp	w1, #0
	mov	w8, #8191
	mov	x9, #140737488355327
	csel	x8, x9, x8, ne
	and	x0, x8, x0
	ret

	.globl	_a_sat_13
_a_sat_13 = _a2_sat_13_wrap
.subsections_via_symbols
