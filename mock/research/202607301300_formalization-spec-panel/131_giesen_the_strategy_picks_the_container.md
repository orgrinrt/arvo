# 131. The strategy picks the container, and the precision comes back as a Nat

**Persona:** Fabian Giesen, cost-model and systems lens. Fourth pass in this panel; file 34 assembled the
three halves, file 48 the stretch, file 72 the unexamined ground, file 86 the levels, file 106 asked whether
it was one pattern or two.
**Date:** 2026-08-07
**Position:** second read on `130_kiselyov_the_surface_without_compromise.md`, after op's three rulings.
Reads `126`, `127b`, `128`, `129`, `130`, `110` where it bears on the numeral and the container, and the
shipped `arvo-strategy/src/container.rs`.
**Pin:** `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `--edition 2024`, scratch tree outside the
repository. `mock/crates` read but untouched, `mock/design_rounds/` untouched. Probes at
`/private/tmp/claude-501/-Users-orgrinrt-Dev-clause-dev/dea47dac-5762-46b6-956e-0d22cc5d3832/scratchpad/rygc/`,
named `a1` through `cv1` roughly in the order they appear here.

Op ruled that the container parameter should not exist, and the reason he gave is not ergonomic. It is that
arvo's whole proposition is that the consumer states a policy and the design picks the storage. That ruling
is already in the standing document, in a line nobody in this panel has quoted: `110:3251` declares
`Lowering::Container` as "the container level W_C: derived, never declared as an axis". `130` put it back as
an axis. So this is not op overruling the panel on taste; it is op catching a regression against a line the
consolidation already carries.

The surface he asked for compiles. `UFixed<13, 3, Warm>`, three written parameters, container derived from
strategy, widths and sign, laws checked, the whole ladder including the wide rung, at `d2_laws.rs`, exit 0,
0.04 s. It costs `min_generic_const_args` plus `generic_const_args` plus `-Znext-solver=globally`, and I am
going to be blunt about that price rather than bury it, because the panel has twice now reported a
gate-free answer that was gate-free only because it had quietly moved the work onto the consumer.

Three findings the second read owes, in descending order of how much they should change what happens next.

**The projection is where `generic_const_args` earns its keep, and the panel dismissed it for the wrong
reason.** `128:147-159` concluded "container selection does not need the feature", and that conclusion is
true only under the assumption op has now removed, that the consumer names the container. `129` then refuted
GCA on canonicity, correctly. The two got merged into "GCA is not needed anywhere", which is false. Under
op's ruling the container projection needs it, and it needs it in a place where `129`'s refutation does not
reach, because there is exactly one syntactic route to the projected type by construction.

**Op's reservation about `Precision` resolves in his favour and better than he expected.** He asked whether
something downstream might need `Precision` as a type-level `Nat`. It might, and the mechanism the container
projection already pays for produces one, canonical, derived from the separated coordinates with no computed
const argument at any use site (`p1_nat.rs`, exit 0). More than that, it makes "these two numerals have the
same precision" a **bound checked at the signature**, which `130:418` records as not statable as a bound.
So the withdrawn canonicity comes back exactly where it was wanted, without making two different formats one
type.

**`130`'s `PRECISION` is wrong, and the same error is load-bearing in `129`.** `130:240` reads
`const PRECISION: u32 = G::EXTRA + I + F`, folding the sign bit into a mathematical coordinate. D69, which
that file cites two lines earlier, says the opposite: precision is mathematical, total width is derived on
the physical side (`110:869-873`). The consequence is not cosmetic. Under `129`'s precision-keyed numeral,
where the signed surface macro folds the sign bit in the same way (`129:521`), the multiplicative law
`R == P + Q` (`129:127`) **refuses the correct width of a signed product**, compiled at `s1_signed_law.rs`.

---

## 0. Gates, and the brief's claims checked before reasoning from them

**Canon gate: passed.** No ratified canon exists for arvo yet; this panel is producing the first one, so
`panels-argue-the-intent-not-the-wording.md` puts the intent and op's own calls in the governing position.
The governing calls here are op's three rulings quoted in the brief, D48's surface (`127b:56-59`), D69's
overturn (`110:869-873`), and the convergence pressure (`127b:12-18`). Section 7 reports the one place I
contradict a ratified line and hands the call back rather than making it.

**Test gate.** Not run, and I am naming it rather than letting it pass. `126:39-44` ran
`cargo test --offline --workspace` and got 155 binaries, 672 passed, 0 failed, 9 ignored, on a tree nothing
has moved since; `129:47-52` and `130:51-56` both declined to re-run it on op's ruling at `108b:174-181`
that a further report of the same collected tautologies is what that ruling exists to stop. My deliverable
touches no crate in that tree. The instrument here was the compiler, thirty-one times.

**The toolchain.** `rustc +nightly-2026-05-28 --version --verbose` reports
`1.98.0-nightly (57d06900f 2026-05-27)`, matching the brief and confirming `128:364-366`'s report that
`unstable-features.md:95` records a stale hash.

### The brief's factual claims

*"File `128` reports the carry-and-read discipline gives the full hardware ladder with zero gates and no
flag; verify and build the surface on it."* **The report holds and the instruction does not follow from it.**
`128:147-159`'s ladder is a **fit check**, not a projection. Its own next sentence says so: "the consumer
names the container and the compiler checks it, rather than the compiler deriving it" (`128:171-173`).
Under op's ruling the consumer does not name the container, so what `128` proved gate-free is not the thing
the brief asks me to build on. I checked whether the projection is available under the same discipline, and
it is not (`a1_gatefree_project.rs`):

```
error: generic parameters may not be used in const operations
23 |     type T = <Picker as Project<{ tag(N) }, G>>::T;
   |                                       ^ cannot perform const operation using `N`
   = help: const parameters may only be used as standalone arguments here, i.e. `N`
