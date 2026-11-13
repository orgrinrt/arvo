	.file	"87duz9diwq79l9q1bo51xoie0"
	.section	.text.decay_floor,"ax",@progbits
	.globl	decay_floor
	.p2align	4
	.type	decay_floor,@function
decay_floor:
	.cfi_startproc
	movl	%edi, %eax
	testl	%edx, %edx
	je	.LBB0_3
	movslq	%esi, %rcx
	.p2align	4
.LBB0_2:
	cltq
	imulq	%rcx, %rax
	shrq	$16, %rax
	decl	%edx
	jne	.LBB0_2
.LBB0_3:
	retq
.Lfunc_end0:
	.size	decay_floor, .Lfunc_end0-decay_floor
	.cfi_endproc

	.section	.text.decay_half_even,"ax",@progbits
	.globl	decay_half_even
	.p2align	4
	.type	decay_half_even,@function
decay_half_even:
	.cfi_startproc
	movl	%edi, %eax
	testl	%edx, %edx
	je	.LBB1_3
	movslq	%esi, %rcx
	.p2align	4
.LBB1_2:
	movslq	%eax, %rsi
	imulq	%rcx, %rsi
	leaq	32768(%rsi), %rdi
	movq	%rdi, %rax
	shrq	$16, %rax
	negw	%si
	seto	%sil
	shrl	$16, %edi
	andl	$1, %edi
	andb	%sil, %dil
	movzbl	%dil, %esi
	subl	%esi, %eax
	decl	%edx
	jne	.LBB1_2
.LBB1_3:
	retq
.Lfunc_end1:
	.size	decay_half_even, .Lfunc_end1-decay_half_even
	.cfi_endproc

	.section	.text.decay_half_even_saturate,"ax",@progbits
	.globl	decay_half_even_saturate
	.p2align	4
	.type	decay_half_even_saturate,@function
decay_half_even_saturate:
	.cfi_startproc
	movl	%edi, %eax
	testl	%edx, %edx
	je	.LBB2_3
	movslq	%esi, %rcx
	movq	$-2147483648, %rsi
	movl	$2147483647, %edi
	.p2align	4
.LBB2_2:
	movslq	%eax, %r8
	imulq	%rcx, %r8
	leaq	32768(%r8), %r9
	movq	%r9, %rax
	sarq	$16, %rax
	negw	%r8w
	seto	%r8b
	shrl	$16, %r9d
	andl	$1, %r9d
	andb	%r8b, %r9b
	movzbl	%r9b, %r8d
	subq	%r8, %rax
	cmpq	$-2147483647, %rax
	cmovlq	%rsi, %rax
	cmpq	$2147483647, %rax
	cmovgeq	%rdi, %rax
	decl	%edx
	jne	.LBB2_2
.LBB2_3:
	retq
.Lfunc_end2:
	.size	decay_half_even_saturate, .Lfunc_end2-decay_half_even_saturate
	.cfi_endproc

	.section	.text.decay_half_up,"ax",@progbits
	.globl	decay_half_up
	.p2align	4
	.type	decay_half_up,@function
decay_half_up:
	.cfi_startproc
	movl	%edi, %eax
	testl	%edx, %edx
	je	.LBB3_3
	movslq	%esi, %rcx
	.p2align	4
.LBB3_2:
	cltq
	imulq	%rcx, %rax
	addq	$32768, %rax
	shrq	$16, %rax
	decl	%edx
	jne	.LBB3_2
.LBB3_3:
	retq
.Lfunc_end3:
	.size	decay_half_up, .Lfunc_end3-decay_half_up
	.cfi_endproc

	.section	.text.decay_toward_zero,"ax",@progbits
	.globl	decay_toward_zero
	.p2align	4
	.type	decay_toward_zero,@function
decay_toward_zero:
	.cfi_startproc
	movl	%edi, %eax
	testl	%edx, %edx
	je	.LBB4_3
	movslq	%esi, %rcx
	.p2align	4
.LBB4_2:
	movslq	%eax, %rsi
	imulq	%rcx, %rsi
	movq	%rsi, %rax
	shrq	$16, %rax
	testq	%rsi, %rsi
	sets	%dil
	testw	%si, %si
	setne	%sil
	andb	%dil, %sil
	movzbl	%sil, %esi
	addl	%esi, %eax
	decl	%edx
	jne	.LBB4_2
