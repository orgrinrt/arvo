# 100_probes outcomes

Toolchain `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `aarch64-apple-darwin`, resolved from
`rust-toolchain.toml` inside the tree. The identical `rustc --version` outside the tree resolves to
stable `1.94.0`, which does not build the const-position work here. HEAD `75710b6`.

Every Rust probe compiles with **zero `#![feature(...)]` gates**. Every numeric claim is exact
integer or exact rational arithmetic; no float enters any load-bearing comparison.

| Probe | Subject | Outcome |
|---|---|---|
| 1 | D4's shape over the capacity `91` section 1.26 settled | WORKS, with one hole found |
| 2 | Where the agreement check actually bites | FAILS as written, repair compiled |
| 3 | Which axis `Layout::Bitpacked` packs at rank > 1 | Flattened dominates, condition derived exactly |
| 4 | Geometry's accumulator widths, and D10's rotor count | Composes freely; D10's count wrong from rank 4 |
| 5 | The identity rotation's representability | Needs no new mechanism, two routes priced |
| 6 | The rotor's extent in type position | WORKS, and refuses D10's literal |
| 7 | The two-vocabulary rank-2 shape | Reproduced, executed |
| 8 | What the rank recursion costs | Flat compile time, folds to a constant at small COUNT |

---

## Probe 1, `probe_1_shape_over_the_settled_capacity.rs`

Compiles clean. Every const assertion held.

- **A.** Rank, per-axis extents and the element count are a fold in const position, no gate. The
  rectangle `Matrix<W, C>` cannot express (`202607281126`) is `Axis<E3, Axis<E4, Scalar>>`, COUNT 12.
