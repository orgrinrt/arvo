# 19. Persona checkpoint on the third stretch

**Date:** 2026-08-08, overnight. **Author:** the `orgrinrt` persona, standing in while op sleeps.
**Status:** PERSONA. No authority. Nothing here ratifies, nothing here settles, nothing here is op's
word. Per `04` and `RULES.md:41-45`, every call below is the persona's and is logged as such in
`PERSONA_CALLS.md`.

Where I am guessing at what op would think rather than reading it in the record, I say so inline with
the word **guessing**, and section 11 collects every instance. Where op's recorded words cut against
my instinct, his govern and I write the conflict down rather than resolving it.

**What I read.** `04`, `01`, `RULES.md`, `PERSONA_CALLS.md`, `14` (my predecessor) and `09`'s entry in
`PERSONA_CALLS.md`, then `15`, `16`, `17`, `18` in full in that order, then `MORNING.md` line by line
against the four. `SETTLED.md` at the passages a claim sent me to, plus `seed/SETTLED_container.md` at
one passage that turned out to matter more than anything else in this file. Section 12 says which
probe outputs I opened, which numbers I reproduced with my own commands, and what I did not do.

This checkpoint went outside the panel directory, which neither predecessor did. That is where its
largest finding is, and it is why section 1 leads with something none of the four files says.

## 1. What I would put in front of op, and none of it is what MORNING leads with

Three, in this order.

### 1.1 Op already gave a second standing instruction, it is the same shape as the fresh-eyes one, and eighteen files have walked past it

`09` found that op's fresh-eyes instruction on the container derivation had gone untaken by four
dispatches. `10` took it and it was the best single result of the second stretch. There is a second
instruction of exactly that shape sitting in the seed material, and after eighteen files nobody has
taken it either.

`seed/SETTLED_container.md:410-420`, quoting op:

> A further deletion proposal (`140`, delete headroom for every strategy) was neither accepted nor
> rejected: op held it explicitly pending harness benches, "hold calls on it until there are actual
> benches" (`140b:69-71`). Harness benches then arrived (`141`, in `mock/benches/`, real competitor
> arms) recommending deletion, but op's next word on it treats the whole body of `141`/`142` bench
> work as **"one instance of evidence, completely unaudited by a second expert set of eyes"**
> (`142b:60-64`), and no later checkpoint records a ruling.

That is an instruction with the same grammar as the one `10` discharged. Op asked for a second expert
set of eyes on a specific artifact, the artifact exists, and it has not been read.

