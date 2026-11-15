Source: kaski channel (coord/kaski of clause-dev), briefs/fixup/pl0-arvo-value-names.md, first posted 2026-11-12 22:31 UTC, read 2026-11-15.

# arvo's value names: the page the proxy, AR4 and AR7a are written against

Roadmap step PL0, 12 November 2026, revised the same night after two outside readers (`stamps-2-spj.md`, `stamps-2-torvalds.md`) read it blind of each other. Lands in arvo as `mock/research/202611122230_value-names.md` on `work/value-names`. Read at arvo `d841c5e`, kaski `0905ce823`, hilavitkutin `af823b7`; every kaski path below is a path at that kaski commit and dies at kaski's regroup, so it is always read with the commit. Nothing was compiled: every signature below is a sketch of a shape, and "const" means the item is meant to be callable in a const context, which AR2 has not yet shown.

This page decides names, argument order and constness. It does not decide the working arm (AR2), the stance (AR1), or who owns the kernel arithmetic vocabulary (call 10; written for default a, arvo owns it). The names sit in the working crate `arvo-numeral` unless a line says otherwise.

What may still change after AR2 reports, and only this: whether `Num` shows a second, defaulted type parameter for the carrier (arm A); the bound that code generic over a signature writes to call an operation (`S: Sole`, or `S: Sole<Repr = i64>` under arm A); whether `Lane` is declared as a plain trait or a const trait inside arvo (arm C); if no arm passes the consumer crate, whether `Num` becomes one type per ladder carrier with the consumer naming the carrier (arm D); and which of the two forms of the accumulator's term bound is built (the section on the exact accumulator; this one was added in the revision). No door name, operation name, argument order or alias name below changes under any of the five arms. The page is revised once more, then.

What the revision changed is said at each place, with what the page said before. In short: the accumulator's term bound is checked where the first page left it unchecked; both overflow markers ship at AR4 where the first page shipped one and marked a place for the other; `Index` and `Count` are two distinct types of the contract crate where the first page made them aliases of `USize`.

## The value type and its doors

```
pub trait Sole: DeclaredSignature {            // sealed; one impl per admitted width and slot family
    type Repr: Copy + Eq + Ord + Hash + Debug + Send + Sync + 'static;   // the ladder carrier
}
#[repr(transparent)]
pub struct Num<S: Sole>(S::Repr);              // field private; one bit pattern per member
```

`Num<S>` implements `Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug` and nothing else: no `Default`, no `Deref`, no `From` of a host integer, no operator traits on this page. A proxy implements exactly that set (check P6 reads it). `Hash` is there for a consumer's own tables and feeds no digest: a digest is fed by the contract crate's `Absorb` alone.

| Door | Sketch | Const | Meaning |
|---|---|---|---|
| `constant` | `Num::<S>::constant::<const V: i128>() -> Self` | yes | `V` is a slot index; a non-member fails the build |
| `adapted` | `Num::<S>::adapted(exact: Exact) -> Self` | yes | ingest: `adapt::<S>(exact, Dither::UNUSED)` (`arvo-format/src/apply.rs:250`) |
| `from_raw` | `Num::<S>::from_raw(raw: S::Repr) -> Self` | yes | total; `adapted` of the on-grid slot the bits name |
| `slot` | `v.slot() -> Slot` | yes | unwrap door, declared as one |
| `raw` | `v.raw() -> S::Repr` | yes | unwrap door, declared as one |

Only the wrapping and the saturating overflow policies, and the five deterministic rounding modes, are admitted on a value signature; `Clamp` and `Stochastic` are refused at build.

## Operations

All are inherent `const fn` on `Num<S>`, take `self` by value, and take one signature: `a.add(b)` across two signatures does not compile.

| Name | Sketch | Admitted for |
|---|---|---|
| `add`, `sub` | `(self, rhs: Self) -> Self` | where `is_addable` holds (`arvo-format/src/addition.rs:260`) |
| `neg` | `(self) -> Self` | the same |
| `mul` | `(self, rhs: Self) -> Self` | constant quantum, radix two, zero phase |
| `eq` | `(self, rhs: Self) -> Bool` | every format |
| `cmp` | `(self, rhs: Self) -> core::cmp::Ordering` | every format with a monotone slot map |
| `lt`, `le`, `gt`, `ge` | `(self, rhs: Self) -> Bool` | as `cmp` |
| `min`, `max` | `(self, rhs: Self) -> Self` | as `cmp` |