```

This is the same universally quantified refusal `130:454-465` catalogued over seven positions and `129`
over eight. It applies here with more force, not less, because selecting a container from a width is
irreducibly a case split on a value, and a case split on a const value is one impl per value.

*"`generic_const_args` is vetted WATCH and is a cost to justify rather than a free choice."* **Holds**,
`128:17-19`. Section 3 justifies it or fails to; the numbers are there either way.

*"The exposure is confined to the declaring crate: a downstream consumer with no gate and no flag compiles
and still receives the refusal."* This is `128:250-262`, quoted at me as settled. **It is false for the
container projection, and it is equally false for the shipped GCE code it was measuring against.** Section
3.4 has the numbers. It is the single most decision-relevant correction in this file, because it is the
difference between one crate carrying an unstable mechanism and the whole downstream stack carrying it.

*"`min_generic_const_args`, `adt_const_params`, `min_specialization` and the const-traits family are
allowed."* Holds per `unstable-features.md`. I use mGCA and GCA and nothing else.

---

## 1. The premises this brief takes for granted

Four converged conclusions in this panel have fallen to someone asking why an assumption was there, so
before answering I went looking for the assumptions in my own brief. Three are load-bearing.

**That the numeral is a format rather than a width**, which `130:774` names as the premise it wants
attacked. I attacked it and it survives, for a reason `130` did not give and could not have given from
inside its own framing: the width is not even well defined without the format. A signed Q12.3 occupies
sixteen bits and carries fifteen significand digits, and which of those two numbers is "the width" is the
question the whole section 7 of this file is about. `129` chose the sixteen, called it precision, and got a
false multiplicative law out of it. A numeral keyed on a single number has to pick one of those meanings and
is wrong about the other. Two coordinates plus a sign marker is not a richer encoding of the same thing; it
is the smallest set from which both numbers are derivable.

**That the container ladder has rungs a width can fall off.** The brief asks me to "say what the ladder's
rungs are and what happens at a width no rung holds", which presupposes such a width exists. In the shipped
code it does not, and the shipped code's own doc comment says it does. `container.rs:107-109` states that
"Absence of a `Project` impl for a given `(TAG, Sign, S)` triple is how `Uint<100, Warm>` (N=100, no native
u256) becomes a compile error". Read the tag function and the impl set in the same file:
`tag_warm_precise(100)` returns 5 (`container.rs:78-91`), and `impl<Sign, const BYTES: usize> Project<5,
Sign, BYTES, Warm> for Picker` exists at `container.rs:238-240`. So `Uint<100, Warm>` resolves to
`WideBits<13, A1>` and compiles. The worked example in the comment is wrong, the
`#[diagnostic::on_unimplemented]` note at `container.rs:110-113` describes a case that cannot fire, and the
projection is total. I did not build arvo to confirm this, because `mock/crates` is out of bounds and the
reading is a two-function logic check against the primary source; mark it a source reading rather than a
compiled result.

**That "the container parameter should not exist" is a statement about the surface.** It is not, or not
only. `110:3251` puts `Container` on `Lowering` as a derived member, and op's own sentence ties it to
hilavitkutin-build reading the same typestate. That means the projection is not a private implementation
detail arvo may spell however it likes; it is a declared member of a ratified trait, and a downstream
optimisation layer is specified to read it. Anything that makes the container a written argument does not
merely inconvenience a consumer, it deletes the thing the build layer was going to consume. Section 6 says
what that layer gets.

---

## 2. The verdict on `130`

Split, because the file is two claims and they do not stand or fall together.

**The core move is right and I reached it independently.** Restoring the second coordinate so the alias
computes nothing is correct, and the argument that settles it is not the alias at all. It is that a numeral
keyed on one number cannot express a rescale, cannot check alignment, and cannot decode. `130:170-193`
compiles that at exit 0 with one decoding as thirty-two. I did not re-run that probe; I ran the sharper
version of the same defect, which is that the one-number keying makes a **law** false rather than merely
making a decode ambiguous. Section 7.

Concretely, on my own build: alignment is a shared parameter and is refused by unification at the consumer's
own line before any const evaluation runs (`n1.rs`):

```
error[E0308]: mismatched types
 4 |     let _s: UFixed<14, 3, Warm> = add(a, b);
   |                                   ---    ^ expected `3`, found `8`
   = note: expected struct `Fixed<_, 3, _, _>`
              found struct `Fixed<8, 8, _, _>`
```

That guarantee exists only because the exponent is in the type, and it is the cheapest kind of check the
language has. `130:412-426` is right about that table and I am not disturbing it, except for row five,
which section 4 overturns in `130`'s favour.

**The container parameter is wrong**, and op has ruled, so I am not relitigating it. What I will add is
that the file's justification for it does not hold up. `130:466-479` argues the carry is "not a workaround,
it is `arvo-toolbox-not-policer.md`'s posture arriving as the only available mechanism". That reads the
toolbox rule backwards. The rule's own text says the substrate exposes **choices the consumer knows the
answer to**, and lists them: workload shape, access pattern, perf budget, semantic intent. Which machine
integer holds a 16-bit Q13.3 is not on that list and cannot be, because the consumer's answer to it is a
function of the strategy they already wrote. `no-bare-primitives.md` puts it in as many words: "Consumers do
not know and should not care which u-primitive the transparent repr lowers to." A parameter whose value is
determined by another parameter is not a choice being exposed, it is a derivation being outsourced.

**`Format::PRECISION` is wrong.** `130:240`:

```rust
const PRECISION: u32 = G::EXTRA + I + F;   // door two: an associated const body
```

Two lines above, the same file cites D69 as "precision and the exponent bounds are primitive; total width,
the hidden bit, and field encoding are derived on the physical side" (`130:141-143`, quoting `110:869-873`).
`G::EXTRA` is the sign bit. The sign bit is field encoding. Putting it into `PRECISION` puts a physical
coordinate into the mathematical set, in the same impl block that cites the overturn separating them.

The fix is one line and it changes what the coordinates say. On my build (`d1_capstone.rs`):

