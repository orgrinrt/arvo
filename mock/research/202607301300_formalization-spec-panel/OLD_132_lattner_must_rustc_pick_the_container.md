# 132. Must rustc pick the container, and what the feature is actually buying

**Persona:** Chris Lattner, compiler and layering lens. Fourth pass in this panel; file 12 was the fresh read,
74 rechecked the taxonomy, 83 asked how many widths, 93 the zero divisor, 107 asked whether it is-a or
contains-a.
**Date:** 2026-08-07
**Position:** premise attack on `131_giesen_the_strategy_picks_the_container.md`, section 8, which asks
whether the container must be a Rust type chosen by rustc at all. Reads `131`, `130`, `130b`, `129`, `128`,
`127b`, `110` where it bears on the container, the shipped `arvo-strategy/src/container.rs`, and the actual
source of `hilavitkutin-build`, `notko-build` and `notko-macros`.
**Pin:** `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `--edition 2024`, scratch tree outside the
repository. `mock/crates` read, never written. Probes at
`/private/tmp/claude-501/-Users-orgrinrt-Dev-clause-dev/dea47dac-5762-46b6-956e-0d22cc5d3832/scratchpad/lat132/`,
seventeen files named `q1` through `q17` in the order they appear here, plus a reproduction of `131`'s
consumer matrix and its GAT ICE.

The door `131` declined to open is open, and what is behind it is more interesting than either of us
expected. The opaque-carrier route does not exist in the form the brief describes, and the reason is not the
one `131` suspected. But opening the door found something that changes what the design is buying.

**The container derivation is two steps, and only one of them costs a feature.** Step A turns
`(I, F, sign, strategy)` into a byte count or a rung index, which is const arithmetic on generic parameters.
Step B turns that rung into a machine type, which is a case split over literal keys. `131` bought
`generic_const_args` for the pair and priced it as the price of container selection. Step B is **gate-free**,
compiles with no `#![feature]` and no `-Z` flag, and produces assembly identical to a hand-written native
container: two instructions scalar, an eight-lane NEON loop vectorised (`q15_op_dispatch.rs`, exit 0). What
the feature buys is step A alone, and step A is one const expression.

**The fixed-width carrier is not merely expensive, it is incompatible with a ruling op already made.** Its
size has to be the widest numeral the program contains, and op removed the width ceiling at `127b:118-126`.
A carrier with no ceiling has no size. The footprint numbers are bad on their own (`q7`, `q10`: eight times
for a machine-word carrier, thirty-two times for a wide one, on a million-record Cold column), but the
structural argument kills it before the numbers are reached.

**Neither build layer can touch a type.** `hilavitkutin-build` is a pragma table and a rustc wrapper, and its
own module doc says so: it "optimises HOW code is compiled (pragmas, profiles, rustc wrapper), **not what it
does**" (`hilavitkutin-build/src/lib.rs:3-6`). `notko-build` copies `.rs` files into `$OUT_DIR` and sets an
env var (`notko-build/src/lib.rs:88-140`). The one mechanism in the workspace that can substitute a type is
`notko-macros`, and it works because it runs before type check, not after. That is the finding, and it points
the deferral in the opposite direction from the one the brief assumed.

---

## 0. Gates, and the brief's claims checked before reasoning from them

**Canon gate: passed.** No ratified canon exists for arvo; this panel is producing the first one, so
`panels-argue-the-intent-not-the-wording.md` puts op's own calls and the intent in the governing position.
The governing calls here are op's container ruling (`130b:41-44`), the no-ceiling instinct
(`127b:118-126`), D48's surface (`127b:56-59`), and the convergence pressure (`127b:12-18`). Section 8 marks
the one place I hand a call back rather than making it.

**Test gate.** Not run, and I am naming it rather than letting it pass. `126:47-48` ran
`cargo test --offline --workspace` on a tree nothing has moved since and got 672 passed, 0 failed, 9 ignored;
`129`, `130` and `131` each declined to re-run it on op's ruling at `108b:174-181`. My deliverable touches no
crate in that tree. The instrument here was the compiler and the assembler.

**The toolchain.** `rustc +nightly-2026-05-28 --version --verbose` reports `1.98.0-nightly (57d06900f
2026-05-27)`, matching the brief.

### The brief's factual claims

*"The workspace ships build infrastructure and custom LLVM passes (`notko-build`, `hilavitkutin-build`)."*
**Half true, and the half that is false is the one the question rests on.** The build infrastructure ships.
The custom LLVM passes do not. `hilavitkutin-build/src/requirements.rs:23-25` names `polka-passes.so` and
`math-peephole.so` as an **external** requirement, an "LLVM pass plugin `.so` ... loaded via
`-Z llvm-plugins`", and neither file exists anywhere in the workspace:

```
find hilavitkutin notko arvo -name "*polka-passes*" -o -name "*math-peephole*"   ->   (empty)
```

