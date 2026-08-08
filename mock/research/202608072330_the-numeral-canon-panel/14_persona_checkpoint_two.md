# 14. Persona checkpoint on the second stretch

**Date:** 2026-08-08, overnight. **Author:** the `orgrinrt` persona, standing in while op sleeps.
**Status:** PERSONA. No authority. Nothing here ratifies, nothing here settles, nothing here is op's
word. Per `04` and `RULES.md`, every call below is the persona's and is logged as such in
`PERSONA_CALLS.md`.

Where I am guessing at what op would think rather than reading it in the record, I say so inline with
the word **guessing**, and section 10 lists every instance in one place. Where op's recorded words
appear to cut against my instinct, his govern and I write the conflict down rather than resolving it.

**What I read.** `04`, `01`, `RULES.md`, `PERSONA_CALLS.md`, `09` (my predecessor), then `10`, `11`,
`12`, `13` in full in that order, then `MORNING.md` line by line against the four. `SETTLED.md` in the
passages a claim sent me to, plus its git history. I did not read `CANON_CANDIDATE.md`, `DROPLIST.md`,
`02_carried`, `seed/`, or the predecessor panel's tree except where a citation sent me. Section 11
says which probe outputs I opened and which numbers I reproduced with my own commands.

## 1. What I would put in front of op, and it is not what MORNING leads with

Three, in this order.

### 1.1 The stretch has proven a type-level skeleton, and MORNING says the design work is done

`MORNING.md:14` is the sentence op reads first:

> What remains is a choice between compiled costs, and it is yours.

That is false, and the four files it summarises say so themselves, in their own coverage sections,
each of which I read in full.

`13:493-494`:

> I did not verify that arithmetic *operations* (as opposed to shapes) work through the containers;
> the probes carry shapes and sizes, not addition on the values.

`12:496-497`:

> I did not build the strategy axis into any candidate. Every probe carries `Hot` or `Warm` as an
> inert marker, and how a strategy interacts with the door or with base ten is untouched.

`12:499`: no signed case, no fractional-only case, no zero-width numeral, in any candidate. `13:494`:
no signed numerals, no strategy beyond a phantom, no `Cold` packing, no wide-rung alignment.
`10:552-555`: the strategy axis untouched, `Cold`'s bitpacked path a source reading rather than a
compiled result.

So what has been compiled across `10`, `11`, `12` and `13` is: a width reaches a container, the
container has the right size, two width types add to the right width type, and a mismatched width
refuses. Nobody has added two numbers through any of it. Every arrangement is a skeleton of shapes
and sizes with an inert marker where the strategy goes.

That is a large and genuine result. It is not "a choice between compiled costs", and the difference
matters because the second phrasing tells op the mechanism question is finished and only a preference
remains. **Guessing** on the reaction, from `01:96-98`, where he says the answer comes from someone
being confident enough to build the thing: I think he would be sharp about being handed a decision
slot when the axis his substrate is built around has not been in a single probe.

**And there is a specific hole inside that, not merely a coverage gap.** Op's own words, quoted at
`11:871-873` from `130b:39-43`:

> Container naming is explicitly wrong. The entire idea of arvo is that the strategy guides container
> selection, not the user.

If the strategy guides container selection, the derivation the design needs is at least
`(strategy, width, sign) -> container`. Every ladder built this stretch is `width -> container`.
`10:194-196` handles this by asserting the strategy decides where the crossover sits and is therefore
an input, which is plausible and is not built or tested anywhere. `SETTLED.md:96` carries "the wide
payload is a strategy consequence rather than a separate mechanism" as RATIFIED, which says the same
thing and also does not make it true of any probe here.

**`MORNING.md` does not contain the word "strategy".** I grepped it: zero hits, case-insensitive,
across 468 lines. Three of the four files name the axis as uncovered; the summary op reads names it
not at all.

### 1.2 The two stretches have not been joined, and where they touch they collide

`06` found that negative integer width has a caller, at 15 of 6561 pairs, and `MORNING.md:274` and
`:292-295` carry it as a live result of the first stretch.

Every width representation built in the second stretch is a **binary natural**. `10`'s and `11`'s
ladder is `D0`, `D1`, `Term`. `13`'s is `Z`, `O<n>`, `E<n>`. `12` inherits `11`'s. None of them can
spell a negative integer width, and none of them says so.

I grepped the four files for "negative integer width" and "negative width": **zero hits in all four.**
The only "negative" in `13` is a negative control, and the only ones in `MORNING` are in the first
stretch's sections.

So the first stretch's own conclusion is that the width coordinate must go below zero, and the second
stretch has built four arrangements whose width coordinate is a natural, and nobody has put those two
sentences next to each other. This is not a detail to schedule. If the shape space genuinely needs
negative integer width, then every arrangement in this stretch is a candidate for a numeral the design
has already established it does not have, and the fix is not obviously cheap: a signed structural nat
is a different ladder, and `13`'s ceiling-division, comparison and container selection are all defined
over the natural encoding.

