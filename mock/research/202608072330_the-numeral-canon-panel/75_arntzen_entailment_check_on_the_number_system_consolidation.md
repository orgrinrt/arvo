# 75. Entailment check on the number-system consolidation

**Author lens:** Arntzen. Independent verification, tracing one claim through its whole lifetime
from establishing source to the document that will replace it, distrust of a compression written by
the person who believes it entails.
**Position:** the independent check on `74`, per `RULES.md:150-162` and `RULES.md:275-277`. Not a
panel member on the topic; I did not write any of `65` through `73` and this file forms no opinion
about the number-system concept itself. It checks whether `74` says what the nine files it compresses
say.
**Method:** three instruments, per the dispatch. Claim-by-claim entailment worked from the member
files forward (I read `65` through `73` in full, in order, before opening `74`). The anchor set
difference, computed mechanically. A live-option census run independently against the member files,
not against `74`'s account of them.

**Canon gate: passes, situation two.** `mock/canon/` does not exist and `mock/crates/` is empty
(`members = []`) by the declared mutation order; this panel is writing the first canon and there is
nothing to defend. `74` itself is not canon and this check does not promote it to canon; it is a
verification of whether a compression entails its sources, which is the standing obligation on every
consolidation in this panel (`RULES.md:275-277`, `RULES.md:332-334`) and is unaffected by whether the
canon exists yet.

**Test gate: no suite exists.** `mock/Cargo.toml` has no members. The substitute here is the check
itself: every citation below was opened, and section 5 states what I checked and did not.

## 0. What I read, and in what order

Read end to end, before opening `74`: `65` (both phases), `66` (both phases), `67`, `68`, `69`, `70`,
`71`, `72`, `73`, and the cited ranges of `INTENTS.md`, `RULES.md` and `OPTIONS.md`. Then `74` in full,
current working-tree state (`git status` shows an uncommitted diff on `74` at dispatch time; I read
the file as it stands on disk, which includes it, per the instruction not to repair the file while
reading it and not to reason about the diff's authorship).

Not read: `00_brief.md`, `01` through `64`, `DROPLIST.md`, `PERSONA_CALLS.md`, `seed/`, `archive/`, or
any probe source file. This matches `74`'s own stated exclusion list exactly, and I checked that match
before relying on it: `74:44-48` states the same "not read" set. I did not verify claims that route
through `63` or `00_brief.md` against those documents directly, because I did not open them either;
where a finding below turns on one of those, I say so.

## 1. Verdict, stated first

`74` is a careful compression. Every headline claim I traced back (the shape, the quantifier result,
the five crossing classes and their corrected universal, the membership/hosting split, the ownership
key, the pipeline's four verbs, the kernel, the two law families, all seventeen corrections in section
7, all five member-only options in section 6) resolves to the member sentence it claims to resolve to,
and I found no case where a claim was inverted, misattributed to the wrong author, or promoted past the
rung its source supports. The self-flagged least-certain items in section 12 are honestly scoped; I
checked item 1 (the five-instance quantifier count) by re-deriving the arithmetic independently and it
holds (section 3 below).

That said, this is not a clean pass. The anchor diff finds 94 member anchors absent from `74`, which
matches `74`'s own count exactly, and I traced every one rather than trusting the count. Most are
legitimate: either citations into files `74` declares out of scope (`63`, `00_brief.md`,
`DROPLIST.md`, `.claude/*`, probe sources), or duplicate line-range spellings of a passage `74` already
cites under a different range. But three are not reroutes. They are content genuinely dropped, one of
them a direct breach of `74`'s own stated editorial policy. Section 2 has them, with the establishing
source and what should have carried.

## 2. Three real content losses, not anchor variance

I traced all 94 missing anchors against `74`'s text before writing this section. The method: for each
anchor, find the passage in the member file it points at, then search `74` for the same claim under any
citation, including a different line range into the same file or a citation into a different member
that carries the same fact. Roughly two-thirds are `63`, `00_brief.md`, `DROPLIST.md`, `.claude/*`,
`mock/agent/*` and probe-source citations, all legitimately excluded per `74`'s stated scope. Most of the
remainder are duplicate-range citations to a passage `74` already cites elsewhere (e.g. `65:511-513` is
a wider spelling of `65:511`, which `74:921` does cite; `68:196` and `68:211` are `73`'s "through"
phrasing of the same range `74:196-211` cites as a dash-joined pair). None of those is a finding.

Three are.

**2a. `70`'s finding that `65`'s change-test literally misfiles its own hierarchy is dropped
entirely, and with it the disposition of `65`'s candidate 1.**

`70` runs `OPTIONS.md` Q19's discriminator mechanically against `65`'s own stated change-test
(`65:80-86`) and finds it does not classify the chain the way `65`'s own prose elsewhere wants:

> Third, `65`'s test, applied literally, misfiles or fails to file most of the chain. The committed
> application: dD, dQ and drho all classify as "system" (O moved); dC classifies as "format"; and dE
> is unclassified, because `65`'s representation clause demands that named values change and an
> encoding swap changes none... The charitable repair, dropping the values-changed conjunct, files dE
> correctly and still misfiles dQ, which `65`'s own section 3 wants under representation (coverage is
> listed as a representation property at `65:169-172`) and which the test mechanically assigns to
> system. (`70:117-127`)

