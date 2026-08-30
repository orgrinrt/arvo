# 23: The smallest thing that checks

**Reviewer:** Fabrice Bellard (small complete systems lens: what is the least code that genuinely
catches something, what does it cost to run and to keep working, and what can it never catch).

**What I read.** The brief's five files first, in the order given:
`16b_op_design_the_shape_not_the_code.md`, `16c_op_the_downstream_contract.md`,
`16d_op_the_spirit_outranks_all.md`, `17b_op_checkpoint_six.md`,
`13c_op_the_standard_and_the_mode.md`. Then `11_current_shape_draft.md`, in full for sections 3 and 5
and by heading elsewhere. Then, of the dive, the three files my question descends from in full or near
it: `19_ringer_the_witness_and_its_upkeep.md` (sections 0 through 2a, 8, 9),
`20_wingo_the_build_layer_contract.md` (sections 5 through 7, which is where the piece I was told to
build is specified), and `17_orchard_are_these_all_grades.md` by the passages the other two quote. I
`ls`'d the panel directory and every probe directory, and read `20_probes/02_run.sh` and
`20_probes/03_the_build_layer_reader.py` line by line, because my dispatch is to build the thing that
file proposed and I wanted to see exactly what had already been built rather than take a summary of
it. On arvo source I read two files I found by grep and did not expect to exist,
`arvo-storage/src/layout_assertions.rs` and `arvo-strategy/src/axes.rs`, and section 5 below is why
that matters more than it sounds.

**What I built and measured, as distinct from what I reasoned about.** Seven probes in `23_probes/`,
all on `rustc +nightly-2026-05-28` (1.98.0-nightly, 57d06900f), all reproducible from the four
`*_run.sh` scripts plus `08_is_the_input_even_reproducible.sh`. Two model crates, two checkers
totalling 68 and 61 lines of code, and several runs against the real arvo tree. Every number in
sections 1 through 5 came out of a program. I also ran
`cargo test --workspace` in `arvo/mock` and got 654 passed, 0 failed, 9 ignored, reproducing every
prior member's figure, and confirmed independently that the suite contains no test comparing a value
to itself and no test of the apparatus this dive is designing. Sections 6 through 9 are argument, and
where I hold more than one reading I say so and leave the choice where it belongs.

**One thing up front, because it changes what the rest of this file is about.** I was asked to build a
post-monomorphisation verifier or establish it should not exist. I built it, it works, and it is 68
lines. I also built two things smaller than it, one of which is a single assertion in the ordinary
test suite with no tool anywhere, and that one catches two of the three defects I planted. So the
answer is not one of the two I was offered. The tool is real and should exist eventually; it should
not be the piece built first, and the piece built first is one line.

---

## 0. The premise I checked before building on it, and where it is wrong

File 20's measurement is correct and its conclusion from it is drawn about the wrong artifact.

The measurement: a v0 symbol carries every const argument by value and every marker type by name, and
`#[inline]` closes the channel entirely (`20_wingo...md:326-373`). Both reproduce. What that file
then states as the constraint deciding the whole design:

> **The intent is legible to a build layer exactly at the granularity where the operation survives as
> a function, and nowhere else.** (`20_wingo...md:371-373`)

That is true of one artifact. `23_probes/01_where_the_channel_is_complete.rs` declares twelve
compositions and two operations each, twenty-four monomorphisations, with **no inline attribute
anywhere**, which is the case arvo will actually be in since neither attribute is the default on a
generic function. Counting what is nameable:

| artifact | `add` recovered | `reduce` recovered |
|---|---|---|
| optimised object file, what ships | 0 / 12 | 0 / 12 |
| optimised object file, fat LTO | 0 / 12 | 0 / 12 |
| optimised LLVM IR | 0 / 12 | 0 / 12 |
| IR with `-Cno-prepopulate-passes` | 0 / 12 | 12 / 12 |
| IR with `-Cno-prepopulate-passes -Zinline-mir=no` | **12 / 12** | **12 / 12** |

