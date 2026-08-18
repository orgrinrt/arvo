	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind
	.p2align	2
__RNvCske4UNIzLImn_7___rustc17rust_begin_unwind:
LBB0_1:
	b	LBB0_1

	.globl	_g_clamp_1
	.p2align	2
_g_clamp_1:
	cmp	x0, #0
	cset	w0, ne
	ret

	.globl	_g_clamp_10
	.p2align	2
_g_clamp_10:
	mov	w8, #1023
	cmp	x0, #1023
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_11
	.p2align	2
_g_clamp_11:
	mov	w8, #2047
	cmp	x0, #2047
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_12
	.p2align	2
_g_clamp_12:
	mov	w8, #4095
	cmp	x0, #4095
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_13
	.p2align	2
_g_clamp_13:
	mov	w8, #8191
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_14
	.p2align	2
_g_clamp_14:
	mov	w8, #16383
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_15
	.p2align	2
_g_clamp_15:
	mov	w8, #32767
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_16
	.p2align	2
_g_clamp_16:
	mov	w8, #65535
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_17
	.p2align	2
_g_clamp_17:
	mov	w8, #131071
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_18
	.p2align	2
_g_clamp_18:
	mov	w8, #262143
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_19
	.p2align	2
_g_clamp_19:
	mov	w8, #524287
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_2
	.p2align	2
_g_clamp_2:
	mov	w8, #3
	cmp	x0, #3
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_20
	.p2align	2
_g_clamp_20:
	mov	w8, #1048575
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_21
	.p2align	2
_g_clamp_21:
	mov	w8, #2097151
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_22
	.p2align	2
_g_clamp_22:
	mov	w8, #4194303
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_23
	.p2align	2
_g_clamp_23:
	mov	w8, #8388607
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_24
	.p2align	2
_g_clamp_24:
	mov	w8, #16777215
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_25
	.p2align	2
_g_clamp_25:
	mov	w8, #33554431
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_26
	.p2align	2
_g_clamp_26:
	mov	w8, #67108863
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_27
	.p2align	2
_g_clamp_27:
	mov	w8, #134217727
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_28
	.p2align	2
_g_clamp_28:
	mov	w8, #268435455
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_29
	.p2align	2
_g_clamp_29:
	mov	w8, #536870911
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_3
	.p2align	2
_g_clamp_3:
	mov	w8, #7
	cmp	x0, #7
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_30
	.p2align	2
_g_clamp_30:
	mov	w8, #1073741823
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_31
	.p2align	2
_g_clamp_31:
	mov	w8, #2147483647
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_32
	.p2align	2
_g_clamp_32:
	mov	w8, #-1
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_33
	.p2align	2
_g_clamp_33:
	mov	x8, #8589934591
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_34
	.p2align	2
_g_clamp_34:
	mov	x8, #17179869183
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_35
	.p2align	2
_g_clamp_35:
	mov	x8, #34359738367
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_36
	.p2align	2
_g_clamp_36:
	mov	x8, #68719476735
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_37
	.p2align	2
_g_clamp_37:
	mov	x8, #137438953471
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_38
	.p2align	2
_g_clamp_38:
	mov	x8, #274877906943
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_39
	.p2align	2
_g_clamp_39:
	mov	x8, #549755813887
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_4
	.p2align	2
_g_clamp_4:
	mov	w8, #15
	cmp	x0, #15
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_40
	.p2align	2
_g_clamp_40:
	mov	x8, #1099511627775
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_41
	.p2align	2
_g_clamp_41:
	mov	x8, #2199023255551
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_42
	.p2align	2
_g_clamp_42:
	mov	x8, #4398046511103
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_43
	.p2align	2
_g_clamp_43:
	mov	x8, #8796093022207
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_44
	.p2align	2
_g_clamp_44:
	mov	x8, #17592186044415
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_45
	.p2align	2
_g_clamp_45:
	mov	x8, #35184372088831
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_46
	.p2align	2
_g_clamp_46:
	mov	x8, #70368744177663
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_47
	.p2align	2
_g_clamp_47:
	mov	x8, #140737488355327
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_48
	.p2align	2
_g_clamp_48:
	mov	x8, #281474976710655
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_49
	.p2align	2
_g_clamp_49:
	mov	x8, #562949953421311
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_5
	.p2align	2
_g_clamp_5:
	mov	w8, #31
	cmp	x0, #31
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_50
	.p2align	2
_g_clamp_50:
	mov	x8, #1125899906842623
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_51
	.p2align	2
_g_clamp_51:
	mov	x8, #2251799813685247
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_52
	.p2align	2
_g_clamp_52:
	mov	x8, #4503599627370495
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_53
	.p2align	2
_g_clamp_53:
	mov	x8, #9007199254740991
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_54
	.p2align	2
_g_clamp_54:
	mov	x8, #18014398509481983
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_55
	.p2align	2
_g_clamp_55:
	mov	x8, #36028797018963967
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_56
	.p2align	2
_g_clamp_56:
	mov	x8, #72057594037927935
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_57
	.p2align	2
