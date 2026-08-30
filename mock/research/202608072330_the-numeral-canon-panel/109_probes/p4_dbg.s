	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_add_bare
	.p2align	2
_add_bare:
	.cfi_startproc
	add	w8, w1, w0
	and	w0, w8, #0xff
	ret
	.cfi_endproc

	.globl	_add_imitating
	.p2align	2
_add_imitating:
Lfunc_begin0:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception0
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	add	w8, w1, w0
	tbnz	w8, #8, LBB1_2
	and	w0, w8, #0xff
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB1_2:
	.cfi_restore_state
Ltmp0:
Lloh0:
	adrp	x0, l_anon.ae4c43905420fe447dfb0eb9743e0170.0@PAGE
Lloh1:
	add	x0, x0, l_anon.ae4c43905420fe447dfb0eb9743e0170.0@PAGEOFF
Lloh2:
	adrp	x2, l_anon.ae4c43905420fe447dfb0eb9743e0170.2@PAGE
Lloh3:
	add	x2, x2, l_anon.ae4c43905420fe447dfb0eb9743e0170.2@PAGEOFF
	mov	w1, #57
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking9panic_fmt
Ltmp1:
	brk	#0x1
LBB1_4:
Ltmp2:
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking19panic_cannot_unwind
	.loh AdrpAdd	Lloh2, Lloh3
	.loh AdrpAdd	Lloh0, Lloh1
Lfunc_end0:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table1:
Lexception0:
	.byte	255
	.byte	155
	.uleb128 Lttbase0-Lttbaseref0
Lttbaseref0:
	.byte	1
	.uleb128 Lcst_end0-Lcst_begin0
Lcst_begin0:
	.uleb128 Ltmp0-Lfunc_begin0
	.uleb128 Ltmp1-Ltmp0
	.uleb128 Ltmp2-Lfunc_begin0
	.byte	1
Lcst_end0:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase0:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.globl	_add_ungated_check
	.p2align	2
_add_ungated_check:
Lfunc_begin1:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception1
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	add	w8, w1, w0
	tbnz	w8, #8, LBB2_2
	and	w0, w8, #0xff
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB2_2:
	.cfi_restore_state
Ltmp3:
Lloh4:
	adrp	x0, l_anon.ae4c43905420fe447dfb0eb9743e0170.0@PAGE
Lloh5:
	add	x0, x0, l_anon.ae4c43905420fe447dfb0eb9743e0170.0@PAGEOFF
Lloh6:
	adrp	x2, l_anon.ae4c43905420fe447dfb0eb9743e0170.3@PAGE
Lloh7:
	add	x2, x2, l_anon.ae4c43905420fe447dfb0eb9743e0170.3@PAGEOFF
	mov	w1, #57
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking9panic_fmt
Ltmp4:
	brk	#0x1
LBB2_4:
Ltmp5:
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking19panic_cannot_unwind
	.loh AdrpAdd	Lloh6, Lloh7
	.loh AdrpAdd	Lloh4, Lloh5
Lfunc_end1:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table2:
Lexception1:
	.byte	255
	.byte	155
	.uleb128 Lttbase1-Lttbaseref1
Lttbaseref1:
	.byte	1
	.uleb128 Lcst_end1-Lcst_begin1
Lcst_begin1:
	.uleb128 Ltmp3-Lfunc_begin1
	.uleb128 Ltmp4-Ltmp3
	.uleb128 Ltmp5-Lfunc_begin1
	.byte	1
Lcst_end1:
	.byte	127
	.byte	0
	.p2align	2, 0x0
Lttbase1:
	.byte	0
	.p2align	2, 0x0

	.section	__TEXT,__const
l_anon.ae4c43905420fe447dfb0eb9743e0170.0:
	.ascii	"attempt to add with overflow"

	.section	__TEXT,__cstring,cstring_literals
l_anon.ae4c43905420fe447dfb0eb9743e0170.1:
	.asciz	"p4_the_bounded_panic_erases_under_lowering.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.ae4c43905420fe447dfb0eb9743e0170.2:
	.quad	l_anon.ae4c43905420fe447dfb0eb9743e0170.1
	.asciz	"-\000\000\000\000\000\000\0009\000\000\000\t\000\000"

	.p2align	3, 0x0
l_anon.ae4c43905420fe447dfb0eb9743e0170.3:
	.quad	l_anon.ae4c43905420fe447dfb0eb9743e0170.1
	.asciz	"-\000\000\000\000\000\000\000[\000\000\000\t\000\000"

	.globl	_add_speed_first
_add_speed_first = _add_bare
.subsections_via_symbols
