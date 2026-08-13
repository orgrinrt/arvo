# Dispatcher note: fourteen validators that were written and never run, and a correction to `41`

**Position:** after `95`, during the strategy-axis unit's cold pair. **Author:** the dispatching agent.
**Standing:** a measurement and a fix, not a design finding. Carries no authority over any open
question.

Recorded separately because it bears on the instrument every number in this panel comes from, and
because it corrects a claim in `41` that currently qualifies the whole bench corpus more broadly than
the facts support.

## What `92` found, and what it turned out to be

`92` reported, incidentally to its own question, that thirteen bench variant crates define a
`validate_output` the harness never calls, because the harness only calls it when a variant declares
`outputs_may_differ`, and exactly one does. The mechanism is correct and is now established precisely.
**The count was low, in `92` and in the first version of this note, and both were low the same way.**

`grep -l "fn validate_output" */src/lib.rs` returns thirteen. Grepping for the property rather than for
one filename returns **fifteen crates**: `bitpack-contend-shared` and `bitpack-write-contend-shared`
keep their `Routine` in `src/routine.rs`, so a `lib.rs`-scoped search cannot see them. Exactly one
crate, `satfold-shared`, declares `outputs_may_differ`. **So fourteen crates have a validator that has
never run, not twelve.** Found by an independent reviewer of the upstream fix, which is the second time
in this panel a count taken from one filename pattern has been low.

`mockspace-bench-harness` at the pinned revision `084e780`, `bench-harness/src/validation.rs:105-113`:

```rust
// The validator is only meaningful when the Routine actually
// declared one; we cannot tell from the bridge alone, so use
// outputs_may_differ as the consent signal.
let validator: Option<fn(&[u8], &[u8]) -> Result<(), String>> =
    if routine.bridge.outputs_may_differ {
        Some(routine.bridge.validator)
    } else {
        None
    };
```

and the three checks below it were an exclusive `if / else if / else` chain, so selecting the
per-variant validator also **deselected** cross-variant comparison. The two were entangled in both
directions.

**The trait documents them as independent, in its own words.** `bench-core/src/lib.rs:91-93` says
`validate_output`'s default is "no structural check; the harness **still** does cross-variant byte
comparison unless `outputs_may_differ` is true", and `bench-core/src/lib.rs:111-112` says
`outputs_may_differ = false` means the harness "**also** does cross-variant byte comparison". Both
sentences describe two checks that run independently. The harness gated one on the other.

Consequence: a routine that declares a validator **and** expects its arms to agree byte-for-byte gets
the validator silently dropped. That is the common case, and it is the case all fourteen are in.

## The fourteen are not decorative

They are the strongest fidelity checks in this repository, and none of them has ever executed:

- `bitpack-shared` recomputes the column sum from `input.logical` as ground truth and refuses if the
  extraction path produced a different value stream.
- `warm-clamp-shared` first checks that both carrier regions hold the same logical column, so the arms
  were fed the same input at all, then compares the output against an independent `u128` reference
  implementation of the declared clamping semantics.
- `quantiser-radix-shared` bounds every significand by the format range and names a carry-out or
  alignment defect when one escapes.

The full list, by `grep -rl "fn validate_output"` under `mock/benches`, one line per crate:
`bitpack-carrier-shared`, `bitpack-footprint-shared`, `bitpack-plan-shared`, `bitpack-shared`,
`bitpack-wide-shared`, `quantiser-fadd-shared`, `quantiser-radix-shared`, `satfold-shared`,
`structural-decomposition`, `spectral-bisection`, `warm-clamp-shared`, `warm-container-shared`,
`wide-rung-shared`, and the two a `lib.rs` grep misses, `bitpack-contend-shared` and
`bitpack-write-contend-shared`. Fifteen; `satfold-shared` is `92`'s own and is the one that declares
`outputs_may_differ`, so it is the one that ran.

**Several of them do exercise their validator from their own in-crate unit tests** (for instance
`bitpack-contend-shared/src/tests.rs:167`), so the logic is not untested. It is uninvoked by the
harness, which is a different and narrower claim than "never checked".

## The fix, upstream

`hiisi-digital/mockspace` PR #18, on `fix/bench-validator-runs-independently`. It splits the decision
into the two questions the contract asks: the per-variant validator runs always, and cross-variant
comparison is a separate choice skipped only when the routine consents to variants differing. Running
the validator unconditionally costs nothing for a routine that declared none, because the trait's
default returns `Ok(())`.

The check came first and failed against the pre-fix code, which is the record that the defect was real:

```
assertion failed: validation_plan(false, None).per_variant
test result: FAILED. 6 passed; 1 failed
```

**Expect red on the first run of any of the fourteen.** A validator that fires is reporting a defect that
was already there and was invisible, not a regression introduced by turning it on.

## The correction to `41`

`41` closes with: "`mockspace-bench-core`'s orchestrator never calls validate and `run_worker_validate`
is not re-exported, so no consumer can reach it. Verified in a clean clone."

**The first half is true and the second half is false**, and `41` contradicts the file it cites on the
second half.

