	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__RINvCsi64w8O2WatP_11p5_open_set3mulNtB2_13LibraryPresetEB2_:
	.cfi_startproc
	mov	w2, #1
	b	__RNvCsi64w8O2WatP_11p5_open_set8arm_fast
	.cfi_endproc

	.p2align	2
__RINvCsi64w8O2WatP_11p5_open_set3mulNtNtB2_8consumer10MyStrategyEB2_:
	.cfi_startproc
	mov	w2, #1
	b	__RNvCsi64w8O2WatP_11p5_open_set9arm_small
	.cfi_endproc

	.private_extern	__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECsi64w8O2WatP_11p5_open_set
	.globl	__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECsi64w8O2WatP_11p5_open_set
	.p2align	2
__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECsi64w8O2WatP_11p5_open_set:
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
	adrp	x1, l_anon.8fcf1906bc939229ee174fe0c565eda2.0@PAGE
Lloh1:
	add	x1, x1, l_anon.8fcf1906bc939229ee174fe0c565eda2.0@PAGEOFF
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
__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECsi64w8O2WatP_11p5_open_set:
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
__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0Csi64w8O2WatP_11p5_open_set:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECsi64w8O2WatP_11p5_open_set
	mov	w0, #0
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__RNSNvYNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0INtNtNtCs5dyeT9KiOLK_4core3ops8function6FnOnceuE9call_once6vtableCsi64w8O2WatP_11p5_open_set:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECsi64w8O2WatP_11p5_open_set
	mov	w0, #0
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__RNvCsi64w8O2WatP_11p5_open_set20mul_runtime_selected:
	.cfi_startproc
	tbz	w2, #0, LBB6_2
	mov	w2, #1
	b	__RNvCsi64w8O2WatP_11p5_open_set8arm_fast
LBB6_2:
	mov	w2, #1
	b	__RNvCsi64w8O2WatP_11p5_open_set9arm_small
	.cfi_endproc

	.private_extern	__RNvCsi64w8O2WatP_11p5_open_set4main
	.globl	__RNvCsi64w8O2WatP_11p5_open_set4main
	.p2align	2
