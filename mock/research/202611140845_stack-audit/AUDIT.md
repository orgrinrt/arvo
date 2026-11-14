# arvo In The Stack: The Second Audit, 14 November 2026

A quality review of arvo as the numeric layer of the stack (notko, arvo, hilavitkutin), at `dev` tip
`d0161800`, one day and thirteen landings after the first audit
(`mock/research/202611131215_stack-audit/`). It changes nothing in the tree. It judges the
direction, the concept, the intents and the designs first, and reads the code as evidence of
whether the designs can be built.

Files: this one (summary, verdicts, what became of the first audit, the ledger, the passes);
`DESIGNS.md` (the concept and each design, with evidence); `NUMBERS.md` (every bench and spike, why
the numbers are what they are, estimates); `BENCH-CATALOGUE.md` (benches to write);
`DOWNSTREAM.md` (fit to kaski, hilavitkutin, generic applications, daemons). One ad-hoc spike was
written for it: `mock/research/sketches/202611140930_stack-audit-2-value-tier/`. Every figure from
it is one shared x86_64 machine, is called an ad-hoc spike, and is not a bench result.

Each part carries one of four marks, in words: good as it is; easy to get wrong but can be right;
going wrong; back to the drawing board.

## Summary

The direction is right and nothing is back to the drawing board. The concept (a format is a domain
and a representable set, an operation is an exact step and a declared adaptation, behaviour is
stated over the declared signature, the container is a separate derivation) carried six new
operations, two standards and the host doors in one night without one row of the spine being
amended. That is the best evidence so far that the factoring is sound, and it is the part of arvo
a consumer cannot get anywhere else.

The integer half of the value is now the hand code. In the ad-hoc spike the sum, difference,
product, order and the egress door time the same as what a programmer writes on the host integer,
at widths below the carrier as well as at it.

The fixed-point half is going wrong, and it is the half the one known consumer waits for. The
product, the quotient and the conversion run the format's exact step at run time: about 350 ns for
a Q16.16 product against 1.1 ns by hand in the ad-hoc spike, 60 to 330 times across the three. The
tree knows the debt and ticketed it; nobody had put a number on it. The design's stated reason for
not writing the fast arm contradicts the contract the same design opens with and the crate
already lives by for its other operations. It is fixable in place.

Two smaller things of the same kind landed the same night. The new host door is 35 to 45 times a
hand clamp in a default release build, because two private functions lack an inline attribute; the
assembly is committed. And under the default debug check an integer product runs the wide step on
every call, about 200 times the host's checked product.

All three have one cause, and it is the first audit's process finding, still standing: the plan
measures last. Value benches are step 19 of 20. No bench file has changed since before the first
audit, and no committed bench calls a type that ships. Some twenty review directories were written
overnight and none ran the code for time.

One stated row now works against a stated goal. Op has said twice that arvo serves general
numerics, applications and daemons. The verdict a daemon needs for untrusted input, whether a host
integer names a member, exists in the source and is kept private under
`ruling::the_ingest_door_waits_on_a_known_use`. The use is now known. The same row holds back a
divisor from a run-time value, which kaski needs. The row is stated, not ratified; this is a
tension between a row and op's later answer, reported as a finding and not weighed away.

Of the first audit's findings: five are fixed (the doors' design, the mutant features, the root
documents, the red test, an executable reference for IEEE binary); nine hold; one I withdraw on
its behalf (that the format design narrates old defects); one mark I change (`Hot` is a labelled
placeholder, not going wrong); and one of its statements is no longer true (there is now an arm in
shipped code, `Num::mul`).

What I would do first, in order: write the fixed-point product as carrier arithmetic held to the
oracle, with its bench in the same change; mark the two door functions inline; put kaski's own
`Fixed` in that bench as the baseline; read the ingest row against op's answer; change the scope
sentence in the README.

## Verdicts