The law of each arithmetic operation is `slot(op(a, b)) == adapt::<S>(exact_op(slot(a), slot(b)), Dither::UNUSED)`. The operation is the total function the oracle is: the oracle `add` at head "carries no check and does not panic; a caller wanting the debug panic asks `panic_on_overflow`" (`arvo-format/src/addition.rs:247-248`, the function at `:251`).

Overflow in a debug build. Each overflow policy ships at AR4 as two marker types over one policy constant. Under the first, leaving the range is an invalid the typestate did not catch, and a debug build asserts, as the ratified row says of what "cannot be caught at compile time through the typestate, on debug builds, in every strategy" (arvo `mock/registry/ruling.toml:1052-1058`). Under the second, completion by the declared policy is the meaning of the operation and nothing asserts in any profile. The assertion asks a const verdict function, as head's `panic_on_overflow` is asked, and never lives inside the total operation. Every exhaustive law of AR4 and AR7a runs on the second marker; one test per policy shows through the verdict function that the first would assert, with no `#[should_panic]`. Call 5 then decides one thing: which of the two the plain spellings `Saturate` and `Wrap` name. The other marker of each pair gets its spelling in AR4's topic and enters this page at its revision; no name already on this page moves under either answer.

What this was and why it changed. The first page had one marker, an assertion in every debug build (call 5 default a), and "a place marked" for a second marker if call 5 were answered b. Both readers found the default unsafe to build on as worded. AR4's own proof is equality with the oracle at every signature of widths 1 to 8, where most pairs overflow, so an asserting `add` aborts the exhaustive law in the profile every helper runs (SPJ, call 5); kaski saturates by meaning (`kaski-engine-contracts/src/lane.rs:64-72` at `0905ce823` clamps in `narrow`), so a debug run of kaski on arvo aborts on ordinary data (Torvalds, call 5). Shipping both from the first landing redoes nothing under any answer.

## Identities

```
pub trait Identity<M> { const IDENTITY: Self; }     // new in arvo-numeral; no such trait is at head
pub struct Additive;  pub struct Multiplicative;
```

`Num<S>: Identity<Additive>` where the format has one (`has_additive_identity`, `arvo-format/src/format.rs:349`), `Identity<Multiplicative>` likewise. The call shape is the one hilavitkutin already writes, `<T as Identity<Additive>>::IDENTITY` (`hilavitkutin-api/src/dispatch_codegen.rs:201`). Whether the refusal on a format without one is a missing impl or a failed constant is AR4's to report; the name is the same under both.

## The exact accumulator

```
pub struct Wide<S: Sole>(/* the signed machine integer twice the carrier's width */);
impl<S: Sole> Num<S>  { pub const fn widen(self) -> Wide<S>; }
impl<S: Sole> Wide<S> {
    pub const ZERO: Self;
    pub const MAX_TERMS: USize;                         // members whose signed sum the register holds exactly
    pub const fn fits_add(self, rhs: Self) -> Bool;     // verdict: the exact sum is inside the register
    pub const fn fits_sub(self, rhs: Self) -> Bool;
    pub const fn add(self, rhs: Self) -> Self;          // exact where fits_add; a debug build asserts fits_add
    pub const fn sub(self, rhs: Self) -> Self;          // exact where fits_sub; a debug build asserts fits_sub
    pub const fn narrow(self) -> Num<S>;                // the one adaptation
}
```

`Wide<S>` implements `Clone, Copy, PartialEq, Eq, Debug`. Its law: `narrow` of an accumulated sum equals `adapt` of the exact sum of the slots, in every order, for every multiset of at most `MAX_TERMS` members. Every `widen` counts as one term, whether it is added or subtracted.

The term bound is checked. Two arvo rows bear on it: "Never any runtime checks, ever: invalids are caught at compile time" (`ruling.toml:949-954`, which carries its own exception for a reason a lead designer blesses) and the debug panic for "what cannot be caught at compile time through the typestate" (`ruling.toml:1052-1058`). A register that silently leaves its range past the bound is neither. The page states two forms and builds exactly one, picked at the revision after AR2 reports, before any author is written against `Lane`.