So the passes are a documented dependency on tooling the consumer supplies, not code this workspace controls.
Section 3 says why it would not matter if they did ship.

*"`131` established that the projection is not available gate-free."* **Holds, and I reproduced the
consumer matrix independently rather than citing it.** Building `arvocore.rs` with the flag and then three
consumers against it: a consumer naming arvo types in a signature compiles with no gate and no flag (exit 0);
a consumer calling a width-generic law without the flag gets `E0277`; the same consumer with
`-Znext-solver=globally` **and no `#![feature]` at all** compiles (exit 0). `131:384-396` is correct and
`128:250-262` is wrong.

*"`generic_const_args` is vetted WATCH."* Holds, `128:17-19`. Section 6 prices it with two numbers neither
`128` nor `131` had.

*"`min_generic_const_args`, `adt_const_params`, `min_specialization` and the const-traits family are
allowed."* Holds per `unstable-features.md`. I checked whether mGCA **alone** suffices for step A, since that
would remove the flag and invert everything, and it does not (`q16_mgca_only.rs`). rustc says so in as many
words:

```
error: generic parameters may not be used in const operations
  --> q16_mgca_only.rs:11:49
   |
11 |     type const BYTES: usize = const { bytes_for(I + F) };
   |                                                 ^
   = help: add `#![feature(generic_const_args)]` to allow generic expressions as the RHS of const items
```

---

## 1. The premises this brief takes for granted

Op has overturned four converged conclusions this session by asking why an assumption was there, so before
answering I went looking for the assumptions in my own brief. Two are load-bearing and one is a
misattribution that should be corrected before it propagates further.

**That op assigned the container decision to hilavitkutin-build.** He did not, and the brief's framing puts
words in his mouth. Here is the whole of what he said (`130b:41-44`):

> Container naming is explicitly wrong. The entire idea of arvo is that the strategy guides container
> selection, not the user. User writes strategy and arvo optimises accordingly. And also, the same semantics
> and typestate will be used by other optimisation steps, such as the already well designed
> hilavitkutin-build.

Read the second sentence and the fourth together. Container selection is assigned to **arvo**. What
hilavitkutin-build gets is the **semantics and typestate**, which it *uses*, downstream. Reading is not
choosing. `131:846-850` restated it as "if hilavitkutin-build can be given the format and the strategy and
left to choose the storage", and the brief inherited that restatement as though it were op's. It is not, and
the difference matters: op's sentence is satisfied by arvo picking the container and hilavitkutin-build
reading the resulting typestate, which is what `110:3251` already specifies when it puts
`Lowering::Container` on the trait as "derived, never declared as an axis". Nothing in op's ruling asks for
deferral past rustc.

I am still answering the question, because "can it be deferred" is worth knowing whether or not op asked for
it, and because if it could be the design would be cheaper. But the answer being no does not leave op's
ruling unsatisfied, and a reader should not come away thinking it does.

**That "opaque carrier" and "deferred choice" are the same thing.** They are not, and separating them is
what produced this file's main finding. A carrier can be opaque in the *type* (bytes, not `u16`) while the
choice of machine width is made at the ordinary Rust level in the operation body. Section 4 shows that shape
compiling gate-free with native codegen. The brief's phrase "with rustc seeing only an opaque carrier"
bundles two independent axes: what the field's type is, and who decides the size. Only the second is hard.

**That the shipped `Cold` treatment is an instance of deferred representation.** The brief says the design
"already treats `Cold`'s bitpacking as an access-path concern, not a container-type concern
(`container.rs:197-198`), which is an existing instance of exactly this move". I read the source and the
citation is accurate:

```rust
// Cold: same primitive ladder as Hot; bitpacking is an access-path
// concern, not a container-type concern.
```

But it is an instance of a *different* move, and section 5 says which. `Cold`'s numeral is a `u8` in a
register exactly like `Hot`'s; what differs is how a **collection** of them is laid out in memory. That is
not the container decision being deferred, it is a second decision, made by ordinary generic Rust in the
column type, about how to store many of a thing whose single-value type is already fixed. It is a real and
useful precedent, and it is a precedent for something the design should do more of, but it does not show that
the numeral's own container can be deferred.

---

## 2. What the storage decision actually is, and when rustc needs it settled

The question decomposes cleanly once you separate the two phases rustc runs.

**Type checking a generic body does not need a size.** It needs `Sized` as a *bound*, which is a fact about
the type parameter, not about any particular instantiation. This is why `[u8; N]` type-checks generically and
why `size_of::<T>()` is callable in generic code.

**Monomorphisation does need the size, and it bakes it in as a literal.** This is the part that decides the
question, and it is worth showing rather than asserting. From `q4_layout_is_baked.rs`, at `-O`, comparing a
one-byte payload against a thirty-two-byte one:

```llvm
define noundef i8 @idx_narrow(ptr ... dereferenceable(4096) %a, i64 noundef %i)
  %0 = getelementptr inbounds nuw i8, ptr %a, i64 %i

