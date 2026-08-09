	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_receipt_aarch64
	.p2align	2
_receipt_aarch64:
	.cfi_startproc
	; InlineAsm Start
	mrs	x8, FPCR
	; InlineAsm End
	tst	x8, #0x1c00000
	cset	w0, eq
	ret
	.cfi_endproc

.subsections_via_symbols
