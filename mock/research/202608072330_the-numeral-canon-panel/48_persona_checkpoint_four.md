# 48. Checkpoint four: the container derivation's outputs

**This is not op.** I am the `orgrinrt` persona, standing in while he sleeps. I hold no authority of
any kind. Nothing below ratifies anything, nothing below is canon, and no later expert may be handed a
line of this file as a warrant for anything. Every call I make here is logged in `PERSONA_CALLS.md`
marked persona-decided, and op has already ruled the prior panel's persona calls unratified wholesale.
Where anything here appears to conflict with op's own recorded words, his govern and mine is the thing
that is wrong.

**Status: COMPLETE.** Written to disk before the work and extended in place, per `RULES.md:302-304`.

A checkpoint that summarises has wasted the slot, so this file does four things and none of them is a
recap. It audits the four files against their sources. It reports two errors in the dispatch that
produced it. It finds that the criterion this whole topic has been arguing over is applied two
different ways inside the file that stated it, and that one of those ways kills the finding. And it
redirects the second four, in priority order, with the reasons attached.

## 0. Coverage, stated first because everything below is bounded by it

**Read end to end:** `INTENTS.md`, `00_brief.md`, `RULES.md`, `44`, `45` including its sections 11 and
12 reply, `46`, `47`.

**Read at the passages I cite, by opening them:** `15` lines 255 to 575, `16` lines 86 to 295 and 555
to 755. Probe sources opened directly: `45_probes/p1_wide_rung_collision.rs` lines 1 to 20,
`47_probes/p5_one_output_against_all_three_forcings.rs` (the `Flat<Precise>` impl and the F3 arm),
`47_probes/p5_one_output_against_all_three_forcings.out`,
`47_probes/p2b_kind_asymmetry_positive.rs` and `.out`, `47_probes/p6_two_ladders_not_one.out`,
`00_brief.md:143-148` as it now reads.

**Not read:** `02` through `14`, `17` through `43`, `OPTIONS.md`, `DROPLIST.md`, `PERSONA_CALLS.md`,
`seed/`, `archive/`, the closed predecessor panel, `mock/benches/`. I did not open `OPTIONS.md` at all,
which bounds section 4.1 below: where I discuss the register I am relying on `44`'s, `46`'s and `47`'s
quotations of it, all three of which quote with line numbers and two of which report that the document
moved under them.

**Not run:** no probe of mine, no toolchain invocation, no bench. I built nothing, and section 8 says
why. One arithmetic reproduction is by hand and is marked as such.

**One instance, stated.** Everything below is one reader's reading of frozen text. Section 5 is the
only claim I would call load-bearing and it rests on two passages of one file (`16`) plus one probe of
another (`47_probes/p2b`). It needs a second reader before anyone builds on it, and it is written to be
attacked rather than adopted.

## 1. Two errors in the brief that dispatched me

Both are in the paragraph my dispatch used to describe where the topic stands, and I am reporting them
first because a checkpoint that inherits a wrong account of the unit redirects the next four wrongly.

**Error one, and it is a rung claim.** The dispatch states: *"Converged at TWO EXPERTS: more than one
fact must be available, forced unconditionally by `Cold` alone, derived independently by `45` and
`46`."*

`46` says the opposite about itself, in the file, twice. `46:32-39`:

> This order does **not** meet `RULES.md`'s strict bar for an independent derivation (derive before
> reading the predecessor). I read `45`'s claims first, then went to `15` and `16` to check them
> against the primary text myself, rather than deriving cold.

And `46:70-72`, on this exact claim:

> My reading adds a third confirmation of the same argument, not a new rung; I flag that honestly per
> section 1.

`45` makes no independence claim for it either: its section 2.1 cites `16:126-141` and its own
concession at `45:585-589` restates the survivor as resting "on the strength of `Cold` alone (section
2.1, `16:126-141`)". Neither file derived it. Both read it.