define noundef i8 @idx_wide(ptr ... dereferenceable(131072) %a, i64 noundef %i)
  %0 = getelementptr inbounds nuw %Wide32, ptr %a, i64 %i

define noundef i64 @sz_narrow() { ret i64 4096 }
define noundef i64 @sz_wide()   { ret i64 131072 }
```

Three things are already constants in the IR rustc handed to LLVM. The stride is in the GEP's type operand.
The extent is in the `dereferenceable` attribute. And `size_of` has been folded to a function that returns a
literal. There is no record anywhere in that IR that `131072` was ever `4096 * 32`, and no annotation saying
which of the many integers in a real module are sizes of this type as opposed to sizes of something else.

That is the precise answer to "what does rustc need settled and when". **Layout is settled at
monomorphisation, and it is settled by being spelled out, not by being referred to.** Everything after that
point receives the answer as data with the question erased.

**Why a later pass cannot undo it, stated as an engineering property rather than an opinion.** To change a
type's size after this point, a pass would have to rewrite every GEP, every `dereferenceable`, every folded
`size_of`, every `memcpy` length, every stack slot, and every argument-passing decision, consistently, across
the whole program including code it cannot see. It would have to know which integer literals were derived
from the layout and which were coincidence. And it would have to be right about `transmute`, about every
`#[repr(C)]` boundary, and about pointer arithmetic the source performed by hand. That is not a pass, it is a
second compiler with more information than the first one had. I have watched several attempts at
whole-program layout rewriting over the years, and the ones that work have the layout decision *inside* the
compiler that emits the code, which is precisely where Rust already puts it.

So the premise dies cleanly, which the brief correctly identified as a real result. But it dies for a
sharper reason than "LLVM cannot change a type's size". It dies because **by the time any layer downstream
of rustc exists, the size is not a decision any more, it is a number**, and there is no arrangement of
downstream tooling that recovers the decision from the number.

---

## 3. What the later layers can and cannot do

I read the source rather than the crate names, because the crate names oversell and the module docs undersell
in exactly the way that would mislead someone reasoning from a directory listing.

**`hilavitkutin-build` is a pragma table plus a rustc wrapper, 1023 lines including tests.** Its own module
doc is the clearest statement of the boundary anyone has written:

> The crate optimises HOW code is compiled (pragmas, profiles, rustc wrapper), not what it does.
> (`hilavitkutin-build/src/lib.rs:5-6`)

The thirteen pragmas it exposes (`lib.rs:35-49`) are `LoopOptimization`, `Polly`, `MathPeephole`, `FastMath`,
`ExpandedLto`, `Pgo`, `Bolt`, `Profiling`, `BuildStd`, `ParallelCodegen`, `SharedGenerics`, `LoopFusion`,
`MimallocAllocator`. Every one is a flag, a profile setting, an external tool invocation, or a generated
`Cargo` config. `pragma.rs` in full is a thirteen-bit mask with an iterator over it. There is no type
machinery in the crate and no place where one could be added without changing what the crate is.

**`notko-build` copies files.** `collect_and_distribute` scans a crate-local `notko-optimizers/` directory
and dependency-propagated `DEP_*_NOTKO_OPTIMISER_PATH` env vars, copies `.rs` files into
`$OUT_DIR/notko-optimisers/`, detects name collisions, and emits two `cargo:` lines
(`notko-build/src/lib.rs:88-140`). That is the whole crate.

**The one layer that can substitute a type is the proc macro, and it is upstream of rustc, not downstream.**
`notko-macros` ships `#[profile(Tier)]`, which "AST-rewrites function bodies between Hot / Warm / Cold
fallibility tiers at compile time" (`notko-macros/README.md:3-4`), and the rewrite genuinely changes a type:
`Result<T, E>` becomes `Just<T>` or stays `Outcome<T, E>` depending on the tier and the `internal` feature.
`notko-build`'s entire job is to get custom tier files to that macro. So the workspace does have a
build-driven type substitution, and it works for the same reason everything else does not: **it happens
before name resolution, when the tokens are still tokens.**

That gives the boundary its honest shape, and it is a shape worth writing into the canon because it will
come up again:

| Layer | Runs | Can it change a type? |
|---|---|---|
| proc macro (`notko-macros`) | before name resolution | yes, on the token stream it is given |
| rustc type check | after | decides the type; nothing above it |
| rustc layout and monomorphisation | after | computes size and align from the type |
| LLVM, plugin passes, LTO | after | receives layout as literals |
| BOLT, post-link | after | rewrites machine code, no type information at all |

