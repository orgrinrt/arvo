# 20: The build layer contract

**Member:** Andy Wingo. Compilers and runtimes from the inside: Guile's bytecode interpreter and JIT,
JavaScriptCore and V8 and SpiderMonkey by way of Igalia, WebAssembly's GC and tail-call proposals, the
Whippet collector. The seam this file is about, a high-level intent having to survive into generated
code, is the seam I have spent most of my time on, usually because it did not survive and something
was silently wrong downstream.

**Position:** seventh member of the algebra dive, file 20. Not a synthesis. The dive continues.

**What I read.** The brief's five op files (`16b`, `16c`, `16d`, `17b`, `13c`) twice each, as
instructed. The spec `11_current_shape_draft.md`, sections 3.1, 3.2, 5.1 and 5.2 closely and the rest
in outline. `17_orchard_are_these_all_grades.md` in full, section 8 line by line, and
`17_probes/08_the_licence_never_leaves_the_crate.rs` in full because it is the artifact I was asked to
test. `19_ringer_the_witness_and_its_upkeep.md` sections 2a and 3, `13c`'s inherited four findings,
`12_lattner_fresh_read.md`'s fidelity passage and `16_fallin_laws_as_backend_licences.md`'s toolchain
findings by their citations in 17. `unstable-features.md`, `arvo-always-optimal-internals.md`. Source
read lightly and only for the two purposes `16b` licenses: `arvo-spectral/src/power.rs`,
`arvo-spectral/src/fiedler.rs`, `arvo-numeric-contracts/src/lib.rs`, and the `no_std` attribute on
every crate root.

**Separating what I measured from what I reasoned about.** Sections 1, 2, 3 and 5 are measurements,
each with a probe in `20_probes/` and a runner that reproduces the numbers. Sections 4, 6, 7 and 8 are
design, reasoned from those measurements and from the spec's own text, and where I could compile a
piece of a proposal I did (section 6's reader is a working program, section 6's rewrite is measured end
to end). I have marked every claim that is reasoning rather than measurement.

**The test gate, first, because it is owed before the assigned work.** I ran the whole arvo suite on
the pinned nightly: exit 0, no failures, no ignored tests except one, roughly four hundred assertions
across sixteen crates plus nine `trybuild` compile-fail cases. I read the bodies of the tests in the
surface I touch. `crates/arvo/tests/strategy_semantics.rs` asserts concrete values against concrete
inputs (`strategy_semantics.rs:23`, `sum.to_raw() == 44u8` for a `u8` wrap) and is a real
test. The compile-fail cases
under `tests/ui/no_multiplicative_identity*` are the strongest thing in the suite, because they assert
a refusal rather than a value. **The surface this file is about has no tests at all, and that is the
correct state**, because the fidelity axis does not exist in the design yet: it appears nowhere in the
spec's ten-axis table (`11_current_shape_draft.md:151-183`). There is nothing to audit and nothing to
refuse. I note one thing the gate did surface and section 2 takes up: the four `private_bounds`
warnings from `arvo-bits-contracts/src/lib.rs:156,222,454,469` are a perimeter question of exactly the
kind `what-you-can-observe-is-what-you-guaranteed.md` names, and they are outside my lens.

---

## 0. Three premise checks, and the spec sentence that motivated this file

The dive's own record is that briefs and drafts in this review have carried false claims that later
members then reasoned from, so I checked mine before building on them. Three came back.

**The spec says a thing about fidelity that my measurements contradict, and it is load bearing.**

> arvo's floating-point types are sealed wrappers around hardware `f32`/`f64`, with LLVM fast-math
> flags as the only lever
> (`11_current_shape_draft.md:665-666`)

"The only lever" is false in both directions. It is false that fast-math flags are *a* lever, because
no rustc flag exposes them: `rustc +nightly-2026-05-28 -C help` and `-Z help` between them contain no
fp-contract, no fast-math, no float-mode option of any kind, which is file 16's finding and which I
reproduced. And it is false that they are the *only* one, because three of the four liberty classes
change their answer from ordinary source and the fourth is reachable through a mechanism section 6
designs. The sentence should go. It is the sentence that made three members reason about a licence
crossing a boundary, and the boundary it describes is not there.

**`17b`'s framing of my first half is accurate and I want to restate it as the thing I actually
tested.** Not "is file 17 right" but "how far does file 17's result reach". It reaches further than
files 12, 15 and 16 allowed and considerably less far than section 8 claims, and the gap between those
two is where the design work is.

**The one grep both file 16 and file 17 flagged as owed and neither ran** (`16_fallin...md:411-415`,
`17_orchard...md` section 9), which is whether any shipped arvo body already takes a liberty under a
licence that does not exist. It does, at three sites, and the shape of how it does is more interesting
than the fact:

```
arvo-spectral/src/power.rs:74     let inv = sq_sum.sqrt().recip();
arvo-spectral/src/fiedler.rs:96   let n_inv = n_f.recip();
arvo-spectral/src/fiedler.rs:137  let inv_norm = sq_sum.sqrt().recip();
```