Two things fall out, and the second one I did not expect.

**In the artifact that ships, the channel is not narrow. It is empty.** Not one of twenty-four
compositions is nameable, with or without LTO. File 20's probe kept its channel open with
`#[inline(never)]` on the operation, which is a fine way to demonstrate that the symbol carries the
information, and it is not the condition arvo's ordinary code is in. A reader pointed at a release
build of arvo finds nothing to read.

**There are two inliners and only one of them is LLVM's.** `-Cno-prepopulate-passes` recovers the
large operation and not the small one, because rustc's own MIR inliner has already eaten the small one
before LLVM is handed anything. Adding `-Zinline-mir=no` recovers everything. So the tension file 20
names between inlining for speed and not inlining for observability is real, and it is not resolved
where that file resolves it, because the first inliner runs inside rustc where no LLVM pass plugin can
reach.

Which turns the constraint into something better. The intent is legible in **whatever build you ask
for it in**, and asking costs two flags on a build you were not going to ship anyway.

---

## 1. The check build, measured on the real tree

The input to everything below is the ordinary build plus `-Cno-prepopulate-passes -Zinline-mir=no
--emit=llvm-ir`. On arvo's actual workspace, cold, via `RUSTFLAGS` and an out-of-tree
`CARGO_TARGET_DIR` (`23_probes/06_the_real_tree_and_its_coverage.sh`):

| | ordinary release | check build |
|---|---|---|
| release library, wall clock | 14.86 s | 12.67 s |
| release library, IR emitted | none | 95.3 MB |
| debug all targets, wall clock | not measured | 14.91 s |
| debug all targets, IR emitted | none | 342 MB |

The check build is **faster** than the release build, because no optimisation pipeline runs. It costs
disk and nothing else. It is not the artifact that ships and never becomes one.

Debug rather than release for the all-targets figure, and not by preference. `cargo build --release
--all-targets` does not compile in this tree, which I found by accident and which section 5.5 covers,
so the debug profile is the only one in which the check build is a whole-tree check build. That is not
a compromise: a check build is not a shipping build, and nothing in what the reader does depends on
the optimiser having run.

That disposes of the cost question so completely that the interesting questions are all about what the
thing reads and what it can conclude, which is the rest of this file.

---

## 2. The verifier, built

`23_probes/03_the_whole_verifier.py`. 110 lines with the docstring, **68 lines of code**, of which
three are the rules table and the rest is a v0 name decoder and two loops. Runs in 20 ms over the
model and **1.5 s over 95 MB of real arvo IR**.

`23_probes/02_declared_against_generated.rs` is what it is pointed at: two axes, `Fidelity`
(`Strict` / `Relaxed`) and `Layout` (`Dense` / `Bitpacked`), two operations, and three cfg arms each
planting one defect. No cfg arm changes a type, a bound, or a declared axis. All four builds compile
clean, which is the hole, stated as a program rather than as prose.

Measured (`23_probes/02_run.sh`):

| build | what is wrong with it | rules verifier | rule-free check |
|---|---|---|---|
| clean | nothing | 0 under, 0 over | 0 inert axes |
| `--cfg underclaim` | `dot` fuses whatever Policy says | **4 under**, 0 over | **1 inert** |
| `--cfg overclaim` | `load` ignores Layout entirely | 0 under, **4 over** | **1 inert** |
| `--cfg swapped` | the liberty goes to `Strict` | **4 under, 4 over** | 0 inert |

The `underclaim` arm is exactly `19_probes/01_liberties_disconnected_from_body.rs`'s finding, the one
Ringer stated could not be closed inside arvo:

> The over-claiming half is not closed by this move and I do not think it can be closed by any
> mechanism internal to arvo (`19_ringer...md` section 2a)