**And the aspirational version does not help either.** `hilavitkutin/mock/agent/MAIN.md.tmpl:24` says the
crate "holds LLVM passes, MIR manipulation, cfg emission, PGO / BOLT workflows". The first two are not in the
source, and if they were, they would sit in rows four and three of that table respectively. MIR is already
post-type-check: a MIR pass sees `Fixed<13, 3, Unsigned, Warm>` with its layout computed. The doc's claim is
what would lead a reader to think this layer has more reach than it does, and it is the kind of gap between a
generated agent instruction and its source that is worth naming when it is load-bearing for a design
question, as it was for the brief that produced this file.

**Why the proc-macro route does not rescue the design, stated so nobody re-derives it.** A macro can compute
`bytes_for(1 + 12 + 3)` at expansion time and emit `Fixed<12, 3, Signed, Warm, 4>` with the byte count
filled in. That works for literal widths. It fails for width-generic code, because inside
`fn mul<const I: u32, const F: u32, ...>` there are no literals for the macro to compute with, and
width-generic code is exactly where the design needs the projection: it is what arvo's own laws are. The
macro also costs D48's plain type syntax, which op held at `127b:59`. So the mechanism that can defer a type
choice is available precisely where the choice is easy and absent precisely where it is hard.

---

## 4. The opaque-carrier route, and the decomposition it uncovers

Here the answer stops being a clean no. There is a real shape, it compiles gate-free, and it is not the one
the brief describes.

### 4.1 The byte carrier is free, and the reason is not the one you would guess

Start with the codegen question, because if a byte-array carrier costs codegen then nothing else about it
matters. From `q8_bytes_vs_native.rs`, comparing `#[repr(transparent)] Native(u16)` against
`#[repr(transparent)] Bytes<2>([u8; 2])`, both wrapping-add, scalar and over a 1024-element array:

```
_v_native:                      _v_bytes:
  ldp q0, q1, [x8, #-32]          ldp q0, q1, [x8, #-32]
  ...                             ...
  add.8h v0, v4, v0               add.8h v0, v4, v0
  ...                             ...
```

Byte for byte identical, including the eight-lane NEON add. So a byte carrier at align 1 costs nothing.

But that result is a trap, and I nearly reported it as one. The `Bytes<2>` body in that probe was
`u16::from_le_bytes(...)`, written against a concrete `2`. The recovery came from **the body naming the
machine type**, not from anything about the carrier. Change the body to be honestly generic over the byte
count, a ripple-carry loop over `[u8; B]`, which is the only body a design with no ladder could write, and
the picture inverts (`q9_generic_limb.rs`, instruction counts from the emitted assembly):

| Function | Instructions | Native equivalent |
|---|---|---|
| `limb_b2` (scalar, `[u8; 2]`) | 10 | `nat_16`: 2 |
| `limb_b8` (scalar, `[u8; 8]`) | 66 | `nat_64`: 2 |
| `v_limb_b2` (1024-element loop) | 57 in the loop body | `v_nat_16`: 17 |

Thirty-three times the instruction count at eight bytes. And the vector form is worse than the count
suggests: it deinterleaves with `xtn.8b` and `shrn.8b`, does byte-lane adds with an explicit carry chain, and
reinterleaves, where the native form does one `add.8h`. LLVM cannot recover the wide add because the carry
chain is a real data dependence the source wrote. It is not a missed optimisation, it is the program.

**So the codegen answer is conditional, and the condition is the whole question: a byte carrier is free
exactly when something names the machine type, and costs up to thirty-three times when nothing does.**

### 4.2 The shape that works, and it is gate-free

Which raises the obvious move. Keep the carrier opaque in the type and put the case split in the operation
body, keyed on the byte count. `q15_op_dispatch.rs`, no `#![feature]`, no `-Z` flag, exit 0:

```rust
pub trait Machine { type M: Copy; fn add(a: Self::M, b: Self::M) -> Self::M; }
pub struct Rung<const B: usize>(PhantomData<[(); B]>);
impl Machine for Rung<1> { type M = u8;  fn add(a: u8,  b: u8)  -> u8  { a.wrapping_add(b) } }
impl Machine for Rung<2> { type M = u16; fn add(a: u16, b: u16) -> u16 { a.wrapping_add(b) } }
impl Machine for Rung<4> { type M = u32; /* ... */ }
impl Machine for Rung<8> { type M = u64; /* ... */ }

impl<const B: usize> Bytes<B> where Rung<B>: Machine {
    #[inline] pub fn add(self, o: Self) -> Self { /* transmute_copy, dispatch, transmute_copy */ }
}
```

Codegen, from the emitted assembly:

```
_d_b2:   add w0, w1, w0 ; ret          (2 insns, identical to native u16)
_d_b8:   add x0, x1, x0 ; ret          (2 insns, identical to native u64)
_v_d_b2: 17-instruction loop with add.8h, identical to _v_nat_16
```

**A container ladder is expressible gate-free and lowers optimally.** That is worth stating plainly because
the panel has been operating on the belief that container selection is what the feature is for. It is not.