That is `arcp`, the reciprocal-approximation liberty, applied unconditionally. File 17's own table
prices it as a real value change (`3ffaaaaaaaaaaaab` against `3ffaaaaaaaaaaaaa`). But look at where it
was taken. `Recip` is a trait in `arvo-numeric-contracts/src/lib.rs:44`, and `power_iteration` demands
it in its own where-clause (`arvo-spectral/src/power.rs:47`). The liberty is not in a `Number`'s body;
it is **in the bound of an algorithm generic over the number**, which has no `S` to read and no way to
be told. This is not a bug to fix in a crate nobody is defending. It is a fact about the shape:

> A fidelity licence attached to `Number<N, S>` does not reach the liberties that algorithms generic
> over `F` take through their trait bounds, because at the point the liberty is taken there is no `S`
> in scope.

Section 7 proposes what to do about it, and I flag now that it is the single largest hole I found in
the fidelity axis as anyone has drafted it, larger than anything about compiler flags.

---

## 1. Half one: how far file 17's dissolution reaches

The claim under test, from `17_orchard...md` section 8: in C the licence has to cross the type-erasure
boundary because the C compiler owns the operation, whereas arvo owns its own operation bodies, so all
four liberty classes are source-expressible, the single residue is vector lanes on a float reduction,
and that residue closes through stable `core::arch`. What a downstream target reads out of the types
is therefore "nothing", and what arvo needs from a build layer is "nothing".

**The core insight is right and it is the best thing in file 17.** Owning the operation genuinely does
change the problem, and the three members who reasoned about annotations crossing erasure were
importing a model that does not apply. I want that kept.

**The reach is narrower than stated, in four places, and each was found by compiling.** The summary
first, then each in its own section.

| liberty or family | file 17's finding | measured here | where it breaks |
|---|---|---|---|
| `reassoc`, value change | source-expressible | holds | nowhere |
| `nsz` | source-expressible | holds | nowhere |
| `arcp` | source-expressible | holds | nowhere, but see section 0 on where it is taken |
| `contract` | source-expressible via `mul_add` | **fails three ways** | section 2 |
| residue: float lanes | closes via `core::arch` | closes, and also closes portably | section 6 |
| saturating integers | not examined | **second residue, no channel exists** | section 3 |
| multi-limb integers | not examined | no residue, for a fragile reason | section 3 |

Measured on the pinned `nightly-2026-05-28` (rustc 1.98.0-nightly, LLVM 22.1.6),
aarch64-apple-darwin, with a cross check on x86_64-unknown-linux-gnu where the host cannot show the
point. The two x86 rows are read from the stable toolchain because that is the one with the x86 std
installed on this machine; `llvm.fma`'s lowering is not toolchain dependent and what changes between
those rows is the target's FMA support, not the compiler. Probes: `20_probes/01`, `20_probes/05`.

---

## 2. `contract` is not a liberty, it is a different operation, unavailable where arvo lives

This is the sharpest correction I have and it took three separate compilations to see fully.

`17_probes/08_the_licence_never_leaves_the_crate.rs:78-85` expresses the `contract` liberty as
`a.mul_add(b, c)`, and file 17's prose states it "is stable and lowers to `llvm.fmuladd`"
(`17_orchard...md` section 8). Three things are wrong with that, and they are wrong in different
layers.

**It is not `fmuladd`.** `20_probes/01_run.sh` part A reads the emitted intrinsic directly:

```
=== A. which LLVM intrinsic does f64::mul_add emit ===
   @llvm.fma.f64
```

`llvm.fma` and `llvm.fmuladd` are different contracts and the difference is exactly the one this dive
is about. `llvm.fmuladd` is a *licence*: the backend may fuse or may not, the result is whichever it
picked, and it is never slower than a multiply and an add. `llvm.fma` is an *operation* with an exact
definition, IEEE 754-2019's `fusedMultiplyAdd`, whose result is the single correctly-rounded value and
is the same value on every target that implements it. So the source form does not express "you may
contract". It expresses "produce the fused result, always". Those are not the same request, and a
design whose whole claim is that the typestate says exactly what is licensed cannot afford to conflate
them.

**It is not available in the environment arvo compiles in.** `f64::mul_add` is a `std` method. All
sixteen arvo crate roots are `#![no_std]` (`crates/arvo/src/lib.rs:14` and fifteen siblings). The
probe compiles clean as a standalone binary with `std` and fails immediately with `#![no_std]`:

```
error[E0599]: no method named `mul_add` found for type `f64` in the current scope
```

The `core` route exists on this nightly as `core::f64::math::mul_add` behind `#![feature(core_float_math)]`,
which is **not on `unstable-features.md`'s allowed, watch, or forbidden tables**, so it is unvetted and
by that rule must not ship enabled until it is vetted. The intrinsic that gives the actual licence
semantics, `core::intrinsics::fmuladdf64`, does emit `@llvm.fmuladd.f64` (I compiled it), and it needs
`#![feature(core_intrinsics)]`, which is **forbidden** by that same document's first table.

