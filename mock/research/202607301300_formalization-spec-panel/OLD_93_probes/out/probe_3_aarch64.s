	.build_version macos, 26, 0	sdk_version 26, 2
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_f                              ; -- Begin function f
	.p2align	2
_f:                                     ; @f
	.cfi_startproc
; %bb.0:
	sdiv	w0, w0, w1
	ret
	.cfi_endproc
                                        ; -- End function
	.globl	_g                              ; -- Begin function g
	.p2align	2
_g:                                     ; @g
	.cfi_startproc
; %bb.0:
	cbz	w1, LBB1_2
; %bb.1:
	sdiv	w0, w0, w1
	ret
LBB1_2:
	mov	w0, #-999                       ; =0xfffffc19
	ret
	.cfi_endproc
                                        ; -- End function
.subsections_via_symbols