.LBB4_3:
	retq
.Lfunc_end4:
	.size	decay_toward_zero, .Lfunc_end4-decay_toward_zero
	.cfi_endproc

	.section	.text.egress_u13,"ax",@progbits
	.globl	egress_u13
	.p2align	4
	.type	egress_u13,@function
egress_u13:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movzwl	%di, %edi
	xorl	%esi, %esi
	callq	*_RNvMNtCscUJ8dHO8G39_11arvo_format5slotsNtB2_4Slot2at@GOTPCREL(%rip)
	movq	%rax, %rdi
	movq	%rdx, %rsi
	callq	*_RNvMNtCscUJ8dHO8G39_11arvo_format5slotsNtB2_4Slot5index@GOTPCREL(%rip)
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end5:
	.size	egress_u13, .Lfunc_end5-egress_u13
	.cfi_endproc

	.section	.text.fmul_f32,"ax",@progbits
	.globl	fmul_f32
	.p2align	4
	.type	fmul_f32,@function
fmul_f32:
	.cfi_startproc
	cmpq	%r9, %rsi
	cmovbq	%rsi, %r9
	cmpq	%r9, %rcx
	cmovbq	%rcx, %r9
	testq	%r9, %r9
	je	.LBB6_3
	xorl	%eax, %eax
	.p2align	4
.LBB6_2:
	movss	(%rdi,%rax,4), %xmm0
	mulss	(%rdx,%rax,4), %xmm0
	movss	%xmm0, (%r8,%rax,4)
	incq	%rax
	cmpq	%rax, %r9
	jne	.LBB6_2
.LBB6_3:
	retq
.Lfunc_end6:
	.size	fmul_f32, .Lfunc_end6-fmul_f32
	.cfi_endproc

	.section	.text.fold_u13_saturating,"ax",@progbits
	.globl	fold_u13_saturating
	.p2align	4
	.type	fold_u13_saturating,@function
fold_u13_saturating:
	.cfi_startproc
	testq	%rsi, %rsi
	je	.LBB7_1
	addq	%rsi, %rsi
	xorl	%ecx, %ecx
	xorl	%edx, %edx
	.p2align	4
.LBB7_3:
	addw	(%rdi,%rcx), %dx
	movzwl	%dx, %eax
	cmpl	$8191, %eax
	movl	$8191, %eax
	cmovbl	%edx, %eax
	addq	$2, %rcx
	movl	%eax, %edx
	cmpq	%rcx, %rsi
	jne	.LBB7_3
	retq
.LBB7_1:
	xorl	%eax, %eax
	retq
.Lfunc_end7:
	.size	fold_u13_saturating, .Lfunc_end7-fold_u13_saturating
	.cfi_endproc

	.section	.text.fold_u13_wrapping,"ax",@progbits
	.globl	fold_u13_wrapping
	.p2align	4
	.type	fold_u13_wrapping,@function
fold_u13_wrapping:
	.cfi_startproc
	testq	%rsi, %rsi
	je	.LBB8_1
	addq	%rsi, %rsi
	xorl	%ecx, %ecx
	xorl	%eax, %eax
	.p2align	4
.LBB8_3:
	addw	(%rdi,%rcx), %ax
	andl	$8191, %eax
	addq	$2, %rcx
	cmpq	%rcx, %rsi
	jne	.LBB8_3
	retq
.LBB8_1:
	xorl	%eax, %eax
	retq
.Lfunc_end8:
	.size	fold_u13_wrapping, .Lfunc_end8-fold_u13_wrapping
	.cfi_endproc

	.section	.text.fold_u16_wrapping,"ax",@progbits
	.globl	fold_u16_wrapping
	.p2align	4
	.type	fold_u16_wrapping,@function
fold_u16_wrapping:
	.cfi_startproc
	testq	%rsi, %rsi
	je	.LBB9_1
	addq	%rsi, %rsi
	xorl	%ecx, %ecx
	xorl	%eax, %eax
	.p2align	4