**And where it is available, it is a pessimisation exactly where a licence should be free.** The point
of a liberty is that granting it can only help. `20_probes/01_run.sh` parts B, C and D, reading emitted
assembly:

| target | `via_mul_add` | `via_source` |
|---|---|---|
| aarch64-apple-darwin | `fmadd`, `ret` | `fmul`, `fadd`, `ret` |
| x86_64-unknown-linux-gnu, baseline | **`jmpq`** (tail call into libm `fma`) | `mulsd`, `addsd`, `retq` |
| x86_64-unknown-linux-gnu, `+fma,+avx` | `vfmadd213sd`, `retq` | `vmulsd`, `vaddsd`, `retq` |

On the x86-64 baseline, which has no FMA unit, two arithmetic instructions become a call into a
software implementation. That is the correct behaviour for `llvm.fma`, which must produce the
single-rounded value whether or not hardware can. It is the opposite of what a liberty is for.

**So what does this do to the design, which is the part that matters.** The row does not simply fail.
It splits, and the split is a finding about the vocabulary:

- **`Fused` is an operation**, not a permission. It has an exact IEEE definition, one answer, and a
  cost that varies by target from one instruction to a function call. It belongs in the design as a
  named operation on the same footing as add and multiply, reachable when a numeral's numeral-side
  contract says the target has it, and its cost is a `Lowering` fact while its answer is a `Policy`
  fact. IEEE 754 has it and `13c`'s test therefore requires arvo be able to express it.
- **`Contract` is a permission**, whose whole content is "either answer is acceptable to me", whose
  value is that the backend picks the cheap one, and which is genuinely **not expressible from
  portable no-std source on this toolchain without a forbidden feature**. It is a real residue, and it
  is the one file 17 believed it had closed.

I would put both in the design and never let one stand for the other. And I note that the two readings
of "fidelity" that section 7 develops are visible already right here, in miniature, inside a single
row: `Fused` is fidelity as a choice of function, `Contract` is fidelity as an envelope of acceptable
answers.

---

## 3. The families nobody measured: saturating integers, and multi-limb

File 17 measured `f64` reductions. arvo's representations are mostly not `f64`.
`20_probes/05_the_widths_file_17_did_not_examine.rs` takes the two families the dive has not touched,
and reads vectorisation out of the LLVM IR rather than counting assembly mnemonics, because the
mnemonic counts are contaminated by loop-index arithmetic and I got two contradictory readings from
them before switching:

```
function                         vector-add  vec-reduce  uadd.sat  vec.sat
add_u256                                  0           0         0        0
reduce_saturating                         0           0         1        0
reduce_saturating_regrouped               0           0         4        0
reduce_u256                               0           0         0        0
reduce_wrapping                           7           1         0        0
```

**Wrapping integers confirm file 17 and file 16.** `reduce_wrapping` produces seven vector adds and one
`llvm.vector.reduce.add` with no annotation whatsoever, because wrapping integer addition is
associative and LLVM needs no licence to regroup it. Nothing is owed to anyone here.

**Saturating integers are a second residue with no channel at all, and this is new.** `Precise` selects
saturating arithmetic, and file 13 established by exhaustive measurement that saturating addition is
non-associative (`13c`, inherited findings). So LLVM correctly refuses to vectorise the reduction:
scalar `llvm.uadd.sat.i64`, zero vector operations. Two consequences, and the second is the one that
matters:

- **Source-level regrouping does not recover it.** For floats, regrouping in source recovered the four
  independent chains, which is file 17's second-best result. For saturating integers the regrouped
  version still produces zero vector operations (four scalar `uadd.sat` instead of one). The
  source-level move that works for floats does not work here.
- **There is no IR flag to grant.** LLVM's fast-math flags are defined on floating-point operations
  only. There is no `reassoc` for integer arithmetic, no equivalent on `llvm.uadd.sat`, and nothing a
  build layer could set. So the pass mechanism section 6 designs, which closes the float residue
  portably, **does not apply here at all**.

For saturating reductions the only route to lane parallelism is arvo writing the vector body itself
(aarch64 has `uqadd` and x86 has `paddus*`, so the kernels exist), per architecture, forever. That is a
larger bill than the float residue's, and it lands on arvo rather than on a build layer, and nothing in
this dive has priced it. I would want it in the spec's own ledger.

**Multi-limb has no residue, for a reason that should be written down rather than relied on.** The
256-bit add compiles to a clean `adds`/`adcs` carry chain from ordinary `carrying_add` calls, and the
reduction over 256-bit values does not vectorise because a carry chain is serial and there is nothing
to vectorise. So nothing is owed. But the good codegen comes from LLVM recognising the `carrying_add`
idiom, and `core::arch::aarch64` **has no carry-propagating intrinsic at all** (x86_64 has
`_addcarry_u64`), so if that idiom recognition ever regresses there is no `core::arch` door to fall
back through and the only remaining lever is inline assembly. That is a dependency on an optimiser
heuristic holding, which is exactly the kind of thing that is true until a toolchain bump and then
quietly is not. It costs one sentence in the spec and a codegen test to make it falsifiable, and I
would spend both.