_g_clamp_57:
	mov	x8, #144115188075855871
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_58
	.p2align	2
_g_clamp_58:
	mov	x8, #288230376151711743
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_59
	.p2align	2
_g_clamp_59:
	mov	x8, #576460752303423487
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_6
	.p2align	2
_g_clamp_6:
	mov	w8, #63
	cmp	x0, #63
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_60
	.p2align	2
_g_clamp_60:
	mov	x8, #1152921504606846975
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_61
	.p2align	2
_g_clamp_61:
	mov	x8, #2305843009213693951
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_62
	.p2align	2
_g_clamp_62:
	mov	x8, #4611686018427387903
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_63
	.p2align	2
_g_clamp_63:
	mov	x8, #9223372036854775807
	cmp	x0, x8
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_64
	.p2align	2
_g_clamp_64:
	ret

	.globl	_g_clamp_7
	.p2align	2
_g_clamp_7:
	mov	w8, #127
	cmp	x0, #127
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_8
	.p2align	2
_g_clamp_8:
	mov	w8, #255
	cmp	x0, #255
	csel	x0, x0, x8, lo
	ret

	.globl	_g_clamp_9
	.p2align	2
_g_clamp_9:
	mov	w8, #511
	cmp	x0, #511
	csel	x0, x0, x8, lo
	ret

	.globl	_g_wrap_1
	.p2align	2
_g_wrap_1:
	and	x0, x0, #0x1
	ret

	.globl	_g_wrap_10
	.p2align	2
_g_wrap_10:
	and	x0, x0, #0x3ff
	ret

	.globl	_g_wrap_11
	.p2align	2
_g_wrap_11:
	and	x0, x0, #0x7ff
	ret

	.globl	_g_wrap_12
	.p2align	2
_g_wrap_12:
	and	x0, x0, #0xfff
	ret

	.globl	_g_wrap_13
	.p2align	2
_g_wrap_13:
	and	x0, x0, #0x1fff
	ret

	.globl	_g_wrap_14
	.p2align	2
_g_wrap_14:
	and	x0, x0, #0x3fff
	ret

	.globl	_g_wrap_15
	.p2align	2
_g_wrap_15:
	and	x0, x0, #0x7fff
	ret

	.globl	_g_wrap_16
	.p2align	2
_g_wrap_16:
	and	x0, x0, #0xffff
	ret

	.globl	_g_wrap_17
	.p2align	2
_g_wrap_17:
	and	x0, x0, #0x1ffff
	ret

	.globl	_g_wrap_18
	.p2align	2
_g_wrap_18:
	and	x0, x0, #0x3ffff
	ret

	.globl	_g_wrap_19
	.p2align	2
_g_wrap_19:
	and	x0, x0, #0x7ffff
	ret

	.globl	_g_wrap_2
	.p2align	2
_g_wrap_2:
	and	x0, x0, #0x3
	ret

	.globl	_g_wrap_20
	.p2align	2
_g_wrap_20:
	and	x0, x0, #0xfffff
	ret

	.globl	_g_wrap_21
	.p2align	2
_g_wrap_21:
	and	x0, x0, #0x1fffff
	ret

	.globl	_g_wrap_22
	.p2align	2
_g_wrap_22:
	and	x0, x0, #0x3fffff
	ret

	.globl	_g_wrap_23
	.p2align	2
_g_wrap_23:
	and	x0, x0, #0x7fffff
	ret

	.globl	_g_wrap_24
	.p2align	2
_g_wrap_24:
	and	x0, x0, #0xffffff
	ret

	.globl	_g_wrap_25
	.p2align	2
_g_wrap_25:
	and	x0, x0, #0x1ffffff
	ret

	.globl	_g_wrap_26
	.p2align	2
_g_wrap_26:
	and	x0, x0, #0x3ffffff
	ret

	.globl	_g_wrap_27
	.p2align	2
_g_wrap_27:
	and	x0, x0, #0x7ffffff
	ret

	.globl	_g_wrap_28
	.p2align	2
_g_wrap_28:
	and	x0, x0, #0xfffffff
	ret

	.globl	_g_wrap_29
	.p2align	2
_g_wrap_29:
	and	x0, x0, #0x1fffffff
	ret

	.globl	_g_wrap_3
	.p2align	2
_g_wrap_3:
	and	x0, x0, #0x7
	ret

	.globl	_g_wrap_30
	.p2align	2
_g_wrap_30:
	and	x0, x0, #0x3fffffff
	ret

	.globl	_g_wrap_31
	.p2align	2
_g_wrap_31:
	and	x0, x0, #0x7fffffff
	ret

	.globl	_g_wrap_32
	.p2align	2
_g_wrap_32:
	mov	w0, w0
	ret

	.globl	_g_wrap_33
	.p2align	2
_g_wrap_33:
	and	x0, x0, #0x1ffffffff
	ret

	.globl	_g_wrap_34
	.p2align	2