.LBB9_3:
	addw	(%rdi,%rcx), %ax
	addq	$2, %rcx
	cmpq	%rcx, %rsi
	jne	.LBB9_3
	retq
.LBB9_1:
	xorl	%eax, %eax
	retq
.Lfunc_end9:
	.size	fold_u16_wrapping, .Lfunc_end9-fold_u16_wrapping
	.cfi_endproc

	.section	.text.host_fold_u13_saturating,"ax",@progbits
	.globl	host_fold_u13_saturating
	.p2align	4
	.type	host_fold_u13_saturating,@function
host_fold_u13_saturating:
	.cfi_startproc
	testq	%rsi, %rsi
	je	.LBB10_1
	addq	%rsi, %rsi
	xorl	%ecx, %ecx
	xorl	%edx, %edx
	.p2align	4
.LBB10_3:
	addw	(%rdi,%rcx), %dx
	movzwl	%dx, %eax
	cmpl	$8191, %eax
	movl	$8191, %eax
	cmovbl	%edx, %eax
	addq	$2, %rcx
	movl	%eax, %edx
	cmpq	%rcx, %rsi
	jne	.LBB10_3
	retq
.LBB10_1:
	xorl	%eax, %eax
	retq
.Lfunc_end10:
	.size	host_fold_u13_saturating, .Lfunc_end10-host_fold_u13_saturating
	.cfi_endproc

	.section	.text.host_fold_u13_wrapping,"ax",@progbits
	.globl	host_fold_u13_wrapping
	.p2align	4
	.type	host_fold_u13_wrapping,@function
host_fold_u13_wrapping:
	.cfi_startproc
	testq	%rsi, %rsi
	je	.LBB11_1
	addq	%rsi, %rsi
	xorl	%ecx, %ecx
	xorl	%eax, %eax
	.p2align	4
.LBB11_3:
	addw	(%rdi,%rcx), %ax
	andl	$8191, %eax
	addq	$2, %rcx
	cmpq	%rcx, %rsi
	jne	.LBB11_3
	retq
.LBB11_1:
	xorl	%eax, %eax
	retq
.Lfunc_end11:
	.size	host_fold_u13_wrapping, .Lfunc_end11-host_fold_u13_wrapping
	.cfi_endproc

	.section	.text.host_fold_u16_wrapping,"ax",@progbits
	.globl	host_fold_u16_wrapping
	.p2align	4
	.type	host_fold_u16_wrapping,@function
host_fold_u16_wrapping:
	.cfi_startproc
	testq	%rsi, %rsi
	je	.LBB12_1
	addq	%rsi, %rsi
	xorl	%ecx, %ecx
	xorl	%eax, %eax
	.p2align	4
.LBB12_3:
	addw	(%rdi,%rcx), %ax
	addq	$2, %rcx
	cmpq	%rcx, %rsi
	jne	.LBB12_3
	retq
.LBB12_1:
	xorl	%eax, %eax
	retq
.Lfunc_end12:
	.size	host_fold_u16_wrapping, .Lfunc_end12-host_fold_u16_wrapping
	.cfi_endproc

	.section	.text.host_ingest_u13_saturating,"ax",@progbits
	.globl	host_ingest_u13_saturating
	.p2align	4
	.type	host_ingest_u13_saturating,@function
host_ingest_u13_saturating:
	.cfi_startproc
	movzwl	%di, %eax
	cmpl	$8191, %eax
	movl	$8191, %eax
	cmovbl	%edi, %eax
	retq
.Lfunc_end13:
	.size	host_ingest_u13_saturating, .Lfunc_end13-host_ingest_u13_saturating
	.cfi_endproc

	.section	.text.host_ingest_u13_wrapping,"ax",@progbits
	.globl	host_ingest_u13_wrapping
	.p2align	4
	.type	host_ingest_u13_wrapping,@function
host_ingest_u13_wrapping:
	.cfi_startproc
	movl	%edi, %eax
	andl	$8191, %eax
	retq
.Lfunc_end14:
	.size	host_ingest_u13_wrapping, .Lfunc_end14-host_ingest_u13_wrapping
	.cfi_endproc

	.section	.text.host_map_u13_saturating,"ax",@progbits
	.globl	host_map_u13_saturating
	.p2align	4
	.type	host_map_u13_saturating,@function