- `run_orchestrator` (`bench-harness/src/harness.rs:508`) still does not call validate. True today,
  upstream, unfixed.
- `run_worker_validate` **is** reachable. `harness` is a `pub mod` and `harness::run_worker_validate`
  resolves. `22:498` says exactly this and reached it, which is how `22` wired validation into arvo's
  own bench driver at all.
- A second entry point, `driver/mod.rs:392`, has called `validation::validate` since 2026-07-19, and
  the pinned revision `084e780` contains that call. So the harness is not uniformly unvalidating; it
  depends which entry point a consumer took.

**What this does not change about `41`.** Its measurement stands: 214 CSVs, 82,960 rows, digest zero in
every one, and the pre-`22` corpus was never cross-checked. Its 50x sweep and the six hits stand. The
`warm-container-kernel` finding stands.

**What it does change.** arvo's bench driver has called `harness::validate` itself since `22`
(`mock/benches/src/main.rs:160-170`), so runs after that point **were** cross-variant validated. `41`'s
"no committed bench in this repository has ever cross-checked that its variants agree" is true of the
corpus `41` measured and is not true of runs since. And the per-variant validators still did not run,
for the different reason above, which is what PR #18 fixes.

## What is still owed upstream

**`run_orchestrator` does not validate, so every consumer must hand-roll it.** arvo did, in `22`, and
again for `check_disasm_duplicates` in `92`. Any other consumer gets neither guard and is not told.
This is the same shape twice: the harness ships a correctness guard that its main entry point never
invokes, so whether a bench is checked depends on whether its author knew to call the checker.

Not fixed in PR #18, deliberately, because it is a separate change with a much larger blast radius:
turning validation on for every consumer at once will surface whatever it surfaces. It wants its own
PR, and it wants a real arvo bench run against it before merging rather than after.

## Two further observations about the instrument, neither fixed

**mockspace's own self-bench does not run.** `mockspace/benches`, the bench whose `bench.toml` says it
exists to validate "the full mockspace bench harness pipeline ... against a real consumer codebase:
mockspace itself", produces zero rows. Every variant reports `<load-fail>`, including with both
variant cdylibs freshly built and present at the manifest's paths. Not diagnosed further; it is not on
the panel's path.

**A variant that fails to load is reported as a timeout and then crashes the run.** The load failures
above print as `TIMEOUT  <load-fail>  warm  0` rows, the run completes and writes a zero-row CSV, and
then `analysis.rs:313` panics with `index out of bounds: the len is 0 but the index is 0`. So the one
condition that is unambiguous, the dylib did not open, is reported as the one that is ambiguous, it
was too slow, and the run ends in a crash rather than a message naming the file that would not load.

Both are the same shape as the finding above and as `41`'s: **the harness's failure modes are quiet or
misleading rather than loud.** Neither is fixed here. Neither bears on any number the panel holds,
because no panel measurement came through this bench.

Attribution, so this is not read as a regression: the load failure and the panic both reproduce on a
path that never calls `validation::validate`, which is the only function PR #18 touches.

## Predicate

Everything above holds for: `mockspace-bench-harness` at `084e780` and at `dev`; arvo at
`feat/arvo-shape-topic`; `nightly-2026-05-28`; the thirteen variant crates named. It says nothing about
any other consumer of the harness, none of which was examined.

## Addendum: the surviving dead tier, and the line offset it cost

Reported independently by both cold derivations of the strategy-axis unit, `93` and `94`, each outside
its assigned question and neither having read the other at the time. Two experts, and verified at
source before acting.

`mock/DESIGN.md.tmpl` and `mock/PRINCIPLES.md.tmpl` survived the crate-tree removal unbannered, and
were being read as current: both cold derivations read them and reasoned from them. They assert as
settled two things that are not. The four-marker strategy set, which I1 demoted to open. And
`feature(generic_const_exprs)`, which `unstable-features.md` forbids on op's own call
(`mock/PRINCIPLES.md.tmpl:32-38` as those files stood). `93` adds that they describe a sixteen-crate
topology that does not exist.

This is exactly what `the-canon-design-code-chain.md` names: a lower tier that survives a change above
it becomes a claim about a document that no longer exists, and it gets read and defended because it is
concrete and detailed next to an abstract statement.

**Both are now bannered as superseded rather than deleted**, because `docs/DESIGN.md` is generated from
the first and deleting a public-facing document is not this run's call. The banner is written to the
public-surface rules: it names what is open without naming the panel, the intent catalogue, or any
`mock/` path, because it renders into a document strangers read.

**The offset, recorded because it is the cost of the fix.** The banner is **8 lines** at the top of
each file. `93` and `94` between them carry **19 line citations** into these two documents, written
before the banner. Every one of those is now low by exactly 8: a citation to
`PRINCIPLES.md.tmpl:288-292` resolves at `:296-300` today. Member files are the historical record and
are not rewritten, so the offset is recorded here instead.

The general lesson, and it is the second time this panel has paid it: **a line citation into a document
that is still alive is fragile by construction.** A heading anchor fails loudly when the heading moves;
a line number resolves to the wrong text in silence.