The reason it works is precisely op's own `Capacity` observation at `127b:41-50`: a const may be carried and
read, never transformed on the way into a type. Here `B` reaches `Rung<B>` as a standalone argument and the
impls are keyed on literals. No const operation appears anywhere.

### 4.3 What the feature is actually buying

Put the two together and the derivation splits at a seam nobody in this panel had drawn:

**Step A.** `(I, F, sign, strategy)` to a byte count or rung index. `bytes_for(G::EXTRA + I + F)`, plus the
strategy's headroom bump. Const arithmetic over generic parameters. **This is what costs
`generic_const_args` and therefore `-Znext-solver=globally`.**

**Step B.** rung to machine type. A case split over literal keys. **Gate-free** (`q15`, exit 0, native
codegen).

`131` bought the pair and reported the price as the price of container selection; `128:147-159` measured a
fit check and reported it as the ladder. Both were describing the same object from different sides and
neither named the seam. The seam matters for three reasons. It says exactly what a future stabilisation would
relieve, which is one const expression and not the ladder. It says where to look for a cheaper mechanism,
which is step A and only step A. And it means a design that ever finds a gate-free step A keeps everything
else unchanged, because step B is already the shape it would keep.

I looked for a gate-free step A in three places and found none.

**The const-parameter default position.** `struct Foo<const N: usize, const B: usize = N>` compiles with
zero features (`q1_default_plain.rs`, exit 0), which is the carry-not-transform pattern again. But
`= { bytes_for(N) }` is refused identically to every other position (`q2_default_expr.rs`):

```
error: generic parameters may not be used in const operations
  = help: const parameters may only be used as standalone arguments here, i.e. `N`
```

Under mGCA plus GCA the default position does admit a `type const` projection (`q6_default_typeconst.rs`,
exit 0), which is a spelling `131` did not try and which may avoid its bound-propagation tax, but it is the
same purchase. Recorded in section 7 as a spelling question rather than a route.

**The fixed-width carrier**, which is the brief's own suggestion. Section 4.4.

**Relocating step A into the operation body.** This is where the byte count would be computed from `I` and
`F` inside a function rather than in a type. It does not help, because the *field* still has to have a size,
and the field's size is step A's output. Moving the computation does not remove the obligation to have
computed it.

### 4.4 The fixed-width carrier, and why it is dead before the numbers

The brief proposes "a fixed-width carrier with the narrow form as a lowering concern rather than a layout
concern". It is buildable and gate-free. The footprint, measured (`q7_maxcarrier.rs`, `q10_word_carrier.rs`,
all assertions hold at exit 0), on a `UFixed<3, 0, Hot>`:

| Carrier | One value | One million records | A 13-field record | Align |
|---|---|---|---|---|
| projected (`u8`) | 1 byte | 1,000,000 | 13 | 1 |
| machine word (`u64`) | 8 bytes | 8,000,000 | 104 | 8 |
| widest (`[u8; 32]`, align 16) | 32 bytes | 32,000,000 | 416 | 16 |

Eight times and thirty-two times, on the exact workload `arvo-toolbox-not-policer.md` names as the reason
arvo exists: "Millions of entities run concurrently ... every saved bit compounds across the entity count ...
Bitpacking is the common case for the substrate's primary downstream, not the edge case." A design whose
carrier is eight times too big has given up the thing it was built for.

But the numbers are not the argument, and I want the argument on the record because it is shorter and it does
not depend on anyone's workload. **A fixed-width carrier needs a fixed width, and a fixed width is a ceiling.
Op removed the ceiling** (`127b:118-126`, and `131:282-284` confirms the shipped ladder is total because the
wide rung is parameterised by a byte count). The width ceiling op refused as a policy decision comes straight
back as the carrier's size, wearing a different hat. So the fixed carrier is not a tradeoff the design may
take at a cost; it is unavailable under a standing call.

That is the cleanest result in this file and it did not need a single measurement.

---

## 5. What the `Cold` precedent actually shows, and what the design should do with it

The brief flags that `131` carried `container.rs:197-198` forward untested, and it is worth untangling
because the precedent is real and is being read as evidence for the wrong claim.

Cold's numeral is the same primitive as Hot's: `impl_native_bucket!(Cold, [0 => u8, 1 => u16, ...])` at
`container.rs:200-204`, identical to Hot's list at `container.rs:191-195`. So a `UFixed<3, 0, Cold>` value in
a register is a `u8`, and its container was chosen by the same ladder at the same time by the same mechanism.
Nothing about the numeral is deferred.

What differs is a **second, separate decision**: how a collection of them occupies memory. A thousand
three-bit values can be a thousand bytes or three hundred and seventy-five, and that choice belongs to the
column, not to the numeral. It is made in ordinary generic Rust, by a type parameterised over the numeral,
with no feature and no deferral, because at that point the numeral's width is already known.