host_map_u13_saturating:
	.cfi_startproc
	cmpq	%r9, %rsi
	cmovbq	%rsi, %r9
	cmpq	%r9, %rcx
	cmovbq	%rcx, %r9
	testq	%r9, %r9
	je	.LBB15_5
	xorl	%eax, %eax
	jmp	.LBB15_2
	.p2align	4
.LBB15_4:
	movw	%cx, (%r8,%rax,2)
	incq	%rax
	cmpq	%rax, %r9
	je	.LBB15_5
.LBB15_2:
	movzwl	(%rdx,%rax,2), %ecx
	addw	(%rdi,%rax,2), %cx
	movzwl	%cx, %esi
	cmpl	$8191, %esi
	jb	.LBB15_4
	movl	$8191, %ecx
	jmp	.LBB15_4
.LBB15_5:
	retq
.Lfunc_end15:
	.size	host_map_u13_saturating, .Lfunc_end15-host_map_u13_saturating
	.cfi_endproc

	.section	.text.ingest_i13_saturating,"ax",@progbits
	.globl	ingest_i13_saturating
	.p2align	4
	.type	ingest_i13_saturating,@function
ingest_i13_saturating:
	.cfi_startproc
	pushq	%r14
	.cfi_def_cfa_offset 16
	pushq	%rbx
	.cfi_def_cfa_offset 24
	subq	$104, %rsp
	.cfi_def_cfa_offset 128
	.cfi_offset %rbx, -24
	.cfi_offset %r14, -16
	movswq	%di, %rdi
	movq	%rdi, %rsi
	sarq	$63, %rsi
	callq	*_RNvMNtCscUJ8dHO8G39_11arvo_format5slotsNtB2_4Slot2at@GOTPCREL(%rip)
	leaq	32(%rsp), %rbx
	movq	%rbx, %rdi
	movq	%rax, %rsi
	callq	*_RNvMs1_NtNtCscUJ8dHO8G39_11arvo_format5apply8positionNtB5_5Exact7on_grid@GOTPCREL(%rip)
	movq	%rsp, %r14
	movl	$1, %r8d
	movq	%r14, %rdi
	xorl	%esi, %esi
	movq	%rbx, %rdx
	xorl	%ecx, %ecx
	callq	*_RNvNtCscUJ8dHO8G39_11arvo_format5apply10round_slot@GOTPCREL(%rip)
	movl	$4095, %r8d
	movl	$1, %edi
	movq	%r14, %rsi
	movq	$-4096, %rdx
	movq	$-1, %rcx
	xorl	%r9d, %r9d
	callq	*_RNvNtCscUJ8dHO8G39_11arvo_format5apply13complete_slot@GOTPCREL(%rip)
	movq	%rax, %rdi
	movq	%rdx, %rsi
	callq	*_RNvMNtCscUJ8dHO8G39_11arvo_format5slotsNtB2_4Slot5index@GOTPCREL(%rip)
	addq	$104, %rsp
	.cfi_def_cfa_offset 24
	popq	%rbx
	.cfi_def_cfa_offset 16
	popq	%r14
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end16:
	.size	ingest_i13_saturating, .Lfunc_end16-ingest_i13_saturating
	.cfi_endproc

	.section	.text.ingest_u13_saturating,"ax",@progbits
	.globl	ingest_u13_saturating
	.p2align	4
	.type	ingest_u13_saturating,@function
