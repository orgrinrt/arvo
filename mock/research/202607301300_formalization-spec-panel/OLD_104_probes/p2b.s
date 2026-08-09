	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.private_extern	__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECs7BzmVNPi1At_16p2b_binding_time
	.globl	__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECs7BzmVNPi1At_16p2b_binding_time
	.p2align	2
__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECs7BzmVNPi1At_16p2b_binding_time:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x4, x3
	mov	x3, x2
	mov	x2, x1
	str	x0, [sp, #8]
Lloh0:
	adrp	x1, l_anon.c9f06586f205e05d49884cce32b8d83b.0@PAGE
Lloh1:
	add	x1, x1, l_anon.c9f06586f205e05d49884cce32b8d83b.0@PAGEOFF
	add	x0, sp, #8
	bl	__RNvNtCsl82qo6vb64n_3std2rt19lang_start_internal
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
	.loh AdrpAdd	Lloh0, Lloh1

	.p2align	2
__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs7BzmVNPi1At_16p2b_binding_time:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	blr	x0
	; InlineAsm Start
	; InlineAsm End
	ldp	x29, x30, [sp], #16
	ret

	.p2align	2
__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0Cs7BzmVNPi1At_16p2b_binding_time:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	ldr	x0, [x0]
	bl	__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs7BzmVNPi1At_16p2b_binding_time
	mov	w0, #0
	ldp	x29, x30, [sp], #16
	ret

	.p2align	2
__RNSNvYNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0INtNtNtCs5dyeT9KiOLK_4core3ops8function6FnOnceuE9call_once6vtableCs7BzmVNPi1At_16p2b_binding_time:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	ldr	x0, [x0]
	bl	__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs7BzmVNPi1At_16p2b_binding_time
	mov	w0, #0
	ldp	x29, x30, [sp], #16
	ret

	.private_extern	__RNvCs7BzmVNPi1At_16p2b_binding_time4main
	.globl	__RNvCs7BzmVNPi1At_16p2b_binding_time4main
	.p2align	2
__RNvCs7BzmVNPi1At_16p2b_binding_time4main:
	.cfi_startproc
	sub	sp, sp, #128
	stp	x20, x19, [sp, #96]
	stp	x29, x30, [sp, #112]
	add	x29, sp, #112
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	bl	__RNvCske4UNIzLImn_7___rustc35___rust_no_alloc_shim_is_unstable_v2
	mov	w0, #6672
	mov	w1, #1
	bl	__RNvCske4UNIzLImn_7___rustc12___rust_alloc
	cbz	x0, LBB4_2
	mov	x19, x0
	mov	w1, #165
	mov	w2, #6672
	bl	_memset
	mov	x0, x19
	mov	w1, #6672
	bl	__RNvCs7BzmVNPi1At_16p2b_binding_time9const_two
	str	x0, [sp]
	mov	x0, x19
	mov	w1, #6672
	bl	__RNvCs7BzmVNPi1At_16p2b_binding_time9const_one
	str	x0, [sp, #8]
	mov	x0, x19
	mov	w1, #6672
	bl	__RNvCs7BzmVNPi1At_16p2b_binding_time7dyn_two
	str	x0, [sp, #16]
	mov	x0, x19
	mov	w1, #6672
	bl	__RNvCs7BzmVNPi1At_16p2b_binding_time7dyn_one
	str	x0, [sp, #24]
Lloh2:
	adrp	x8, __RNvXsd_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3impyNtB9_7Display3fmt@GOTPAGE
Lloh3:
	ldr	x8, [x8, __RNvXsd_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3impyNtB9_7Display3fmt@GOTPAGEOFF]
	mov	x9, sp
	stp	x9, x8, [sp, #32]
	add	x9, sp, #8
	stp	x9, x8, [sp, #48]
	add	x9, sp, #16
	stp	x9, x8, [sp, #64]
	add	x9, sp, #24
	stp	x9, x8, [sp, #80]
Lloh4:
	adrp	x0, l_anon.c9f06586f205e05d49884cce32b8d83b.1@PAGE
Lloh5:
	add	x0, x0, l_anon.c9f06586f205e05d49884cce32b8d83b.1@PAGEOFF
	add	x1, sp, #32
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	mov	x0, x19
	mov	w1, #6672
	mov	w2, #1
	bl	__RNvCske4UNIzLImn_7___rustc14___rust_dealloc
	ldp	x29, x30, [sp, #112]
	ldp	x20, x19, [sp, #96]
	add	sp, sp, #128
	ret
LBB4_2:
	mov	w0, #1
	mov	w1, #6672
	bl	__RNvNtCseduYQEDYcHM_5alloc7raw_vec12handle_error
	.loh AdrpAdd	Lloh4, Lloh5
	.loh AdrpLdrGot	Lloh2, Lloh3
	.cfi_endproc

	.p2align	2
__RNvCs7BzmVNPi1At_16p2b_binding_time7dyn_one:
	.cfi_startproc
	mov	x2, x1
	mov	x8, x0
	mov	x0, #0
	mov	w10, #3
	mov	w11, #53251
LBB5_1:
	lsr	x9, x10, #3
	add	x1, x9, #8
	cmp	x1, x2
	b.hi	LBB5_4
	ldr	x9, [x8, x9]
	and	x12, x10, #0x7
	lsr	x9, x9, x12
	and	x9, x9, #0x1f
	add	x0, x9, x0
	add	x10, x10, #13
	cmp	x10, x11
	b.ne	LBB5_1
	ret
LBB5_4:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
Lloh6:
	adrp	x3, l_anon.c9f06586f205e05d49884cce32b8d83b.3@PAGE
Lloh7:
	add	x3, x3, l_anon.c9f06586f205e05d49884cce32b8d83b.3@PAGEOFF
	mov	x0, x9
	bl	__RNvNtNtCs5dyeT9KiOLK_4core5slice5index16slice_index_fail
	.loh AdrpAdd	Lloh6, Lloh7
	.cfi_endproc

	.p2align	2
__RNvCs7BzmVNPi1At_16p2b_binding_time7dyn_two:
	.cfi_startproc
	mov	x2, x1
	mov	x10, #0
	mov	x8, #0
LBB6_1:
	lsr	x9, x10, #3
	add	x1, x9, #8
	cmp	x1, x2
	b.hi	LBB6_4
	ldr	x9, [x0, x9]
	and	x11, x10, #0x7
	lsr	x9, x9, x11
	ubfx	x9, x9, #3, #5
	add	x8, x9, x8
	add	x10, x10, #13
	cmp	x10, #13, lsl #12
	b.ne	LBB6_1
	mov	x0, x8
	ret
LBB6_4:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
Lloh8:
	adrp	x3, l_anon.c9f06586f205e05d49884cce32b8d83b.4@PAGE
Lloh9:
	add	x3, x3, l_anon.c9f06586f205e05d49884cce32b8d83b.4@PAGEOFF
	mov	x0, x9
	bl	__RNvNtNtCs5dyeT9KiOLK_4core5slice5index16slice_index_fail
	.loh AdrpAdd	Lloh8, Lloh9
	.cfi_endproc

	.p2align	2
__RNvCs7BzmVNPi1At_16p2b_binding_time9const_one:
	.cfi_startproc
	mov	x2, x1
	mov	x8, x0
	mov	x0, #0
	mov	w10, #3
	mov	w11, #53251
LBB7_1:
	lsr	x9, x10, #3
	add	x1, x9, #8
	cmp	x1, x2
	b.hi	LBB7_4
	ldr	x9, [x8, x9]
	and	x12, x10, #0x7
	lsr	x9, x9, x12
	and	x9, x9, #0x1f
	add	x0, x9, x0
	add	x10, x10, #13
	cmp	x10, x11
	b.ne	LBB7_1
	ret
LBB7_4:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
Lloh10:
	adrp	x3, l_anon.c9f06586f205e05d49884cce32b8d83b.5@PAGE
Lloh11:
	add	x3, x3, l_anon.c9f06586f205e05d49884cce32b8d83b.5@PAGEOFF
	mov	x0, x9
	bl	__RNvNtNtCs5dyeT9KiOLK_4core5slice5index16slice_index_fail
	.loh AdrpAdd	Lloh10, Lloh11
	.cfi_endproc

	.p2align	2
__RNvCs7BzmVNPi1At_16p2b_binding_time9const_two:
	.cfi_startproc
	mov	x2, x1
	mov	x10, #0
	mov	x8, #0
LBB8_1:
	lsr	x9, x10, #3
	add	x1, x9, #8
	cmp	x1, x2
	b.hi	LBB8_4
	ldr	x9, [x0, x9]
	and	x11, x10, #0x7
	lsr	x9, x9, x11
	ubfx	x9, x9, #3, #5
	add	x8, x9, x8
	add	x10, x10, #13
	cmp	x10, #13, lsl #12
	b.ne	LBB8_1
	mov	x0, x8
	ret
LBB8_4:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
Lloh12:
	adrp	x3, l_anon.c9f06586f205e05d49884cce32b8d83b.6@PAGE
Lloh13:
	add	x3, x3, l_anon.c9f06586f205e05d49884cce32b8d83b.6@PAGEOFF
	mov	x0, x9
	bl	__RNvNtNtCs5dyeT9KiOLK_4core5slice5index16slice_index_fail
	.loh AdrpAdd	Lloh12, Lloh13
	.cfi_endproc

	.globl	_main
	.p2align	2
_main:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	mov	x3, x1
	sxtw	x2, w0
Lloh14:
	adrp	x8, __RNvCs7BzmVNPi1At_16p2b_binding_time4main@PAGE
Lloh15:
	add	x8, x8, __RNvCs7BzmVNPi1At_16p2b_binding_time4main@PAGEOFF
	str	x8, [sp, #8]
Lloh16:
	adrp	x1, l_anon.c9f06586f205e05d49884cce32b8d83b.0@PAGE
Lloh17:
	add	x1, x1, l_anon.c9f06586f205e05d49884cce32b8d83b.0@PAGEOFF
	add	x0, sp, #8
	mov	w4, #0
	bl	__RNvNtCsl82qo6vb64n_3std2rt19lang_start_internal
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
	.loh AdrpAdd	Lloh16, Lloh17
	.loh AdrpAdd	Lloh14, Lloh15

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.c9f06586f205e05d49884cce32b8d83b.0:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__RNSNvYNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0INtNtNtCs5dyeT9KiOLK_4core3ops8function6FnOnceuE9call_once6vtableCs7BzmVNPi1At_16p2b_binding_time
	.quad	__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0Cs7BzmVNPi1At_16p2b_binding_time
	.quad	__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0Cs7BzmVNPi1At_16p2b_binding_time

	.section	__TEXT,__cstring,cstring_literals
l_anon.c9f06586f205e05d49884cce32b8d83b.1:
	.asciz	"\300\001 \300\001 \300\001 \300\001\n"

l_anon.c9f06586f205e05d49884cce32b8d83b.2:
	.asciz	"p2b_binding_time.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.c9f06586f205e05d49884cce32b8d83b.3:
	.quad	l_anon.c9f06586f205e05d49884cce32b8d83b.2
	.asciz	"\023\000\000\000\000\000\000\000=\000\000\000*\000\000"

	.p2align	3, 0x0
l_anon.c9f06586f205e05d49884cce32b8d83b.4:
	.quad	l_anon.c9f06586f205e05d49884cce32b8d83b.2
	.asciz	"\023\000\000\000\000\000\000\0001\000\000\000*\000\000"

	.p2align	3, 0x0
l_anon.c9f06586f205e05d49884cce32b8d83b.5:
	.quad	l_anon.c9f06586f205e05d49884cce32b8d83b.2
	.asciz	"\023\000\000\000\000\000\000\000&\000\000\000*\000\000"

	.p2align	3, 0x0
l_anon.c9f06586f205e05d49884cce32b8d83b.6:
	.quad	l_anon.c9f06586f205e05d49884cce32b8d83b.2
	.asciz	"\023\000\000\000\000\000\000\000\032\000\000\000*\000\000"

.subsections_via_symbols
