# The consolidation drop audit: what the standing base lost, and why the format lost it

**Date:** 2026-08-05
**Position in the panel:** after `108b_op_checkpoint_twentysix.md`, which ordered it. **Required reading**
with the eleventh consolidation, before it is written.

`108b` adopted three findings about the archive and named the third as a dispatch: "one dispatch diffs
every consolidation against its predecessor for material that vanished without a droplist entry, because
three instances were found only by accident." This is that dispatch. It diffs all nine adjacent pairs
across the ten consolidations, reading each in full rather than sampling, and checks every candidate
against the rest of the later document and against the intervening deliverables before calling it lost.

The headline is worse than the three instances suggested. **The count is 127 items of material that left
the standing base with no droplist entry.** Eleven of them trace to text the lead designer ratified or
authored. Two of the archive's own conventions, the droplist and the "this document replaces it" sentence,
are each individually broken in a way that guarantees the rest, and they have been broken since the fifth
consolidation without anyone noticing, because the sentence that would have caught it is the sentence that
stopped being true.

The verdict, stated once here and argued in section 6: **this is the format, not any individual
consolidation.** Every consolidation from three onward opens by claiming to replace its predecessor, and
from six onward none of them does. The mechanism is one phrase, "Unchanged this stretch," which is true as
a statement about the stretch and false as a statement about the document, and no reader of a single
consolidation can tell the difference.

---

## 1. The three known instances: two confirmed, one wrong

`108b` names three. I read each independently before reading the others.

**Instance one, the level-or-subject clause, confirmed and larger than stated.** `91:113-126` states the
pricing pillar with a clause and a site list: "it names the width **level** it is a function of, because
two levels coinciding at the one preset everyone measures is exactly how a compile-time fact computed from
the wrong level survives review (`83:290-316`)... Known sites: the bitpacked decode and encode plans (on
the stored width), the value mask (on the fields), the far point and the write granule (unchecked,
flagged), the digest's own field mask (a second consumer of an already-named site, unchecked). A known
non-site: `Encoding::Canonical`'s trailing-zero removal, genuinely data-dependent." `102:90` says
"**The pricing pillar.** Unchanged in statement," and carries none of it. Absorbed nowhere: `value mask`,
`field mask`, `trailing-zero` and `Encoding::Canonical` each return zero hits in `102`. Two of the four
sites were flagged **unchecked**, so what vanished is not only the rule's sharpening but its own list of
places it has not yet been applied.

**Instance two, the demand-driven clause, confirmed with a correction to its history.** `55:163-165`
states it: "It fires at **use**, not at declaration, because an associated const nothing touches is not
evaluated. A `Capacity` impl whose two spellings disagree survives until someone folds with it," with the
fix at `55:166-169` (put the reference on `Capacity::filled` and `Capacity::from_fn`). `108b` says no
consolidation absorbed it in ten attempts. That is not quite right: **the fifth consolidation absorbed it
in full**, at `58:754-761` ("a forced const assertion firing at use (`Capacity::filled`/`from_fn`, every
entry point) rather than at declaration"). The sixth dropped it. `63:533-539` rewrites the section's
"What is not settled" paragraph, keeps `Pos`-has-no-zero, and drops the `Pos` face, the forced assertion,
and the at-use timing. Consolidations seven through nine carry nothing. Then `102:820-830` rediscovers the
identical defect from the other side, forty-five files later, through file 100's compiled rank-3 case, and
proposes a repair (`const AGREES: bool` on `Capacity` itself) that is a superset of the one already
recorded and lost. **The drop cost a re-derivation, and the panel paid for the same finding twice.**

**Instance three, the open question for op, is wrong as stated and the record is better than `108b`
credits.** The item is per-application against per-value-moved event counting, carried in eight's
"Unchanged from the seventh consolidation, untouched this stretch" list at `78:967-968` and absent from
nine's equivalent at `91:1037-1041`. It was **not** dropped silently. `91:959-960` disposes of it
explicitly: "15. Per-application against per-value-moved event counting: resolved this stretch (section
1.14); no longer on this list." The sibling items from that list (`SC_WRAP<n>`, `DatumDeterministic`, the
`Gcd` coherence question, the dither-versus-`Refuse` interaction, the reduction firing site) are all still
reported at `102:1084-1089`, which is the twenty-eight-files-later part of the claim and is correct. So
the shape `108b` describes is real, the sibling asymmetry is real, and this particular item is the one
case where the bookkeeping worked. The real instances of that shape are in section 4 below, and there are
seven of them in the same pair.

---

## 2. Drops of ratified or op-authored material, ranked first

These are the worst available and the reason the audit was ordered. Each is text the lead designer either
wrote or ratified, absent from a later document that claims to stand as the reference, with no droplist
entry.

**R1. Both preset tables, ratified in full at `70b`, exist in exactly one consolidation, and the tenth's
pointer to them is false.** `78:407-455` carries the fixed-point and float tables as markdown, ratified at
op's own checkpoint. `91:502` says "Both ratified preset tables stand exactly as **the eighth
consolidation** carries them," which is true. `102:555` says "Both ratified preset tables stand exactly as
**the ninth consolidation** carries them, unchanged this stretch," which is false: nine carries the
sentence, not the tables. Mechanically, `TowardNegative`, `ToEven`, `HostFloat` and "in-range direction"
each occur four, two, four and three times in `78` and **zero times in both `91` and `102`**. A reader of
the tenth consolidation who follows its pointer lands on another pointer. This is the single clearest
proof that the standing-base claim has become decorative.

**R2. The `tree-meaning` prohibition, adopted in full at `70b`, is gone from the tenth consolidation
entirely.** `78:368-370`: "`tree-meaning`: the shipped source's own prose is offered as the reason a design
construct should mean what it means. **This ground is forbidden.** No claim may carry it." `91:485-486`
compresses it to a name in a list. `102:542` says "The five-row grounding table and the four-member
transfer-ground vocabulary stand unchanged" and names neither the split nor the prohibition; `tree-meaning`
returns zero hits in `102`. The prohibition exists because a shipped doc comment was used to justify three
rows of a design table (`78:349-361`), which op corrected personally. A future member reading only the
tenth consolidation has no way to know the ground is forbidden, and the failure it was written against is
exactly the kind a fresh member repeats.