__RNvCsi64w8O2WatP_11p5_open_set4main:
	.cfi_startproc
	sub	sp, sp, #208
	.cfi_def_cfa_offset 208
	stp	x28, x27, [sp, #112]
	stp	x26, x25, [sp, #128]
	stp	x24, x23, [sp, #144]
	stp	x22, x21, [sp, #160]
	stp	x20, x19, [sp, #176]
	stp	x29, x30, [sp, #192]
	add	x29, sp, #192
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
	.cfi_remember_state
Lloh2:
	adrp	x0, l_anon.8fcf1906bc939229ee174fe0c565eda2.1@PAGE
Lloh3:
	add	x0, x0, l_anon.8fcf1906bc939229ee174fe0c565eda2.1@PAGEOFF
	mov	w1, #155
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	mov	x24, #0
	mov	x8, #0
	mov	x25, #0
	mov	w19, #-128
LBB7_1:
	mov	w20, #-128
LBB7_2:
	mov	x26, x8
	mov	x0, x19
	mov	x1, x20
	mov	w2, #0
	bl	__RNvCsi64w8O2WatP_11p5_open_set8arm_fast
	mov	x21, x0
	mov	x0, x19
	mov	x1, x20
	mov	w2, #0
	bl	__RNvCsi64w8O2WatP_11p5_open_set9arm_small
	mov	x22, x0
	cmp	w21, w0
	cinc	x27, x26, ne
	cmp	w21, #0
	cinc	x25, x25, ne
	mov	x0, x19
	mov	x1, x20
	mov	w2, #1
	bl	__RNvCsi64w8O2WatP_11p5_open_set8arm_fast
	mov	x23, x0
	mov	x0, x19
	mov	x1, x20
	mov	w2, #1
	bl	__RNvCsi64w8O2WatP_11p5_open_set9arm_small
	cmp	w23, w0
	cinc	x8, x27, ne
	cmp	w23, #0
	cinc	x25, x25, ne
	add	x24, x24, #2
	cmp	w20, #127
	cinc	w20, w20, ne
	b.ne	LBB7_2
	cmp	w19, #127
	cinc	w19, w19, ne
	b.eq	LBB7_5
	cmp	w19, #127
	b.le	LBB7_1
LBB7_5:
	cmp	w21, w22
	stp	x25, x24, [sp, #16]
	cinc	x9, x26, ne
	cmp	w23, w0
	str	x8, [sp, #8]
	cinc	x8, x9, ne
	sub	x8, x24, x8
	str	x8, [sp, #32]
Lloh4:
	adrp	x8, __RNvXsd_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3impyNtB9_7Display3fmt@GOTPAGE
Lloh5:
	ldr	x8, [x8, __RNvXsd_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3impyNtB9_7Display3fmt@GOTPAGEOFF]
	add	x9, sp, #32
	stp	x9, x8, [sp, #40]
	add	x9, sp, #24
	stp	x9, x8, [sp, #56]
	add	x9, sp, #8
	stp	x9, x8, [sp, #72]
	add	x9, sp, #16
	stp	x9, x8, [sp, #88]
Lloh6:
	adrp	x0, l_anon.8fcf1906bc939229ee174fe0c565eda2.2@PAGE
Lloh7:
	add	x0, x0, l_anon.8fcf1906bc939229ee174fe0c565eda2.2@PAGEOFF
	add	x1, sp, #40
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	ldr	x8, [sp, #8]
	cbnz	x8, LBB7_8
	ldr	x8, [sp, #16]
	cbz	x8, LBB7_8
Lloh8:
	adrp	x0, l_anon.8fcf1906bc939229ee174fe0c565eda2.3@PAGE
Lloh9:
	add	x0, x0, l_anon.8fcf1906bc939229ee174fe0c565eda2.3@PAGEOFF
	mov	w1, #155
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	mov	w0, #100
	mov	w1, #100
	bl	_call_library_preset
	stur	w0, [x29, #-88]
	mov	w0, #100
	mov	w1, #100
	bl	_call_consumer_strategy
	stur	w0, [x29, #-84]
	mov	w0, #100
	mov	w1, #100
	mov	w2, #1
	bl	_call_runtime_selected
	str	w0, [sp, #32]
	sub	x8, x29, #88
Lloh10:
	adrp	x9, __RNvXs9_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3implNtB9_7Display3fmt@GOTPAGE
Lloh11:
	ldr	x9, [x9, __RNvXs9_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3implNtB9_7Display3fmt@GOTPAGEOFF]
	stp	x8, x9, [sp, #40]
	sub	x8, x29, #84
	stp	x8, x9, [sp, #56]
	add	x8, sp, #32
	stp	x8, x9, [sp, #72]
Lloh12:
	adrp	x0, l_anon.8fcf1906bc939229ee174fe0c565eda2.4@PAGE
Lloh13:
	add	x0, x0, l_anon.8fcf1906bc939229ee174fe0c565eda2.4@PAGEOFF
	add	x1, sp, #40
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh14:
	adrp	x0, l_anon.8fcf1906bc939229ee174fe0c565eda2.5@PAGE
Lloh15:
	add	x0, x0, l_anon.8fcf1906bc939229ee174fe0c565eda2.5@PAGEOFF
	mov	w1, #3
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh16:
	adrp	x0, l_anon.8fcf1906bc939229ee174fe0c565eda2.6@PAGE
Lloh17:
	add	x0, x0, l_anon.8fcf1906bc939229ee174fe0c565eda2.6@PAGEOFF
	mov	w1, #151
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	.cfi_def_cfa wsp, 208
	ldp	x29, x30, [sp, #192]
	ldp	x20, x19, [sp, #176]
	ldp	x22, x21, [sp, #160]
	ldp	x24, x23, [sp, #144]
	ldp	x26, x25, [sp, #128]
	ldp	x28, x27, [sp, #112]
	add	sp, sp, #208
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
LBB7_8:
	.cfi_restore_state
Lloh18:
	adrp	x0, l_anon.8fcf1906bc939229ee174fe0c565eda2.7@PAGE
Lloh19:
	add	x0, x0, l_anon.8fcf1906bc939229ee174fe0c565eda2.7@PAGEOFF
	mov	w1, #17
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	mov	w0, #1
	bl	__RNvNtCsl82qo6vb64n_3std7process4exit
	.loh AdrpAdd	Lloh2, Lloh3
	.loh AdrpAdd	Lloh6, Lloh7
	.loh AdrpLdrGot	Lloh4, Lloh5
	.loh AdrpAdd	Lloh16, Lloh17
	.loh AdrpAdd	Lloh14, Lloh15
	.loh AdrpAdd	Lloh12, Lloh13
	.loh AdrpLdrGot	Lloh10, Lloh11
	.loh AdrpAdd	Lloh8, Lloh9
	.loh AdrpAdd	Lloh18, Lloh19
	.cfi_endproc

	.p2align	2
__RNvCsi64w8O2WatP_11p5_open_set8arm_fast:
	.cfi_startproc
	smull	x8, w1, w0
	lsr	x8, x8, #3
	tbz	w2, #0, LBB8_2
	cmn	w8, #128
	mov	w9, #-128
	csel	w8, w8, w9, gt
	mov	w9, #127
	cmp	w8, #127
	csel	w0, w8, w9, lt
	ret
LBB8_2:
	and	w9, w8, #0x7f
	orr	w10, w8, #0xffffff00
	tst	w8, #0x80
	csel	w0, w9, w10, eq
	ret
	.cfi_endproc

	.p2align	2
__RNvCsi64w8O2WatP_11p5_open_set9arm_small:
	.cfi_startproc
	mov	x8, #0
	cbz	w0, LBB9_3
	cmp	w0, #0
	cneg	w9, w0, mi
	sxtw	x10, w1
LBB9_2:
	sbfx	x11, x9, #0, #1
	and	x11, x11, x10
	add	x8, x11, x8
	lsl	x10, x10, #1
	lsr	x9, x9, #1
	cbnz	x9, LBB9_2
LBB9_3:
	cmp	w0, #0
	cneg	x8, x8, mi
	lsr	x8, x8, #3
	tbz	w2, #0, LBB9_5
	cmn	w8, #128
	mov	w9, #-128
	csel	w8, w8, w9, gt
	mov	w9, #127
	cmp	w8, #127
	csel	w0, w8, w9, lt
	ret
LBB9_5:
	and	w9, w8, #0x7f
	orr	w10, w8, #0xffffff00
	tst	w8, #0x80
	csel	w0, w9, w10, eq
	ret
	.cfi_endproc

	.globl	_call_consumer_strategy
	.p2align	2
_call_consumer_strategy:
	.cfi_startproc
	b	__RINvCsi64w8O2WatP_11p5_open_set3mulNtNtB2_8consumer10MyStrategyEB2_
	.cfi_endproc

	.globl	_call_library_preset
	.p2align	2
_call_library_preset:
	.cfi_startproc
	b	__RINvCsi64w8O2WatP_11p5_open_set3mulNtB2_13LibraryPresetEB2_
	.cfi_endproc

	.globl	_call_runtime_selected
	.p2align	2
_call_runtime_selected:
	.cfi_startproc
	b	__RNvCsi64w8O2WatP_11p5_open_set20mul_runtime_selected
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
Lloh20:
	adrp	x8, __RNvCsi64w8O2WatP_11p5_open_set4main@PAGE
Lloh21:
	add	x8, x8, __RNvCsi64w8O2WatP_11p5_open_set4main@PAGEOFF
	str	x8, [sp, #8]
Lloh22:
	adrp	x1, l_anon.8fcf1906bc939229ee174fe0c565eda2.0@PAGE
Lloh23:
	add	x1, x1, l_anon.8fcf1906bc939229ee174fe0c565eda2.0@PAGEOFF
	add	x0, sp, #8
	mov	w4, #0
	bl	__RNvNtCsl82qo6vb64n_3std2rt19lang_start_internal
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
	.loh AdrpAdd	Lloh22, Lloh23
	.loh AdrpAdd	Lloh20, Lloh21
	.cfi_endproc

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.8fcf1906bc939229ee174fe0c565eda2.0:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__RNSNvYNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0INtNtNtCs5dyeT9KiOLK_4core3ops8function6FnOnceuE9call_once6vtableCsi64w8O2WatP_11p5_open_set
	.quad	__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0Csi64w8O2WatP_11p5_open_set
	.quad	__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0Csi64w8O2WatP_11p5_open_set

	.section	__TEXT,__const
l_anon.8fcf1906bc939229ee174fe0c565eda2.1:
	.ascii	"T1 the consumer-defined strategy compiles against an unchanged library: PASS\n"

	.section	__TEXT,__cstring,cstring_literals
l_anon.8fcf1906bc939229ee174fe0c565eda2.2:
	.asciz	"\031C3 the two arms agree on \300\004 of \300\027 inputs, disagreements=\300\022, nonzero results=\300\001\n"

	.section	__TEXT,__const
l_anon.8fcf1906bc939229ee174fe0c565eda2.3:
	.ascii	"C3 PASS: the arms are two routes to one answer, and the check is non-vacuous\n"

	.section	__TEXT,__cstring,cstring_literals
l_anon.8fcf1906bc939229ee174fe0c565eda2.4:
	.asciz	"\030sample outputs: library=\300\n consumer=\300\t runtime=\300\001\n"

	.section	__TEXT,__const
l_anon.8fcf1906bc939229ee174fe0c565eda2.5:
	.byte	10

l_anon.8fcf1906bc939229ee174fe0c565eda2.6:
	.ascii	"T2 and T3 are answered by the asm scan in p5_scan.sh, not by this program.\n"

	.section	__TEXT,__literal8,8byte_literals
l_anon.8fcf1906bc939229ee174fe0c565eda2.7:
	.ascii	"C3 FAIL\n"

.subsections_via_symbols
