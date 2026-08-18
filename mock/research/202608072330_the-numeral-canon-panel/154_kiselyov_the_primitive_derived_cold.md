# 154. What a primitive is, derived cold

**Phase one. Written blind.** Read: `INTENTS.md`, `RULES.md`, this repository's `.claude/` and
`CLAUDE.md`, the workspace rules at `/Users/orgrinrt/Dev/clause-dev/.claude/rules/`, `mock/Cargo.toml`
with its comments, `rust-toolchain.toml`, and `mock/benches/` (layout, `bench.toml`, the variant
crates, the committed CSV/meta/findings). Nothing else. No panel file, no `AGREEMENTS.md`,
`OPTIONS.md`, `DROPLIST.md`, `HANDLES.md`, `PRIOR_CALLS.md`, `PERSONA_CALLS.md`, no `SEED_*`, nothing
under `archive/` or `seed/`, no other member's probes, no `git log`, no commit subject.

**Declaration about the cut.** This dispatch was interrupted by a network error partway through the
test gate and resumed. Nothing outside the permitted premise list was read before the cut or after
it. The only thing on disk from before the cut was this file's header.

Phase two is appended at the end and phase one is not rewritten.

---

## 0. The two gates

### 0.1 The canon gate: passed, with one thing I am naming rather than resolving

Checked against `INTENTS.md` in full, including its "How to read an entry" section. The assigned
question ("what is a primitive, in arvo?") is licensed: `INTENTS.md:190-197` (I11) makes the base and
the contracts above it the stated purpose of the library, and `INTENTS.md:286-289` (I14) makes "public
API positions use the stack's own primitives rather than bare integers, floats, `bool` or `usize`" an
IN FORCE constraint, which presupposes that there is a determinate answer to what one of those is. A
canon cannot state I14 without stating what a primitive is.

**What I am naming rather than resolving.** The word "primitive" appears in the governing material in
at least three incompatible senses, and no intent disambiguates them. Section 1 below is that finding.
Under the panel's ambiguity handling this would be grounds to hand the call back; I am not doing that,
because the brief's own question ("whether a primitive is one thing or several things under one word")
is precisely this question, so the ambiguity is the assignment rather than a blocker to it. I flag it
so that a later reader does not mistake my section 1 for a decision op made.

### 0.2 The test gate: passed, and the suite is unusually good, with two real defects

`mock/crates/` is empty (`ls mock/crates/` returns nothing; `mock/Cargo.toml:19` has `members = []`
with a header explaining the deletion). The suite-bearing surface is therefore
`mock/benches/variants/` alone.

**Counts, each with its command.**

```
grep -rn "#\[test\]" mock/benches/variants/ mock/benches/src/ | wc -l      -> 124
ls mock/benches/variants/ | wc -l                                          -> 94
grep -rl "#\[test\]" mock/benches/variants/ | sed 's|.*/\([^/]*\)/src.*|\1|' | sort -u | wc -l -> 13
```

Thirteen of the ninety-four variant crates carry tests; they are the `*-shared` crates, which is the
right place for them (the arms are thin, the model is shared). `cargo test` at `mock/benches/` runs
**0 tests**, because the variants are path dependencies rather than workspace members, so the suite
has to be run crate by crate. That is a real trap and I record it: an agent that runs `cargo test` at
the bench root and reads `test result: ok. 0 passed` as green has measured nothing.

Per-crate results, `cargo test --offline --manifest-path variants/<c>/Cargo.toml`:

| crate | tests | result |
|---|---|---|
| bitpack-carrier-shared | 9 | ok |
| bitpack-contend-shared | 12 | ok |
| bitpack-footprint-shared | 6 | ok |
| bitpack-plan-shared | 5 | ok |
| bitpack-shared | 3 | ok |
| bitpack-wide-shared | 6 | ok |
| bitpack-write-contend-shared | 15 | ok, but see below |
| quantiser-fadd-shared | 1 | ok |
| quantiser-radix-shared | 3 | ok |
| satfold-shared | 11 | ok |
| warm-clamp-shared | 7 | ok |
| warm-container-shared | 15 | ok |
| wide-rung-shared | 30 | ok, 115.43s |

I read test bodies rather than names, and scanned all 123 parsed test bodies mechanically for the
absence of any assertion or panic (`154_probes/gate_scan.py`, committed with its output). Eighteen came
back with no assertion in their own body; seventeen of those delegate to a helper that asserts
(`bitpack-carrier-shared/src/lib.rs:361-363` calls `check_size::<16384>()`, and the same shape for the
rest), and the eighteenth is a macro body. **No tautological test found.** No test asserting a constant
against its own definition. No sampled law where the whole matrix was available: the opposite, in fact,
and the crates say so in their own comments.

`warm-container-shared/src/lib.rs:171-175`:

> /// The shipped rule widens every width at or below 64 bits. Asserted
> /// over all 64, since a sample would not establish "every".

`warm-container-shared/src/lib.rs:23-27` on its oracle:

> /// The value is also checked against the independent `u128` reference,
> /// so four agreeing arms sharing one wrong transform is caught rather
> /// than confirmed.

And `warm-container-shared/src/lib.rs:86-92` records a probe that was dead and how it was found:

> /// The first version of this check reported zero nanoseconds and was
> /// wrong: it exclusive-ored the result into a sink an even number of
> /// times, so the sink was provably zero and the whole loop was dead.
> /// `black_box` on both ends is what makes the call observable.

That is the negative-control discipline applied by the author to its own instrument, which is the
thing this workspace's rules were written because nobody did.

**Defect one: a test that structurally cannot fail, costing minutes of wall clock on every run.**
`bitpack-write-contend-shared/src/stress.rs:96-109`,
`naive_kernel_corruption_rate_under_real_concurrency`, runs 3000 concurrent trials and then asserts
nothing at all. Its own comment (`stress.rs:103-108`) explains why, and the reasoning is sound: a
scheduler-dependent corruption rate is not a threshold anybody should gate on, and the sibling control
`naive_kernel_never_corrupts_when_the_split_is_aligned` (`stress.rs:114-124`) does the gating. **The
reasoning is right and the placement is wrong.** A `#[test]` that cannot fail is not a test, it is a
diagnostic, and putting it in the default run means every `cargo test` in that crate pays thousands of
threaded trials for a number that is read off stderr. It belongs behind `#[ignore = "diagnostic: ..."]`
so the count that gets cited as coverage stops including it and the default run stops paying for it.
This is the one place in the suite where the gate's "tests that assert nothing" bullet lands, and it
lands softly, because the author declared it.

**Defect two: the suite is slow enough that it will stop being run.** `wide-rung-shared` takes 115s and
`bitpack-write-contend-shared` had not finished after twelve minutes of wall clock on this host, held
by the three `stress::` tests. A suite nobody runs is a suite that is not measuring, whatever its
quality, and the whole cost sits in two files. Moving the three stress tests behind `#[ignore]` (they
are explicitly "run outside the timed bench path", `stress.rs:1`) takes the surface from twelve-plus
minutes to under two.

Neither defect touches the question I was dispatched for and neither is a reason to refuse the work.
I proceed.