- **B.** D4's `Cons<H, T>::Array<E> = H::Array<T::Array<E>>` survives the `Slot<N, K>` split
  verbatim. `size_of::<Store345>() == 240 == COUNT * 4`, agreeing by two independent routes (the
  language's nested-array layout, and a fold over the `Nat` values).
- **C.** A function generic over rank exists, one body for every rank, in two flavours: one walking
  the storage, one reading only the shape (`stride`). `202607281127` calls this "the concrete missing
  piece ... with no worked example anywhere in arvo".
- **D, and this is the finding.** A shape whose middle axis declares a `Nat` of 4 against a literal
  of 7 has `COUNT == 12` and `size_of(Store) == 21`. **Both compile. Both are const-evaluable. They
  disagree, and nothing raises.**

## Probe 2, `probe_2_the_agreement_door_is_not_one_door.rs`

Compiles clean. Three refusals checked by uncommenting, each reproduced below.

- **A.** The reason. Section 1.26's door is `Slot`'s **inherent** `build`; D4's composition calls the
  **trait** method `Capacity::filled`, which has no check. Through the inherent door the lying type
  is refused:

  ```
  error[E0080]: evaluation panicked: capacity's length disagrees with its value
     evaluation of `leaking::Slot::<Pz<O<O<H>>>, 7>::build::<u8>::{constant#1}` failed here
  ```

  Through the trait it compiles and returns `[u8; 7]`.

- **B.** The repair: lift the agreement to `const AGREES: bool` on `Capacity` itself. The trait route
  is then refused too:

  ```
  error[E0080]: evaluation panicked: capacity's declared length disagrees with its value
     evaluation of `<sealed_door::Slot<Pz<O<O<H>>>, 7> as sealed_door::Capacity>::AGREES` failed here
  ```

- **C, the honest scope.** The repair as stated fires only where `AGREES` is reached, so a lying
  capacity whose `COUNT` is merely *read* still compiles (checked: `EXIT=0`). Adding
  `assert!(Hd::AGREES)` inside `Axis`'s own `COUNT` closes that too, and the const read alone then
  refuses, with no construction anywhere:

  ```
  note: erroneous constant encountered
  175 |  assert!(Hd::AGREES, "axis capacity's length disagrees with its value");
  ```

## Probe 3, `probe_3_which_axis_is_packed.py`

`91:563-570` ratifies the group arithmetic `P = 8/gcd(W, 8)`, `G = W*P/8` for one dimension and
`91:645-651` prices the mutation gap against "one tail-group region for the whole column". Neither
sentence has a rank.

- **A. My first statement of the coincidence condition was FALSE, and the exhaustive sweep caught
  it.** "Byte-identical exactly when the innermost row is byte-aligned" gives 5632 mismatches over
  the 131072-case grid W in 1..64, inner in 1..64, outer in 1..32. Adding "or outer == 1" halves it
  to 2816. The exact condition, **zero mismatches over the same grid**, is

  > the two readings coincide exactly when `outer * rowpad < 8`, that is, when the whole per-axis
  > padding fits inside one byte.

  With a one-line proof: per-axis padding is `outer * ((-x) mod 8)` and flattened padding is
  `(-outer*x) mod 8`, which is the same quantity reduced mod 8, so they are equal exactly when the
  first is already below 8.

- **B.** Per-axis is never cheaper: 0 of 131072 cases. Largest per-axis excess 28 bytes at
  `(W, inner, outer) = (1, 1, 32)`, an eight-fold footprint.
- **C.** Under the flattened reading the rank-N container **is** the rank-1 container of COUNT
  elements. `P` and `G` depend on `W` alone, so nothing about them is rank-sensitive; 0 violations.
- **D.** The write granule becomes a shape fact: an outer-axis partition is legal only when
  `inner mod P == 0`. Of the twelve model shapes, six are illegal, including `3x4 of 7-bit` and
  `17x13 of 23-bit`.

## Probe 4, `probe_4_geometry_composes_and_the_rotor_count.py`

Exact Clifford algebra over `Fraction`, Euclidean signature, blades as bitmasks.

- **A.** A general rotor occupies the whole even subalgebra, `2^(n-1)` components, not
  `1 + n(n-1)/2`. The two counts agree at n = 2 (2) and n = 3 (4) and diverge from n = 4 (8 against
  7). **Rotor storage first exceeds matrix storage at n = 7** (64 against 49), which reverses D10's
  own reason for rejecting the matrix.
- **B.** Not bookkeeping. `R = (3/5 + 4/5 e01)(5/13 + 12/13 e23)` has grade-4 part `48/65`, nonzero,
  and `R R~ = 1`, so it is a legitimate unit rotor that `1 + n(n-1)/2` slots cannot hold.
- **C.** Dense term counts. Rotor apply at n = 3 is 28 against a matrix's 9; at n = 4, 96 against 16.
  These are upper bounds (a sparse sandwich exploits that `R x R~` is grade 1); the order is what the
  comparison rests on.
- **D.** Every geometric operation lands in a growth class `91` already has: dot product and rotor
  compose at `2p + ceil(log2 terms)`, rotor sandwich at `3p + ceil(log2 terms)`, affine apply at
  `max(2p + ceil(log2 n), p + F) + 1`. **No new growth class.** The one operation that leaves the
  multilinear world is renormalisation, and its exponential class is reached in the statement and
  never in the storage, because normalisation quantises rather than materialising an exact quotient.
- **E.** Exact composition depth is exponential for **every** representation, identically: 16 bits
  becomes 1036 at depth 6. So depth is not a discriminator between representations; it is the
  quantiser-site story `91` section 1.14 already tells.
- **F.** The closed-form bivector exponential, which is what makes a rotor cheap to interpolate,
  survives to rank 4 (closed-form split by the dual). At rank 5 and up it is a degree-`floor(n/2)`
  root problem, and at rank 10 Abel-Ruffini forecloses it, the same ceiling this round already
  recorded for exact distance to a cubic Bezier.

## Probe 5, `probe_5_the_closed_interval_numeral.rs`

Compiles clean, every assertion in const position.

- **A.** Dyadic purely fractional misses `+1` at every F from 1 to 24, signed and unsigned, and the
  signed form reaches `-1`. The gap to `+1` is exactly one quantum at every width, which is the worst
  place for it: no rounding mode recovers it.
- **B.** The ratified rational `Adjustment` (`40:493`, `91:153-154`) represents **both** endpoints at
  the same container width, at `A = 1/(r^F - 1)`. That is Direct3D's UNORM rule, which this round
  already recorded from the sibling colour pass. **No new mechanism: it is a value of a parameter the
  design already seals.**
- **C.** The cost. The adjustment composes multiplicatively, so a product lands at `1/(r^F - 1)^2`
  and renormalising is division by the fixed nonzero representable constant `r^F - 1`, which
  `91:288-291` names the exact subfamily "at zero new mechanism". Checked: one acts as one exactly,
  at every operand, at every width to 20.
- **D.** The other route, one bit of integer headroom, costs `2^(n-1)` bits for a rotor against `n^2`
  for a matrix, and crosses over at the same rank 7.

## Probe 6, `probe_6_the_rotor_extent_reaches_type_position.rs`

Compiles clean, zero gates.

- **A.** `2^rank` reaches **type position** by structural recursion on the list's constructors, with
  no arithmetic expression in any type: `TwoPow` for `Axis<Hd, Tl>` is `O<Tl::P>`, one constructor
  per step. The rotor's extent is the tail's `TwoPow`, so the decrement is the list's own shape.
  Checked at ranks 2, 3, 4, 5, 8 against 2, 4, 8, 16, 128.
- **B.** D10's count derived alongside for comparison inside one compilation. The two agree at rank
  2 and 3 and separate from rank 4, which is the separation requirement's own shape: **a model
  checked only at rank 3, the rank everyone reaches for, cannot tell the two counts apart.**
- **C.** The derived extent pairs with a literal exactly as section 1.26 requires, and the wrong
  literal is refused, naming the derived `Nat` in the message:

  ```
  error[E0080]: evaluation panicked: capacity's declared length disagrees with its value
     evaluation of `<Slot<Pz<O<O<O<H>>>>, 7> as Capacity>::AGREES` failed here
  ```

  `Pz<O<O<O<H>>>>` is 8. The literal 7 is D10's own count at rank 4.

## Probe 7, `probe_7_the_unchecked_axis.rs`

Built and run. Standalone; nothing depends on building the tree.

Factual check at source, why-evidence only: `arvo-bitmask/src/matrix.rs:34` declares
`pub rows: C::Array<Mask<W>>`, `:52-55` guards the row index against `cap_size(C::CAP)`, `:63-67`
guards the same row index and then passes the **column** index to `insert` unguarded. The column's
only bound is `W`'s physical bit width.

```
CLAIM A  guarded axis: write at row 9 of 4 is refused, image unchanged  OK
CLAIM B  unguarded axis: write at column 9 of 4 LANDS and is readable  OK
CLAIM C  value-equal at every declared index, byte images differ      OK
         c = [2, 0, 0, 0, 0, 0, 0, 0]
         d = [2, 2, 0, 0, 0, 0, 0, 0]
CLAIM D  one guard from the shape covers both axes, images agree      OK
```

Claim C is the mutation gap `91:598-627` quantifies for one dimension, reached at rank 2 through an
ordinary **safe** call, with no unsafe, no transmute and no niche. The reason it is reachable is the
asymmetry: one axis is a `Capacity` and has a shape to be checked against, the other is a `Bits<N>`
width and does not.

## Probe 8, `probe_8_what_the_rank_recursion_costs.sh`

Compile time is `--emit=metadata`, best of three, so it measures trait-solver work and not codegen.
Instruction counts are `-O --emit=asm` on `aarch64-apple-darwin`, counted over the mangled `total`
body.

**Compile time is flat in rank.** 82, 81, 80, 82, 85, 82, 82, 84, 82 ms at ranks 1, 2, 4, 6, 8, 10,
12, 14, 16. Metadata grows linearly at roughly 142 bytes per axis (24584 bytes at rank 1, 26715 at
rank 16). The marginal cost of an axis is below rustc's own process-start noise at every rank
measured. This answers the round's own worry directly (`202607281220` addendum: the
`shapeless`/`frunk` compile-time warning "does not transfer ... a per-width impl table is flat
breadth", and D4's cost question is "real, narrower than feared").

**The rank-generic fold folds away entirely at small COUNT.** At rank 3, COUNT 8, `total` is five
instructions:

```
	mov	w8, w0
	add	x8, x8, x8
	add	x8, x8, x8
	lsl	x0, x8, #1
	ret
```

At rank 8, COUNT 256, thirteen instructions, vectorised to NEON (`uaddl.2d`, four `add.2d`). The
nested-array build and the recursive fold are both gone. This is D43's own "a per-element surface
could look like it forfeits the cheap path; it does not" measured rather than asserted.

**Where it stops being free, and what the variable actually is.** Holding COUNT fixed and varying
rank:

| COUNT | rank 1 | rank 2 | rank 3 | rank 4 | rank 6 | rank 8 | rank 12 |
|---|---|---|---|---|---|---|---|
| 256 | 62 | 11 | | 11 | | 13 | |
| 4096 | 44 | 76 | 248 | 168 | 423 | | 907 |
| 65536 | | 79 | | 1212 | | | |

So emitted size is a function of **both** rank and COUNT, non-monotonically in rank (rank 1 at COUNT
256 emits 62 instructions where rank 2 emits 11, because the nested case vectorises and the flat one
does not). This is a codegen artifact, not a design fact, and belongs in `mock/benches/` per
`bench-in-bench-harness-never-sketches.md` rather than being read off here.

The stack allocation is a function of COUNT alone: 262144 bytes at COUNT 65536 for both rank 2 and
rank 4, with a stack-probe loop. **A dense shape is a stack object**, so `COUNT * size_of(E)` runs
into the platform's stack limit and not into anything the design controls.

Two cases did not finish inside the probe's time budget (COUNT 65536 at rank 8 and rank 16). Recorded
as unmeasured rather than dropped.

**One incidental result worth keeping.** While generating the fixed-COUNT cases I wrote
`O<O<O<O<O<O<O<H>>>>>>>` (128) for an extent of 256. The agreement repair from probe 2 caught it, at
compile time, unprompted, before any number was read:

```
error[E0080]: evaluation panicked: assertion failed: N::VAL == K
  --> fix_1_256.rs:23:26
```

That is the mechanism doing its job against a real mistake rather than a constructed one.