It is closed here, after monomorphisation, in 68 lines, with no plugin, no LLVM coupling and no
unstable flag beyond `-Z inline-mir`. So the mechanism file 20 proposed and nobody built does work,
and I want that said plainly before I spend the rest of this file arguing about whether it is the
piece to build.

Three things I learned building it that are not in file 20's design, and each is a correction to it.

### 2.1 The marker carries its defining crate, which is better than the promise file 20 asks for

File 20's contract asks arvo to promise that "axis marker names are public interface", so renaming a
marker is a breaking change downstream (`20_wingo...md:395-397`). That promise is real and it is the
weaker half of what is available. Measured in `23_probes/04_the_marker_carries_its_crate.sh`, with
markers in one crate, a generic operation in a second and a consumer in a third:

```
_RINvCsaR5ijk9BJoP_12arvo_numeric4loadKtd_NtCs1IfMbHeZ33w_13arvo_lowering5DenseECsjzug6YXchPc_4user
```

`NtCs<hash>_13arvo_lowering5Dense`. **The crate that defined the marker is in the symbol.** So a
verifier scopes itself structurally, by "this monomorphisation has a generic argument defined in an
axis crate", and never carries a hand-written list of marker names.

The difference is not stylistic and I measured it. Running the verifier over the real tree with a name
list produces sixteen findings, every one of them inside `syn`, on types called `Attribute` and
`LitStr` and `Punctuated`, because a shift with no `Bitpacked` in scope is a shift in a proc-macro
dependency. With crate scoping, that noise is zero, by construction, forever.

And it answers the brief's question about what it costs to keep true when an axis is added. Adding an
axis **instance** costs nothing at all: the new marker lives in the same crate and is in scope the
moment it exists. Adding an axis **rule**, meaning a statement of what instruction witnesses the new
liberty, is hand work and is the only hand work.

This puts one requirement on the design, and it happens to be one the crate table in
`11_current_shape_draft.md:374-381` already satisfies: the axis markers live in their own crates
(`arvo-numeral`, `arvo-policy`, `arvo-lowering`) and not mixed in with the operations. Keep that and
the verifier needs no vocabulary.

### 2.2 The two directions need different granularity, and getting it wrong is total noise

My first version checked both directions per monomorphised function. The under-claim direction is
correct that way. The over-claim direction produced a **100 percent false-positive rate on the clean
build**: twelve reports, all of the form "declares `Bitpacked`, no packed load in body", every one of
them about `dot`, which has no reason to contain a load of any kind.

An operation is not obliged to exercise every axis of the composition it belongs to. So the over-claim
question is not askable of an operation at all. It is askable of a **composition**, as the union over
every operation instantiated for it. Grouped that way, the clean build reports nothing and the
`overclaim` arm reports exactly the four `Bitpacked` compositions.

The same correction applies one level down. Two rules witnessing the same marker have to be OR'd, or
the second one reports every composition that satisfied the first. Before that fix the clean build
carried four permanent false reports, from a rule for fast-math flags that can never fire, for the
reason in 5.2 below.

Neither of these is a bug in the idea. They are the difference between the idea and a program, and
they are why I would not trust this mechanism stated as prose by anyone, including me.

### 2.3 The flags that keep the composition visible push the liberty one call outside it

`f64::mul_add` is a `std` function. Under `-Zinline-mir=no` it stays a call, so the `Relaxed` body
contains `call @_RNvMNtCs..._3std3f64d7mul_add` and the `llvm.fma.f64` is one level down in a function
that has no axes on it. The verifier at call depth 0 sees a `Relaxed` function with no fused
instruction in it.

Two ways out, and I took both. The rule can match the callee **name**, which is present in the body
text and is arguably the better observation anyway, since the check is then "which operation was
called" rather than "which instruction was emitted". And the reader can union in callee bodies to a
bounded depth, which is eight lines and made no difference on this model.

Worth stating as a general shape: the check build is not the shipping build, so a rule written about
the shipping build's instruction mix will not fire. The rules have to be written about what rustc
emits before anything runs, and that is a different vocabulary than a person reading disassembly
expects.

