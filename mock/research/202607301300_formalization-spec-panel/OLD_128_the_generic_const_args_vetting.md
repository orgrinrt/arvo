# The `generic_const_args` vetting

**Date:** 2026-08-01
**Position:** after `127b_op_checkpoint_twentyseven.md`.
**Procedure:** `.claude/rules/unstable-features.md`, "Vetting procedure for an unknown feature" (lines 57 to 67).
**Canon gate:** passed. The rule's own tier 3 (line 11) says a feature whose status is not settled by the
document gets vetted before it is enabled. `generic_const_args` has no row in any table, so this dispatch is
the thing the rule asks for rather than a deviation from it.

Everything below that concerns compiler behaviour was compiled on the pinned toolchain in a scratch
directory outside the tree. Nothing is cited from a prior file where it could be run instead. Probe sources
are at `/private/tmp/claude-501/-Users-orgrinrt-Dev-clause-dev/dea47dac-5762-46b6-956e-0d22cc5d3832/scratchpad/fallin128/`,
named `p1` through `p14` in the order they appear here.

## The verdict

**WATCH** on the feature itself, in the rule's vocabulary: allowed, sound, carrying named
incomplete-implementation rough edges. It is not forbidden, and the rule's own template for forbidding does
not apply to it.

That verdict covers the feature gate and nothing else. The `-Znext-solver=globally` flag it hard-requires is
a separate exposure that the rule's three tiers do not describe, and the tier it should land in is op's call,
not mine. Section "Where the flag lands" states what I would propose and why I am not treating my own
proposal as settled.

Op's clarification narrows this correctly to the rule's two-part gate, so those two answers come first and
stay separate from everything softer.

## Gate answer one: is it unsound by nature

**No.** Not on the evidence available, and the shape of the feature argues against it.

Tracking issue #151972 carries `A-const-generics`, `C-tracking-issue`, `F-generic_const_args`, `T-compiler`,
`T-lang`, `T-types`. It carries no `I-unsound`. It carries no `S-tracking-perma-unstable` and no
`S-tracking-stalled`, which are the two labels that condemned `auto_traits` and `unboxed_closures` in the
forbidden table (rule lines 75 to 76).

The design property that matters is stated in the issue itself:

> Unlike in GCE, GCA expressions are only allowed as the right-hand side of const items. Uses of these const
> items use "definitional equality" unless they are able to be evaluated by const eval.

That is the specific restriction that separates it from `generic_const_exprs`. GCE's defect is that it
admits arbitrary expressions in type position and then owes the type system a decision procedure for
equality of arbitrary expressions, which is where "fundamentally flawed" and the rustc-dev-guide's "the now
dead" come from. GCA does not take that obligation on. It confines expressions to the right-hand side of
const items and uses opaque definitional equality where const eval cannot reduce. That is a weaker and
checkable claim rather than an unfixable hole.

I also tested the one place a const-generics feature could plausibly damage soundness in this workspace's
specific sense, and it does not.

The rule's section "The forbidden list is verification infrastructure, not only hygiene" (lines 24 to 51)
says the `specialization` and `TypeId` bans exist because they let an implementation observe which
instantiation it is in and behave differently, which is what would break the transfer of a property checked
at a model width to a real width. GCA does not provide that. Coherence still refuses a blanket impl
partitioned by a specific const:

```
error[E0119]: conflicting implementations of trait `Op` for type `S<8>`
10 | impl<const N: u16> Op for S<N> { fn run() -> u32 { 1 } }
   | ------------------------------ first implementation here
11 | impl Op for S<8> { fn run() -> u32 { 2 } }
   | ^^^^^^^^^^^^^^^^ conflicting implementation for `S<8>`
```

(`p6_observe.rs`, compiled with `-Znext-solver=globally`.) An impl cannot observe its width and dispatch
differently. Monomorphisation stays uniform and the model-width transfer argument is untouched by the
feature.

One qualification, stated because it is a genuine consequence and it is not the feature's fault. The thing
the design wants to build with GCA is a width-to-container ladder, and a ladder does mean that width 3 runs
through `u8` and width 47 runs through `u64`. A property checked at one model width then transfers within a
rung, not across the ladder. That is a consequence of any ladder, gate-free or not, and it means model-width
validation owes one model per rung. It is a design obligation rather than a soundness hole, and it would
arrive identically with the gate-free construction in `p12`.

