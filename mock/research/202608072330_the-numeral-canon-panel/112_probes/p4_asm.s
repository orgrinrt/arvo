	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.private_extern	__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECsh38WkIC3yDD_48p4_where_a_missed_merge_actually_costs_something
	.globl	__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECsh38WkIC3yDD_48p4_where_a_missed_merge_actually_costs_something
	.p2align	2
__RINvNtCsl82qo6vb64n_3std2rt10lang_startuECsh38WkIC3yDD_48p4_where_a_missed_merge_actually_costs_something:
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
	adrp	x1, l_anon.4f9e1bf70eba26db74865750b4215fde.0@PAGE
Lloh1:
	add	x1, x1, l_anon.4f9e1bf70eba26db74865750b4215fde.0@PAGEOFF
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
__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECsh38WkIC3yDD_48p4_where_a_missed_merge_actually_costs_something:
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
__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0Csh38WkIC3yDD_48p4_where_a_missed_merge_actually_costs_something:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECsh38WkIC3yDD_48p4_where_a_missed_merge_actually_costs_something
	mov	w0, #0
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__RNSNvYNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0INtNtNtCs5dyeT9KiOLK_4core3ops8function6FnOnceuE9call_once6vtableCsh38WkIC3yDD_48p4_where_a_missed_merge_actually_costs_something:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__RINvNtNtCsl82qo6vb64n_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECsh38WkIC3yDD_48p4_where_a_missed_merge_actually_costs_something
	mov	w0, #0
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.private_extern	__RNvCsh38WkIC3yDD_48p4_where_a_missed_merge_actually_costs_something4main
	.globl	__RNvCsh38WkIC3yDD_48p4_where_a_missed_merge_actually_costs_something4main
	.p2align	2