**The corrected reach of file 17's claim**, stated as I would put it in the spec: the licence does not
have to cross the erasure boundary for any liberty whose effect is a *different answer*, because arvo
owns the body and can write both answers. It does have to cross, or be given up, for liberties whose
effect is the *same answer computed differently*, because the choice of how belongs to the backend and
arvo cannot write it portably. Three of the four float rows are the first kind. `contract` and lane
parallelism are the second kind. Saturating integers are a third kind again, where the choice belongs
to the backend and no channel to it exists.

---

## 4. Which contract fidelity sits in decides whether it may be delegated

This is reasoning, from the spec's text, and it is the piece I would most want a second reader on
because it settles a question this dive has been arguing without noticing there was a rule for it.

The spec gives a sorting test for which of the three contracts an axis belongs to:

> change it and ask whether the set of representable values changed. If yes, it is identity
> (`Numeral`). If the same values are representable but the arithmetic differs, it is policy
> (`Policy`). If neither changed and only the cost did, it is lowering (`Lowering`).
> (`11_current_shape_draft.md:140-142`)

and defines `Lowering` as "What it costs to hold and to compute. **Changes no answer.**"
(`11_current_shape_draft.md:115`).

Run fidelity through it. `Strict` and `Relaxed` represent identical value sets, and the arithmetic
differs: file 17 measured a real answer change on all four rows, and I reproduced three. So fidelity is
**`Policy`**. It is not `Lowering`, and the design's own definition of `Lowering` forbids it from being
`Lowering`, because it changes answers.

That reclassification has a consequence for op's framing at `16b` that I do not think anyone has drawn:

> arvo declares; the build side discovers and lowers.

is exactly right for `Lowering` axes and cannot be right, as stated, for `Policy` axes. A build layer
acting on `StoredWidth`, `Widening` or `Layout` cannot change a returned value, by the definition of
the contract those axes sit in, so delegating them is safe in a strong sense: no test can tell whether
the build layer ran. A build layer acting on a `Policy` axis changes what the program computes, which
means the numbers a test asserts depend on whether the build layer was in the loop. That is a
different kind of delegation and it needs a different rule.

I do not read this as contradicting op. I read it as the missing half of the same instruction, and it
sharpens `16c`'s obligation into something checkable:

> **A build layer may read every axis. It may act freely on `Lowering`, because acting there cannot
> change an answer. It may act on `Policy` only inside the envelope that `Policy` itself declared, and
> never at all on `Numeral`.**

Section 6 turns that into a mechanical criterion. Section 8 shows it subsumes file 17's prohibition,
which I was asked to test rather than inherit.

**The reading I am carrying against my own.** There is a coherent position that fidelity is
`Lowering` after all, on the grounds that the *mathematical* answer is unchanged and only the
floating-point realisation of it moves, so the design should treat a reassociated sum and a strict sum
as the same number computed at two costs. That reading is what makes op's original framing come out
right without amendment, and it is what a numerical analyst who thinks in error bounds would say. I do
not take it, because `Lowering`'s own definition says "changes no answer" rather than "changes no
mathematical answer", and because file 17's measurement (`2` against `0` on a four-element sum) is not
a rounding difference but a total loss of the result. But someone who wants an error-bounded reading of
`Lowering` could rebuild it, and if they do, the whole delegability question reopens in their favour.

---

## 5. What survives monomorphisation, measured

File 16 section 8 argued that type information is erased, and file 17 section 8 concluded from it that
what a downstream target reads out of the types is "nothing". The first is true about the *type
system*. Rust has no reflection here, `TypeId` is banned by
`unstable-features.md`'s own reasoning about model-width transfer, and rustc emits no metadata section
describing generic arguments. Nothing I found changes that.

It is false about the *object file*, and this is the measurement I would most like carried forward.

**Monomorphisation does not erase the type. It prints it.** `20_probes/02` declares a stand-in for
`Number<const I, const F, S, L>` with two const widths and two marker types, instantiates four
compositions, and dumps the symbols:

```
__RINvCs..._33_02_what_survives_monomorphisation4sum4 Kt17_ Kt29_ NtB2_6Strict  NtB2_9Bitpacked E B2_
__RINvCs..._33_02_what_survives_monomorphisation4sum4 Kt3_  Kt5_  NtB2_7Relaxed NtB2_9Bitpacked E B2_
__RINvCs..._33_02_what_survives_monomorphisation4sum4 Kt7_  Kt9_  NtB2_6Strict  NtB2_5Dense     E B2_
__RINvCs..._33_02_what_survives_monomorphisation4sum4 Kt7_  Kt9_  NtB2_7Relaxed NtB2_5Dense     E B2_
```

`Kt17_` is a `u16` const of value 0x17, which is 23, which is the `I` I passed. `Kt29_` is 41, which is
`F`. `6Strict` and `9Bitpacked` are the marker types by name. **Every generic argument of every
instantiation is in the symbol table, exactly, with its value.** The v0 mangling scheme is the default
on this toolchain, measured: I got the identical symbols with no `-C symbol-mangling-version` flag at
all. The channel is open today, for free, with no cooperation from anyone.

