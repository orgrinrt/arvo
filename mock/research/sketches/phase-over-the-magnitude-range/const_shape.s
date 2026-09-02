	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.private_extern	__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECsicMrR4tVax5_11const_shape
	.globl	__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECsicMrR4tVax5_11const_shape
	.p2align	2
__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECsicMrR4tVax5_11const_shape:
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
	adrp	x1, l_anon.79a8eb8af067b935218b0d42e2ea10ed.0@PAGE
Lloh1:
	add	x1, x1, l_anon.79a8eb8af067b935218b0d42e2ea10ed.0@PAGEOFF
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
__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECsicMrR4tVax5_11const_shape:
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
__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0CsicMrR4tVax5_11const_shape:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECsicMrR4tVax5_11const_shape
	mov	w0, #0
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__RNSNvYNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0INtNtNtCs5dyeT9KiOLK_4core3ops8function6FnOnceuE9call_once6vtableCsicMrR4tVax5_11const_shape:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECsicMrR4tVax5_11const_shape
	mov	w0, #0
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.private_extern	__RNvCsicMrR4tVax5_11const_shape4main
	.globl	__RNvCsicMrR4tVax5_11const_shape4main
	.p2align	2
__RNvCsicMrR4tVax5_11const_shape4main:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
Lloh2:
	adrp	x19, __RNvXsg_NtCs5dyeT9KiOLK_4core3fmtbNtB5_7Display3fmt@GOTPAGE
Lloh3:
	ldr	x19, [x19, __RNvXsg_NtCs5dyeT9KiOLK_4core3fmtbNtB5_7Display3fmt@GOTPAGEOFF]
Lloh4:
	adrp	x20, l_anon.79a8eb8af067b935218b0d42e2ea10ed.1@PAGE
Lloh5:
	add	x20, x20, l_anon.79a8eb8af067b935218b0d42e2ea10ed.1@PAGEOFF
	stp	x20, x19, [sp]
Lloh6:
	adrp	x0, l_anon.79a8eb8af067b935218b0d42e2ea10ed.2@PAGE
Lloh7:
	add	x0, x0, l_anon.79a8eb8af067b935218b0d42e2ea10ed.2@PAGEOFF
	mov	x1, sp
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh8:
	adrp	x8, l_anon.79a8eb8af067b935218b0d42e2ea10ed.3@PAGE
Lloh9:
	add	x8, x8, l_anon.79a8eb8af067b935218b0d42e2ea10ed.3@PAGEOFF
	stp	x8, x19, [sp]
Lloh10:
	adrp	x0, l_anon.79a8eb8af067b935218b0d42e2ea10ed.4@PAGE
Lloh11:
	add	x0, x0, l_anon.79a8eb8af067b935218b0d42e2ea10ed.4@PAGEOFF
	mov	x1, sp
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	stp	x20, x19, [sp]
Lloh12:
	adrp	x0, l_anon.79a8eb8af067b935218b0d42e2ea10ed.5@PAGE
Lloh13:
	add	x0, x0, l_anon.79a8eb8af067b935218b0d42e2ea10ed.5@PAGEOFF
	mov	x1, sp
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	stp	x20, x19, [sp]
Lloh14:
	adrp	x0, l_anon.79a8eb8af067b935218b0d42e2ea10ed.6@PAGE
Lloh15:
	add	x0, x0, l_anon.79a8eb8af067b935218b0d42e2ea10ed.6@PAGEOFF
	mov	x1, sp
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh16:
	adrp	x0, l_anon.79a8eb8af067b935218b0d42e2ea10ed.7@PAGE
Lloh17:
	add	x0, x0, l_anon.79a8eb8af067b935218b0d42e2ea10ed.7@PAGEOFF
	mov	w1, #139
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w19
	.cfi_restore w20
	ret
	.loh AdrpAdd	Lloh16, Lloh17
	.loh AdrpAdd	Lloh14, Lloh15
	.loh AdrpAdd	Lloh12, Lloh13
	.loh AdrpAdd	Lloh10, Lloh11
	.loh AdrpAdd	Lloh8, Lloh9
	.loh AdrpAdd	Lloh6, Lloh7
	.loh AdrpAdd	Lloh4, Lloh5
	.loh AdrpLdrGot	Lloh2, Lloh3
	.cfi_endproc

	.globl	_folds_becomes_whole
	.p2align	2
_folds_becomes_whole:
	.cfi_startproc
	mov	w0, #1
	ret
	.cfi_endproc

	.globl	_folds_half
	.p2align	2
_folds_half:
	.cfi_startproc
	mov	w0, #0
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
Lloh18:
	adrp	x8, __RNvCsicMrR4tVax5_11const_shape4main@PAGE
Lloh19:
	add	x8, x8, __RNvCsicMrR4tVax5_11const_shape4main@PAGEOFF
	str	x8, [sp, #8]
Lloh20:
	adrp	x1, l_anon.79a8eb8af067b935218b0d42e2ea10ed.0@PAGE
Lloh21:
	add	x1, x1, l_anon.79a8eb8af067b935218b0d42e2ea10ed.0@PAGEOFF
	add	x0, sp, #8
	mov	w4, #0
	bl	__RNvNtCsl82qo6vb64n_3std2rt19lang_start_internal
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
	.loh AdrpAdd	Lloh20, Lloh21
	.loh AdrpAdd	Lloh18, Lloh19
	.cfi_endproc

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.79a8eb8af067b935218b0d42e2ea10ed.0:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__RNSNvYNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0INtNtNtCs5dyeT9KiOLK_4core3ops8function6FnOnceuE9call_once6vtableCsicMrR4tVax5_11const_shape
	.quad	__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0CsicMrR4tVax5_11const_shape
	.quad	__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0CsicMrR4tVax5_11const_shape

	.section	__TEXT,__const
l_anon.79a8eb8af067b935218b0d42e2ea10ed.1:
	.byte	1

	.section	__TEXT,__cstring,cstring_literals
l_anon.79a8eb8af067b935218b0d42e2ea10ed.2:
	.asciz	",Zero (phase 0, constant)                    \300\001\n"

l_anon.79a8eb8af067b935218b0d42e2ea10ed.3:
	.space	1

l_anon.79a8eb8af067b935218b0d42e2ea10ed.4:
	.asciz	",HalfStep (phase 1/2, constant)              \300\001\n"

l_anon.79a8eb8af067b935218b0d42e2ea10ed.5:
	.asciz	"-WholeOutOfReach (phase 4, Indexed+Signed<2>) \300\001\n"

l_anon.79a8eb8af067b935218b0d42e2ea10ed.6:
	.asciz	"-FractionalBecomesWhole (phase 1/2, slope -1) \300\001\n"

	.section	__TEXT,__const
l_anon.79a8eb8af067b935218b0d42e2ea10ed.7:
	.ascii	"control: cutting the magnitude range to one takes both back to false\n"

	.globl	_folds_out_of_reach
_folds_out_of_reach = _folds_becomes_whole
	.globl	_folds_zero
_folds_zero = _folds_becomes_whole
.subsections_via_symbols