ingest_u13_saturating:
	.cfi_startproc
	pushq	%r14
	.cfi_def_cfa_offset 16
	pushq	%rbx
	.cfi_def_cfa_offset 24
	subq	$104, %rsp
	.cfi_def_cfa_offset 128
	.cfi_offset %rbx, -24
	.cfi_offset %r14, -16
	movzwl	%di, %edi
	xorl	%esi, %esi
	callq	*_RNvMNtCscUJ8dHO8G39_11arvo_format5slotsNtB2_4Slot2at@GOTPCREL(%rip)
	leaq	32(%rsp), %rbx
	movq	%rbx, %rdi
	movq	%rax, %rsi
	callq	*_RNvMs1_NtNtCscUJ8dHO8G39_11arvo_format5apply8positionNtB5_5Exact7on_grid@GOTPCREL(%rip)
	movq	%rsp, %r14
	movl	$1, %r8d
	movq	%r14, %rdi
	xorl	%esi, %esi
	movq	%rbx, %rdx
	xorl	%ecx, %ecx
	callq	*_RNvNtCscUJ8dHO8G39_11arvo_format5apply10round_slot@GOTPCREL(%rip)
	movl	$8191, %r8d
	movl	$1, %edi
	movq	%r14, %rsi
	xorl	%edx, %edx
	xorl	%ecx, %ecx
	xorl	%r9d, %r9d
	callq	*_RNvNtCscUJ8dHO8G39_11arvo_format5apply13complete_slot@GOTPCREL(%rip)
	movq	%rax, %rdi
	movq	%rdx, %rsi
	callq	*_RNvMNtCscUJ8dHO8G39_11arvo_format5slotsNtB2_4Slot5index@GOTPCREL(%rip)
	addq	$104, %rsp
	.cfi_def_cfa_offset 24
	popq	%rbx
	.cfi_def_cfa_offset 16
	popq	%r14
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end17:
	.size	ingest_u13_saturating, .Lfunc_end17-ingest_u13_saturating
	.cfi_endproc

	.section	.text.ingest_u13_wrapping,"ax",@progbits
	.globl	ingest_u13_wrapping
	.p2align	4
	.type	ingest_u13_wrapping,@function
ingest_u13_wrapping:
	.cfi_startproc
	pushq	%r14
	.cfi_def_cfa_offset 16
	pushq	%rbx
	.cfi_def_cfa_offset 24
	subq	$104, %rsp
	.cfi_def_cfa_offset 128
	.cfi_offset %rbx, -24
	.cfi_offset %r14, -16
	movzwl	%di, %edi
	xorl	%esi, %esi
	callq	*_RNvMNtCscUJ8dHO8G39_11arvo_format5slotsNtB2_4Slot2at@GOTPCREL(%rip)
	leaq	32(%rsp), %rbx
	movq	%rbx, %rdi
	movq	%rax, %rsi
	callq	*_RNvMs1_NtNtCscUJ8dHO8G39_11arvo_format5apply8positionNtB5_5Exact7on_grid@GOTPCREL(%rip)
	movq	%rsp, %r14
	movl	$1, %r8d
	movq	%r14, %rdi
	xorl	%esi, %esi
	movq	%rbx, %rdx
	xorl	%ecx, %ecx
	callq	*_RNvNtCscUJ8dHO8G39_11arvo_format5apply10round_slot@GOTPCREL(%rip)
	movl	$8191, %r8d
	xorl	%edi, %edi
	movq	%r14, %rsi
	xorl	%edx, %edx
	xorl	%ecx, %ecx
	xorl	%r9d, %r9d
	callq	*_RNvNtCscUJ8dHO8G39_11arvo_format5apply13complete_slot@GOTPCREL(%rip)
	movq	%rax, %rdi
	movq	%rdx, %rsi
	callq	*_RNvMNtCscUJ8dHO8G39_11arvo_format5slotsNtB2_4Slot5index@GOTPCREL(%rip)
	addq	$104, %rsp
	.cfi_def_cfa_offset 24
	popq	%rbx
	.cfi_def_cfa_offset 16
	popq	%r14
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end18:
	.size	ingest_u13_wrapping, .Lfunc_end18-ingest_u13_wrapping
	.cfi_endproc

	.section	.text.map_u13_saturating,"ax",@progbits
	.globl	map_u13_saturating
	.p2align	4
	.type	map_u13_saturating,@function
map_u13_saturating:
	.cfi_startproc
	cmpq	%r9, %rsi
	cmovbq	%rsi, %r9
	cmpq	%r9, %rcx
	cmovbq	%rcx, %r9
	testq	%r9, %r9
	je	.LBB19_5
	xorl	%eax, %eax
	jmp	.LBB19_2
	.p2align	4
.LBB19_4:
	movw	%cx, (%r8,%rax,2)
	incq	%rax
	cmpq	%rax, %r9
	je	.LBB19_5