```rust
const PRECISION: u32 = I + F;            // significand digits, sign-free
const STORED_WIDTH: u32 = G::EXTRA + I + F;  // derived on the physical side
```

and the assertions that pin it, compiled:

```rust
const _: () = assert!(<IFixed<12, 3, Warm> as Format>::PRECISION == 15);
const _: () = assert!(<IFixed<12, 3, Warm> as Format>::STORED_WIDTH == 16);
```

**`130:568-570` is false as stated**, and I checked it because it is the premise of that file's whole
diagnostic design. It says a consumer sees "an error every span of which is in someone else's crate, and
**no span of their own**". For a direct call from a consumer crate, the consumer's own line is named, with
their own source quoted (`n2.rs`, tail):

```
note: the above error was encountered while instantiating `fn arvocore::mul::<1000, 1000, 1200, 1300, 2201, 2300, Unsigned, Precise>`
 --> n2.rs:4:43
  |
4 |     let _p: UFixed<2201, 2300, Precise> = mul(a, b);
  |                                           ^^^^^^^^^
```

`130` measured the case where a **library wrapper** sits between the consumer and the law, and generalised
it to all consumers. The generalisation is what its long message was designed for, and section 6 shortens
the message accordingly.

---

## 3. The container projection

### 3.1 What a consumer writes

From `d2_laws.rs`, verbatim and compiling, exit 0:

```rust
pub fn consumer(
    _a: UFixed<13, 3, Warm>, _b: UFixed<8, 8, Warm>, _c: UFixed<40, 30, Precise>,
    _d: UFixed<3, 0, Hot>, _e: UFixed<0, 8, Cold>, _f: IFixed<12, 3, Warm>,
    _g: UFixed<200, 100, Hot>,
) {}

pub fn arithmetic(a: UFixed<13, 3, Warm>, b: UFixed<13, 3, Warm>) {
    let _p: UFixed<26, 6, Warm> = mul(a, b);
    let _s: UFixed<14, 3, Warm> = add(a, b);
    let _r: UFixed<8, 8, Warm>  = rescale(a);
    let _w: UFixed<20, 3, Warm> = widen(a);
}
```

Three written parameters, which is D48's literal arity (`127b:56-59`), one fewer than either spelling
`130:728-739` offered. Plain type syntax, no macro, no exclamation mark, no table, no cap, arbitrary widths.
The container appears nowhere.

The declaration, which is the whole of the change against `130`:

```rust
pub struct Fixed<const I: u32, const F: u32, G: Sign, S: Store<I, F, G>> {
    raw: <S as Store<I, F, G>>::T,
    _m: PhantomData<G>,
}
pub type UFixed<const I: u32, const F: u32, S> = Fixed<I, F, Unsigned, S>;
pub type IFixed<const I: u32, const F: u32, S> = Fixed<I, F, Signed, S>;
```

Both aliases are generic and legal, because every argument is standalone. That is `130`'s door one, kept
intact; the container simply moved from an argument to a projection.

### 3.2 The ladder, and what happens at the top

Unchanged from the shipped design (`container.rs:10-29`), which I am restating because the panel has been
citing a five-rung ladder and it is six, and because the strategies are asymmetric over it.

| Rung | Hot and Cold hold | Warm and Precise hold |
|---|---|---|
| 0 | `u8` / `i8`, physical width 1 to 8 | `u16` / `i16`, physical width 1 to 8 |
| 1 | `u16` / `i16`, 9 to 16 | `u32` / `i32`, 9 to 16 |
| 2 | `u32` / `i32`, 17 to 32 | `u64` / `i64`, 17 to 32 |
| 3 | `u64` / `i64`, 33 to 64 | `u128` / `i128`, 33 to 64 |
| 4 | `u128` / `i128`, 65 to 128 | unreachable |
| 5 | `Wide<BYTES, A16>`, above 128 | `Wide<BYTES, A1>`, above 64 |

Hot and Cold take the minimum aligned native; Warm and Precise take one rung of headroom, which is what
carries single-operation overflow room for Warm's wrapping and Precise's saturating semantics. Warm and
Precise therefore have no rung 4 and fall to wide at 65 rather than 129. Hot uses align-16 at the wide rung
for the SSE2 and NEON baseline; the other three use align-1.

**What happens at a width no rung holds: nothing, because there is no such width.** The wide rung is a
catch-all parameterised by a byte count, so the ladder is total and every width above it is served. This is
the answer op's no-ceiling instinct at `127b:118-126` asks for, and it arrives without a design decision,
because the ceiling was never there. It is also why the shipped `#[diagnostic::on_unimplemented]` at
`container.rs:110-113` cannot fire, per section 1.

Compiled, every rung, both signs, all four strategies (`d1_capstone.rs`, `ladder`):

```rust
let _: <Warm as Store<13, 3, Unsigned>>::T = 0u32;   // 16 physical bits plus headroom
let _: <Hot  as Store<13, 3, Unsigned>>::T = 0u16;   // 16 physical bits, min aligned
let _: <Hot  as Store<3, 0, Unsigned>>::T  = 0u8;
let _: <Cold as Store<0, 8, Unsigned>>::T  = 0u8;
let _: <Hot  as Store<12, 3, Signed>>::T   = 0i16;   // 1 + 12 + 3 = 16
let _: <Warm as Store<12, 3, Signed>>::T   = 0i32;
let _: <Warm as Store<40, 30, Unsigned>>::T = Wide::<9,  A1>(..);
let _: <Hot  as Store<200, 100, Unsigned>>::T = Wide::<38, A16>(..);
```

Those are exit-0 type equalities, not assertions about them. If the projection landed on the wrong rung the
file would not build.

### 3.3 The mechanism, and why it is the shipped one

The projection is `container.rs`'s Pattern C, unchanged in shape, with the forbidden feature swapped for
the vetted one. The tag and the byte count are factored into `type const` items, which is the position
rustc itself names when you take its repairs in order:

```
error: generic parameters in const blocks are only allowed as the direct value of a `type const`
   = help: consider factoring the expression into a `type const` item and use it as the const argument instead
```

(`a4_gca_project.rs`.) Taking it:

```rust
pub struct Rung<const I: u32, const F: u32, G: Sign, S>(PhantomData<(G, S)>);
pub trait Tagged { type const TAG: usize; type const BYTES: usize; }

impl<const I: u32, const F: u32, G: Sign> Tagged for Rung<I, F, G, Warm> {
    type const TAG:   usize = const { tag_headroom(G::EXTRA + I + F) };
    type const BYTES: usize = const { bytes_for(G::EXTRA + I + F) };
}

impl<const I: u32, const F: u32, G: Sign> Store<I, F, G> for Warm
where Picker: Project<{ <Rung<I,F,G,Warm> as Tagged>::TAG }, G,
                      { <Rung<I,F,G,Warm> as Tagged>::BYTES }, Warm>,
{
    type T = <Picker as Project<{ <Rung<I,F,G,Warm> as Tagged>::TAG }, G,
                                { <Rung<I,F,G,Warm> as Tagged>::BYTES }, Warm>>::T;
}
```

Four `Tagged` impls, four `Store` impls, twenty-four `Project` impls (five or four native rungs times two
signs times four strategies, plus four wide). No enumeration over widths anywhere, which is
`127b:36-50`'s requirement. Widths are unbounded, capped only by the const parameter's own type, which
`123:44-51` located as not a design decision.

**`129`'s refutation of GCA does not reach this.** That file's finding is that under a generic parameter,
`(A + B) + C` and `A + (B + C)` are distinct types, so two construction routes to one precision do not
unify (`129:154-168`). Here there is exactly one construction route by definition: the impl body is the only
place the expression is written, and every use is a projection through it. The definitional-equality defect
needs two syntactic routes to bite, and the projection admits one. That is not luck, it is a property worth
stating in the canon: **a `type const` whose body appears in exactly one impl is canonical for free.**

**The GAT spelling ICEs, and this is a rough edge `128`'s vetting did not name.** The natural shape is a
generic associated type on `Lowering`, which is where `110:3251` already puts the container and which would
carry no where-clause and therefore no propagation tax. It crashes the compiler. Minimal reproduction,
eighteen lines, `c1_ice_min.rs`:

```rust
pub trait Lowering { type Store<const N: u32>: Copy; }
impl Lowering for Hot {
    type Store<const N: u32> = <Picker as Project<{ <Rung<N> as Tagged>::TAG }>>::T;
}
```

```
thread 'rustc' panicked at compiler/rustc_type_ir/src/binder.rs:1317:13:
cannot find `!BoundConst { var: 1, .. }` in param-env: ParamEnv { ... }
```

`c2_nongat.rs` is the same body on a standalone trait and is exit 0. So the ICE is specific to the GAT, and
the design has to take the standalone trait, which costs the `S: Store<I, F, G>` bound on the numeral and
on every width-generic signature. In `mul` that is three bounds:

```rust
where S: Store<I, F, G> + Store<J, K, G> + Store<M, N, G>,
```

That tax lands on arvo's own generic code and on any consumer writing width-generic code. It does not land
on concrete consumer code. Whether it lifts when the ICE is fixed upstream is unknown and worth a note in
whatever row GCA gets, because the GAT form is strictly better and costs nothing but a compiler fix.

### 3.4 What it actually costs, which is more than `128` priced

This is the number that should decide the fork, and the panel has been carrying a wrong one.

`128:250-262` reports that exposure is confined to the declaring crate and that a downstream consumer with
no gate and no flag compiles and still receives full checking. I built the library and three consumers
against it (`arvocore.rs`, `cons_type.rs`, `cons_ok.rs`, `cons_facade.rs`):

| What the consumer does | Feature gate | `-Znext-solver=globally` | Result |
|---|---|---|---|
| Names arvo types in a signature | no | no | exit 0 |
| Calls a fully concrete arvo function | no | no | exit 0 |
| Calls a width-generic arvo law | no | **yes** | exit 0 |
| Calls a width-generic arvo law | no | no | `E0277`, unsatisfied `Project` bound |
| Calls a width-generic arvo law | yes | no | `error: generic_const_args requires -Znext-solver=globally` |

So the feature gate confines and **the flag does not**. Any consumer that calls `mul` or `add`, which is
any consumer doing arithmetic, compiles with `-Znext-solver=globally`. That is a whole-crate trait solver
replacement reaching hilavitkutin, vehje, and everything downstream, which is categorically larger than
`128:298-302` priced when it called the flag "a larger exposure than any feature gate in the tables" and
then confined it to one crate.

**Two things make that less bad than it sounds, and both are measured.**

First, it is not a regression. I built the same projection under the forbidden `generic_const_exprs`, which
is the status quo, and the bare consumer fails identically (`gce_core.rs` then `cons_gce.rs`):