| Part | Mark | Where |
|---|---|---|
| Format spine, constant quantum | good as it is | `DESIGNS.md`, the concept |
| Format spine, floats | easy to get wrong but can be right: one operation over a magnitude-indexed quantum before anything is designed on the clause | same |
| Behaviour per declared signature | good as it is | same |
| Never a run-time check, at the program's edge | easy to get wrong but can be right, leaning wrong: publish the verdict, a divisor from a value, name the pattern | same |
| Arms over regions | easy to get wrong but can be right: bench both sides of every shipped arm, let the consumer see which it got | same |
| Strategy axis | easy to get wrong but can be right: the mode and policy table on two hosts before a preset loses its ticket | same |
| Parity with the standards | easy to get wrong but can be right: arithmetic through both sides; a committed MATLAB table | same |
| `arvo-numeral`: construction, admission, integer ring, order, doors' design | good as it is | `DESIGNS.md`, the crates |
| `arvo-numeral`: fixed-point product, quotient, conversion | going wrong, fixable in place | same; `NUMBERS.md` |
| `arvo-numeral`: traits, hash, default, layout promise | easy to get wrong but can be right | same |
| `arvo-format` as oracle and home of the standards | good as it is | same |
| `arvo-format` as the arithmetic a value runs on | going wrong, fixable in place; decide which crate owns the fast adaptation | same |
| `arvo-placement`, sole | good as it is | same |
| `arvo-placement`, shared | easy to get wrong but can be right | same |
| `arvo-strategy` presets | easy to get wrong but can be right | same |
| `arvo-bits` | good as a register field; going wrong as a table field, fixable in place | same |
| Root documents | good as it is, with the scope sentence to change | same |
| Registry as an instrument | good as it is | same |
| Order of work (measure last, consumer not ported) | going wrong, fixable in place | same |
| Bench corpus as an instrument | going wrong, fixable in place | `NUMBERS.md` |
| Bench corpus as evidence for the designs | easy to get wrong but can be right, per family | same |
| Saturating-fold and `contend-best` families | good as they are | same |
| The disassembly sketch and the two spikes as method | good as it is, as far as an ad-hoc spike goes | same |
| Fit to kaski | easy to get wrong but can be right | `DOWNSTREAM.md` |
| Fit to hilavitkutin | easy to get wrong but can be right | same |
| Fit to a generic application | easy to get wrong but can be right | same |
| Fit to a server daemon | easy to get wrong but can be right | same |
| notko under arvo | good as it is | same |

## What became of the first audit's findings

| First audit said | Now | Evidence |
|---|---|---|
| The doors go through the oracle and a 128-bit slot | Fixed in design; fixed in code under fat LTO only | `mock/crates/arvo-numeral/src/host.rs:88-174`; spike; `host.rs:49-63` not inline |
| A product through the oracle would be 10 to 50 times slow, by reasoning | It was built that way and is 60 to 330 times, by ad-hoc spike | `num.rs:396-403`, `ratio.rs:58-63`, `:112-117` |
| `Hash` hashes the 128-bit index | Holds | `num.rs:435-439` |
| No operator or ordering traits | Holds; inherent order added | `order.rs:14-82`; `DESIGN.md.tmpl:360-372` of the numeral crate |
| Mutant features a dependent could switch | Fixed | `mock/crates/arvo-numeral/Cargo.toml:26-37` |
| No default adaptation | Holds, by an open row | `alias.rs:6-13`; `mock/registry/ruling.toml:1074` |
| Layout promise open | Holds | `DESIGN.md.tmpl:136-144`; `mock/registry/question.toml:98` |
| One red test in the format | Fixed, by reading; not run | `multiplication/tests/the_inverse_round_trip.rs:37` |
| Parity is a reading of documents | Fixed for IEEE binary rounding; holds for decimal and MATLAB; no arithmetic through either | `mock/crates/arvo-format/tests/ieee754_binary_parity.rs:215-241`, `:488-511` |
| The format design narrates old defects | Withdrawn; one to three percent by a reader's sample. The length holds, 2424 lines | `DESIGNS.md` |
| No arm exists in the tree | No longer true | `num.rs:379`, `rung.rs:91-100` |
| `Hot` as shipped is going wrong | Finding kept, mark changed: a ticketed placeholder nothing reads | `mock/crates/arvo-strategy/src/lib.rs:89-92` |
| Three presets bind one objective; no value takes a strategy | Holds | `lib.rs:95`, `:132`, `:150`; `mock/DESIGN.md.tmpl:47` |
| `Bits<N>` is a `u64` at every width, no set operation | Holds | `mock/crates/arvo-bits/src/lib.rs:48-50` |
| Root documents describe a tree that is gone | Fixed, but for the scope sentence | `f939933f`; `README.md:41` |
| The benches measure no shipped type | Holds | `mock/registry/probe.toml:1309`; no bench file changed |
| The plan measures last | Holds | `mock/research/202611130950_canon-double-check/PLAN.md:125-126` |
| The consumer is consulted through documents | Holds | hilavitkutin pins the old arvo (`DOWNSTREAM.md`) |
| Scope: are general numerics in | Answered by op: yes | op-answers-26 and -36 |

