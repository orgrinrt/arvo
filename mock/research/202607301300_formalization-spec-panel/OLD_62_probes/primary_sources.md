# Primary-source extracts, file 62

Short verbatim quotes for verification, with provenance. Both documents were read in full for the
sections quoted; the quotes are the load-bearing sentences only.

## OCP 8-bit Floating Point Specification (OFP8), Revision 1.0

Obtained from the Open Compute Project's own FP8 repository
(`github.com/opencomputeproject/FP8`, "OCP 8-bit Floating Point Specification (OFP8) Revision 1.0
2023-06-20.pdf", date approved June 20, 2023, authors from NVIDIA, Intel, Arm, Google, AMD, Meta).

Page 12, section 5.1, prose:

> The E5M2 format represents infinities and NaNs. Interpretation of the three mantissa values for
> NaNs is not defined. The E4M3 format does not represent infinities and uses only two bit
> patterns for NaN (a single mantissa-exponent bit pattern but allowing both values of the sign
> bit) in order to increase emax to 8 and thus to increase the dynamic range by one binade.

Page 13, Table 1 (OFP8 exponent parameters): E4M3 exponent bias 7, emax (unbiased) 8, emin
(unbiased) -6; E5M2 exponent bias 15, emax 15, emin -14.

Page 13, Table 2 (value encoding details): E4M3 infinities N/A, NaN `S.1111.111`, max normal
`S.1111.110 = +-448`, dynamic range 18 binades; E5M2 infinities `S.11111.00`, NaN
`S.11111.{01, 10, 11}`, max normal `+-57,344`, dynamic range 32 binades.

**Internal contradiction in the primary source, found during this check.** Page 11, section 4.2
(Abbreviations and acronyms), defines:

> E4M3: An OFP8 format with 1 sign bit, 4 biased exponent bits, 3 mantissa bits, and an exponent
> bias of 15. See below for further details.
> E5M2: An OFP8 format with 1 sign bit, 5 biased exponent bits, 2 mantissa bits, and an exponent
> bias of 7. See below for further details.

The two biases are transposed relative to Table 1 and to the arithmetic (E4M3 min normal `2^-6`
requires bias 7; E5M2 min normal `2^-14` requires bias 15). Table 1 and Table 2 are internally
consistent with each other and with the value formula on page 12; section 4.2 is the typo.

**Scope fact.** The document defines E4M3 and E5M2 only. No FNUZ variant appears anywhere in it.

## IEEE Std 754-2019, clause 5.2 (Decimal exponent calculation)

Extracted from a university-hosted licensed copy of the standard (IEEE Std 754-2019, page 30);
clause number and text identical in structure to 754-2008's clause 5.2.

The governing paragraph, verbatim:

> For all computational operations except where stated otherwise, if the result is inexact the
> cohort member of least possible exponent is used to get the maximum number of significant
> digits. If the result is exact, the cohort member is selected based on the preferred exponent
> for a result of that operation, a function of the exponents of the inputs. Thus for finite x,
> depending on the representation of zero, 0 + x might result in a different member of x's
> cohort. If the result's cohort does not include a member with the preferred exponent, the
> member with the exponent closest to the preferred exponent is used.
> For quantize and roundToIntegralExact, a finite result has the preferred exponent, whether or
> not the result is exact.

The clause's own scoping sentence for quantize, verbatim:

> Except for the quantize operation, the value of a floating-point result (and hence its cohort)
> is determined by the operation and the operands' values; it is never dependent on the
> representation or encoding of an operand.

The definition of the term (clause 3 definitions), verbatim:

> preferred exponent: For the result of a decimal operation, the value of the exponent q which
> best reflects the quanta of the operands when the result is exact.

Per-operation preferred exponents as listed in clauses 5.3 through 5.5 (paraphrased to the
formulas, which the text states one per operation): addition and subtraction `min(Q(x), Q(y))`;
multiplication `Q(x) + Q(y)`; division `Q(x) - Q(y)`; squareRoot `floor(Q(x) / 2)`;
fusedMultiplyAdd `min(Q(x) + Q(y), Q(z))`; quantize `Q(y)`; roundToIntegral `max(Q(x), 0)`;
scaleB `Q(x) + N`; remainder `min(Q(x), Q(y))`; conversions from integer formats `0`.