**And this panel cannot see it, because the compression dropped it.** `SETTLED.md`'s open list runs
from line 137 to line 178. I read it in full. The headroom thread is not in it. The `141`/`142` bench
body is not in it. Op's "unaudited by a second expert set of eyes" is not in it. `grep -ni headroom
SETTLED.md` returns nothing. The only headroom-adjacent row that survived the compression is
`SETTLED.md:83`, which carries the unrelated "Warm is what Rust does" ruling.

So this is the **third** compression defect found in `SETTLED.md` tonight, after the container
"either" that `10` corrected and the ergonomics-bar qualifier that `12` spent an hour on. It is the
largest of the three, because the first two dropped a qualifier and this one drops a live open item
plus a standing instruction. `MORNING.md:441-449` and `:501-507` both note that the theme sweep
underneath had it right and the compression did not. Same again, third time, and this one has cost the
whole night.

**Guessing**, and marked: I think op would be sharper about this than about anything else in the
stretch, because he answered the question once already. `01` section 3, on being asked about the
headroom rule at all:

> This is no conflict or case to bring up to me. I was very clear. At some point, somebody has to be
> confident enough about their take on it to write the benches, and once benches exist, it's hard to
> deny what they tell, and if it isn't, then the thing is still settling and there's no reason to rule
> on it

The benches exist. They were committed on 2026-08-07, the day this panel opened, at commit `ccf0509`,
"panel file 141, the container fork benched on the harness": nine bench sections, 653 lines added to
`bench.toml`, six arm crates plus a shared transform crate, 57 committed CSV plus meta plus findings
triples. `141_xu_the_container_fork_benched.md:11` states that inventory itself.

One file in this panel mentions `141`. `10:190` mentions it in one clause about the headroom thread
and reads it through `SETTLED_container.md`'s "unsettled at panel's end" rather than through `141`'s
own verdict. No file in the panel names `mock/benches/`. `grep -rniE 'mock/benches|benches/' *.md`
over the panel returns one hit, and it is `RULES.md:120` quoting the workspace rule about what a bench
is.

### 1.2 The trade the panel calls unpriced is priced, in this repository, and the number runs against the design's flagship strategy

`17:552-576` is the finding it did not go looking for: the packed walk does not vectorise and the
byte-aligned walk does, 51 instructions with 1 SIMD-shaped opcode against 92 with 83. `17` then does
the correct thing and refuses to put a number on it:

> Every packed-storage figure in `15` and `16` is a storage figure. Nobody has measured the walk, and
> until a bench harness runs, the sentence "packed storage saves 23.1%" is half of a trade whose other
> half is unmeasured.

`MORNING.md:236-242` carries that faithfully. Both are correct that no bench harness has run **in this
panel**. Both are wrong about the state of the question, and the gap between those two sentences is
the whole problem.

`mock/benches/bitpack-sequential-sum` is the arm. Three variants, committed, on the mockspace harness,
with CSV, meta and findings triples at three sizes:

| variant | what it walks | median at n=16384 |
|---|---|---|
| `bitpack-native-seq` | a native `[u16; N]` carrier array | 1667 ns |
| `bitpack-aligned-seq` | byte-aligned slots in a byte buffer | 5570 ns |
| `bitpack-zeropad-seq` | the bit-packed form, no inter-value padding | 7679 ns |

Read off `bitpack-sequential-sum_n16384_findings.md`. The packed walk is **4.6x the native walk** and
38% slower than byte-aligned slots, significant, CI excluding zero, winning 0 of 40 passes. The three
variant sources are 25 lines each at `variants/bitpack-{native,aligned,zeropad}-seq/src/lib.rs` and
they are the same three-arm comparison `17`'s `t2` built by hand, with warmups, cooldowns, forty
samples, bootstrap CIs and an artifact trail.

**And the direction turns with scale, which is the part that actually matters and which nobody has
stated.** At 7,000,000 elements, past L2 on this host:

- `bitpack-footprint-dense_n7000000_findings.md`: dense at **810 us** median.
- `bitpack-footprint-packed_n7000000_findings.md`: packed at **1157 us** median.

Same machine (Apple M1), same pin (`1.98.0-nightly (57d06900f 2026-05-27)`), same harness, runs 529
seconds apart by their `meta.json` timestamps. That is roughly **1.43x**, against 4.6x at n=16384.

**This last comparison is mine and it is weaker than the others, and I am marking it as such.** The
harness computed no statistic across those two entries, because they are two bench sections rather
than one. Two things also differ between the arms besides the layout: `bitpack-footprint-packed` is
the plan-driven decoder and `bitpack-zeropad-seq` is index-driven, and
`bitpack-decoder-shape_n262144_findings.md` measures that difference separately at 133.84 us
index-driven against 29.21 us plan-driven. So the honest statement is:

> The packed walk costs roughly 4.6x when read element by element at L1 sizes and roughly 1.4x when
> read through the plan-driven decoder at 7M elements. Both ends are on the harness with committed
> artifacts. The single bench that would settle it, dense against packed at matched n across the
> sweep, does not exist and is a `bench.toml` edit rather than new variants.

That is a far better thing to hand op than "unpriced", and it is better in the direction that costs
the design something, which is the direction to prefer.

**Why it matters more than a number.** `arvo-toolbox-not-policer.md` states that `Cold` "is the reason
arvo exists" and grounds the claim on consumers whose "access patterns are predictable, sequential
where possible, cache-friendly by construction". Sequential and cache-friendly is precisely the access
pattern where the vectoriser fires, and it is therefore precisely where packing costs the most. The
rule and the measurement are not in contradiction, because the rule's currency is gigabytes of RAM and
the measurement's is throughput. But nobody has ever put the two sentences next to each other, and the
the rule forbids arvo ruling on it, and op's own instruction says the benches decide. The benches
are half built.

### 1.3 The panel has been auditing its own instruments for a stretch and has not audited the one op asked about

Put 1.1 and 1.2 together and the shape of the night is uncomfortable.

`17` was dispatched to "separate what is proved from what is trusted" and it built a defect matrix, a
validation-direction oracle and a rustdoc perimeter instrument. Good work, and section 5.3 says so.
Its section 6 is a nine-item trusted-base list, which it names as its deliverable. Item 6 of that list
reads "That the programs sampled for operation erasure are representative of the programs consumers
write."

The panel's own bench evidence, sitting in `mock/benches/`, is a body of measurement op has explicitly
flagged as one unaudited instance and asked for a second set of eyes on. It does not appear on the
trusted-base list. It does not appear anywhere in `17`. A file whose subject is enumerating what the
design trusts without checking has omitted the one item the lead designer personally named as trusted
without checking.

That is not a criticism of `17`, which read what its brief and `RULES.md:171-173`'s curated list gave
it. It is a statement about what the curation dropped, and it is the same mechanism as 1.1.

## 2. Were `14`'s findings acted on, and what `PERSONA_CALLS.md` says about it

`14:576-580` found that `PERSONA_CALLS.md` was inaccurate about `09`: it said "nothing yet ... were
not applied", with an mtime of 02:30, while commits `5de4d51` (02:32:25) and `cf9a55c` (02:32:58) had
applied the fixes within three minutes.

**The same thing happened again, and the file records it the same way again.**

`14` was committed at `8c12df3`, 04:47:00. Then:

- `30fef51`, 04:47:40, "correct the overselling lead and name the two gaps nobody joined", 33 insertions and 13 deletions to `MORNING.md`.
- `60ff1e3`, 04:49:39, "anchor the morning numbers and mark one claim as reproducible not archived", 18 insertions and 9 deletions to `MORNING.md` plus a new `11_probes/typenum_extract/COUNTS.md`.

Forty seconds and two minutes thirty-nine. `PERSONA_CALLS.md:109-110` says of `14`: "What was done
with it: nothing at the time of writing. Delivered for op's morning. The recommended `MORNING.md` edits
are recommendations and were not applied by the persona."

The added clause "by the persona" makes the sentence technically true and leaves the reader with the
same wrong picture, one stretch after the identical wrong picture was pointed out in that very file.
The file whose entire job is keeping the persona's calls distinguishable from op's is now understating
the persona's effect for the second consecutive stretch, after being told about the first.

**Which of `14`'s findings actually landed.** I checked each.

| `14` finding | State now |
|---|---|
| lead paragraph oversells, "a choice between compiled costs" | **Fixed.** `MORNING.md:51-73` now says "a type-level skeleton, not a design" and names the correction as the checkpoint's. |
| the table survives every candidate, lead does not say so | **Fixed.** `MORNING.md:62-64`. |
| `MORNING.md` never says "strategy" | **Fixed.** `MORNING.md:59-61` names it and says the document did not contain the word. |
| the negative-width collision, unjoined | **Fixed and then closed by `15`.** `MORNING.md:66-70`. |
| the standing-instruction self-contradiction at `:176` against `:447` | **Fixed.** `MORNING.md:762` now opens "had been walked past by four dispatches, and `10` then took it. The account below is the state before `10` ran." |
| typenum counts rest on `~/.cargo`, not in the repo | **Fixed.** `11_probes/typenum_extract/COUNTS.md` committed by `60ff1e3`, and `MORNING.md:366` notes the re-run. |
| zero `file:line` anchors and zero probe names | **Half fixed.** See 5.2. |
| `MORNING.md:353` merges the two-expert rung with the three-instance bar | **Not fixed.** See 5.1. |
| the 81-versus-zero discrepancy | **Not fixed.** Fourth stretch. |
| five-arrangement menu under "what is yours" | **Fixed by `15`, not by the dispatcher.** `15:694-695` declines to add to it and `MORNING.md:123-131` now carries one question. |
| dense-table residue keeping its timing table | **Not addressed.** The number is gone from `MORNING.md` but the residue is unmarked in `10`. |
| base ten as a live direction | **Effectively dropped.** No longer in `MORNING.md`. |
| `12`'s C2 and C3 as candidates | **Not addressed.** `MORNING.md:419` still says "All five candidate surfaces". |

Eight of thirteen landed within three minutes, which is a good rate and is the second consecutive time
the checkpoint mechanism has paid for itself. Three of the five that did not are the three that
`09` had already raised, which is the pattern worth naming: **the defects that get fixed are the ones
about the current stretch, and the ones that persist are the ones about the record.**

## 3. Per file

### 3.1 `15`

**Holds, and it is the best single result of the night by a distance.** It did what `10` did and did
it twice: broke its own brief first, then went at a gap by changing coordinates rather than by paying
for it.

`15:39-42` catches the brief quoting `06` one clause short, and the missing clause changes the gap
from "can the arrangements carry a shape they must carry" to "what does carrying it cost, against what
clamping costs". `15:57-69` shows the work. That is a brief defect the dispatcher has not recorded
anywhere, and section 6 counts it.

**The finding, and it is canon-shaped.** `15:111-114`: the integer width is the only coordinate that
goes negative; total width and fraction width never do at either site with a caller. So key the numeral
on `(W, F)` and the entire negative-width corner is naturals. `15:118-131` then settles the case `06`
had marked untested: repeated squaring of `U<0,1>` drives the integer width to `-(2^k - 1)` while the
total width stays at 1. **The corner is unbounded in the panel's coordinates and constant in these.**
Compiled over the whole 81-shape box, 6400 plus 6561 assertions, zero features, with a negative
control at `q06_negctl.rs` that prints the offending shape back in naturals.

That sentence passes both of `RULES.md:79-83`'s tests. It survives a rewrite in another language, and
three teams told "the numeral is keyed on total width and fraction width" would converge. If anything
from this stretch reaches a canon I think it is that, and it is a stronger candidate than anything the
second stretch produced.

**Second, the two-output finding, and the way it was found is the point.** `15:341-364` reports being
wrong twice in the same way and pinning both with negative controls. `15:375-379` draws the
generalisation and it is the right one: a one-output derivation checks clean against `size_of` and
cannot see this class of error at all.

**Third, and it is the one place the panel's own discipline visibly paid.** `15:71-94` records a wrong
turn where instruments A and B shared a bug and agreed everywhere, and the cross-check caught it only
because B short-circuits a case A does not. "Two instruments that share a line of reasoning are one
instrument." That is `RULES.md:116-118`'s three-instance rule arriving from inside, and it is the
sentence I would carry into the rule file.

**Thin.** `15:665-667` says of itself what `13` said: no arithmetic through the values. Nobody has
built `Add` on numerals whose shapes differ, three stretches in. `15:669-672` did not build `Precise`
as anything but `Warm` with a different name, and `16` then shows that `Precise`'s semantics decides
whether the two-output shape is forced by arithmetic or only by the type system. That is a small
question with a large consequence and it is nobody's.

**Would push.** `15:686-690` names what the `(W, F)` change does to a `Display`, a const constructor, a
serialisation format or anything reflective as untouched. That is the consumer-visible half of the one
question `15` puts to op, and it is unbuilt, so op is being asked a taste question whose consequences
have been enumerated and not measured. The tag answers it for the diagnostic and for nothing else.

### 3.2 `16`

**Holds, and section 10.1 is the most creditable paragraph in three stretches.** It reads its own
criterion back at its own answer, finds its third component is an input travelling under a new name,
and corrects itself to `15`'s narrower and better count. Against interest, before anyone asked, with
the reason stated: "a canon sentence that says the derivation emits an extent invites an implementer to
store the width twice, and two copies of one fact drift." That is a design reason rather than a
concession.

**The finding I would keep.** `16:401-421`, the second-order blindness. A too-narrow load returns the
right answer whenever the truncated bits happened to be zero, so the catching check is green at 0 of
64 with values 0 to 63 and fails at 32 of 64 with the width filled. `16:418-421` generalises it past
the present question: any check on a packed representation needs data that exercises the high bits, and
a test written with counters and small literals exercises none of them. That is a permanent sentence
and it belongs wherever the workspace states its test discipline.

**Second, the perimeter argument at 6.** "Recoverable by arithmetic is not the same as available at
the type level", with `p5b_const_to_type.rs` refusing in three syntactic positions and its `.err`
committed. It closes the objection that would otherwise have kept the two-output question open, and it
closes it with a compiled refusal, which `17:664-670` independently classifies as the panel's strongest
evidence class.

**Third, `p7`.** It built the adversarial pair `15` said it could not decide, two wide payloads of
identical size 32 and identical stride 256 bits at align 1 and align 16, and settled that alignment
rides on the carrier. `15:553-556` named that as its residual doubt. Resolving a predecessor's stated
doubt by building the thing is exactly what a second read is for.

**Thin.** `16:545-549` names the signed packed case as its largest hole and it is untouched: every
width in every probe is unsigned. `17`'s `D4` then measures it at 144 defect instances, so the hole was
real and it took a third file to fill.

**Would push, on the downgrade.** Section 4.

### 3.3 `17`

**Holds where I checked it, and I checked the load-bearing part harder than `17` could.** Section 4
gives the detail. The short form: the aggregate false negative is real and the operands agree as well
as the opcodes, and the optimisation-level table reproduces exactly on my own runs.

**The structural finding, and it is the one I would carry into a canon.** `17:221-245`: clause four
decomposes into layout erasure (proved by construction, `repr(transparent)`), dispatch erasure (proved
by construction, conditional on the ban list), and operation erasure (validated at one point). Naming
that split says which part of the clause survives a compiler upgrade and which has to be re-checked
after one. That is intent rather than implementation and it passes the permanence test.

**Second, the requirement nobody had.** `17:346-354`, `D3`: the write procedure has to be adversarial,
not just the data. A writer spilling one bit into the next element is invisible to an ascending
whole-run write because the next write repairs the damage before anything reads it. Confirmed in the
matrix at zero observations for the two ascending procedures against 96 for the poisoned-buffer one.
`17:734-738` then explains why property-based testing cannot replace it: a generator over values
generates ascending writes forever. That is a real methodological result and it is new.

**Third, the tail-element finding.** `17:362-379`: a word-load reader of an exactly sized packed run
reads one byte past the buffer, at `W = 13, N = 1000` reading byte 1625 of a 1625-byte buffer. Found as
a control failure `17` could not explain away, and `17:373-374` names the line in `16`'s probe that
papers over it and says it is right for a probe and would hide it in shipping code. The canon-shaped
form is at `17:377-379` and it is correct: a packed run's allocation is not `ceil(N*W/8)` bytes if
elements are read by word load.

**Fourth, and it went back for it.** `17:408-459` names the C1 perimeter instrument as a gap, then
builds it, then builds the alias variant that exercises its own stated weakness, finds the instrument
reports clean, closes the gap, and keeps the failing version. `RULES.md:90-91` asks for exactly that
and this is the clearest instance of it in the panel.

**Thin, and it is the file's own frame.** Section 4.

**Would push, on two things.**

`17:252-253` says "Everything is under `17_probes/`. Nothing is committed, per the dispatch." Twenty-one
files are tracked under `17_probes/` and `git status --porcelain` on the panel directory is empty. The
sentence is false about its own state as committed, and under `RULES.md:108-110` a reader taking it at
face value concludes every claim in the file is void. It is a stale line rather than a defect in the
work, and it should not sit in the file that classifies claim reliability.

`17:589-595` presents as its own structural result a paragraph that is already a workspace rule.
`17`: "So the forbidden list is load-bearing for the transfer argument, and if either ban were relaxed,
every exhaustive check at a model width would stop establishing anything about the real widths."
`unstable-features.md:34-38`: "`specialization` and `TypeId` are what let a property checked at a small
model width transfer to the real widths ... with either available, a check at the model width
establishes nothing about the real one." Same claim, same two features, same consequence. `17` cites
that rule two sections later, unnamed, for the const-eval quadrupling at `17:741-744`, so it read it.
The rung on `17:589-595` is inherited, and the file's framing does not say so. This is the second
instance of that shape in two stretches, after `13:32-58` restating a `SETTLED.md` row it had been
handed.

### 3.4 `18`

**Holds, and its central move is the right one.** `18:45-50` read `07_probes/p5_postfixpoint_accumulator.py`
rather than `07`'s summary and found the inner loop is `acc = R(acc + p, VA)`. I opened the probe and
the line is at `p5:126`, elements drawn from `VP = {0,1,2,3}`, accumulator starting at `VA[0]`.
Additions only over non-negative elements. So `07`'s zero-failure result is quantified over a monotone
non-decreasing operation set and `07`'s prose does not say so. Section 4 checks the narrowing that
follows.

**The finding I would keep, and it is canon-shaped.** `18:432-436`: **a partitioning set denotation is
free, an overlapping one is not.** Measured at `p1`: point, cell and absorbing-top denotations all keep
the value-level order total at 120 of 120 pairs, while intervals are comparable at 42.05% falling to
35.45% one width up. That is why the design's own two quiet set denotations cost nothing and intervals
cost the order, and until this file it looked like an accident of which cases came up. Permanent, and
it survives three teams implementing independently.

**Second, the exclusion is defended on better grounds than `08` used.** `18:337-341`: `08` excludes
intervals because the construction sits above the numeral and needs nothing new; `18` excludes them
because the algebra sits above the numeral and needs a different order, which "survives the case where
someone finds a construction that does need something new." A reason that survives a counterexample to
the original reason is a stronger reason.

**Third, the reframing of `Precise` on `inexact`.** `18:471-485` gives an open question that has sat
since `145` a statement rather than a preference: a strategy that refuses on inexact is the strategy
that demands its data keep a point denotation. And `p6` measures the size of the demand: addition and
subtraction keep it on every in-range pair, multiplication and division do not, and at `U<4,4>` a
point-denotation strategy admits 4.60% of in-range divisions. `01` section 4 records op acking the arm
reading and then saying the base is too loose to settle it. It is now less loose by one measurement.

**Fourth, and it is the discipline I would hold up.** `18:52-57` records a defect in its own probe
rather than fixing it quietly: `p3_interval_laws.py` held the first argument fixed at the one interval
where `A - A == 0` is true and reported 136 successes; the corrected answer is 16 of 136, and the
original stays on disk with its output. That is the workspace's test gate applied to the author's own
work, by the author, unprompted.

**Thin.** `18:626-630` states it: every numeral is `U<1,1>` through `U<4,4>` and `I<1,2>`, nothing above
8 bits of logical width. And `18:640-644` is the most honest coverage sentence in the panel: seven of
its probes "share one author and one model, so per `RULES.md:116-118` they are one instance of evidence
wearing seven hats, not seven." Nobody else in eighteen files has said that about their own probe set,
and it is true of most of them.

**Would push.** `18:509-552` offers three readings of where the clause belongs and declines to choose,
which `04` requires. But `MORNING.md:308-316` then presents reading 4.1 as "`18`'s reading, offered as
one of three and the best supported", which is accurate, and `MORNING.md:10-30`'s question four then
folds the other two out of sight. The three readings are `18`'s deliverable and two of them do not
reach the map's front page.

## 4. Is the self-correction real, or is the stretch marking its own homework

The brief's suspicion is the right one to hold and it is not borne out, with one exception that runs
the other way. I checked each of the three self-corrections rather than accepting them.

### 4.1 `17`'s false-failure claim: it holds, and it holds more strongly than `17` argued

`17:461-509` claims the panel's erasure oracle reports NOT ERASED where erasure holds, at the aggregate
arity, and that it therefore cannot be repaired by pointing it at bigger programs. This is a strong
claim about the panel's only assembly-emitting instrument and it deserved checking rather than
accepting.

**I extracted both bodies from `17_probes/asm/t2_aggregate_erasure.s` and compared them instruction by
instruction, which is a check `17` did not run.** `17:726-732` concedes in its own section 9 that an
opcode multiset "is insensitive to operand values, so it would equate a shift by three with a shift by
four", and leaves that as a known weakness of its oracle. That weakness does not bite here, and I can
say so because I looked at the operands.

`t2_typed_sum` and `t2_handwritten_sum` are 67 lines each. Every immediate is identical across the
two: `#39`, `#996`, `#5`, `#0x4`, `#0x6`, `#0x7`, `#0x1fff`, `#52`, `#4`, the tail offsets 1618, 1620,
1621, 1622, 1623, and the three `ubfx` field specifications `#4,#13`, `#1,#13`, `#6,#13`. The only
differences are register allocation and the position of the three-instruction `and` group relative to
the `ldrh`/`add` group inside the loop body. Both are choices a scheduler is free to make.

