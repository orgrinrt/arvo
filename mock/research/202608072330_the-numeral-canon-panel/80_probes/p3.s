	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__RINvCs9VZ2pobYgZo_19p3_select_and_erase10sel_staticKm0_EB2_:
	.cfi_startproc
	add	x8, x2, x1
	mul	x0, x8, x0
	ret
	.cfi_endproc

	.p2align	2
__RINvCs9VZ2pobYgZo_19p3_select_and_erase10sel_staticKm8_EB2_:
	.cfi_startproc
	mov	x8, #128
	madd	x9, x1, x0, x8
	madd	x8, x2, x0, x8
	asr	x8, x8, #8
	add	x0, x8, x9, asr #8
	ret
	.cfi_endproc

	.private_extern	__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECs9VZ2pobYgZo_19p3_select_and_erase
	.globl	__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECs9VZ2pobYgZo_19p3_select_and_erase
	.p2align	2
__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECs9VZ2pobYgZo_19p3_select_and_erase:
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
	adrp	x1, l_anon.397ce40a8b34ceb20792150f03600b0a.0@PAGE
Lloh1:
	add	x1, x1, l_anon.397ce40a8b34ceb20792150f03600b0a.0@PAGEOFF
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
__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs9VZ2pobYgZo_19p3_select_and_erase:
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
__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0Cs9VZ2pobYgZo_19p3_select_and_erase:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs9VZ2pobYgZo_19p3_select_and_erase
	mov	w0, #0
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__RNSNvYNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0INtNtNtCs5dyeT9KiOLK_4core3ops8function6FnOnceuE9call_once6vtableCs9VZ2pobYgZo_19p3_select_and_erase:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs9VZ2pobYgZo_19p3_select_and_erase
	mov	w0, #0
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__RNvCs9VZ2pobYgZo_19p3_select_and_erase11sel_dynamic:
	.cfi_startproc
	sub	w8, w3, #1
	mov	w9, #1
	lsl	x8, x9, x8
	madd	x9, x1, x0, x8
	asr	x9, x9, x3
	madd	x8, x2, x0, x8
	asr	x8, x8, x3
	add	x8, x9, x8
	add	x9, x2, x1
	mul	x9, x9, x0
	cmp	w3, #0
	csel	x0, x8, x9, ne
	ret
	.cfi_endproc

	.p2align	2
__RNvCs9VZ2pobYgZo_19p3_select_and_erase13sel_static_f0:
	.cfi_startproc
	b	__RINvCs9VZ2pobYgZo_19p3_select_and_erase10sel_staticKm0_EB2_
	.cfi_endproc

	.p2align	2
__RNvCs9VZ2pobYgZo_19p3_select_and_erase13sel_static_f8:
	.cfi_startproc
	b	__RINvCs9VZ2pobYgZo_19p3_select_and_erase10sel_staticKm8_EB2_
	.cfi_endproc

	.private_extern	__RNvCs9VZ2pobYgZo_19p3_select_and_erase4main
	.globl	__RNvCs9VZ2pobYgZo_19p3_select_and_erase4main
	.p2align	2
