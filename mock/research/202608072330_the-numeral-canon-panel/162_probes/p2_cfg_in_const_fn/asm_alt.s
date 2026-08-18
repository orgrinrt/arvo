	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_lowered
	.p2align	2
_lowered:
	.cfi_startproc
	and	x0, x0, #0x1fff
	ret
	.cfi_endproc

.subsections_via_symbols