**Unchecked, and marked as such:** I did not audit the open next-solver bug list for soundness-shaped
entries. Reported counts from search were roughly 76 open, characterised as mostly internal compiler errors
and performance problems. I did not verify that characterisation myself, and it bears on the flag rather
than on the feature.

## Gate answer two: is it itself on the stabilisation path

**Yes, and this is the finding that decides the vetting.**

The rule's standing gate, stated at line 132, is that a feature is allowed only if it is not proven unsound
or unstable **and** is itself on the stabilisation path. That clause is what forbade `generic_const_exprs`:
GCE is not itself on the path, `min_generic_const_args` is the path, and GCE is what the path replaces.

`generic_const_args` is on the opposite side of that relation, and the compiler enforces it. Enabling the
gate alone produces:

```
error: `generic_const_args` requires `min_generic_const_args` to be enabled
 --> p8_issue.rs:2:12
  |
2 | #![feature(generic_const_args)]
  |            ^^^^^^^^^^^^^^^^^^
```

(`p8_issue.rs`.) You cannot have GCA without mGCA. It is not an alternative to the allowed feature, it is a
layer on top of it, and rustc refuses to let you separate them. The tracking issue says the same in prose:
the feature is "based on the machinery developed for `min_generic_const_args`".

The upstream ordering matches. The 2026 const generics goal describes mGCA as a prototype to be finished
first, and then:

> We also intend to work on a more "full" extension than the above feature which will have less
> restrictions. We have an idea of what this should look like but it is has yet to be prototyped.

That text is now behind the tree. The prototype exists on the pinned nightly, it has its own
`C-tracking-issue` at #151972, and that issue carries a stabilisation checklist (implementation,
normalization fixes, documentation, style guide, stabilisation PR). Both T-lang and T-types are on it.

So the two-part gate reads: not unsound by nature, and on the stabilisation path as the declared next
increment of a feature the rule already allows. Both halves pass.

## The relationship to `min_generic_const_args`, stated plainly

The brief asked me to get this right because the whole verdict turns on it, so here it is as a direct
comparison against the case the rule uses as its template for forbidding.

`min_specialization` is a **sound subset carved out of** `specialization` precisely because the full feature
cannot be made sound. The min form exists as a rescue. The full form is condemned.

`min_generic_const_args` is a **ground-up rewrite replacing** `generic_const_exprs`. The predecessor is
condemned; the rewrite is the path. This is the relation the rule reasoned about at line 74 and it is
correct.

`generic_const_args` is **the next increment of the rewrite**, not a rival to it and not a rejected
predecessor of it. It requires the min form to be enabled. It is built on the min form's machinery. It is,
in the const-generics effort's own sequencing, what comes after mGCA rather than what mGCA was invented to
escape.

Three features, three different relations, and the naming similarity between `generic_const_exprs` and
`generic_const_args` is the trap. Reading GCA as "the full version of the thing whose min version we allow,
therefore forbidden by the `specialization` precedent" would be the wrong inference, and it is the inference
the names invite. The rule's precedent applies to `generic_const_exprs`, which is already forbidden, and it
does not reach this feature.

## What actually needed the feature, which is less than the brief claims

The brief states that GCA closes the numeral canonicity problem and closes container selection from a width
"at the same time". I tested both. The first is true. The second is not.

**Container selection does not need the feature.** The carry-and-read discipline op named in checkpoint
`127b`, where a const is carried and read but never transformed on the way into a type, gives the full
ladder with no feature gate and no flag at all:

```rust
pub trait Container: Copy + Default { const BITS: u16; }
impl Container for u8 { const BITS: u16 = 8; }   // ...through u128

pub struct Fx<const W: u16, C: Container> { pub raw: C }
impl<const W: u16, C: Container> Fx<W, C> {
    const FITS: () = assert!(W <= C::BITS, "width does not fit its container");
    pub fn new(raw: C) -> Self { let () = Self::FITS; Fx { raw } }
}
```

`p12_gatefree.rs` compiles clean on the pin with no gates and no `-Z` flag, admits widths 3, 13, 47 and 100
against their containers, and refuses a bad pairing at const eval:

```
error[E0080]: evaluation panicked: width does not fit its container
17 |     const FITS: () = assert!(W <= C::BITS, "width does not fit its container");
   |                      ^^^^^^ evaluation of `Fx::<9, u8>::FITS` failed here
```

The difference against the GCA form is that the consumer names the container and the compiler checks it,
rather than the compiler deriving it. That is an ergonomic cost, not a capability gap, and it is exactly the
`Capacity` move op cited as the precedent.

**Canonicity over computed widths does need it**, and rustc closes every exit in sequence. Starting from the
carry-and-read shape with arithmetic in value position:

```
error: use of `const` in the type system not defined as `type const`
help: add `type` before `const` for `Add2::SUM`
```

(`p13_mgca_assoc.rs`, mGCA alone.) Taking that repair:

```
error: complex const arguments must be placed inside of a `const` block
9 |     type const SUM: u16 = I + F;
```

(`p1_mgca_typeconst.rs`.) Taking that repair:

```
error: generic parameters may not be used in const operations
9 |     type const SUM: u16 = const { I + F };
  |                                   ^
  = help: add `#![feature(generic_const_args)]` to allow generic expressions as the RHS of const items
```

(`p2_mgca_constblock.rs`.) The compiler walks from mGCA to GCA in three steps and names the feature itself
at the end. There is no fourth repair. Under `min_generic_const_args` alone, a precision computed from
generic parameters cannot be read into type position.

With the gate and the flag it works, and canonicity holds where op ratified it. `PrecisionOf<13, 3>` and
`PrecisionOf<8, 8>` are both `W<16>` and both satisfy `takes16` (`p3_gca.rs`), a wrong sum is refused as
`W<17>` against `W<16>` (`p4_neg.rs`), nested composition normalises (`p10_stress.rs`), and the ladder works
in generic function bodies carrying the bound (`p11_generic_ctx.rs`).

**One sharp limit on canonicity, which the panel should have in writing.** It holds where const eval can
run, meaning at concrete instantiation. It does not hold under a generic parameter, because equality there
is definitional rather than semantic:

```
error[E0308]: mismatched types
   = note: expected struct `W<const { F + I }>`
              found struct `W<const { I + F }>`
```

(`p9b_defeq_generic.rs`.) `I + F` and `F + I` are the same value and different types in a generic context.
The same file's concrete case compiles. This is the tracking issue's own open question about opaque
definitional equality, arriving in practice. The design consequence is that there must be exactly one
syntactic path to any given precision. Two construction routes that compute the same width differently will
not unify while generic. That is a constraint on the design rather than a defect in the feature, and it is
cheap to satisfy if it is known about and expensive to discover later.

## The superseded objection was wrong on its load-bearing clause

The rule's one existing mention of the feature is at line 132, inside the 2026-05-29 GCE resolution that op
superseded on 2026-07-28. It says migration "is all-or-nothing with no incremental validation path". The
brief invited me to check whether the mutual-exclusivity half still describes a real conflict. It does, and
the all-or-nothing conclusion drawn from it does not follow.

The exclusivity is real and hard-enforced:

```
error: `-Znext-solver=globally` and `generic_const_exprs` are incompatible, using them at the same time is
not allowed
```

(`p5_gce_flag.rs`.) But it is a per-invocation constraint, not a per-dependency-graph one. Both directions
across a crate boundary compile:

- A GCA crate built with the flag, depending on a GCE crate built without it, and using the GCE crate's
  generic-const-expr-typed surface (`mix/gcalib.rs`, exit 0).
- A GCE crate built without the flag, depending on a GCA crate built with it, re-exporting its types and
  calling into its normalised surface (`mix/gcefacade.rs`, exit 0). This is the direction that matters for
  arvo, because the facade at `arvo/src/lib.rs:25` carries GCE.

The downstream story is stronger still, and it confirms the interrupted expert's two-crate result rather
than merely repeating it. A consumer with **no feature gate and no flag** compiles against a GCA library and
still receives full checking, including the refusal:

```
error[E0308]: mismatched types
 4 | pub fn use_bad() { takes16(PrecisionOf::<13, 4> {}); }
   |                    ------- ^^^^^^^^^^^^^^^^^^^^^^^ expected `16`, found `17`
