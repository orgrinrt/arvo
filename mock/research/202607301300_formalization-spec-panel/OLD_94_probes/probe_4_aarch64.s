	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_receipt_fast_math
	.p2align	2
_receipt_fast_math:
	.cfi_startproc
	; InlineAsm Start
	mrs	x8, FPCR
	; InlineAsm End
	mvn	w8, w8
	mov	w9, #29884416
	tst	x8, x9
	cset	w0, eq
	ret
	.cfi_endproc

	.globl	_receipt_hand_written_file_90
	.p2align	2
_receipt_hand_written_file_90:
	.cfi_startproc
	; InlineAsm Start
	mrs	x8, FPCR
	; InlineAsm End
	tst	x8, #0x1c00000
	cset	w0, eq
	ret
	.cfi_endproc

	.globl	_receipt_ieee_default
	.p2align	2
_receipt_ieee_default:
	.cfi_startproc
	; InlineAsm Start
	mrs	x8, FPCR
	; InlineAsm End
	mov	w9, #29884416
	tst	x8, x9
	cset	w0, eq
	ret
	.cfi_endproc

.subsections_via_symbols