**So the aggregate does erase, at the operand level and not merely at the opcode level, and the panel's
instrument says it does not.** `17`'s claim survives a harder check than `17` applied to it. That is
the opposite of marking your own homework.

**I reproduced the second regime independently.** Six `rustc` invocations at `-C opt-level=0,1,2` on
`15_probes/q12_erasure_asm.rs`, pin `nightly-2026-05-28`:

- At `-O0` and `-O1` the assembler emits **no `_q12_* = _q12_*` aliases at all**.
- At `-O2` all three appear: `_q12_arvo_hot = _q12_arvo_cold`, `_q12_native_i16 = _q12_arvo_signed`, `_q12_native_u16 = _q12_arvo_cold`.
- Opcode multisets at `-O0`: every one of the five bodies is 14 instructions, and `arvo_cold`, `native_u16` and `arvo_hot` are byte-for-byte the same multiset. At `-O1`: 3 instructions, `and`/`madd`/`ret` for the unsigned trio and `madd`/`ret`/`sxth` for the signed pair.

`17`'s table at `17:522-527` is exact. Erasure holds at every level including zero; the instrument
fails below `-O2`. Reproduced, by me, with my own commands.

**The one thing `17` overstates, and it is a framing rather than a result.** `MORNING.md:212-219`
carries "the oracle does not go quiet. It reports failure" and calls it "a different and harder problem
than the one `16` found". Harder to repair, yes. Worse for the design, no, and the two get merged.
`16`'s finding is a green result over a derivation that is wrong, which is the direction that certifies
something untrue. `17`'s two regimes are red results where the thing is right, which costs effort and
never certifies anything false. `17:533-536` gets this exactly right in its own words ("those point
opposite ways and both are true") and `MORNING` files them as one indictment. Section 5.4.

**One thing `17` asserts on its own oracle's word.** `17:504` reads "So the aggregate erases, and the
panel's oracle says it does not." The only instrument saying the aggregate erases is `17`'s opcode
multiset, and `17:726-732` says that is not a correct erasure oracle. `t3_opcode_oracle.out`'s own
closing text is more careful: "the two oracles DISAGREE at the aggregate, and the disagreement is not a
tie." The probe's wording is right and the file's prose upgrades a disagreement into a verdict. It
happens to be the correct verdict, and I only know that because I read the operands.

### 4.2 `18`'s narrowing of `07`: it holds, and the number reproduces

I reran `18_probes/p2_absorbing_top_operation_set.py`. Output matches the file exactly: add-only at 4
and 6 steps and both multiply variants give 0 unsound under absorbing; add-and-subtract at 4 steps
gives **936 of 5184**. `p2b`'s both-ends table gives 840 at 4 steps, splitting 568 top-then-down and
272 bottom-then-up.

The narrowing is real and it is not a contradiction of `07`. `07` measured a domain and did not state
it; `18` widened the domain and the result changed. That is precisely `17:677-684`'s Class C: "the
fragility is never in the arithmetic. It is that the count's domain and the claim's domain are stated
in different places or not at all." Two files reaching the same diagnosis of the same failure by
different routes, in the same stretch, without citing each other.

**One thing to watch when this travels.** The first witness `p2` prints is `start=0, abstract=1,
exact=0`, which is a **bottom** clamp under a top-only-absorbing model, not a top one. `18:379-382`
notices this itself and builds `p2b` in response. So the 936 is measured under `07`'s own model, where
the bottom is a point, and 840 of it survives when both ends absorb. `MORNING.md:281-282` carries both
numbers, correctly. If either travels alone the panel acquires the exact defect `15:757-761` warns
about.

### 4.3 `16`'s downgrade: it does not go far enough, and it goes too far in one place

The brief calls the downgrade "admirable, and also convenient". Both, and in different directions.

**It does not go far enough on the second contamination.** `16:35-38` records that listing
`15_probes/` printed 48 filenames "including `q07_three_input_map` and `q13_cold_packed`", calls
filenames "a weak leak", and downgrades nothing for it. Those two filenames carry more than `16`
credits them with. `q07_three_input_map` names the **three-input** framing, which is `16`'s section 2
framing. `q13_cold_packed` names **the site**, which is `16`'s answer to "what fails without it".
Combined with the commit subject's "needs two outputs", the leak covers the count, the arity of the
input, and the location of the failure.

`16:30-33` keeps independence for "which two, what the second is keyed on, what fails without it, and
which check is blind". Of those four, **"what fails without it" is `Cold` packing**, which
`q13_cold_packed` telegraphed. The downgrade should reach it. The other three stand.

**It goes too far in the direction that flatters nobody, which is the right direction to err.** `16`
gives the count away entirely, and the count is the headline. A member that wanted its rung would have
found a reading under which the commit subject "needs two outputs" is ambiguous about which two. It
did not look for one.

**And there is a rung inflation downstream that is not `16`'s fault.** `MORNING.md:174-176`: "The
**identity** of the two, the **keying** of the second, and the **blindness** of the certifying check
were derived independently and stand at two."

The blindness does not stand at two. `16:628-648` says so in its own words, twice: "`15` does not say
so", and "Mine is about a one-output map passing the certification. They compose ... and the second
half is not in `15`." A finding one expert derived and no second expert reached is ONE EXPERT under
`RULES.md:28-30`, however independently it was derived. Independence is a necessary condition of the
rung, not the rung.

`17` reproduced it, which is worth something and is not the rung either: `17:758-759` says it read `16`
in full first, and `RULES.md:28-30` requires each to have derived its own answer before reading the
other. So the correct classification is **one expert, plus one checked reproduction**, which is a
stronger thing to say than "two" and is also true.

This is the third rung inflation in `MORNING.md` in three stretches. `09` caught `07`'s self-flagged
inherited claim being promoted. `14` caught line 353 merging the two-expert rung with the three-instance
bar. Now this. `RULES.md`'s provenance ladder exists to stop exactly this, and the document being
written from it is the one place it keeps failing.

### 4.4 So: real, with one systematic tilt

The three self-corrections are real. Two of them I reproduced with my own commands and one of them I
checked harder than its author did, and all three survived.

The tilt is not in the experts. It is in the compression. **In every case the expert's own hedge is
present in the expert file and absent or weakened in `MORNING.md`**: `16`'s "and `15` does not say so"
becomes "stand at two"; `17`'s "those point opposite ways and both are true" becomes "a different and
harder problem"; `17`'s "the two oracles disagree" becomes "the aggregate erases". Three for three, all
in the direction of a cleaner story.

That is the finding about self-correction, and it is worth more than a verdict on any one file:
**the experts are hedging correctly and the summary is unhedging them.**

## 5. The next defects in `MORNING.md`

`09` found four and six of seven of its items were repaired. `14` found four in its section 3 and seven
in its section 8, and eight of thirteen were repaired. Here are the next ones. I checked each with a
command and give the command where a count is involved.

### 5.1 `MORNING.md:668` still merges the two-expert rung with the three-instance bar, for the third stretch running

> That is two instances arrived at differently, which is the bar.

`RULES.md:116`: "**One instance of evidence is never enough.** Three independent ones is the bar."

`09` flagged this as `MORNING.md:353`. `14` section 3 flagged it again by the same line number, and
recorded that six of `09`'s seven items were repaired and this one was not. It is now at line 668 after
the document grew, unchanged, in its third stretch.

It is the smallest defect in this list and the most revealing one, because merging those two rungs is
the operation that produces every other rung inflation in the document, including 4.3's. A claim with
two independent instances is a claim that has cleared the *provenance* question and not the *evidence*
question, and once the two words are interchangeable the document can promote either from either.

### 5.2 The anchor repair was applied to the second stretch and not to the third

`14:252-281` ran the check `RULES.md:153-157` asks for and found zero `file:line` anchors and zero
probe names in 468 lines against 158 anchors in the sources. Commit `60ff1e3` at 04:49:39 responded by
adding eight probe-name parentheticals.

Every one of the eight is in the second-stretch material. Third-stretch sections, `MORNING.md:78-334`:

```
numeric tokens                          60
probe names   ( `p3` / `q12` / `t1` )    0
file:line anchors                        0
```

So the repair was applied to the text that existed when the finding landed, and the 60 numbers written
afterwards carry nothing. Among them: 23.1, 174 of 420, 936 of 5184, 840, 28 of 64, 42, 35, 461 against
476, 6400, 6561, 5184, 648, 625, 66, 81. Not one names its probe.

`a-compression-is-checked-by-someone-else.md` states the mechanism: "compression preserves prose and
discards addresses, because addresses carry no meaning to the compressor and are the entire value to
the reader." The repair confirmed the diagnosis without changing the behaviour that produced it, which
is the shape worth naming: **the document was patched, and the practice was not.**

### 5.3 `MORNING.md:255-256` states a number that matches nothing, in the sentence it presents as the one to sit with

> And one number worth sitting with: **across this entire panel, four probe directories emit assembly,
> and the instrument clause four rests on is a single one of them.**

Four counts are available and none of them is four probe directories.

- `17:640` prints the command and its answer: `grep -rl 'emit asm' */*.rs */*.sh | wc -l -> 2 probes emitting assembly`.
- That command, run against `17`'s own commit `858475e` with `git grep -l`, returns **four files**: `15_probes/q12_erasure_asm.rs`, `15_probes/verify.sh`, `17_probes/t2_aggregate_erasure.rs`, `17_probes/verify.sh`. So `17`'s printed output does not reproduce from `17`'s printed command.
- Today, after `18`, the same command returns five files across **three** probe directories: 15, 17, 18.
- `find . -name '*.s'` finds committed assembly in **eight** directories: 06, 07, 08, 10, 11, 12, 17, 18.

`17`'s *sentence* is right under a reading its command does not implement: the two `verify.sh` files are
runners rather than probes, so "two probes" is correct and "the command returned two" is not. The
domain is in the prose and not in the command. That is verbatim `17:677-684`'s own Class C failure,
committed inside the section that names it and prescribes the fix ("a count carries its domain in the
same sentence as its value").

`MORNING` then changes the unit from probes to probe directories and the value from two to four, which
matches neither the file it summarises nor the repository. And it flags the result as the number worth
sitting with, which is the number most likely to be quoted.

### 5.4 `MORNING.md:210-219` merges a false positive with a false negative and calls the false negative worse

The section reads:

> **Pointed at what it cannot see, the oracle does not go quiet. It reports failure.** ... **So the
> instrument cannot be repaired by pointing it at bigger programs.** That is a different and harder
> problem than the one `16` found.

Harder to repair, yes. That is not what the sentence says next to a section whose subject is what the
panel's evidence is worth. The two findings sit on opposite sides of the ledger:

- `16`'s finding: the certifying check comes back **green** over a derivation that is wrong. Something
  untrue gets certified.
- `17`'s two regimes: the check comes back **red** where the thing is right. Effort is wasted and
  nothing false is certified.

`17:533-536` states this correctly in its own words: "Those point opposite ways and both are true,
which is the usual shape when an instrument and its subject have been conflated." `MORNING` drops that
sentence and keeps the ranking, so op reads two findings of the same sign where the file he is being
pointed at says they are of opposite signs.

### 5.5 `MORNING.md:326-327` undercounts the dispatcher's own error rate and mis-attributes one of them

> Three briefs tonight have carried an error of mine into a dispatch. Each was caught by the expert
> rather than by me.

I count more than three, and two of the three the sentence appears to mean are not briefs.

**Dispatch briefs to experts, in this stretch alone, each caught by the expert:**

- `15:39-42`, the brief quoted `06` one clause short, and `15:57-69` shows the missing clause changes what the gap is. **`MORNING` does not record this anywhere.**
- `17:30`, `:44-47`, the brief asserted `16`'s `p3` demonstrates the panel's erasure check is blind; it is a runtime value comparison and the probe says so in a comment. `17` went and ran the real instrument instead.
- `18:81-88`, the brief said `07` reports rounding adjoint to the embedding; that is `07`'s **refuted** prediction, and `18:86-87` states that reasoning from it "would have inverted section 2 of this file".

**Persona briefs, both caught by the persona:**

- `09:302-305`, the brief said the first stretch had barely touched the erasure gate; `09` found the opposite on the record.
- `14:566-572`, the brief misstated the clause coverage, and `14` notes it is the second time.

That is five, in two categories, and only one of the five is recorded in `MORNING` as the dispatcher's.
`MORNING.md:184-186` records `17`'s correction but frames it as "`17` ... corrected its own premise on
the way in", which relocates the error from the brief that supplied the premise to the expert that
caught it. `15`'s is absent. The two persona briefs are absent.

And the sentence's own count of three appears to include the two `SETTLED.md` compression defects
(`MORNING.md:441-449` and `:501-507`), which are compressions rather than briefs, and which
`MORNING.md:449` counts separately as "the second time tonight a compression of mine misled an expert".
So the sentence is counting two different things and arriving at a number smaller than either.

**The honest form**, and it is more useful than the current one because it separates the two failures:
three dispatch briefs and two persona briefs carried a factual error into a dispatch, and three
`SETTLED.md` compressions misled an expert, counting 1.1's. The first class is caught by the expert
every time and costs a paragraph. The second class is what cost the night.

### 5.6 `MORNING.md:26-28` and `:280-282` carry a restriction that `18`'s own table refutes

Question four's answer, as `MORNING` states it:

> absorbing is sound **exactly while the computation stays at the endpoint**, which additions alone
> satisfy and subtractions do not.

And `18:408-410`'s cheap repair, which `MORNING.md:283` carries as "the cheap repair is to write the
restriction down":

> the reading holds while the operation set cannot decrease

`18_probes/p2.out`, which I reran, has a row that refutes it:

```
add and multiply by zero, 3 steps   chains=512   unsound point=48   unsound absorbing=0
```

Multiply by zero decreases. The operation set `{add, mul-by-0}` therefore **can** decrease, and it is
sound at zero failures. So the proposed restriction refuses a sound operation set, and it does so on
evidence sitting in the same table it was derived from.

The reason is visible once stated. Under the absorbing reading the top datum stands for `[top, inf)`.
Multiplying that set by zero gives exactly `{0}`, which the numeral denotes exactly, so nothing is
lost. Subtracting one gives `[top - 1, inf)`, which it does not. **The condition is not that the
computation stays at the endpoint and not that the operations cannot decrease. It is that every
operation maps the absorbed set onto a set the numeral can denote exactly.** "Stays at the endpoint"
is sufficient and not necessary; `p2`'s mul-by-zero row is the witness.

`18:403-404` says of this exact result: "it is the single thing in this file I would most want checked
by someone else." This is that check, it is a persona's reading of `18`'s own committed table rather
than a new measurement, and it wants a real second read before anyone writes a clause from it. What it
does establish is that the clause as currently drafted would be wrong, and that matters because
`MORNING` puts it in front of op as question four.

### 5.7 `MORNING` drops `17`'s deliverable

`17:95-96` names what its file is: "That is what a trusted-base list is for, and it is why the
deliverable below is a list rather than a verdict." Section 6 is that list, nine items, and
`17:628-630` ranks the two cheapest to move off it.

`grep -niE 'forbidden|ban list|specializ|TypeId|trusted base|trusted list' MORNING.md` returns six
hits, all of them in the second-stretch sections about `generic_const_exprs`. The trusted base does not
appear. Item 4, that the bans on full `specialization` and `TypeId` hold because the transfer from a
bounded check to the full width range rests on them, is the item with a consequence outside this panel,
and it is absent. So is the split's own conditional: `17:236-237` marks dispatch erasure "proved by
construction, **conditional on the ban list holding**", and `MORNING.md:194-198` carries the split
without the condition.

The section reporting a file's results has dropped the thing the file says it is.

## 6. Canon, or a methodology for evaluating canons

The brief's question, and it deserves a real answer rather than a diplomatic one.

**By the count of files, this stretch is not a night of instruments.** `15` is design work. `18` is
design work. `16` is design work plus one methodological finding. `17` is almost wholly methodology.
One of four.

**By the count of canon-shaped sentences, this stretch beat both predecessors**, which surprised me.
Judged against `RULES.md:79-83`'s two tests, five sentences from this stretch would survive a total
rewrite and would make three independent teams converge:

- The numeral is keyed on total width and fraction width, because the integer width is the only coordinate that goes negative (`15:111-114`).
- The derivation has two outputs, the carrier and the stride, and every layout quantity is a function of those two with the declared width (`16:580-586`, corrected to `15`'s narrower form).
- Clause four decomposes into layout erasure and dispatch erasure, proved by construction, and operation erasure, which is the only part needing inspection (`17:221-245`).
- A packed run's allocation is not `ceil(N*W/8)` bytes if elements are read by word load; it is that plus the access slack, or the tail is a special case (`17:377-379`).
- A partitioning set denotation is free and an overlapping one is not (`18:432-436`).

Against `13`'s "cross once, at literals, in one direction" and `11`'s section 5 table from the second
stretch, and `08`'s boundary sentence from the first. So the trend is up, not sideways.

**And yet the honest answer to the question is: neither, and that is worse than either.**

`RULES.md:85-86`: "**The canon must say which things are doable**, which is what probes are for."
Everything the panel has established is about shape and refusal: what compiles, what refuses, what a
table's domain is, what an instrument can and cannot see. Eighteen files have established that the
mechanism is expressible. Not one has established that it is **worth it**, and for arvo those are not
the same question, because the design's own justification is a workload claim.

`arvo-toolbox-not-policer.md` puts it in the strongest terms the workspace has: `Cold` "is the reason
arvo exists", and the intersection that "created the need for arvo to be its own thing" is a set of
workload properties. A canon that says the derivation is doable and does not say what it buys has said
the smaller half.

So: **converging on a methodology for the mechanism, and not yet on a canon, because the doability the
canon has to assert is a workload claim and the panel has not made one.** The instruments are good and
were worth building. They measure whether the design says what it means. Nothing measures whether it is
the right thing to mean, and section 1 shows the material that would has been sitting in the repository
the whole time.

**A second night of instruments would be the wrong call and I do not think anybody is proposing one.**
What I would say instead is that the next dispatch is not a nineteenth expert. It is the second set of
eyes op asked for on `141`/`142`, plus the one `bench.toml` entry that puts dense against packed at
matched n. Both are inside op's own standing instructions and neither needs a ruling from him.

## 7. What I would refuse

Five, stated as the persona's and carrying no weight beyond the argument.

**Refuse the five questions at `MORNING.md:8-30` going to op in that form.** Not the questions, which
are good and are much sharper than the second stretch's menu. The framing. `MORNING.md:10` says
"Everything else in this file is context for them", and three of the five are questions whose answer
changes nothing until somebody builds something: what a datum stands for is not actionable until the
operation set is characterised, and 5.6 says the current characterisation is wrong. What is missing
from the list is the thing that **is** actionable tonight and is his own instruction: the `141`/`142`
bench body wants a second read and nobody has given it one.

**Refuse "no bench harness has run" as a statement about the question rather than about the panel.**
Four files say it and each is literally true. `MORNING.md:242` and `:758` promote it to "everything is
unpriced", which is false of the repository. Section 1.2 has the numbers. The correct sentence is that
no bench harness has run **in this panel**, that arvo's harness carries a bitpack family with committed
artifacts that bears directly on `Cold`, and that nobody has read it.

**Refuse `MORNING.md:174-176`'s two-expert rung on the blindness finding.** Section 4.3. One expert
plus one checked reproduction, which is a good rung and is not that one. Given `01` section 0 exists
specifically because the previous panel filed things too high, filing a rung too high inside the
document written from the provenance ladder is the failure the ladder is for.

**Refuse `MORNING.md:419`'s "all five candidate surfaces".** `14` refused this and it was not
addressed. `12:342-345` says C2 and C3 are refused shapes on their face, inside `SETTLED.md:110`'s
four-times refusal. They are measured comparators. A five-row candidate table invites op to weigh two
things he has already refused four times.

**Refuse the 81-versus-zero discrepancy staying where it is, for the fourth stretch.** `09` said drop
or promote. `14` said it is still sitting. `MORNING.md:780-782` now says "It must be promoted to a task
or explicitly abandoned before any consolidation", which is the recommendation restated for the third
time in place of the act. `15:757-761` has now written a second one into the record on purpose, with
its convention attached, which is the right handling and also means the panel is one careless quote
from having two.

## 8. What is genuinely good, specifically

`RULES.md:99-101` makes keeping something a result, so this is not padding, and I checked each rather
than taking the file's word.

**`15` changing the coordinates rather than paying for the corner.** `14` opened the negative-width
collision, listed three ways it might resolve, and marked the list as guessing. `15:600-603` says
plainly that it is a fourth thing not on the list and that `14`'s instinct to open it first was right
while its enumeration was "wrong in the direction of expense". That is a predecessor being corrected
precisely, in the direction that costs less, with a compiled whole-matrix check and a firing negative
control behind it. It is the best thing in the stretch.

**`16` refuting its own third component against its own criterion.** Section 3.2. And doing it in the
paragraph immediately after conceding the count, which is the moment a file is most tempted to find
something to have been right about.

**`17` going back for the instrument it had named and left.** `17:408-412`: "I named this in a first
pass as the cheapest missing thing and then left it, which is a blocker reported rather than attacked.
So it is built." Then it built the variant that exercised its own stated weakness, found the instrument
blind to it, closed the gap, and kept the failing version. `RULES.md:90-91` and
`a-dispatch-is-an-order-to-go-down-the-rabbit-hole.md` both ask for that and this is the clearest
instance in eighteen files.

**`18` recording its own probe defect rather than fixing it quietly.** `18:52-57`: a slice that held the
first argument fixed at the one interval where the law is true, 136 successes, corrected to 16 of 136,
original left on disk with its output. The workspace's test gate names that exact shape as setup that
helps, and `18` applied it to itself, unprompted, in its gate section.

**`18:640-644` is the most honest coverage sentence in the panel.** Seven probes sharing one author and
one model are "one instance of evidence wearing seven hats, not seven". Nobody else has said that about
their own probe set and it is true of nearly all of them.

**The evidence discipline held and I checked it rather than assuming.** `git ls-files` against the four
probe directories returns 46, 19, 21 and 21, matching the on-disk file counts exactly, and `git status
--porcelain` on the panel directory is empty. Every probe output I opened matched the file citing it.
Two of the four ship a `verify.sh`. Two members recorded refuted predictions. Two corrected themselves
against their own criteria.

**And the register held.** All four used "unpriced" where `RULES.md:119-122` reserves it and none
reached for a magnitude it had not measured. `15:415-416` and `15:650-652` call their own assembly reads
"an ad-hoc quick spike with no substance" as measurements, which is the phrase the rule prescribes and
which is unflattering to the file using it. The one place the discipline slipped is `MORNING.md`, for
the third stretch running, which is the dispatcher's document rather than an expert's.

## 9. What the dispatcher has got wrong

Stated separately because it is about how the night was run rather than about any file.

**The curated reading list has now cost the panel its most valuable material twice.** `RULES.md:171-173`
names `SETTLED.md`, the latest consolidation, the immediate predecessor and op's files. That list is a
good answer to the 210,000-token problem it was written for. It has no slot for the repository, and the
standing instruction that `mock/crates` is being nuked has been read by every member as "the repo is
not evidence". `mock/benches` is not `mock/crates`, it is not being nuked, it carries 535 tracked files,
and it holds the arm for the question four files call unpriced. Nobody looked, and nobody was told to.

**And the compression of the seed material dropped a standing instruction.** Section 1.1. This is the
third `SETTLED.md` compression defect of the night and the first one that loses an instruction rather
than a qualifier. The other two were found by experts reading the establishing text. Nobody was
dispatched to read the establishing text against the compression as a task, and `MORNING.md:449` and
`:505-507` both note the pattern without acting on it.

**Three dispatch briefs and two persona briefs carried factual errors, and the accounting says three.**
Section 5.5. The rate is not the problem: five briefs with an error, each caught by its recipient
before it did damage, is a system working. The accounting is the problem, because it is the input to
whether anything gets changed about how briefs are written, and it currently reads as one third of what
happened, with one of the errors reattributed to the expert who caught it.

**The anchor repair was applied to the paragraph and not to the practice.** Section 5.2. Sixty numbers
written after the finding landed, none of them anchored.

**`PERSONA_CALLS.md` is wrong in the same direction for the second consecutive stretch**, after being
told about the first, and the correction added a clause that makes the sentence true and leaves the
picture unchanged. Section 2.

**Two of four members did not report the canon gate.** `RULES.md:206-210` says every member runs it.
`15:13-27` and `18:20-32` state it explicitly and both did real work in it: `15` found two rows that
push toward its dispatch and `18` found the record has no denotation statement at all, which it then
made the subject of its section 4. `16` and `17` do not report one. `14:582-586` found the same split in
the previous stretch, two of four, different two. Neither is a substance failure and it should not
quietly become optional, because the gate's value is that its absence is visible.

**And the brief handed to me was accurate**, which is worth recording because the previous two were not.
Every claim in it about `15` through `18` checked out against the files, including the two I was told to
distrust. The one thing I would correct is small: it says my predecessors "found four and then four more
factual defects" in `MORNING.md`, and `14` found more than four, seven of them in its section 8.

## 10. What I would drop

**`17:252-253`, "Nothing is committed, per the dispatch."** Twenty-one files are tracked under
`17_probes/`. The line is stale and under `RULES.md:108-110` it reads as a statement that voids the
file. One line, and it is inside the file whose job is classifying what the panel's evidence is worth.

**The dense-table residue's timing table, again.** `14` refused it and nothing was done. `11:597-600`
closed the route on structure, and a spike showing 8193 rows compile in 3.11 seconds is a number
attached to a shape that compilation cannot rescue. Numbers travel.

**`MORNING.md:419`'s five-candidate framing.** Section 7.

**The 81-versus-zero discrepancy.** Fourth stretch. Section 7.

**`15`'s and `06`'s overshoot counts travelling separately.** Not a drop, a binding: 476 and 461 are the
same measurement under two conventions, reconciled exactly at `15:186-201` as 160 + 301 + 15.
`MORNING.md:329-333` carries the reconciliation, which is right. The thing to drop is either number
appearing anywhere without the other.

**Everything `09` and `14` put on the droplist stays dropped**: the Moore completion, the
`canonical_exponent` naming call as a cost argument, `03`'s reading F, route 15 and the `d01`
bare-parameter carrier as anything but a closure, `11`'s `e01` as a candidate, and base ten as a live
direction. `MORNING` no longer carries any of them and they should not come back.

## 11. Where I am guessing

Collected so op can discount them in one place.

I am **guessing** that op would treat the untaken second-eyes instruction on `141`/`142` as the sharpest
thing in this file. The inference is from his own words at `01:96-98` and from the fact that `09`'s
equivalent finding about the fresh-eyes instruction was acted on within minutes. It is an inference
about his reaction, not a reading of his words. What is not a guess is that the instruction is in
`seed/SETTLED_container.md:410-420` in his own quoted words, that `SETTLED.md` does not carry it, and
that no file in this panel has taken it.

I am **guessing** at the significance of my dense-against-packed comparison at 7M elements. The two
numbers are real, committed, from the same machine and pin nine minutes apart, and I read them off two
findings files. The harness computed no statistic across them because they are two bench entries, the
kernels differ in a way `bitpack-decoder-shape` measures separately, and I am asserting the ratio is
meaningful rather than measuring that it is. The 4.6x at n=16384 is inside one bench entry with the
harness's own CI and is not a guess.

I am **guessing** that 5.6's generalisation is the right one. What I checked is that `18`'s proposed
restriction is refuted by a row in `18`'s own committed table, which is a reading rather than a
measurement. The condition I offer in its place, that every operation must map the absorbed set onto an
exactly denotable set, is mine, is unbuilt, and `18:403-404` already asked for exactly this to be
second-read by someone else. It should be, and by an expert rather than by me.

I am **guessing** that the five sentences in section 6 are the canon-shaped ones. That is a judgement
about permanence built on two of `RULES.md`'s own tests, and it is mine.

I am **not** guessing that the packed walk is slower, that the erasure oracle has two false-negative
regimes, that 936 of 5184 reproduces, that `MORNING.md:668` is unrepaired, that the third-stretch
sections carry sixty unanchored numbers, or that `mock/benches` holds a bitpack family nobody cited.
Each of those is a command I ran or a file I opened, cited where it appears.

## 12. Coverage, and what this checkpoint did not do

**What I ran myself**, rather than reading in a file that cites it:

- Extracted both aggregate bodies from `17_probes/asm/t2_aggregate_erasure.s` and compared them instruction by instruction including every immediate, which is the operand-level check `17` did not run.
- Reran the optimisation-level sweep on `15_probes/q12_erasure_asm.rs` at `-C opt-level=0,1,2` on `nightly-2026-05-28`, and computed the opcode multisets of all five bodies at each level myself.
- Reran `18_probes/p2_absorbing_top_operation_set.py` and matched every row, including the mul-by-zero row that 5.6 turns on.
- `git grep -l 'emit asm'` at commit `858475e` to establish what `17`'s own command returned at `17`'s own commit, and the same command today.
- Anchor and number counts over `MORNING.md` whole and over its lines 78 to 334, with the patterns quoted in 5.2.
- `git ls-files` against `15_probes` through `18_probes` and `git status --porcelain` over the panel.
- `git log` on the panel directory, and `git show --stat` on `30fef51` and `60ff1e3` for section 2.
- `grep -rniE '\b141\b|mock/benches|benches/'` over the panel's markdown, which is what established that one file mentions `141` in one clause and none names the bench directory.
- Read `07_probes/p5_postfixpoint_accumulator.py:100-141` in full to check `18`'s reading of it.

**Files I opened outside the panel**: `seed/SETTLED_container.md` at lines 405 to 425;
`mock/research/202607301300_formalization-spec-panel/141_xu_the_container_fork_benched.md` at its header
and verdict; `mock/benches/bench.toml`; `mock/benches/variants/bitpack-{native,aligned,zeropad}-seq/src/lib.rs`;
five `*_findings.md` and two `*.meta.json` under `mock/benches/`;
`.claude/rules/unstable-features.md` and `arvo-toolbox-not-policer.md`.

**What I did not do.** I did not run the bench harness. Every number in 1.2 is read off an artifact
committed by a previous panel and none of it was produced tonight, so it is evidence about what was
measured then rather than about the design as this panel has reshaped it. I did not read `141` beyond
its header and verdict, and I did not read `142` at all, so I am reporting that op asked for a second
set of eyes on that pair and I am not providing one. I did not read `CANON_CANDIDATE.md`, `DROPLIST.md`,
`02_carried`, `09` in full (only its `PERSONA_CALLS.md` entry and `14`'s citations of it), or the closed
panel's tree except the two passages named above.

I did not verify `15`'s `q05`, `q06` or `q08` matrices by recompiling them, `16`'s `p1`, `p2`, `p5`, `p6`
or `p7`, `17`'s `t1`, `t4` or `t5`, or `18`'s `p1`, `p3b`, `p3c`, `p4`, `p5` or `p6`. I took those from
the files and from their committed outputs. I did not check `17`'s three-toolchain result at 4.3, whose
artifacts went to `/tmp` and are not in `17_probes/`, and the same is true of its six-level sweep, which
I reproduced at three levels rather than confirming its committed trail because there is none.

**Nothing here is priced.** No bench harness run bears on this file. Where I quote a number from
`mock/benches/`, it was priced by the harness in a previous panel and I am reading its artifact, which
is a different thing from having measured it.

**Nothing here settles**, per `04`, including every correction above, which is a persona's reading of
what four files and one bench directory say about themselves and should be checked against them before
either is edited.
