# 101. The periphery re-audited: the ground is ratified, the panel had read its distillation all along, and the ignorance my file 98 reported was mostly mine

Donald Knuth, file 101. I wrote file 01 (the mathematical rigour of the founding identity), file 39
(whether the normalised encodings still represent the numbers), file 62 (the verification backlog),
and file 98 (the completeness assessment). This file exists because file 100 found file 98 wrong on a
load-bearing point, and the dispatch asks me to re-audit the whole periphery against what is actually
there, correct my own section 0, and be exact about the shape of the error rather than only its
content. All three are below, the correction first, the working behind it, and the error's anatomy
last, because the lead designer reads this beside file 98 and the correction is what he needs first.

## What I read

`91_consolidation_nine.md` in full, the standing base. My own `98` in full, as a claim list to be
attacked rather than trusted. `100_quilez_shape_and_geometry.md` in full, the correction this
dispatch extends. Then the thing file 98 never opened: the design round at
`mock/design_rounds/202607300800/`, via one `ls` of its thirty-eight files and a full read of
`202607300700_topic.consolidated-round-state.md` (2284 lines, the round's own self-consolidation,
carrying every decision D1 through D52 with provenance marked per decision; cited below as
`round:NNN`). One `ls` of `mock/design_rounds/` root, whose three loose topic files include
`202607301000_topic.inherited-state-from-the-formalization-round.md`, load-bearing for section 1.
From the panel's own record, re-read because this audit turned on them:
`74_lattner_the_taxonomy_rechecked.md` in full (the file my 98 cited and did not weigh),
`72_giesen_the_unexamined_ground.md:40-75`, `11_current_shape_draft.md:29-60`, `00_context.md:93-96`,
`78_consolidation_eight.md:679` and `:870`, `50_fog_the_float_model.md:476-490`,
`03_jhala_what_is_provable.md:56-94`, `07_spj_is_the_type_story_sound.md:324,417`,
`99_smith_the_elementary_functions.md:1-60`. One `ls` of the panel directory, current through
`100_probes`.

## Gates, run before the work

**Canon gate.** `grep -rln "Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"` and the same
with `FullRange\|UTerm\|AddWidth`, both exit 1, empty, at HEAD `350953f` (the file 100 commit). The
governing material for this dispatch is `91` as the panel's standing base plus the op-ratified round
`202607300800`, which sits on the governing rung and outranks every panel file including mine. Gate
passed. One framing correction to my own dispatch is recorded in section 1 rather than treated as a
refusal: the brief's sentence "the thing neither of you had read" is true of file 98's author and
false of the panel corpus, and the difference changes the diagnosis.

**Test gate.** `cargo test --offline --workspace` from `mock/`, summed per binary: **155 binaries,
672 passed, 0 failed, 9 ignored**, matching files 98, 99, and 100 exactly, same attribution to the
concurrent dispatch's uncommitted `mock/Cargo.toml` and `mock/benches/*` state, which I did not
touch. Test bodies read in the surfaces this file's subjects touch, rather than counted:

- `arvo/tests/predicate_family_const_probe.rs` (the predicate concept's only test surface): real.
  Thirteen const-position pins with runtime assertions, negatives present (`_U16_NONZERO_AS_ZERO`,
  `_I16_ZERO_NOT_POSITIVE`), NaN cases asserted false on every sign predicate (`:122-129`). It
  exercises Family 2, which D15 keeps as the degenerate instance, so it survives the ratified
  deletion of Family 1; Family 1 itself is still shipped and exported (`arvo/src/lib.rs:59`,
  callers at `arvo-comb/src/greedy.rs:37`, `arvo-comb/src/dp.rs:38`), the expected pre-restructure
  state.
- `arvo-hash/tests/` (the pseudorand row's surface), three files. `fnv1a.rs` and the known-vector
  half of `algo.rs` are real (streaming-versus-oneshot, chunked-versus-full, offset-basis and
  external FNV vectors). Two findings the gate obliges me to name. **`arvo-hash/tests/aliases.rs:
  16-23` (`content_hash_roundtrip`) is a tautology**: `from_raw(K)` and `from_raw(K)` with the same
  literal, compared. The same computation on both sides of the `assert_eq!`, structurally incapable
  of failing unless construction is nondeterministic, in the green total, and nobody has flagged it
  in one hundred files. It joins `arvo-tensor/tests/capacity.rs:14-18` (twenty-four files since
  flagging) and its sibling `const_capacity.rs:49-53` (file 100's find, re-verified at source this
  session) as the third of its family, and it should be deleted in the same trivial commit
  `95b:145-149` already assigns the first. **And `arvo-hash/tests/fnv1a.rs:8` carries
  `#![feature(generic_const_exprs)]`**, one of the eight test-file gates whose removal op ratified
  five days ago (`round:787-796`, D3: "The eight test-file gates follow their crates"). The tree is
  the deprecated implementation and the removal is decided, pending work rather than drift, so this
  is a count, not a refusal; but a test surface carrying a feature the canon forbids is worth one
  line in the record every time it is walked past, and this is that line.

Toolchain `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, `aarch64-apple-darwin`, confirmed inside
the tree. No probes; this dispatch's evidence is documents, greps, and the reads above. Verified
versus reasoned is tagged per claim throughout.

---

## 0. The corrected answer, replacing file 98's section 0, paragraph 3

File 98's first, second, and fourth paragraphs stand: the core is covered and mostly attacked, the
open list is small and sorted, and everything remains conditional on op confirming or striking `90b`
and `95b`. The third paragraph is wrong, and it is wrong in the direction that matters most for a
morning read, because it reported ignorance where the record holds ratified decisions.

**Corrected: the periphery is not unexamined ground. It is op-ratified ground, decided in the round
that closed hours before this panel opened (`202607300800`, decisions D1 through D52, each marked
"Decision (op)" inline with rejected alternatives recorded), distilled into a file the panel was
required to read from its first hour (`00_context.md:93-96` lists
`202607301000_topic.inherited-state-from-the-formalization-round.md` as subject input 3), and
re-checked row by row at file 74, which read the distillation at decision granularity and returned
survival verdicts for every crate my file 98 called never-examined (`74:61-73`).** What was
genuinely missing was per-subject **content** review of the kind file 100 performed for shape and
geometry, and the empirical yield of that review, run twice now, is compiled corrections to stated
grounds (a storage count, a missing identity, a one-door claim false above rank 0) rather than
reopened decisions.

**The genuine-ignorance list, restated with the arithmetic of section 3: two items, not five or
six.** The uniform-sampling spec question in `arvo-pseudorand` (uniform over values versus uniform
over data, undecided by D42, its obligation-sentence imposed at `74:72` and nothing since), and the
integer-saturating SIMD lane residue (known-open since file 20, unmoved). The elementary-functions
row, which 98 correctly counted, was closed by file 99 the very next dispatch. Everything else on
98's periphery list is ratified material awaiting either a content review (three subjects, section
2) or op's per-row confirmation of file 74's suggestions, which was already on the open list at
`91:1027-1029` and does not grow.

**The direction of the move: the canon is more complete than file 98 reported, and the reading was
less complete than file 98 reported.** The error bar 98 hung on the periphery ("outcomes ran from
closed-on-inspection to three full consolidation sections") collapses for five of its six subjects,
because a subject with a ratified decision cannot cost a from-scratch derivation; it can only cost a
review, and the two reviews run so far landed at the cheap end. What 98 called the cheapest way to
falsify its section 4, running the sweep, was correct advice that it should have taken itself.

---

## 1. What the record actually is, and the three-layer correction

The dispatch, file 98, and file 100 each carry a different fraction of the truth about this ground,
and stating all three precisely is the audit.

**Layer one, which file 98 got wrong: the ground is ratified, not open.** Verified by reading the
round: thirty-two topic files, fifty-two decisions, each provenance-marked, the agent-delegated ones
flagged as such in their own closing notes (`round:1097-1103` marks D24 "the most suspect thing in
this file by the workspace's own provenance rule"; `round:1464-1466` marks D34's 2x consequence "the
agent's derivation... the more suspect of the two halves"). This is the top rung of the ladder. A
completeness audit that files it under "genuine ignorance" has misfiled op's own decisions as
nobody's, which inverts what the audit was for.

**Layer two, which the dispatch's brief gets wrong: the panel had read this material.** The
inherited-state distillation is one of the three subject files named in `00_context.md:93-96`, and
file 74 read it at decision granularity, citing D1/D2, D4 through D9, D10/D11, D15/D16/D17, D25,
D27/D28, D29/D30, D43/D44/D45 by line (`74:16-19`), stating outright "this file is the first
row-by-row recheck, sixty-three files after the table was written" (`74:40-46`). The brief's "the
thing neither of you had read" is true of me and false of the corpus. This matters because the
correct diagnosis is not "the panel never saw the canon"; it is **"the panel saw it, processed it at
file 74, and then its later files, mine included, stopped reading file 74 and re-derived the
ignorance from file 11's stale table."** The drift was internal re-compression of the panel's own
record, which is a cheaper failure to prevent than a missing input, and section 4 says how.

**Layer three, which file 100 got right and this file confirms at the remaining rows: ratified does
not mean correct, and the useful work at this periphery is checking the decisions' stated grounds
against what the panel has since established.** File 100 found two compiled errors inside the
round's reasoning while overturning no decision. The table in section 2 runs the same check on every
remaining row, and finds the same pattern: the decisions survive; several of their stated grounds,
placements, and vocabularies have been overtaken, mostly by op-checkpointed panel material that
postdates them, which is exactly the "ratified but since overtaken" case the dispatch names as the
interesting one.

One sharpened irony, recorded because it is the cleanest evidence of the re-compression failure:
file 98's own citation for "nothing has reviewed these rows" was `91:769` ("Every other row survives
exactly as the eighth consolidation states it"). The eighth consolidation's row table (`78:679`) **is
file 74's verdict table, carried forward**. The sentence I cited as proof the recheck never happened
is downstream of the recheck. Verified by comparing `78:679` against `74:67`: the arvo-platform row
is the same verdict, same fork, compressed.

---

## 2. The periphery, subject by subject

The dispatch's three-way scale: **ratified-unread** (the panel simply never read it in content),
**open** (genuinely undecided), **overtaken** (ratified, but something the panel settled after it
locked changes it). I add the provenance of each overtake, because an overtake by an op checkpoint
and an overtake by a panel suggestion are different animals, and the ladder says which wins today.

| Subject | Round decisions | Verdict | The specifics |
|---|---|---|---|
| `arvo-shape` | D1-D4, D7, D8, D43, D44 (all op) | **Ratified, now content-reviewed (100), overtaken in two details** | D4's recursion survives the capacity resolution verbatim, compiled (`100` §2.1). Overtaken: D4's sentence making the array composition constitutive of `Shape` is resolved against it by op's own later D43 (`round:1765-1785`), an in-round supersession; and the ground under D2's capacity crate moved, because the panel's capacity resolution (`91:780-812`, two-read complete, awaiting op) replaces `Dim<N>`/`cap`/`cap_size` with `Capacity: Nat` and the `Slot` pairing, and adds the shared bottom-carrier crate beneath what D2 drew. File 100's `AGREES` repair (one-door claim false above rank 0) is one-pass, second read owed. |
| `arvo-geom` | D2 contents, D10, D11, D40, D41 (op; D40's name delegated) | **Ratified, content-reviewed (100), one ground overtaken by arithmetic** | D10's decision stands on its surviving grounds; its storage count is wrong from rank 4 and reverses at rank 7 (`100` §3.2, compiled, exact rational). D41's requirements list now exists (`100` §4.3). The division-hold obligation `74:65` attached is discharged (`100` §3.1). Still open and op's: both benches, the PGA convention, the curve research lead. The algebra-contracts dependency edge for the algorithm crates, open since file 26 (`78:870`), now includes geom. |
| `arvo-platform` | D27 (op) | **Ratified-unread, with one open fork the panel raised and op has not picked** | No panel file has read the crate's contents. File 74 attached the one real question: the tower's derived booleans either pull `Bool`, and so this crate, below the numeral contracts per `arvo-bridge-home-rule.md`, or the contracts go generic over notko's truth contract per D17; "two spellings, both workable, op's pick" (`74:67`, `74:247-253`, carried at `78:679`, compressed into `91:769`'s blanket row-survival). That fork is the crate's whole design question and it is already op-queued inside the per-row confirmations. Content review beyond it is small: five wrappers and a residual. |
| `arvo-float` packaging | D29, D30 (op; D30 a direction), D50 (op) | **Ratified, and overtaken in the strongest sense: the tower absorbed the contents and thereby forced the boundary** | File 74's row, verified: "the contents the decision was about have migrated into the tower... D30's NaN-as-typestate intent is fulfilled by the tower (`Specials` product, `Encoding::Canonical` payload collapse), not by a wrapper carve-out... The 'packaging, not a mathematical claim' boundary argument is now forced rather than chosen" (`74:70`). D50 said the same thing hours later from the round side (`round:2094-2100`), so round and panel converged independently here, and both are op's. What remains for the crate (IEEE instantiations, the hardware door) is already panel-specified at `91` §1.21 and §1.27. Residual content review: near zero. One residue in the other direction, section 2.1 below. |
| Predicate concept | D15, D16, D17 (op) | **Split verdict: D16 overtaken by op-checkpointed panel work; D15 and D17 ratified-unread** | D16's derived/asserted dichotomy was demolished at file 03 ("D16's dichotomy is not a dichotomy", `03:56-94`), rebuilt as a three-rung ladder at file 07 (`07:324,417`), and generalised into the design's own crossing discipline (`68:585` cites it by name; `40:249`), all inside consolidations op checkpointed. That is the interesting case working correctly: a ratified call refined by later ratified work, with the refinement's provenance clean. D15 (Family 1 deleted, the hlist typestate predicate) and D17 (notko home on a `Cardinal`-shaped truth contract) have had no content read anywhere in one hundred and one files; file 74's row calls them "survives, strengthened" on the D16 evidence alone (`74:71`). The D17 truth contract is also one leg of the platform fork above, so the two unread subjects share their one open question. |
| `arvo-pseudorand`, PRNG half | D42 (op) | **Ratified locus; the sampling spec genuinely open** | D42 settles the redistribution (rename, both axes, contracts split deferred until three traits exist, dependency on the algebra crates; `round:1643-1686`). The hash half has since gained its digest chapter in full (`91:766` row). The PRNG half's one spec question, whether sampling is uniform over values or over data, is decided by nothing: D42 does not touch it, the panel imposed only the obligation to say (`74:72`), and no artifact names it since. **This is the one row where file 98's "genuine ignorance" was the right description**, and even it has a ratified frame around it. |
| `arvo-container` | D27, D28, D45 (op) | **Ratified; D45's placement overtaken twice, disposition awaiting op** | The panel substantially rewrote the crate's contract (owns `place`, statement C, the only-door projection, the foreign-bytes obligation; `91:764`). D45's representational-limits contract: reopened by the round's own last file (`round:2163-2166`, may become a derived impl over the format) and dissolved-in-placement by file 74 ("D45's distinction survives; D45's placement dissolves", `74:185-204`, the saturation contract realised by the preset `Resolution` axis). Both overtakes preserve op's distinction and move only its home; the disposition sits inside the per-row confirmation op already owes (`91:1027-1029`). |
| `arvo-bitfield` | D25 (op) | **Ratified-unread in content; obligations attached, packaging argument untouched** | `74:69` and `91:765`: a bitfield is a hand-laid `place` map, the byte-sharing law bounds which shapes have per-field byte images, statement C applies per field. The proc-macro reasoning D25 stands on is untouched by anything since. Cheapest of the remaining reviews. |
| `arvo-num-systems` | D38, D39 (op) | **Ratified and panel-worked; not periphery at all** | 98 correctly excluded it from the unexamined list. `91` §1.6 carries the corrected uniqueness theorem, the embedding-signature caveat, and the seven-upper-members residue, the last two op's own queue items. |
| `notko-hlist` + `Cardinal` | D5, D6, D7, D9, D18b (op) | **Ratified; one sentence owed** | The binding-time sentence (type-level `Nat` versus runtime `Cardinal`), owed since file 74, still owed (`91:767`). |

### 2.1 Three overtakes the table compresses, stated in full because each is a decision op should see

**The round's Growth axis is dead, and D34 survives wearing different clothes.** The round's
formalization proposal named `Growth` (FullPrecision, KeepLsb, KeepMsb, Specify) as a first-class
policy axis (`round:2022-2027`) and its last file renamed D34's Warm headroom "Growth =
FullPrecision" (`round:2177-2178`). The panel then removed Growth (and Widening) from the axis table
entirely, ratified shut at `39b` (`91:239-241`, the `Policy` contract at `91:716` carries the
removal as RATIFIED). D34's content, 2x-logical at every width, survives as the `StoredWidth`
axis's `doubled` instance on `Lowering`, with the container never declared and always projected
(`91:495-520`, forced three ways and independently confirmed). So: D34's principle (op's) stands;
its agent-derived wide-bucket consequence got a ratified home; the round's Growth **vocabulary** is
overtaken by a later op checkpoint and should not reappear in spec text. Provenance is clean at
every step; this is the ladder working.

**The FLX member did not survive, and nobody has said so out loud.** The round's format
decomposition proposed a three-member Underflow (`Unbounded` for FLX, `Gradual`, `Flushed`), with
`Unbounded` argued worth having partly on op's own D49 determinism point (`round:2127-2135`); that
decomposition is flagged in its own file as "the most suspect thing here", agent's. The panel's
ratified vocabulary is a two-instance `Underflow` (`Gradual` | `Abrupt`) on a bounded
`Ranged<EMIN, EMAX, U, S>` exponent (`50:476-490`, standing through `91:410-418` and `91:728`), in
which an exponent unbounded below has no expressible form. On provenance the panel's form wins
twice over (later, and ratified against an agent proposal). But the drop was never argued anywhere
I can find, only made; if op wants the FLX-shaped format for the subnormal-free determinism
argument, that is a one-line residue for the queue, and if he does not, one sentence recording the
drop closes it. Verified: grep for `Unbounded`/`FLX` across the corpus; every panel hit is either
the fold-arity `Unbounded` (a different concept, files 55/58/63) or the format family named in
passing (`28:77`).

**D36 is answered, from the other side.** The round left "the shape of the fix" open (encoding as an
unnamed parameter, `round:1439-1451`); its formalization proposal named the UNORM encoding an
MV-chain. The panel's ratified identity contract carries `Adjustment` as a sealed, value-unique
slope parameter (`91:153-154`), and file 100 §3.3 compiled the specific value `1/(r^F - 1)` that
makes the closed interval exact, concluding a UNORM-shaped type "is not a type" but a parameter
value. Round and panel converged on the same mechanism with different names; the spec text should
say once that D36's open shape is closed by `Adjustment`, and the MV-chain observation survives as
the literature name for what that instantiation is.

---

## 3. The corrected arithmetic

File 98 section 3's counts for the core stand unchanged; nothing here touches them. What moves is
section 4, and by composition, section 0's summary.

**Before (98 §4):** five or six crate-level subjects "no panel file has ever examined", plus one
underived operation family, plus one SIMD residue; total unknowns ≈ 8, each carrying an error bar
from "closed on inspection" to "three consolidation sections", so the periphery's contribution to
remaining work was effectively unbounded above.

**After, verified against the round and files 74, 99, 100:**

- **Genuinely open: 2.** The sampling spec question (ratified frame, undecided content) and the
  integer-saturating SIMD residue (unchanged). Neither has an unbounded error bar; the first is one
  dispatch on a named question, the second is a known-open row on the consolidation's own list.
- **Closed since 98 wrote: 1.** Elementary functions, by file 99, at 98's own estimated size ("one
  decision plus one growth-class derivation"; 99 confirms "the estimate was close").
- **Content-reviewed since 98 wrote: 2.** Shape and geometry, by file 100. Yield: zero decisions
  overturned, two compiled errors in stated grounds, one repair proposed, a requirements artifact
  D41 was owed. That is the measured cost of a periphery row: about one dispatch, landing at the
  cheap end of 98's error bar.
- **Ratified, content review still owed: 3.** Platform (small, its one fork already op-queued),
  predicate concept's D15/D17 content (shares that same fork), bitfield (cheapest). Float packaging
  drops off this list: its contents migrated into the tower and what remains is already specified,
  so its "review" is the one-paragraph residue in section 2.1.
- **Op-decision queue: +1 genuinely new item** (the FLX residue, one line), everything else already
  inside the per-row confirmations 98 counted. The queue does not otherwise grow.

**Net effect on the completeness picture:** 98 reported roughly thirty-two panel-side and op-side
open items plus an unbounded periphery. The corrected statement is the same roughly thirty-two items
plus a **bounded** periphery of five dispatch-sized units (two open questions, three reviews), with
the empirical review yield so far being corrections-to-grounds rather than new design. By the
implementer test 98 adopted, the canon's answer-coverage was already higher than 98 said, because
five of the six "unanswered" subjects had op-ratified answers 98 did not read; what the implementer
inherits at those five is the same risk file 100 demonstrated, that a ratified ground can contain a
compiled falsehood, which is a review cost, not an ignorance cost. Those are different quantities,
and confusing them was file 98's substantive contribution to its own headline being wrong.

---

## 4. The anatomy of file 98's error, and whether the standing requirements could have caught it

The dispatch asks for the error's shape, not only its content, and for whether the two standing
requirements, which 98 performed and reported performing, would have caught it. Honest answers: the
shape is reading compressions as derivations, at three nested depths; and no, the requirements as
constituted could not have caught it, for a reason worth one process suggestion.

**The three depths, each verifiable in 98's own text.** First: 98 §4 rested on `11:44-56` (a table
whose "Reviewed by the panel: No" column was written at file 11 and never updated) and `72:52-60`
(whose "sixty-one files landed after that table and none re-visited it" was true when written at
file 72 and falsified two files later by 74). I inherited both claims' **dates** along with their
text and cited them as current at file 98. Second: 98 cited `91:766` and `91:769-770` as
confirmation, treating three agreeing panel artifacts as corroboration when all three were
compressions in one lineage, and one of them (`91:769`) descends from the very recheck being denied
(section 1). Agreement between unratified artifacts is shared drift; the workspace rule says so in
every file, including in the rules loaded into the session that wrote 98. Third, and the one that
stings: 98's own reading list records "One `ls` of `mock/design_rounds/`" and files its result as
"the formalization round's three topic files are the design record's live tail" (`98:21-23`). That
`ls` returned a file literally named `inherited-state-from-the-formalization-round.md` and a closed
round directory dated the morning of the panel's opening, during a completeness audit whose §4 was
about to classify that round's subjects as unexamined. The `ls` discipline was performed as
ceremony; its return value was never read. File 100 ran the identical `ls`, opened what it
returned, and found the ground in an afternoon.

**Why the two standing requirements could not have caught it.** The definitional-completeness line
audits the terms a file introduces; the separation requirement audits whether the file's models
distinguish anything where checked. Both are inward-facing: they quantify over the file's own
vocabulary and verdicts, not over the sources it failed to read. 98's performance of both was
genuine and both passed genuinely, which is precisely the finding: **a file can satisfy both
requirements while its central claim rests on an unperformed search.** The requirement that would
have caught it exists in the workspace already, in the panels rule: "the first thing a panel does is
try to break its own brief", check the cheap factual claims against source. 98 applied that
discipline to its §1 claims (the Sqrt/Recip gap carries its own grep, performed fresh, `98:126-128`)
and not to its §4 claims, which were carried by citation. The distinction that failed is
**established-by-search versus carried-by-citation**, and the suggestion, offered not ruled: for
audit-shaped deliverables, every universally quantified negative ("no file has", "never", "nothing
touched") carries the grep or `ls` that establishes it, performed for this file, quoted with its
date, exactly as judgements carry a `file:line`. A negative claim inherits the date of whoever last
searched, and citing a search is not performing one. Had 98 run `grep -rn "arvo-platform" *.md`
across its own corpus, seconds, it would have hit `74:67` and this file would not exist.

**What this bears on for every future audit.** The panel's compressions (file 11's table, the
consolidations' row-survival sentences) are correct in their own frame and lethal out of it:
"panel-unreviewed as a crate in its own right" (`91:766`) is a precise sentence whose precision I
flattened into "never examined". The consolidation cannot fix this by adding words; the reader has
to keep the frame. The mechanical guard is the one above.

---

## 5. What this file does not decide

The per-row confirmations, all op's, unchanged in count. The platform/D17 truth-contract fork, op's
pick between two spellings both on record. The FLX residue, op's, one line either way. The sampling
spec question, one dispatch when its turn comes, after the digest per file 72's ordering. The
second reads file 100's findings are owed (the `AGREES` repair, the signed closed-interval niche
compile, the layout projection compile), unchanged. Whether the three remaining content reviews
(platform, predicate, bitfield) are worth dispatches before consolidation ten or are folded into
op's row confirmations: the dispatcher's, and the measured cost from file 100 (about one dispatch
each, cheap-end yield) is the input to that call. Nothing in this file reopens any D-numbered
decision; every overtake reported in section 2 is either op-on-op (later checkpoint wins) or a
panel suggestion already sitting in op's queue.

## 6. The two requirement performances, on this text, before it stands

**The definitional-completeness line, performed.** Terms this file introduces, with dispositions:
*ratified-unread / open / overtaken* (the dispatch's own three-way scale, adopted verbatim, with
"overtaken" refined in section 2 by the provenance of the overtaking material, op-checkpoint versus
panel suggestion, because the scale's third value is not one thing); *carried-by-citation* versus
*established-by-search* (defined, section 4: a claim whose supporting search was performed by the
citing file, versus one inheriting another file's search and its date); *re-compression* (defined,
section 1: a panel file re-deriving a claim from a compression of its own corpus rather than from
the compressed material). Terms used from the record without redefinition: the provenance ladder's
rungs, the blind-spot table, the per-row confirmations, the three width levels, `Adjustment`,
`Specials`, `Underflow`, `StoredWidth`. Nothing else is coined. Checked by grep over this file's
emphasised terms.

**The separation requirement, performed.** This file's one model is the three-way verdict scale
plus its provenance refinement. Where it is nonvacuous: at D16 versus D15/D17, one round-file's
three decisions land in two different verdicts (overtaken versus ratified-unread), so the scale
separates within a single source, which an "examined or not" binary could not; and at the Growth
axis versus D34, where the same design content is simultaneously overtaken (as round vocabulary)
and standing (as ratified principle), which only the provenance refinement can say without
contradiction. Where it is vacuous, I say so: at bitfield and notko-hlist the three-way scale adds
nothing over "ratified, small obligation attached", and my verdicts there rest on the two row
tables (`74`, `91`) alone. **The honest limit, inherited from 97 and 98 and now demonstrated by
98:** these performances verify the file's terms and scale have content; they do not verify its
claims searched everything they quantify over. Per section 4's own suggestion, this file's negative
claims carry their searches: the FLX-drop claim (grep, section 2.1), the sampling-undecided claim
(D42 read in full plus corpus grep), the D15/D17-unread claim (the section-1 greps that found
`74:71` and nothing deeper). Anything my greps' vocabularies would miss, a discussion using none of
my terms, is missed the same way file 72 warned its own sweep could miss, and a second reader with
different search terms is the check on me, as I was on myself and failed once already.

## 7. Standing

The periphery file 98 reported as the canon's remaining genuine ignorance is op-ratified ground:
decided in round `202607300800` before the panel opened, distilled into the panel's required
reading, row-rechecked at file 74, and since then content-reviewed at two of its subjects with a
yield of compiled corrections and no overturned decisions. The genuinely open remainder is two
items, both already framed. The corrected completeness picture is stronger than file 98's on
coverage and identical on its structural caveat: everything persona-ratified stays strikeable on
op's word, and the persona-checkpoint confirmation remains the single largest move available.

Ratified does not mean correct, and the reviews that proved it (100's two compiled errors, the
D16 rework, the D45 dissolution) are the model for the three reviews still owed. And unread does
not mean unratified, which is the lesson my file 98 now demonstrates from the inside: the most
expensive sentence in it was a negative claim carried on the authority of files that had stopped
being true, over ground whose truth was one `ls`-and-read away, in a directory I had listed and
not opened.

Only op's calls are final, and even those go stale. Everything above is evidence and suggestion.

*Grounded on: ratified (the round `202607300800` at the `round:` lines cited per decision; `39b`,
`70b`, `74b`, `77b` via `91` as marked; the persona-tier `90b`/`95b` as marked), settled shapes
(`74:16-19,40-46,61-73,185-204,247-253`, `78:679,870`, `91` sections 1.6, 1.10, 1.16, 1.21, 1.25,
1.26 and its section 4, `50:476-490`, `03:56-94`, `07:324,417`, `99` and `100` as cited in place,
`11:29-60`, `72:40-75`, `00_context:93-96`), measured (the canon greps and the workspace test run,
commands inline, run fresh this session; the corpus greps for `Unbounded`/`FLX`, `arvo-platform`,
`D15|D16|D17`, `truth contract`, quoted where used), verified at source
(`arvo-hash/tests/aliases.rs:16-23`, `arvo-hash/tests/fnv1a.rs:8`,
`arvo-tensor/tests/const_capacity.rs:49-53`, `arvo/tests/predicate_family_const_probe.rs`,
`arvo/src/lib.rs:59`, `arvo-comb/src/greedy.rs:37`, `arvo-comb/src/dp.rs:38`, HEAD `350953f`, the
uncommitted manifest state, attribution only), reasoned (the verdicts of section 2, the arithmetic
of section 3, the anatomy of section 4, mine, offered as an assessment and not a ruling).*