`20_probes/03_the_build_layer_reader.py` is the reader, so the claim is a program rather than a
paragraph. Piped the `llvm-nm` output and nothing else:

```
   I    F   Policy    Lowering
   3    5   Relaxed   Bitpacked
   7    9   Relaxed   Dense
   7    9   Strict    Dense
  23   41   Strict    Bitpacked

4 instantiations recovered from the symbol table alone.
```

No source, no type information, no build-script cooperation, no arvo change. That is the entire
"what a downstream target reads out of the types" question, answered by running it. A real
implementation calls `rustc-demangle` and walks the parse tree instead of my regexes; mine exists to
show the information is present and recoverable and to be checkable by hand against `llvm-nm`.

**And here is the constraint that decides the whole design, which I found by trying to break my own
result.** Change `#[inline(never)]` to `#[inline]` on the operation and re-dump:

```
symbols containing sum4 with #[inline]:  0
all symbols:  _call_a  _call_b  _call_c  _call_d  ltmp0
```

The channel closes completely. So:

> **The intent is legible to a build layer exactly at the granularity where the operation survives as
> a function, and nowhere else.** Inlining is what makes arvo fast and inlining is what destroys the
> channel. They are the same mechanism seen from two sides.

That is the honest statement of the boundary, and it is much more precise than "types are erased". It
also immediately explains why file 16's compilation-unit objection was right: after inlining, one
function contains operations from many compositions, so any per-function lever grants liberties to
compositions that declined them. The objection is not about compilation units. It is about **any**
granularity coarser than the operation, and inlining coarsens it.

---

## 6. The contract, designed

`16c` asks for a design rather than an observation: what a downstream target reads, what it can
determine, what it does with it, and what arvo must guarantee. Here it is, concretely enough to build,
in four parts. The first three are measured. The fourth is the part I would build first and it needs
nothing from LLVM at all.

### 6.1 What arvo guarantees, which is three declarative things and no build machinery

arvo grows no build harness. `16c` is explicit that a substrate forced to do so has taken on something
painful to maintain and inconvenient to adopt, and nothing here asks it to. arvo's whole obligation:

1. **Axis marker names are public interface.** The symbol carries `6Strict`, not a hash, so renaming a
   marker is a breaking change to anything downstream keying on it. This is a real cost and it should
   be stated where the markers are declared, not discovered later by whoever renames one.
2. **Residue operations are not inlined, and are named.** For the small set of operations whose licence
   is not source-expressible (section 2's `Contract`, section 1's lane parallelism, and nothing else
   that I found), arvo emits `#[inline(never)]` bodies whose entire content is the operation. Every
   other operation stays inlined and stays invisible, which is correct, because nothing downstream
   needs to see it.
3. **The generic parameter order of the composition type is stable**, because position in the symbol is
   positional. This is already true of any public generic type and costs nothing extra.

Note what is not on the list. No section, no manifest, no macro at the consumer's declaration site, no
build script, no feature flag, no attribute. arvo's side of this contract is three sentences of
documentation and one attribute on a handful of functions.

### 6.2 What the build layer reads, and how, given monomorphisation has already happened

It reads the symbol table of the compiled objects, parses the v0 names, and recovers one record per
instantiation: the composition's const arguments by value and its marker types by name.
`20_probes/03` does this in sixty lines. Three properties of this channel are worth stating because
they are what make it the right one:

- **It is exact, not approximate.** One record per monomorphisation actually emitted. Not per
  composition declared, not per composition mentioned in a doc, not per type alias. What ran.
- **It is pull-shaped and requires nothing at the push end.** arvo does not register anything. The
  linker's own bookkeeping is the manifest.
- **It degrades to nothing.** With no build layer present, there is no cost, no section, no dead
  symbol, and no semantic difference. That is the property I would insist on hardest, because a
  contract whose absence changes behaviour is not a boundary, it is a dependency.

The alternative I considered and would not take: a pushed manifest, one `#[used] #[link_section]`
record per composition, in the style of `linkme` or `inventory`. It has exactly one advantage, that it
survives inlining because the record is emitted at the declaration site rather than the call site. It
has two disadvantages that outweigh it. It records what a consumer *declared* rather than what was
*instantiated*, which is strictly worse information and silently misses every composition reached
through generic code. And it cannot be written for a generic function at all, because Rust forbids an
item inside a generic function from naming the function's parameters, so it would have to be emitted by
a macro at the consumer's declaration site, which taxes every consumer for a build layer most of them
do not run. I would keep it in the ledger as the mechanism to reach for if the symbol channel ever
closes, and not build it.

The build layer should set `-C symbol-mangling-version=v0` explicitly rather than rely on the default I
measured, for the same reason one pins a toolchain.

### 6.3 What it does with it: a per-function rewrite, measured end to end

This is the part I want most carefully read, because it closes the residue **portably**, which is
better than file 17's per-architecture answer, and because I did not believe it until I ran it.

`20_probes/04` takes the float reduction that file 17 measured as unvectorisable, emits rustc's IR
before the optimisation pipeline runs, changes one word on one instruction, runs the ordinary `-O3`
pipeline, and reads the result:

```
=== 1. the fadd rustc emits for the reduction, before any LLVM pass ===
    %3 = fadd double %2, %_6

=== 2. same IR, same pipeline, one flag on one instruction ===
  as-emitted                 vec-fadd=0   scalar-fadd=9    vec-width-in-IR=none
  plus-reassoc               vec-fadd=7   scalar-fadd=1    vec-width-in-IR=<2 x double>
  plus-fast                  vec-fadd=7   scalar-fadd=1    vec-width-in-IR=<2 x double>
```

Zero vector additions become seven, `<2 x double>`, from adding `reassoc` to a single `fadd`. No
architecture-specific code, no intrinsic, no `core::arch`, nothing arvo has to write once per target.
The same rewrite works for whatever target the backend is aimed at, which is the whole point.

The obvious objection is that the un-inlined shim now costs a call, and this is where a pass has to do
three things rather than one. Part 3 of the same probe performs exactly the edits a `FunctionPass`
makes (`F.removeFnAttr(NoInline)`, `F.addFnAttr(AlwaysInline)`, and the same removal on the call site,
which rustc also marks and which cost me two failed attempts to notice):

```
=== 3. cost of the un-inlined shim, after the pass marks it alwaysinline ===
  calls to the shim left in @caller: 0
  vector fadd inside @caller:        7
```

Zero calls, seven vector additions, inlined into the caller. **The un-inlined shim costs nothing once
the pass has run.** So the tension in section 5 between inlining for speed and not inlining for
observability is not a tension after all, provided the un-inlining is undone by the same pass that
consumes the observation. That is the ordering: rustc emits calls, the pass reads symbols and rewrites
flags and clears the inline barrier, then the inliner and vectoriser run normally.

Two details a builder needs and one bill.

The pass must be registered at a pipeline extension point that runs *before* the vectoriser
(`PipelineStartEP` or equivalent), because the vectoriser is what consumes the flag. My first attempt
ran `llc` alone on already-optimised IR and measured zero improvement, which was the pass running after
the only consumer of its output.

The mechanism is `-Z llvm-plugins`, which exists on the pinned nightly (I checked `-Z help`) and takes
a shared library.

**The bill, stated plainly because it is the kind that gets discovered rather than budgeted.** An
out-of-tree LLVM pass plugin must be built against rustc's exact LLVM, which is 22.1.6 on this pin
(`rustc -vV`). Every toolchain bump is a potential rebuild and a potential source break, `-Z
llvm-plugins` is a nightly-only unstable flag that is itself unvetted against
`unstable-features.md`'s procedure, and the pass is C++ against an API with no stability promise. This
is a real recurring maintenance cost. It lands entirely on `hilavitkutin-build`, which is where `16b`
says the vehicle for custom flags and passes belongs, and arvo neither builds nor links nor knows about
it. I would still want it written down next to the mechanism, because a contract whose cost is
invisible gets adopted and then resented.

### 6.4 The half I would build first, which needs no LLVM coupling at all

Everything above is the lowering half. There is a checking half, it is cheaper, it is more valuable,
and I only saw it because file 19 left it open.

File 19 found that the over-claiming direction of a fidelity grant cannot be closed inside arvo:

> The over-claiming half is not closed by this move and I do not think it can be closed by any
> mechanism internal to arvo: it is a promise with nothing behind it until the day something reads it,
> which per `16c`'s own downstream-contract obligation is exactly the moment it needs to be true.
> (`19_ringer...md` section 2a)

The build layer is that day, and it does not need a pass to be it. Given the symbol table and
`--emit=llvm-ir`, a reader can check the declaration against the generated code offline:

- **Over-claim.** A composition's symbol says `Relaxed`, so the licence claims liberties. If no
  instruction in that function carries the corresponding flag and no fused or regrouped form appears,
  the grant is empty. It is not wrong, but it is a promise nothing behind, and file 19 asked for
  exactly this to be made visible rather than merely unfalse.
- **Under-claim, which is the one that is actually unsound.** A composition's symbol says `Strict`,
  and the function contains a fused multiply-add, a flagged instruction, or a regrouped reduction. The
  checked text and the executed text have diverged, and this is `19_probes/01_liberties_disconnected_from_body.rs`'s
  exact finding (a body that regroups under a grade whose liberty array says `false`) caught **after**
  monomorphisation, where nothing internal to arvo can see it.

This is a post-monomorphisation verifier for a property the type system provably cannot check, it
needs no plugin, no LLVM version coupling, and no unstable flag. It is a program that reads two text
formats. And it converts the fidelity licence from the unfalsifiable permission `17b` identified
("nothing in the type system can see it, because no value has the wrong type") into a checked fact.

If I could only have one half of this contract, I would take this one, and I would take it even if the
lowering half is never built. It also generalises past fidelity with no extra work: the same reader
checks that a `Bitpacked` composition's loads are actually the packed sequence, that a `Minimum`
stored width did not silently widen, and that a `Hot` composition contains no widening at all. Those
are `Lowering` claims, which nothing anywhere currently checks, and which the type system cannot check
because they are facts about generated code.

