# A Quality Review Of arvo As Part Of The Stack

Date: 13 November 2026. Branch `work/stack-audit`, read against `dev` at `326b52f5`. Written by one
helper session on op's order for a full review of the stack for hilavitkutin and arvo; a sibling
session reviews hilavitkutin. Nothing in this directory is canon and nothing in it changes a crate, a
design, a registry row or a test.

## The summary, which can be read alone

The concept holds and needs no course change. A format as a domain and a representable set, arithmetic
as an exact step and a named adaptation, behaviour stated over what was declared and never over the
container, invalid constructions refused as failed constants, and many small implementations selected
at compile time: each of these is sound, each is better than what a consumer gets from the host
language or from the libraries I can compare it to, and the code written on 12 and 13 November shows
the canon can carry a real implementation quickly. Nothing in the intent goes back to the drawing
board.

What exists is early and is narrower than the documents say. A consumer can depend today on one value
type that adds and compares. The format tier underneath has six operations as an exact oracle. There
is no subtraction, product or division on values, no fixed-point value anyone has timed, no bit set,
no float operation, and no arm above the placement.

One design decision should go back to the drawing board: a strategy that names an adaptation. As
shipped, choosing a strategy chooses whether arithmetic wraps or saturates and how it rounds, so a
consumer changing a performance preference changes its results. Three of the four presets differ in
nothing else, all four adaptations are marked as placeholders, and the value type does not read a
strategy at all. Op's direction of 13 November, an overflow default that is a generic the consumer
overrides per case, fits better if the adaptation default is its own parameter and a strategy keeps to
cost.

Four things are going wrong and can be fixed where they stand.

1. The exact oracle is on a run-time path. Bringing a host integer into a value and taking it out
   again goes through 128-bit arithmetic. In this audit's ad-hoc spike on one x86 host that is eight
   to nine nanoseconds against about one for the host's own clamp when the build has no link-time
   optimisation, and a wrap that stays seven times slower with it. The sum itself is the host's sum.
   The product, the quotient and conversion arrive at the value tier next, and the easy way to write
   each is to call the oracle.
2. The benches do not measure the tree. Seven months of bench work is 49 registered benches over
   94 variant crates and 254 committed result sets, none of which touches a shipped type, all timed
   on one Apple M1. The one family re-run on
   an x86 host turned its sign. The registry's rows about cost rest on that.
3. The plan measures last. Value benches are step 19 of 20 and the port of the one real consumer is
   step 20. Everything this audit learned by running something was available the day the value type
   landed.
4. The root documents and the instructions every agent session reads describe a tree two crates
   smaller, with strategy markers that drive code generation. They do not.

The process produced more record than product: 184 design rounds and a registry of nearly 800 rows
against five small crates, with September spent on 71 rounds of edits to one design document. The
registry itself is good, more honest about what is known than any design record I can compare it to.
The finding is about order of work, not about the instrument: build, bench and port alongside the
canon, as the last two days did.

For hilavitkutin and kaski, arvo is going the right way and both need the same next things:
subtraction and the product on values, a cheap door in and out at the container's width, ordering, and
benches of the value tier on more than one host. hilavitkutin's source unwrapped the old arvo's
integer in every hot loop, which is the strongest evidence that the doors matter more than the
families. kaski asks for half to even on the product; in the spike that costs twice a floor's latency
in exactly the dependent chains a simulation runs, and a half-up arm costs what a floor costs.

For generic applications and server daemons, which op asks to be assumed, the tree's own documents say
they are out of scope, the build needs a nightly compiler through notko's features, and printing,
parsing, bytes, ordering and defaults are unbuilt. arvo has something for them nobody else offers
together: decimal scaled integers and exact widths with declared rounding and overflow. Whether to
offer it is the one question in this audit that is about intent, and it is op's.

The best parts, to be kept as they are: the format spine and its six rounding names, behaviour per
declared signature, the debug check as a type parameter, admission by failed constant, the exhaustive
suites with an independent oracle and planted mutants, the saturating-fold bench family, the
registry's retirements and probe standings, and the README's honesty.

## How this report is laid out

The report is five files, each under the repository's file-size limit.

| File | What it holds |
|---|---|
| `AUDIT.md` | This summary, the verdicts in one table, what was read, what each pass changed, the tickets |
| `DESIGNS.md` | The concept, each crate's design, the root documents, the canon and the way of working |
| `NUMBERS.md` | Why each bench and spike on record reads as it does, and the estimates against other ways of writing the same thing |
| `BENCH-CATALOGUE.md` | The benches worth writing, in order |
| `DOWNSTREAM.md` | hilavitkutin, kaski, the stand-ins, a generic application, a server daemon, notko |

The audit's own measurement is at `mock/research/sketches/202611131300_stack-audit-value-yardsticks/`.
It is an ad-hoc spike on one shared virtual machine, outside the bench harness, and every number from
it in these files is called that.

Every part carries one of four answers (passes in intent and needs only implementation; can be fixed
in place; going wrong; back to the drawing board) and one of four marks in words (good as it is; easy
to get wrong but can be right; going wrong; back to the drawing board). The two scales mostly agree
and are both given because the brief asks for both.

