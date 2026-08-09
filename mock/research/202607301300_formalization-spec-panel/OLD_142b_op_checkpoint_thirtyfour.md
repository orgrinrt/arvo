# Op's thirty-fourth checkpoint: Warm is settled and stays settled

**Date:** 2026-08-07
**Position:** after `142_giesen_excel_everywhere.md`.
**Required reading with the standing base.** The first section closes a question that has now been put to
op three times.

## Warm is what Rust does. Stop asking.

`142` found the `70b`-ratified preset table gives `Warm` and `Cold` the clamp while `140b` defines `Warm`
as behaving like a native Rust primitive, and put the tension to op as a fork. He refused the framing:

> Intent holds. Whatever feels intuitive and is how rust behaves. That's warm. You are once again bringing
> me questions about Warm when I've been clear on this. If the rust one is different for different profiles
> or targets, then our Warm should too, obviously.

So the ruling, and it is not a new one:

**`Warm` does whatever a native Rust primitive does, including varying where Rust varies.** Rust's integer
overflow behaviour is profile-dependent: it panics under `debug-assertions` and wraps in release. `Warm`
inherits that, and inherits any target variation the same way. The resolution is therefore **not a fixed
cell in a table**; it is a function of the build profile, exactly as it is for the primitive it imitates.

**The `70b` clamp cell for `Warm` is stale**, under op's own staleness principle (`108b:11-20`): it was
ratified under an understanding the intent statement has since superseded. `Cold`'s cell is untouched by
this, since `Cold` is not defined by the native-primitive intent.

**And the meta-point is the reason this section exists.** This is the third time Warm has come back to op,
and each time the panel arrived carrying a mechanism and asked him to adjudicate between it and the intent.
The intent was never in question. The mechanism is what moves. A future member finding an apparent conflict
between a ratified mechanism cell and the Warm intent should conclude the cell is stale and say so, rather
than escalating. That is the standing shape at `140b:24-27`: the intent remains, the mechanisms and theory
live freely and shift under and around it.

## What four files built on, and why it survived

`131`, `139`, `140` and `141` all built on `Warm` wrapping, taken from a **source doc comment** at
`arvo-strategy/src/container.rs:15-16` which says the doubled width "carries single-op overflow headroom for
Warm wrapping". The ratified table saying otherwise sits in the standing base those files were all given,
at `110:2702-2712` and `124:2600-2612`.

So this was not a briefing gap. It was four files trusting a source comment over the design, which is the
exact failure `design-is-the-oracle.md` names and which op has restated repeatedly. The comment at
`container.rs:15-16` does not survive whatever else is decided.

By the intent ruling above, the outcome is that those files were closer to right than the table was, and
they got there for the wrong reason. That does not make the method acceptable: they had no way to know, and
the next comment they trust will not happen to agree with op.

## The fold accumulator: second read owed before anything moves

`142` found the stated ground for `StoredWidth = doubled` is an accuracy property across a fold rather than
per-operation overflow room, that the design already ratifies the exact derivation (`W + ceil(log2 n)`, the
Motorola guard-bit rule), and that the doubling is therefore a fixed, always-paid approximation to a
quantity the design can compute. **Op: not yet, this needs a second read.** It reinterprets a ratified
sentence and refutes four files at once, and it is one instance of evidence.

## The harness comes first

Op's sequencing, and it precedes any further panel work:

> Not only is option 3 our choice, but we go beyond. Identify more to get closer to the desired composition.
> This is just one instance of evidence, completely unaudited by a second expert set of eyes. And before
> that, gotta fix the upstream.

So: **fix mockspace's bench harness upstream first**, through a fresh clone, a proper fix, a PR and a
reviewer pass, and only then return to the panel. Two defects found by `142`:

- `bench-harness/src/disasm.rs:16` passes `bench_entry` where the Mach-O symbol is `_bench_entry`, so the
  objdump path is dead on macOS and silently falls through to `otool`.
- `check_duplicates` compares `bench_entry`, which under `#[bench_variant]` is a 592-instruction dispatcher
  identical across every arm, so it cannot see what an arm computes.

And an audit is owed inside the panel: **`--emit=asm` on a library under fat or thin LTO shows pre-link code
with zero vector operations on everything.** Several panel files cite `--emit=asm` and nobody has checked
which carried an LTO profile. Any instruction count taken that way is void.

## Then the composition, wider

After the harness: the signed case, where `142` conceded the retraction lemma is false and which it named as
the first thing next, and beyond that more levers toward the composition. Four are measured so far, all on
the harness: fold arity as a const generic up to 20.5x and never costing, accumulator narrowing 1.1x to
4.0x, saturating-fold reassociation 41.0x at high arity and -8.0x at low, and an interior-safety predicate
deleting the clamp up to 51x. `141`'s claim that the container axis is monotone is refuted, and attacking
the losing arm recovered 41.0x of a 44x loss, so the container was never the cost.

All of it is one instance of evidence and unaudited.

## Standing

Only op's calls are final and they go stale when their evidence moves. The panel produces canon, not source;
`mock/research/` and `mock/benches/` are its ground and `mock/crates` is out of bounds. Experts are
dispatched one at a time, each reading the ones before it, each writing incrementally, each going down the
rabbit hole rather than reporting blockers, and each taking small wins because many of them are the program.