__RNvCs9VZ2pobYgZo_19p3_select_and_erase4main:
	.cfi_startproc
	sub	sp, sp, #240
	.cfi_def_cfa_offset 240
	stp	x28, x27, [sp, #144]
	stp	x26, x25, [sp, #160]
	stp	x24, x23, [sp, #176]
	stp	x22, x21, [sp, #192]
	stp	x20, x19, [sp, #208]
	stp	x29, x30, [sp, #224]
	add	x29, sp, #224
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
	mov	x19, #-4096
	movk	x19, #65529, lsl #16
	mov	x20, #-8192
	movk	x20, #65523, lsl #16
	mov	w21, #61568
	movk	w21, #505, lsl #16
	mov	w23, #61568
	movk	w23, #249, lsl #16
	mov	w8, #7
	str	x8, [sp]
	mov	x8, sp
	; InlineAsm Start
	; InlineAsm End
	mov	w8, #5
	str	x8, [sp, #8]
	add	x8, sp, #8
	; InlineAsm Start
	; InlineAsm End
	mov	w8, #3
	str	x8, [sp, #16]
	add	x8, sp, #16
	; InlineAsm Start
	; InlineAsm End
Lloh2:
	adrp	x0, l_anon.397ce40a8b34ceb20792150f03600b0a.1@PAGE
Lloh3:
	add	x0, x0, l_anon.397ce40a8b34ceb20792150f03600b0a.1@PAGEOFF
	mov	w1, #43
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	ldp	x8, x9, [sp]
	mul	x24, x9, x8
	ldr	x10, [sp, #16]
	mul	x25, x10, x8
	add	x9, x10, x9
	mul	x26, x9, x8
	add	x8, x25, x24
	cmp	x8, x26
	stp	x8, x26, [sp, #64]
	cset	w8, eq
	strb	w8, [sp, #56]
	add	x8, sp, #64
Lloh4:
	adrp	x28, __RNvXse_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3impxNtB9_7Display3fmt@GOTPAGE
Lloh5:
	ldr	x28, [x28, __RNvXse_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3impxNtB9_7Display3fmt@GOTPAGEOFF]
	stp	x8, x28, [sp, #80]
	add	x8, sp, #72
	stp	x8, x28, [sp, #96]
	add	x22, sp, #56
Lloh6:
	adrp	x27, __RNvXsg_NtCs5dyeT9KiOLK_4core3fmtbNtB5_7Display3fmt@GOTPAGE
Lloh7:
	ldr	x27, [x27, __RNvXsg_NtCs5dyeT9KiOLK_4core3fmtbNtB5_7Display3fmt@GOTPAGEOFF]
	stp	x22, x27, [sp, #112]
Lloh8:
	adrp	x0, l_anon.397ce40a8b34ceb20792150f03600b0a.2@PAGE
Lloh9:
	add	x0, x0, l_anon.397ce40a8b34ceb20792150f03600b0a.2@PAGEOFF
	add	x1, sp, #80
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	add	x8, x24, #128
	add	x9, x25, #128
	asr	x9, x9, #8
	add	x8, x9, x8, asr #8
	add	x9, x26, #128
	asr	x9, x9, #8
	stp	x8, x9, [sp, #64]
	cmp	x8, x9
	cset	w8, eq
	strb	w8, [sp, #56]
	add	x8, sp, #64
	stp	x8, x28, [sp, #80]
	add	x8, sp, #72
	stp	x8, x28, [sp, #96]
	stp	x22, x27, [sp, #112]
Lloh10:
	adrp	x0, l_anon.397ce40a8b34ceb20792150f03600b0a.3@PAGE
Lloh11:
	add	x0, x0, l_anon.397ce40a8b34ceb20792150f03600b0a.3@PAGEOFF
	add	x1, sp, #80
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	mov	x8, #0
	add	x9, x23, #97, lsl #12
	add	x10, x21, #97, lsl #12
	mov	x13, #-4096
	mov	w11, #4052
	mov	x12, #128
	mov	x14, #-64319
	movk	x14, #65524, lsl #16
	mov	w15, #9409
	mov	w16, #18818
	mov	x17, #-60223
	movk	x17, #65530, lsl #16
	mov	x0, #-8192
	movk	x0, #65523, lsl #16
	mov	x1, #-4096
	movk	x1, #65529, lsl #16
LBB9_1:
	madd	x2, x13, x11, x12
	asr	x2, x2, #8
	mov	x5, #-4096
	mov	x3, x10
	mov	x4, x21
LBB9_2:
	mov	x7, #0
	mov	x6, #0
	madd	x22, x5, x13, x12
	asr	x24, x22, #8
	mov	w25, #84
LBB9_3:
	add	x22, x9, x7
	add	x26, x23, x7
	add	x22, x24, x22, asr #8
	add	x26, x24, x26, asr #8
	add	x27, x3, x7
	add	x28, x4, x7
	cmp	x22, x27, asr #8
	cinc	x8, x8, ne
	cmp	x26, x28, asr #8
	cinc	x6, x6, ne
	add	x7, x7, x0
	subs	x25, x25, #2
	b.ne	LBB9_3
	add	x8, x6, x8
	add	x6, x2, x24
	add	x7, x5, #4052
	madd	x7, x7, x13, x12
	cmp	x6, x7, asr #8
	cinc	x8, x8, ne
	add	x4, x4, x1
	add	x3, x3, x1
	cmp	x5, #3999
	add	x5, x5, #97
	b.lt	LBB9_2
	add	x21, x21, x14
	add	x1, x1, x15
	add	x0, x0, x16
	add	x23, x23, x17
	add	x10, x10, x20
	add	x9, x9, x19
	cmp	x13, #3999
	add	x13, x13, #97
	b.lt	LBB9_1
	mov	w9, #24301
	movk	w9, #9, lsl #16
	stp	x9, x8, [sp, #24]
	str	xzr, [sp, #40]
Lloh12:
	adrp	x8, l_anon.397ce40a8b34ceb20792150f03600b0a.4@PAGE
Lloh13:
	add	x8, x8, l_anon.397ce40a8b34ceb20792150f03600b0a.4@PAGEOFF
	str	x8, [sp, #80]
Lloh14:
	adrp	x22, __RNvXse_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3impxNtB9_7Display3fmt@GOTPAGE
Lloh15:
	ldr	x22, [x22, __RNvXse_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3impxNtB9_7Display3fmt@GOTPAGEOFF]
	stp	x22, x8, [sp, #88]
Lloh16:
	adrp	x8, l_anon.397ce40a8b34ceb20792150f03600b0a.5@PAGE
Lloh17:
	add	x8, x8, l_anon.397ce40a8b34ceb20792150f03600b0a.5@PAGEOFF
	stp	x22, x8, [sp, #104]
	str	x22, [sp, #120]
	add	x19, sp, #24
Lloh18:
	adrp	x20, __RNvXsd_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3impyNtB9_7Display3fmt@GOTPAGE
Lloh19:
	ldr	x20, [x20, __RNvXsd_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3impyNtB9_7Display3fmt@GOTPAGEOFF]
	stp	x19, x20, [sp, #128]
Lloh20:
	adrp	x0, l_anon.397ce40a8b34ceb20792150f03600b0a.6@PAGE
Lloh21:
	add	x0, x0, l_anon.397ce40a8b34ceb20792150f03600b0a.6@PAGEOFF
	add	x1, sp, #80
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	add	x8, sp, #32
	stp	x8, x20, [sp, #80]
	stp	x19, x20, [sp, #96]
Lloh22:
	adrp	x0, l_anon.397ce40a8b34ceb20792150f03600b0a.7@PAGE
Lloh23:
	add	x0, x0, l_anon.397ce40a8b34ceb20792150f03600b0a.7@PAGEOFF
	add	x1, sp, #80
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	add	x8, sp, #40
	stp	x8, x20, [sp, #80]
	stp	x19, x20, [sp, #96]
Lloh24:
	adrp	x0, l_anon.397ce40a8b34ceb20792150f03600b0a.8@PAGE
Lloh25:
	add	x0, x0, l_anon.397ce40a8b34ceb20792150f03600b0a.8@PAGEOFF
	add	x1, sp, #80
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	ldp	x19, x20, [sp]
	ldr	x21, [sp, #16]
	mov	x0, x19
	mov	x1, x20
	mov	x2, x21
	bl	__RNvCs9VZ2pobYgZo_19p3_select_and_erase13sel_static_f0
	str	x0, [sp, #48]
	mov	x0, x19
	mov	x1, x20
	mov	x2, x21
	bl	__RNvCs9VZ2pobYgZo_19p3_select_and_erase13sel_static_f8
	str	x0, [sp, #56]
	str	wzr, [sp, #80]
	add	x8, sp, #80
	; InlineAsm Start
	; InlineAsm End
	ldp	x0, x1, [sp]
	ldr	x2, [sp, #16]
	ldr	w3, [sp, #80]
	bl	__RNvCs9VZ2pobYgZo_19p3_select_and_erase11sel_dynamic
	str	x0, [sp, #64]
	mov	w8, #8
	str	w8, [sp, #80]
	add	x8, sp, #80
	; InlineAsm Start
	; InlineAsm End
	ldp	x0, x1, [sp]
	ldr	x2, [sp, #16]
	ldr	w3, [sp, #80]
	bl	__RNvCs9VZ2pobYgZo_19p3_select_and_erase11sel_dynamic
	add	x8, sp, #48
	stp	x0, x8, [sp, #72]
	add	x8, sp, #56
	stp	x22, x8, [sp, #88]
	add	x8, sp, #64
	stp	x22, x8, [sp, #104]
	add	x8, sp, #72
	stp	x22, x8, [sp, #120]
	str	x22, [sp, #136]
Lloh26:
	adrp	x0, l_anon.397ce40a8b34ceb20792150f03600b0a.9@PAGE
Lloh27:
	add	x0, x0, l_anon.397ce40a8b34ceb20792150f03600b0a.9@PAGEOFF
	add	x1, sp, #80
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	.cfi_def_cfa wsp, 240
	ldp	x29, x30, [sp, #224]
	ldp	x20, x19, [sp, #208]
	ldp	x22, x21, [sp, #192]
	ldp	x24, x23, [sp, #176]
	ldp	x26, x25, [sp, #160]
	ldp	x28, x27, [sp, #144]
	add	sp, sp, #240
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
	.cfi_restore w27
	.cfi_restore w28
	ret
	.loh AdrpAdd	Lloh10, Lloh11
	.loh AdrpAdd	Lloh8, Lloh9
	.loh AdrpLdrGot	Lloh6, Lloh7
	.loh AdrpLdrGot	Lloh4, Lloh5
	.loh AdrpAdd	Lloh2, Lloh3
	.loh AdrpAdd	Lloh26, Lloh27
	.loh AdrpAdd	Lloh24, Lloh25
	.loh AdrpAdd	Lloh22, Lloh23
	.loh AdrpAdd	Lloh20, Lloh21
	.loh AdrpLdrGot	Lloh18, Lloh19
	.loh AdrpAdd	Lloh16, Lloh17
	.loh AdrpLdrGot	Lloh14, Lloh15
	.loh AdrpAdd	Lloh12, Lloh13
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
	adrp	x8, __RNvCs9VZ2pobYgZo_19p3_select_and_erase4main@PAGE
Lloh29:
	add	x8, x8, __RNvCs9VZ2pobYgZo_19p3_select_and_erase4main@PAGEOFF
	str	x8, [sp, #8]
Lloh30:
	adrp	x1, l_anon.397ce40a8b34ceb20792150f03600b0a.0@PAGE
Lloh31:
	add	x1, x1, l_anon.397ce40a8b34ceb20792150f03600b0a.0@PAGEOFF
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
l_anon.397ce40a8b34ceb20792150f03600b0a.0:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__RNSNvYNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0INtNtNtCs5dyeT9KiOLK_4core3ops8function6FnOnceuE9call_once6vtableCs9VZ2pobYgZo_19p3_select_and_erase
	.quad	__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0Cs9VZ2pobYgZo_19p3_select_and_erase
	.quad	__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0Cs9VZ2pobYgZo_19p3_select_and_erase

	.section	__TEXT,__const
l_anon.397ce40a8b34ceb20792150f03600b0a.1:
	.ascii	"p3: the select stage\n"

	.section	__TEXT,__cstring,cstring_literals
l_anon.397ce40a8b34ceb20792150f03600b0a.2:
	.asciz	"\020  F=0 : general=\300\007 fused=\300\007 agree=\300\001\n"

l_anon.397ce40a8b34ceb20792150f03600b0a.3:
	.asciz	"\020  F=8 : general=\300\007 fused=\300\007 agree=\300\001\n"

	.section	__TEXT,__literal8,8byte_literals
	.p2align	3, 0x0
l_anon.397ce40a8b34ceb20792150f03600b0a.4:
	.asciz	"\000\020\000\000\000\000\000"

	.p2align	3, 0x0
l_anon.397ce40a8b34ceb20792150f03600b0a.5:
	.asciz	"a\000\000\000\000\000\000"

	.section	__TEXT,__cstring,cstring_literals
l_anon.397ce40a8b34ceb20792150f03600b0a.6:
	.asciz	"\032  swept raw operands in [-\300\002, \300\007) step \300\002: \300\t triples\n"

l_anon.397ce40a8b34ceb20792150f03600b0a.7:
	.asciz	"'  at F=8 the two lowerings disagree on \300\004 of \300\001\n"

l_anon.397ce40a8b34ceb20792150f03600b0a.8:
	.asciz	"\032  at F=0 they disagree on \300\004 of \300\001\n"

l_anon.397ce40a8b34ceb20792150f03600b0a.9:
	.asciz	"\020  sel_static_f0=\300\017 sel_static_f8=\300\022 sel_dynamic(f=0)=\300\022 sel_dynamic(f=8)=\300\001\n"

.subsections_via_symbols
