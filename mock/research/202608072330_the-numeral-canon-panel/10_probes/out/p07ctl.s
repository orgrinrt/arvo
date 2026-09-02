	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCsazk7ErYjC2t_11p7_law_site8law_site
	.p2align	2
__RNvCsazk7ErYjC2t_11p7_law_site8law_site:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
Lloh0:
	adrp	x0, l_anon.2a841b5b273377fe48de9399aecbce0f.0@PAGE
Lloh1:
	add	x0, x0, l_anon.2a841b5b273377fe48de9399aecbce0f.0@PAGEOFF
Lloh2:
	adrp	x2, l_anon.2a841b5b273377fe48de9399aecbce0f.2@PAGE
Lloh3:
	add	x2, x2, l_anon.2a841b5b273377fe48de9399aecbce0f.2@PAGEOFF
	mov	w1, #19
	bl	__RNvNtCs5dyeT9KiOLK_4core9panicking5panic
	.loh AdrpAdd	Lloh2, Lloh3
	.loh AdrpAdd	Lloh0, Lloh1
	.cfi_endproc

	.globl	_arvo16
	.p2align	2
_arvo16:
	.cfi_startproc
	add	w0, w1, w0
	ret
	.cfi_endproc

	.globl	_arvo64
	.p2align	2
_arvo64:
	.cfi_startproc
	add	x0, x1, x0
	ret
	.cfi_endproc

	.globl	_arvo_vec
	.p2align	2
_arvo_vec:
	.cfi_startproc
	add	x8, x0, #32
	add	x9, x1, #32
	mov	w10, #1024
LBB3_1:
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
	b.ne	LBB3_1
	ret
	.cfi_endproc

	.globl	_arvo_wide200
	.p2align	2
_arvo_wide200:
	.cfi_startproc
	ldp	x9, x10, [x0]
	ldp	x11, x12, [x0, #16]
	ldp	x13, x14, [x1]
	ldp	x15, x16, [x1, #16]
	adds	x9, x13, x9
	adcs	x10, x14, x10
	adcs	x11, x15, x11
	adc	x12, x16, x12
	stp	x9, x10, [x8]
	stp	x11, x12, [x8, #16]
	ret
	.cfi_endproc

	.globl	_bar_wide256
	.p2align	2
_bar_wide256:
	.cfi_startproc
	ldp	x9, x10, [x0]
	ldp	x11, x12, [x1]
	adds	x9, x11, x9
	adcs	x10, x12, x10
	ldp	x11, x12, [x0, #16]
	ldp	x13, x14, [x1, #16]
	adcs	x11, x13, x11
	adc	x12, x14, x12
	stp	x9, x10, [x8]
	stp	x11, x12, [x8, #16]
	ret
	.cfi_endproc

	.globl	_consumer_818
	.p2align	2
_consumer_818:
	.cfi_startproc
	stp	x28, x27, [sp, #-80]!
	.cfi_def_cfa_offset 80
	stp	x26, x25, [sp, #16]
	stp	x24, x23, [sp, #32]
	stp	x22, x21, [sp, #48]
	stp	x20, x19, [sp, #64]
	.cfi_offset w19, -8
	.cfi_offset w20, -16
	.cfi_offset w21, -24
	.cfi_offset w22, -32
	.cfi_offset w23, -40
	.cfi_offset w24, -48
	.cfi_offset w25, -56
	.cfi_offset w26, -64
	.cfi_offset w27, -72
	.cfi_offset w28, -80
	ldp	x9, x10, [x0]
	ldp	x11, x12, [x0, #16]
	ldp	x13, x14, [x0, #32]
	ldp	x15, x16, [x0, #48]
	ldp	x17, x2, [x0, #64]
	ldp	x3, x4, [x0, #80]
	ldr	x0, [x0, #96]
	ldp	x5, x6, [x1]
	ldp	x7, x19, [x1, #16]
	ldp	x20, x21, [x1, #32]
	ldp	x22, x23, [x1, #48]
	ldp	x24, x25, [x1, #64]
	ldp	x26, x27, [x1, #80]
	ldr	x1, [x1, #96]
	adds	x9, x5, x9
	adcs	x10, x6, x10
	adcs	x11, x7, x11
	adcs	x12, x19, x12
	adcs	x13, x20, x13
	adcs	x14, x21, x14
	adcs	x15, x22, x15
	adcs	x16, x23, x16
	adcs	x17, x24, x17
	adcs	x2, x25, x2
	adcs	x3, x26, x3
	adcs	x4, x27, x4
	adc	x0, x1, x0
	stp	x9, x10, [x8]
	stp	x11, x12, [x8, #16]
	stp	x13, x14, [x8, #32]
	stp	x15, x16, [x8, #48]
	stp	x17, x2, [x8, #64]
	stp	x3, x4, [x8, #80]
	str	x0, [x8, #96]
	ldp	x20, x19, [sp, #64]
	ldp	x22, x21, [sp, #48]
	ldp	x24, x23, [sp, #32]
	ldp	x26, x25, [sp, #16]
	ldp	x28, x27, [sp], #80
	.cfi_def_cfa_offset 0
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w21
	.cfi_restore w22
	.cfi_restore w23
	.cfi_restore w24
	.cfi_restore w25
	.cfi_restore w26
	.cfi_restore w27
	.cfi_restore w28
	ret
	.cfi_endproc

	.section	__TEXT,__const
l_anon.2a841b5b273377fe48de9399aecbce0f.0:
	.ascii	"not yet implemented"

	.section	__TEXT,__cstring,cstring_literals
l_anon.2a841b5b273377fe48de9399aecbce0f.1:
	.asciz	"reproduced/p7_law_site.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.2a841b5b273377fe48de9399aecbce0f.2:
	.quad	l_anon.2a841b5b273377fe48de9399aecbce0f.1
	.asciz	"\031\000\000\000\000\000\000\000\273\000\000\000\005\000\000"

	.globl	_native_vec
_native_vec = _arvo_vec
	.globl	_native16
_native16 = _arvo16
	.globl	_native64
_native64 = _arvo64
.subsections_via_symbols
