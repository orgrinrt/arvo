	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_cold13_storage_roundtrip
	.p2align	2
_cold13_storage_roundtrip:
	.cfi_startproc
	and	x0, x0, #0xffff
	ret
	.cfi_endproc

	.globl	_cold13_storage_to_operand
	.p2align	2
_cold13_storage_to_operand:
	.cfi_startproc
	and	w0, w0, #0xffff
	ret
	.cfi_endproc

	.globl	_precise13_operand_widen
	.p2align	2
_precise13_operand_widen:
	.cfi_startproc
	ret
	.cfi_endproc

.subsections_via_symbols
