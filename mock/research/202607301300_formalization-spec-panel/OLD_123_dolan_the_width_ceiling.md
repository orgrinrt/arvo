# 123. The width ceiling: what bounds a total, and what the design can honour

The question handed over is whether arvo should put a ceiling on the total width a consumer may write,
after file 119 found that the precision bridge bounds each width and not their sum, so `UFixed<40, 30>`
compiles today at a total of 70 with nothing anywhere admitting 70 (`119:236-245`, carried into the open
list at `110:5424-5431`). Op supplied an instinct as input rather than as a ruling: no ceiling, document
ordinarily, whatever width one needs.

The derivation below says the instinct is right about the half that is open and wrong about the half that
is closed, and the closed half was closed before the question was asked. **Arvo already has a ceiling on
each written width, it is forced rather than chosen, and it cannot be removed.** What is genuinely open is
whether a second and lower ceiling is added on the sum, and the answer there is no, on three compiled
grounds rather than on agreement with the person who asked.

Everything below was compiled under the pin (`rustc 1.98.0-nightly (57d06900f 2026-05-27)`,
`--edition 2024 --crate-type=lib`) against a rebuilt tower, its carry-chain addition, and a generated
bridge table, outside the repository tree. `mock/crates` untouched.

## 1. The brief's own claims, checked before reasoning from them

Three checked, two hold, one is misattributed. `110_consolidation_eleven.md` is 6916 lines. The
`UFixed<40, 30>` case reproduces on a freshly written tower and a 64-row table:
`<PrecisionOf<40, 30> as Nat>::VAL == 70` at exit 0, and so does the harder case
`<PrecisionOf<63, 63> as Nat>::VAL == 126`, twice the table's top row. The brief credits files 119 and 122
with finding the gap; **119 found it alone** (`119:234-264`) and 122 reproduced it into the open list
(`122:425-431`). Nothing turns on that, but a later reader tracing provenance should go to 119.

The brief's framing sentence, "it bounds each width separately and not their sum", is exact and is the
only part of the question that was ever open.

## 2. What actually bounds the total today

Four bounds exist and they sit at very different heights. Only one of them is a decision.

**The table's row count, and this is the one that is forced.** The bridge is an enumeration, one impl per
admitted width, because an impl is the only case split Rust has over a const parameter and file 119
compiled every other route shut: recursion needs a const expression in type position, its cases overlap at
`E0119` regardless, a `type const` body may not compute from a generic parameter, and separating the base
case needs full `specialization`, which `unstable-features.md` forbids (`119:56-121`). **An enumeration
has a largest row.** So the per-width ceiling is not a policy arvo adopted; it is the shape of the only
mechanism available, and no wording removes it.

**The const parameter's own type**, above that. At `u16` the largest nameable width is 65535, and the
refusal is rustc's, deny by default:

```
error: literal out of range for `u16`
   --> wrap2.rs:144:28
144 | pub fn sig(_x: PrecisionOf<65549, 3>) {}
    = note: `#[deny(overflowing_literals)]` on by default
