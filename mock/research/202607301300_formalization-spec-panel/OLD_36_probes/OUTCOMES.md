# Probes for panel file 36, the normal form and its price

Nine `.rs` artifacts plus a compile-cost sweep. Every probe was compiled (or
refused, where the refusal is the claim) against the workspace pin,
`rustc 1.98.0-nightly (57d06900f 2026-05-27)`, confirmed with `rustc --version`
from inside the repo. The pin matters here in a way it has not in earlier
files: run from outside the repo the same commands resolve to stable 1.94,
where `type const` does not even parse, which is a trap worth recording once.

Build line unless stated otherwise:

```
rustc --edition 2021 --crate-type lib <file> --out-dir <tmp>
```

| Probe | Question | Outcome |
|---|---|---|
| `probe_1_the_shipped_width_chain_admits_a_second_zero.rs` | Does the width chain the multiplicative half uses satisfy the value-uniqueness obligation, as file 34 states it does for free (`34:328-329`)? | WORKS, and no. `UInt<UTerm, B0>` inhabits `Width` with `VALUE == 0`, as does `UInt<UInt<UTerm, B0>, B0>`; three spellings of thirteen likewise, all asserted. The adder propagates the spelling rather than normalising it (`0 + 0` is `UTerm` from one spelling and `UInt<UTerm, B0>` from another). A fixed-length literal-to-type table, the obvious way to generate one at a large bound, emits padded rows for every width and type-checks. The claim was true of the values the current operations happen to produce, not of the encoding, and the obligation is about the encoding. |
| `probe_1b_two_zeros_refuse_to_unify.rs` | Are the two `Width` inhabitants of zero the same type? | FAILS WITH E0308, committed refusing: ``expected `PhantomData<UTerm>`, found `PhantomData<UInt<UTerm, B0>>` ``. File 34's probe 5b reproduced one layer down, on the half it believed was already safe. |
| `probe_2_value_unique_naturals_by_construction.rs` | Can the encoding be made value-unique structurally, so nothing normalises? | WORKS. `Pos ::= H \| O<P: Pos> \| I<P: Pos>`, `Nat ::= Z \| Pz<P: Pos>` (Coq's `positive`/`N`). Uniqueness by induction, stated in the header. Addition with carry, eighteen impls, no output ever zero so no case needs a repair. The sums the multiplicative half needs (13+7=20, 3+2=5, and the zero cases) are asserted for value AND accepted by a type-equality demand, and associativity and commutativity of the width adder hold as type identities. |
| `probe_2b_the_padded_spelling_has_no_type.rs` | Can the padded spelling reach a bounded position? | FAILS WITH two E0277s, committed refusing: `` the trait bound `Z: Pos` is not satisfied `` for `O<Z>`, and the same for `Pz<Z>` against `Nat`. The perimeter is the bound, and the illegal state fails it. |
| `probe_3_stein_gcd_on_the_value_unique_encoding.rs` | Build the trait-level gcd the obligation needs. | WORKS. Stein's binary gcd, which is also what the prior art uses (`typenum-1.20.1/src/uint.rs:1467-1528`; I read it rather than assuming Euclid, and the assumption would have been wrong). 28 binary instantiations plus 4 three- and four-argument folds asserted against hand-computed values including the classical Euclid pair (1071, 462) -> 21, the coprime cases a reduction actually asks about, and file 34's own biased-MAC numbers (`34_probes/probe_3...rs:50-51,69,120`: gcd(16,8,8) = 8 and gcd(16,8,8,4) = 4). Commutativity and associativity hold as type identities, which is what makes the three- and four-argument folds in the design's own formulas well formed. |
| `probe_4_the_rational_normal_form.rs` | State and build the rational normal form; does file 34's probe 5b compile under it? | WORKS. Normal form: `Ratio<N, D>` over the value-unique positives with `gcd(N, D) = 1`. Reduction needs a division, which is the only new operation the obligation costs; it is exact division by an odd divisor, LSB-first (Jebelean/Hensel), reached after `Strip2` removes the common power of two structurally. 8 division cases and 16 reduction assertions, including 6/12 -> 1/2 (file 34's exact witness), 15/255 -> 1/17 (the UNORM shape), and 12/8 -> 3/2. Reduction is idempotent as a type identity. File 34's refusing 5b now compiles: `Reduced<P6, P12>` and the directly written `Ratio<P1, P2>` are one type. |
| `probe_4b_the_unreduced_ratio_is_not_an_adjustment.rs` | Is the coprimality enforced or merely satisfied? | FAILS WITH E0271 and E0277, committed refusing. E0271: `` type mismatch resolving `<O<I<H>> as Gcd<O<O<I<H>>>>>::Out == H` ``, so `Ratio<6, 12>` is a well-formed type that is not an `Adjustment` and cannot reach a bounded position. E0277: `ExactDivOdd` with an even divisor does not resolve rather than returning a wrong quotient, which pins the precondition `Strip2` establishes. |
| `probe_6_signed_bias_is_the_same_construction.rs` | The bias is signed (`31:399-400`'s `bias = B1 * B2`); does it need a different construction? | WORKS, and no: Coq's `Z ::= Z0 \| Zpos p \| Zneg p`, value-unique by the same induction because `p: Pos` excludes zero and therefore excludes negative zero. Positive multiplication (shift-and-add, doubling structural) asserted at 7 values, signed multiplication at all four sign combinations plus both zero sides, and the two spellings a sign-magnitude encoding would give for zero collapse to one type. Note where this lands against the identity contract: signed zero is real and wanted, on the DATUM side inside `Encoding::Canonical` (`31:370-374`); a numeral parameter is value-level and must not carry two zeros. |
| `probe_5_sealed_perimeter_lib.rs` + `probe_5b_downstream_cannot_widen_the_perimeter.rs` | Probe 2's uniqueness induction assumes the three constructors are the only impls. Is that checked? | Probe 5 WORKS (sealed `Pos`/`Nat` via a private supertrait). Probe 5b FAILS WITH E0277, committed refusing, compiled as a genuinely separate crate against probe 5's rlib: `` the trait bound `MySix: vu_sealed::sealed::PosSealed` is not satisfied ``. Without the seal a downstream crate reinstates probe 1's defect with `impl Pos for MySix { const VAL = 6; }`, one crate away and invisible. Two crates rather than two modules, because a private supertrait is visible inside its own crate and the claim is about the outside. |

Two-step build for probe 5/5b:

```
rustc --edition 2021 --crate-type lib probe_5_sealed_perimeter_lib.rs --out-dir <dir>
rustc --edition 2021 --crate-type lib --extern vu_sealed=<dir>/libvu_sealed.rlib \
      probe_5b_downstream_cannot_widen_the_perimeter.rs --out-dir <dir>
```

`vu_nat.rs` is the shared module probe 4, probe 4b and the sweep include with
`#[path]`, so those three read one text rather than three copies that can
decorrelate. `vu_gcd_maxmin.rs` is a submodule of it holding the ablation gcd
of section 4, split out at the 500-line smell threshold; it shares the tower
above rather than extending it. Probes 2 and 3 carry standalone copies on purpose: each is
evidence about one thing (the encoding, the gcd) and should compile with
nothing else present.

## The wall, re-verified rather than cited

The consolidation's droplist (`26:719-724`) records that width arithmetic
cannot be computed in type position from generic const parameters. I checked
it myself rather than reasoning from the citation, on three shapes, and it
holds on all three:

| Shape | Result |
|---|---|
| `25_probes/00`, an associated const as a const-generic argument | ``error: use of `const` in the type system not defined as `type const` `` |
| `25_probes/01`, `type const OUT: u16 = A + B` | `error: complex const arguments must be placed inside of a `const` block` |
| `25_probes/02`, the same inside a `const` block | `error: generic parameters may not be used in const operations` |
| mine: a `type const` projecting a recursive type-level chain back down into a numeral's own const parameter, so the numeral's identity would be a plain value | `error: generic parameters may not be used in const operations`, with the note `help: add #![feature(generic_const_args)]` |

The fourth is the one worth recording, because it is the shape that would have
made the whole obligation evaporate: if a trait-level calculator could project
its answer back into a `u64` const parameter, value-uniqueness would come free
from structural equality on that value and no normal form would need stating.
It cannot. The suggested feature is `generic_const_args`, which is neither
`min_generic_const_args` (permitted) nor `generic_const_exprs` (forbidden), and
is unvetted under `unstable-features.md`. So the numeral's identity is a type,
normalisation is a type-level computation, and the rest of this file follows.

## Diagnostic length, one measured pair

A type-level numeral is printed into every diagnostic that mentions it, and
the consolidation already treats rendered diagnostic length as a real cost
(it is what killed the three-parameter split, `26:33-35`). Measured on one
deliberate width mismatch (13 against 20), same rustc, same shape of demand:

| encoding | rendered bytes | what the message names |
|---|---|---|
| `UTerm` / `UInt<Hi, Lo>` | 759 | `` expected `UTerm`, found `UInt<UTerm, B1>` ``, an inner-node mismatch, with the outer note eliding two levels as `_` |
| `H` / `O<P>` / `I<P>` | 715 | `` expected `PhantomData<I<O<I<H>>>>`, found `PhantomData<O<O<I<O<H>>>>>` ``, both numerals in full |

Slightly shorter, and more usefully, it names the two numerals rather than
the first node at which their chains diverge. One pair is one pair; I did not
sweep this and would not read the 6 percent as a trend.

## The price

`price/gen.py` generates the sweep sources, `price/sweep.sh` runs it,
`price/results.csv` is the record. Generated sources and object files are
derived and not committed; the script regenerates them byte for byte from the
seeded generator. The one manual step is the prior-art baseline:

```
cargo +nightly-2026-05-28 new /tmp/tnbuild && add typenum = "1.20.1" && cargo build --offline
```

which must be built under the same pin, or `rustc` refuses the rlib with
E0514.

Build shape is `--emit=metadata`, which is trait solving with no codegen.
That is the honest measurement here, because type-level arithmetic is entirely
a trait-solving cost. Times are min of three runs, in milliseconds, with the
`count = 0` row subtracted as the fixed cost. Every instantiation is forced by
a const assertion against a Python-computed answer, so nothing is elided and
correctness is checked by the same run that times it.

| shape | 8-bit operands | 16-bit operands |
|---|---|---|
| `vu_gcd`, Stein on the value-unique encoding, tail form | 0.79 ms/composition | 5.08 ms/composition |
| `vu_gcd_maxmin`, same encoding, typenum's odd/odd formulation | 1.26 | 8.25 |
| `tn_gcd`, typenum's `Gcf`, the prior art | 2.69 | 15.55 |
| `vu_reduce`, the full reduction (gcd + exact division) | 2.19 | 12.07 |
| `vu_reduce_dyadic`, the same over power-of-two adjustments | not run | 0.50 |

Scaling is linear in the composition count over 0, 25, 50, 100, 200, 400 for
all three of the swept shapes; the 400-point figures above are representative
of the slope, not of a knee.

The ablation decomposes the 3.06x (16-bit) between the prior art and this
file's gcd into 1.65x from the odd/odd formulation and 1.87x from the encoding
and everything else about typenum's implementation. I do not claim the second
factor is purely the absence of `Trim`: it also contains typenum's `Unsigned`
and `NonZero` bounds, its `Gcf` alias, and the crate boundary. What the
ablation does establish is that neither half accounts for the win alone, and
that the larger half is the one the perimeter question is about.

Emitted symbols: zero, at `-C opt-level=2`, for 400 instantiations of either
`vu_gcd` or `vu_reduce`, measured with `nm -g` on the rlib. That is the
expected answer rather than a surprising one, since the whole tower is phantom
types and associated types with no values; it is recorded because "how many
symbols does this cost" is the question file 32's and file 34's measurements
answer for the other halves of the design, and the answer here is none. What
does grow is metadata: the empty crate's rlib is 53 KB, 400 `vu_gcd`
instantiations take it to 575 KB and 400 `vu_reduce` to 827 KB, roughly 1.3 KB
and 1.9 KB per composition of type names. That is a real downstream cost and it
is not measured here beyond its size.