`70` then disposes of it explicitly:

> Attacked and repaired rather than killed: `65`'s change-test. Mechanically, its three clauses
> classify only C cleanly, leave E unclassified under the literal text, and misfile Q against `65`'s
> own section 3... `65`'s candidate 1 ("the change-test of section 1 as the boundary" between the three
> concepts) should not enter a consolidation in that form. (`70:284-289`)

`74` correctly does not carry `65`'s candidate 1 into section 4; no candidate sentence resembling "the
change-test as the concept's boundary" appears. But that is the only trace of the disposition. `70`'s
finding is not cited anywhere in `74`, the internal self-contradiction inside `65`'s own document
(coverage filed under representation in prose, filed under system by the test) is not named, and
`74`'s own section 7, whose stated purpose is exactly "corrections the unit generated, for anyone
reading its files later" (`74:909`), lists eleven items and this is not one of them. Section 6's
"deliberately not offered" list under N1 does not name it either. The candidate sentence is correctly
absent; the reason it is absent, which is a real finding with a mechanical demonstration behind it, is
gone. A later reader who wants to know why `65`'s change-test does not appear in section 4 has nothing
to find.

This is exactly the failure mode `RULES.md` names for consolidations: a true sentence (the change-test
is missing from section 4) standing where content belongs (why it is missing). Repair: one line in
section 7, citing `70:117-127` and `70:284-289`, stating that `65`'s change-test misfiles its own
chain and that the repair keeps the instrument as a crossing/compatibility classifier (which is what
became N6 and N7) rather than a level boundary.

**2b. `71`'s explicit credit to `66` for correctly posing the conversion/resolution split is dropped
from N9, in direct violation of `74`'s own stated merge policy.**

`74:465-467` states the policy governing section 4: "Merging was done only where two files state one
claim; authorship is kept." N9 is exactly such a merge. `66` poses the conversion-versus-resolution
split and states, correctly per `71`'s own assessment, that resolution needs a rule rather than a
conversion function:

> What `66` got right and I would keep: resolution "needs a rule, not just a conversion function"
> (`66:329-331`). What the measurement adds is which rule. (`71:399-401`)

N9 states the resulting claim, sourced only to `71`:

> **N9, conversion and resolution.** *Moving one value into a declared system and combining several
> values from disagreeing systems are one obligation at two arities... * Permanence and equivalence:
> pass (`71:702-708`). ONE EXPERT. (`74:520-524`)

No citation to `66` anywhere in N9, its rung line, or its entry in section 3.3. `74` does credit `66`
correctly elsewhere for other claims (the crate-table attack, the three-role miscitation, the 952
mis-attribution), so this is not a pattern of erasing `66`'s contributions generally. It is one merge
where the stated policy was not applied. Repair: add `66:310-345, 66:329-331` to N9's sources, or add a
clause noting `66` posed the split and named its correct half.

**2c. `71`'s methodological aside about the limits of the type-system reflex is dropped without
trace.**

Section 4 of `71` builds a typestate contract proving both routes through a two-coordinate crossing are
well-typed and equally so, and draws a conclusion about the workspace's own standing discipline:

> One thing that is settled, and it is a bound on every option. The two orders are not equally
> principled but they are equally typed... The typestate cannot break the tie. This is worth stating
> plainly because the workspace's standing reflex is to push an invariant into the types until the
> wrong program is unwritable, and here that reflex does not reach: both programs are right programs,
> and only a canon sentence says which one the notation means. (`71:339-344`)

