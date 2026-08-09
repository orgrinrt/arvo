	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_receipt_checked
	.p2align	2
_receipt_checked:
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