---

## 7. The shape that avoids the coupling, and the fork the spec has not named

`16c` says an answer that closes one of these boundaries is worth more than any number of observations
that it exists. There is one, it is not mine originally (it is what every serious numerics library
does), and the design's own vocabulary is one instance short of being able to say it.

**The fork.** A fidelity marker can mean two incompatible things and the spec does not say which.

- **Fidelity as an envelope.** `Relaxed` means "any answer within these liberties is acceptable to me".
  The body is unspecified, the result varies with target and toolchain, and the build layer is the
  natural vehicle because the type has already agreed not to care. This is what C's `-ffast-math`
  means. It is what file 12, 15 and 16 were all reasoning about.
- **Fidelity as a choice of function.** `Relaxed` names a specific, deterministic, differently-written
  algorithm: a four-way blocked reduction, a fused form, a reciprocal multiply. The answer is exactly
  reproducible, testable, and identical on every target. No build layer is needed, ever, because arvo
  wrote the body. This is what `17_probes/08` actually implements, and what file 17 then argued
  dissolves the need for the first reading.

File 17 conflated them, and the conflation is why its dissolution reads stronger than it is: it built
the second and concluded the first was unnecessary. They are different products with different
downstream-visible properties, and reproducibility is the one that decides.

**And `13c`'s own test decides it, which is the argument I would put in front of op.** The standard is
that MATLAB, IEEE 754 and SystemC are a test rather than an inspiration. All three are
function-shaped. IEEE 754 specifies every operation's exact result and names `fusedMultiplyAdd` as an
operation with one answer. SystemC exists to model hardware bit-exactly, and an envelope would make it
useless for the one thing it is for. MATLAB is deterministic. **Not one of the systems the design is
tested against is envelope-shaped.** C's fast-math is, and C is not on the list.

So my reading, held as one reading and not a ruling: the design's primary fidelity axis should be
function-shaped, `Relaxed` should name an algorithm rather than a permission, and on that reading
**the coupling disappears entirely**. arvo owes a build layer nothing for fidelity, not because the
liberties turned out to be source-expressible, but because a function-shaped licence never needed a
backend's cooperation in the first place. That is a stronger and more defensible version of file 17's
conclusion than file 17's own argument for it, and it survives everything in sections 2 and 3 that
broke the original argument.

The price is real and I will not soften it. Function-shaped fidelity means arvo carries the kernels:
blocked reductions in portable source, and per-architecture vector bodies through `core::arch` where
lanes matter, one per architecture, forever, including the saturating integer case in section 3 where
no other route exists. `arvo-always-optimal-internals.md` already asks for exactly this by default as
its Kind 1, so the design has already accepted the bill in principle. Envelope-shaped fidelity buys the
compiler's entire optimisation arsenal across every target for the cost of a build layer, and pays for
it in reproducibility.

**What I would actually propose, which is that the axis has more than two instances.** The fork is only
forced if fidelity is a two-point axis. Make it an axis with instances and both readings fit, and every
system on `13c`'s test list becomes expressible:

- `Exact`, the strict per-operation IEEE answer. SystemC, MATLAB's default, IEEE 754.
- `Fused`, permitting the single-rounded fused form where the numeral's target has it, deterministic,
  an operation and not a permission, per section 2.
- `Blocked<N>`, a named regrouping with `N` the block count: deterministic, reproducible, exactly
  file 17's measured source-level regrouping, and the instance that carries the instruction-level
  parallelism McSherry priced at 2x.
- `Envelope<L>`, liberties as a set, the only instance that is a permission, the only one a build layer
  may act on, and the only one that gives up reproducibility. Consumers who want C's semantics can have
  them and can see in the type that they gave something up.

The first three need no build layer at all. The fourth is where section 6's contract applies, and
section 4's rule then reads cleanly: the build layer may act on a `Policy` axis exactly when that axis'
instance is `Envelope`, because that instance is the one that declared an envelope to act inside.

I hold this as a proposal and not a ruling. The reading I am carrying against it is that four instances
where the design has been trying to hold two is a real cost in the axis table, in the preset matrix, and
in every error message that renders a composition, and that a design which ships `Exact` and
`Envelope<L>` only, with `Fused` and `Blocked<N>` as operations a consumer reaches for explicitly rather
than as fidelity instances, gets most of the expressiveness for two instances instead of four. I can
argue that one nearly as well. What I would not accept is the two-point axis with the fork unnamed,
because that is the state that produced four members disagreeing without noticing they were describing
different products.

---

## 8. File 17's prohibition, tested rather than inherited

`17b` asked for this specifically. The prohibition:

> Any mechanism that selects among arvo's implementation variants *after* arvo's own type checking has
> run reopens Thread C's fourth-pass gap at the build layer: the checked text and the executed text
> stop being the same text, and every guarantee the witness machinery establishes is void.
> (`17_orchard...md` section 8)

