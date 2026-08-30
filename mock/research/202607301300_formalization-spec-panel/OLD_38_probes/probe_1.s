	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.private_extern	__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECsd4knXWyury7_43probe_1_collapse_survives_the_encoding_swap
	.globl	__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECsd4knXWyury7_43probe_1_collapse_survives_the_encoding_swap
	.p2align	2
__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECsd4knXWyury7_43probe_1_collapse_survives_the_encoding_swap:
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
	adrp	x1, l_anon.c06290cef2e51b7f979c63e90809c2d0.0@PAGE
Lloh1:
	add	x1, x1, l_anon.c06290cef2e51b7f979c63e90809c2d0.0@PAGEOFF
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
__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECsd4knXWyury7_43probe_1_collapse_survives_the_encoding_swap:
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
__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0Csd4knXWyury7_43probe_1_collapse_survives_the_encoding_swap:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECsd4knXWyury7_43probe_1_collapse_survives_the_encoding_swap
	mov	w0, #0
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__RNSNvYNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0INtNtNtCs5dyeT9KiOLK_4core3ops8function6FnOnceuE9call_once6vtableCsd4knXWyury7_43probe_1_collapse_survives_the_encoding_swap:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECsd4knXWyury7_43probe_1_collapse_survives_the_encoding_swap
	mov	w0, #0
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.private_extern	__RNvCsd4knXWyury7_43probe_1_collapse_survives_the_encoding_swap4main
	.globl	__RNvCsd4knXWyury7_43probe_1_collapse_survives_the_encoding_swap4main
	.p2align	2
__RNvCsd4knXWyury7_43probe_1_collapse_survives_the_encoding_swap4main:
	.cfi_startproc
	sub	sp, sp, #96
	.cfi_def_cfa_offset 96
	stp	x29, x30, [sp, #80]
	add	x29, sp, #80
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	w8, #59836
	movk	w8, #106, lsl #16
	stp	x8, x8, [sp, #8]
	str	x8, [sp, #24]
	add	x8, sp, #8
Lloh2:
	adrp	x9, __RNvXse_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3impxNtB9_7Display3fmt@GOTPAGE
Lloh3:
	ldr	x9, [x9, __RNvXse_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3impxNtB9_7Display3fmt@GOTPAGEOFF]
	stp	x8, x9, [sp, #32]
	add	x8, sp, #16
	stp	x8, x9, [sp, #48]
	add	x8, sp, #24
	stp	x8, x9, [sp, #64]
Lloh4:
	adrp	x0, l_anon.c06290cef2e51b7f979c63e90809c2d0.1@PAGE
Lloh5:
	add	x0, x0, l_anon.c06290cef2e51b7f979c63e90809c2d0.1@PAGEOFF
	add	x1, sp, #32
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	.cfi_def_cfa wsp, 96
	ldp	x29, x30, [sp, #80]
	add	sp, sp, #96
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.loh AdrpAdd	Lloh4, Lloh5
	.loh AdrpLdrGot	Lloh2, Lloh3
	.cfi_endproc

	.globl	_hot_mul_direct
	.p2align	2
_hot_mul_direct:
	.cfi_startproc
	mul	x0, x1, x0
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
Lloh6:
	adrp	x8, __RNvCsd4knXWyury7_43probe_1_collapse_survives_the_encoding_swap4main@PAGE
Lloh7:
	add	x8, x8, __RNvCsd4knXWyury7_43probe_1_collapse_survives_the_encoding_swap4main@PAGEOFF
	str	x8, [sp, #8]
Lloh8:
	adrp	x1, l_anon.c06290cef2e51b7f979c63e90809c2d0.0@PAGE
Lloh9:
	add	x1, x1, l_anon.c06290cef2e51b7f979c63e90809c2d0.0@PAGEOFF
	add	x0, sp, #8
	mov	w4, #0
	bl	__RNvNtCsl82qo6vb64n_3std2rt19lang_start_internal
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
	.loh AdrpAdd	Lloh8, Lloh9
	.loh AdrpAdd	Lloh6, Lloh7
	.cfi_endproc

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.c06290cef2e51b7f979c63e90809c2d0.0:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__RNSNvYNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0INtNtNtCs5dyeT9KiOLK_4core3ops8function6FnOnceuE9call_once6vtableCsd4knXWyury7_43probe_1_collapse_survives_the_encoding_swap
	.quad	__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0Csd4knXWyury7_43probe_1_collapse_survives_the_encoding_swap
	.quad	__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0Csd4knXWyury7_43probe_1_collapse_survives_the_encoding_swap

	.section	__TEXT,__cstring,cstring_literals
l_anon.c06290cef2e51b7f979c63e90809c2d0.1:
	.asciz	"\rOK: direct = \300\016, composite = \300\t, wide = \300\001\n"

	.globl	_hot_mul_via_full_then_quantize
_hot_mul_via_full_then_quantize = _hot_mul_direct
	.globl	_precise_mul_widens
_precise_mul_widens = _hot_mul_direct
.subsections_via_symbols