The night's landings, as claims: that the value's operations equal the format's on every member I
did not test by running the suite, which could not be built here. The ad-hoc spike asserts each
value arm equal to an independent hand arm on 4096 inputs at 13 and 32 bits before timing, and
none disagreed. That is small and it is the only execution of the new code in this audit.

## Ledger: what was read, how far, and what was not

Read whole by me:
- `arvo-numeral`: every source file and the design.
- `arvo-strategy`, `arvo-placement`, `arvo-bits`: source and designs (bits has source only).
- Root: `README.md`, `mock/PRINCIPLES.md.tmpl`, `mock/DESIGN.md.tmpl`.
- The first audit: `AUDIT.md`, `DESIGNS.md`, `NUMBERS.md`, `DOWNSTREAM.md`.
- `mock/research/202611130200_bench-harness-at-head/FINDINGS.md`; both spikes' findings.

Read in part by me:
- `arvo-format` source: `apply.rs`, `multiplication.rs:60-394`, `division.rs:64-198`, the
  multiply, divide and common-divisor parts of `wide.rs`. Not its other modules, not its tests.
- `PLAN.md` of the canon double check: lines 98 to 232 and 505 to 568.
- Registry: the ruling, obligation, question and probe rows cited here, opened at their lines. Not
  the rest of the 110 rulings; none of the 173 proposals, 196 retirements, laws, dimensions or
  topics as rows.
- Bench results: nine findings files opened to check figures, and the headline of every family the
  first audit did not name. Not the 254 result sets.
- The first audit's `BENCH-CATALOGUE.md`: headings only.

Through readers (sub-agents that read and reported; the judgement on what they found is mine, and
each use is marked in the text):
- Op's answers 1 to 38 on the kaski channel. The response captures under `.data/op-responses/` were
  not listed by that reader and are mostly unread.
- The registry, counted and summarised by namespace.
- `arvo-format`'s design (2424 lines) and its test suites, section by section.
- The 61 sketch directories, the 225 design rounds (57 from 12 to 14 November) and the research
  directories of the night, at the level of what each is and how it ended. Not judged one by one.
- kaski, hilavitkutin and notko sources, sampled; most searches there were refused to the reader.

Not done:
- No test was run. The shared build directory ran out of disk when I tried the numeral suite.
- No bench was rerun. Nothing ran on aarch64 or wasm32.
- A reader sent over the bench corpus did not report in time; `NUMBERS.md` does not rest on it.
- `cargo mock canon` was not used; rows were found by search.
- hilavitkutin's and notko's own audits of today were not read.

## The passes

Pass one. Everything above and in the four sibling files.
