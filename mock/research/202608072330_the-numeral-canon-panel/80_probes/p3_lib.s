	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RINvCs27aTlh8mbtI_19p3_select_and_erase10sel_staticKm0_EB2_
	.p2align	2
__RINvCs27aTlh8mbtI_19p3_select_and_erase10sel_staticKm0_EB2_:
	.cfi_startproc
	add	x8, x2, x1
	mul	x0, x8, x0
	ret
	.cfi_endproc

	.globl	__RINvCs27aTlh8mbtI_19p3_select_and_erase10sel_staticKm8_EB2_
	.p2align	2
__RINvCs27aTlh8mbtI_19p3_select_and_erase10sel_staticKm8_EB2_:
	.cfi_startproc
	mov	x8, #128
	madd	x9, x1, x0, x8
	madd	x8, x2, x0, x8
	asr	x8, x8, #8
	add	x0, x8, x9, asr #8
	ret
	.cfi_endproc

	.globl	__RNvCs27aTlh8mbtI_19p3_select_and_erase11sel_dynamic
	.p2align	2
__RNvCs27aTlh8mbtI_19p3_select_and_erase11sel_dynamic:
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

	.globl	__RNvCs27aTlh8mbtI_19p3_select_and_erase13sel_static_f0
	.p2align	2
__RNvCs27aTlh8mbtI_19p3_select_and_erase13sel_static_f0:
	.cfi_startproc
	b	__RINvCs27aTlh8mbtI_19p3_select_and_erase10sel_staticKm0_EB2_
	.cfi_endproc

	.globl	__RNvCs27aTlh8mbtI_19p3_select_and_erase13sel_static_f8
	.p2align	2
__RNvCs27aTlh8mbtI_19p3_select_and_erase13sel_static_f8:
	.cfi_startproc
	b	__RINvCs27aTlh8mbtI_19p3_select_and_erase10sel_staticKm8_EB2_
	.cfi_endproc

	.globl	__RNvCs27aTlh8mbtI_19p3_select_and_erase15only_general_f8
__RNvCs27aTlh8mbtI_19p3_select_and_erase15only_general_f8 = __RINvCs27aTlh8mbtI_19p3_select_and_erase10sel_staticKm8_EB2_
	.globl	__RNvCs27aTlh8mbtI_19p3_select_and_erase13only_fused_f0
__RNvCs27aTlh8mbtI_19p3_select_and_erase13only_fused_f0 = __RINvCs27aTlh8mbtI_19p3_select_and_erase10sel_staticKm0_EB2_
	.globl	__RNvCs27aTlh8mbtI_19p3_select_and_erase15only_general_f0
__RNvCs27aTlh8mbtI_19p3_select_and_erase15only_general_f0 = __RINvCs27aTlh8mbtI_19p3_select_and_erase10sel_staticKm0_EB2_
.subsections_via_symbols
