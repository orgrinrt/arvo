	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_chain_round_everywhere
	.p2align	2
_chain_round_everywhere:
	.cfi_startproc
	mov	x8, #128
	madd	x9, x1, x0, x8
	add	x9, x2, x9, asr #8
	madd	x9, x9, x1, x8
	add	x9, x2, x9, asr #8
	madd	x9, x9, x1, x8
	add	x9, x2, x9, asr #8
	madd	x8, x9, x1, x8
	add	x0, x2, x8, asr #8
	ret
	.cfi_endproc

	.globl	_chain_switch_at_2
	.p2align	2
_chain_switch_at_2:
	.cfi_startproc
	mul	x8, x1, x0
	add	x8, x2, x8, asr #8
	mov	x9, #128
	madd	x8, x8, x1, x9
	add	x8, x2, x8, asr #8
	madd	x8, x8, x1, x9
	add	x8, x2, x8, asr #8
	madd	x8, x8, x1, x9
	add	x0, x2, x8, asr #8
	ret
	.cfi_endproc

	.globl	_chain_truncate_everywhere
	.p2align	2
_chain_truncate_everywhere:
	.cfi_startproc
	mul	x8, x1, x0
	add	x8, x2, x8, asr #8
	mul	x8, x8, x1
	add	x8, x2, x8, asr #8
	mul	x8, x8, x1
	add	x8, x2, x8, asr #8
	mul	x8, x8, x1
	add	x0, x2, x8, asr #8
	ret
	.cfi_endproc

.subsections_via_symbols
