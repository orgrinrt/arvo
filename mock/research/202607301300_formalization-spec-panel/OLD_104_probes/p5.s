	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.private_extern	__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECsjGlUjB2m6oY_17p5_occupancy_mask
	.globl	__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECsjGlUjB2m6oY_17p5_occupancy_mask
	.p2align	2
__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECsjGlUjB2m6oY_17p5_occupancy_mask:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x4, x3
	mov	x3, x2
	mov	x2, x1
	str	x0, [sp, #8]
Lloh0:
	adrp	x1, l_anon.53015848fa908cd83f5f1957e0de4eda.0@PAGE
Lloh1:
	add	x1, x1, l_anon.53015848fa908cd83f5f1957e0de4eda.0@PAGEOFF
	add	x0, sp, #8
	bl	__RNvNtCsl82qo6vb64n_3std2rt19lang_start_internal
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
	.loh AdrpAdd	Lloh0, Lloh1

	.p2align	2
__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECsjGlUjB2m6oY_17p5_occupancy_mask:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	blr	x0
	; InlineAsm Start
	; InlineAsm End
	ldp	x29, x30, [sp], #16
	ret

	.p2align	2
__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0CsjGlUjB2m6oY_17p5_occupancy_mask:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	ldr	x0, [x0]
	bl	__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECsjGlUjB2m6oY_17p5_occupancy_mask
	mov	w0, #0
	ldp	x29, x30, [sp], #16
	ret

	.p2align	2
__RNSNvYNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0INtNtNtCs5dyeT9KiOLK_4core3ops8function6FnOnceuE9call_once6vtableCsjGlUjB2m6oY_17p5_occupancy_mask:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	ldr	x0, [x0]
	bl	__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECsjGlUjB2m6oY_17p5_occupancy_mask
	mov	w0, #0
	ldp	x29, x30, [sp], #16
	ret

	.p2align	2
__RNvCsjGlUjB2m6oY_17p5_occupancy_mask12digest_union:
	mov	w8, #16353
	and	w0, w0, w8
	ret

	.p2align	2
__RNvCsjGlUjB2m6oY_17p5_occupancy_mask13digest_prefix:
	and	w0, w0, #0x3fff
	ret

	.private_extern	__RNvCsjGlUjB2m6oY_17p5_occupancy_mask4main
	.globl	__RNvCsjGlUjB2m6oY_17p5_occupancy_mask4main
	.p2align	2
__RNvCsjGlUjB2m6oY_17p5_occupancy_mask4main:
	.cfi_startproc
	sub	sp, sp, #144
	stp	x28, x27, [sp, #48]
	stp	x26, x25, [sp, #64]
	stp	x24, x23, [sp, #80]
	stp	x22, x21, [sp, #96]
	stp	x20, x19, [sp, #112]
	stp	x29, x30, [sp, #128]
	add	x29, sp, #128
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	.cfi_offset w23, -56
	.cfi_offset w24, -64
	.cfi_offset w25, -72
	.cfi_offset w26, -80
	.cfi_offset w27, -88
	.cfi_offset w28, -96
Lloh2:
	adrp	x8, l_anon.53015848fa908cd83f5f1957e0de4eda.1@PAGE
Lloh3:
	add	x8, x8, l_anon.53015848fa908cd83f5f1957e0de4eda.1@PAGEOFF
Lloh4:
	adrp	x20, __RNvXs8_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3impmNtB9_7Display3fmt@GOTPAGE
Lloh5:
	ldr	x20, [x20, __RNvXs8_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3impmNtB9_7Display3fmt@GOTPAGEOFF]
	stp	x8, x20, [sp, #16]
Lloh6:
	adrp	x8, l_anon.53015848fa908cd83f5f1957e0de4eda.2@PAGE
Lloh7:
	add	x8, x8, l_anon.53015848fa908cd83f5f1957e0de4eda.2@PAGEOFF
Lloh8:
	adrp	x19, __RNvXsi_NtNtCs5dyeT9KiOLK_4core3fmt3numtNtB7_6Binary3fmt@GOTPAGE
Lloh9:
	ldr	x19, [x19, __RNvXsi_NtNtCs5dyeT9KiOLK_4core3fmt3numtNtB7_6Binary3fmt@GOTPAGEOFF]
	stp	x8, x19, [sp, #32]
Lloh10:
	adrp	x0, l_anon.53015848fa908cd83f5f1957e0de4eda.3@PAGE
Lloh11:
	add	x0, x0, l_anon.53015848fa908cd83f5f1957e0de4eda.3@PAGEOFF
	add	x1, sp, #16
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh12:
	adrp	x8, l_anon.53015848fa908cd83f5f1957e0de4eda.4@PAGE
Lloh13:
	add	x8, x8, l_anon.53015848fa908cd83f5f1957e0de4eda.4@PAGEOFF
	stp	x8, x19, [sp, #16]
Lloh14:
	adrp	x0, l_anon.53015848fa908cd83f5f1957e0de4eda.5@PAGE
Lloh15:
	add	x0, x0, l_anon.53015848fa908cd83f5f1957e0de4eda.5@PAGEOFF
	add	x1, sp, #16
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh16:
	adrp	x8, l_anon.53015848fa908cd83f5f1957e0de4eda.6@PAGE
Lloh17:
	add	x8, x8, l_anon.53015848fa908cd83f5f1957e0de4eda.6@PAGEOFF
	stp	x8, x19, [sp, #16]
Lloh18:
	adrp	x0, l_anon.53015848fa908cd83f5f1957e0de4eda.7@PAGE
Lloh19:
	add	x0, x0, l_anon.53015848fa908cd83f5f1957e0de4eda.7@PAGEOFF
	add	x1, sp, #16
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	mov	w19, #0
	mov	w21, #0
	mov	w22, #0
	mov	w23, #0
	mov	w24, #0
LBB6_1:
	and	w25, w19, #0xffff
	mov	x0, x19
	bl	__RNvCsjGlUjB2m6oY_17p5_occupancy_mask13digest_prefix
	and	w26, w0, #0xffff
	mov	x0, x19
	bl	__RNvCsjGlUjB2m6oY_17p5_occupancy_mask12digest_union
	and	w27, w0, #0xffff
	eor	w0, w19, #0x1e
	bl	__RNvCsjGlUjB2m6oY_17p5_occupancy_mask12digest_union
	and	w28, w0, #0xffff
	eor	w0, w19, #0x1e
	bl	__RNvCsjGlUjB2m6oY_17p5_occupancy_mask13digest_prefix
	and	w8, w0, #0xffff
	cmp	w26, w8
	cinc	w23, w23, ne
	cmp	w27, w28
	cinc	w24, w24, ne
	eor	w0, w19, #0xffffc000
	bl	__RNvCsjGlUjB2m6oY_17p5_occupancy_mask12digest_union
	and	w28, w0, #0xffff
	eor	w0, w19, #0xffffc000
	bl	__RNvCsjGlUjB2m6oY_17p5_occupancy_mask13digest_prefix
	and	w8, w0, #0xffff
	cmp	w26, w8
	cinc	w23, w23, ne
	cmp	w27, w28
	cinc	w24, w24, ne
	eor	w0, w19, #0x20
	bl	__RNvCsjGlUjB2m6oY_17p5_occupancy_mask12digest_union
	and	w28, w0, #0xffff
	eor	w0, w19, #0x20
	bl	__RNvCsjGlUjB2m6oY_17p5_occupancy_mask13digest_prefix
	and	w8, w0, #0xffff
	cmp	w26, w8
	cinc	w21, w21, eq
	cmp	w27, w28
	cinc	w22, w22, eq
	add	w19, w25, #1
	tbz	w19, #16, LBB6_1
	stp	w21, w22, [sp, #8]
	stp	w23, w24, [sp]
	mov	x8, sp
	stp	x8, x20, [sp, #16]
	add	x8, sp, #8
	stp	x8, x20, [sp, #32]
Lloh20:
	adrp	x0, l_anon.53015848fa908cd83f5f1957e0de4eda.8@PAGE
Lloh21:
	add	x0, x0, l_anon.53015848fa908cd83f5f1957e0de4eda.8@PAGEOFF
	add	x1, sp, #16
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	add	x8, sp, #4
	stp	x8, x20, [sp, #16]
	add	x8, sp, #12
	stp	x8, x20, [sp, #32]
Lloh22:
	adrp	x0, l_anon.53015848fa908cd83f5f1957e0de4eda.9@PAGE
Lloh23:
	add	x0, x0, l_anon.53015848fa908cd83f5f1957e0de4eda.9@PAGEOFF
	add	x1, sp, #16
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	ldp	x29, x30, [sp, #128]
	ldp	x20, x19, [sp, #112]
	ldp	x22, x21, [sp, #96]
	ldp	x24, x23, [sp, #80]
	ldp	x26, x25, [sp, #64]
	ldp	x28, x27, [sp, #48]
	add	sp, sp, #144
	ret
	.loh AdrpAdd	Lloh18, Lloh19
	.loh AdrpAdd	Lloh16, Lloh17
	.loh AdrpAdd	Lloh14, Lloh15
	.loh AdrpAdd	Lloh12, Lloh13
	.loh AdrpAdd	Lloh10, Lloh11
	.loh AdrpLdrGot	Lloh8, Lloh9
	.loh AdrpAdd	Lloh6, Lloh7
	.loh AdrpLdrGot	Lloh4, Lloh5
	.loh AdrpAdd	Lloh2, Lloh3
	.loh AdrpAdd	Lloh22, Lloh23
	.loh AdrpAdd	Lloh20, Lloh21
	.cfi_endproc

	.globl	_main
	.p2align	2
_main:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x3, x1
	sxtw	x2, w0
Lloh24:
	adrp	x8, __RNvCsjGlUjB2m6oY_17p5_occupancy_mask4main@PAGE
Lloh25:
	add	x8, x8, __RNvCsjGlUjB2m6oY_17p5_occupancy_mask4main@PAGEOFF
	str	x8, [sp, #8]
Lloh26:
	adrp	x1, l_anon.53015848fa908cd83f5f1957e0de4eda.0@PAGE
Lloh27:
	add	x1, x1, l_anon.53015848fa908cd83f5f1957e0de4eda.0@PAGEOFF
	add	x0, sp, #8
	mov	w4, #0
	bl	__RNvNtCsl82qo6vb64n_3std2rt19lang_start_internal
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
	.loh AdrpAdd	Lloh26, Lloh27
	.loh AdrpAdd	Lloh24, Lloh25

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.53015848fa908cd83f5f1957e0de4eda.0:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__RNSNvYNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0INtNtNtCs5dyeT9KiOLK_4core3ops8function6FnOnceuE9call_once6vtableCsjGlUjB2m6oY_17p5_occupancy_mask
	.quad	__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0CsjGlUjB2m6oY_17p5_occupancy_mask
	.quad	__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0CsjGlUjB2m6oY_17p5_occupancy_mask

	.section	__TEXT,__literal4,4byte_literals
	.p2align	2, 0x0
l_anon.53015848fa908cd83f5f1957e0de4eda.1:
	.asciz	"\016\000\000"

	.section	__TEXT,__const
	.p2align	1, 0x0
l_anon.53015848fa908cd83f5f1957e0de4eda.2:
	.ascii	"\377?"

l_anon.53015848fa908cd83f5f1957e0de4eda.3:
	.asciz	"\006W_F = \300\020, prefix mask = \303 \000\200i\022\000\001\n"

	.p2align	1, 0x0
l_anon.53015848fa908cd83f5f1957e0de4eda.4:
	.ascii	"\341?"

l_anon.53015848fa908cd83f5f1957e0de4eda.5:
	.asciz	"\025       union  mask = \303 \000\200i\022\000\001\n"

	.section	__TEXT,__cstring,cstring_literals
	.p2align	1, 0x0
l_anon.53015848fa908cd83f5f1957e0de4eda.6:
	.asciz	"\036"

	.section	__TEXT,__const
l_anon.53015848fa908cd83f5f1957e0de4eda.7:
	.asciz	"\032hole  = prefix & !union = \303 \000\200i\022\000\001\n"

	.section	__TEXT,__cstring,cstring_literals
l_anon.53015848fa908cd83f5f1957e0de4eda.8:
	.asciz	"\027prefix mask: separates \300\030 equal pairs, conflates \300\017 unequal pairs\n"

l_anon.53015848fa908cd83f5f1957e0de4eda.9:
	.asciz	"\027union  mask: separates \300\030 equal pairs, conflates \300\017 unequal pairs\n"

.subsections_via_symbols
