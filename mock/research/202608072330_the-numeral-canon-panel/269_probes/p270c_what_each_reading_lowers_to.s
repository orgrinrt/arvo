	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_ties_away
	.p2align	2
_ties_away:
	.cfi_startproc
	asr	w8, w0, #31
	sxtw	x9, w0
	add	x8, x9, w8, sxtw
	add	x8, x8, #128
	lsr	x0, x8, #8
	ret
	.cfi_endproc

	.globl	_ties_away_narrow
	.p2align	2
_ties_away_narrow:
	.cfi_startproc
	add	w8, w0, w0, asr #31
	add	w8, w8, #128
	asr	w0, w8, #8
	ret
	.cfi_endproc

	.globl	_ties_away_via_toward_zero
	.p2align	2
_ties_away_via_toward_zero:
	.cfi_startproc
	cmp	w0, #0
	mov	x8, #-128
	cneg	x8, x8, pl
	adds	x8, x8, w0, sxtw
	add	x9, x8, #255
	csel	x8, x9, x8, mi
	lsr	x0, x8, #8
	ret
	.cfi_endproc

	.globl	_ties_even
	.p2align	2
_ties_even:
	.cfi_startproc
	asr	w8, w0, #8
	and	w9, w0, #0xff
	cmp	w9, #128
	and	w9, w0, #0x1ff
	mov	w10, #384
	ccmp	w9, w10, #4, ls
	cinc	w0, w8, eq
	ret
	.cfi_endproc

	.globl	_ties_pos_inf
	.p2align	2
_ties_pos_inf:
	.cfi_startproc
	sxtw	x8, w0
	add	x8, x8, #128
	lsr	x0, x8, #8
	ret
	.cfi_endproc

	.globl	_ties_pos_inf_narrow
	.p2align	2
_ties_pos_inf_narrow:
	.cfi_startproc
	add	w8, w0, #128
	asr	w0, w8, #8
	ret
	.cfi_endproc

.subsections_via_symbols
