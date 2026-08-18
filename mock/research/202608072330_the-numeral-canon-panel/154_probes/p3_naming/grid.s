	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind
	.p2align	2
__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind:
LBB0_1:
	b	LBB0_1

	.globl	_d_clamp_11
	.p2align	2
_d_clamp_11:
	mov	w8, #2047
	cmp	x0, #2047
	csel	x0, x0, x8, lo
	ret

	.globl	_d_clamp_13
	.p2align	2
_d_clamp_13:
	mov	w8, #8191
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_d_clamp_14
	.p2align	2
_d_clamp_14:
	mov	w8, #16383
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_d_clamp_23
	.p2align	2
_d_clamp_23:
	mov	w8, #8388607
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_d_clamp_27
	.p2align	2
_d_clamp_27:
	mov	w8, #134217727
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_d_clamp_3
	.p2align	2
_d_clamp_3:
	mov	w8, #7
	cmp	x0, #7
	csel	x0, x0, x8, lo
	ret

	.globl	_d_clamp_31
	.p2align	2
_d_clamp_31:
	mov	w8, #2147483647
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_d_clamp_47
	.p2align	2
_d_clamp_47:
	mov	x8, #140737488355327
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_d_clamp_61
	.p2align	2
_d_clamp_61:
	mov	x8, #2305843009213693951
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_d_clamp_7
	.p2align	2
_d_clamp_7:
	mov	w8, #127
	cmp	x0, #127
	csel	x0, x0, x8, lo
	ret

	.globl	_d_wrap_11
	.p2align	2
_d_wrap_11:
	and	x0, x0, #0x7ff
	ret

	.globl	_d_wrap_13
	.p2align	2
_d_wrap_13:
	and	x0, x0, #0x1fff
	ret

	.globl	_d_wrap_14
	.p2align	2
_d_wrap_14:
	and	x0, x0, #0x3fff
	ret

	.globl	_d_wrap_23
	.p2align	2
_d_wrap_23:
	and	x0, x0, #0x7fffff
	ret

	.globl	_d_wrap_27
	.p2align	2
_d_wrap_27:
	and	x0, x0, #0x7ffffff
	ret

	.globl	_d_wrap_3
	.p2align	2
_d_wrap_3:
	and	x0, x0, #0x7
	ret

	.globl	_d_wrap_31
	.p2align	2
_d_wrap_31:
	and	x0, x0, #0x7fffffff
	ret

	.globl	_d_wrap_47
	.p2align	2
_d_wrap_47:
	and	x0, x0, #0x7fffffffffff
	ret

	.globl	_d_wrap_61
	.p2align	2
_d_wrap_61:
	and	x0, x0, #0x1fffffffffffffff
	ret

	.globl	_d_wrap_7
	.p2align	2
_d_wrap_7:
	and	x0, x0, #0x7f
	ret

	.globl	_s_alias_0
_s_alias_0 = _d_wrap_13
	.globl	_s_alias_1
_s_alias_1 = _d_wrap_13
	.globl	_s_alias_10
_s_alias_10 = _d_wrap_13
	.globl	_s_alias_11
_s_alias_11 = _d_wrap_13
	.globl	_s_alias_12
_s_alias_12 = _d_wrap_13
	.globl	_s_alias_13
_s_alias_13 = _d_wrap_13
	.globl	_s_alias_14
_s_alias_14 = _d_wrap_13
	.globl	_s_alias_15
_s_alias_15 = _d_wrap_13
	.globl	_s_alias_16
_s_alias_16 = _d_wrap_13
	.globl	_s_alias_17
_s_alias_17 = _d_wrap_13
	.globl	_s_alias_18
_s_alias_18 = _d_wrap_13
	.globl	_s_alias_19
_s_alias_19 = _d_wrap_13
	.globl	_s_alias_2
_s_alias_2 = _d_wrap_13
	.globl	_s_alias_3
_s_alias_3 = _d_wrap_13
	.globl	_s_alias_4
_s_alias_4 = _d_wrap_13
	.globl	_s_alias_5
_s_alias_5 = _d_wrap_13
	.globl	_s_alias_6
_s_alias_6 = _d_wrap_13
	.globl	_s_alias_7
_s_alias_7 = _d_wrap_13
	.globl	_s_alias_8
_s_alias_8 = _d_wrap_13
	.globl	_s_alias_9
_s_alias_9 = _d_wrap_13
.subsections_via_symbols
