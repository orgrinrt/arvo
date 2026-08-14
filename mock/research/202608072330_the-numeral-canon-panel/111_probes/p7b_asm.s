	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.private_extern	__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECsbhamypr2ijH_41p7b_a_declared_range_inside_the_container
	.globl	__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECsbhamypr2ijH_41p7b_a_declared_range_inside_the_container
	.p2align	2
__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECsbhamypr2ijH_41p7b_a_declared_range_inside_the_container:
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
	adrp	x1, l_anon.4bb7a5b5488537302fe6f1f624dc55e5.0@PAGE
Lloh1:
	add	x1, x1, l_anon.4bb7a5b5488537302fe6f1f624dc55e5.0@PAGEOFF
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
__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECsbhamypr2ijH_41p7b_a_declared_range_inside_the_container:
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
__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0Csbhamypr2ijH_41p7b_a_declared_range_inside_the_container:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECsbhamypr2ijH_41p7b_a_declared_range_inside_the_container
	mov	w0, #0
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__RNSNvYNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0INtNtNtCs5dyeT9KiOLK_4core3ops8function6FnOnceuE9call_once6vtableCsbhamypr2ijH_41p7b_a_declared_range_inside_the_container:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECsbhamypr2ijH_41p7b_a_declared_range_inside_the_container
	mov	w0, #0
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.private_extern	__RNvCsbhamypr2ijH_41p7b_a_declared_range_inside_the_container4main
	.globl	__RNvCsbhamypr2ijH_41p7b_a_declared_range_inside_the_container4main
	.p2align	2
__RNvCsbhamypr2ijH_41p7b_a_declared_range_inside_the_container4main:
	.cfi_startproc
	sub	sp, sp, #144
	.cfi_def_cfa_offset 144
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
	mov	w19, #0
	mov	w23, #0
	mov	w24, #0
LBB4_1:
	mov	w20, #0
LBB4_2:
	and	w21, w20, #0xff
	mov	x0, x19
	mov	x1, x20
	bl	_proved_sat
	and	w22, w0, #0xff
	mov	x0, x19
	mov	x1, x20
	bl	_proved_sat
	cmp	w22, w0, uxtb
	cinc	w24, w24, eq
	add	w23, w23, #1
	cmp	w21, #100
	csinc	w20, w21, w20, eq
	b.ne	LBB4_2
	and	w8, w19, #0xff
	cmp	w8, #100
	csinc	w19, w8, w19, eq
	b.eq	LBB4_5
	and	w8, w19, #0xff
	cmp	w8, #100
	b.ls	LBB4_1