**R3. The grounding registry is used in every section of the tenth consolidation and defined in none of
it.** `102` carries eleven `*Grounded on:*` footers using `ratified`, `settled shapes`, `compiled`,
`measured` and `reasoned`. The five rows of the table those names come from (`ratified decisions`,
`settled shapes`, `physical grounds`, `tree grounds`, `unreproducible`) and the four transfer-ground
members (`symmetry`, `saturation`, `induction`, `unargued`) are named at `78:343-345` and at **zero**
places in `91` or `102`. Worse, `unargued`'s own rule, "`unargued` as the default is what makes the scheme
honest: a claim naming no ground does not silently inherit one" (`68:459-488`), left at the seventh
consolidation. The apparatus that grades every claim in the document is the one apparatus the document no
longer states.

**R4. D38's ratified content is compressed to a crate name and never restored.** `40:209-212`: "**D38 and
D39 are op's calls** (the `arvo-num-systems` crate: ℕ, ℤ, ℚ, ℝ, ℂ, ℍ, 𝕆, surreal, hyperreal, p-adic,
shipped even if nothing uses them, vocabulary fixed by mathematics...)". `49:206` reduces it to "the
`arvo-num-systems` crate ships; membership through algebraic structure." The enumerated vocabulary, the
"shipped even if nothing uses them" instruction and "vocabulary fixed by mathematics" all vanish.
Aggravating: file 45 defines the `d38` ground slug by pointing at the deleted text (`45:127`, "op, carried
`40:209-212`"), so the fourth consolidation deleted the text its own grounding registry cites as a
definition. The consequence lands seven consolidations later: `78:678` instructs a future member to "scope
the 'finest' fact to the real/Cayley-Dickson chain explicitly" against a ten-member vocabulary the standing
reference has not contained since the third consolidation, and `68:301-309`'s Ostrowski argument for why
the uniqueness justification fails is itself dropped at `78:212-216`.

**R5. Op's own standard for the whole review left the record at the fourth consolidation.** `40:602-605`:
"The standard is optimal and ideal, capable of representing MATLAB, IEEE 754 and SystemC as a test rather
than an inspiration, an abstraction that cannot express one of them being a defect rather than an accepted
boundary." It originates in `13c_op_the_standard_and_the_mode.md`. `49:786-791` is the standing-directives
paragraph and does not carry it. "as a test rather than an inspiration" and "optimal and ideal" both
return zero hits in `49` and in every consolidation after it. The third consolidation's entire stretch was
organised around running that test (`40:6-9`).

**R6. Two of op's four standing directives left the record at the fifth consolidation.** `49:787-789`
carries them: "no member resolves to a single angle on anything substantive. Where the current shape can be
kept it should be, and rewrite cost is the tiebreaker between designs otherwise equal against the intent."
Both are op verbatim at `16d_op_the_spirit_outranks_all.md:11-21`. `58:962-964` restates the standing set
and asserts it is "restated in the same words each time," carrying only "the intent outranks every
instruction, is vague on purpose, and only op's calls are final, and even those go stale."
`102:940-942` inherits the shortened version. "single angle", "tiebreaker" and "current shape can be kept"
return zero hits in `58` through `102`. The rewrite-cost tiebreaker is the clause that decides between two
designs equal on the intent, which is the position the eleventh consolidation is about to be in.

**R7. Op's early checkpoint files stopped being cited before the second consolidation and were never
restored.** `04b`, `06b`, `08b`, `12b`, `13b`, `16b`, `16c`, `16d`, `17b` and `24b` return **zero**
citations across all nine consolidations; `13c` gets one, in the second. `30b` and `34b` fall to zero at
the ninth. `68b`, op's own return checkpoint and the source of the panel's scope reset, falls to zero at
the tenth. The content of some of them survives uncited (the spirit clause, the novelty posture); the
content of others does not, which is R5, R6, R8 and R9.

**R8. Thread A, kept open by op at `04b` with an explicit instruction to keep iterating, left at the second
consolidation.** `11:449-453`: "Three threads were explicitly kept open by op through the panel's mid-run
checkpoints, each with a specific instruction to keep iterating rather than to stop at the first working
answer. None of the three is settled." `26` mentions Thread C once and Threads A and B nowhere;
`modifier`, `OverRangeOf`, `nominal` and `identifier` all return zero hits in `26` and `40`. `04b` records
op's words: "Option 1 but not just price, iterate on; there might be ergonomics to be won when taking
further and specializing, instead of stopping at this solution." No droplist entry closes it.

**R9. Thread B's delivery reframe, one of the two results `06b` explicitly left standing as a proposal,
left at the same point.** `11:522-530` states it: whether a refusal arrives as a checked sum type, an
absorbing bottom value, or a sticky flag "is by this design's own axis-sorting test a `Lowering`-level
choice", together with the finding at `11:544-546` that "the `ConstantTime` derived marker is currently
keyed on data that does not decide it: delivery decides it, and delivery is not one of the ten axes."
`delivery` occurs fourteen times in `11` and zero times in `26` and every consolidation after;
`ConstantTime` four times and zero.

**R10. Preset divergence, noted at op's seventh checkpoint as available and explicitly not adopted, with an
instruction attached, left at the fourth consolidation.** `40:693-696`: "**Preset divergence**... has a
working, probe-verified, unstable-feature-free mechanism... noted at op's seventh checkpoint as available
and explicitly not adopted: op's call is that this deserves more than the first mechanism that works, and a
later member should take it further." "preset divergence" and "parent preset" return zero hits in `49` and
in every consolidation after, and zero in files 41 through 57. A working mechanism with an op instruction
attached to take it further simply left the record.

**R11. The membership licence's constraint clause, part of op's held D38/D39, left at the sixth
consolidation.** `58:266-267`: "D38/D39 (op) hold; **membership licenses only the exact, widening operation
family, gated on `Specials = None`**." `63:227`: "Unchanged and untouched this stretch. D38/D39 (op) hold."
`Specials = None` returns zero hits in `63` and every consolidation after. What licenses what is the
content of the held decision; the shortened sentence records that a decision is held and not what it says.

---

## 3. What a reader of the tenth consolidation actually has

Before the per-pair findings, the aggregate, because it is the finding that matters most.

The tenth consolidation has twenty-eight numbered subsections under "The agreed shape." **Fourteen of them
contain no content.** Measured as non-blank words between the heading and the next:

| section | last full statement | words there | words in `102` | what `102` says |
|---|---|---|---|---|
| 1.1 What a number is | `40:39-56` | 205 | 8 | "Unchanged from file 40 and every consolidation since." |
| 1.2 The identity contract | `58:122-161` | 438 | 1 | "Unchanged." |
| 1.3 Encoding, nested inside Lowering | `40:106-144` | 344 | 1 | "Unchanged." |
| 1.6 Membership | `40:207-242` | 454 | 3 | "Unchanged this stretch." |
| 1.7 The algebra | `40:243-327` | 1117 | 3 | "Unchanged this stretch." |
| 1.8 The fold | `40:328-357` | 363 | 3 | "Unchanged this stretch." |
| 1.9 The multiplicative half | `40:358-385` | 367 | 33 | "Unchanged this stretch." |
| 1.15 The exponent and the spine rule | `49:552-570` | 231 | 1 | "Unchanged." |
| 1.17 Radix ten | `58:561-623` | 853 | 3 | "Unchanged this stretch." |
| 1.18 The numeral notation | `63:362-436` | 941 | 3 | "Unchanged this stretch." |
| 1.20 The algorithm crates | `63:476-540` | 816 | 7 | "Unchanged in content from the sixth consolidation." |
| 1.21 The strategy door | `78:391-484` | 1058 | 15 | "stand exactly as the ninth consolidation carries them" (false, R1) |
| 1.23 The assembled trait table | `91:696-750` | 411 | 36 | "Unchanged this stretch from the ninth consolidation's own text." |
| 1.24 The cost model | `58:890-934` | 513 | 3 | "Unchanged this stretch." |

Roughly **eight thousand words** of the design's own statement of itself sit outside the document that
claims to be its reference, and five of the fourteen stubs give no pointer at all.

The chain is worse than one hop. Section 1.7 in the tenth says "Unchanged this stretch"; so does the ninth,
the eighth and the seventh. The sixth carries a 75-word summary, the fifth a 146-word summary citing
`49:216-254`, the fourth a 496-word compression, and only the third states the thing (`40:243-327`, 1117
words). **Five consecutive "unchanged" stubs bottom out on a summary, not on a statement**, and only one
link in the chain carries a line range. The same shape is documented independently at `68:368-375`, which
says the float model "stand[s] as `63:296-321` states them", where `63:296-321` says it "stand[s] as
`58:497-560` states them".

The live-defect registry is a two-hop pointer by the tenth: `102:1019-1020` says "Entries 1 through 7 from
the ninth consolidation's own registry stand unchanged... See the ninth consolidation for their full text",
and `91:970` says "**1** through **5**, **7**: unchanged, see the eighth consolidation for full text."

---

## 4. The findings, by pair

Every item below carries a line range in the earlier document, a line in the later one where it should
have been carried or droplisted, and has been checked for absorption elsewhere in the later document and
for resolution by an intervening deliverable. Items already stated in section 2 are not repeated.

### `11` to `26` (15 items)

The scope section entire, including the taxonomy table's twelve rows and its reading instruction
(`11:27-60`, "Nothing below should be read as a statement about any of the untouched rows... that is a
noted gap, not a finding"), against `26:12`'s "It stands alone". Ten of the twelve crate rows return zero
hits in `26` and `40`, including the `arvo-container`-versus-`Lowering` gap that `26`'s own §1.6
delegability rule reasons about. The two live forbidden-feature gates the restructuring is obliged to
remove (`11:76-80`), against `26`'s silence: `generic_const_exprs` returns zero hits in `26`. Arvo's
standing never-police principle (`11:82-85`), against `26:527-596`'s enumeration of eight governing
principles that asserts "none is optional context". The axis table's instances and sub-parameters
(`11:156-181`, `Narrowed<W: IntermediateWidth, A: Anchor>`, `Unbounded`/`Gradual`/`Flushed`) against
`26:39-42`'s "Table unchanged from file 11", which lists names only; `Anchor` and `Flushed` return zero
hits. The affine value map and the UNORM8 worked example proving `Adjustment` and `Bias` are independent
(`11:176-181`) against `26:643-646`, which names the formula without stating it, in a document whose
successor spends a whole ratification hold on what `Bias` is. The preset-redefinition audit obligation
(`11:344-353`, "flipping them, test by test, in the same change that flips the implementation, is the
audit obligation this redefinition carries") against `26:655-659`, which keeps the question and drops the
obligation. The conventions mechanism entire, `conv-ieee754` through `conv-flocq`, plus the adequacy test
and the two unrepaired gaps (`11:356-366`, `11:718-733`) against `26:533-541`, which restates op's standard
that the design must represent those three standards as a test while `conv-` returns zero hits. The
in-range/out-of-range boundary fix (`11:712-717`) against `26:44-48`, which restates the quantiser in the
unfixed classify-first form; it was independently re-found nine files later and ratified at `40:165-180`.
`FullRange<0>`'s division by zero and the dropped `F >= 2` bound (`11:674-678`) against `26:643-647`. The
affine formula's failure to cover `Stored` numerals (`11:680-684`) against `26:643-652`, which carries two
of the three items from the same section of `11` and flags each as unresolved. The phantom-type closure's
unpaid cost (`11:750-758`) against `26:517-521`, which calls it "the real closure mechanism" and drops the
qualification that connecting the proof type to the byte-holding type "is a real design exercise nobody has
completed". The four-bin ledger's contents (`11:813-868`) against `26:99-101`, which names the four bins
and updates them while `TypeId`, `specialisation`, `Deterministic` and `ConstantTime` all return zero hits;
the `TypeId`-and-specialisation dependency is the item that later became a workspace rule, and `40:31-32`
restores it, so `26` is the only broken link. Plus R8 and R9 above.

One of the fifteen is a false attribution rather than a drop, and it is worth its own line. `26:523-525`
says "File 11's other open packaging questions (where `Width`/`Exponent` and the container projection
live, whether `Bits<N, S, Sign>`'s `S` should re-bound to `Lowering` alone) are untouched by this dive and
remain exactly as open as file 11 left them." Neither question exists in file 11. File 11's actual §5.3
list is at `11:793-811` and is four different questions, three of which vanish. **The sentence that
asserts carry-over is the sentence that performs the drop.**

### `26` to `40` (9 items)

The fidelity thread entire, under a heading asserting the section is unchanged. `26:352-360`, `26:629-641`
and `26:684-687` carry it as a proposed axis with three named undecided sub-questions and a residue;
`26:438-441` names the residue: "`Contract` (the actual permission, 'either answer is acceptable') is the
one real residue that genuinely cannot be expressed from portable `no_std` source today, and needs either
the receipt-and-pass machinery or the unvetted feature path." `40:519-532` says "Both unchanged from file
26 and untouched by every deliverable in this stretch" and enumerates six carried items, none of them
fidelity. `fidelity` drops from seventeen hits in `26` to two in `40`, both inside droplist lines, and zero
from the fifth consolidation on. `Contract` and `Fused` return zero hits in every file of the panel after
`26`. The saturating-reduction residue's priced obligation (`26:442-450`, "a real, currently unpriced cost
that lands on arvo rather than on any build layer", with `uqadd` and `paddus*` named) reduced at
`40:662-665` to one line in a list of owed codegen regression tests; a test pinning the fact is not the
obligation to write the kernels. The multi-limb fragility item (`26:452-457`, "a dependency on an optimiser
heuristic holding, not a guarantee, and it costs one codegen test to make falsifiable") against `40`, where
`carrying_add` and `WideBits` return zero hits. `notko-hlist`, `26:661-666`'s own "single cheapest, most
repeatedly-flagged open item in the whole document", against `40:9-10`'s claim that this stretch "reads the
two pieces of prior art eight separate members had flagged and none had opened"; `hlist` returns zero hits
in `40` and the stretch reported on one of the two. The model-inadequacy standing risk (`26:104-109`,
"Nothing catches the case where a model is too narrow to see a *value* disagreement... No mechanical fix
for the second case exists yet") against `40`, which rests more claims on bounded exhaustion at a model
width than `26` did and carries neither the risk nor an entry. The classification-versus-exhaustive-check
overlap (`26:92-97`) against `40`, which carries neither the mechanism nor the question. The rounding-bias
findings including an owed checked const bound (`26:288-305`, "this validity range must ship as a checked
const bound alongside the credit given to it") against `40:616-696`, which lists many owed items and not
this one, while absorbing the error-feedback half of the same paragraph at `40:193-201`, which makes it a
selective drop rather than a section supersession. Whether a law attaches to a type or to nothing
(`26:625-627`) silently resolved by adoption at `40:243-247`, with the discarded reading and the explicit
warning against reading mechanism-agreement as noun-agreement both gone. The accumulator's three readings
(`26:619-622`) silently resolved the same way, in a stretch that closed both of the interactions `26` named
as untraced.

### `40` to `49` (14 items)

R4, R5 and R10 above, plus eleven. The `Specials`-as-identity cost measurement (`40:81-85`, five branchless
instructions against six, disassembled) against `49:120-128`, while `45:292` carries it as a live registry
row. The finest-view mechanism's price against the alternative it beat (`40:314-318`, 0.130 ms against
0.193, 907 bytes against 1854) against `49:216-254`; this is the only measurement supporting one of op's
three ratifications at `39b`, and `45:353` marks it current. The shaping-is-a-scan counter-reading
(`40:202-205`) against `49:183-186`, which states the conclusion and deletes the condition under which it
fails, while `45:321` records the claim's ground as resting on nothing ratified and says explicitly "the
counter-reading is carried in `40:200-205`". Interior safety's formal definition and its relation to total
safety (`40:330-332`, `40:340-344`, including "interior safety can hold while total safety does not" and
"A combinator states which condition it checked") against `49:256-258`, which keeps the headline; the loss
then propagates through `58:287-288`'s "unchanged from `49:255-265`". The direction-in-key condition
(`40:290-294`, the derived reason multiplication needs `mul_full` and addition does not) against
`49:239-244`, which reproduces the heading sentence verbatim and replaces the body with meta-commentary
about how the probe was re-run. The strongest erasure measurement in the review (`40:526-531`,
byte-identical to a bare `wrapping_add` under the shipping build shape) against `49:705-713`, where the
test-debt line item survives and the measured result it was a test for does not. The measurement justifying
the nested-`Encoding` shape (`40:123-126`, the 1.8x rendered-diagnostic cost of a three-parameter split)
against `49:134-149`, which shows the nested declaration with no reason. The finest-system derivation
table's staleness finding and its repair instruction (`40:238-241`) against `49:211-214`. The open
packaging item deleted from a sentence whose closed half was kept (`40:532-536` against `49:712-713`;
`algebra-contracts` returns zero hits in `49`). The constructive-deliverable directive (`40:610-612`)
against `49:786-791`.

### `49` to `58` (7 items)

R6 above, plus six. `SC_SAT_SYM` and the payoff of the `Sign` split (`49:122-124`, the identical
`TowardNegative` clamp delivering `-8` under `AsymmetricLow` and `-7` under `Symmetric`) against
`58:122-161`, which rewrites §1.2 and states neither; `SC_SAT` returns zero hits in `58` and in files 50
through 57. The never-typed-from-the-outside item (`49:881-885`) deleted whole at `58:1068-1132`, when file
54 had discharged one third of it and left the explicit instruction at `54:660-665` that "the item should
shrink to name them rather than stay at its original width". The pin-hash discrepancy flagged out-of-band
for op and the workspace (`49:779-783`, `cced03bfd` against `57d06900f` in `workspace.md`) against
`58:55-66`, which records the hash and not the discrepancy, and `58:1008-1019`, which does not carry it;
`cced03bfd` returns zero hits in `58` and in files 50 through 57. "No division fold is owed" (`49:460-462`)
against `58:374-386`; a statement about what the combinator surface does not need reads, in its absence,
as an unanswered question. The second half of the membership claim and `ExactWindow` (`49:206-209`) against
`58:264-269`'s "Unchanged and untouched", where `ExactWindow` returns zero hits and file 45 grounds the
claim as a single two-part claim carried by two independent readings. The argument the nested shape stands
on (`49:125-128`, the `Underflow`-has-no-bottom argument, with block floating point withdrawn as evidence)
against `58:122-161` and `58:507-523`; the BFP half is droplisted, the positive argument is not, and it is
the only support the ratified nested shape has.

### `58` to `63` (24 items)

The largest single-pair loss in the archive, and the pair where the format changes character: the sixth
consolidation is the first to reduce whole sections to "Unchanged this stretch" without a line range.

R11 above, plus twenty-three. The `Monotone<Add>` two-door design for the algorithm crates
(`58:771-777`) against `63:476-539`, which rewrites the section and carries no occurrence of `Monotone` in
it; this takes with it the compiled reason `Hot`'s ordering inverted and `Precise`'s did not. The planned
deletion of `longest_path`'s workaround and `matrix_chain_dp`'s parallel `Bool` reachability matrix once
`Specials` lands (`58:777-781`); `annihilator`, `semiring`, `longest_path` and `matrix_chain` all return
zero hits in `63`. The compiled prohibition on giving `fold_compensated` the reassociation licence
(`58:457-464`, whose violation collapses the compensation term to `fsub s0, s1, s1`) against
`63:284-289`. The Apple-silicon refutation of the flush-to-zero argument (`58:838-841`) against
`63:607-614`. `IS_EXACT` and `Total<Op>` together (`58:276-277`) against `63:235-237`; this clause was a
correction file 38 made to file 37, flagged at `49:235-237` as moved "from prospective to load-bearing" by
file 43's `div_floor`/`rem`, and `IS_EXACT` returns zero hits in `63` and every consolidation after. The
cadence correction as an item op still has to confirm (`58:1016-1017`) against `63:752-778`, which lists
it as settled instead. Division's compiled float cause split and its `Theta(2^p)` growth class
(`58:376-386`) against `63:280-282`'s "held exactly as `44b` left it". The unavailability of the standard's
own carrier (`58:405-409`, zero `fetestexcept` in `rust-src`, no FPCR in `core::arch::aarch64`,
`_mm_setcsr` deprecated) against `63:284-289`. The fixpoint grade's trip-count independence and what
`Unbounded` is (`58:420-433`) against `63:534-536`, which names `Unbounded` without defining it, and the
same line names `foldnum` without defining it (`58:747-752`). The crossing contract's derivation
blockquote (`58:181-186`) against `63:192`'s "stands unchanged", in the document `68` then builds §1.4 on.
The overflow band's two clauses, the zero-under-prediction measurement over 5,184 triples, the six-row case
table and "Division has no row" (`58:235-260`) against `63:205`. The algebraic difference from `Implicit`,
the single fact from which three results follow (`58:226-233`) against `63:204-205`, which cites a range
stopping one line short of it. The exact accumulator width formula and the real-format figures
(`58:291-307`, 277 bits for binary32, 2,098 for binary64) against `63:244-246`, which names the formula
without stating it. What `R = 1` and `R = 0` admitted (`58:150-158`) against `63:173-174`, on an item that
is number three on both documents' loudest-for-op lists. The positive enumeration of eleven operations and
the universal conclusion it licenses (`58:322-335`) against `63:255-259`. The flush-to-zero measurement
that is the evidence for the entire strategy-door design (`58:800-806`) against `63:543-549`, which asserts
the property as derived. The numerator-dominates finding and its two-digit-numerator row (`58:914-933`)
against `63:669-679`, which prints four of six cost rows. Six named standing contract mechanisms
(`58:939-941`) against `63:681-689`. Live-defect entry 2's mechanism and its compiling fix
(`58:1034-1044`) against `63:793-795`'s "Unchanged, vehicle still held", on item 6 of both loudest-for-op
lists. The grounding convention's perimeter and its two unbuilt tiers (`58:727-729`) against `63:437-474`.
The bound-rather-than-equality lever, "the strongest diagnostic message this whole review has found"
(`58:658-673`, restated at `58:785-787`) against `63:427-435`. Non-canonical codes as a third and larger
source of non-injectivity, with the 209-of-768 measurement (`58:586-590`) against `63:325-328`. Five open
items that stopped being reported: `algebraic_mul` decoupled from `contract` (`58:1093-1094`), the
face-level sibling pricing question (`58:1107-1108`), the `on_unimplemented` `{Self}` sweep
(`58:1108-1110`), the `#[deprecated]`-shaped lint on a direct `Reduce` bound (`58:1110-1111`), and the
decimal face being untested (`58:1090`), none of them in `63:863-868` and none declared answered. Plus the
codegen-flag audit narrowed from five named files' unswept claims to one unrun reproduction
(`58:1117-1120` against `63:870-872`), and the second half of the widened-result-numeral question, the
operand-numeral door's `Monotone` gate (`58:1103-1104` against `63:537-539`), erased from the design and
from the open list in the same stretch.

### `63` to `68` (10 items)

The seal's honest limit (`63:276-278`, "the enumeration is verified as 'every attack found lands in one of
the routes'... not 'the routes are the whole space'") against `68:342-354`, which states the five routes
and drops the qualifier; `58:370-372` had explicitly instructed that the block carry it "rather than the
stronger reading", so its removal converts a sampled result into an unqualified one. The `HostImplemented`
locus item (`63:621-623`) against `68:791-793`, which closes three narrow items by accounting for two.
`62b`'s three-part rewording of the `unstable-features.md` drift entry (`63:768-772`) against `68:798-805`,
which renumbers item 8 to a different three-clause package and leaves the original as narration. The
five-row grounding table with its `rung` and `examples` columns (`63:443-449`) against `68:431-433`, in a
document whose own §3 tags entries "Grounded `tree`, `pin`, `flags`" and whose §1.19 heading reads
"replacing what `ffl` was credited with" while defining `ffl` nowhere. The cost model's six figures and the
cliff (`63:669-679`) against `68:595-597`'s "Unchanged this stretch", in a document that still cites
`53b`'s adoption of them as spec text. The `Pos` structural and readout ceilings and the adopted two-tiered
refusal (`63:392-409`) against `68:383-427`. The 923-assertion whole-matrix test and the `37/53` bug it
caught (`63:411-417`) against the same rewrite; the only verification evidence behind the adopted notation
vehicle. The 4.5x staging measurement (`63:419-425`) against `68:412-417`, which derives a methodological
control and closes by citing a measurement whose figures the document no longer contains. The
radix-general quantiser repair's three places and the radix-ten validation leg (`63:207-219`) against
`68:290-292`. Live-defect entries 3 and 4's mechanisms and remedy (`63:797-801`) against `68:852-854`. Plus
one open residual replaced by a droplist entry: `68:388-389` enumerates "The three named at `63:864-868`"
and substitutes, for the third (`63:866-868`, a host arithmetic wider than `u128` to exercise the
structural ceiling), the pricing hazard that is `63:902-905`, a droplist entry. The real residual is
neither carried nor closed.

### `68` to `78` (28 items)

The pair with the largest number of stub-induced compressions, because the eighth consolidation is where
the identity half of the design becomes unrecoverable.

Eleven silent drops. The sealed `Arity` decision, one of the five second reads `67b` closed
(`68:104-114`) against `78:131-135`; `Arity`, `Unbounded` and `Fin<P>` all return zero hits in `78`. The
crossing contract's three explicitly-refused alternatives (`68:272-274`, "All three move a
declaration-time fact into a use-time check, which the design's own binding-time discipline exists to
avoid") against `78:198-204`; a recorded exclusion exists to stop re-proposal. File 66's
`Abrupt`-on-a-decimal-numeral note (`68:282-286`) against `78:266-330`. The spec sentence the notation
chapter told the spec to retire (`68:424-425`) against `78:336-339`, which says the residuals stay closed
and does not carry an instruction that was never a residual. The pricing-hazard control on
declaration-cost measurement (`68:415-418`, "two arms are comparable only when they force the same ones")
against `78`, in the same document whose central new measurement is a declaration price. The
`#[diagnostic::on_unimplemented]` condition attached to the structural derivation (`68:637-640`, "if it
cannot, the shape question returns rather than shipping a worse diagnostic") against `78:701-708`, which
declares the measurement standing while dropping the unmet condition. The facade migration's atomicity
requirement (`68:696-701`) against `78:710-738`, which closes the fork and says nothing about how it
lands. The two dispatch conventions adopted at `67b` (`68:678-683`, the whole-crate compile owed by a
universal "only" claim, and the sketches-directory listing) against `78:12-16`; `sketches` returns zero
hits in `78` and in files 69 through 77. `57b`'s cadence correction (`68:749-750`) against `78:829-838`.
`53b`'s reassociation-licence design shape (`68:736-737`) against `78:866-867`, which keeps the feature
verdict and drops the design it licensed.

And one that is not a drop but a reversion, which is worse. `68:806-807` carries the correction: "The
bench table softened from '13x to 17x at every point' to 'ten to seventeen across two runs' (`62b`); the
repair is already committed." The correction itself is at `63:611-613`, measured. `78:459-460`, inside
§1.21's newly ratified text, says "degrading **thirteen to seventeen** times slower with no diagnostic".
**The eighth consolidation does not merely drop a correction; it restates the corrected figure in text
ratified at `70b`.**

Fifteen compressions that lost content, of which four are load-bearing enough to name here. §1.1's
definition of what a number is (`68:147-151`, "an integer k, drawn from a finite interval, together with a
type-level rule injecting k into a set of rationals") against `78:186`; "integer k" returns zero hits in
`78`, so the document's central object is undefined in it. §1.2's identity contract, including both
`ExponentForm` constructors and `SignDomain`'s three members (`68:155-165`) against `78:188-191`;
`Implicit`, `Ranged`, `NonNegative` and `AsymmetricLow` all return zero hits in `78`, **and never return in
`91` or `102`**, while `78`'s own §1.16 far-point rule and §1.21 float table both quantify over
`Ranged`-shaped numerals the document never defines. §1.4's three round-trip statements themselves
(`68:186-190`) against `78:204`'s "the crossing contract's own three statements plus their precondition are
untouched", which asserts they are untouched without stating them; `encode ∘ decode` returns zero hits in
`78`. §1.19's transfer-ground vocabulary definitions, the default rule, the `Ranged` assignment, the
509,660,160-instance check and the measured saturation thresholds (`68:459-488`) against `78:344-345`'s
four bare names, which is R3's origin.

The other eleven, in brief, each against a stub or a shortened sentence in `78`: §1.3's "double duty"
sharpening (`68:176-181`), whose deletion at `78:193-196` removes the licence for statement P that `78`
then introduces at `78:569-578`; §1.4's compiled ill-typedness diagnosis (`68:201-211`); §1.4's leak's
general rule and its eight-cell table (`68:221-240`); §1.5's round-first validation against binary32 on
41,380,159 operations and the even-radix tie fact (`68:290-294`), which is one of the two compiled supports
under the transfer-argument refutation `78:341-345` claims stands unchanged; §1.6's Ostrowski-based
refutation of the uniqueness justification (`68:301-309`); §1.9's `mul_full` signature and its
254,830,080-instance check (`68:325-330`); §1.12's six named sealed carriers (`68:344-354`) against a
count with no names; §1.16's flush-to-zero placement clause (`68:370-372`) against the identical sentence
with one clause deleted at `78:268`; §1.18's `Adjustment` entry-point closure and the compiled
silently-wrong-value defect that forced it (`68:390-400`); §1.19's container-class transfer coordinate and
the twelve-container fact (`68:490-504`), leaving `78:946`'s owed companion model with nothing stating what
it is for; and §1.25's per-width-table pricing, quadratic to 116 seconds at 4096 and past 25 minutes at
8192 (`68:633-637`), which is the whole reason the structural form is the design, dropped at `78:701-708`
in the document that names the pricing pillar.

Two open questions stopped being reported: the exact `foldnum` closed form as a type-level computation
(`68:903-905`) dropped from the verification bundle `78:942-949` explicitly claims to carry forward, while
its sibling from the same bundle survives at `78:944`; and `foldnum`'s characterisation as spec text
(`68:773-775`), leaving `78:944`'s owed compile with no stated expected result anywhere in `78`.

### `78` to `91` (8 items)

The L0 spine-rule migration section entire (`68:606-711`, `78:699-784`), which drops as a section at `91`
and leaves the record as three sentences in the op-calls ledger (`91:823-824`) and the registry. By `102`,
`L0`, `migration`, `route Z`, `facade` and `capacity unification` all return zero hits. The fork was
ratified closed to route Z at `77b`.

Six open items from `78:934-963` absent from `91:978-1041`, none droplisted at `91:1043-1085`, none
resolved by any file between: the precision axis's `unargued` status (`78:955-956`; `unargued` returns zero
hits in `91`, `102`, and every file after `78`); the container-class coordinate's companion model
(`78:956-957`); the `InfOnly` `Specials` witness (`78:959`); the `10^20` figure (`78:959-960`, which
returns **zero hits in any panel file after `78`**); the reciprocal-table strength reduction
(`78:960-961`); and the codegen-flag audit (`78:963`, `codegen-flag` returning zero hits after `78`). The
first two were themselves marked "unchanged since the seventh consolidation" and the decimal three
"unchanged", so each had survived several consolidations before vanishing at this one.

And a bookkeeping drop with a stated purpose behind it. `91:980-982` says "Items closed this stretch are
named once and not repeated" so that "the next member does not re-open them". Four items that files 80 and
85 genuinely closed (the IEEE §4.3.1 overflow tie, the OCP mode split, `Crosses`'s second read, and
statement 0 against `quantize` and `roundToIntegralExact`, tabulated as closed at `85:52-58`) leave the
open list without appearing in the closed list. They are gone rather than recorded as done, which is the
same end state the list exists to prevent.

### `91` to `102` (12 items)

R1, R2 and R3 above and instance one from section 1, plus eight.

The value-keyed against datum-keyed digest fork and the design obligation attached to it (`91:653-658`,
"The choice between the two digest kinds is a real cost fork, not a style preference, and the design should
expose it as a named choice per `arvo-toolbox-not-policer.md`, never pick one silently") against
`102:559-560`, which says the digest contract stands "exactly as the ninth consolidation states them";
`value-keyed` occurs eight times in `91` and zero in `102`, and `arvo-toolbox-not-policer.md` three times
and zero, in a document that at `102:587` invokes "the workspace toolbox rule's own authority" without
naming the rule. The byte-image guarantee's scope sentence (`91:684-687`, "a same-process,
same-build-target fact, not a wire format", itself marked unchanged from the eighth consolidation);
`wire format` returns zero hits in `102`. The commitment to ship both decodes and pick on the operation's
lane width (`91:674-676`). The write granule clause (`91:676-678`, "adjacent values share bytes, so no
element is independently writable, and a consumer partitioning the column for parallel writes must place
every boundary on a multiple of `P`"), which `102:837` then builds on ("The write granule becomes a shape
fact: an outer-axis partition is legal only when `inner mod P == 0`") without the document containing the
statement it generalises. The memoized-digest freshness scope exclusion (`91:659-661`). The bench harness's
overwrite defect (`91:1025-1027`), an owed item whose artifact was "a per-section filter, or run artifacts
landing beside rather than over committed ones, plus a by-reference input path"; `102:1033` retracts the
by-reference half as never having existed and the other half leaves the list, with `overwrite`, `per-section
filter` and `landing beside` all returning zero hits in `102`. The §5.12-against-§5.2 distinctness note
(`91:1019-1021`, "confirmed genuinely distinct from the §5.2 citation the review already has") flattened to
"IEEE clause 5" at `102:1046`. And the corrected, `82b`-ratified bitpacking figures (`91:663-674`, 1.50x
on a sum and 1.29x under per-element work, replacing the eighth consolidation's 4.6x) against
`102:566-588`, which reports a new sweep with a 1.66x L1-resident peak and a 1.43x crossover and never
reconciles the two; a reader of `102` alone sees neither the ratified figures nor that the 4.6x they
replaced was wrong.

---

## 5. The droplist's own accounting

Every consolidation's droplist was checked entry by entry against whether the entry corresponds to material
that genuinely left the predecessor's text.

| consolidation | entries authored | correspond to a real removal from the predecessor | items this audit found with no entry |
|---|---|---|---|
| `26` two | 12 | 1 | 15 |
| `40` three | 18 new (+12 carried) | 2 | 9 |
| `49` four | 10 new (+30 carried) | 3 | 14 |
| `58` five | 9 new (rest by reference) | 3 | 7 |
| `63` six | 6 new (rest by reference) | 3 | 24 |
| `68` seven | 9 new (rest by reference) | 2 (+1 partial) | 10 |
| `78` eight | 7 new (rest by reference) | 1 | 28 |
| `91` nine | 8 new (rest by reference) | 3 | 8 |
| `102` ten | 8 new (rest by reference) | 3 | 12 |
| **total** | **87** | **21 (24%)** | **127** |

Three readings follow, and each is independently damaging.

**The droplist is a record of the stretch's own reversals, not of what left the standing base.** Sixty-six
of the eighty-seven entries retire a proposal born inside the deliverables that stretch absorbed. Those are
worth keeping and are exactly what the preamble describes. But the artifact does not do the job its
position implies: nothing in it reports on the difference between this document and the last one, which is
the only job that would have caught any of the 127.

**The droplist stopped being a standing record at the fifth consolidation, in the same document where the
sections started stubbing.** Consolidations two, three and four restate every prior entry. From the fifth
on, each carries a sentence of the form "Carried forward from files 26, 40, 49 through 91's own section 5"
(`102:1093-1094`) and nothing else. A reader of the tenth consolidation sees eight entries out of roughly
ninety.

**The carried entries lost their reasoning while the preamble promising it was carried verbatim.**
`49:889-891` reproduces the third consolidation's preamble word for word: entries are "stated with just
enough of their reasoning that a member who believes a retest would come out differently knows what has to
be overturned." The entry immediately below it, at `49:893-894`, reads in full: "Relocating the
algebraic-law machinery to hilavitkutin: refused by op directly and independently undercut by measurement."
The version at `26:694-698` carried the theory being refused and the measurement's own figure. The version
at `40:704-706` carried the measurement's reason. The fourth carries neither. The same strip is applied to
at least a dozen carried entries. A member who wants to retest the hilavitkutin relocation now cannot learn
from the droplist what has to be overturned, which is the one thing the paragraph above it promises.

---

## 6. The format is the problem, and the sentence that hides it

Every consolidation from the third onward opens with a variant of one sentence. The second states it in
full: "This document replaces file 11 as the sole reference for the design's current state. It stands
alone: no file in the panel directory is assumed read" (`26:12-13`). The third through tenth inherit it in
compressed form ("This document replaces it").

**The claim was true for the second, three and four. It has been false since the sixth, and the falsity is
undetectable from inside any single document**, because the mechanism that makes it false, the phrase
"Unchanged this stretch," is a true statement about the stretch. Nothing in a consolidation's own text
distinguishes "this section's content is below" from "this section's content is in a document I am
replacing." The two read identically, and the second is the one that has been shipping.

The failure compounds in a way none of the other archive defects do:

**A stub is not re-derivable, which collides with `108b`'s own first principle.** `108b` adopts that "a
file building on a ratified sentence whose grounds have visibly moved re-derives it before use rather than
citing it." A member cannot re-derive `102:180`. There is nothing there. The instruction and the format are
in direct conflict, and the format wins by default at every section that carries no line range.

**Pointer chains defeat the claim transitively even where a pointer exists.** `68:368-375` points at
`63:296-321`, which points at `58:497-560`. `102:1019-1020` points at `91:970`, which points at the eighth.
`102:555` points at `91:502`, which points at the eighth, and `102:555` states its own hop wrongly (R1).
Three hops, one of them broken, on ratified material.

**Compression under an "unchanged" heading is the exact failure `108b` legislated against, and it is the
archive's dominant mode rather than an exception.** Of the 127 items, roughly half sit under a heading or a
sentence asserting the material is carried. The audit's sharpest single example is not any individual
clause: it is that `49:889-891` and `49:893-894` are eight lines apart, and the second violates the first.

**What the format needs is not more discipline from the next author.** Eight consecutive authors applied
the discipline that existed and produced this. The two structural changes that would have caught nearly all
127 are mechanical: a consolidation states its own content or dies, with "unchanged" permitted only when
followed by the text, and the droplist gains a second half that is a diff against the predecessor rather
than a record of the stretch's own reversals. `108b`'s first adopted part already names the first half of
this ("either the compression is checked to entail the prior text or the difference is a droplist entry");
what the audit adds is that the check has to be performed by someone other than the author of the
compression, because the author of the compression is the person who believes it entails.

---

## 7. Reported anyway, outside the question asked

**One correction reverted in ratified text.** `78:459-460` uses "thirteen to seventeen times slower" inside
§1.21, ratified at `70b`, after `63:611-613` measured the figure down to "ten to seventeen across two runs"
and `68:806-807` recorded the correction as committed. This is the only case in the archive where a
consolidation does not merely drop a correction but restates the corrected claim, and it did so in the one
kind of text that is hardest to correct later.

**Four citation defects, three of them load-bearing.** `78:41` says "`68:816-817`'s facade-migration
framing is superseded"; `68:816-817` is the membership item, and the facade claim `78` means lived at
`63:816-817`, which `68` had already corrected at `68:26-30` and droplisted at `68:965-967`, so the eighth
consolidation presents as newly superseded a claim its predecessor had already refuted. `68:23-25`
attributes "exactly one cell of the matrix leaks" to "a sentence in the sixth consolidation's own section
6"; `63`'s section 6 is Verification and contains no such sentence, the only "one cell" in `63` is at
`63:598` about an unrelated sixteen-cell matrix, and the sentence is file 66's, written after `63`. The
droplist entry at `68:949-951` inherits the false attribution. `26:523-525` attributes to file 11 two open
packaging questions file 11 does not contain, while dropping three that it does. `102:555` is R1.

**The one case the machinery worked, recorded so it is not lost with the rest.** `102:1095-1101` droplists
`91:12-13`'s own performance claim as false, found by file 98 and reconfirmed. That is the archive
correctly catching a fabricated diligence claim in its predecessor. It is also the only entry across
eighty-seven that reports on the predecessor's own prose rather than on a deliverable's proposal, which is
why it is the exception that measures the rule.

**Two things the audit could not do and a future one should.** It did not check the probe directories or the
member files for material that no consolidation ever absorbed; `108b`'s second adopted part (a file stating
a general mechanism inside a specific finding flags it for absorption) is aimed at that population, and it
is larger than this one. And it verified that material left the standing base, not that any of it was
correct; a clause that vanished may have deserved to. Where a droplist entry would have said so, that is
precisely the entry that does not exist.

---

## 8. Method

All ten consolidations were read in full, not sampled. Each adjacent pair was diffed by three passes: a
mechanical set difference over every backticked identifier in the pair, a section-by-section word count to
locate compressions, and a full read of the later document's droplist, open list, live-defect registry and
lead-designer's-calls section against the earlier's. Every candidate was then checked twice before being
reported, once by grepping the whole of the later document for the material's distinctive terms, and once
by grepping the intervening deliverables for a resolution or an overturn. Candidates that failed either
check were dropped rather than reported. Four of the nine pairs were diffed by independent readers whose
findings were then re-verified at source by the author of this file; every load-bearing quotation and every
zero-occurrence claim in sections 2 through 5 was re-run as a grep for this document.

**The definitional-completeness line, performed.** Terms this file introduces: *silent drop* (material
present in N, absent from N+1, with no droplist entry in N+1 and no absorption elsewhere in N+1 and no
resolution in the intervening deliverables), *compression that lost content* (a statement present in both
where N+1's wording no longer entails N's), *stub* (a numbered subsection whose body is under forty words),
and *pointer chain* (a stub whose pointer resolves to another stub). Each is defined at first use.
*Ratified* is used in this document's strict sense, meaning text op wrote or a checkpoint whose own text
records op deciding; `48b`, `53b`, `57b`, `62b`, `67b`, `90b`, `95b`, `101b` and `106b` state in their own
text that a persona stood in, and nothing in section 2 rests on one of them. Named open rather than
defined: whether any of the 127 items should be restored, which is a design question this audit does not
answer and has no standing to.

**The separation requirement, performed.** The audit's central model, "a consolidation is a standing base",
is checked at the instantiation where the distinction between standing base and changelog is nonvacuous:
section 1.7, whose content differs by 1114 words between the document that states it and the document that
claims to carry it. At a section that genuinely did not change and is short, the two readings coincide and
the distinction says nothing, which is why the audit reports section-word deltas rather than the count of
"unchanged" phrases. The droplist accounting in section 5 separates entries retiring in-stretch proposals
from entries retiring predecessor text, which is nonvacuous at exactly the twenty-one entries in the second
column and vacuous at the sixty-six in the first.

**The freshly-performed-search requirement, performed.** Every universally quantified negative claim above
("returns zero hits", "returns zero hits in any panel file after `78`", "cited zero times") was run fresh
against the directory at HEAD for this document on 2026-08-05, not cited from an earlier file's search. The
searches were literal `grep` over the panel directory's `.md` files, case-sensitive except where a term's
casing varies.
