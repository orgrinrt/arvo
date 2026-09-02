	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.private_extern	__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECs8wZAD669Sqp_46p7_the_merge_is_visible_in_the_emitted_symbols
	.globl	__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECs8wZAD669Sqp_46p7_the_merge_is_visible_in_the_emitted_symbols
	.p2align	2
__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECs8wZAD669Sqp_46p7_the_merge_is_visible_in_the_emitted_symbols:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x4, x3
	mov	x3, x2
	mov	x2, x1
	str	x0, [sp, #8]
Lloh0:
	adrp	x1, l_anon.4bb88a279aca7a64d43c463164043676.0@PAGE
Lloh1:
	add	x1, x1, l_anon.4bb88a279aca7a64d43c463164043676.0@PAGEOFF
	add	x0, sp, #8
	bl	__RNvNtCsl82qo6vb64n_3std2rt19lang_start_internal
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.loh AdrpAdd	Lloh0, Lloh1
	.cfi_endproc

	.p2align	2
__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs8wZAD669Sqp_46p7_the_merge_is_visible_in_the_emitted_symbols:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	blr	x0
	; InlineAsm Start
	; InlineAsm End
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0Cs8wZAD669Sqp_46p7_the_merge_is_visible_in_the_emitted_symbols:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs8wZAD669Sqp_46p7_the_merge_is_visible_in_the_emitted_symbols
	mov	w0, #0
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__RNSNvYNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0INtNtNtCs5dyeT9KiOLK_4core3ops8function6FnOnceuE9call_once6vtableCs8wZAD669Sqp_46p7_the_merge_is_visible_in_the_emitted_symbols:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs8wZAD669Sqp_46p7_the_merge_is_visible_in_the_emitted_symbols
	mov	w0, #0
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.private_extern	__RNvCs8wZAD669Sqp_46p7_the_merge_is_visible_in_the_emitted_symbols4main
	.globl	__RNvCs8wZAD669Sqp_46p7_the_merge_is_visible_in_the_emitted_symbols4main
	.p2align	2
__RNvCs8wZAD669Sqp_46p7_the_merge_is_visible_in_the_emitted_symbols4main:
	.cfi_startproc
	sub	sp, sp, #96
	.cfi_def_cfa_offset 96
	stp	x24, x23, [sp, #32]
	stp	x22, x21, [sp, #48]
	stp	x20, x19, [sp, #64]
	stp	x29, x30, [sp, #80]
	add	x29, sp, #80
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	.cfi_offset w23, -56
	.cfi_offset w24, -64
	mov	w19, #0
	mov	w21, #0
LBB4_1:
	mov	w20, #0
LBB4_2:
	and	w22, w20, #0xff
	mov	x0, x19
	mov	x1, x20
	bl	_proved_sat
	and	w23, w0, #0xff
	mov	x0, x19
	mov	x1, x20
	bl	_proved_sat
	cmp	w23, w0, uxtb
	cinc	w21, w21, eq
	cmp	w22, #100
	csinc	w20, w22, w20, eq
	b.ne	LBB4_2
	and	w8, w19, #0xff
	cmp	w8, #100
	csinc	w19, w8, w19, eq
	b.eq	LBB4_5
	and	w8, w19, #0xff
	cmp	w8, #101
	b.lo	LBB4_1
LBB4_5:
	mov	w19, #0
	mov	w22, #0
	str	w21, [sp, #4]
LBB4_6:
	mov	w20, #0
LBB4_7:
	and	w8, w20, #0xff
	add	w21, w8, #1
	mov	x0, x19
	mov	x1, x20
	bl	_ungated_sat
	and	w23, w0, #0xff
	mov	x0, x19
	mov	x1, x20
	bl	_proved_sat
	cmp	w23, w0, uxtb
	cinc	w22, w22, ne
	mov	x20, x21
	tbz	w21, #8, LBB4_7
	and	w8, w19, #0xff
	add	w19, w8, #1
	tbz	w19, #8, LBB4_6
	str	w22, [sp, #8]
	add	x8, sp, #4
Lloh2:
	adrp	x19, __RNvXs8_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3impmNtB9_7Display3fmt@GOTPAGE
Lloh3:
	ldr	x19, [x19, __RNvXs8_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3impmNtB9_7Display3fmt@GOTPAGEOFF]
	stp	x8, x19, [sp, #16]
Lloh4:
	adrp	x0, l_anon.4bb88a279aca7a64d43c463164043676.1@PAGE
Lloh5:
	add	x0, x0, l_anon.4bb88a279aca7a64d43c463164043676.1@PAGEOFF
	add	x1, sp, #16
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	add	x8, sp, #8
	stp	x8, x19, [sp, #16]
Lloh6:
	adrp	x0, l_anon.4bb88a279aca7a64d43c463164043676.2@PAGE
Lloh7:
	add	x0, x0, l_anon.4bb88a279aca7a64d43c463164043676.2@PAGEOFF
	add	x1, sp, #16
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	mov	w0, #100
	mov	w1, #100
	bl	_proved_sat
	strb	w0, [sp, #15]
	add	x19, sp, #15
Lloh8:
	adrp	x20, __RNvXNtNtNtCs5dyeT9KiOLK_4core3fmt3num3imphNtB6_7Display3fmt@GOTPAGE
Lloh9:
	ldr	x20, [x20, __RNvXNtNtNtCs5dyeT9KiOLK_4core3fmt3num3imphNtB6_7Display3fmt@GOTPAGEOFF]
	stp	x19, x20, [sp, #16]
Lloh10:
	adrp	x0, l_anon.4bb88a279aca7a64d43c463164043676.3@PAGE
Lloh11:
	add	x0, x0, l_anon.4bb88a279aca7a64d43c463164043676.3@PAGEOFF
	add	x1, sp, #16
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	mov	w0, #200
	mov	w1, #200
	bl	_ungated_sat
	strb	w0, [sp, #15]
	stp	x19, x20, [sp, #16]
Lloh12:
	adrp	x0, l_anon.4bb88a279aca7a64d43c463164043676.4@PAGE
Lloh13:
	add	x0, x0, l_anon.4bb88a279aca7a64d43c463164043676.4@PAGEOFF
	add	x1, sp, #16
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	mov	w0, #200
	mov	w1, #200
	bl	_proved_sat
	strb	w0, [sp, #15]
	stp	x19, x20, [sp, #16]
Lloh14:
	adrp	x0, l_anon.4bb88a279aca7a64d43c463164043676.5@PAGE
Lloh15:
	add	x0, x0, l_anon.4bb88a279aca7a64d43c463164043676.5@PAGEOFF
	add	x1, sp, #16
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	.cfi_def_cfa wsp, 96
	ldp	x29, x30, [sp, #80]
	ldp	x20, x19, [sp, #64]
	ldp	x22, x21, [sp, #48]
	ldp	x24, x23, [sp, #32]
	add	sp, sp, #96
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w21
	.cfi_restore w22
	.cfi_restore w23
	.cfi_restore w24
	ret
	.loh AdrpAdd	Lloh14, Lloh15
	.loh AdrpAdd	Lloh12, Lloh13
	.loh AdrpAdd	Lloh10, Lloh11
	.loh AdrpLdrGot	Lloh8, Lloh9
	.loh AdrpAdd	Lloh6, Lloh7
	.loh AdrpAdd	Lloh4, Lloh5
	.loh AdrpLdrGot	Lloh2, Lloh3
	.cfi_endproc

	.globl	_proved_sat
	.p2align	2
_proved_sat:
	.cfi_startproc
	add	w0, w1, w0
	ret
	.cfi_endproc

	.globl	_ungated_sat
	.p2align	2
_ungated_sat:
	.cfi_startproc
	and	w8, w1, #0xff
	add	w8, w8, w0, uxtb
	mov	w9, #255
	cmp	w8, #255
	csel	w0, w8, w9, lo
	ret
	.cfi_endproc

	.globl	_main
	.p2align	2
_main:
	.cfi_startproc
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x3, x1
	sxtw	x2, w0
Lloh16:
	adrp	x8, __RNvCs8wZAD669Sqp_46p7_the_merge_is_visible_in_the_emitted_symbols4main@PAGE
Lloh17:
	add	x8, x8, __RNvCs8wZAD669Sqp_46p7_the_merge_is_visible_in_the_emitted_symbols4main@PAGEOFF
	str	x8, [sp, #8]
Lloh18:
	adrp	x1, l_anon.4bb88a279aca7a64d43c463164043676.0@PAGE
Lloh19:
	add	x1, x1, l_anon.4bb88a279aca7a64d43c463164043676.0@PAGEOFF
	add	x0, sp, #8
	mov	w4, #0
	bl	__RNvNtCsl82qo6vb64n_3std2rt19lang_start_internal
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
	.loh AdrpAdd	Lloh18, Lloh19
	.loh AdrpAdd	Lloh16, Lloh17
	.cfi_endproc

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.4bb88a279aca7a64d43c463164043676.0:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__RNSNvYNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0INtNtNtCs5dyeT9KiOLK_4core3ops8function6FnOnceuE9call_once6vtableCs8wZAD669Sqp_46p7_the_merge_is_visible_in_the_emitted_symbols
	.quad	__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0Cs8wZAD669Sqp_46p7_the_merge_is_visible_in_the_emitted_symbols
	.quad	__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0Cs8wZAD669Sqp_46p7_the_merge_is_visible_in_the_emitted_symbols

	.section	__TEXT,__cstring,cstring_literals
l_anon.4bb88a279aca7a64d43c463164043676.1:
	.asciz	"/within the proved bound, sat and wrap agree on \300\020 of 10201 pairs\n"

l_anon.4bb88a279aca7a64d43c463164043676.2:
	.asciz	"\033outside it, they differ on \300\020 of 65536 pairs\n"

l_anon.4bb88a279aca7a64d43c463164043676.3:
	.asciz	"\030proved_sat(100,100)   = \300\001\n"

l_anon.4bb88a279aca7a64d43c463164043676.4:
	.asciz	"\030unproved_sat(200,200) = \300\001\n"

l_anon.4bb88a279aca7a64d43c463164043676.5:
	.asciz	"\030unproved_wrap(200,200)= \300\001\n"

	.globl	_proved_wrap
_proved_wrap = _proved_sat
	.globl	_ungated_wrap
_ungated_wrap = _proved_sat
	.globl	_unproved_wrap
_unproved_wrap = _proved_sat
	.globl	_unproved_sat
_unproved_sat = _ungated_sat
.subsections_via_symbols