note: function defined here
  --> libarvo.rs:11:8
```

(`tc/consumer_bad.rs`, built with no gate and no `-Z`.) The exposure is confined to the declaring crate. A
downstream consumer of arvo inherits the guarantee without inheriting the instability.

There is one real complication the objection could have been reaching for and did not state. Under Cargo,
`RUSTFLAGS` is global, so the naive route does reproduce all-or-nothing:

```
error: `-Znext-solver=globally` and `generic_const_exprs` are incompatible
 --> gcelib/src/lib.rs:2:12
error: could not compile `gcelib` (lib)
```

(`cargotest/`, `RUSTFLAGS="-Znext-solver=globally" cargo build`.) The escape is per-package rustflags:

```toml
[profile.dev.package.gcalib]
rustflags = ["-Znext-solver=globally"]
```

with `[unstable] profile-rustflags = true` in `.cargo/config.toml`. Both crates then build, mixed, in one
workspace (`cargo build` finished, exit 0). So the incremental path exists but is bought with a **second**
unstable mechanism, this one on the Cargo side rather than the compiler side.

That fourth dependency is transitional. It is needed only while GCE crates remain in the workspace. Once GCE
is gone, which the rule already mandates as drift remediation, a plain global `RUSTFLAGS` suffices and
`profile-rustflags` drops out.

## Where the flag lands

`-Znext-solver=globally` is not a `#![feature]` gate and the rule's three tiers do not describe it. The
brief asked what I think it should be. What I think, offered as a proposal and not as a call:

The flag is on a stronger institutional footing than most feature gates in the allowed table. It is an
accepted 2026 project goal, designated flagship and roadmap. The next solver is already stable for coherence
checking since Rust 1.84, so this is a widening of something shipped rather than a switch to something new.
The declared intent is that it becomes the only solver and the old implementation is removed. Remaining
blockers are named as finishing crater triage, achieving performance parity, and an RFC for cycle semantics.

Against that, it is categorically a larger exposure than any feature gate in the tables. A feature gate adds
a construct. This flag replaces the trait solver for the entire crate, so every trait resolution in that
crate, including ones that have nothing to do with const generics, goes through a different implementation.
The failure mode is not "the new construct misbehaves" but "unrelated code compiled differently".

My proposal is that the rule grows a fourth category for required compiler flags, and that this one enters
it at watch, with the rough edge named as whole-crate solver replacement and the exit condition being the
flag becoming the default. I am the first read on that and a second is owed under the two-expert rule. I am
explicitly not treating it as decided.

## What is op's to decide

Four things, and I have tried to keep them from bleeding into each other.

**His, and the rule says so.** The verdict's ratification. The rule's line 67 reserves a genuinely unclear
verdict for the human overseer, and line 4 of every table row that carries "(op, ...)" shows adoption calls
are his in any case. I am proposing a row; he ratifies it or does not.

**His, because the rule has no answer.** Which tier `-Znext-solver=globally` belongs in, and whether the
rule should grow a category for flags at all. The document does not cover this and I am not licensed to
invent the position.

**His, because it is a design trade and not a soundness question.** Whether the canonicity capability is
worth two unstable mechanisms when container dispatch, which was half the stated motivation, turns out to
need neither. The gate-free type-level numeral route also compiles (`p14_typelevel.rs`, exit 0, canonical by
construction, reaches a readable `const V: u16`) and buys canonicity with no gates at the cost of a
different consumer surface and a const-to-numeral bridge. That fork is a design call sitting on top of a
passed vetting, not part of the vetting.

**Mine, and I have made it.** The two gate answers. Not unsound by nature, and on the stabilisation path.
Op's clarification says that if both hold, "then that's fine". Both hold, on compiled evidence and on the
labels and text of #151972.

## The proposed row

For the "Watch (allowed, sound, but carries known incomplete-implementation rough edges)" table. Not to be
applied by me; `unstable-features.md` is untouched by this file.