I am not claiming the arrangements are dead. I am claiming that nobody has checked, that the check is
one probe, and that it is the cheapest way available to find out whether the night's headline survives
contact with the night's own earlier result.

### 1.3 The table survives every candidate, and the lead paragraph does not say so

`SETTLED.md:110`: no enumeration, ever, if it can be helped, **RATIFIED, four times**.

`12:341-348`, stated plainly by the file that produced the headline:

> the choice is not between a design with a table and one without. All of them have one except C1, and
> C1 fails the bar. What differs is **how much the table has to cover**

`13`'s arrangement A carries 65 bridge rows (`13:243-245`). `13`'s arrangement D carries the same
forward table. `12`'s C4 carries a door. `10`'s `137` carries eleven to thirteen rows. The thing op has
refused four times is present in every surviving candidate.

`MORNING.md:104-107` does say this, correctly, on line 104. `MORNING.md:10-14`, the paragraph op reads
first, does not. It says the const surface "forces a table", that the table "cannot be closed", and
that "a nat-keyed surface removes the ceiling while meeting your ergonomics bar character for
character". A reader stopping after the lead paragraph will believe the table went away with the
ceiling. It did not; its domain shrank from unbounded to the literals in the source text.

That is still a real and valuable finding, and it is the one `12` and `13` actually earned. It should
be the sentence in the lead, because it is both more accurate and more defensible than the one that is
there.

## 2. Does `13`'s independence claim survive inspection

Mostly yes, and the strongest evidence for it is something `13` does not claim. I checked in both
directions rather than taking the file's word, because the whole point of the corrected dispatch shape
is that the claim is checkable.

### 2.1 What supports it

**The probe ordering holds.** `13:10` invites the check against probe timestamps, so I ran it. The
`.out` files run `p01_the_wall.out` at 04:11:10 through `p27_declared_too_narrow_diag.out` at 04:21:59,
then a gap, then `p28_both_forms.out` at 04:25:55 and `p29_extension_without_towers.out` at 04:26:29.
That is exactly the order `13:18-22` states: derive and build `p01` through `p27`, then read, then
`p28` and `p29`.

**Nothing it read before deriving carried the claim.** `13`'s step-one reading list is `RULES.md`,
`01`, `04`, `SETTLED.md`, and `seed/SETTLED_surface.md:135-159`. `SETTLED.md`'s last edit before `13`
started was commit `b09b487` at 04:01:41, and I read that diff: it changed exactly one row, the
ergonomics bar row. Grepping `SETTLED.md` for `ceiling`, `closed under`, `law algebra`, `outgrow`,
`octave`, `b01`, `b02`, `b03` returns **nothing**. `11`'s closure finding was not in `13`'s reading
list in any form.

**The vocabulary is disjoint.** `12` names its candidates C0 through C4; `13` names arrangements A, B
and D. A file working from another file's structure inherits its labels.

**And the best evidence, which `13` does not point at.** `13`'s `p19` reached a conclusion `12` had
already refuted: it turned `lazy_type_alias` on over the whole ladder, got a fifteen-error cascade, and
called the route closed. `12:399-420` had already written the bounds that make it work and measured the
cost. `13:578-584` then withdrew its own conclusion after reading `12`. `p19_lazy_type_alias.out` is
34,070 bytes, timestamped 04:18:12, before the read.

**A derivation that copied its predecessor does not manufacture a wrong answer the predecessor already
fixed.** That is a stronger independence signal than any agreement in the file, and it is the one thing
here I would call decisive.

### 2.2 What does not support it, and one section that is inflated

**The `.rs` sources are useless as evidence.** Every one of them carries mtime 04:31:14, to the second,
which is after `13`'s own deliverable was written. Something rewrote all of them in one operation,
almost certainly to add the reproduction headers `RULES.md` asks for. Benign, and it means only the
`.out` files carry usable timestamps. Worth saying so before someone cites the source mtimes.

**The prose ordering has no artifact behind it.** `13` was committed once, at 04:31:13. `RULES.md:168`
requires writing to disk early and extending in place, and if that happened there is no intermediate
state to check. So "everything above was on disk before I opened `12`" rests on the member's word plus
the probe ordering. The probe ordering supports it. Nothing independently confirms the prose.

**One section is inherited and presented as derived.** `13`'s "Reading the bar from the establishing
text, not from the compression" (`13:32-58`) offers as its own reading the two things it says
`SETTLED.md:109` loses: that op's own sentence names two disqualifiers while four more are the panel's,
and that "disqualified at that site specifically" was dropped. Both of those had been written **into
`SETTLED.md:109` itself** ten minutes before `13` started, by commit `b09b487` at 04:01:41, at `12`'s
prompting. `13` read that row at step one. So that section restates a row it was handed and reads as an
independent reading of the establishing text.