.LBB19_2:
	movzwl	(%rdx,%rax,2), %ecx
	addw	(%rdi,%rax,2), %cx
	movzwl	%cx, %esi
	cmpl	$8191, %esi
	jb	.LBB19_4
	movl	$8191, %ecx
	jmp	.LBB19_4
.LBB19_5:
	retq
.Lfunc_end19:
	.size	map_u13_saturating, .Lfunc_end19-map_u13_saturating
	.cfi_endproc

	.section	.text.qmul_floor_saturate,"ax",@progbits
	.globl	qmul_floor_saturate
	.p2align	4
	.type	qmul_floor_saturate,@function
qmul_floor_saturate:
	.cfi_startproc
	cmpq	%r9, %rsi
	cmovbq	%rsi, %r9
	cmpq	%r9, %rcx
	cmovbq	%rcx, %r9
	testq	%r9, %r9
	je	.LBB20_5
	xorl	%eax, %eax
	movq	$-2147483648, %rcx
	jmp	.LBB20_2
	.p2align	4
.LBB20_4:
	movl	%esi, (%r8,%rax,4)
	incq	%rax
	cmpq	%rax, %r9
	je	.LBB20_5
.LBB20_2:
	movslq	(%rdi,%rax,4), %r10
	movslq	(%rdx,%rax,4), %rsi
	imulq	%r10, %rsi
	sarq	$16, %rsi
	cmpq	$-2147483647, %rsi
	cmovlq	%rcx, %rsi
	cmpq	$2147483647, %rsi
	jl	.LBB20_4
	movl	$2147483647, %esi
	jmp	.LBB20_4
.LBB20_5:
	retq
.Lfunc_end20:
	.size	qmul_floor_saturate, .Lfunc_end20-qmul_floor_saturate
	.cfi_endproc

	.section	.text.qmul_floor_wrap,"ax",@progbits
	.globl	qmul_floor_wrap
	.p2align	4
	.type	qmul_floor_wrap,@function
qmul_floor_wrap:
	.cfi_startproc
	cmpq	%r9, %rsi
	cmovbq	%rsi, %r9
	cmpq	%r9, %rcx
	cmovbq	%rcx, %r9
	testq	%r9, %r9
	je	.LBB21_3
	xorl	%eax, %eax
	.p2align	4
.LBB21_2:
	movslq	(%rdi,%rax,4), %rcx
	movslq	(%rdx,%rax,4), %rsi
	imulq	%rcx, %rsi
	shrq	$16, %rsi
	movl	%esi, (%r8,%rax,4)
	incq	%rax
	cmpq	%rax, %r9
	jne	.LBB21_2
.LBB21_3:
	retq
.Lfunc_end21:
	.size	qmul_floor_wrap, .Lfunc_end21-qmul_floor_wrap
	.cfi_endproc

	.section	.text.qmul_half_even_saturate,"ax",@progbits
	.globl	qmul_half_even_saturate
	.p2align	4
	.type	qmul_half_even_saturate,@function
qmul_half_even_saturate:
	.cfi_startproc
	cmpq	%r9, %rsi
	cmovbq	%rsi, %r9
	cmpq	%r9, %rcx
	cmovbq	%rcx, %r9
	testq	%r9, %r9
	je	.LBB22_7
	xorl	%eax, %eax
	jmp	.LBB22_2
	.p2align	4
.LBB22_6:
	movl	%ecx, (%r8,%rax,4)
	incq	%rax
	cmpq	%rax, %r9
	je	.LBB22_7
.LBB22_2:
	movslq	(%rdi,%rax,4), %rcx
	movslq	(%rdx,%rax,4), %rsi
	imulq	%rcx, %rsi
	leaq	32768(%rsi), %r10
	movq	%r10, %rcx
	sarq	$16, %rcx
	negw	%si
	seto	%sil
	shrl	$16, %r10d
	andl	$1, %r10d
	andb	%sil, %r10b
	movzbl	%r10b, %esi
	subq	%rsi, %rcx
	cmpq	$-2147483647, %rcx
	jl	.LBB22_3
	cmpq	$2147483647, %rcx
	jl	.LBB22_6
	jmp	.LBB22_5
	.p2align	4