Form D, the cheapest sound form this page can state, and the one it is written for:

1. `add` and `sub` on the accumulator assert `fits_add` and `fits_sub` in a debug build. In a release build the register's declared mode past the bound is two's-complement wrap, stated in the type's documentation. Tests assert on the verdict functions, not on a panic.
2. `MAX_TERMS` is a constant of the accumulator and an associated constant of `Lane`. By this page's arithmetic, not by a sketch, it is two to the power of the carrier's width for a register twice as wide (65536 for a 16 bit lane), held at the platform count's maximum where that is smaller. AR7a states the value per ladder carrier as a test.
3. A kernel form whose term count is a constant states it, and the plumbing refuses at build a kernel whose count exceeds the format's bound: the window kernel's footprint is such a constant (lane V, slice V3), and the refusal is an inline `const` assertion where the kernel is instantiated, with no forbidden gate.
4. A form whose term count is a run-time value never accumulates in `Wide`: the fold kernel's monoid is a declared total monoid (V3). A sum over an extent owned by the store has no meaning in this vocabulary.
5. Two laws beside the one above: at exactly `MAX_TERMS` members of extreme magnitude the sum is exact; one term more and the verdict function answers no. The mutant: an accumulator whose verdict always answers yes.

Form T, the better form if it builds: the term count is carried in the accumulator's type (`Wide<S, N>`, an addition yielding the sum of the two counts, `narrow` offered only where the count is within the bound), by a type-level count with no `generic_const_exprs`. Then nothing past the bound can be written and items 1 and 3 of form D fall away. Under form T the type of every intermediate differs, so `Lane::Wide` becomes a family and every author's accumulation is spelled against it: the choice cannot be made after authors exist. The roadmap's correction C2 asks AR2 whether such a count builds in the consumer crate with no feature line; if AR2 reports that it does not, or does not report on it, form D is what ships.