_g_wrap_34:
	and	x0, x0, #0x3ffffffff
	ret

	.globl	_g_wrap_35
	.p2align	2
_g_wrap_35:
	and	x0, x0, #0x7ffffffff
	ret

	.globl	_g_wrap_36
	.p2align	2
_g_wrap_36:
	and	x0, x0, #0xfffffffff
	ret

	.globl	_g_wrap_37
	.p2align	2
_g_wrap_37:
	and	x0, x0, #0x1fffffffff
	ret

	.globl	_g_wrap_38
	.p2align	2
_g_wrap_38:
	and	x0, x0, #0x3fffffffff
	ret

	.globl	_g_wrap_39
	.p2align	2
_g_wrap_39:
	and	x0, x0, #0x7fffffffff
	ret

	.globl	_g_wrap_4
	.p2align	2
_g_wrap_4:
	and	x0, x0, #0xf
	ret

	.globl	_g_wrap_40
	.p2align	2
_g_wrap_40:
	and	x0, x0, #0xffffffffff
	ret

	.globl	_g_wrap_41
	.p2align	2
_g_wrap_41:
	and	x0, x0, #0x1ffffffffff
	ret

	.globl	_g_wrap_42
	.p2align	2
_g_wrap_42:
	and	x0, x0, #0x3ffffffffff
	ret

	.globl	_g_wrap_43
	.p2align	2
_g_wrap_43:
	and	x0, x0, #0x7ffffffffff
	ret

	.globl	_g_wrap_44
	.p2align	2
_g_wrap_44:
	and	x0, x0, #0xfffffffffff
	ret

	.globl	_g_wrap_45
	.p2align	2
_g_wrap_45:
	and	x0, x0, #0x1fffffffffff
	ret

	.globl	_g_wrap_46
	.p2align	2
_g_wrap_46:
	and	x0, x0, #0x3fffffffffff
	ret

	.globl	_g_wrap_47
	.p2align	2
_g_wrap_47:
	and	x0, x0, #0x7fffffffffff
	ret

	.globl	_g_wrap_48
	.p2align	2
_g_wrap_48:
	and	x0, x0, #0xffffffffffff
	ret

	.globl	_g_wrap_49
	.p2align	2
_g_wrap_49:
	and	x0, x0, #0x1ffffffffffff
	ret

	.globl	_g_wrap_5
	.p2align	2
_g_wrap_5:
	and	x0, x0, #0x1f
	ret

	.globl	_g_wrap_50
	.p2align	2
_g_wrap_50:
	and	x0, x0, #0x3ffffffffffff
	ret

	.globl	_g_wrap_51
	.p2align	2
_g_wrap_51:
	and	x0, x0, #0x7ffffffffffff
	ret

	.globl	_g_wrap_52
	.p2align	2
_g_wrap_52:
	and	x0, x0, #0xfffffffffffff
	ret

	.globl	_g_wrap_53
	.p2align	2
_g_wrap_53:
	and	x0, x0, #0x1fffffffffffff
	ret

	.globl	_g_wrap_54
	.p2align	2
_g_wrap_54:
	and	x0, x0, #0x3fffffffffffff
	ret

	.globl	_g_wrap_55
	.p2align	2
_g_wrap_55:
	and	x0, x0, #0x7fffffffffffff
	ret

	.globl	_g_wrap_56
	.p2align	2
_g_wrap_56:
	and	x0, x0, #0xffffffffffffff
	ret

	.globl	_g_wrap_57
	.p2align	2
_g_wrap_57:
	and	x0, x0, #0x1ffffffffffffff
	ret

	.globl	_g_wrap_58
	.p2align	2
_g_wrap_58:
	and	x0, x0, #0x3ffffffffffffff
	ret

	.globl	_g_wrap_59
	.p2align	2
_g_wrap_59:
	and	x0, x0, #0x7ffffffffffffff
	ret

	.globl	_g_wrap_6
	.p2align	2
_g_wrap_6:
	and	x0, x0, #0x3f
	ret

	.globl	_g_wrap_60
	.p2align	2
_g_wrap_60:
	and	x0, x0, #0xfffffffffffffff
	ret

	.globl	_g_wrap_61
	.p2align	2
_g_wrap_61:
	and	x0, x0, #0x1fffffffffffffff
	ret

	.globl	_g_wrap_62
	.p2align	2
_g_wrap_62:
	and	x0, x0, #0x3fffffffffffffff
	ret

	.globl	_g_wrap_63
	.p2align	2
_g_wrap_63:
	and	x0, x0, #0x7fffffffffffffff
	ret

	.globl	_g_wrap_7
	.p2align	2
_g_wrap_7:
	and	x0, x0, #0x7f
	ret

	.globl	_g_wrap_8
	.p2align	2
_g_wrap_8:
	and	x0, x0, #0xff
	ret

	.globl	_g_wrap_9
	.p2align	2
_g_wrap_9:
	and	x0, x0, #0x1ff
	ret

	.globl	_g_wrap_64
_g_wrap_64 = _g_clamp_64
.subsections_via_symbols