LBB4_5:
	mov	w19, #0
	mov	w21, #0
	mov	w22, #0
	stp	w24, w23, [sp, #8]
	mov	w23, #-56
LBB4_6:
	mov	w20, #0
LBB4_7:
	and	w24, w20, #0xff
	mov	x0, x19
	mov	x1, x20
	bl	_ungated_sat
	and	w25, w0, #0xff
	mov	x0, x19
	mov	x1, x20
	bl	_ungated_wrap
	cmp	w25, w0, uxtb
	cinc	w22, w22, ne
	add	w21, w21, #1
	cmp	w24, #200
	csinc	w20, w23, w20, eq
	b.ne	LBB4_7
	and	w8, w19, #0xff
	cmp	w8, #200
	csinc	w19, w23, w19, eq
	b.eq	LBB4_10
	and	w8, w19, #0xff
	cmp	w8, #200
	b.ls	LBB4_6
LBB4_10:
	stp	w22, w21, [sp, #16]
Lloh2:
	adrp	x19, l_anon.4bb7a5b5488537302fe6f1f624dc55e5.1@PAGE
Lloh3:
	add	x19, x19, l_anon.4bb7a5b5488537302fe6f1f624dc55e5.1@PAGEOFF
Lloh4:
	adrp	x20, __RNvXs8_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3impmNtB9_7Display3fmt@GOTPAGE
Lloh5:
	ldr	x20, [x20, __RNvXs8_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3impmNtB9_7Display3fmt@GOTPAGEOFF]
	stp	x19, x20, [sp, #24]
Lloh6:
	adrp	x0, l_anon.4bb7a5b5488537302fe6f1f624dc55e5.2@PAGE
Lloh7:
	add	x0, x0, l_anon.4bb7a5b5488537302fe6f1f624dc55e5.2@PAGEOFF
	add	x1, sp, #24
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	stp	x19, x20, [sp, #24]
Lloh8:
	adrp	x0, l_anon.4bb7a5b5488537302fe6f1f624dc55e5.3@PAGE
Lloh9:
	add	x0, x0, l_anon.4bb7a5b5488537302fe6f1f624dc55e5.3@PAGEOFF
	add	x1, sp, #24
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	add	x8, sp, #8
	stp	x8, x20, [sp, #24]
	add	x8, sp, #12
	stp	x8, x20, [sp, #40]
Lloh10:
	adrp	x0, l_anon.4bb7a5b5488537302fe6f1f624dc55e5.4@PAGE
Lloh11:
	add	x0, x0, l_anon.4bb7a5b5488537302fe6f1f624dc55e5.4@PAGEOFF
	add	x1, sp, #24
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	stp	x19, x20, [sp, #24]
Lloh12:
	adrp	x0, l_anon.4bb7a5b5488537302fe6f1f624dc55e5.5@PAGE
Lloh13:
	add	x0, x0, l_anon.4bb7a5b5488537302fe6f1f624dc55e5.5@PAGEOFF
	add	x1, sp, #24
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	add	x8, sp, #16
	stp	x8, x20, [sp, #24]
	add	x8, sp, #20
	stp	x8, x20, [sp, #40]
Lloh14:
	adrp	x0, l_anon.4bb7a5b5488537302fe6f1f624dc55e5.6@PAGE
Lloh15:
	add	x0, x0, l_anon.4bb7a5b5488537302fe6f1f624dc55e5.6@PAGEOFF
	add	x1, sp, #24
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh16:
	adrp	x0, l_anon.4bb7a5b5488537302fe6f1f624dc55e5.7@PAGE
Lloh17:
	add	x0, x0, l_anon.4bb7a5b5488537302fe6f1f624dc55e5.7@PAGEOFF
	mov	w1, #23
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	mov	w0, #100
	mov	w1, #100
	bl	_proved_sat
	strb	w0, [sp, #63]
	add	x19, sp, #63
Lloh18:
	adrp	x20, __RNvXNtNtNtCs5dyeT9KiOLK_4core3fmt3num3imphNtB6_7Display3fmt@GOTPAGE
Lloh19:
	ldr	x20, [x20, __RNvXNtNtNtCs5dyeT9KiOLK_4core3fmt3num3imphNtB6_7Display3fmt@GOTPAGEOFF]
	stp	x19, x20, [sp, #24]
Lloh20:
	adrp	x0, l_anon.4bb7a5b5488537302fe6f1f624dc55e5.8@PAGE
Lloh21:
	add	x0, x0, l_anon.4bb7a5b5488537302fe6f1f624dc55e5.8@PAGEOFF
	add	x1, sp, #24
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	mov	w0, #100
	mov	w1, #100
	bl	_proved_sat
	strb	w0, [sp, #63]
	stp	x19, x20, [sp, #24]
Lloh22:
	adrp	x0, l_anon.4bb7a5b5488537302fe6f1f624dc55e5.9@PAGE
Lloh23:
	add	x0, x0, l_anon.4bb7a5b5488537302fe6f1f624dc55e5.9@PAGEOFF
	add	x1, sp, #24
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	mov	w0, #150
	mov	w1, #150
	bl	_ungated_sat
	strb	w0, [sp, #63]
	stp	x19, x20, [sp, #24]
Lloh24:
	adrp	x0, l_anon.4bb7a5b5488537302fe6f1f624dc55e5.10@PAGE
Lloh25:
	add	x0, x0, l_anon.4bb7a5b5488537302fe6f1f624dc55e5.10@PAGEOFF
	add	x1, sp, #24
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	mov	w0, #150
	mov	w1, #150
	bl	_ungated_wrap
	strb	w0, [sp, #63]
	stp	x19, x20, [sp, #24]
Lloh26:
	adrp	x0, l_anon.4bb7a5b5488537302fe6f1f624dc55e5.11@PAGE
Lloh27:
	add	x0, x0, l_anon.4bb7a5b5488537302fe6f1f624dc55e5.11@PAGEOFF
	add	x1, sp, #24
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	.cfi_def_cfa wsp, 144
	ldp	x29, x30, [sp, #128]
	ldp	x20, x19, [sp, #112]
	ldp	x22, x21, [sp, #96]
	ldp	x24, x23, [sp, #80]
	ldp	x26, x25, [sp, #64]
	add	sp, sp, #144
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w21
	.cfi_restore w22
	.cfi_restore w23
	.cfi_restore w24
	.cfi_restore w25
	.cfi_restore w26
	ret
	.loh AdrpAdd	Lloh26, Lloh27
	.loh AdrpAdd	Lloh24, Lloh25
	.loh AdrpAdd	Lloh22, Lloh23
	.loh AdrpAdd	Lloh20, Lloh21
	.loh AdrpLdrGot	Lloh18, Lloh19
	.loh AdrpAdd	Lloh16, Lloh17
	.loh AdrpAdd	Lloh14, Lloh15
	.loh AdrpAdd	Lloh12, Lloh13
	.loh AdrpAdd	Lloh10, Lloh11
	.loh AdrpAdd	Lloh8, Lloh9
	.loh AdrpAdd	Lloh6, Lloh7
	.loh AdrpLdrGot	Lloh4, Lloh5
	.loh AdrpAdd	Lloh2, Lloh3
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
	mov	w9, #200
	cmp	w8, #200
	csel	w0, w8, w9, lo
	ret
	.cfi_endproc

	.globl	_ungated_wrap
	.p2align	2
_ungated_wrap:
	.cfi_startproc
	and	w8, w1, #0xff
	add	w8, w8, w0, uxtb
	mov	w9, #653
	mul	w9, w8, w9
	lsr	w9, w9, #17
	mov	w10, #201
	msub	w0, w9, w10, w8
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
Lloh28:
	adrp	x8, __RNvCsbhamypr2ijH_41p7b_a_declared_range_inside_the_container4main@PAGE
Lloh29:
	add	x8, x8, __RNvCsbhamypr2ijH_41p7b_a_declared_range_inside_the_container4main@PAGEOFF
	str	x8, [sp, #8]
Lloh30:
	adrp	x1, l_anon.4bb7a5b5488537302fe6f1f624dc55e5.0@PAGE
Lloh31:
	add	x1, x1, l_anon.4bb7a5b5488537302fe6f1f624dc55e5.0@PAGEOFF
	add	x0, sp, #8
	mov	w4, #0
	bl	__RNvNtCsl82qo6vb64n_3std2rt19lang_start_internal
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
	.loh AdrpAdd	Lloh30, Lloh31
	.loh AdrpAdd	Lloh28, Lloh29
	.cfi_endproc

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.4bb7a5b5488537302fe6f1f624dc55e5.0:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__RNSNvYNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0INtNtNtCs5dyeT9KiOLK_4core3ops8function6FnOnceuE9call_once6vtableCsbhamypr2ijH_41p7b_a_declared_range_inside_the_container
	.quad	__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0Csbhamypr2ijH_41p7b_a_declared_range_inside_the_container
	.quad	__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0Csbhamypr2ijH_41p7b_a_declared_range_inside_the_container

	.section	__TEXT,__literal4,4byte_literals
	.p2align	2, 0x0
l_anon.4bb7a5b5488537302fe6f1f624dc55e5.1:
	.asciz	"\310\000\000"

	.section	__TEXT,__cstring,cstring_literals
l_anon.4bb7a5b5488537302fe6f1f624dc55e5.2:
	.asciz	"\033declared logical range 0..=\300\017, container u8\n"

l_anon.4bb7a5b5488537302fe6f1f624dc55e5.3:
	.asciz	"-proved   (operands <= 100, propagated 200 <= \300\003):\n"

l_anon.4bb7a5b5488537302fe6f1f624dc55e5.4:
	.asciz	"\030  sat and wrap agree on \300\004 of \300\007 pairs\n"

l_anon.4bb7a5b5488537302fe6f1f624dc55e5.5:
	.asciz	"-unproved (operands <= 200, propagated 400 >  \300\003):\n"

l_anon.4bb7a5b5488537302fe6f1f624dc55e5.6:
	.asciz	"\031  sat and wrap differ on \300\004 of \300\007 pairs\n"

	.section	__TEXT,__const
l_anon.4bb7a5b5488537302fe6f1f624dc55e5.7:
	.ascii	"witnesses:\n"

	.section	__TEXT,__cstring,cstring_literals
l_anon.4bb7a5b5488537302fe6f1f624dc55e5.8:
	.asciz	"\033  proved_sat(100,100)    = \300\001\n"

l_anon.4bb7a5b5488537302fe6f1f624dc55e5.9:
	.asciz	"\033  proved_wrap(100,100)   = \300\001\n"

l_anon.4bb7a5b5488537302fe6f1f624dc55e5.10:
	.asciz	"\033  unproved_sat(150,150)  = \300\001\n"

l_anon.4bb7a5b5488537302fe6f1f624dc55e5.11:
	.asciz	"\033  unproved_wrap(150,150) = \300\001\n"

	.globl	_unproved_wrap
_unproved_wrap = _ungated_wrap
	.globl	_proved_wrap
_proved_wrap = _proved_sat
	.globl	_unproved_sat
_unproved_sat = _ungated_sat
.subsections_via_symbols
