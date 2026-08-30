	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCs2MpE7arxLI9_26d01_bare_parameter_carrier8octave_1
	.p2align	2
__RNvCs2MpE7arxLI9_26d01_bare_parameter_carrier8octave_1:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
Lloh0:
	adrp	x0, l_anon.c06ddca7eb79834db152764664ba74f8.0@PAGE
Lloh1:
	add	x0, x0, l_anon.c06ddca7eb79834db152764664ba74f8.0@PAGEOFF
Lloh2:
	adrp	x2, l_anon.c06ddca7eb79834db152764664ba74f8.2@PAGE
Lloh3:
	add	x2, x2, l_anon.c06ddca7eb79834db152764664ba74f8.2@PAGEOFF
	mov	w1, #19
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking5panic
	.loh AdrpAdd	Lloh2, Lloh3
	.loh AdrpAdd	Lloh0, Lloh1
	.cfi_endproc

	.globl	__RNvCs2MpE7arxLI9_26d01_bare_parameter_carrier8octave_2
	.p2align	2
__RNvCs2MpE7arxLI9_26d01_bare_parameter_carrier8octave_2:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
Lloh4:
	adrp	x0, l_anon.c06ddca7eb79834db152764664ba74f8.0@PAGE
Lloh5:
	add	x0, x0, l_anon.c06ddca7eb79834db152764664ba74f8.0@PAGEOFF
Lloh6:
	adrp	x2, l_anon.c06ddca7eb79834db152764664ba74f8.2@PAGE
Lloh7:
	add	x2, x2, l_anon.c06ddca7eb79834db152764664ba74f8.2@PAGEOFF
	mov	w1, #19
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking5panic
	.loh AdrpAdd	Lloh6, Lloh7
	.loh AdrpAdd	Lloh4, Lloh5
	.cfi_endproc

	.globl	__RNvCs2MpE7arxLI9_26d01_bare_parameter_carrier8octave_3
	.p2align	2
__RNvCs2MpE7arxLI9_26d01_bare_parameter_carrier8octave_3:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
Lloh8:
	adrp	x0, l_anon.c06ddca7eb79834db152764664ba74f8.0@PAGE
Lloh9:
	add	x0, x0, l_anon.c06ddca7eb79834db152764664ba74f8.0@PAGEOFF
Lloh10:
	adrp	x2, l_anon.c06ddca7eb79834db152764664ba74f8.2@PAGE
Lloh11:
	add	x2, x2, l_anon.c06ddca7eb79834db152764664ba74f8.2@PAGEOFF
	mov	w1, #19
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking5panic
	.loh AdrpAdd	Lloh10, Lloh11
	.loh AdrpAdd	Lloh8, Lloh9
	.cfi_endproc

	.globl	_d01_arvo16
	.p2align	2
_d01_arvo16:
	.cfi_startproc
	add	w0, w1, w0
	ret
	.cfi_endproc

	.globl	_d01_arvo_vec
	.p2align	2
_d01_arvo_vec:
	.cfi_startproc
	add	x8, x0, #32
	add	x9, x1, #32
	mov	w10, #1024
LBB4_1:
	ldp	q0, q1, [x8, #-32]
	ldp	q2, q3, [x8]
	ldp	q4, q5, [x9, #-32]
	ldp	q6, q7, [x9], #64
	add.8h	v0, v4, v0
	add.8h	v1, v5, v1
	add.8h	v2, v6, v2
	add.8h	v3, v7, v3
	stp	q0, q1, [x8, #-32]
	stp	q2, q3, [x8], #64
	subs	x10, x10, #32
	b.ne	LBB4_1
	ret
	.cfi_endproc

	.globl	_d01_native_vec
	.p2align	2
_d01_native_vec:
	.cfi_startproc
	add	x8, x0, #32
	add	x9, x1, #32
	mov	w10, #1024
LBB5_1:
	ldp	q0, q1, [x8, #-32]
	ldp	q2, q3, [x8]
	ldp	q4, q5, [x9, #-32]
	ldp	q6, q7, [x9], #64
	add.8h	v0, v4, v0
	add.8h	v1, v5, v1
	add.8h	v2, v6, v2
	add.8h	v3, v7, v3
	stp	q0, q1, [x8, #-32]
	stp	q2, q3, [x8], #64
	subs	x10, x10, #32
	b.ne	LBB5_1
	ret
	.cfi_endproc

	.section	__TEXT,__const
l_anon.c06ddca7eb79834db152764664ba74f8.0:
	.ascii	"not yet implemented"

	.section	__TEXT,__cstring,cstring_literals
l_anon.c06ddca7eb79834db152764664ba74f8.1:
	.asciz	"d01_bare_parameter_carrier.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.c06ddca7eb79834db152764664ba74f8.2:
	.quad	l_anon.c06ddca7eb79834db152764664ba74f8.1
	.asciz	"\035\000\000\000\000\000\000\0008\000\000\000\005\000\000"

	.globl	_d01_native16
_d01_native16 = _d01_arvo16
.subsections_via_symbols