## The verdicts in one table

| Part | Answer | Mark |
|---|---|---|
| Format as domain and set, operation as exact step and adaptation | Passes in intent | Good as it is |
| Floats as points of the same shape | Unproven | Easy to get wrong but can be right |
| Behaviour per declared signature, placement apart | Passes in intent | Good as it is |
| Never a run-time check, debug abort by type parameter | Passes in intent | Easy to get wrong but can be right, at untrusted input |
| Arms over regions | Passes in intent, least built | Easy to get wrong but can be right |
| Strategy as an intent decided by measurement | Passes in intent | Good as it is |
| Strategy bound to an adaptation, as shipped | Back to the drawing board on that binding | Going wrong |
| Parity with the standards | Passes in intent, IEEE half unbuilt | Easy to get wrong but can be right |
| `arvo-format` as an oracle | Passes | Good as it is |
| `arvo-format` on a run-time path, and its design document's length | Can be fixed in place | Going wrong |
| `arvo-placement` at sole occupancy | Passes | Good as it is |
| `arvo-placement` at shared occupancy | Needs implementation | Easy to get wrong but can be right |
| `arvo-numeral` construction, admission, the sum | Passes | Good as it is |
| `arvo-numeral` doors, hash, mutant features | Can be fixed in place | Going wrong |
| `arvo-numeral` traits, default adaptation, layout promise | Can be fixed in place | Easy to get wrong but can be right |
| `arvo-bits` | Can be fixed in place | Going wrong as something a table holds many of |
| Root documents and agent instructions | Can be fixed in place | Going wrong |
| The registry as an instrument | Passes | Good as it is |
| Order of work: measuring and porting last | Can be fixed in place | Going wrong |
| The bench corpus as evidence about arvo | Can be fixed in place | Going wrong |
| The saturating-fold family | Passes | Good as it is |
| Fit to hilavitkutin | Needs implementation, in another order | Easy to get wrong but can be right |
| Fit to kaski | Needs implementation | Easy to get wrong but can be right |
| Fit to generic applications and daemons | Intent has two readings | Easy to get wrong but can be right |

## What was read, and to what depth

Read whole by me: the brief; the README; `mock/PRINCIPLES.md.tmpl` and `mock/DESIGN.md.tmpl`; the
design documents of `arvo-numeral`, `arvo-placement`, `arvo-strategy` and `arvo-bits`; the source of
`arvo-numeral` (`num.rs`, `alias.rs`, `rung.rs`, the manifest); the public source of `arvo-placement`,
`arvo-strategy` and `arvo-bits`; all seventeen obligation rows; the canon check of 13 November
(`CHECK.md`, `PLAN.md`); `mock/benches/bench.toml`; the six reader notes named below.

Read in part by me: the `arvo-format` design document, lines 1 to about 900 of 2122; `arvo-format`
source (`lib.rs`, `standards.rs`, parts of `apply.rs`, `slots.rs`, `addition.rs`, `multiplication.rs`,
`division.rs`, `conversion.rs`); the ruling rows, those carrying op's words on the strategy axis, the
run-time check, the arms, the defaults and the standards; findings files of the bench families cited
in `NUMBERS.md`, each opened for the numbers quoted; kaski's `fixed.rs`; notko's feature list.

Read through sonnet readers, whose notes I read whole and whose claims are marked as theirs where I
did not open the evidence: the tail of the format design and the format suite; the 116 probe rows and
the sketches; the bench corpus variant by variant; the registry's counts by namespace and rung, and
the pairs of rows that pull against each other; the research tree and the design rounds by date and
scope; hilavitkutin, kaski, kirjo, lato, riimu and notko.

Not read at all: the 159 proposal rows and the 196 retirement rows one by one; the 110 question rows
beyond those the designs cite; the 184 design rounds beyond their dates and scope lines; most of
`mock/research/` before November; vehje, kolli and tarina; the lint sources under `mock/lints/`; the
mockspace bench framework itself; the sibling session's hilavitkutin review.

Run by me: `cargo test -p arvo-format` in a debug build (297 pass, one fails, ticket `eae41fda0fd6`);
the spike. Nothing else was built or run, and no bench in `mock/benches/` was re-run.

## What each pass changed

Pass one, this text as first committed, is the reading above and the judgments drawn from it.

Later passes are recorded here as they finish.

## Tickets

Filed or bumped by this session, in the workspace ticket store.

| Id | About |
|---|---|
| `eae41fda0fd6` | The inverse round-trip test in `arvo-format` fails on `dev` |
| `fdbc079d46a7` | The pre-commit hook rewrites the consumer crate's lockfile |
| `707a2a63027a` | The value-add disassembly sketch is stale against the source |
| `684a26a63155` | Six markers in `arvo-strategy` and `arvo-format` name no ticket |
| `fa0b1d97df50` | The saturation guard counts sub-agents that already handed back |
| `ad67b351e8d9`, `1e8626a64f44` | The stand-up assumes kaski and a coordinator line for a helper elsewhere |

The findings of this report are not themselves tickets. They are for op and the coordinator to accept
or refuse first, and a ticket for a refused finding would be noise.
