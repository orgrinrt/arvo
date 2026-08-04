	.build_version macos, 26, 0	sdk_version 26, 2
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_f                              ## -- Begin function f
	.p2align	4
_f:                                     ## @f
	.cfi_startproc
## %bb.0:
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	movl	%edi, %eax
	cltd
	idivl	%esi
	popq	%rbp
	retq
	.cfi_endproc
                                        ## -- End function
	.globl	_g                              ## -- Begin function g
	.p2align	4
_g:                                     ## @g
	.cfi_startproc
## %bb.0:
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset %rbp, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register %rbp
	testl	%esi, %esi
	je	LBB1_1
## %bb.2:
	movl	%edi, %eax
	cltd
	idivl	%esi
	popq	%rbp
	retq
LBB1_1:
	movl	$-999, %eax                     ## imm = 0xFC19
	popq	%rbp
	retq
	.cfi_endproc
                                        ## -- End function
.subsections_via_symbols