**It is right about what it names and it is stated too broadly to be the rule.** As written it forbids
the section 6.3 mechanism, which does not select a variant: it applies a transformation to the body
arvo wrote, under a licence the type declared, with the checked text and the executed text remaining
the same text. Only the instruction flags differ. If the prohibition covers that, it also covers the
optimiser, which rewrites arvo's checked text constantly and which nobody proposes to forbid.

The distinction the prohibition is reaching for is not before-and-after. It is **substitution against
transformation**, and it has a mechanical form:

> A build layer may apply a transformation `T` to a monomorphised body only if, for every input, the
> result of `T(body)` lies within the set of behaviours the composition's own declared axes permit.
> Substituting a different body is never such a transformation, because the declaration says nothing
> about a body arvo did not write.

That is checkable rather than judged, if a licence is defined as a set of IR flags on a set of opcodes,
which section 7's `Envelope<L>` instance would be. Then the pass is legal by construction: it sets
flags in `L` and nothing else, and section 6.4's verifier confirms after the fact that it set nothing
else. Variant substitution fails immediately because it is not flag-setting. Granting a liberty the
type declined fails because the flag is not in `L`.

Stated that way it composes with section 4's contract rule into one criterion I would put in the spec,
replacing both:

> A build layer may read every axis of every instantiation. It may act on `Lowering` axes freely,
> because by their own definition acting there changes no answer. It may act on a `Policy` axis only by
> a transformation whose every result lies inside the envelope that axis declared. It may never act on
> `Numeral`, and it may never substitute a body.

**And one thing I would add that file 17 did not, which is the failure mode I have actually seen
ship.** The dangerous case is not a build layer that grants too much. It is a build layer that acts on
*some* translation units and not others, so the same composition computes different answers in the same
program depending on which crate it was instantiated in. Monomorphisation makes this easy to hit: a
generic instantiated in two crates produces two symbols, which a pass keyed on symbols will see as two
functions, and if the plugin is only loaded for one crate's compilation they diverge. The linker then
picks one for both call sites, or does not, and either way something is wrong that no test in either
crate can see. So the contract owes one more guarantee, on the build layer's side rather than arvo's:
**the rewrite is all-or-nothing across the whole link unit**, and section 6.4's verifier should check
exactly that, since it is the one check that catches it.

---

## 9. What I would flag for the next member, unresolved

**Section 0's finding about `power_iteration` is the largest hole and I did not design the fix.** A
fidelity licence on `Number<N, S>` does not reach an algorithm written against `F: Add + Mul + Sqrt +
Recip`, because at the point the liberty is taken there is no `S`. The shape I would try, and did not
build, is that liberties become *bounds*: an algorithm that wants the reciprocal multiply demands a
trait that only compositions whose `S` permits it implement, so the where-clause is the licence check
and the composition's `S` decides whether it is satisfiable. That turns a permission into a data-shaped
question, which is exactly what `17b` says the licence side lacks and what file 19's grants machinery
is already built to answer. It should be compiled before anyone trusts it, and this dive's record is
that every pass at this question has been broken by whoever compiled the previous one.

**Section 4's reclassification of fidelity as `Policy` is reasoning, not measurement, and it changes
what op's `16b` framing licenses.** It deserves a second independent read from the canon and the
spec's own text before anything is built on it. I have given the counter-reading in the same section
and I do not think it is weak.

**Section 3's saturating-integer residue is unpriced.** It is a second residue, no IR flag exists for
it, source regrouping does not recover it, and the only route is per-architecture vector bodies inside
arvo. Nobody has counted how many operations and how many architectures that is, and it lands on arvo
rather than on a build layer, which makes it the more expensive of the two.

**I did not test the multi-limb float case, because there is not one.** arvo's floats are hardware
`f32`/`f64` only, per the spec's own section 5.1. If the general `Stored<BITS, U>` axis ever gets a
real consumer, every measurement in section 1 has to be redone, because software floating point is
arvo's own code all the way down and every liberty becomes source-expressible by construction.

**The `-Z llvm-plugins` flag section 6.3 depends on is unvetted** against `unstable-features.md`'s
procedure. It is not on the allowed, watch, forbidden or vetted-and-not-needed tables. Somebody should
run the vetting before the mechanism is designed around, and I note that the flag is on the build
layer's side rather than in arvo's source, which may or may not put it outside that document's audit
scope. That is a question for whoever owns the rule, not for me.

**Section 6.4's verifier is the thing I would build first and I built only its reader.**
`20_probes/03` recovers the intent. The half I did not write is the IR side: reading `--emit=llvm-ir`
output and matching flagged instructions against declared liberties. It is a day of work and it would
close file 19's open half, which is the highest-value unclosed item I encountered in the whole
directory.

**And I have not read `arvo-num-systems` or the earlier files 01 through 10**, which file 17 also
flagged for itself. My sections 4 and 7 make claims about how the axis table should grow, and the
quantisation structure in section 3.3 of the spec is the closest existing precedent for an axis with
internal structure. Somebody who has read that section properly should check whether `Envelope<L>` wants
to be shaped like `Quantisation`'s in-range and out-of-range halves rather than like a flag set.