What this was and why it changed. The first page said "Past `MAX_TERMS` nothing is promised; the register does not check." Both readers refused it against the two rows above (SPJ, P0-3: unsound, refused; Torvalds: an unchecked overflow against `:1052` and `:949`, a matter for arvo's panel). The case that makes it reachable is a narrow lane (call 9 b) summed over a run-time extent. kaski did not meet it because its only accumulator is an `i128` over an `i64` (`lane.rs:44`), and its sentence that "The wide type holds a sum of any number of lanes" (`lane.rs:16`) is false of any finite register and is not carried over.

The trait form, which kernels generic over a number are bounded by, keeps the item names kaski ships (`kaski-engine-contracts/src/lane.rs:17-40`), less `wide_div` and `Layer`. `Layer` is kaski vocabulary. Division by a run-time count is a named fault; the shipped diffusion uses it (`kaski-engine-lattice/src/tiles/block.rs:256-259`), so no author written against this trait reproduces that kernel's bits.

```
pub trait Lane: Copy + Send + Sync + 'static {
    type Wide: Copy + PartialEq + Debug + Send + Sync + 'static;
    const WIDE_ZERO: Self::Wide;
    const MAX_TERMS: USize;                              // form D; under form T the bound is in the type
    fn widen(self) -> Self::Wide;
    fn wide_add(a: Self::Wide, b: Self::Wide) -> Self::Wide;   // asserts in a debug build past the register
    fn wide_sub(a: Self::Wide, b: Self::Wide) -> Self::Wide;   // the same
    fn narrow(wide: Self::Wide) -> Self;
}
impl<S: Sole> Lane for Num<S> { type Wide = Wide<S>; /* forwards to the inherent items */ }
```

Trait methods are not const on this page: a kernel in a crate with no feature line calls them in ordinary functions only. A product on the trait arrives later as its own trait, which edits nobody. Place marked for call 10 answered b: `Lane` moves to the contract crate and `Num<S>` implements it there; the names stay.

## Aliases

| Name | Sketch | Note |
|---|---|---|
| `Uint<const BITS: u32, A>` | `Num<Signature<points::UFixed<BITS, 0>, A>>` | `A: Adaptation` |
| `Int<const BITS: u32, A>` | `Num<Signature<points::Integer<BITS>, A>>` | |
| `USize` | `Uint<{ usize::BITS }, Adapt<TowardZero, Wrap>>` | the value; `arvo_format::points::USize` is the format and keeps its name |
| `ISize` | `Int<{ usize::BITS }, Adapt<TowardZero, Wrap>>` | |
| `Bool` | re-export of `arvo_format::Bool` | doors `of`, `get`, `and`, `or`, `not`, `FALSE`, `TRUE` (`width.rs:95-131`) |

`USize` is the same type as the literal-width alias on each target (arvo `ruling.toml:1750-1756`), so its `Repr` is the ladder carrier, `u64` or `u32`, never `usize`. Two more doors therefore exist on the platform-width values only, both const: `USize::from_host(n: usize) -> Self` and `v.host() -> usize`. In the `Wrap` column above, `Wrap` is whichever marker call 5 gives the plain spelling.

Index and count are not arvo names, and they are not aliases. `Index` and `Count` are two distinct types the contract crate declares, each with one private `USize` field and named conversions; they stay the contract crate's own types after the swap, with arvo's `USize` beneath them. What this was and why it changed: the first page made both aliases of one `USize`. Both readers called that the wrong direction of a one-way choice (SPJ, P0-1; Torvalds, section 4): as aliases, a count passed where an index goes compiles in every implementor, and turning an alias into a distinct type later edits every frozen implementor, while merging distinct types later edits nobody. The roadmap's own text for HL0a already said "index and count types with a private field".

Fixed point aliases take a fraction length and are AR7a's to name.

## What the proxy module holds until the swap

One private-field type `USize` with `from_host`, `host`, `add`, `sub`, `eq`, `cmp`, `lt`, `le`, `gt`, `ge`, `min`, `max`, `Identity<Additive>` and the trait set above. `Index` and `Count` as two distinct private-field types over it, each with `from_host`, `host`, the comparisons, and only the arithmetic that keeps its meaning (an index plus a count is an index, an index minus an index is a count, a count plus or minus a count is a count; the exact list is the surface fixture's). `Bool` as above. The `Lane` trait as written here, `MAX_TERMS` and the debug assertions included, so that an author written against the proxy is not edited when the bound arrives. `Identity`, `Additive`. It exposes no `raw` and no `from_raw` on `USize`, because their type differs per target after the swap. Each item carries a marker naming `arvo-numeral`. After AR4 the proxy `USize` becomes an alias of arvo's and `Index` and `Count` keep their own types over it; after AR7a `Lane` becomes a re-export.

The two host doors are the only places a bare `usize` stands at a public position of the contract crate. hilavitkutin carries `[lints.no-bare-numeric]` (`mockspace.toml:220`), which at head is off at commit and at warn for build and push. If it reports the doors, the brief's answer is to post the line and never to lower the lint.

## Raised by one reader

- Whether call 5 is the lead designer's at all (Torvalds). arvo's own row routes open canon questions to two independent agreements "without him" (`ruling.toml:1713`), which would make call 5 a question for AR1's kind of second read and no morning question. SPJ answers it as his call. Nothing on this page depends on which: both markers ship. Settled by the coordinator reading `:1713` against the roadmap's call list.
- That associativity must stay a verdict and never become a marker beside `Identity<M>`, because the registry retired declared law markers (SPJ, P0-6, citing the stance file's 3.3). Not checked by this revision against the registry. Settled by AR4's helper asking `cargo mock canon find` for the retirement row before `Identity` lands.
- That no sketch tonight tests a crate with no feature line that is generic over `L: Lane` and calls `widen`, `wide_add`, `wide_sub`, `narrow` against arvo's arm, which is exactly frozen author 1 (Torvalds, the AR2 correction). Settled by AR2 adding that consumer case; until then the trait form is the least tested thing on this page.
- That an arm whose `add` converts to a slot and calls the oracle passes every equality test by construction and must be disqualified (Torvalds). A matter for AR2's brief, no name here.

## What this page could not establish

Whether anything on it compiles. Whether form T builds. The value of `MAX_TERMS` per carrier beyond the arithmetic stated. Whether hilavitkutin's bare-number lint reports a door. The spelling of the second overflow marker of each pair.