```

The dangerous case is the one that would wrap onto an admitted row, and it is the case the lint catches.
Forwarding a wider const through a cast is refused separately, by the ban that forces the table in the
first place (`error: generic parameters may not be used in const operations`, with the `generic_const_exprs`
suggestion arvo cannot take).

**Nothing bounds the sum**, which is the finding under review, reproduced above. The sum is produced by
the tower's addition and never touches the table, so the greatest total width a consumer can write today
is twice the table's top row.

**`Pos`'s two ceilings are nowhere near.** Structural nameability stops at 128 bits and readout at
`2^64 - 1` (`110:2436-2440`), and those are ceilings on a magnitude, not on a width. A width would have to
exceed `2^64` to meet either. The full `u16` table compiles with every row present, which verifies the
depth question in passing: a width of 65535 is a numeral 16 constructors deep against a limit of 129.

## 3. Whether no-ceiling is available

On the sum, entirely. On each written width, no, and the honest answer to op is that the word "no" is
unavailable rather than unwise. What is available instead is a ceiling set so high that no consumer meets
it, and **the design's own pricing pillar removes the only argument against setting it there.**

The table, built alone, timed with `/usr/bin/time`, three runs after the first, spread under 10 ms:

| rows | table alone | with per-row assertions | peak RSS | greatest writable total |
|---|---|---|---|---|
| 256 | 0.05 s | 0.07 s | | 510 |
| 1024 | 0.13 s | 0.26 s | | 2046 |
| 4096 | 0.94 s | 1.60 s | | 8190 |
| 8192 | 3.06 s | | 234 MB | 16382 |
| 16384 | 11.78 s | 17.31 s | 388 MB, 714 MB | 32766 |
| 32768 | 65.46 s | | 702 MB | 65534 |
| 65536 | 279.62 s | | 1.30 GB | 131070 |

The first three rows reproduce file 119's independently (`119:123-138`) to within 0.16 s, which is worth
recording because 119's reading of them does not survive the extension: **the cost is not linear.** Each
doubling past 4096 costs between 3.3x and 5.6x, so the curve is roughly quadratic, the same character the
document already measured for the per-width container dispatch (`110:5513-5515`). The whole `u16` range is
reachable: 65536 rows, exit 0, 279.62 s, 1.30 GB, paid once by the crate that declares the carrier.

Against op's own ratified statement of what compile time is for (`110:501-508`, verbatim: "Compile time is
nothing. That can be literal minutes for all we care"), 279 seconds is inside the licence, explicitly.

**The number that does argue is the downstream one, and it is small.** The table built as an rlib, then a
separate crate compiled against it, three runs:

| table rows | bare `extern crate` | 64 distinct compositions | per composition |
|---|---|---|---|
| 1024 | 0.04 s | 0.12 s | 1.25 ms |
| 4096 | 0.04 s | 0.12 s | 1.25 ms |
| 16384 | 0.04 s | 0.22 s | 2.8 ms |

Linking the table costs a consumer nothing at any size; metadata is loaded lazily and the bare figure does
not move. Using it costs a surcharge per distinct composition, flat to 4096 rows and roughly doubling by
16384. Set against the ratified cost model's dyadic row at 2.1 to 2.3 ms per composition
(`110:3698-3701`), a 4096-row table is under that model's own noise and a 16384-row table roughly doubles
what a composition costs. That is the one measurement that says a table has a sensible operating point,
and it says the operating point is a few thousand rather than a few tens of thousands.

## 4. What the extremes do, and which failure a consumer can read

A width outside the table refuses legibly, and `#[diagnostic::on_unimplemented]` reaches it:

```
error[E0277]: width `Idx<200>` is outside the widths arvo admits
    = note: a written width must lie in 0 ..= 63
    = help: the following other types implement trait `AdmittedWidth`:
              Idx<0>  Idx<10>  Idx<11>  ... and 56 others
```

Every token there reads as a number. **The sum refusal does not**, and this is the finding that decides
the open half. Emitting file 119's proposed marker, one line per row, and writing the case it exists to
catch:

```
error[E0277]: the total width `I + F` this composition needs is outside the widths arvo admits
    = note: a total width must lie in 0 ..= 63
help: the trait `AdmittedPrecision` is not implemented for `Pz<O<I<I<O<O<O<H>>>>>>>`
    = help: the following other types implement trait `AdmittedPrecision`:
              Pz<H>  Pz<I<H>>  Pz<I<I<H>>>  ... and 55 others
```

