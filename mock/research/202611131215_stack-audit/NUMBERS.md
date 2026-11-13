# The Numbers On Record, Why They Are What They Are, And What To Expect

Part of the stack audit; `AUDIT.md` beside this file is the entry and carries the summary and the
reading ledger. This file takes every bench family and every timing spike arvo has committed, says what
each one measured as against what it is cited for, gives the mechanism behind the number where one can
be shown and says "not known" where it cannot, and then estimates how the designs will stand against
hand-written code, plain Rust and comparable libraries.

Three rules hold throughout. A committed result is named by its file. Anything this audit ran itself
is an ad-hoc spike and is called that every time; all of it is in
`mock/research/sketches/202611131300_stack-audit-value-yardsticks/`. An estimate is a piece of reasoning
with its assumptions written beside it, and is never written as a measurement. Paths and line numbers
are at `dev` tip `326b52f5`.

Each part carries one of four marks in words: good as it is; easy to get wrong but can be right; going
wrong; back to the drawing board.

## What every committed number shares

Before any family is read, five facts about the corpus under `mock/benches/` decide how much weight a
number in it can carry. They are stated once here and not repeated per family.

Every one of the 254 result regions was taken on one Apple M1 under one toolchain, and all but one
carry a git hash ending in `-dirty`, so no result can be tied to a tree
(`mock/benches/bitpack-carrier-width_n4194304.meta.json` is an example, hash `fbac19c-dirty`; the count
is a reader's sweep over `*.meta.json` that I checked on four files). No M1 meta file records the build
profile, which the registry already knows as `probe::no_bench_artifact_records_its_build_profile`. The
harness now writes the field: the x86 rerun under
`mock/research/202611130200_bench-harness-at-head/results-x86_64-xeon/` records fat LTO and one codegen
unit.

The timing block is four passes of ten samples, each sample the mean of a hundred calls, with no
cooldown (`mock/benches/bench.toml:160-165`), and the findings files flag lag-one autocorrelation of
0.5 to 0.8 on the long rows. The instruction and cycle columns are zero in every file, so nothing in
the corpus is a counted instruction or a counted cycle.

The same-code control arm is the honest noise floor, and it is not small: in the threaded rows two
copies of one function differ by seven to nine percent
(`mock/benches/bitpack-wide_n167772164_findings.md`, 628794 ns against 691162 ns). A difference under
about ten percent in a threaded row, or under about one percent in a single-thread row, says nothing.

175 of the 254 regions were run before the driver validated that the arms compute the same answer
(`probe::most_committed_bench_regions_predate_the_validation_gate`), and one family is known to contain
an arm that had stopped doing the work.

And the fact that matters most for this audit: no committed bench measures a type that is in
`mock/crates/` today. Every arm is a standalone copy of a transform inside `mock/benches/variants/`,
written against a tree that was removed, or against no arvo type at all. Four sections (the two hashes,
structural decomposition, spectral bisection) depend on removed crates and cannot be built. So the
corpus is evidence about transforms and about the machine, and none of it is evidence that arvo as
shipped is fast or slow.

Mark for the corpus as an instrument: going wrong, and fixable in place. The repairs are known and
ticketed in `mock/research/202611130200_bench-harness-at-head/FINDINGS.md` (the host in the file name,
the profile in every meta, arms the harness builds itself); what is missing is a second and third host
and the counters. Mark for the corpus as evidence for the design: easy to get wrong but can be right,
family by family below.

## Family by family

### Packing against dense carriers, read sequentially

The carrier-width family sums a masked 13-bit column held four ways: 16, 32 and 64-bit dense, and
packed at 13 bits a member. On the M1 the packed arm costs about 49 percent more time than the 16-bit
dense arm at every size from 16384 to 8388608 members, equals the 64-bit arm at two million and beats
it by 10 to 13 percent from four million up
(`mock/benches/bitpack-carrier-width_n4194304_findings.md`: d16 365496 ns, d32 405220, d64 626364,
packed 542471, packed-simd 505631). On the x86 Xeon, same source and same compiler, the sign turns
over: packed is 17 to 38 percent faster than 16-bit dense at every size and fastest of all arms from
one million members up
(`mock/research/202611130200_bench-harness-at-head/results-x86_64-xeon/bitpack-carrier-width_n4194304_findings.md`:
d16 1548785 ns, d32 1186656, d64 1687751, packed 1046742).

Why the M1 number is what it is: the packed penalty is flat from a column that fits the first-level
cache to one far past the second, so it is the cost of the shift and mask per member and not a memory
effect. The 64-bit arm does not get faster with four threads while the 16-bit arm does
(`mock/benches/bitpack-contention_n41943041_findings.md` against `..._n41943044_findings.md`), so the
wide arm is bound by memory bandwidth and the narrow one by its own loop. That is a scaling argument,
with no counter behind it.

Why the x86 number is what it is: not known. The dense 16-bit arm being slower than the 32-bit arm on
that host points at the SSE2 widening chain the baseline compiles to, and the committed disassembly of
that arm is SSE2 unpack and add; nobody has read the loop. Host, instruction set and build profile all
differ between the two tables at once, so the pair cannot say which of the three turned the sign.

What it measures against what it is cited for: it measures a wrapping sum, the most memory-bound and
least arithmetic-heavy thing one can do to a column, read front to back. It is cited for whether the
footprint objective pays (`question::which_carrier_the_packing_claim_is_about`). It answers that for
one access pattern on one machine, and the second machine disagrees.

Mark: easy to get wrong but can be right. Right requires the table on at least three hosts with the
profile recorded, and the strongest dense arm as the baseline, which the next family shows this one
did not have.

### The dense baseline was weak

`contend-best` adds a 16-bit dense arm written with pairwise widening adds. It is 58 to 62 percent
faster than the 16-bit dense arm every earlier comparison used, at one thread and at four
(`mock/benches/bitpack-contend-best_n41943044_findings.md`: d16 129591 ns, d16-padal 54132, the best
packed arm 112916). With that arm in the table, dense beats every packed arm at every committed row at
four and eight million members on the M1.

Why: the plain loop widens each member before adding and the compiler vectorises it poorly; the
pairwise form keeps the lanes narrow until they must widen. That is read from the arm's design and the
ratio, and no disassembly is committed for it.

Mark: good as it is, as a correction, and it is the most useful result in the corpus because it shows
the method working: an arm was attacked and lost. What it changes downstream has not been carried
through: every statement in the registry that prices packing against the 16-bit carrier on these
tables is priced against the weak arm.

### Packing under index-driven and random access, and under a heavier kernel

The sequential-sum, random-sum, decoder-shape and kernel-amortisation families run at columns of 256
to 262144 members. Dense native is 58 to 79 percent faster than every packed decoder, and a
multiply-accumulate kernel on top does not shrink the gap
(`mock/benches/bitpack-kernel-amortisation_n262144_findings.md` and its siblings; these four I took
from a reader's tables and did not open myself). Random access exists only at 16384 members and under,
which is inside the first-level cache, so the one place packing is supposed to pay, a random read into
a column too big for the cache, has never been measured.

Mark: going wrong as a basis for the footprint objective, fixable in place. The benches are sound for
what they run; the region the objective lives in is absent, and `BENCH-CATALOGUE.md` carries it as
`packed-access-that-is-not-sequential` and `footprint-per-entity`.

### Writes into a packed column

Dense writes are 18 to 20 times faster than packed writes, and a parallel packed write with split
points off the packing period corrupts the column, which the validation gate caught and the arm was
dropped for (`mock/benches/bitpack-write-contend-safe_n20971522_findings.md`, reader's table, not
opened by me). The ratio is the read-modify-write encoder against a plain store.

Mark: good as it is. It prices the thing the placement design will have to say out loud once a shared
placement holds values: a packed column is cheap to read in order and expensive and unsafe to write in
parallel.

### The container fork and the clamp family

The container fork compares a carrier with one spare bit against the minimum carrier. At 13 and at 60
bits the two are indistinguishable, and both are three to twenty times slower than the arms that
project once at the end and fold in lanes
(`mock/benches/warm-container-width-l1_n130003_findings.md`: headroom 8387 ns, minimum 8591, native
397, kernel 826). So the family does not measure what its name says. What moves the number is whether
the mask is applied at every step or once, and whether the fold is serial.

The clamp family shows the same thing from the other side: at 13 bits and arity 256 the arm that
clamps at every step in a narrow accumulator takes 9789 ns and the arm that proves the interior cannot
leave the range and clamps once takes 190
(`mock/benches/warm-clamp-arity-w13_n130080_findings.md`), a factor of 51. That factor is a serial
chain against a vectorised sum.

One arm of the neighbouring `precise-container-width-l1` family reports 4 ns for 8192 members
(`mock/benches/precise-container-width-l1_n130103_findings.md`): a saturating accumulator that reached
its fixpoint and stopped depending on the input. In the same file the minimum carrier is a third
slower than the one with headroom, and why is not known.

Both families were run before validation was wired, apart from one clamp section rerun.

Mark: easy to get wrong but can be right. The finding inside them is real and is the most important
performance fact arvo has: per-step adaptation is where the time goes, and knowing statically that a
run of steps stays in range is worth one to two orders of magnitude. The families are cited for other
things (which container, wrap or clamp), and for those they carry little.

### The saturating fold

A saturating 8-bit fold over 32 KiB, by reduction length. At length 4096 the plain loop takes 41713 ns,
the same loop given the bounds proof but not the reassociation 31997, sixteen lanes with the law 1528,
sixty-four lanes 670, hand-written NEON 362 and 255
(`mock/benches/satfold-length-l1_n12000_findings.md`). A const gate that selects the fast arm at
compile time times the same as the fast arm written directly, and the gate set false times the same as
the plain loop (`mock/benches/satfold-const-gate_n10000_findings.md`: 1438 against 1456, and 38391
against 38741).

Why: the plain loop is a chain of saturating adds each waiting on the last, about 1.27 ns a member,
roughly four cycles on that core. The law (for members that cannot go below zero, a saturating fold is
the exact sum clamped once) removes the chain and lets the lanes run. The proof of bounds alone buys 23
percent and the reassociation buys the rest.

This family ran with validation wired and has controls on both sides. It is the best bench in the tree.

Mark: good as it is. It is the committed evidence that the heart of the design, an arm that holds over
a nameable region and is selected by a const predicate, costs nothing to select and is worth a factor
of 27 to 160 where it applies. It is also M1 only, and the hand-written arm is aarch64 only, so the
family cannot be built on x86 at all.

### The quantiser, the decimal sweep, the wide rung, the hashes and the graph benches

The quantiser family reports the hardware float add as 16.5 times faster than a software round-first
quantiser. Both arms are functions marked never-inline and called once per member
(`mock/benches/variants/quantiser-fadd-shared/src/lib.rs:147` and `:163`), so the hardware figure is a
call, a load, an add and a store, and the ratio is against a baseline that was deliberately kept from
vectorising. As a statement about what a software float costs beside the hardware one it is a lower
bound on the gap, since a vectorised hardware loop would be several times faster again.

The wide-rung family was built to test a ratified payload rule and retired it: the byte difference
never turned into time, which the registry records as
`retirement::r22_r1_the_wide_rung_item_14_instruction_and_byte_claim`. The one large effect, at 129
bits, has no known mechanism.

The hash and graph benches time code that no longer exists. The graph ones have a single arm, so they
compare nothing.

Marks: the quantiser bench is easy to get wrong but can be right, and right requires both arms inlined
into a loop the compiler is free to vectorise. The wide-rung family is good as it is, as a retirement.
The hash and graph result files are back to the drawing board in the plain sense that there is nothing
to repair: they describe a removed tree and belong with the returning families' own rounds.

### The timing that sits in sketches

Two sketches before this audit carry a number. `mock/research/sketches/202611130342_value-add-disassembly/`
reads the assembly of the value's sum and is not a timing; it found the sum equal to the host's at full
width and an add and a mask, or an add, a compare and a select, at 13 bits in 16, and on a first
reading found 24 calls into `arvo-format` accessors because none of them is marked inline. Its source
no longer builds against the tree, since it imports two names `arvo-numeral` has since dropped; that is
ticket `707a2a63027a`. One sketch from September carries a single timer and labels it ad hoc.

The registry's `probe` namespace is where a measurement is supposed to be anchored, and a reader's
sweep found that it cites no sketch directory at all and that eight probe rows stand as uncontrolled,
two as defective and one as withdrawn (reader's reading of `mock/registry/probe.toml`; I did not open
the rows).

Mark: easy to get wrong but can be right. The disassembly reading is the right instrument for the
claim it supports and the design says so itself (`mock/crates/arvo-numeral/DESIGN.md.tmpl:344-346`).
What right requires is the harness being able to commit the disassembly of a named function, so that
the reading is rebuilt when the code moves, which is already a ticket in the harness findings.

## What this audit ran: the value's doors, its fold, and a hand product

Everything in this section is an ad-hoc spike on one shared x86_64 virtual machine, default target
features, median of 2001 runs with no pinning and no noise gate. `FINDINGS.md` in the sketch directory
has the tables and what they do not show. Three results matter here.

The sum is the host's sum. A fold and an elementwise sum through `Num` time within two percent of the
hand loop over `u16`, saturating and wrapping, at 13 bits and at 16, with fat LTO and without it, and
the library's assembly has no call in any of them.

The doors are not. The only public way to bring a run-time host integer into a value below pointer
width is `Num::adapted` over an `Exact` built from a `Slot`
(`mock/crates/arvo-numeral/src/num.rs:126-136`), and the only way out is `Num::slot`
(`:140-157`), which answers in a 128-bit index. Without LTO each ingest is five calls across the crate
boundary and each egress two, and the spike timed an ingest at seven to eight times a hand mask or
clamp behind the same call overhead. With fat LTO the saturating ingest folds to the hand code and the
wrapping ingest stays about seven times slower; the likely reason is the three 128-bit euclidean
remainders in the wrap completion (`mock/crates/arvo-format/src/apply.rs`, the wrap arm of
`complete_slot`), which was not confirmed in the binary.

A fixed-point product costs about the same in every rounding mode as throughput and twice as much in
latency for the modes that test a tie or a sign: on a chain, floor 1.5 ns a step, half up 1.7, toward
zero 3.2, half to even 3.1. The same chain shows floor drifting 0.12 percent low after thirty thousand
steps of a decay while half up and half to even stay within one part in ten million of the real
answer.

## Estimates

Each row says what is compared, the estimate, and what it rests on. "Spike" means the ad-hoc spike
above. "Reasoned" means no number stands behind it beyond the ones named.

| What is compared | Estimate | Rests on |
|---|---|---|
| `Num::add` at the carrier's full width, against hand assembly, against `wrapping_add` and `saturating_add`, against `core::num::Wrapping` and `Saturating` | The same instruction, ratio 1.0, in any profile | The committed disassembly sketch, and the spike's folds at 16 bits |
| `Num::add` below the carrier's width (13 in 16), against a hand mask or clamp in Rust | Ratio 1.0 per sum | Spike |
| A wrapping fold below the carrier's width, against hand assembly that masks once at the end | About 3 times slower at SSE2 with fat LTO, about 10 times without, until something above the value may defer the mask | Spike: 0.098 ns against 0.031, and 0.350 against 0.034, with the hand per-step loop equally slow |
| A saturating fold of members that cannot go negative, against hand-written vector code using the clamp-once law | 27 to 160 times slower as the value ships, on the committed M1 table; equal once the fold is an arm with the law | `satfold-length-l1_n12000`: 41713 ns for the plain loop, 1528 for sixteen lanes, 255 for hand NEON. The value's `add` in a loop is the plain loop |
| Bringing a run-time integer into a value and out again, against a mask, a clamp or a cast | Equal under fat LTO for saturation; about 7 times slower for wrap; 7 to 8 times slower for both without LTO | Spike |
| The same doors against `core::num::Wrapping(x)` or a newtype with a public field, which is what hilavitkutin does today with `USize(a.0 + b.0)` | Those are free in every profile; the doors are not | Reasoned from the spike and from `hilavitkutin/mock/crates/hilavitkutin/src/dispatch/morsel.rs:89-103` |
| A fixed-point product through a value, once written as a widening multiply and a shift on the carrier, against the hand product | Ratio 1.0 for floor; for half to even about equal as throughput and about twice the latency of floor on a chain | Spike, on hand code. The value has no product, so this is what the hand side costs and what the value must match |
| The same product if it were routed through the format's exact oracle at run time, as the doors are | 10 to 50 times slower than the hand product | Reasoned: the oracle forms the product in four 64-bit limbs and divides, and the far simpler ingest already costs 9 ns unlinked. Nothing has built it |
| Fixed point against `f32` on a multiply over a column | About 9 times slower at SSE2 for a 32-bit carrier, because the float loop vectorises and the widening multiply does not; expected within 2 times where a vector widening multiply exists | Spike for the first half. The second half is reasoned and unmeasured |
| `Hash` on a value against the host integer | Up to twice the hashing work for a value of 64 bits or less, more for a byte | Reasoned: the impl hashes the 128-bit slot index (`mock/crates/arvo-numeral/src/num.rs:296-300`), so every value feeds sixteen bytes |
| `Bits<N>` held in a table against a field of the host's narrowest integer | Two to eight times the footprint for `N` of 32 or less, with the masking itself free | Reasoned from the one carrier, `u64` (`mock/crates/arvo-bits/src/lib.rs:48-50`) |
| A packed column read in order, against the best dense loop | 2 to 3.5 times slower on the M1 while both fit the cache or the memory system keeps up; ahead only against a carrier four times wider than needed, past about four million members | `bitpack-contend-best_n41943044`, `bitpack-carrier-width_n4194304` |
| The same on x86 at SSE2 | Not estimated. The one table says packed is ahead and the baseline there has not been attacked | The Xeon rerun |
| A debug build with the `Abort` check, against a debug build of host arithmetic with overflow checks | Slower by a small integer factor per sum | Reasoned: `fits_add` goes to 128-bit slot indices and a range test (`num.rs:208-210`) where the host tests a flag. Unmeasured, and a row in the catalogue |

Against named libraries the comparison is mostly of shape, since nothing of arvo's has been timed
beside any of them. None of the statements about another library below was checked against that
library's documentation in this audit, and each is from memory.

The `fixed` crate holds binary fixed point in a primitive carrier with the fraction count in the type,
and offers each operation in wrapping, saturating and checked spellings. It is the closest thing to
what kaski's carrier does by hand. arvo's value tier, once it has a product, should reach the same code
on the same carrier, and offers what `fixed` does not: a declared rounding mode per signature, widths
that are not the carrier's, scaled integers at another radix, and a verdict on which laws survive. What
`fixed` has today and arvo does not is the entire surface: the product, the quotient, conversion,
formatting, parsing, the transcendental functions, and operator traits.

Crates that offer integers of arbitrary width inside a wider carrier (the `ux` and `arbitrary-int`
family) mask after each operation as `Num` does, and expose a constructor from the host integer that
compiles to a mask or a compare. On the sum arvo is level with them by construction. On the way in and
out it is behind them, by the doors above.

Bit containers such as `bitvec`, and the fixed-size set crates, are what
`obligation::set_operations_over_a_fixed_size_bit_set` will be read against. `Bits<N>` has no set
operation at all today, so there is nothing to estimate; the obligation names its own yardstick, one
instruction per operation for a set of 64 or fewer.

MATLAB `fi` is a parity target and not a speed one. The parity file asserts stored integers against
values written by hand from the vendor's documentation (`mock/crates/arvo-format/tests/matlab_fi_parity.rs`),
which is a reading of a document checked against the oracle, and is not yet the thing op asked for in
`obligation::every_standard_convention_expressible_as_an_alias_over_the_primitives`: the same
computation run through both and compared.