---

## 3. The cruder check, which has no rules and no semantics at all

`23_probes/05_the_axis_that_generated_nothing.py`, 61 lines of code, knows nothing about any axis:

> for each axis, does varying that axis alone, with everything else fixed, ever change one single
> instruction anywhere in the program

An axis that never changes anything is a declaration with no consequence. That is the strongest form
of the over-claim Ringer asked to be made visible, it is exactly the defect file 16 found by hand when
it noticed the two float types compiling identically, and finding it needs no knowledge of what any
axis means.

Measured, it catches two of the three planted defects, with no rules, and reports nothing on the clean
build. On the clean model it prints the truth and only the truth: Policy changes `dot`, Layout changes
`load`, neither changes the other.

It misses `--cfg swapped`, and that miss is the whole boundary between the two tools. In `swapped` the
liberty goes to the wrong instance, so the axis still changes generated code and there is nothing
inert to see. **A check with no semantics catches an axis that does nothing. Only a rule about what a
liberty is catches an axis that does the wrong thing.**

The maintenance asymmetry is the reason to care, and it goes directly to the brief's question. When
the design adds an axis, the rule-free check costs nothing, because it derives what to compare from
the symbols themselves. The rules-based verifier costs one row per axis instance, and a row nobody
writes is a check nobody gets, silently, which is the failure mode where a verification apparatus
quietly stops covering the thing it is cited as covering.

---

## 4. The version with no tool at all

This is the part I would put in front of op first, because it is smaller than everything above by
three orders of magnitude and it was sitting there the whole time.

`23_probes/07_no_tool_at_all.rs`. Two monomorphisations of one operation, and one assertion:

```rust
assert_ne!(dot::<Strict> as usize, dot::<Relaxed> as usize);
```

If two monomorphisations compile to identical code, the compiler folds them to one address. So the
inequality of two function pointers **is** the statement that the axis generated something. It is an
ordinary assertion, in the ordinary suite, on the ordinary build. No IR, no flags, no scanner, no
build layer, no dependency, nothing to keep working.

Measured across optimisation levels (`23_probes/07_run.sh`), with a `--cfg inert` arm making the axis
do nothing:

| flags | live axis: one address? | inert axis: one address? |
|---|---|---|
| `-Copt-level=0` | false | false |
| `-Copt-level=1` | false | false |
| `-Copt-level=2` | false | **true** |
| `-Copt-level=3` | false | **true** |
| `-Copt-level=3 -Clto=fat` | false | **true** |
| `-Copt-level=s` | false | **true** |

Correct from `-Copt-level=2` upward, including under fat LTO and under size optimisation. Below it the
check is wrong in the safe direction only, reading an inert axis as live, so the assertion is gated to
a release-profile test rather than trusted everywhere. It never reports a live axis as inert at any
level I measured.

I also measured the variant that looks cruder still, reading the first 128 bytes of each function's
machine code and comparing them, and it is **unusable in both directions**: at level 0 it calls a live
axis inert, and at level 1 it calls an inert axis live. The address is the better observation
precisely because the compiler computed it rather than the test.

So the honest floor of this whole question is one assertion per claim, and the cost of adding an axis
is one more assertion, written by whoever adds the axis, at the moment they add it. That is exactly
the discipline `catalogue-edge-cases-as-tests.md` already asks for, applied to a class of claim nobody
had noticed was assertable.

What it cannot do is everything in section 3's `swapped` column and everything in section 2. It is the
cheap half of the cheap half. It is also the only version that survives a year of nobody maintaining
it.

---

## 5. What already ships, which changes what should be built

Two files I found by grep, expecting nothing.

`arvo-storage/src/layout_assertions.rs` is 407 lines and contains **73 compile-time layout
assertions**, plus a block pinning each preset's axis table. From
`arvo-storage/src/layout_assertions.rs:231`:

```rust
assert!(<<Hot as HasAxes>::Layout as StorageLayout>::DISCRIMINANT == Dense::DISCRIMINANT);
```

and at `:243` the same for `Cold` against `Bitpacked`. `arvo-strategy/src/axes.rs:133` declares the
`Bitpacked` marker and `:183` the preset's `type Layout = Bitpacked`.

Three consequences, and the middle one is the sharpest thing in this file.

**Keep this shape.** `16d` says rewrite cost is the tiebreaker, and here there is a working, honest,
compile-time mechanism for exactly the class of Lowering claim that is about layout. The new design's
`StoredWidth` and `Layout` axes should extend this module's pattern rather than acquire a new
mechanism. Whatever else gets built, this stays.

**And it is, precisely, the shape the brief calls the sharper hole.** `assert!(Hot::Layout ==
Dense)` relates two **declared** things. It says a preset declares the axis instance the design says
it declares. It says nothing whatever about whether any body honours it, which is the disconnection
Ringer named at a different layer. So the shipped tree already contains 73 instances of exactly the
witness shape whose limit this dispatch is about, and they are all correct, all passing, and all
silent about the thing that matters. That is not a criticism of them. It is the clearest available
demonstration that the declaration-to-declaration layer and the declaration-to-body layer are
different layers, and that a suite full of the first tells you nothing about the second.

**The split it suggests is the useful one.** A Lowering claim about **size or alignment** is a fact
about a type, it is checkable with `const { assert!(size_of::<..>() == ..) }`, it costs nothing, and
arvo already does it. A Lowering claim about **generated code shape** (this load is the packed
sequence, this arithmetic never widened) is a fact about a body, no const assertion can see it, and it
is what the sections above are for. Sorting the new design's Lowering axes into those two piles is
worth doing before anyone builds a tool, because the first pile needs no tool and I suspect it is the
larger pile.

### 5.5 One thing in the tree that blocks the recommendation, found by accident

`cargo build --release --all-targets` in `arvo/mock` fails, exit 101, with five instances of:

```
error[E0599]: no associated function or constant named `NAME` found for struct `Hot`
  --> crates/arvo/tests/cross_width.rs:20:44
```