Two things follow, and I think both belong in the canon.

**The precedent is for a split the design should name explicitly.** There are two representation questions,
not one: what a value is in a register, and what many values are in memory. The first must be a Rust type and
must be settled at monomorphisation. The second is free, is ordinary code, and is where every representation
trick the design wants (bitpacking, striping, delta encoding, SoA) actually lives. `110:3251` currently puts
`Container` on `Lowering` as a single derived member; the pair deserves two names, because they have
different mechanisms and different costs.

**It also relieves the worry the brief raised.** `Cold`'s bitpacked access path survives the projection
unchanged, because the projection changes how the container type is *reached* and not what it *is*: Cold's
row of the ladder is untouched by whether the tag is computed under GCE, under GCA, or written by hand. I did
not build arvo to confirm this and mark it a source reading rather than a compiled result, but the reading is
a two-macro comparison in one file and it is not close.

---

## 6. The comparison, and what the flag costs a downstream crate

`131` says the flag reaching consumers is bigger than the panel was told and hands the trade to op. It is
the right thing to hand over, but the input was incomplete: nobody had measured what the flag does to a crate
that receives it. I did.

**Reproduction of the exposure, independently, on my own build.** Library compiled with the flag, then three
consumers:

| Consumer | Feature gate | Flag | Result |
|---|---|---|---|
| names arvo types in a signature | no | no | exit 0 |
| calls a width-generic law | no | no | `E0277`, unsatisfied `Project` bound |
| calls a width-generic law | no | **yes** | exit 0 |

`131:384-396` reproduces exactly. Note the third row carefully: **the flag alone is sufficient and the
feature gate is not needed downstream.** So what a consumer inherits is one `-Z` flag in its build
invocation, not a `#![feature]` in its crate root. That is a smaller thing than the panel's phrasing has
implied, and it matters for how the cost is described.

**What the flag costs in compile time: nothing measurable.** A 759-line crate with 120 fold implementations,
chained associated-type `Store` impls, a `Tagged` wrapper, an indexed hlist `Contains` in the `AccessSet`
shape, and generic reduction functions (`q12_scale.rs`), warm, three runs each:

| Configuration | Runs |
|---|---|
| baseline | 0.15 s, 0.16 s, 0.16 s |
| `-Znext-solver=globally` | 0.16 s, 0.16 s, 0.17 s |

Within noise. Whatever the flag costs, it is not the trait solver being slower on ordinary generic code.

**What the flag costs in compatibility: nothing I could find, including the things most at risk.** The
next-generation solver's known rough edges are around coherence and specialization, which is exactly the
machinery hilavitkutin-api ships. `q13_downstream_gates.rs` puts the whole of that gate list in one crate,
`min_specialization` with a `default fn` and a concrete override in the `column_value.rs` shape,
`marker_trait_attr`, `negative_impls`, `impl_trait_in_assoc_type`, `associated_type_defaults`,
`adt_const_params` and `const_trait_impl`:

```
baseline:                    exit 0
-Znext-solver=globally:      exit 0
```

And full `specialization`, which the engine still ships at `hilavitkutin/src/lib.rs` and which
`unstable-features.md` records as drift to remediate, also survives the flag (`q14_full_spec.rs`, both
configurations exit 0). So the flag does not collide with the current engine even in its unremediated state.

**What is not measured, stated so it is not read as more than it is.** This is one 759-line synthetic crate
plus a gate-compatibility probe, not hilavitkutin and not vehje. A behavioural difference in the new solver
on some inference shape those crates contain would not appear here. What I can say is that the two failure
modes most likely a priori, compile-time blowup and coherence rejection, did not appear on the code most
likely to trigger them.

**The status quo is worse and the panel keeps understating this.** `generic_const_exprs` is live in
`arvo/mock/crates/arvo/src/lib.rs:25`, `arvo/mock/crates/arvo-strategy/src/lib.rs:11`, and
`hilavitkutin/mock/crates/hilavitkutin/src/lib.rs:24`, verified by grep. It is **forbidden** as of op's
2026-07-28 ruling, on the grounds that the const-generics team calls the design "fundamentally flawed" and
that `min_generic_const_args` is the ground-up rewrite. So the comparison is not "buy a watch-tier feature
versus buy nothing". It is:

| | today | under `131`'s projection |
|---|---|---|
| feature in arvo | `generic_const_exprs`, **forbidden** | `min_generic_const_args` + `generic_const_args`, allowed + WATCH |
| what a consumer inherits | the same refusal, unfixable | one `-Z` flag, measured free |
| drift status | open remediation item since 2026-07-28 | closed |

### The verdict

**Take `131`'s projection.** Three reasons, in the order I weigh them.

**One, there is no alternative, only alternatives to having the design at all.** Section 2 closes the
deferral route on the structure of compilation rather than on any implementation's limits. Section 4.4 closes
the fixed carrier on a standing call. Section 3 shows the only layer that can substitute a type runs before
the point where the widths are still generic. What is left is: derive it in the type system, or write it and
give up op's ruling.