__RNvCsh38WkIC3yDD_48p4_where_a_missed_merge_actually_costs_something4main:
Lfunc_begin0:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception0
	sub	sp, sp, #192
	.cfi_def_cfa_offset 192
	stp	x28, x27, [sp, #96]
	stp	x26, x25, [sp, #112]
	stp	x24, x23, [sp, #128]
	stp	x22, x21, [sp, #144]
	stp	x20, x19, [sp, #160]
	stp	x29, x30, [sp, #176]
	add	x29, sp, #176
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
	adrp	x0, l_anon.4f9e1bf70eba26db74865750b4215fde.1@PAGE
Lloh3:
	add	x0, x0, l_anon.4f9e1bf70eba26db74865750b4215fde.1@PAGEOFF
	mov	w1, #101
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	bl	__RNvCske4UNIzLImn_7___rustc35___rust_no_alloc_shim_is_unstable_v2
	mov	w0, #74
	mov	w1, #1
	bl	__RNvCske4UNIzLImn_7___rustc12___rust_alloc
	cbz	x0, LBB4_34
	mov	x8, #4340410370284600380
	orr	x8, x8, #0x1111111111111111
	stp	x8, x8, [x0]
	ldr	q0, [x0]
	stp	q0, q0, [x0, #16]
	str	q0, [x0, #48]
	str	x8, [x0, #64]
	mov	w8, #15677
	strh	w8, [x0, #72]
	mov	w8, #74
	stp	x8, x0, [sp, #64]
	str	x8, [sp, #80]
	add	x8, sp, #64
Lloh4:
	adrp	x9, __RNvXsq_NtCseduYQEDYcHM_5alloc6stringNtB5_6StringNtNtCs5dyeT9KiOLK_4core3fmt7Display3fmt@PAGE
Lloh5:
	add	x9, x9, __RNvXsq_NtCseduYQEDYcHM_5alloc6stringNtB5_6StringNtNtCs5dyeT9KiOLK_4core3fmt7Display3fmt@PAGEOFF
	stp	x8, x9, [sp]
Ltmp0:
Lloh6:
	adrp	x0, l_anon.4f9e1bf70eba26db74865750b4215fde.2@PAGE
Lloh7:
	add	x0, x0, l_anon.4f9e1bf70eba26db74865750b4215fde.2@PAGEOFF
	mov	x1, sp
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Ltmp1:
	ldr	x1, [sp, #64]
	cbz	x1, LBB4_4
	ldr	x0, [sp, #72]
	mov	w2, #1
	bl	__RNvCske4UNIzLImn_7___rustc14___rust_dealloc
LBB4_4:
Lloh8:
	adrp	x19, l_anon.4f9e1bf70eba26db74865750b4215fde.3@PAGE
Lloh9:
	add	x19, x19, l_anon.4f9e1bf70eba26db74865750b4215fde.3@PAGEOFF
	mov	x0, x19
	mov	w1, #3
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh10:
	adrp	x0, l_anon.4f9e1bf70eba26db74865750b4215fde.4@PAGE
Lloh11:
	add	x0, x0, l_anon.4f9e1bf70eba26db74865750b4215fde.4@PAGEOFF
	mov	w1, #45
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	mov	w22, #7
	strb	w22, [sp]
	mov	x20, sp
Lloh12:
	adrp	x21, __RNvXNtNtNtCs5dyeT9KiOLK_4core3fmt3num3imphNtB6_7Display3fmt@GOTPAGE
Lloh13:
	ldr	x21, [x21, __RNvXNtNtNtCs5dyeT9KiOLK_4core3fmt3num3imphNtB6_7Display3fmt@GOTPAGEOFF]
	stp	x20, x21, [sp, #64]
Lloh14:
	adrp	x0, l_anon.4f9e1bf70eba26db74865750b4215fde.5@PAGE
Lloh15:
	add	x0, x0, l_anon.4f9e1bf70eba26db74865750b4215fde.5@PAGEOFF
	add	x1, sp, #64
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	strb	w22, [sp]
	stp	x20, x21, [sp, #64]
Lloh16:
	adrp	x0, l_anon.4f9e1bf70eba26db74865750b4215fde.6@PAGE
Lloh17:
	add	x0, x0, l_anon.4f9e1bf70eba26db74865750b4215fde.6@PAGEOFF
	add	x1, sp, #64
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh18:
	adrp	x0, l_anon.4f9e1bf70eba26db74865750b4215fde.7@PAGE
Lloh19:
	add	x0, x0, l_anon.4f9e1bf70eba26db74865750b4215fde.7@PAGEOFF
	mov	w1, #129
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	mov	x0, x19
	mov	w1, #3
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh20:
	adrp	x0, l_anon.4f9e1bf70eba26db74865750b4215fde.8@PAGE
Lloh21:
	add	x0, x0, l_anon.4f9e1bf70eba26db74865750b4215fde.8@PAGEOFF
	mov	w1, #135
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh22:
	adrp	x0, l_anon.4f9e1bf70eba26db74865750b4215fde.9@PAGE
Lloh23:
	add	x0, x0, l_anon.4f9e1bf70eba26db74865750b4215fde.9@PAGEOFF
	mov	w1, #157
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	mov	w8, #15
	strb	w8, [sp, #56]
	mov	w8, #2
	strb	w8, [sp]
	add	x8, sp, #56
	stp	x8, x21, [sp, #64]
	stp	x20, x21, [sp, #80]
Lloh24:
	adrp	x0, l_anon.4f9e1bf70eba26db74865750b4215fde.10@PAGE
Lloh25:
	add	x0, x0, l_anon.4f9e1bf70eba26db74865750b4215fde.10@PAGEOFF
	add	x1, sp, #64
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh26:
	adrp	x0, l_anon.4f9e1bf70eba26db74865750b4215fde.11@PAGE
Lloh27:
	add	x0, x0, l_anon.4f9e1bf70eba26db74865750b4215fde.11@PAGEOFF
	mov	w1, #221
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	mov	x0, x19
	mov	w1, #3
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh28:
	adrp	x0, l_anon.4f9e1bf70eba26db74865750b4215fde.12@PAGE
Lloh29:
	add	x0, x0, l_anon.4f9e1bf70eba26db74865750b4215fde.12@PAGEOFF
	mov	w1, #127
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh30:
	adrp	x0, l_anon.4f9e1bf70eba26db74865750b4215fde.13@PAGE
Lloh31:
	add	x0, x0, l_anon.4f9e1bf70eba26db74865750b4215fde.13@PAGEOFF
	mov	w1, #169
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh32:
	adrp	x0, l_anon.4f9e1bf70eba26db74865750b4215fde.14@PAGE
Lloh33:
	add	x0, x0, l_anon.4f9e1bf70eba26db74865750b4215fde.14@PAGEOFF
	mov	w1, #173
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh34:
	adrp	x0, l_anon.4f9e1bf70eba26db74865750b4215fde.15@PAGE
Lloh35:
	add	x0, x0, l_anon.4f9e1bf70eba26db74865750b4215fde.15@PAGEOFF
	mov	w1, #145
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	mov	x0, x19
	mov	w1, #3
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh36:
	adrp	x0, l_anon.4f9e1bf70eba26db74865750b4215fde.16@PAGE
Lloh37:
	add	x0, x0, l_anon.4f9e1bf70eba26db74865750b4215fde.16@PAGEOFF
	mov	w1, #121
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	mov	x0, x19
	mov	w1, #3
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	mov	w9, #0
	mov	x8, #0
	mov	w10, #128
	dup.2d	v0, x10
LBB4_5:
	movi.2d	v1, #0000000000000000
	mov.d	v1[0], x8
	add.2d	v1, v1, v0
	addp.2d	d1, v1
	fmov	x8, d1
	cmp	w9, #255
	add	w9, w9, #1
	b.ne	LBB4_5
	str	x8, [sp, #16]
	str	x8, [sp]
Lloh38:
	adrp	x22, __RNvXsi_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3impjNtB9_7Display3fmt@GOTPAGE
Lloh39:
	ldr	x22, [x22, __RNvXsi_NtNtNtCs5dyeT9KiOLK_4core3fmt3num3impjNtB9_7Display3fmt@GOTPAGEOFF]
	mov	x8, sp
	stp	x8, x22, [sp, #64]
	add	x8, sp, #16
	stp	x8, x22, [sp, #80]
Lloh40:
	adrp	x0, l_anon.4f9e1bf70eba26db74865750b4215fde.17@PAGE
Lloh41:
	add	x0, x0, l_anon.4f9e1bf70eba26db74865750b4215fde.17@PAGEOFF
	add	x1, sp, #64
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	mov	x11, #0
	mov	w9, #0
	mov	x8, #0
	mov	w10, #15
LBB4_7:
	mov	w15, #0
LBB4_8:
	mov	x12, x11
	and	w16, w15, #0xff
	add	w11, w15, w9
	and	w13, w11, #0xff
	cmp	w13, #15
	csel	w13, w13, w10, lo
	and	w14, w11, #0xf
	cmp	w13, w14
	cinc	x11, x12, ne
	add	x8, x8, #1
	cmp	w16, #15
	csinc	w15, w16, w15, eq
	b.eq	LBB4_10
	and	w16, w15, #0xff
	cmp	w16, #15
	b.ls	LBB4_8
LBB4_10:
	and	w15, w9, #0xff
	cmp	w15, #15
	csinc	w9, w15, w9, eq
	b.eq	LBB4_12
	and	w15, w9, #0xff
	cmp	w15, #15
	b.ls	LBB4_7
LBB4_12:
	cmp	w13, w14
	str	x8, [sp, #24]
	cinc	x9, x12, ne
	sub	x8, x8, x9
	str	x8, [sp]
	mov	x8, sp
	stp	x8, x22, [sp, #64]
	add	x8, sp, #24
	stp	x8, x22, [sp, #80]
Lloh42:
	adrp	x0, l_anon.4f9e1bf70eba26db74865750b4215fde.18@PAGE
Lloh43:
	add	x0, x0, l_anon.4f9e1bf70eba26db74865750b4215fde.18@PAGEOFF
	add	x1, sp, #64
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	mov	w9, #0
	mov	x8, #0
LBB4_13:
	add	x8, x8, #1
	cmp	w9, #100
	cinc	w9, w9, ne
	b.eq	LBB4_15
	cmp	w9, #101
	b.lo	LBB4_13
LBB4_15:
	str	x8, [sp, #32]
	str	x8, [sp]
	mov	x8, sp
	stp	x8, x22, [sp, #64]
	add	x8, sp, #32
	stp	x8, x22, [sp, #80]
Lloh44:
	adrp	x0, l_anon.4f9e1bf70eba26db74865750b4215fde.19@PAGE
Lloh45:
	add	x0, x0, l_anon.4f9e1bf70eba26db74865750b4215fde.19@PAGEOFF
	add	x1, sp, #64
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh46:
	adrp	x0, l_anon.4f9e1bf70eba26db74865750b4215fde.3@PAGE
Lloh47:
	add	x0, x0, l_anon.4f9e1bf70eba26db74865750b4215fde.3@PAGEOFF
	mov	w1, #3
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh48:
	adrp	x0, l_anon.4f9e1bf70eba26db74865750b4215fde.20@PAGE
Lloh49:
	add	x0, x0, l_anon.4f9e1bf70eba26db74865750b4215fde.20@PAGEOFF
	mov	w1, #113
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	mov	w19, #0
	mov	x25, #0
	mov	x26, #0
LBB4_16:
	mov	w20, #0
LBB4_17:
	and	w23, w20, #0xff
	mov	x0, x19
	mov	x1, x20
	bl	_add_licensed_100_100
	and	w24, w0, #0xff
	mov	x0, x19
	mov	x1, x20
	bl	_add_general_u8
	cmp	w24, w0, uxtb
	cinc	x26, x26, ne
	add	x25, x25, #1
	cmp	w23, #100
	csinc	w20, w23, w20, eq
	b.ne	LBB4_17
	and	w8, w19, #0xff
	cmp	w8, #100
	csinc	w19, w8, w19, eq
	b.eq	LBB4_20
	and	w8, w19, #0xff
	cmp	w8, #100
	b.ls	LBB4_16
LBB4_20:
	mov	w19, #0
	mov	x23, #0
	mov	x24, #0
	stp	x26, x25, [sp, #40]
LBB4_21:
	and	w8, w19, #0xffff
	cmp	w8, #255
	csinc	w25, w8, w19, eq
	cmp	w8, #101
	b.hs	LBB4_28
	mov	w20, #0
	b	LBB4_26
LBB4_23:
	mov	w26, #255
LBB4_24:
	add	x23, x23, #1
	add	w8, w20, w19
	and	w27, w8, #0xff
	mov	x0, x19
	mov	x1, x20
	bl	_add_general_u8
	cmp	w27, w0, uxtb
	cinc	x24, x24, ne
LBB4_25:
	and	w8, w20, #0xffff
	mov	x20, x26
	cmp	w8, #255
	b.eq	LBB4_31
LBB4_26:
	and	w8, w20, #0xffff
	cmp	w8, #255
	b.eq	LBB4_23
	add	w26, w20, #1
	cmp	w8, #101
	b.hs	LBB4_24
	b	LBB4_25
LBB4_28:
	mov	w20, #0
LBB4_29:
	and	w26, w20, #0xffff
	add	w8, w20, w19
	and	w27, w8, #0xff
	mov	x0, x19
	mov	x1, x20
	bl	_add_general_u8
	cmp	w27, w0, uxtb
	cinc	x24, x24, ne
	add	x23, x23, #1
	cmp	w26, #255
	csinc	w20, w26, w20, eq
	b.eq	LBB4_31
	and	w8, w20, #0xffff
	cmp	w8, #255
	b.ls	LBB4_29
LBB4_31:
	and	w8, w19, #0xffff
	cmp	w8, #255
	b.eq	LBB4_33
	and	w8, w25, #0xffff
	mov	x19, x25
	cmp	w8, #255
	b.ls	LBB4_21
LBB4_33:
	str	x23, [sp]
	add	x8, sp, #40
	stp	x24, x8, [sp, #56]
	add	x8, sp, #48
	stp	x22, x8, [sp, #72]
	str	x22, [sp, #88]
Lloh50:
	adrp	x0, l_anon.4f9e1bf70eba26db74865750b4215fde.21@PAGE
Lloh51:
	add	x0, x0, l_anon.4f9e1bf70eba26db74865750b4215fde.21@PAGEOFF
	add	x1, sp, #64
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	add	x8, sp, #56
	stp	x8, x22, [sp, #64]
	mov	x8, sp
	stp	x8, x22, [sp, #80]
Lloh52:
	adrp	x0, l_anon.4f9e1bf70eba26db74865750b4215fde.22@PAGE
Lloh53:
	add	x0, x0, l_anon.4f9e1bf70eba26db74865750b4215fde.22@PAGEOFF
	add	x1, sp, #64
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh54:
	adrp	x0, l_anon.4f9e1bf70eba26db74865750b4215fde.3@PAGE
Lloh55:
	add	x0, x0, l_anon.4f9e1bf70eba26db74865750b4215fde.3@PAGEOFF
	mov	w1, #3
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh56:
	adrp	x8, l_anon.4f9e1bf70eba26db74865750b4215fde.23@PAGE
Lloh57:
	add	x8, x8, l_anon.4f9e1bf70eba26db74865750b4215fde.23@PAGEOFF
	stp	x8, x21, [sp, #64]
Lloh58:
	adrp	x0, l_anon.4f9e1bf70eba26db74865750b4215fde.24@PAGE
Lloh59:
	add	x0, x0, l_anon.4f9e1bf70eba26db74865750b4215fde.24@PAGEOFF
	add	x1, sp, #64
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
Lloh60:
	adrp	x8, l_anon.4f9e1bf70eba26db74865750b4215fde.25@PAGE
Lloh61:
	add	x8, x8, l_anon.4f9e1bf70eba26db74865750b4215fde.25@PAGEOFF
	stp	x8, x21, [sp, #64]
Lloh62:
	adrp	x0, l_anon.4f9e1bf70eba26db74865750b4215fde.26@PAGE
Lloh63:
	add	x0, x0, l_anon.4f9e1bf70eba26db74865750b4215fde.26@PAGEOFF
	add	x1, sp, #64
	bl	__RNvNtNtCsl82qo6vb64n_3std2io5stdio6__print
	.cfi_def_cfa wsp, 192
	ldp	x29, x30, [sp, #176]
	ldp	x20, x19, [sp, #160]
	ldp	x22, x21, [sp, #144]
	ldp	x24, x23, [sp, #128]
	ldp	x26, x25, [sp, #112]
	ldp	x28, x27, [sp, #96]
	add	sp, sp, #192
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
LBB4_34:
	.cfi_restore_state
	mov	w0, #1
	mov	w1, #74
	bl	__RNvNtCseduYQEDYcHM_5alloc7raw_vec12handle_error
LBB4_35:
Ltmp2:
	mov	x19, x0
	ldr	x1, [sp, #64]
	cbz	x1, LBB4_37
	ldr	x0, [sp, #72]
	mov	w2, #1
	bl	__RNvCske4UNIzLImn_7___rustc14___rust_dealloc
LBB4_37:
	mov	x0, x19
	bl	__Unwind_Resume
	.loh AdrpAdd	Lloh2, Lloh3
	.loh AdrpAdd	Lloh6, Lloh7
	.loh AdrpAdd	Lloh4, Lloh5
	.loh AdrpAdd	Lloh36, Lloh37
	.loh AdrpAdd	Lloh34, Lloh35
	.loh AdrpAdd	Lloh32, Lloh33
	.loh AdrpAdd	Lloh30, Lloh31
	.loh AdrpAdd	Lloh28, Lloh29
	.loh AdrpAdd	Lloh26, Lloh27
	.loh AdrpAdd	Lloh24, Lloh25
	.loh AdrpAdd	Lloh22, Lloh23
	.loh AdrpAdd	Lloh20, Lloh21
	.loh AdrpAdd	Lloh18, Lloh19
	.loh AdrpAdd	Lloh16, Lloh17
	.loh AdrpAdd	Lloh14, Lloh15
	.loh AdrpLdrGot	Lloh12, Lloh13
	.loh AdrpAdd	Lloh10, Lloh11
	.loh AdrpAdd	Lloh8, Lloh9
	.loh AdrpAdd	Lloh40, Lloh41
	.loh AdrpLdrGot	Lloh38, Lloh39
	.loh AdrpAdd	Lloh42, Lloh43
	.loh AdrpAdd	Lloh48, Lloh49
	.loh AdrpAdd	Lloh46, Lloh47
	.loh AdrpAdd	Lloh44, Lloh45
	.loh AdrpAdd	Lloh62, Lloh63
	.loh AdrpAdd	Lloh60, Lloh61
	.loh AdrpAdd	Lloh58, Lloh59
	.loh AdrpAdd	Lloh56, Lloh57
	.loh AdrpAdd	Lloh54, Lloh55
	.loh AdrpAdd	Lloh52, Lloh53
	.loh AdrpAdd	Lloh50, Lloh51
Lfunc_end0:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2, 0x0
GCC_except_table4:
Lexception0:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 Lcst_end0-Lcst_begin0
Lcst_begin0:
	.uleb128 Lfunc_begin0-Lfunc_begin0
	.uleb128 Ltmp0-Lfunc_begin0
	.byte	0
	.byte	0
	.uleb128 Ltmp0-Lfunc_begin0
	.uleb128 Ltmp1-Ltmp0
	.uleb128 Ltmp2-Lfunc_begin0
	.byte	0
	.uleb128 Ltmp1-Lfunc_begin0
	.uleb128 Lfunc_end0-Ltmp1
	.byte	0
	.byte	0
Lcst_end0:
	.p2align	2, 0x0

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__RNvXsq_NtCseduYQEDYcHM_5alloc6stringNtB5_6StringNtNtCs5dyeT9KiOLK_4core3fmt7Display3fmt:
	.cfi_startproc
	mov	x2, x1
	ldp	x8, x1, [x0, #8]
	mov	x0, x8
	b	__RNvXsi_NtCs5dyeT9KiOLK_4core3fmteNtB5_7Display3fmt
	.cfi_endproc

	.globl	_add_general_u8
	.p2align	2
_add_general_u8:
	.cfi_startproc
	and	w8, w0, #0xff
	add	w8, w8, w1, uxtb
	mov	w9, #255
	cmp	w8, #255
	csel	w0, w8, w9, lo
	ret
	.cfi_endproc

	.globl	_add_licensed_100_100
	.p2align	2
_add_licensed_100_100:
	.cfi_startproc
	add	w0, w1, w0
	ret
	.cfi_endproc

	.globl	_cast_radix_10_to_2
	.p2align	2
_cast_radix_10_to_2:
	.cfi_startproc
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
Lloh64:
	adrp	x8, __RNvCsh38WkIC3yDD_48p4_where_a_missed_merge_actually_costs_something4main@PAGE
Lloh65:
	add	x8, x8, __RNvCsh38WkIC3yDD_48p4_where_a_missed_merge_actually_costs_something4main@PAGEOFF
	str	x8, [sp, #8]
Lloh66:
	adrp	x1, l_anon.4f9e1bf70eba26db74865750b4215fde.0@PAGE
Lloh67:
	add	x1, x1, l_anon.4f9e1bf70eba26db74865750b4215fde.0@PAGEOFF
	add	x0, sp, #8
	mov	w4, #0
	bl	__RNvNtCsl82qo6vb64n_3std2rt19lang_start_internal
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
	.loh AdrpAdd	Lloh66, Lloh67
	.loh AdrpAdd	Lloh64, Lloh65
	.cfi_endproc

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.4f9e1bf70eba26db74865750b4215fde.0:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__RNSNvYNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0INtNtNtCs5dyeT9KiOLK_4core3ops8function6FnOnceuE9call_once6vtableCsh38WkIC3yDD_48p4_where_a_missed_merge_actually_costs_something
	.quad	__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0Csh38WkIC3yDD_48p4_where_a_missed_merge_actually_costs_something
	.quad	__RNCINvNtCsl82qo6vb64n_3std2rt10lang_startuE0Csh38WkIC3yDD_48p4_where_a_missed_merge_actually_costs_something

	.section	__TEXT,__const
l_anon.4f9e1bf70eba26db74865750b4215fde.1:
	.ascii	"p4. Where a missed merge actually costs something\n"

	.section	__TEXT,__cstring,cstring_literals
l_anon.4f9e1bf70eba26db74865750b4215fde.2:
	.asciz	"\300\001\n"

	.section	__TEXT,__const
l_anon.4f9e1bf70eba26db74865750b4215fde.3:
	.byte	10

l_anon.4f9e1bf70eba26db74865750b4215fde.4:
	.ascii	"S1. Monomorphic sites\n"

	.section	__TEXT,__cstring,cstring_literals
l_anon.4f9e1bf70eba26db74865750b4215fde.5:
	.asciz	"#  sum over the radix-2 spelling  : \300\001\n"

l_anon.4f9e1bf70eba26db74865750b4215fde.6:
	.asciz	"#  sum over the radix-10 spelling : \300\001\n"

	.section	__TEXT,__const
l_anon.4f9e1bf70eba26db74865750b4215fde.7:
	.ascii	"  neither call mentions the other spelling, so the cost is zero\n"

l_anon.4f9e1bf70eba26db74865750b4215fde.8:
	.ascii	"S2. One function over both spellings, by abstracting the parameter\n"

l_anon.4f9e1bf70eba26db74865750b4215fde.9:
	.ascii	"  sum_any_radix is ONE generic function and both arrays above went through it\n"

	.section	__TEXT,__cstring,cstring_literals
l_anon.4f9e1bf70eba26db74865750b4215fde.10:
	.asciz	":  sum_any_policy likewise accepts Obs<Sat> and Obs<Wrap>: \300\005 and \300\001\n"

	.section	__TEXT,__const
l_anon.4f9e1bf70eba26db74865750b4215fde.11:
	.ascii	"  so a missed merge is repairable at a FUNCTION boundary, for a spurious axis and for an observable one alike\n"

l_anon.4f9e1bf70eba26db74865750b4215fde.12:
	.ascii	"S3. The storage boundary, which the abstraction does not reach\n"

l_anon.4f9e1bf70eba26db74865750b4215fde.13:
	.ascii	"  a homogeneous container is one type by construction, so no generic signature lets\n"

l_anon.4f9e1bf70eba26db74865750b4215fde.14:
	.ascii	"  Spur<2> and Spur<10> share one array, one slice or one column. The compile-fail arm\n"

l_anon.4f9e1bf70eba26db74865750b4215fde.15:
	.ascii	"  is `p4b`, whose diagnostic is recorded in `p4b_expected_failure.txt`.\n"

l_anon.4f9e1bf70eba26db74865750b4215fde.16:
	.ascii	"THE DIRECTION COUNT, checked against what the operations do\n"

	.section	__TEXT,__cstring,cstring_literals
l_anon.4f9e1bf70eba26db74865750b4215fde.17:
	.asciz	"2  spurious (radix): the cast commutes with add on \300\001/\300\036 pairs, both directions exist\n"

l_anon.4f9e1bf70eba26db74865750b4215fde.18:
	.asciz	"5  observable (policy): the cast commutes with add on \300\001/\300A pairs, so it is a REINTERPRETATION and not a map of the algebra\n"

l_anon.4f9e1bf70eba26db74865750b4215fde.19:
	.asciz	"2  refinement (bound): widening is the identity on \300\001/\300R representations, and the tightening is refused before the program exists (`p4c`)\n"

	.section	__TEXT,__const
l_anon.4f9e1bf70eba26db74865750b4215fde.20:
	.ascii	"THE LICENSED ARM, inside and outside the declared bound\n"

	.section	__TEXT,__cstring,cstring_literals
l_anon.4f9e1bf70eba26db74865750b4215fde.21:
	.asciz	"\036  inside the declared bound : \300\001/\300\n disagree\n"

l_anon.4f9e1bf70eba26db74865750b4215fde.22:
	.asciz	"\036  outside it                : \300\001/\300\n disagree\n"

	.section	__TEXT,__const
l_anon.4f9e1bf70eba26db74865750b4215fde.23:
	.byte	200

	.section	__TEXT,__cstring,cstring_literals
l_anon.4f9e1bf70eba26db74865750b4215fde.24:
	.asciz	"4Declared bound sums: BSum<Lit<100>, Lit<100>>::HI = \300\001\n"

	.section	__TEXT,__const
l_anon.4f9e1bf70eba26db74865750b4215fde.25:
	.byte	255

	.section	__TEXT,__cstring,cstring_literals
l_anon.4f9e1bf70eba26db74865750b4215fde.26:
	.asciz	"4                     BSum<Lit<200>, Lit<100>>::HI = \300\001\n"

	.globl	_cast_radix_2_to_10
_cast_radix_2_to_10 = _cast_radix_10_to_2
	.globl	_plain_identity
_plain_identity = _cast_radix_10_to_2
	.globl	_widen_100_to_200
_widen_100_to_200 = _cast_radix_10_to_2
	.globl	_widen_7_to_255
_widen_7_to_255 = _cast_radix_10_to_2
.subsections_via_symbols