Nothing downstream moves: `13`'s answer satisfies the bar under both readings and it says so. But the
rung on that section is ONE EXPERT, inherited, and the file's framing does not say so.

### 2.3 What the rung actually is

The claim MORNING headlines, that a nat-keyed surface meets the bar with the spelling unchanged, is
genuinely **TWO EXPERTS**, derived in the correct order, and it is the first one this panel has earned.
That is worth saying plainly, because the corrected dispatch shape from `09` was applied once and it
worked, which is the strongest argument for keeping it.

And the related claim, that the table's domain is bounded by the program's source text, has **three
independent instances** with an argument for their independence at `13:554-557`: `11` reached it by
removing the const keying from its own failing case, `12` by substituting a door into `11`'s case, and
`13` by asking what the algebra consults. That clears `RULES.md:116-118`'s three-instance evidence bar,
which almost nothing in this panel does.

MORNING headlines the two-expert rung and does not mention the three instances. It has undersold the
one thing it could legitimately headline while overselling everything around it.

## 3. Is the conclusion oversold in `MORNING.md`

Yes, in four specific places, and one of them is a self-contradiction.

**`MORNING.md:14`.** "What remains is a choice between compiled costs, and it is yours." Section 1.1.
The costs compile; they are not priced; and what remains is most of the design rather than a choice.
Every one of the four files says "unpriced" of itself and `MORNING.md:443` repeats it, which makes the
lead paragraph's phrasing harder to defend rather than easier.

**`MORNING.md:10-14` omits the table.** Section 1.3.

**`MORNING.md:98-107` commits the exact error it later reports.** Its heading reads "The resolution of
the trade, and it is cleaner than a trade", and the body says "**But the two are not in conflict**, so
there was no trade to be offered." That is `12` section 9's headline sentence. `13:636-646` flags that
exact sentence as a compression hazard, because `12`'s own section 8 says C4 pays the diagnostic unless
one of two repairs is taken and each repair has a price. `MORNING.md:167-168` then reports `13`'s
warning: "`12`'s headline reads stronger than `12`'s own table two sections above it. Worth knowing
before you read that file."

So MORNING carries the overstated sentence as its own section heading sixty lines above the place where
it warns op about that sentence. Reporting a hazard and committing it in the same document is worse
than doing either alone, because the warning reads as evidence the compression was checked.

**`MORNING.md:16` treats a rung as a settlement in a night where settlement is forbidden.** "**This is
the one claim of the night at TWO EXPERTS.**" `04:35-37` is explicit: "**even convergence between
experts does not settle anything tonight.** Convergence is a result to present, not a conclusion to
adopt." Set beside line 14's "what remains is a choice", the pair reads as: two experts agree,
therefore the mechanism question is closed and only your preference is outstanding. That is `01`
section 0's failure arriving from the other end. Op wrote section 0 to stop the panel filing his acks
as closures; this files a convergence as one.

**And one defect `09` found is still there.** `MORNING.md:353`: "That is two instances arrived at
differently, which is the bar." `RULES.md:116`: "Three independent ones is the bar." `09:132-136`
flagged this precisely; six of the seven things `09` itemised were repaired in commit `5de4d51` and
this one was not.

### 3.1 The self-contradiction, which is the plainest defect in the document

`MORNING.md:176-178` opens a section: "## Your standing instruction, taken at last, and it went well.
Nobody had taken it for four dispatches. `10` did, and the answer is better than the record suggested."

`MORNING.md:447-452`, under "What is still open, and what is running":

> **Op's one standing instruction is the one nobody took.** ... Four dispatches ran and none located
> the attempt. There is also a live contradiction inside it: op's recollection ... is flatly
> contradicted by `SETTLED.md` ... opening the candidate is one probe's work.

`10` located the attempt, opened it, resolved the contradiction as a summarisation defect on both
sides, and `MORNING.md:186-192` says so and says the correction has landed in `SETTLED.md`. The
still-open section is a verbatim survival of the pre-`10` state, including the resolved contradiction
restated as live and the "one probe's work" that has been done.

This is the third time the open list has gone stale. `09:168-173` found it stale after `06`; it was
repaired; it is stale again. The pattern is that the narrative sections get rewritten as each file
lands and the open list is appended to rather than re-derived. Re-deriving it from the four files each
time is cheap and is what stops op reading a discharged instruction as outstanding.

### 3.2 `MORNING.md` has discarded every address

The mechanical check `RULES.md:153-157` asks for, and which `09:445-448` said was owed and cheap:

```
grep -oE '`[0-9a-zA-Z_./]+:[0-9]+' MORNING.md | wc -l   ->  0
```

Zero. Against 27 in `03`, 10 in `06`, 14 in `07`, 13 in `08`, 38 in `10`, 39 in `11`, 10 in `12`, 7 in
`13`. One hundred fifty-eight anchors in the sources, none in the summary.

And it is worse than the anchor count suggests:

```
grep -oE '`[pbeicd][0-9]+[a-z]?`' MORNING.md | wc -l    ->  0
```

Zero probe names. MORNING states roughly thirty numbers, including 1148 rows and 4758 lines, 512 of
1024, 6100 of 6561, 15 of 6561, a 4225-pair matrix, 95 instructions, 1002 bytes, 16 to 34 percent, and
34,976 pairs, and not one of them names the probe or the command that produced it. `RULES.md:123-126`:
"**Counts are measurements.** Produce every number with a command and say which command. The prior
panel propagated two floating numbers nobody could reproduce."

This is exactly the compression failure `a-compression-is-checked-by-someone-else.md` describes:
"A compression preserves prose and discards addresses, because addresses mean nothing to the compressor
and are the whole value to the reader." It is happening in the document op reads first and the one a
consolidation will be built from.

A map does not need 158 anchors. It needs one per headline number, which is one token each.

## 4. What is being avoided now

`09` asked this of the first stretch and found op's standing instruction untaken by four dispatches.
`10` took it. So the honest answer this time is different and smaller, and it has three parts.

**The four clauses, stated accurately, because the brief I was handed gets this wrong.** The gate at
`SETTLED.md:65-71` has four: the consumer expresses usage in bits and bytes, the typestate derives
container and representation, it validates, and it erases on lowering. `09:294-300` found the first
stretch exercised clauses three and four and not one and two. This stretch went at one and two, which
is the correct complement, and touched three and four again in passing (`12`'s post-monomorphisation
check, `13`'s comparison refusals, the assembly reads in `10`, `12` and `13`).

So all four clauses now have evidence. What they do not have is evidence at the right arity or on the
right objects: clause two was exercised as `width -> container` rather than
`(strategy, width, sign) -> container`, and clause four was exercised on identity functions and shape
skeletons rather than on arithmetic. That is a sharper statement than "clause N was skipped" and it is
the one the record supports.

**`13` is the only file to read clause one as naming two units.** `13:465-474`: "**Bits and bytes**,
both. ... a later member reading 'bits and bytes' as a synonym pair will not notice that the gate names
two units and the current surface offers one." That is a good catch, it is on op's own sentence, and
nobody has built it. It stays open.

**The one thing op actually asked for is in worse shape than when he asked.** `01` section 1: he wanted
the consequences of one-family versus several laid out before ruling, and picked the comparison over
the three options. `03` delivered it. `06` then reframed the question operationally, `07` showed the
join and the product are one function at different arguments, and `08` showed `03`'s antichain is an
artifact of the named shapes rather than of the concept. `MORNING.md:457` now files it as "the family
question itself, now reframed twice."

He asked one question, was handed a comparison, and the comparison has been reframed twice by the files
that came after it. That is legitimate panel work and it is also the plain fact that the thing he asked
for is not ready to hand back. It should be said in those words rather than left as a clause in the
open list, because the alternative is that he reads three superseded framings looking for the answer he
asked for.

## 5. Per file

### 5.1 `10`

**Holds, and it is the best single result of the night.** It took an instruction four dispatches walked
past, located the artifact, and resolved a contradiction the panel's own index called "the largest
structural gap" into a summarisation defect on both sides. Op's four properties: three hold as stated,
and the fourth holds of the ladder and fails of the bridge, which is the one item he personally
rejected at `137b:28-41` three months of panel-time ago. That is a memory doing an ordinary thing, and
`10` says so without either flattering or embarrassing him.

The counting that carries it is the right kind: `grep -cE 'impl .*<[0-9]+>' p5_total_ladder.rs` returns
0 and `grep -oE '[0-9]+ =>' p6_surface_end_to_end.rs | wc` returns 11. Two commands, and the
contradiction dissolves.

**Second, and it corrected its own brief before reasoning from it.** `10:56-80` grepped the closed panel
for "fresh eyes", found ten hits, and found the only call of that shape attaches to the headroom rule
rather than to the container derivation. It then identified `137` from op's four properties and flagged
that identification as its own inference. `RULES.md`'s "the first thing a panel does is try to break its
own brief" working exactly as written, and it is worth noting that the brief it broke came from `09`,
which is mine.

**Third, a defect nobody had ever read.** `10:290-300`: rustc dumps arvo's entire shipped width table
into the consumer's error output, sorted lexicographically, so a consumer sees "Idx<0> Idx<100>
Idx<13> Idx<16> Idx<200>" at every failed build. Found by reading a diagnostic. It appears nowhere in
the record.

**Thin.** The four improvements are presented as pure gain at identical codegen, and `11` found one of
them prints a false sentence. `10:242-243` says why it slipped: the control chose a wrong width that
was in the table, "so the bridge does not mask the law", which is the right control for the case being
tested and precisely the case where the two failure modes cannot be told apart. Honest, and it is the
tell that a second control was owed.

**Would push.** `10:474-490`, the dense-table residue. It is marked correctly as a residue and not
proposed, and `11:597-600` then reclassified the whole route as **structurally insufficient** rather
than merely refused, because no finite table is closed under the algebra. The residue should not
survive that reclassification with its timing table intact. A route that is dead on structure does not
need a spike showing 8193 rows compile in 3.11 seconds; carrying the number invites someone to quote
it.

### 5.2 `11`

**Holds, and section 5 is the most canon-shaped thing produced in two stretches.** The table of what
each system pays for the property that removes the bridge is permanent, it survives a total rewrite,
and it converts arvo's table from an embarrassment into a currency. `11:148-157`'s sentence, that the
bridge "is the shadow of pre-instantiation checking", is the reason rather than the symptom, and it is
the kind of sentence `RULES.md:79-83`'s permanence test exists to select for.

**The typenum finding, which I reproduced rather than trusted.** Both commands, on this machine:

```
grep -c '^impl ToUInt for Const<' .../typenum-1.20.1/src/gen/generic_const_mappings.rs   ->  1148
wc -l < .../typenum-1.20.1/src/gen/generic_const_mappings.rs                             ->  4758
```

Exact. The ecosystem shipped arvo's three declarations under different names, arrived at
independently, and that is genuinely three instances of the shape counting `137`, typenum and `10`'s
re-derivation.

**The closure argument, and it is the night's largest structural finding.** `b01` fails at the first
multiply of two tabled widths; `b02` adds the row by hand and fails one octave up; the argument then
does not depend on the table's size. I opened the committed logs rather than trusting the file, and
`11_probes/out/b01_table_caps_the_algebra.log:30` and `b02...log:59,80` carry the errors as quoted.
And `11:592-596` connects it to the objection that already killed the fixed-width carrier, which is the
part that makes it a design finding rather than an inconvenience.

**Two of its own predictions refuted and recorded** (`11:1075-1077`), which is the discipline that makes
the rest of its numbers worth reading.

**Thin.** Five of the ten survey entries are recollection with no toolchain, and `11` says so and names
Ada as the one it most wants checked, correctly, because Ada carries P4 and P4 is the only available
property. Sections 3.3 and 3.4 sit a rung above that, "checked against the docs" and "checked against
the source", resting on fetches with nothing committed. Those two carry section 4's property taxonomy,
which is the file's spine.

**Would push, on evidence discipline.** The typenum counts are the most-quoted number in the file, and
`MORNING.md:52` headlines them. They rest on `~/.cargo/registry/...`, which is not in the repo.
`RULES.md:108-110` says evidence lives in the repo or it never happened. I checked and both numbers are
exact, so the claim stands and the remedy is the mechanical one from
`evidence-lives-in-the-repo-or-it-never-happened.md`: copy the extract, or the head plus the sorted row
list, into `11_probes/` and commit it. No audit pass, no rewrite. Same for the GHC and Bluespec fetches.

`grep -rl typenum 11_probes/` returns nothing today.

### 5.3 `12`

**Holds, and section 7 is the finding I would keep from it.** Not the surface comparison, which is
excellent, but this: an undeclared width written at the alias-definition site produces **no error at
all**, under the current design and under every candidate, because a Rust type alias does not check its
bounds. The error lands at the first use, spanning a name the consumer never wrote, citing an internal
type they have never seen, in a different file.

`12:391-393` gets the attribution right: "**This is a defect of the const door and it belongs to the
design as it stands, not to the hybrid** ... It has simply never been looked at, because nobody had
compiled the tier-two experience as a tier-two consumer would meet it." That is the exact site the
ratified ergonomics bar was written about, and it was found by compiling what a consumer actually
types.

**Second, and it changes the shape of the question.** `12:64-67`: the bar never said the width must be
a const generic parameter, it said `UInt<5>` must be what the writer types, and a type alias with a
const parameter decouples the two. Byte-for-byte identical spellings at every tier, measured off
compiling text with a committed count script. That is the result the night turns on.

**Third, a fact nobody had recorded** (`12:236-240`): a type parameter default may be a projection off
a const parameter of the same struct.

**Thin.** `p08`'s two-head split is the thing that makes the whole composition work and `12:504-507`
says it was "not built as anything a consumer would want to use", with a `.derived()` call at the entry
to every law and the question of hiding it behind a bound left open. And `lazy_type_alias` is a
**consumer-side** feature, which `12:420` calls "a serious constraint on a library and I have not
thought it through". Both are load-bearing and both are unworked.

**Would push, on the canon gate.** `12:9-22` finds the RATIFIED ergonomics-bar row possibly not
terminal, under `01` section 0, and proceeds with a qualification rather than stopping.
`RULES.md:198-200` says ambiguous means stop and hand the call back. `12` neutralised it properly
(`12:478`: "I do not need that distinction for anything here, because C4 satisfies both readings"), so
nothing rests on it and I would not call it a violation.

But `13` did the same thing independently in the same stretch (`13:56-58`), and both happened not to
need it. **Op wrote `01` section 0 to stop the panel filing his acks as closures. It is now being used
by experts to reclassify closures as acks.** That is the inverse failure, it appeared twice in one
stretch, and the third expert to reach for it will be the one who needs it to hold. This is worth a
sentence from op about how far the solvent reaches, because if every RATIFIED row is re-openable on
"the record does not show convergence", then `SETTLED.md` carries nothing.

### 5.4 `13`

**Holds.** The independence audit is section 2 and I will not repeat it. On substance, three things.

`13:186-196`: type-level addition normalises, checked over the **whole** 4225-pair matrix generated by
a committed script, with a negative control that refuses. Against three sampled points previously. That
is `a-test-that-cannot-compile-is-the-finding.md`'s whole-matrix discipline applied without being asked,
and it is the right instrument for an identity that everything else rests on.

`13:588-597`: joining the ceiling to the reverse wall. `11` localised the ceiling to the const surface,
`12` separately closed the reverse crossing, neither joined them, and joining them gives the general
statement: an operation's output can only be **named** by crossing back, that crossing is the same
refusal as the forward one, and a table is the only implementation of either. The rule it draws,
"**cross once, at literals, in one direction**", is short, permanent, and passes both of `RULES.md:79-83`'s
tests. If one sentence from this stretch ever reaches a canon, I think it is that one.

I opened `13_probes/p24_reverse_wall.out` rather than trusting the citation: the refusal is real, and
`= note: type parameters may not be used in const expressions` is the load-bearing line.

`13:499-524`: the extension price. Nobody had said what a consumer types to add a bridge row, the
obvious guess is a hand-typed digit tower, and it is not: it is readable arithmetic over shipped
widths, checked against the algebra. Found while trying to break its own arrangement D rather than
while supporting it, which is the direction that makes a finding worth something.

**Thin.** Arrangement D is one instance, and `13:409-411` and `13:620-626` say so without hedging: tier
one's inference story, whose whole premise is `T: Add` with no typestate, is not worked out. That is not
a corner. It is the tier with the most consumers.

**Would push.** `13:670-673` puts five options under "What is op's": the ceiling, the diagnostic, a
second head constructor, a base-ten ladder, or declared outputs. That is a menu, and `RULES.md:65-66`
is explicit: "Bring him a converged thing, with the angles considered and the alternatives laid out,
and ask then." Five compiled arrangements is an excellent breadth result and a poor thing to file under
op's name, because the experts have not converged and `04` forbids treating their agreement as closure
anyway. The five belong on the map as alternatives; the question that belongs to op is narrower and I
do not think anyone has isolated it yet.

## 6. What I would refuse

Five, stated as the persona's and carrying no weight beyond the argument.

**Refuse `MORNING.md`'s lead paragraph as written.** Sections 1.1, 1.3 and 3. The two sentences to fix
are "what remains is a choice between compiled costs" and the omission of the table. The replacement I
would offer, and it is stronger rather than weaker: the const width surface forces a table whose domain
is unbounded, and a nat-keyed surface bounds that domain to the literals in the source text while
keeping the consumer's spelling character for character. The table does not go away, the ceiling does.
Three independent instances support the bounded-domain half and two experts derived the ergonomics half
in the correct order.

**Refuse the five-arrangement menu going to op under "what is yours".** Section 5.4. Present them as
what they are, five compiled alternatives on the map, and let the next dispatch narrow them. Handing him
a five-way pick before the experts have stopped disagreeing is the shape `01` section 0 rules out.

**Refuse the dense-table residue keeping its timing table.** Section 5.1. `11` closed route 13 on
structure. A spike showing 8193 rows compile in 3.11 seconds is now a number attached to a shape that is
dead for reasons compilation cannot touch, and numbers travel.

**Refuse base ten as a live direction.** `12`'s `p09` and `p10` are real work and they are complete, and
they buy a readable digit tower for roughly sixty impls. `13`'s arrangement D prints `expected 13, found
12` with no tower at all, at zero impls, on the same pin. Base ten belongs in the trail as the thing
that was built and beaten, not on the list of things op weighs.

**Refuse `12`'s C2 and C3 appearing as candidates.** `12:342-345` says both are refused shapes on their
face: a shipped `N0..N64` alias layer is the width table, and a declaration-site macro is the macro
escape, both inside `SETTLED.md:110`'s four-times refusal. They are useful as measured comparators and
they are not candidates, and a five-row candidate table invites them to be read as such.

## 7. What is genuinely good, specifically

`RULES.md:99-101` makes keeping something a result, so this is not padding.

**The corrected dispatch shape paid for itself on its first use.** `09` found the TWO EXPERTS rung
unreachable by construction and proposed a dispatch change rather than a rule change; `RULES.md:175-192`
was written; `13` was dispatched under it; and the panel now has its first genuinely independent
agreement, checkable against probe timestamps, on the claim the night turns on. That is the single most
valuable thing to come out of the checkpoint mechanism and it should be said before anything else about
process.

**`10` discharging the untaken instruction, and the manner of it.** It broke its own brief first, it
corrected op's recollection carefully and in the direction the record supports, and it resolved a
contradiction two documents deep by opening the artifact and running two greps. That is what the panel
is for.

**The evidence discipline held across all four, and I checked it rather than assuming.** 261 probe files
committed across `10_probes`, `11_probes`, `12_probes` and `13_probes`, all tracked, nothing untracked
in the panel directory except this file. `11` and `13` each ship a `verify.sh` that reruns everything in
one command with its output committed; `13`'s last line reads `unexpected outcomes: 0`. Every probe
output I opened matched the file citing it: `11`'s false-sentence logs, `13`'s reverse wall, `11`'s `e04`
terminal refusal, `12`'s verify table. Two members recorded their own refuted predictions rather than
dropping them. One withdrew a conclusion after being shown wrong.

**`11` section 5 and `13`'s "cross once, at literals, in one direction".** Both are permanent sentences
that survive a rewrite in another language, which is rarer than it sounds and is what a canon is made
of.

**`12` section 7.** The finding nobody was looking for, on the exact site the ratified bar governs,
attributed to the design as it stands rather than to the proposal it was testing.

**And one thing about the register.** All four files marked their ad-hoc spikes as ad-hoc spikes and
used the word "unpriced" where `RULES.md:119-122` reserves it. Nobody reached for a magnitude they had
not measured. Measured against the predecessor's two floating numbers nobody could reproduce, that is a
clean stretch, and the one place the discipline slipped is `MORNING.md`, which is the dispatcher's
document rather than an expert's.

## 8. What the dispatcher has got wrong

Stated separately because it is about how the panel is run.

**`MORNING.md` has zero addresses.** Section 3.2. Zero `file:line` anchors, zero probe names, roughly
thirty numbers. `09:445-448` said this check was cheap and owed and it was not run. This is the one item
here I would fix before op wakes, because it costs one token per number and the document is what a
consolidation gets built from.

**`MORNING.md` contradicts itself about the standing instruction.** Section 3.1. Op reads "taken at
last, and it went well" at line 176 and "the one nobody took" at line 447, with the resolved
contradiction restated as live.

**`MORNING.md:353` was flagged by `09` and not repaired.** Six of seven were; this one merges the
two-expert provenance rung with the three-instance evidence bar, and merging them is what let the lead
paragraph treat a convergence as a closure.

**`MORNING.md` never says "strategy".** Section 1.1. Zero hits in 468 lines, on the axis arvo's own
identity is built around, in a stretch entirely about deriving containers.

**The brief handed to me misstates the clause coverage, for the second time.** It says "the panel has
now spent two stretches on the first two [clauses]". `09:294-300` found the opposite of the first
stretch: it exercised clauses three and four and not one and two. `09:302-305` already caught one
inverted clause attribution in a persona brief ("the brief I was handed says 'Almost nothing this
stretch touched' the erasure gate. That is wrong on the record"). Two for two. A checkpoint aimed at a
wrong absence spends itself confirming the record instead of testing it, and I lost time here rederiving
what `09` had already written down.

**`PERSONA_CALLS.md` is wrong about its own subject.** It records the `09` entry with "**What was done
with it:** nothing yet. Delivered for op's morning. Recommended `MORNING.md` edits are recommendations
and were not applied." Its mtime is 02:30. Commit `5de4d51` at 02:32:25 repaired the MORNING defects and
commit `cf9a55c` at 02:32:58 applied the dispatch-shape fix, both within three minutes. The file whose
entire job is keeping the persona's calls distinguishable from op's is now inaccurate about what
happened to them, in the direction of understating the persona's effect on the panel. That is the wrong
direction for that file to be wrong in.

**Two of four members did not report the canon gate.** `RULES.md:196` says every member runs it before
its assigned work. `11` and `12` state it. `10` and `13` do not. Both did equivalent work in substance,
`10` by checking and correcting its brief and `13` by stating its reading order, and neither is a
substance failure. It is a reporting gap and it should not quietly become the norm, because the gate's
value is that its absence is visible.

**And the framing in the brief matches `MORNING.md`'s lead.** The brief describes the stretch as "a
surface was found that removes the ceiling while meeting the ratified ergonomics bar". Same sentence,
same omission of the table, in two documents written by the same hand. That is what drift looks like
before anyone has done anything wrong: one framing, held consistently, propagating into the instrument
built to check it.

## 9. What I would drop

**The 81-versus-zero discrepancy, again.** `09:405-409` said it should be dropped or promoted to a task
and not left where it is. `MORNING.md:465-467` now says "It must be promoted to a task or explicitly
abandoned before any consolidation", which is the recommendation restated rather than acted on. Third
stretch, still sitting, still described as poison, and a consolidation is closer than it was.

**Route 15 and the `d01` bare-parameter carrier as anything but a closure.** `11:832-885` builds it,
finds two of its own predictions refuted, and then closes it as "route 11 wearing a byte count", refused
by op directly. The refuted predictions are worth keeping. The vectorisation result should not travel
without the closure attached to it, because "a byte array at align 1 vectorises identically" is the kind
of finding that gets quoted into a different argument.

**`11`'s `e01` as a candidate.** `11:994-997` disclaims it correctly and the disclaimer is easy to lose:
it is not a candidate, `SETTLED.md:95` is ratified, and eight-times overshoot is not a limitation to
accept. What survives is the sentence, that arvo is one `ceil` away rather than a language away, and
`MORNING.md:42-46` carries that correctly.

**The Moore completion, the `canonical_exponent` naming debt, and `03`'s reading F** stay dropped.
`MORNING.md:459-463` applied all three from `09` and they should not come back.

## 10. Where I am guessing

Collected so op can discount them in one place.

I am **guessing** that op would be sharp about "what remains is a choice" arriving with no arithmetic
run through any candidate and the strategy axis a phantom in every probe. The inference is from
`01:96-98` and from `130b:39-43` as quoted in `11:871-873`. It is an inference about his reaction, not a
reading of his words.

I am **guessing** that the width-is-a-natural collision in section 1.2 is load-bearing rather than a
detail. What I read is that `06` found negative integer width has a caller and that every arrangement
encodes naturals. Whether that refutes the arrangements, costs one more ladder, or dissolves under a
sign parameter is a question nobody has opened, and I am asserting it is worth opening first.

I am **guessing** that `13`'s "cross once, at literals, in one direction" is the sentence most likely to
survive into a canon. That is a judgement about permanence, built on two of `RULES.md`'s own tests, and
it is mine.

I am **guessing** that the `01` section 0 solvent will be reached for again by an expert that needs it
to hold. Two used it this stretch and neither needed it. The pattern is two instances; the prediction is
mine.

Everything else above is a reading of a file, a probe output, a git commit, or a command I ran, with the
line or the command cited.

## 11. Coverage, and what this checkpoint did not do

**What I ran myself**, rather than reading in a file that cites it:

- The `file:line` anchor count, with the pattern quoted in section 3.2, over `MORNING.md` and all
  eight expert files.
- The same pattern narrowed to probe names over `MORNING.md`.
- `grep -i strategy MORNING.md` (zero hits) and `grep -i "negative width\|negative integer width"` over
  `10` through `13` (zero hits).
- `git show b09b487` for the `SETTLED.md` diff, and `git log` on `SETTLED.md` and the panel directory,
  for section 2.
- `grep -n -iE "ceiling|closed under|law algebra|outgrow|octave|b01|b02|b03" SETTLED.md`, empty, which
  is what establishes `13` could not have read the ceiling claim.
- `ls -lT 13_probes/` for the timestamps, and the observation that every `.rs` carries 04:31:14.
- Both typenum commands, independently, in section 5.2. Both matched exactly.
- `git ls-files` counts for the four probe directories, and `git status` for untracked files.

**Probe outputs I opened** rather than trusting their citing file: `13_probes/p24_reverse_wall.out`,
`13_probes/out_verify.txt`, `11_probes/out/b01_table_caps_the_algebra.log`,
`11_probes/out/b02_the_table_chases_its_tail.log`, `11_probes/out/e04_overshoot_const_block.log`,
`12_probes/out/verify.txt`, and directory listings for all four members.

**What I did not do.** I did not recompile anything. I did not verify any enumeration's arithmetic. I did
not read `CANON_CANDIDATE.md`, `DROPLIST.md`, `02_carried`, `seed/`, or the predecessor panel's tree, so
where a file claims something is absent from that record I checked its own citation and did not
independently confirm the absence. I did not open `10`'s `p12`, `p16` or the assembly diff behind its
codegen-identity claim; I took that from the file. I did not check `11`'s C++ or Zig comparators, which
are the two entries in its survey that were actually compiled.

**Nothing here is priced.** No bench harness run bears on this file or on the four it checks, and I use
that word as `RULES.md:119-122` reserves it.

**Nothing here settles**, per `04`, including the corrections to `MORNING.md`, which are a persona's
reading of what four files say about themselves and should be checked against those files before either
is edited.