N8 keeps the substance ("the typestate cannot break the tie") but not the reflection about why this case
is worth flagging: it is a case where `harness-the-type-system.md`'s own discipline, ladder-climbed
correctly, still does not decide the question, because the question is not a type error, it is a naming
choice. This is a smaller loss than 2a and 2b, and it is arguably the kind of aside a canon-facing
document is entitled to drop since it is about workspace practice rather than about the number-system
concept. I flag it because it is the only place in nine files where a member explicitly names a limit
of the panel's own type-first reflex, and a document whose whole apparatus (N2's quantifier discipline,
N23's address-not-verdict framing) leans on that reflex working is a natural place to keep the one
recorded instance of it not working.

**Two smaller thinnings, not full losses, worth a sentence each.** `OPTIONS.md:1010-1053` (Q10's
inclusion predicate, `03`'s measurement of 188 disagreements at radices 2 and 3, sufficient always and
necessary only where the source carries at least two values) is flagged by `74` as live and unresolved
("Q10 gained a consumer... its requested second read has still not run", `74:904-905`) but the specific
evidentiary content behind Q10 is not carried anywhere in `74`; a reader is told the question is open
and unmeasured for a second read, not what the first measurement found. And `68:158-163`'s "layout
erasure is nearly a language tautology" becomes `74`'s "close to a language guarantee" (`74:350`),
which is a real but small wording shift: `68`'s point is that layout erasure establishes almost
nothing because `repr(transparent)` already guarantees it as a matter of Rust semantics, and "tautology"
carries that skepticism more sharply than "guarantee" does. Neither changes a conclusion; both are worth
a look if this file is ever tightened.

## 3. Verifying `74`'s own flagged risk: the five-instance quantifier count

`74:1038-1041` names this its highest-risk reconciliation and asks the check to look at it rather than
trust it. I re-derived it independently.

`67:114-121` catalogues, in `67`'s own words: "the panel has already paid for this three times... `61`
found `56`'s coherence law stated without the restriction... `57b`'s own `p7` then failed twice by...
the same over-quantification `61` caught... And `42`'s clamp-counting sentence was refuted for the same
reason." The natural parse of "three times" against three named events (`56`'s law, `57b`'s `p7`, `42`'s
sentence) is three, with `p7`'s double failure counted as one recurring instance rather than two, since
`67` states the total as three explicitly rather than four.

`67:252` states a universal quantified over five telescope classes while `67`'s own instrument measured
three (`67:143` names five coordinates; `67:236-274` measures indices 1, 2, 3). `71:66-89` locates this
and `72:43-53` accepts it, calling it "the fourth instance of the failure `67` section 1 catalogues
three of." That is 3 catalogued + 1 = 4, and `72`'s count is internally consistent.

`69:74-76` repeats the same universal in the checkpoint without checking its quantifier, and `69`'s own
text calls its own repetition "the fourth": "This checkpoint repeated the universal without checking its
quantifier, which is the same failure `67` section 1 catalogues three prior instances of, and this is
the fourth." Read on its own, `69` is counting 3 catalogued + 1 (its own repetition) = 4, which does not
account for `67:252` itself as a distinct prior instance, since `69` discusses `67:252`'s error in the
same paragraph without folding it into its own count.

So two files independently label two different events "the fourth", and `74:157-167` reconciles them by
noting the operand each was counting: `67:252` is the fourth relative to the three catalogued in `67`
section 1; `69`'s repetition is a fifth event, occurring after `67:252`, which `69` mislabeled because it
did not count `67:252` as a distinct instance separate from the three it catalogues. The arithmetic
`74` states, 3 + 1 + 1 = 5, holds under this reading and I could not construct a reading of the four
source passages that gives a different total without discarding one of them as not a genuine instance.

I could not independently verify `74`'s own stated residual risk, that one of the three instances
catalogued at `67:114-121` might itself be a restatement of another, because that would require opening
`55`, `56`, `57`, `57b` and `61`, `62`, none of which are in this dispatch's premises and all of which
`74` itself declined to open. The risk is honestly stated and honestly left open; I have not closed it
and I am not the person positioned to.

## 4. The live-option census: no sixth found

I built my own list of every "carried forward, not settled, not appended to the register" item across
`65` through `73`, reading each member's own closing section (`65`'s section 12, `66`'s closing
paragraph, `67`'s section 11, `68`'s "not done" paragraph, `70`'s section 8, `71`'s section 12, `72`'s
section 8, `73`'s section 13) before comparing against `74`'s five member-only options (O-A through
O-E, `74:840-899`).