```
error[E0277]: the trait bound `Picker: Project<gcecore::::{impl#4}::{constant#0}, Unsigned>` is not satisfied
 4 |     let _p: UFixed<26, 6, Warm> = mul(a, b);
```

And the tree confirms that consumers already pay it: `generic_const_exprs` is live at
`arvo/mock/crates/arvo/src/lib.rs:25`, `arvo/mock/crates/arvo-strategy/src/lib.rs:11`, and
`hilavitkutin/mock/crates/hilavitkutin/src/lib.rs:24`. The consumer carries the forbidden gate **today**.
The migration therefore trades a forbidden gate in every consumer for a watched flag in every consumer, and
that is strictly an improvement under `unstable-features.md`'s own ordering, quite apart from removing the
last live GCE gates, which the rule has mandated as drift remediation since 2026-07-28.

Second, the confinement mechanism works, it is just narrow. A concrete facade confines completely
(`cons_facade.rs`, exit 0, no gate, no flag). That is not a practical escape for a library whose subject is
arbitrary widths, but it does mean a consumer with a fixed set of formats can wrap once and pay nothing.

### 3.5 Compile cost

`d2_laws.rs`, the whole surface with the ladder, the projection, four laws and the consumer section:
`/usr/bin/time -p`, cold then two: **0.10 s, 0.04 s, 0.04 s.** Level with the fastest thing this panel has
measured, which is `126:229-232`'s 0.04 s, and against `125:245-250`'s 0.06 s through a 4096-row table and
5.87 s through use-site realisation, while doing more than either.

---

## 4. The `Nat` and `Precision` spellout

Op's reservation, verbatim: the proposal moved `Precision` from a type to a const read, and somebody has to
establish that nothing downstream will need it as a type-level `Nat`, "because if something does, the whole
question reopens". He then answered his own worry provisionally, that `Precision` has all it needs to derive
itself from the separated components. He is right, and it is worth showing rather than asserting, because
the derivation is not free and the thing that makes it work is the mechanism section 3 already bought.

**The enumeration.** Every position I can find where a precision or a width could want to be a type rather
than a const read, and what each actually needs.

| Position | Needs a `Nat`? | Evidence |
|---|---|---|
| The numeral's own storage (the container) | no, needs the const in a `type const` | `d1_capstone.rs`, section 3 |
| A law relating output coordinates to inputs | no, output parameters plus a check | `d2_laws.rs` |
| Alignment ("same exponent") as a bound | no, a shared const parameter | `n1.rs`, `E0308` |
| Format identity as a bound | no, shared parameters | trivially, `Fixed<I, F, ..>` twice |
| Container identity as a bound | no, associated-type equality | `Store<I, F, G, T = u16>` |
| **Precision agreement as a bound** | **yes, or an associated-const equality** | `p1_nat.rs`, section 4.1 |
| A type keyed on precision (a column, a bitfield slot, an arena row) | **yes** | `p1_nat.rs`, section 4.2 |
| `Crosses<N: Numeral>`, which takes the numeral as a type | no, the numeral is already a type | `110:3262` |
| Radix and exponent form | no, both are already types | `110:913-916`, `130:699-703` |
| Type-level arithmetic over precisions (the tower) | removed | `126:294-335` |

Two rows want a `Nat`. Both get one.

### 4.1 Precision agreement is a bound, which overturns `130:418`

`130`'s table records "Two numerals have the same precision | not statable as a bound | const eval". That
is true gate-free and false under the mechanism section 3 already requires. `p1_nat.rs`, exit 0:

```rust
pub trait Numeral { type const PRECISION: u32; }
impl<const I: u32, const F: u32, G: Sign, S> Numeral for Fixed<I, F, G, S> {
    type const PRECISION: u32 = const { I + F };
}

pub fn same_precision<A, B, const P: u32>(_: A, _: B)
where A: Numeral<PRECISION = { P }>, B: Numeral<PRECISION = { P }> {}

pub fn agree(a: UFixed<13, 3, Warm>, b: UFixed<8, 8, Warm>) { same_precision(a, b); }
```

`agree` compiles. Q13.3 and Q8.8 are different types and their precisions unify at the signature, checked
before any monomorphisation, with the const parameter `P` **inferred** from the bound rather than supplied.
That is the associated-const-equality mechanism `129:246-263` found and correctly reported as a near miss
gate-free; under GCA it is a hit.

It refuses a real mismatch, and the diagnostic is the value comparison rather than a structural one:

```
error[E0271]: type mismatch resolving `17 == 16`
  --> p1_nat.rs:41:84
```

### 4.2 The `Nat` itself, derived without a computed const argument

```rust
pub struct Nat<const P: u32>;
pub type PrecisionOf<X> = Nat<{ <X as Numeral>::PRECISION }>;
```

Every use site passes a standalone projection. The addition lives in the `type const` body, which is the one
place GCA admits it, and the alias itself computes nothing. Compiled (`p1_nat.rs`, exit 0):

```rust
pub fn wants16(_: Nat<16>) {}
pub fn canonical(a: PrecisionOf<UFixed<13, 3, Warm>>, b: PrecisionOf<UFixed<8, 8, Warm>>,
                 c: PrecisionOf<IFixed<12, 3, Warm>>) {
    wants16(a);           // 13 + 3
    wants16(b);           // 8 + 8, same type
    let _: Nat<15> = c;   // signed: precision is sign-free, 12 + 3
}
```

And it composes into a nested projection, which is the column and bitfield case:

```rust
pub struct Column<N, const LEN: usize>(PhantomData<N>);
pub type ColumnOf<X, const LEN: usize> = Column<PrecisionOf<X>, LEN>;
pub fn column(_: ColumnOf<UFixed<13, 3, Warm>, 4096>) {}
pub fn column_agrees(x: ColumnOf<UFixed<8, 8, Warm>, 4096>) { column(x); }   // one type
```

So a column of Q13.3 and a column of Q8.8 are the same column type, which is what a bitpacked store wants,
while the numerals themselves stay distinct, which is what the arithmetic wants. Both, at once, which is
the thing the panel has spent two files treating as a fork.

### 4.3 The limit, stated exactly

Precision agreement holds at concrete sites and under a generic parameter **when the coordinates unify**.
It fails when they are permuted (`p2_generic.rs`):

```rust
pub fn generic_agree<const I: u32, const F: u32, G: Sign, S>(
    a: Fixed<I, F, G, S>, b: Fixed<I, F, G, S>) { same_precision(a, b); }        // exit 0

pub fn generic_cross<const I: u32, const F: u32, G: Sign, S>(
    a: Fixed<I, F, G, S>, b: Fixed<F, I, G, S>) { same_precision(a, b); }        // refused
```

```
error[E0271]: type mismatch resolving `const { I + F } == const { I + F }`
```

That is `128:210-223` and `129:154-168` arriving in this construction, and it lands squarely in the region
op has just declared uninteresting: `Fixed<I, F>` and `Fixed<F, I>` are exactly the flipped pair he said
should not interexchange anywhere. So the limit is real and it costs nothing anybody wants.

**The diagnostic, however, is indefensible and somebody should say so upstream.** `const { I + F } == const
{ I + F }` prints two identical strings and calls them different, because the display shows the impl's body
rather than the substitution. A consumer meeting that will file a compiler bug. It is a rustc issue rather
than a design one, and it belongs in whatever row GCA gets as a named rough edge alongside the GAT ICE.

### 4.4 The answer to the reservation

Nothing is foreclosed. `Precision` as a const read and `Precision` as a type-level `Nat` are the same
declaration viewed twice: `type const PRECISION: u32` is readable in value position, projectable into type
position, and bindable in a where clause. The design does not have to choose, and it does not have to
decide now which downstream position will want which.

The reservation was well placed all the same, because the answer is only available **because of the
container projection**. Had the panel landed on `130`'s written container, the crate would carry no GCA, and
then a downstream position wanting a canonical `Nat` really would reopen the question. The two answers are
one purchase. That is worth recording as the reason, not as a coincidence.

---

## 5. What replaces canonicity

Op withdrew "two numerals of equal precision are the same type" and said what replaces it is open, adding
that if two such numerals should ever relate, the relation is a conversion rather than type identity, and
that the design owes a statement of which conversions are implicit and which are written.

**The withdrawal is right and the compiled reason is sharper than the one in the record.** `130:170-193`
shows the one-number keying making a decode ambiguous. The sharper version is that it makes a **law false**.
Section 7 has it.

**What the ratification was protecting survives, in the one place it was wanted.** The failure op named at
`127b:24-26` is that "the compiler reports `E0308` where a consumer expects agreement". Section 4.1 is that
agreement, checked at a signature, between Q13.3 and Q8.8, with no `E0308`. Canonicity moved from the
numeral to the precision projection, which is where it was always about storage rather than about
arithmetic. So the honest restatement of the ratified sentence is:

> Two numerals of equal precision have the same `Precision`, and `Precision` is a type. They are not the
> same numeral, because they are not the same number.

### The conversion story

Five relations, and only five. Rust has no user-extensible implicit coercion, so "implicit" here means one
of two concrete things: the type is produced directly by the operation and inferred from the annotation, or
it is reached by `From` and written as `.into()`. Everything else is a named call.

**One. Identity.** Same `(I, F, G, S)` is the same type. Nothing to convert. Structural.

**Two. Inferred at the operation, which is what op's "implicitly castable via the typestate and rust
autotyping" actually is.** The laws take their output coordinates as parameters, so the consumer writes an
annotation and never writes a conversion:

```rust
let _p: UFixed<26, 6, Warm> = mul(a, b);
```

There is no cast here and there is no `From` here. The output type is chosen by the annotation and the law
checks that the choice follows from the inputs. This is the mechanism that delivers what op wants and it is
worth naming, because "implicit conversion" is the wrong picture of it: there is nothing to convert, the
operation produced that type.

**Three. Written, total, lossless: `widen`.** Same exponent, more integer digits. The stored value is
unchanged and only the container may grow. This is the one relation that is a candidate for `From`, because
it is total and value-preserving in both directions of reasoning. My recommendation is to ship it as both:
`widen` for the explicit reading and a `From` impl for the `.into()` reading. It is the only conversion in
the design that may be implicit under any definition.

**Four. Written, and never implicit: `rescale`.** The exponent changes, which multiplies or divides by a
power of two and can drop digits off the bottom. Under the withdrawn canonicity this was an assignment,
invisible and unwritten. Under the two-coordinate numeral it has a name and the consumer writes it:

```rust
let _r: UFixed<8, 8, Warm> = rescale(a);
```

The equal-precision-different-scale pair op was reasoning about is exactly this relation. It is a real
arithmetic operation, and type identity would have made it invisible. That is the case for the withdrawal
in one sentence.

**Five. Refused.** Mixing two numerals that do not relate is an `E0308` at the consumer's own line naming
both coordinates (`cv1_conversion.rs`):

```
error[E0308]: mismatched types
15 |     let _bad: UFixed<8, 8, Warm> = a;
   |               ------------------   ^ expected `8`, found `13`
   = note: expected struct `Fixed<8, 8, _, _>`
              found struct `Fixed<13, 3, _, _>`
```

Narrowing the integer part is deliberately absent from the list. It is lossy in a way the strategy has to
adjudicate (wrap for Hot, saturate for Precise), so it is not a conversion at all; it is an operation
carrying a policy, and it belongs with the arithmetic rather than with the conversions. I have not designed
it and it is on the open list.

---

## 6. The two diagnostics, priced and compiled

Op adopted both and asked whether "almost free" survives contact. It mostly does, with one correction to
each.

### 6.1 Named-item laws

The finding is `130:578-589`'s and I reproduced it rather than citing it: an anonymous `const {}` block
reports as `mul::<..>::{constant#0}`, mixing the law's coordinates with the strategy and giving the block a
number, while a named item reports as the law. From `n2.rs`:

```
evaluation of `arvocore::ProductFormat::<1000, 1000, 1200, 1300, 2201, 2300>::HOLDS` failed here
```

Six numbers in the law's own order and nothing else.

**The price, measured** (`gen_laws.py`, `lawsweep_*.rs`), sweeping the law count with 64 checked
compositions per law at four-digit widths:

| Laws | Items | Checked uses | `real` |
|---|---|---|---|
| 1 | 2 | 64 | 0.08 s |
| 8 | 16 | 512 | 0.06 s |
| 32 | 64 | 2048 | 0.16 s |
| 64 | 128 | 4096 | 0.31 s |

Two items per law, one struct and one impl, and the curve is linear in checked uses with a very small
constant. At sixty-four laws and four thousand checked compositions it is a third of a second. "Almost
free" survives, comfortably.

**The one correction: the real law count is not in the record.** The brief asks me to measure at it.
`110:1420` opens the algebra section and defines what a law is, in full, and the document nowhere enumerates
them. So the number is a gap rather than a measurement I declined to take, and it should go on the open list
next to the section that defines laws without listing them.

**The message should be shorter than `130:601-610`.** That message spends six lines telling the consumer
which spans to trust and how to find their own call site, and section 2 established that a direct caller is
already given their own line and their own source. The long form is right for the wrapper case and noise for
the common one. The shape I would ship states the law, names the printed order, and gives the search handle
in one clause, keeping the wrapper case to a single sentence:

```
error[E0080]: evaluation panicked: arvo: the product's format does not follow from its inputs.
   The law: Fixed<I, F> times Fixed<J, K> has format Fixed<I + J, F + K>.
   The line above prints ProductFormat::<I, F, J, K, M, N> with the actual digit
   counts, in that order. Name the output with the first four added pairwise. If the
   note below names a function you did not write, that function states a format
   relation that does not hold; search your own source for the last two numbers.
```

Two mechanical constraints, both compiled. `assert!`'s second argument is a format string, so a message
containing a brace fails to build (`130:594-596`, which I did not re-run). And const evaluation supports no
formatting, so no value can be interpolated; the numbers come from rustc's own instantiation line, which is
the entire reason naming the law matters.

### 6.2 The witness set

The mechanism is `130:620-659`'s and it is sound: a generic wrapper cannot compute its output coordinates,
so the space of expressible wrong claims is `{one of the wrapper's own parameters, a literal}`, and a small
fixed set of instantiations refutes every member.

I confirmed the hole exists on my build (`w1_hole.rs`, exit 0, two wrong wrappers, uninstantiated,
accepted), and then did the selection `130:661-665` explicitly left undone. Two corrections came out of it.

**Correction one: a witness can be a false negative, and `130`'s worked example is one.** A wrapper claiming
a constant output format is only refuted at a point where the constant is wrong. My first witness pair had
`square_lit` at Q13.3, whose true product Q26.6 happens to equal the claimed constant, so it passed. The
second, at Q7.2, fired:

```
evaluation of `arvocore::ProductFormat::<7, 2, 7, 2, 26, 6>::HOLDS` failed here
note: the above error was encountered while instantiating `fn arvocore::mul::<7, 2, 7, 2, 26, 6, Unsigned, Warm>`
  --> w2_witness.rs:12:3
```

So witnesses are not arbitrary and "add a couple" is not a discipline. The selection rule, which is
mechanical once stated:

- Against a claim that an output coordinate equals one of the wrapper's input parameters, one witness at
  which that equality is false.
- Against a claim that it equals a literal, **two** witnesses whose true output coordinates differ, so that
  no single literal can satisfy both.

For `ProductFormat`, two witnesses whose true outputs differ in every coordinate discharge both, for example
Q13.3 giving Q26.6 and Q7.2 giving Q14.4. For `SumFormat`, whose relation is `max(I, J) + 1` and which
`130:663-665` flagged as needing "one witness on each side of the maximum", two are enough if they are
chosen with `I > J` in one, `J > I` in the other, and different maxima. I verified that pair against all
three expressible wrong wrappers (`wt_M_is_I.rs`, `wt_M_is_J.rs`, `wt_M_is_const.rs`), and all three are
refused.

**Correction two, and it makes the whole thing cheaper: the witness is refuted by `E0308`, not by the law.**
Because the witness declares its own true output type, the wrong wrapper fails unification at the witness
before any const evaluation runs:

```
error[E0308]: mismatched types
   = note: expected struct `arvocore::Fixed<10, _, _, _>`
              found struct `arvocore::Fixed<6, _, _, _>`
```

`expected 10, found 6` is a better diagnostic than the law's `E0080`, it is pre-monomorphisation, and it
means the witness set does not depend on the law items at all. The cost per law is **two lines, each
carrying hand-computed true output coordinates**, and the maintenance obligation is exactly that the
numbers are computed by hand rather than copied from the wrapper. A witness that copies the wrapper's claim
passes vacuously, which is the failure mode to write into the canon alongside the mechanism.

So: one declaration per law for the diagnostic, two lines per law for the witness, both discharged in the
crate that declares the law, both failing that crate's own `cargo check`. Belts and suspenders, and op's
"almost free" is right at roughly four lines per law.

---

## 7. The finding that decides the numeral, and it is about signed multiplication

This is the sharpest thing in the file and it is why `130`'s core move is right for a reason `130` did not
give.

Under `129`'s numeral the parameter is the precision, and the signed surface macro folds the sign bit into
it (`129:521`):

```rust
macro_rules! IFixed { ($i:literal, $f:literal, $c:ty, $s:ty) => { Fx<{ 1 + $i + $f }, $c, Signed, $s> }; }
```

So `IFixed!(12, 3, ..)` is `Fx<16>`, and the sixteen is a **stored width**, not a significand count. The
multiplicative law is `R == P + Q` (`129:127`).

Now take two signed Q12.3 values. Each carries fifteen significand digits and occupies sixteen bits. Their
product carries thirty significand digits, twenty-four integer and six fraction, and occupies **thirty-one**
bits, because a product has one sign, not two. A consumer who names the correct output is refused
(`s1_signed_law.rs`):

```
error[E0080]: evaluation panicked: mul: output precision must equal the sum of the input precisions
   evaluation of `mul::<16, 16, 31, u32, Signed, Warm>::{constant#0}` failed here
note: the above error was encountered while instantiating `fn mul::<16, 16, 31, u32, Signed, Warm>`
  --> s1_signed_law.rs:25:41
```

**The law is false for signed numerals, and the design's only recourse is to require the wrong answer.**
It is not a rounding of the truth in a safe direction either. At the ladder it costs a rung
(`s2_rung.rs`, all four assertions hold):

```rust
const TRUE_WIDTH: u32 = 1 + (31 + 32);   // 64, fits u64
const LAW_WIDTH:  u32 = 32 + 33;         // 65, next rung, u128
const _: () = assert!(rung(TRUE_WIDTH) == 64);
const _: () = assert!(rung(LAW_WIDTH) == 128);
```

A false law doubling a container is exactly the class of defect the strategy axis exists to prevent.

The cause is not the one-number keying by itself. It is the conflation `130` reproduces at `130:240`:
**a number that folds the sign bit is a width, and a number that does not is a precision, and a law written
over one of them is wrong if applied to the other.** Keep them apart and the product law is sign-free and
correct for both families at once, which is what my `ProductFormat` asserts and what `d2_laws.rs`'s
`signed` function exercises:

```rust
pub fn signed(a: IFixed<12, 3, Warm>) {
    let _p: IFixed<24, 6, Warm> = mul(a, a);   // 24 = 12 + 12, 6 = 3 + 3, sign unchanged
}
```

Two coordinates and a sign marker, three numbers, from which both the precision and the stored width are
derivable, and no law needs to know which family it is in. That is the argument for the format-keyed
numeral, and it is stronger than the decode argument because a wrong decode is a consumer's mistake while a
wrong law is the library's.

---

## 8. What is op's, separately from what I decided

**Mine, and compiled.** That the container projection from a generic width is unavailable gate-free, with
the same universally quantified refusal that closed seven positions in `130` and eight in `129`. That it is
available under `min_generic_const_args` plus `generic_const_args` plus `-Znext-solver=globally`, in the
shipped Pattern C shape, giving `UFixed<13, 3, Warm>` with three written parameters and the container never
named. That `129`'s definitional-equality defect does not reach the projection, because the projection has
exactly one syntactic route by construction. That the GAT spelling ICEs the compiler, in eighteen lines, so
the design must take the standalone trait and its bound-propagation tax. That the flag reaches every
consumer calling a width-generic law, which `128:250-262` reports as confined and is not, and that the same
is true of the forbidden GCE the tree carries today, so the migration is not a regression. That precision
must be sign-free, that `130:240` and `129`'s signed macro both fold the sign bit into it, and that the
resulting multiplicative law is false and costs a container rung. That a type-level `Nat` for precision is
derivable from the separated coordinates under the mechanism the projection already needs, that precision
agreement is therefore a bound rather than a const-eval check, and that `130:418` is overturned by that.
That `130:568-570`'s "no span of their own" is false for a direct consumer call. That a witness can be a
false negative, that two correctly chosen witnesses discharge each of the two law shapes, and that the
refutation arrives as `E0308` at the witness rather than as the law's `E0080`.

**His, and it is the one that blocks.** Whether the container projection is worth
`-Znext-solver=globally` reaching every consumer that does arithmetic. My reading, offered as a reading:
yes, because the alternative is not a cheaper projection but no projection, and because the status quo
already imposes a forbidden gate on the same consumers. But it is a trade with a real number on both sides
and the number is bigger than the panel has been told.

**His, because the rule has no answer.** Where `-Znext-solver=globally` sits in
`unstable-features.md`, raised at `128:287-306` and still unsettled. Section 3.4 changes the input to that
question, because the flag is now known to reach consumers rather than one crate, and it should be decided
with that on the table rather than without it.

**His, because it touches a ratified structure.** Whether `Precision` may be sign-free, which is my reading
of D69 (`110:869-873`) and contradicts `130:240` and `129`'s signed macro. Section 7 is the compiled case.
Op wrote the sentence D69 rests on and the reading is his to confirm.

**His, because it is his own withdrawn ratification.** Whether section 5's restatement is what he meant:
two numerals of equal precision have the same `Precision`, `Precision` is a type, and they are not the same
numeral. Section 4.1 compiles the agreement he was protecting.

**Owed under the two-expert rule.** I am the second read on `130`'s core move and I agree with it, from my
own compiled work, on a different argument. I am the **first** read on everything in sections 3, 4 and 6,
which propose a mechanism no second expert has seen and which adopt a feature the panel had concluded was
unnecessary. None of that should enter the canon on one expert's word. A second read should attack the
premise I have taken for granted: **that the container must be a Rust type chosen by rustc at all.** If
hilavitkutin-build can be given the format and the strategy and left to choose the storage, with rustc
seeing only an opaque carrier, then the whole GCA purchase is avoidable and my conclusion inverts. I did not
open that door because I do not know what hilavitkutin-build can do to a Rust type's layout, and I suspect
the answer is nothing, but suspecting is not checking.

---

## 9. What I did not check

- **Whether hilavitkutin-build can select storage after rustc has laid out the type.** Section 8's premise
  attack. It is the one route that would make the feature purchase unnecessary and I did not open it.
- **Whether the GAT ICE has an upstream issue, and whether the GAT form works on a later nightly.** The
  design changes shape if it does, losing the bound-propagation tax entirely, so it is worth a search
  before the canon fixes the standalone-trait spelling.
- **The real law count**, section 6.1. It is not in `110` and the pricing curve is parametric instead.
- **Whether `Cold`'s bitpacked access path survives the projection unchanged.** The shipped ladder gives
  Cold the same primitive ladder as Hot on the grounds that "bitpacking is an access-path concern, not a
  container-type concern" (`container.rs:197-198`). I carried that forward without testing it, and it is
  the one strategy whose container story I have taken on trust.
- **The four families beyond fixed point.** `130:669-707` compiles float and decimal against one contract
  with the container written. I did not redo that section against the projected container, and the
  exponent form's own laws are still open there.
- **Whether `min_specialization` could partition the ladder by const ranges.** I reasoned it cannot,
  because specialization is structural and ranges are not, and I did not compile the refutation.
- **The next-solver open bug list**, which `128:78-81` and `129:548` also left unchecked and which now
  matters more, because the flag is going further than either of them thought.