It is not a regression and not carelessness. `arvo-strategy/src/lib.rs:101` gates `Strategy::NAME`
behind `#[cfg(debug_assertions)]`, deliberately, with the reason stated at `:98` ("static strings are
gated out of release builds, zero `.rodata` footprint"), and `cross_width.rs` uses it. The test target
therefore exists in debug and not in release.

Two things follow, and I would not have gone looking for either.

**The suite is silent about it because the suite runs in debug.** `cargo test --workspace` exits 0 at
654 passed, 0 failed, 9 ignored, in a fresh target directory, which I checked precisely because the
number reproduced so cleanly across every member of this dive. It is a true number about the debug
profile and nothing else, and this is exactly the shape where a green count gets cited as though it
covered more than it does.

**And it blocks section 4 specifically.** The address assertion is measured valid from
`-Copt-level=2` upward (section 4's table), which means it has to live in a release-profile test. There
are no release-profile tests in this tree today, because the test targets do not compile there. The
fix is small, either an unconditional `NAME` or the same `cfg` on the assertions that use it, and it is
a prerequisite for the cheapest recommendation in this file rather than a side observation.

---

## 6. What none of this can ever catch

Four ceilings, and one I expected to find that turned out not to exist. The second is the one that
bounds the whole idea and is not fixable by working harder on the tool. The third is the one I got
wrong first, and I have kept the wrong version in.

**Instruction-level rules only work when the instruction is the liberty.** `llvm.fmuladd` is a good
witness for fusion because the instruction is the thing being licensed. `lshr` is a witness for
nothing: it is what division by 64 compiles to, and what every hash and every capacity calculation
compiles to. Measured, my `Bitpacked` rule fired sixteen times on the real tree before crate scoping,
and every hit was in `syn`. Claims whose witness is a **pattern** of instructions rather than an
instruction have no reliable rule of this kind, and I would not write one, because a rule with poor
precision is worse than no rule: it trains whoever reads the output to skim it.

**The coverage is test-suite-shaped, not type-system-shaped, and now measured rather than argued.**
The verifier sees the compositions a build **instantiates**. On the all-targets check build there are
16187 generic monomorphisations, of which 1753 carry a strategy marker (Hot 821, Cold 390, Precise
346, Warm 207). On the release **library** build alone there are 10263 generic monomorphisations
carrying **four**. Every strategy composition the verifier can see exists because a test or a bench
constructed it, which is not a figure of speech about test-suite-shaped coverage. It is the literal
mechanism. Set that against the design's own space. The quantisation axis alone, per
`11_current_shape_draft.md:212-219`, is three direction members over six markers and two range members
over nine, which is 6^3 times 9^2, or 17496 instances, before a single other axis is chosen. A
verifier can say nothing at all about a composition nobody constructed. This is the same coverage
property a test suite has, with the same failure mode, and no
amount of work on the reader changes it. Anyone citing this apparatus as evidence about the design has
to say "for the compositions this build instantiated", every time.

**The set it sees is reproducible, which I nearly reported the opposite of.** Three cold builds of
unchanged source, `-Zinline-mir=no -Cno-prepopulate-passes --emit=llvm-ir`, comparing the full symbol
list with crate-disambiguator hashes normalised away
(`23_probes/08_is_the_input_even_reproducible.sh`):

| | IR files | generic monomorphisations | set difference between runs |
|---|---|---|---|
| release library, three cold runs | 52 each | 10263 each | **0** |
| debug all targets, two cold runs | 173 each | 16187 each | **0** |

Byte for byte the same set. So a scanner over this input **can** gate, and the ceiling I was about to
report does not exist.

I am keeping the wrong version here because of how I got it. My first measurement used `--release
--all-targets` and found 1490 to 1629 strategy-carrying symbols across three runs, with the sets
differing by 675 lines, and I had written the paragraph concluding that codegen-unit partitioning made
gating impossible. The variance is real and the explanation was invented. That build **fails**, exit
101, and three parallel builds racing to a failure stop at different points. This dive's whole record
on this question is that reasoning about generated code goes wrong until somebody compiles it, and
this is my instance of it, found one command after I had written the conclusion down.

**The under-claim direction is nearly unreachable from Rust source today, which cuts both ways.** Safe
stable Rust emits no fast-math flag. The one liberty on the fidelity axis a Rust body can take by
itself is `f64::mul_add`, and `core::intrinsics::fadd_fast` is behind `core_intrinsics`, which
`unstable-features.md`'s forbidden table already bans. So on the shape where arvo writes its own
bodies, the unsound direction has exactly one reachable trigger, and a `grep` for `mul_add` under a
`Strict` bound is most of the check. It is when a **build layer** starts rewriting instructions that
the direction becomes rich, which is the next point.

**The half that checks a build layer cannot read the shipping artifact, and file 20's design has it
reading the wrong thing.** A pass that acts on a licence must run before the vectoriser, which file 20
measured (`20_wingo...md:481-484`). The inliner runs after that. So by the time there is a final
binary, the pass's edits have been merged into callers whose composition sets are unions, and nothing
can reconstruct which edit was licensed by which composition. A post-hoc reader of the shipped
artifact cannot referee the pass at all. It can only referee **arvo's own source**, and for that it
does not need the shipping artifact, because the check build has the same source. That is a real
correction to section 6.4 of file 20 and it is why section 7 below asks the build layer for something.

---

## 7. The downstream contract, designed rather than observed

`16c` requires this of every member. Here it is, for the piece I touched, in the two directions it
names.

### What arvo owes a downstream target

Four things, none of which is a build harness, and three of which arvo either already does or gets for
free.

1. **Axis markers live in axis crates**, separate from the operations. The crate table at
   `11_current_shape_draft.md:374-381` already does this. It is what makes a reader's scoping
   structural instead of a maintained name list, per 2.1, and it is worth writing down as a
   requirement rather than leaving as an accident of packaging.
2. **Marker names, marker crate names and generic parameter order are public interface.** File 20
   already asks for the first and third. The second is new and comes with the crate scoping.
3. **A build reproduces with `-Cno-prepopulate-passes -Zinline-mir=no` and produces IR.** arvo does
   nothing to enable this; it is two flags on a build nobody ships. The obligation is only that arvo
   never depends on optimisation for correctness, which `#![no_std]` and its const-heavy shape already
   imply.
4. **An operation whose axis liveness is claimed has an out-of-line handle**, meaning something can
   take its function pointer. Taking a pointer costs one out-of-line copy in the test binary and
   changes nothing at real call sites, which stay inlined.

Note what is absent, deliberately, matching `16c`'s own warning. No section, no manifest, no macro at
a consumer's declaration site, no build script, no feature flag, no attribute, and nothing a consumer
who runs no build layer ever pays for or hears about.

### What arvo needs back, which is the thing it cannot express

This is the part I want read, because it is a requirement nobody has stated and it is unrecoverable if
it is missed.

**A build layer that acts on a declared licence must emit a receipt of what it acted on, keyed by
symbol, at the moment it acts.** Not afterwards. The reason is in section 6: the pass runs before the
inliner by construction, and after the inliner the mapping from edited instruction to licensing
composition does not exist in any artifact and cannot be reconstructed by anyone, including the pass
itself.

The receipt is one line per rewrite: the mangled symbol of the function it edited, the transformation
it applied, and the axis instance it read as the licence. A text file. It is checkable, offline, by
the same reader in `23_probes/03`, against the same check build, by two joins: every symbol in the
receipt must decode to a composition declaring that licence, and no function outside the receipt may
contain a flag only a licence could have put there.

That closes the loop `16c` asks about. arvo declares, the build layer discovers and lowers, and the
receipt is what makes the lowering checkable rather than trusted. Without it, a licence is exactly the
unfalsifiable permission `17b` identified, moved one layer down and made harder to see.

Whether the receipt is cheap depends on a fork this dive has not closed, and I will not close it here:
on file 20's own reading in its section 7, where fidelity is **function-shaped** and `Relaxed` names a
specific deterministic algorithm arvo wrote, there is no pass, no rewriting, no receipt and no build
layer to referee. On the envelope-shaped reading, all of it is mandatory. The receipt requirement is
what the envelope reading costs, and it belongs on the ledger next to the LLVM version coupling file
20 already priced.

---

## 8. What I would do, held as readings rather than a ruling

Three shapes. I hold the first most strongly and the third is the one I would want argued against.

**Reading A, and the one I would act on. Build the one-line version now, the rule-free scan when there
is an axis to point it at, and the rules-based verifier only if a pass ships.** Concretely: an
`assert_ne!` on function addresses per operation-and-axis pair whose liveness the design claims, in
release-profile tests, alongside the 73 layout assertions that already exist and in the same spirit.
Section 5.5 is the prerequisite and has to land first, because there are no release-profile tests in
this tree today. That is section 4, it costs one line per claim, it needs nothing to keep working, and it turns "this
axis is real" from prose into a test. Then `23_probes/05` as a periodic scan rather than a gate, since
it needs no maintenance and answers a question no test asks, which is whether an axis is inert
**everywhere** rather than in the one place a test looked. The 68-line rules verifier stays in the
directory, working, as the thing to reach for on the day a build layer starts rewriting instructions,
because on that day it is the only mechanism that can referee it.

**Reading B, which I do not take but which is coherent. Build none of it.** On file 20's
function-shaped reading of fidelity, arvo writes its own bodies, a body's shape is decided by source
arvo controls and reviews, and the entire under-claim direction reduces to a `grep` for `mul_add`
under a `Strict` bound plus a code review. On that reading the whole apparatus is machinery for
catching a mistake that a reviewer catches, and `19_ringer`'s own argument, that a verification
apparatus nobody maintains is worth less than a smaller one that survives, points at zero rather than
at small. The reason I do not take it is the coverage argument in reverse: a reviewer sees the source
once, the assertion sees it on every build forever, and one line is not a maintenance burden by any
measure I can construct.

**Reading C, which is a claim about what the verifier is for and which I would like someone to break.**
The tool's job is not checking arvo. It is refereeing the second party that edits the code after the
types are gone. Everything about it follows from that: it exists only if a build layer rewrites, it
must read the receipt rather than the binary, its rules are the pass's vocabulary rather than arvo's,
and it belongs in `hilavitkutin-build` next to the pass rather than in arvo. On this reading, file
20's framing of it as "the half I would build first, which needs no LLVM coupling at all"
(`20_wingo...md:499`) is exactly inverted: it needs no LLVM *linkage*, and it is meaningless without an
LLVM pass to check. I hold this at maybe two-thirds confidence, and the third that disagrees is the
observation in section 2 that the `underclaim` arm is a real defect in arvo's own source, with no
build layer anywhere near it, caught by the tool and by nothing else.

**On the spec text itself.** Section 5.4's ledger (`11_current_shape_draft.md:813-868`) sorts every
claim into four bins and its "trusted, named explicitly" bin is where I would put the changes. Two
entries belong there that are not there: that a composition's declared axes are honoured by the bodies
that run under them, which is what this whole dispatch is about and which the ledger does not mention
in any bin; and that any build layer acting on a licence acted only inside it. Adding them is not a
defect report, it is the ledger doing its job, and the bin they land in moves from "trusted" to
"validated per artifact" exactly to the extent that any of section 8's shapes gets built.

---

## 9. For whoever comes after me

Five things, unresolved, ordered by how much I think they matter.

**The over-claim direction may not be worth having at all.** I got it to zero false positives on a
model by grouping per composition and OR-ing the rules, and I do not believe that survives contact
with a real program, where "no operation in this build exercised this liberty" is the normal state of
most compositions most of the time. Someone should point it at a real axis and count, rather than
trust my model. If it is noisy there, the under-claim direction is the whole tool and the code gets
smaller again.

**Somebody should check my address-folding result on a second platform.** Everything in section 4 was
measured on aarch64-apple-darwin. Function merging is an optimiser and linker behaviour, and the table
of optimisation levels is the kind of thing that differs on a different backend. The failure mode is
benign (an inert axis reads as live, so the assertion passes when it should fail) but it is silent,
which is worse than benign.

**The v0 decoder in my probes is a toy and a real one is not.** Regexes cannot decode v0 in general,
because backreferences (`B3_`, `B5_` in a real arvo symbol) mean a name cannot be read without reading
the whole thing, and because `Nt` introduces module path segments as well as type arguments, which is
why my decoder reported `mem|slice|str|vec` as axis markers on the real tree. The real answer is
`rustc-demangle` and a tree walk. That is a maintained crate owned by the Rust project, not a burden
on arvo, but it is a dependency and someone should say so out loud before this is called dependency
free.

**The receipt in section 7 is designed and not built.** I would want it compiled before anyone trusts
it, on this dive's own record about prose in this area, which is that the first two attempts at every
piece of this apparatus had a hole the next member found by compiling.

**And the question I could not get at.** Everything here checks a body against the axes on its own
signature. Nothing checks a body against the axes of the composition it was **called from**, and after
inlining those are different sets. A `Strict` reduction inlined into a `Relaxed` caller sits inside a
function the pass is licensed to rewrite. Whether the licence should be read at the call site or the
definition site is a design question this dive has not asked, my tools cannot answer, and the answer
decides whether a pass acting per function is sound at all.