The message and the note are recoverable, exactly as 119 predicted (`119:284-290`). The `help:` lines are
not, and there are ten of them: the consumer wrote `UFixed<40, 30>` and is shown eleven numeral trees, none
of which reads as a number. Against the consumer-facing half of the bar (`110:403-406`, op's "invisible for
the most part to downstream consumers"), that is a refusal paid for in the one place the bar is strictest.

One legibility note on the outer bound, worth a sentence and not more. When an over-`u16` literal wraps
onto a width the table does not hold, the type error fires before the lint pass and the consumer is told
about the wrapped value:

```
pub fn huge(_x: PrecisionOf<70000, 3>) {}
error[E0277]: width `Idx<4464>` is outside the widths arvo admits
```

`70000` appears nowhere. The wrap onto an admitted row, which is the case that would otherwise compile
silently, is caught by `overflowing_literals` as shown above, so this is a confusing message rather than a
hole.

## 5. The one thing that could force a sum ceiling, and why the design already removed it

If any mechanism keyed on the total width were itself a per-width menu, the sum would need a row and the
ceiling on the total would be forced down to the table's own top. The candidate is the container:
`110:2996-3004` makes the container width "a type-valued projection of the stored width through the
dispatch menu", and a menu is a table.

**The design has already ruled against that menu, on its own measurement.** `110:5513-5515`: the per-width
container alternative "is quadratic in its ceiling: 0.42 s at 256 widths, 5.3 s at 1024, 116 s at 4096,
and past 25 minutes at 8192, paid by every build of every consumer forever, which is the whole reason the
structural form is the design."

The structural form removes the forcing, and this is compiled rather than argued. A carrier is reachable
from any numeral by **five impls**, three on `Pos` and two on `Nat`, with no table row, no
`generic_const_exprs`, and `size_of` checked at three widths including two the table does not hold:

```rust
impl Carrier for H { type Out = Unit; }
impl<P: Pos + Carrier> Carrier for O<P> { type Out = Twice<<P as Carrier>::Out>; }
impl<P: Pos + Carrier> Carrier for I<P> { type Out = Succ<Twice<<P as Carrier>::Out>>; }

const _: () = assert!(size_of::<CarrierOf<PrecisionOf<13, 3>>>()  == 16);   // a row
const _: () = assert!(size_of::<CarrierOf<PrecisionOf<40, 30>>>() == 70);   // no row
const _: () = assert!(size_of::<CarrierOf<PrecisionOf<63, 63>>>() == 126);  // twice the top row
```

Exit 0, and at a 4096-row table the same shape carries a width of 8190 for a further 0.5 s. The nested-pair
carrier is a demonstration and not a proposal; what it establishes is that totality over `0 ..= 2 * W_MAX`
is available from a constant number of impls, so **the container does not force a sum ceiling** and the two
open items are connected: the container fix that op withheld at `68b` on scope grounds is the same fix that
keeps the sum unbounded. If the per-width menu were ever restored, the sum ceiling comes back with it, not
as a choice.

## 6. The two options, priced

**Option A. One range, on each written width, and no separate bound on the total.** The table's range is
the only stated ceiling; a total is the sum of two written widths and lies in `0 ..= 2 * W_MAX` by
arithmetic rather than by declaration. Nothing is added to the design and one paragraph is added to the
canon. Op picks `W_MAX`, and the measurement above says 4095 costs 0.94 s once and nothing measurable per
composition, which reaches a total of 8190 bits.

What it costs: the design admits totals it never enumerated, so every mechanism keyed on a total must be
total over twice the table's range. Section 5 shows that is available in five impls, and shows the one
mechanism that would not have been is already rejected on its own measurement.

**Option B. Bound the total as well, at the same number, one extra emitted marker per row.** Its case is
real and worth stating at strength: one number would mean one thing in both positions, and every numeral
that reaches a lowering would be a row the table enumerated and asserted beside itself, which is bin one of
the four-bin ledger rather than an adder's word for it. Under A, the numerals from `W_MAX + 1` to
`2 * W_MAX` are produced by the addition and carry no per-row assertion at all.

**Three compiled facts refuse it, and the first two are fatal.**

It refuses a legitimate product. `mulnum` sums the operand precisions, so a 20-by-20 fixed-point value
squared has precision 80, and with the marker in place that is the same illegible error as above:
`the trait AdmittedPrecision is not implemented for Pz<O<O<O<O<I<O<H>>>>>>>`. The document already states
this objection in prose (`110:5427-5429`); it is now compiled.

It cannot be narrowed to the written site to dodge that. The obvious repair is to hang the bound on the
public alias rather than on the numeral, and Rust refuses outright:

```
error: where clauses are not allowed after the type for type aliases
    = help: add `#![feature(lazy_type_alias)]` to the crate attributes to enable
```

`lazy_type_alias` appears on no list in `unstable-features.md`, which makes it unvetted and unshippable by
that rule's own terms. So B's only home is the numeral, which is where the product objection bites. Giving
the written site a struct of its own instead would change the public spelling and break D48 and D31
(`110:3477-3485`), and it makes B "one line per row plus a new type in the design" rather than one line per
row.

Its verification argument does not survive either. B does not close the gap it names, it moves where the
gap starts: precisions past `2 * W_MAX` are reachable through multiplication under either option, so
neither option has a row for every numeral that reaches a lowering. The adder is 21 impls whose correctness
is one law over the encoding, not a thousand separate claims, and that is where the argument belongs.

And its refusal is unreadable, per section 4, which is the cost it pays in the place the bar is strictest.

**A, and `16d:14-15`'s tiebreaker points the same way**: A keeps the shape that exists and costs one
paragraph, B adds a mechanism and a type. What remains genuinely op's is one number, `W_MAX`, and the
measurement above is the input to it rather than a recommendation dressed as one.

## 7. The sentence the canon should carry

Documenting ordinarily means saying what a wide pair costs, in stored width, in compile time, and in what
the lowering has to hold. One paragraph, at the bridge, with `W_MAX` filled in at op's number (4095 below,
because that is the operating point the surcharge measurement names):

> **Widths.** A written width lies in `0 ..= 4095`. A total width is the sum of two written widths and so
> lies in `0 ..= 8190`; it is not separately bounded, and every mechanism keyed on a total width is total
> over that range rather than over the table's own. Both numbers come from one place. The bridge from a
> written width to a type-level numeral is a table with one row per admitted width, and it is a table
> because an impl is the only case split Rust has over a const parameter and every other route needs a
> feature arvo forbids. The range is therefore the size of an enumeration rather than a judgement about
> what a workload should need, and it is set where the table stops being free rather than where a consumer
> stops being reasonable: 0.94 s to build, once, in the crate that declares the carrier, nothing at all for
> a downstream crate to link, and about 1.25 ms per distinct composition to use, against 2.1 ms for the
> composition itself. Nothing downstream sees the table: no per-width symbol, no runtime table, no
> relocation. A width outside the range is refused where it is written, naming the width and the range.

That paragraph is compatible with `arvo-toolbox-not-policer.md`'s "No bit-width cap below the largest
container the substrate is willing to dispatch through", because the cap it states is not below anything:
it is the dispatch's own extent, and the sentence says so rather than dressing a guess as a fact about
the layer.

## 8. What is left open, and it is one number

Not whether there is a ceiling, which is settled by the mechanism, and not whether the sum takes a second
one, which section 6 settles against on compiled grounds. `W_MAX` itself, which is op's, and which the cost
table prices at every plausible value.

One item is created rather than closed, and it is small: under A the design should say, once, that the
tower's addition is total and correct over the whole reachable range, because A leans on it where the table
does not reach. That is one law over 21 impls, not a per-row obligation, and it belongs with the tower's
own arithmetic rather than with the bridge.

## Verification

Every figure and every diagnostic above was produced this pass, under the pin, from a scratch tree outside
the repository. The tower is 21 addition impls plus the sealed carrier, reproduced from `110:3312-3382`'s
own declarations; the table is generated at each row count; the negative control fires
(`error[E0080]: evaluation panicked: assertion failed: <PrecisionOf<13, 3> as Nat>::VAL == 17`). Timing is
`/usr/bin/time -p`, three runs each after a discarded first, and peak RSS is `/usr/bin/time -l`. The
consumer figures come from a separately compiled crate linking the table as an rlib. Nothing in this pass
touched `mock/crates` or any file under `mock/design_rounds/`.