| Feature | Tracking | Rough edge to be aware of |
|---|---|---|
| `generic_const_args` | #151972 | Vetted 2026-08-01. The declared next increment of `min_generic_const_args` (already allowed), not a rival to it: rustc **requires** the min gate to be enabled for this one, and the tracking issue states the feature is "based on the machinery developed for `min_generic_const_args`". The `specialization` / `min_specialization` precedent does not apply, because that min form is a sound subset rescuing a condemned full form, whereas this full form extends a rewrite that is itself the stabilisation path. No `I-unsound`, no `S-tracking-perma-unstable`, `C-tracking-issue` with a stabilisation checklist, T-lang and T-types engaged. Confines expressions to const-item right-hand sides, so it never takes on GCE's arbitrary-expression equality obligation. Rough edges: (1) hard-requires `-Znext-solver=globally`, a whole-crate trait solver replacement, tier unassigned pending op; (2) canonicity holds at concrete instantiation but **not** under a generic parameter, where equality is definitional, so `I + F` and `F + I` are distinct types (compiled: `p9b_defeq_generic.rs`); the design must admit exactly one syntactic route to any precision; (3) carries `incomplete_features`. Exposure is confined to the declaring crate: a downstream consumer with no gate and no flag compiles and still receives the refusal (compiled: `tc/consumer_bad.rs`). Mixed GCE and GCA crates interoperate in both directions; under Cargo this needs the unstable `profile-rustflags`, which drops out once GCE is gone. |

## If the answer is not adoption

Stated in one paragraph so the panel is not left with a verdict and no direction, per the brief.

Take the container dispatch gate-free now, because it is already proven and it is the piece op's checkpoint
`127b` names as next and as foundational. `p12_gatefree.rs` gives the whole ladder with zero gates, arbitrary
widths, no cap, and a const-eval refusal of a bad pairing, which is the carry-and-read discipline applied
exactly as op described it for `Capacity`. That removes the last live reason `arvo-strategy/src/lib.rs:11`
holds a forbidden GCE gate, which is the drift the rule has been carrying since 2026-07-28, and it is worth
doing whether or not GCA is adopted. Then treat computed-precision canonicity as the single remaining
question, where the fork is genuinely two-sided: adopt GCA and keep the `UFixed<13, 3, Warm>` surface at the
cost of a feature gate plus a `-Z` flag, or take the gate-free type-level numeral, which is canonical by
construction and compiles today, at the cost of a changed surface and a const-to-numeral bridge that is
itself the hard part. Do not let the container work wait on that fork, because it does not depend on it.

## Defects found in the governing rule while vetting

Reported because the rule is the authority here and these are wrong in it. None is mine to fix.

1. **The cited sketch does not exist.** Line 132 cites
   `arvo/mock/research/sketches/202605291007_min-gca-feasibility/` as the proof that mGCA cannot express the
   patterns. `ls` returns "No such file or directory". The extant sketch on this subject is
   `202607291400_const-args-under-min-gca`. The rule's conclusion happens to be right, which I re-established
   by compiling (`p13` into `p1` into `p2`), but it currently rests on a citation that resolves to nothing.
2. **The recorded toolchain hash does not match the installed toolchain.** Line 95 records
   `nightly-2026-05-28` as `1.98.0-nightly (cced03bfd)`. `rustc +nightly-2026-05-28 --version --verbose`
   reports `1.98.0-nightly (57d06900f 2026-05-27)`. I ran against the installed one.
3. **The forbidden feature is still shipping.** `generic_const_exprs` is forbidden (line 74) and remains
   live at `arvo/mock/crates/arvo/src/lib.rs:25` and `arvo/mock/crates/arvo-strategy/src/lib.rs:11`. The rule
   names this as drift to remediate and it is still unremediated. It is also the only thing making the Cargo
   side of a GCA migration need `profile-rustflags`.
4. **A superseded objection is being carried forward as if it still bites.** The "all-or-nothing with no
   incremental validation path" clause at line 132 is false as stated, in both crate-boundary directions and
   under Cargo with per-package flags. It is marked superseded, but it was quoted to me as live context in
   the brief for this dispatch, which is how a retired claim keeps steering decisions.

## What I did not check

- The open next-solver bug list, for soundness-shaped entries specifically. Counts and characterisation
  above are from search results I did not verify.
- Compile-time cost of `-Znext-solver=globally` on a real arvo crate. My probes are small files, so they say
  nothing about the performance parity blocker in practice.
- Whether `profile-rustflags` itself has a tracking issue worth vetting. It is a second unstable mechanism
  and I named it but did not run the rule's procedure on it.