**Two, the price is smaller than the panel has been describing, in both directions.** Downstream, one `-Z`
flag rather than a feature gate, measured free on compile time and compatible with every gate the named
downstream consumer ships. Upstream, the purchase buys one const expression, not the ladder, because step B
is gate-free and would be kept unchanged under any future mechanism.

**Three, it removes a forbidden gate.** Under `unstable-features.md`'s own ordering, trading a forbidden
feature for a WATCH one plus a flag is strictly an improvement, and it closes a remediation item that has
been open since 2026-07-28.

I hold this as a reading rather than a ruling, and section 8 marks which part is op's. But I want to be
direct about where I would push back if the panel started reaching for a compromise: **there is no cheaper
projection, so the choice is between this and no projection.** A design that lets the consumer write the
container is not a cheaper version of op's ruling, it is the thing he refused, and it deletes the derived
member `110:3251` specifies for the downstream optimisation layer to read.

---

## 7. The two side questions

### 7.1 The GAT ICE is, as far as I can find, unreported

I reproduced it on my own build (`c1_ice_min.rs`, exit 101, eighteen lines):

```
thread 'rustc' panicked at compiler/rustc_type_ir/src/binder.rs:1317:13:
cannot find `!BoundConst { var: 1, .. }` in param-env: ParamEnv {
        Binder { value: ConstArgHasType(!BoundConst { var: 0, .. }, u32), bound_vars: [] },
        Binder { value: ProjectionPredicate(AliasTerm { args: [Hot, ^BoundConst { var: 0, .. }], ... }
```