Everything I found sorts into one of three bins. **Already O-A through O-E**: the ambient-family
question (`67:571-584`, O-A), the two shapes of composition (`67:586-598`, O-B), the ownership key as a
structural axis (`70:333-345`, O-C), whose reduction governs a lossy crossing (`71:626-637`, O-D), and
crossing-class naming plus when an order is owed (`71:639-650` and `72:289-299`, O-E). **Already a
numbered register entry or a section-6/section-5 item in `74`**: the role-set question (Q23), Q10's
second read, the observable-set question `70` raises for its own p3 instrument, block floating point's
admission, the value-preserving/operation-destroying crossing's missing name, `59`'s order-and-adder
hypothesis against a redundant encoding, the H1/H2 attack, transfer past the model width, nonzero
fraction width. **Genuinely minor and correctly dropped**: `70`'s question of whether a runtime-owned
realisation component exists on any coordinate besides Q (attested only once, on a single instrument,
and `70` itself does not press it), `71`'s unexplained 1360-collision curiosity, `72`'s "why 226 and not
240" residual, and `73`'s question of whether "membership" and "hosting" are the right words (which
`73` itself frames as a naming question already deferred once in this unit for "crossing").

I found no sixth option at the substance level of O-A through O-E: a genuine, unresolved fork with
distinguishing costs stated on both sides that a future reader would want to build from. The closest
candidate, `70`'s "should the observable set (V, M, O, L) be canonical or open per client", is real but
is a sub-question of N15's already-carried point that effect-class counts are observable-relative
(`74:565-570`), not a separate fork; carrying it as a sixth full option would double-count N15 rather
than add anything N15 does not already say.

## 5. Coverage, bounded

Read end to end, in the order stated in section 0. Opened and checked at the source: every citation
quoted in sections 2 and 3 above, plus a spot check of `OPTIONS.md:985-1000` and `OPTIONS.md:1660-1745`
against `74`'s Q29 through Q32 and Q21-amendment restatements, which match their `OPTIONS.md` originals
closely enough that I found no drift worth reporting there. Ran the anchor-diff command myself (section
2's method), which reproduced `74`'s own stated counts (190/119 for the unit, 293/251 for `74`, 94
absent) closely enough (my extraction found 296/254 for `74` under a slightly broader regex, the
difference being a handful of citations into `INTENTS.md`/`RULES.md` my pattern also caught that `74`'s
own accounting may have scoped to the unit's own files only) that I treat `74`'s count as accurate and
did not chase the discrepancy further, since the substantive check is the set difference, not the raw
totals, and the set difference reproduced exactly at 94.

Not verified: any claim routed through `63`, `00_brief.md`, or `DROPLIST.md`, since I did not open
those either, matching `74`'s own stated boundary. Not re-run: any probe. I traced numbers to the
member file's own prose rather than to the underlying script or transcript in every case; where `74`
cites a number, I checked it against the member sentence that states it, not against the `.txt` or
`.stderr` artifact behind that sentence. That is one layer short of the strongest possible check and I
say so rather than implying I ran the probes.

## 6. What I would want the next check to do

Two things, in order of cheapness. First, land the three fixes in section 2: one line in section 7 for
`70`'s change-test disposition, one citation added to N9 for `66`'s share of the conversion/resolution
claim, and a decision on whether `71:339-344`'s reflex-aside is worth a home. None of these moves a
rung, changes a candidate sentence's substance, or reopens a located disagreement; all three are
additions, not revisions, so `74` does not need to be nuked to take them, only extended, the way
section 7's own entries were extended by `70` and `73` without anyone rewriting `69`.

Second, and it is the larger item: nobody in this unit has produced a second, independently-derived
instance of the sequence-shape claim (N1), the two-depths claim (N6), the exposure-list claim (N11), or
the two-instruments claim (N15). Every one of them is marked ONE EXPERT in `74` and every one of their
own authors names it as the sentence they would most want attacked. That is not a defect in `74`; it is
`74` reporting honestly what the unit actually reached. It is the next unit's job, not this check's, and
I say so because a check that ends without naming the actual next step is a check that stopped at the
document rather than at the topic.

**Nothing here settles anything about the number-system concept.** This file settles only whether `74`
says what its nine sources say, and the answer is: almost entirely, with three named exceptions and two
smaller thinnings, none of which changes a rung, a candidate sentence, or a located disagreement.