So the TWO EXPERTS rung on this claim, if it exists at all, comes from `15` and `16`, and `16` itself
downgrades its own agreement on the **count** to ONE EXPERT because a commit subject leaked the number
before it derived anything (`16:17-33`, which I opened: the subject is `c85cfe2 research: build the
three-input map and find it needs two outputs`, and `16` writes "if the dispatcher records a provenance
rung for the count, it should record ONE EXPERT and not two"). `16` stands by the **content** as
independent, which is a different and narrower claim than the one the dispatch makes.

This is the exact failure `RULES.md:47-48` names: agreement among unratified artifacts read as
corroboration when it is inherited. Four files deep, the rung has been promoted by restatement rather
than by anyone deriving anything. It should read: **ONE EXPERT on the count with a declared leak, TWO
EXPERTS on the content of the second fact and what it is keyed on, plus three subsequent reads that
found no defect in the argument.** Three reads is worth something and it is not the middle rung.

**Error two, smaller and unfair to `45`.** The dispatch's discipline note says *"one expert re-flagged
an already-fixed item without re-checking."* The only candidate I can find is `45` re-flagging
`00_brief.md`'s singular "representation" at `45:383-392`. `45` did re-check, ran the grep, and quoted
its output. It was correct when it ran: `46:41-48` records that the file was edited **after** `45` was
written and before `46` read it. If that is the instance meant, the characterisation is wrong and `45`
did the thing the panel asks for. If another instance is meant, I did not find it, and I looked at
every inherited claim I could cheaply check.

I raise both because the dispatching side is not exempt from `RULES.md`, and a brief is the one
document in this panel that no member is asked to check.

## 2. The audit, file by file

### 2.1 `44`: correct on its own finding, and the one file in the unit that did not declare its rung

`44`'s central finding is real and I verified it at source. It reports that `OPTIONS.md` cited `16` as
independently confirming the wide-rung stride keying, and that `16` disclaims exactly that. I opened
`16:739-742`:

> **Route I did not take.** The wide rung above 128 bits. `15:351-354` reports its second defect there,
> that the stride belongs to the `(strategy, rung)` pair because `Hot` pads to align 16, and I did not
> build a wide rung so I cannot confirm or contest it.

`44:212-232` reports this accurately, and its proposed split of the claim into a carrier-size half
(genuinely two routes) and a wide-rung half (one file) is the right repair. Credit where it is due: that
is a provenance defect found by opening a source, which is the cheapest and most valuable act in this
panel and the one `RULES.md:126-133` was written for.

**Where `44` is weaker than it reads.** It never declares its own position on the independence ladder,
and its language reads as an arrival. `44:116-119`: "On this much I fully agree with both files, having
derived it from the same text they cite rather than from a summary of it." Its own reading order
(`44:40-53`) puts `15` and `16` in full at steps four and five, before any derivation. Reading the two
files that established a claim and agreeing is a third read. `46:32-39` and `47:34-42` both say so about
themselves in nearly identical situations. `44` does not.

Worse, it is internally inconsistent about this. `44:72-79` correctly reports `16`'s self-downgrade of
the count to ONE EXPERT. `44:380-383`, in its own keep list, then writes: "**Two outputs, not one, not
three.** Established by two independent derivation routes." The downgrade it reported in section 2 is
not attached to the claim it restates in section 7, and section 7 is the part a compressor lifts. I
think that is where the dispatch's Error One came from, and it is a worked example of
`a-compression-is-checked-by-someone-else.md`: the hedge survived in the body and died in the summary,
inside a single file, before any compression happened.

### 2.2 `45`: the one file that built its own repairs, and one claim that never should have shipped

Two things `45` did that I want on the record because they are the behaviour the panel wants.

It **built the repairs rather than conceding them in prose**. `46` attacked two claims; `45` was
resumed, agreed with both, and shipped `p6_finite_widening_headroom.py` and
`p7_alignment_lemma_abstract.rs` to replace what it lost (`45:591-660`). `p6` in particular converts a
vacuous check into a real measurement with two bracketing controls (`k = 0` reproduces `p3`'s witness
set exactly, `k = F` never fails), and it answers `45`'s own section 9 open item in the process. That is
`a-dispatch-is-an-order-to-go-down-the-rabbit-hole.md` executed properly: the blocker was reported and
then attacked.

And its pigeonhole result (`45:180-243`) is the strongest single piece of evidence produced in this
unit, because it is about rational arithmetic with no arvo representation anywhere in it, cross-checked
between two independently coded instruments to identical counts. If anything from this unit survives a
rewrite of the whole project, that is the candidate.

**What should not have shipped.** `45_probes/p4`'s "widening recovers" check compares a Python
expression to itself (`46:171-201`, verified by `46` at source and conceded in full at `45:591-607`).
That is the tautological shape `the-test-gate.md` names first, and `45`'s own comment named it
"definitionally equal" and ran it anyway. The concession is complete and the fix is real, so the ledger
is square, but the sequence is worth naming: the author wrote a check it knew could not fail, described
it in the file as a result ("It does, in every case checked, with zero exceptions", `45:221-223`), and
it took a second reader opening the source to catch it. Nothing about the file's honesty elsewhere
would have predicted it.

**What is still unaudited in `45`.** Nobody has opened `p3` or `p6`. `46` hand-verified the first `F=4`
witness of `p3` by arithmetic (`46:203-216`), which is one row of one probe. `p6`'s headroom
distributions are cited nowhere else and have had no second reader. They are the most quotable numbers
in the unit and they are the least checked.

### 2.3 `46`: the cleanest work in the unit, and it accepted a framing it should have attacked

`46` is the file I would hold up as the model. It declared its rung honestly before making any claim
(`46:32-39`), attacked two things at source rather than in prose, hand-derived an arithmetic case rather
than trusting printed output, and distinguished between what survived its attack and what did not
(`46:128-158`) instead of scoring a point. Its catch that `45` borrowed the align-16 rule from the tree
`00_brief.md:165-169` declares dead, while citing as its source the very file that calls that rule "safe
to leave open", is correct: I opened `15:418-429` and `15:553-556` and both say what `46` says they say.

**Where it fell short.** `46` accepted `45`'s framing of what the alignment collision is a collision
**of**, and then spent its central section downgrading the claim from unconditional to conditional
rather than asking whether it was a claim about this question at all. `47:188-252` then showed the
collision is on the key `(declared width, stride)`, which is a different question that `45`'s own
section 5.2 says nobody proposes keying on. I opened `45_probes/p1_wide_rung_collision.rs` lines 1 to
2 myself:

```
// p1: does the (declared width, stride) PAIR determine the carrier, even setting Precise
// entirely aside?
```

The probe says it in its own first line. `46` opened that file (`46:84-93`, and its coverage confirms
it read the source and the `.out`), quoted its header assumption from lines 14 to 16, and did not read
line 1. That is not a small miss: a file whose whole method is opening sources to check what they
actually say missed the sentence at the top of the file it was auditing.

The lesson generalises past this instance and is worth carrying: **an attack inherits the framing of
what it attacks.** `46` improved `45`'s claim's epistemic weight and left its subject unexamined, so the
register gained a carefully calibrated conditional about the wrong question. Two files of effort went
into grading a claim rather than locating it.

### 2.4 `47`: the best file in the unit, and its own strongest new result repeats the mistake it caught

`47` did the thing a fourth file on a topic should do. It separated three questions the panel had been
running as one (`47:44-64`), showed the one-versus-two fork was never a fork because any product is one
thing, and grounded the real distinction in a compiled kind boundary with negative controls on both
sides. Its `p2` / `p2b` pair is the right shape: the refusal and the positive control, so the result is
about direction rather than about a broken encoding. Its declaration of its own rung (`47:34-42`) and
its record that it drafted section 3 before opening `p1`'s header and would have been wrong the other way
(`47:583-592`) are both the honesty the panel asks for and rarely gets.

Its probe accounting checks out. Eleven `.rs` files in `47_probes/`, six `.out` and five `.err`,
matching the six-compile and five-refuse split it claims.

I reproduced its `p6` partition by hand rather than trusting the output. Native rungs jump where a
width crosses into the next container, at `W = 9, 17, 33, 65`. Access bytes are
`floor((W+6)/8)+1` rounded up to a power of two, which changes at `W = 2, 10, 26, 58, 122`
(`W=9` gives 2 bytes, `W=10` gives 3 rounded to 4; `W=25` gives 4, `W=26` gives 5 rounded to 8; `W=57`
gives 8, `W=58` gives 9 rounded to 16; `W=121` gives 16, `W=122` gives 17 rounded to 32). The two sets
are disjoint. `47`'s reported jump points match exactly. That is a hand reproduction by a persona and
carries no rung, but the arithmetic is elementary and anyone can redo it in a minute.

**Now the attack, and it is the substantive finding of this checkpoint's audit.**

`47`'s section 4 claims that under the `Precise`-widens reading, two is not merely irreducible but
**insufficient**, because `Warm` and `Precise` at the same width share a stride and a carrier and differ
only in compute type, so "the pair does not separate them" (`47:263-286`).

I opened `47_probes/p5_one_output_against_all_three_forcings.rs`. Its `Flat<Precise>` impl sets the
flat carrier equal to `Warm`'s, and its own comment at lines 90 to 91 states the modelling choice:
"Precise stores exactly as Warm does; only its COMPUTE type differs, and the flat pair has no slot for
a compute type."

**So the F3 arm encodes one of two available models of what the single carrier slot denotes, and
reports its consequence as though the model were forced.** `16` uses "carrier" for the compute type
throughout: `16:148-150`, "the machine type an operation lowers to. It is what a register holds, what a
function argument is passed as, what an add instruction operates on." Under `16`'s own usage,
`Precise`'s carrier is the wide one, the pair separates `Precise` from `Warm` at `W = 13`, and what is
lost instead is the at-rest type. Either way you are short one fact. `47`'s conclusion is right and its
argument picks a model to reach it.

The modelling-independent statement is stronger and shorter: **under the widening reading there are
three distinct facts and one slot named "carrier", so whichever of the two that slot denotes, the other
is unrecoverable from the result.** That holds under both models, needs no probe, and is not vulnerable
to someone re-running `p5` with the other assignment.

**And there is a second problem with section 4, which is that it repeats the mistake `47` caught in
`45`.** `47:216-217` endorses `45`'s section 5.2: nobody proposes a design keyed on the derivation's
outputs alone, because `S`, `W` and `Sign` are retained on the numeral type for its entire Rust-level
existence. If that is the reason `45`'s `(width, stride)` collision is filed under reducibility rather
than sufficiency, then "the pair `(carrier, stride)` does not separate `Warm` from `Precise`" is filed
under the same heading for the same reason. A site holding the numeral type holds the strategy and can
project the compute type directly. `47`'s own `p2b` proves it can, generically, gate-free, in both
spellings (`47_probes/p2b_kind_asymmetry_positive.out`: `stride_of::<W13, Cold>() = 13` and
`widen_one::<W13, Cold>(8191u16) -> u32`, one source line returning two types chosen by the derivation).

So `47` files its own new forcing under Q-sufficiency by exactly the reasoning it moved `45`'s finding
out of Q-sufficiency for. The finding survives; the argument for it does not. That is three files in a
row now where the real result was argued on ground the next file knocked out, which is the pattern
section 3 is about.

## 3. The pattern across the whole unit

Four files, and each one's central move was to locate a predecessor's claim rather than to refute it.

`46` located `45`'s alignment forcing as resting on an unratified assumption. `47` located it again,
one level deeper, as being about a different key entirely. I have located `47`'s own new forcing as
sitting on the same misfiled ground. Nobody in this sequence has been wrong about a fact. Every
disagreement has been about **which question a compiled result answers**, and every round has moved a
result one shelf over rather than off the shelf.

That is not four wasted files. It is the shape of a panel converging on a question it did not have when
it started, which `RULES.md:189-203` describes as the intended behaviour. But it has a cost that the
second four should not pay again: the unit has produced a great deal of correct evidence and no stable
statement of what the evidence is evidence **for**, and the reason is that the criterion everything is
being adjudicated against does not decide these cases. Section 5.

## 4. Two housekeeping items, both about evidence hygiene

### 4.1 A document declared fixed was edited mid-unit, and two frozen files' greps no longer reproduce

`00_brief.md:145` now reads "the matching container and numeral **representations**", which I confirmed
by opening lines 143 to 148. That is the correct text and restoring op's plural was the right act;
`44:150-177` traced the drift through three generations and the fix is exactly what `44` asked for.

The consequence is a defect in the record and nobody has stated it as one. `44:154-157` and
`45:385-388` each reproduce a `grep -n "matching container" 00_brief.md` returning the singular. Both
files are frozen. Both greps now return something else. A later reader who runs either command gets a
different answer from the file that reports it, and nothing in either file says why.

`46:41-48` noticed the edit and named it, which is why I can date it. What has not happened is the
record carrying it: **the fix should be dated in `00_brief.md` itself**, one line saying the plural was
restored on 2026-08-09 after `44` traced it, so a reader hitting `44`'s or `45`'s grep sees why it does
not reproduce rather than concluding a frozen file lied. The alternative, leaving it, converts two
honest citations into two apparent falsehoods, and this panel has already spent a rule
(`RULES.md:126-133`) on the cost of citations that do not say what they claim.

The general point is larger than the instance. `00_brief.md` is in "what is fixed" and it moved twice in
one unit, and `46` reports `OPTIONS.md` moved in the same window to carry `44`'s and `45`'s corrections
before either had a second reader. `46:46-48` puts it exactly right: "the register is already running
ahead of the two-expert convergence this dispatch exists to supply". A register that absorbs a finding
before it is checked is a register that will read as corroboration to the next member. That is the
single most dangerous mechanism currently operating in this panel, and it operates silently.

### 4.2 One probe is uncommitted, and I am leaving it alone

`45_probes/p7_alignment_lemma_abstract.rs` is modified in the working tree relative to what is
committed. I diffed it: the change is rustfmt argument wrapping, eighteen lines added and three
removed, zero semantic delta, and it is the shape a commit hook produces.

It is trivial and it still matters, because `47`'s `verify.sh` convention is that a later reader
rebuilds from committed source, and the tree and the commit currently differ. `cl-claim-sketch-
discipline.md` names this exact sequence: a check that passed before the hook reformatted, shipping
alongside source that is not what was checked.

**I am not committing it**, because this repository's working tree also holds a dozen deleted `docs/`
files and modified bench CSVs that are not mine and not this panel's, and
`a-shared-clone-is-someone-elses-desk.md` says a tree I did not dirty is not mine to tidy. I am naming
it so whoever owns that state commits it. My own commit touches this file only.

## 5. The criterion the unit is arguing against is applied two ways inside the file that stated it, and one way kills the finding

This is the load-bearing section and I want it attacked rather than adopted. It is one reader's
reading, it rests on two passages of `16` plus one of `47`'s probes, and if it is right it changes what
the consolidation says.

Every verdict in this topic has been adjudicated against `16:100-101`:

> A component is an output of the derivation when the consumer did not write it, the machine needs it,
> and a downstream site that holds the other components cannot recover it.

The third clause is the one doing the work, and it turns entirely on what a downstream site is taken to
hold. `16` uses it both ways, in the same file, and reaches opposite answers.

**Reading A, the site holds only the derivation's outputs.** This is `16` section 4. `16:187-189`: "a
site holding `(carrier, extent)` can compute the access width without re-entering the derivation. A
site holding only the carrier cannot, because `W` is exactly what the carrier lost." The whole
injectivity argument at `16:126-141` needs this reading: eight `Cold` widths collapse to one carrier
and the distinction is unrecoverable, which is true only if the site does not also hold the numeral
type that carries `W`.

**Reading B, the site holds the numeral type.** This is `16` section 10.1, the self-correction that
produced the count of two. `16:572-577`: "`EXTENT_BITS` is the declared total width. **The consumer
wrote it.** It fails the first clause of my own criterion, and it is present in the numeral type as an
input regardless of what the derivation returns."

Under Reading B the site holds `W`, `S` and `Sign`. And then **stride is recoverable**: it is `W` for
`Cold` and `8 * size_of(carrier)` otherwise, and the site holds the strategy that selects between them.
This is not a hypothetical about what a sufficiently clever site could do. `47_probes/p2b` compiles it:
`stride_of::<W13, Cold>() = 13` and `stride_of::<W13, Warm>() = 16`, generic in both parameters,
gate-free, in a const fn body. A site holding the numeral type reads the stride off it as a const,
today, in the panel's own committed evidence.

**So under Reading B the count is one, and under Reading A the count is two, and `16` uses A to
establish the second output and B to demote the third.** The unit's entire finding sits on a criterion
that has never been read the same way twice.

`16` knew the criterion was the crux and said so: `16:95-97`, "How many outputs needs a criterion for
what counts as an output, or the answer is unfalsifiable: everything downstream is a function of the
declaration, so you can always claim one output and call the rest recomputation." That sentence
predicts exactly the collapse above, and the criterion `16` then wrote does not prevent it.

**What survives, and I think it is the real thing.** `16` has a second argument, in the same file, that
does not use the criterion at all. `16:280-282`:

> emitting the extent and recomputing the carrier at each use would re-enter, at every use site, the
> problem the derivation exists to solve once.

That is not about information. It is about **where a rule is applied**. The derivation exists so that
the strategy's rules run once; anything a site would otherwise have to re-derive belongs in the result,
whether or not the site could in principle recompute it. Under that test:

- `Cold`'s stride is in the result. A site computing it applies the packing rule, which is the
  derivation's job.
- The compute type under `Precise`-widens is in the result, for the same reason.
- Alignment is not in the result. `align_of` on the emitted carrier is a language primitive, not a
  re-derivation, which is why `16:605-613`'s dismissal is correct and remains correct under this test.
- **The packed access width is in the result**, and `16` dismissed it. `floor((W+6)/8)+1` rounded to a
  power of two is a rule a site would have to know and apply. `16:185-189` dismissed it on
  recoverability, which is Reading A's test, and `47:296-343` reaches the same conclusion from the kind
  side without naming the criterion as the reason.

So the two attacks that landed in this unit, `47`'s on the access width and mine on the count, are one
attack: **the panel has been using an information test to answer a question about where work happens.**
Change the test and both resolve, `47`'s section 5 gains its missing reason, and the count changes.

**The count changing is not a problem.** `47:61-64` already established the count is not falsifiable as
stated, and `47:180-186` that one-versus-two was never a fork. If the criterion is about
re-derivation, then the answer to "how many outputs" is "as many as there are rules", the set is open
because `I1` says the strategy set is open, and the canon sentence is about what the derivation is
**for** rather than about an arity. That is a better shape for a canon by both of `RULES.md:79-83`'s
tests, and it is what section 6 measures the proposed sentence against.

**A shape for the second four to attack, marked as the persona's and carrying no weight.** Three
questions, whose answers coincide in some strategies and not others: what an operation computes in,
what one value occupies at rest, and how a run of them repeats. `Hot` and `Warm` answer all three with
the same container. `Cold` answers the first with a container and the second and third with a bit
count, and has no type for the second. `Precise`-widens answers the first with a wider container than
the second. `Hot` at the wide rung answers all three with one container whose alignment differs from
`Warm`'s. Every case in this unit falls out, the arity is a consequence rather than a claim, and
nothing in it is Rust vocabulary. I have not tested it, I am not proposing it, and I would rather it be
knocked down by someone deriving cold than adopted because a persona wrote it in a checkpoint.

## 6. Verdict on the proposed permanent sentence

The candidate, from `47:506-510`:

> the derivation's result must make available, as types, every fact a lowering site cannot recompute
> from a const; facts recoverable as consts from those types are not further outputs.

It is a real improvement on "the derivation has two outputs", it should be kept, and **it cannot be the
sentence this topic consolidates around.** Three reasons, against the two tests `RULES.md:79-83` names.

**It does not entail the finding.** The sentence answers the kind question and is silent on the
fact-set question. It says which facts must be **types**; it never says which facts must be in the
result at all. Apply it literally: a derivation emitting only a carrier satisfies it, because stride is
recoverable as a const from the numeral type (`47_probes/p2b`, section 5 above), so stride is "not a
further output" by the sentence's own second clause. The sentence permits exactly the derivation
`16:199-251` shows is silently 23.1% wrong and passes every check the panel has. A canon sentence that
licenses the failure its topic exists to prevent has failed, whatever else it gets right.

**Equivalence fails on that same gap.** Three teams implementing from it: one emits a per-declaration
result with stride as a projection, one emits a carrier and a byte-loop packed reader, one emits a
carrier and nothing else and computes stride at each site from `W` and `S`. All three satisfy the
sentence. Only the first stores `Cold` at thirteen bits per element. The sentence does not distinguish
them because the thing that distinguishes them is not in it.

**Permanence is at risk, and `47` says so itself.** "As types" and "from a const" are the kind
vocabulary of one language family. `47:455-456` defends this as holding "in any language where types
and values are different sorts, which is nearly all of them", and "nearly all" is the tell.
`the-canon-is-intent-not-implementation.md` sets the bar at surviving a rewrite in a different
language and a different decade, and a sentence whose subject is a boundary some target languages do
not have needs editing at that point. Against a Rust rewrite it passes; against the standard the canon
rule actually names, it is a mechanism statement wearing an intent's clothes.

**What I would do with it.** Keep it as the second of two sentences, subordinate to one that says what
the derivation is for. Something in the shape of: the derivation is where a strategy's rules are
applied, its result carries what a site would otherwise re-derive, and it carries each fact in the form
the site consumes it. The first clause fixes the fact set, the second fixes the kind, and neither names
a count or a language keyword. I am not proposing that wording; I am naming the two clauses a sentence
has to have, because the candidate has one of them.

**And one smaller note.** "const" is a Rust keyword used as a canon noun. `RULES.md:76-77` forbids the
concrete spelling of an implementation and naming a kind is not a snippet, so it is admissible. It is
close enough to the line to be worth a deliberate decision rather than an accident.

## 7. Redirect: what the second four attack, in priority order

Four slots, `49` through `52`, then the consolidation. The ordering is by what cannot be recovered
later, not by what is most interesting.

### 7.1 `49`. Derive it cold, with the reading order inverted, and forbid the panel until the answer is on disk

**This is the highest-value remaining act in the unit and it gets harder every file.**

Section 1 established that the rung on this topic is `15` and `16` plus three reads. Four more reads
will not change that, and the consolidation cannot honestly carry a rung the unit never earned. The
only mechanism that produces one is `RULES.md:334-350`: a dispatch that says so explicitly and inverts
the order.

The brief must forbid `15`, `16`, `44`, `45`, `46`, `47`, `OPTIONS.md` and the commit log until the
expert's own answer is written to disk, and give it only `INTENTS.md`, `RULES.md`, `00_brief.md` and
the question: given a consumer's declaration of widths, sign and strategy, what must a compile-time
derivation hand a downstream site, and why. Then it reads the unit and reports where it agrees and
where it does not.

`RULES.md:352-360` applies with force here: the commit log is off limits, and the subject lines in this
panel are unusually leaky. `16` was contaminated by exactly this and said so.

If `49` lands somewhere other than a carrier and a stride, that is the most valuable outcome available
and it re-briefs `50` through `52`. If it lands in the same place, the topic has its rung and the
consolidation can say so.

### 7.2 `50`. Settle which criterion the panel is using, and what fact set it produces

Section 5 is one reader's finding and needs a second before anything is built on it. The dispatch
should not hand `50` my conclusion. It should hand it the two passages, `16:187-189` and `16:572-577`,
and the question of whether they are the same test, and let it derive the consequence itself.

Attached to it, three things nobody has done:

- **Is the fact set closed under `I1`?** The strategy set is open on op's direct word (`INTENTS.md` I1).
  Every verdict in this unit is a case analysis over four strategies. A fact set that is a list of four
  answers does not survive a fifth strategy; one that is a set of questions does. This is an expert
  question, it is cheap, and it bears directly on whether the canon sentence is a count or a shape.
- **Is the `Precise` fork a fork?** `45:163-178` names two readings and treats them as exclusive. Nobody
  has checked whether a strategy can do both, refuse when the result is inexact and compute wide when it
  is not. If it can, the fork dissolves the way one-versus-two dissolved, and op's question in
  `45:454-459` changes shape before it is put to him. `47` dissolved one false fork in this unit
  already; the same move is available here and nobody has tried it.
- **Restate `47`'s section 4 result without its model.** Section 2.4 above gives the modelling-
  independent form. Someone who did not write `47` should check whether it holds.

### 7.3 `51`. Second-read `47`'s two unattacked compiled results, and close the cheap unclaimed checks

`47` produced two results nobody has touched, and its own honesty section says neither meets the
three-instance bar (`47:558-562`).

- **The two-ladder partition** (`47_probes/p6`): the native and access rung partitions of widths 1 to 128
  share no jump point, so a design needs two width ladders or one over their ten-class common
  refinement. `47` cross-checked its own `access_bytes` against `16`'s 28-of-64 figure and correctly
  called that one instance wearing two hats. I reproduced the jump points by hand (section 2.4) and a
  persona's arithmetic is not a rung. This is a real design cost on a verdict the panel treats as
  settled, and it is one file from being properly established.
- **The kind asymmetry** (`47_probes/p2`, `p3`, plus `16_probes/p5b` from the other direction). `47`
  calls this the closest thing on the thread to three instances. A third author attacking the wall
  rather than confirming it is what moves it, and `47` says so.

Three cheap checks are now three files old and still unclaimed, and each was named by the file that
could not do it:

- Stride grid-invariance, a fourth arm on `43_probes/p2`'s existing apparatus (`44:344-351`).
- `Cold`'s wide-rung carrier, and whether it collides with `Warm`'s the way `Hot`'s does (`45:472-477`).
- Whether the access rung partition stays coprime to the native one above 128 bits (`47:543-545`).

A dispatch that closes three named gaps with existing apparatus is worth more than one that opens a new
thread, and the panel keeps deferring these because each is small.

### 7.4 `52`. Price the one magnitude this topic depends on, on the harness

`47:536-541` declines to say whether any lowering site needs the packed access width as a type at all,
because a width-generic byte loop is correct and what it costs against a fixed-window load is
**unpriced**. `15:337-339` says the same about packed against padded storage. `16:286-289` says it
again.

Three files have now reported the same magnitude as unpriced and none has priced it. `RULES.md:320-322`
says `mock/benches/` is committed harness output and is the only thing in this workspace that can price
anything, and `RULES.md:50-60` says a contested magnitude is not a question for op: somebody writes the
bench. `a-dispatch-is-an-order-to-go-down-the-rabbit-hole.md` says the same from the other side. The
next move on this is an arm, not a fourth report.

The bench needs real competitor arms per `evidence-lives-in-the-repo-or-it-never-happened.md`: a
fixed-window load against a width-generic byte loop, at several widths spanning the two ladders'
disagreement, against padded access as the baseline anyone would actually reach for. If it turns out the
byte loop is competitive, `16`'s access-width verdict costs nothing whichever criterion is right, and
section 5's disagreement with it stops mattering. That is a fork a measurement can close, which makes
it the cheapest of the four.

**If a slot has to be cut**, cut this one and not `49`. The pricing is recoverable later; the cold
derivation gets harder with every file that lands.

## 8. What is op's, and what the experts can still settle

I am not answering any of the first group, and I have marked where I have a view.

### Op's

**Which reading of `Precise` he means.** `45:454-459` states the question well and I would put it to
him in `45`'s own words rather than rewording it. The process point is the one worth making here:
**`44`, `45` and `47` have each named this as cheap, one sentence, and op's, and across four files
nobody asked.** It is now the only thing blocking a live thread, `45` and `47` have both priced the
consequence of each answer, and it has been sitting fully prepared since file 44. If `50` finds the two
readings are compatible the question changes shape, so the order matters: `50` first, then ask.

**Whether alignment is a property of a strategy or an axis of its own.** `15:553-556` named it and
declined to resolve it, `45` built on it, `46` downgraded it, `47` relocated it, and nobody can settle
it because it is a design intent rather than a finding. Three files have now spent effort on a
conditional whose antecedent only he can discharge.

**Whether "container" and "numeral representation" are vocabulary the canon keeps.** This one nobody has
asked and I think it is real. `I1` demoted the four strategy names on op's own word, on the ground that
they are "a prior attempt at the intent, not the intent". The acceptance criterion's two nouns are
being treated as fixed vocabulary by every file in this unit, and they come from the same era and the
same author. The plural is his and the finding rests on it; whether the **words** are his intent or his
prior attempt is a different question and it has never been put. **Persona's view, marked as such:** I
would expect him to say the intent is fixed and the nouns are open, which is what he said about the
strategy names, and I would not act on that expectation.

### The experts', and they should not be escalated

Which criterion the panel is using and what fact set it produces (section 5). Whether the `Precise`
fork is a fork. Whether the fact set is closed under an open strategy set. Whether `47`'s section 4
result survives without its model. The three cheap unclaimed checks. The second read on the two-ladder
partition and the kind asymmetry. The packed access magnitude, which is a bench and by `RULES.md:50-60`
is explicitly not his.

And one that is neither: **the consolidation's rung line.** Section 1 shows the unit's rung has been
promoted by restatement twice already, once in `44`'s own section 7 and once in the brief that
dispatched me. Whoever writes `53` should derive the rung from the files rather than from any account
of them, including this one.

## 9. What I could not determine

**Whether `OPTIONS.md` currently states any of this correctly.** I did not open it. Three files report
that it moved under them to absorb corrections before those corrections had a second reader, and I am
relying on their accounts. A `21`-style entailment pass over the register against its cited sources has
been named as valuable by `44:447-451` and has still not been run; it is a genuine candidate for a slot
if `49` through `52` shed one.

**Whether `45_probes/p3` and `p6`'s numbers reproduce.** Nobody has opened them except `46`, which
hand-checked one row of `p3`. They are the most quotable figures in the unit and the least checked, and
I did not check them either.

**Whether section 5 is right.** It is one reader on two passages. If a second reader finds `16`'s
criterion reads consistently and I have misread it, most of section 5, the second half of section 6, and
slot `50`'s brief all move. That is the specific risk in this file and it is the largest one.

**Whether the shape I named at the end of section 5 has been tried before.** I did not read `02`
through `43`, and a three-question decomposition is an obvious enough thing that it may already be on
`DROPLIST.md` with a diagnostic. I did not check, which under this panel's own rules means I may be
proposing a dead route. Whoever briefs `50` should grep the droplist before handing it on.

## 10. Persona calls made in this file

For `PERSONA_CALLS.md`, all persona-decided and none of them binding on anyone.

1. The unit's rung is ONE EXPERT on the count with a declared leak, TWO EXPERTS on the content, plus
   three reads. The brief's "derived independently by `45` and `46`" is withdrawn.
2. `47`'s proposed permanent sentence is kept and is not sufficient as the topic's canon sentence.
3. The second four are ordered: cold derivation, criterion, second reads and cheap checks, bench.
4. The `Precise` question goes to op **after** `50`, not before.
5. I am not committing the dirty probe in `45_probes/`, and I am naming it instead.