I searched for it. The nearest candidate by name is rust-lang/rust#131668, "ICE: Unexpected const bound var",
which is a **different panic** (`compiler/rustc_infer/src/infer/freshen.rs:178`, message "unexpected const
^0"), involves `generic_associated_types_extended` and higher-ranked bounds rather than `type const`
projections, and is closed as not planned. rust-lang/rust#102333 ("ICEs galore with const-generic associated
type in trait bound") and #107920 ("cannot relate constants") are in the same neighbourhood and are also not
this. The one hit that matches the message shape is a rust-analyzer fix (rust-analyzer#21235), a different
codebase.

So: **not found, likely unreported, and worth filing.** The reproduction is eighteen lines and it is against
two features actively being developed, which is the best possible position for a bug report. The design's
shape changes if it is fixed, because the GAT form carries no where-clause and therefore no bound-propagation
tax, and `110:3251` already puts the container on `Lowering` where the GAT would sit. Filing it costs
minutes and the payoff is a simpler canon.

Alongside that, `q6_default_typeconst.rs` shows the const-parameter default position accepting a `type const`
projection under mGCA plus GCA. That is a third spelling of the same purchase, and it may or may not carry
`131`'s bound-propagation tax. I did not price it against the standalone-trait spelling. Section 9.

### 7.2 Precision must be sign-free, second read, confirmed

`131:45-51` and its section 7 are correct, and I checked the arithmetic independently rather than accepting
the compiled diagnostic (`q17_precision.rs`, every assertion holds, exit 0):

```rust
const _: () = assert!(precision(12, 3) == 15);   // signed Q12.3: significand digits
const _: () = assert!(stored(1, 12, 3) == 16);   // ... in 16 stored bits
const _: () = assert!(precision(24, 6) == 30);   // coordinates add, sign-free
const _: () = assert!(stored(1, 24, 6) == 31);   // ONE sign bit in the product, not two
```

The general fact is that a two's-complement product of two `n`-bit signed values needs `2n - 1` bits, not
`2n`, because the product carries one sign and the inputs carried two. A law written over stored widths says
`16 + 16 == 32` and is wrong by exactly that bit. It is not wrong in a safe direction: the same file confirms
`rung(1 + 31 + 32) == 64` and `rung(32 + 33) == 128`, so the false law crosses the sixty-four-bit boundary
and doubles the container.

This is the second independent read `131:834-836` asked for, reached from the arithmetic rather than from its
file, and it agrees. It bears on my own question in a way worth stating: **the projection is keyed on
`(I, F, sign, strategy)` and not on a single width**, so keeping the sign a separate marker is what lets one
`Project` impl serve both families at every rung. Folding the sign into precision would need the ladder to
know which family it is in, which is a fifth axis for no gain.

I record it as a second read and not as settled. It contradicts `130:240` and `129:521`, and op wrote the D69
sentence it rests on.

---

## 8. What is op's, separately from what I decided

**Mine, and compiled.** That layout is settled at monomorphisation and emitted as literals, with the stride
in the GEP type, the extent in `dereferenceable`, and `size_of` folded to a constant (`q4`), so no layer
downstream of rustc can revisit it. That `hilavitkutin-build` is a pragma table and a rustc wrapper by its
own module doc and by its whole 1023-line source, that `notko-build` copies files, that the named LLVM pass
plugins are external and absent from the workspace, and that the only layer able to substitute a type is the
proc macro, which runs before the widths are generic. That a byte-array carrier is free when the body names
the machine type and costs up to thirty-three times when it does not (`q8`, `q9`). That the container ladder,
step B, is expressible **gate-free** with codegen identical to a native container (`q15`), so what
`generic_const_args` buys is step A alone, one const expression. That `min_generic_const_args` alone does not
buy step A, with rustc's own diagnostic naming `generic_const_args` as the requirement (`q16`). That the
const-parameter default position carries but does not transform (`q1`, `q2`), and admits a `type const`
projection under GCA (`q6`). That a fixed-width carrier costs eight times at a machine word and thirty-two
times at the widest, on the workload arvo exists for (`q7`, `q10`), and is unavailable regardless because its
size is the ceiling op removed. That the flag reaching consumers reproduces exactly as `131` reports, that
**the flag alone suffices downstream with no feature gate**, that it costs nothing measurable in compile time
on 759 lines of ordinary generic code, and that it accepts `min_specialization`, `marker_trait_attr`,
`negative_impls`, `impl_trait_in_assoc_type`, `associated_type_defaults`, `adt_const_params`,
`const_trait_impl` and even full `specialization` (`q11` through `q14`). That the GAT ICE reproduces and has
no upstream issue I could find.

**His, and it is the one that blocks.** The same one `131` handed over, now with two numbers it did not have.
Whether the container projection is worth `-Znext-solver=globally` reaching every consumer that does generic
arithmetic. My reading, offered as a reading: yes, and more confidently than `131` offered it, because the
downstream inheritance is a `-Z` flag rather than a feature gate, because it measured free on both compile
time and gate compatibility, and because the alternative is not a cheaper projection but the ruling being
withdrawn.

**His, because the rule has no answer.** Where `-Znext-solver=globally` sits in `unstable-features.md`.
Raised at `128:287-306`, unsettled through `131:829-832`. Section 6 changes the input again: the flag is now
known to reach consumers, known to be sufficient on its own without a feature gate, and measured against the
gate list of the named downstream consumer. It should be decided with all three on the table.

**His, because it touches a ratified structure.** Whether `110:3251`'s single `Lowering::Container` member
should become two, the register container and the collection layout, per section 5. The shipped `Cold`
treatment already behaves as though they are two and the doc names one. This is a change to a ratified line
and I am not making it.

**His, and it is small but it is his.** Whether precision may be sign-free. This is the second read `131`
asked for and it agrees from independent arithmetic, but op wrote the D69 sentence and two files currently
say otherwise.

**Owed under the two-expert rule.** I am the second read on `131`'s core claim that the projection is
unavailable gate-free, and I agree, having reproduced its consumer matrix and added `q16` showing mGCA alone
does not suffice. I am the **first** read on the step A / step B decomposition, on the gate-free ladder, and
on the flag-cost measurements, and none of that should enter the canon on one expert's word. The premise a
second read should attack in my file: **that step A is irreducible.** I searched three positions and found no
gate-free route, but "I looked in three places" is not "there is nowhere". The specific shape I would send
someone after is whether the strategy's headroom rule and the byte-count rule can be restated so that the
only thing crossing into type position is a value the consumer already writes, which would make the whole
purchase disappear. I do not think it can, because `I` and `F` are written separately and their sum is the
transform, but I did not exhaust it.

---

## 9. What I did not check

- **Whether `q6`'s const-parameter-default spelling avoids `131`'s bound-propagation tax.** It is a third
  spelling of the same purchase and it compiles; I did not build a law surface against it and compare the
  where-clauses. If it does avoid the tax it is the better spelling and the GAT ICE stops mattering.
- **Whether the flag changes behaviour on hilavitkutin or vehje proper.** I measured a synthetic crate and a
  gate-compatibility probe. Building those trees was available and I judged the cost against the panel's
  convergence pressure; a reader who wants certainty should run it before the canon locks the choice.
- **Whether the wide rung's `WideBits<BYTES, A>` arithmetic pays the `q9` penalty.** Above 128 bits there is
  no native type, so the multi-limb body is the correct implementation rather than a defect, but nobody has
  priced how much of `q9`'s thirty-three-times figure is inherent at that rung and how much a limb width of
  `u64` rather than `u8` would recover.
- **Whether `Cold`'s bitpacked access path survives the projection.** I reduced this to a source reading in
  section 5, which is stronger than `131`'s untested carry-forward but is still not a compiled result.
- **The real law count**, still not in `110`, still parametric. `131:663-666` flagged it and it is still open.
- **Whether filing the ICE upstream produces a fix on any timeline the design can wait for.** It should be
  filed regardless; the design should not plan around it landing.