.LBB22_3:
	movq	$-2147483648, %rcx
	cmpq	$2147483647, %rcx
	jl	.LBB22_6
.LBB22_5:
	movl	$2147483647, %ecx
	jmp	.LBB22_6
.LBB22_7:
	retq
.Lfunc_end22:
	.size	qmul_half_even_saturate, .Lfunc_end22-qmul_half_even_saturate
	.cfi_endproc

	.section	.text.qmul_half_even_wrap,"ax",@progbits
	.globl	qmul_half_even_wrap
	.p2align	4
	.type	qmul_half_even_wrap,@function
qmul_half_even_wrap:
	.cfi_startproc
	cmpq	%r9, %rsi
	cmovbq	%rsi, %r9
	cmpq	%r9, %rcx
	cmovbq	%rcx, %r9
	testq	%r9, %r9
	je	.LBB23_3
	xorl	%eax, %eax
	.p2align	4
.LBB23_2:
	movslq	(%rdi,%rax,4), %rcx
	movslq	(%rdx,%rax,4), %rsi
	imulq	%rcx, %rsi
	leaq	32768(%rsi), %rcx
	movq	%rcx, %r10
	shrq	$16, %r10
	negw	%si
	seto	%sil
	shrl	$16, %ecx
	andl	$1, %ecx
	andb	%sil, %cl
	movzbl	%cl, %ecx
	subl	%ecx, %r10d
	movl	%r10d, (%r8,%rax,4)
	incq	%rax
	cmpq	%rax, %r9
	jne	.LBB23_2
.LBB23_3:
	retq
.Lfunc_end23:
	.size	qmul_half_even_wrap, .Lfunc_end23-qmul_half_even_wrap
	.cfi_endproc

	.section	.text.qmul_half_up_wrap,"ax",@progbits
	.globl	qmul_half_up_wrap
	.p2align	4
	.type	qmul_half_up_wrap,@function
qmul_half_up_wrap:
	.cfi_startproc
	cmpq	%r9, %rsi
	cmovbq	%rsi, %r9
	cmpq	%r9, %rcx
	cmovbq	%rcx, %r9
	testq	%r9, %r9
	je	.LBB24_3
	xorl	%eax, %eax
	.p2align	4
.LBB24_2:
	movslq	(%rdi,%rax,4), %rcx
	movslq	(%rdx,%rax,4), %rsi
	imulq	%rcx, %rsi
	addq	$32768, %rsi
	shrq	$16, %rsi
	movl	%esi, (%r8,%rax,4)
	incq	%rax
	cmpq	%rax, %r9
	jne	.LBB24_2
.LBB24_3:
	retq
.Lfunc_end24:
	.size	qmul_half_up_wrap, .Lfunc_end24-qmul_half_up_wrap
	.cfi_endproc

	.section	.text.qmul_toward_zero_wrap,"ax",@progbits
	.globl	qmul_toward_zero_wrap
	.p2align	4
	.type	qmul_toward_zero_wrap,@function
qmul_toward_zero_wrap:
	.cfi_startproc
	cmpq	%r9, %rsi
	cmovbq	%rsi, %r9
	cmpq	%r9, %rcx
	cmovbq	%rcx, %r9
	testq	%r9, %r9
	je	.LBB25_3
	xorl	%eax, %eax
	.p2align	4
.LBB25_2:
	movslq	(%rdi,%rax,4), %rcx
	movslq	(%rdx,%rax,4), %rsi
	imulq	%rcx, %rsi
	movq	%rsi, %rcx
	shrq	$16, %rcx
	testq	%rsi, %rsi
	sets	%r10b
	testw	%si, %si
	setne	%sil
	andb	%r10b, %sil
	movzbl	%sil, %esi
	addl	%esi, %ecx
	movl	%ecx, (%r8,%rax,4)
	incq	%rax
	cmpq	%rax, %r9
	jne	.LBB25_2
.LBB25_3:
	retq
.Lfunc_end25:
	.size	qmul_toward_zero_wrap, .Lfunc_end25-qmul_toward_zero_wrap
	.cfi_endproc

	.ident	"rustc version 1.98.0-nightly (57d06900f 2026-05-27)"
	.section	".note.GNU-stack","",@progbits
