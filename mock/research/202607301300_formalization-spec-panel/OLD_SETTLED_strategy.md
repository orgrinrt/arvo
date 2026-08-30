# Settled: the strategy axis and the profile axis

Archival survival sweep, restricted to the strategy axis (`Hot`/`Warm`/`Cold`/`Precise`), the preset
tables and their cells, profile variation, the relationship between a strategy and a build profile, and
the granularity question of whether a cell is a constant or a function. Extraction and verification only;
nothing here is proposed.

The theme's whole history is a single long correction. A preset table was ratified twice (`70b`, then
restated at `124`), then a checkpoint discovered the panel had forgotten that a strategy's cells can vary
at all (`142c`), then a same-day correction inside that same file discovered the "variation" it had just
proposed was not the variation op meant, then a third correction inside the same file again repriced the
whole consumer-surface question it had just reframed. Two expert files (`143`, `144`) then split on what
the correction actually implied, and op closed the split with a ruling he called settled canon in those
words (`143b`), then closed a second, larger misreading that the two expert files had both been built on
(`144b`, `144c`): that arvo's strategies and notko's build-profile mechanism were ever the same thing.
Reading only the endpoints of this chain would give a wrong and confident answer at every step. The
sequence has to be walked.

## Survivors

### 1. The strategy set is closed at exactly four: `Hot`, `Cold`, `Warm`, `Precise`

**Where settled:** `124:3602` (consolidation twelve, restating D72 from the design talk): "`arvo-strategy`
| `Hot`, `Cold`, `Warm`, `Precise`, and nothing else."

**Provenance:** RATIFIED. D72 is an op decision (2026-07-30) carried into the panel's own consolidation
and cited by every later file as the closed-set fact (`143:194`, `144:172-174`, `148`'s discussion of the
join).

**Rests on:** nothing upstream within this theme; it is the foundational fact every other claim below
indexes against.

### 2. Each preset names a stated intent, not a derived rule

**Where settled:** first stated at the design-talk level and quoted into the panel at `70:106-109` and
`124:2578-2580`: "`Hot` is as fast as possible, `Cold` stores as small as possible, `Precise` is the most
precise at the price of both storage and compute, `Warm` is the compromise that suits most default cases
and behaves intuitively."

**Provenance:** RATIFIED (op's own words, quoted rather than paraphrased at `78:398-405` per `124`'s audit
of the quotation's drift, and requoted whole again at `124:2578-2580`).

**Rests on:** claim 1 (the four names the intents attach to).

### 3. `Warm` is defined by imitation of a native Rust primitive, as a standing intent that outranks its mechanism

**Where settled:** `140b:16-21`, op present, his own words: "My standing call is 'It should behave like
native primitives in regular old rust would'... The intent, here, is what matters. The mechanisms and
theory may live freely and shift under and around it, the intent is what remains and matters."

**Provenance:** RATIFIED.

**Rests on:** claim 2 (`Warm`'s intent slot). Nothing below it: it is a constraint on the container rule,
not a consequence of one, so a storage or overflow mechanism may vary freely as long as the experienced
behaviour matches plain Rust.

This is the single most load-bearing survivor in the theme. Every later file that touches the strategy
axis cites it: `142c`'s tier-one derivation depends on it (`142c:257-259` quoting `140b`), `143`'s
uniqueness argument depends on it (`143:256-258`), `144` restates it more strongly as a uniqueness theorem
rather than a default (`144:135-141`), and `144b`'s survivor list keeps it explicitly: "the bare-primitive
path is `Warm` by derivation rather than by default." It was re-stated by op twice in two days because it
kept failing to stick in the standing base (`140b:37-52`), which the panel itself flags as a presentation
defect rather than a content dispute.

### 4. The fixed-point and float preset tables are ratified, and stand as one arm of a function rather than as an unconditioned constant

**Where settled:** ratified at `70b:6-23` ("Both preset tables are ratified"), restated in the
consolidation at `124:2601-2607` (fixed-point) and `124:2653-2660` (float), and reframed without being
edited at `143b`'s clarification section: "The existing cells are not wrong and not broken. They are one
arm of a function whose other arms are unwritten."

**Provenance:** RATIFIED, both the original content and the later reframing.

**Rests on:** claims 2 and 3. The fixed-point table's four rows (in-range direction, `OverRange`/
`UnderRange`, `StoredWidth`, `Layout`, with `Door` inert throughout since an integer ALU has no
rounding-mode state to distinguish, `70:180-191`) and the float table's divergence (`Warm`'s `StoredWidth`
drops to minimum because a real FPU delivers correctly-rounded results with no extra storage, where the
fixed-point path has no hardware behind it, `70:333-343`) both derive from the intent sentences alone,
under the discipline that a row's justification must survive deleting its shipped-source citation
(`70:42-44`, adopted at `70b:44-50` as the `tree-fact`/`tree-meaning` split).

This table degraded across the consolidations between its ratification and its restatement: `124` itself
records that four load-bearing terms from the ratified table (`TowardNegative`, `ToEven`, `HostFloat`,
"in-range direction") occur repeatedly in file `78` and zero times in the two consolidations after it
(`124:2570-2575`), which `124` calls "the single clearest proof the standing-base claim had become
decorative... on ratified material." The table itself was never wrong; the panel stopped carrying it
forward until `124` restored it in full. That the reframing at `143b` names the table "one arm... plausibly
the arm reserved for debug-assertions time" and states plainly that "nothing is owed retroactively" (
`143b:82-107`) is the final word: the table's content is not corrected, only its status is, from "the
strategy" to "the arm the panel happened to write first."

### 5. A cell in the design is a function; being constant is the special case, and any stated value holds over a domain rather than absolutely

**Where settled:** `143b:1-31`, op, stated as canon in his own words: "Function can also be a constant.
It's not either or there. And all things change and act granularly, not just warm. I call this as intent,
settled canon, right now. This small bit in this association now governs future talks."

**Provenance:** RATIFIED, explicitly named "settled canon" by op and confirmed still governing at the far
end of the panel: `148:445-446` cites it directly while resolving an unrelated conversion question ("Under
`143b`, which op called settled canon, a constant is a function, and a function whose value is derivable
from its inputs is not data").

**Rests on:** nothing upstream; it is a correction to how the design states any fact, not a fact about the
strategy axis specifically, though the strategy axis is where the panel discovered it needed stating.

This is the direct answer to the granularity question the brief names. There is no category of design
element that is "a constant" as opposed to "a function." A design sentence stating a fixed value without
naming its domain is underspecified by construction, and for any cell or fact the design states, two
things are owed: what it varies over, and where it is constant.

### 6. Variation is a general property of the design's elements, not a property confined to `Warm` or to the strategy axis

**Where settled:** `143b:32-46`, same checkpoint as claim 5: "All things change and act granularly. Not
only the build condition, and not only the strategy axis."

**Provenance:** RATIFIED, closing a live three-way disagreement past every position argued for it.

**Rests on:** claim 5.

This resolves a dispute the theme's own record shows happening in the open. `142c`'s first section argued
build-condition dependence is "a general property of the axis" (`142c:70-72`). `143` argued instead that it
reaches `Warm` alone, because `Warm` alone is defined by imitation, and proposed the rule "a cell is a
constant unless the strategy's definition is an imitation" (`143:117-121`). Op's ruling is broader than
either: not scoped to the strategy axis, and not scoped to `Warm`. `144:522-550` states the consequence for
each: `143`'s imitation rule does not survive as stated, because its "is a constant unless" clause makes
constancy the default when the ruling makes variation the default; `142c`'s framing was closer in outcome
but reached it through a claim about the strategy axis specifically, when the ruling is about everything.
What survives of `143`'s observation, restated to fit the ruling: "A posture defined by imitation inherits
the arms of the thing it imitates. A posture defined by a rule has the arms someone writes for it. Both
vary; they differ in who supplies the variation" (`144:541-546`).

### 7. arvo's strategies and notko's build-profile mechanism are not the same concept and are not required to align

**Where settled:** `144b:9-19`, op, stated twice in one round: "Notko or hv are not directly associated
with arvo. The concepts need not align, they are different things for different purposes and in different
projects. They have synergy, but no continuity as such." And again: "arvo strategy is not the same as
notko optimize for profiles. They have synergy, nothing more." Closed further at `144c:16-26`: the
alias-reach limitation the two expert files spent a full round on "is not arvo's... A limitation in notko's
rewriter is not an arvo canon finding," and the mechanism built to fix it "stays in the record as a worked
answer rather than as a proposal, and nothing in arvo's canon should be built on it."

**Provenance:** RATIFIED.

**Rests on:** nothing within the strategy axis; this is a scope-boundary ruling, not a content claim about
the four strategies.

This is the decisive settlement for what "profile" means in this theme. It was op's correction to a
framing the dispatching agent had carried into two consecutive expert briefs (`143`, `144`), both of which
built substantial mechanism (notko attribute granularity, tier-one/two/three consumer routes, an
alias-reach fix) on the assumption that a notko "profile" tier and an arvo "strategy" had to correspond.
They do not have to, and per this ruling do not. What remains true after the ruling is narrower and is
listed as its own survivor next.

### 8. A bare trait bound (e.g. `T: Add<Output = T>`) carries no representational information; the representation travels with the concrete type the consumer supplied

**Where settled:** established independently at `143`'s `p8b_bound_determines_no_width.rs` (`E0576`, no
associated width to read) and at `144`, which formed the same answer before reading `143` per the
two-expert protocol (`144:123-129`). Confirmed surviving `144b`'s notko-scope correction at `144b:46-48`:
"A bare `T: Add<Output = T>` carries nothing about width, range, resolution or container, and the
representation travels with the `T` the consumer supplied. arvo reads it rather than deriving it."

**Provenance:** TWO EXPERTS, independently derived before either read the other, per `144:56-57`'s own
statement of method ("I formed answers and built probes. Only then did I open `143`").

**Rests on:** claim 1 is presupposed (there must be a fixed strategy set for the question "which one does
a bound select" to be meaningful), but the claim itself is compiler-checked and independent of the exact
set.

### 9. A total (non-fallible) signature excludes a refusing strategy

**Where settled:** `143`'s `p8_what_a_bound_determines.rs` (`E0271`, a `PreciseNum`'s `Add` cannot satisfy
`Output = Self` because `Precise` returns through a refusing branch), reached independently at `144:149-154`.
Confirmed surviving at `144b:47`.

**Provenance:** TWO EXPERTS, independently derived.

**Rests on:** claim 3's cousin for `Precise`, namely that `Precise` is fallible by construction
(`Refuse`/`Refuse` at `OverRange`/`UnderRange`, ratified in the table at claim 4), and claim 1.

### 10. When two sources would supply conflicting strategies, the four do not form a lattice; no join exists, so the only sound dispositions are precedence (with a mandatory report) or refusal

**Where settled:** conclusion reached independently by `143:284-297` (grounded on D72's closed four-element
crate table, `124:3602`) and `144:169-197` (grounded instead on the ratified view lattice, `124:1386-1391`:
"the lattice is not a chain: `Hot` on a signed numeral and `Precise` below its accumulator's
interior-safety threshold sit at incomparable points"). `144` itself judges its own ground stronger, since
it "does not evaporate if op ever adds a posture" (`144:576-577`), and this file adopts that reading as the
one to keep, while recording that the conclusion carries two independent groundings.

**Provenance:** TWO EXPERTS, same conclusion, independently derived grounds. Not separately re-ratified
by op in the material read for this sweep.

**Rests on:** claim 1 (`143`'s ground) and the panel's ratified view-lattice result at `124:1386-1391`
(`144`'s ground, outside this theme's own derivation but cited as settled fact).

The disposition itself, precedence plus a mandatory diagnostic report rather than a silent override, is
agreed by both files and is grounded in the already-ratified downstream contract at `124:3579` ("a build
layer... acts on `Policy` only inside its own declared envelope") independently found by both
(`143:504-511`, `144:159-163`). Note that under claim 7, the "enclosing annotation" this precedence rule
was written to arbitrate against was the notko `#[profile]` attribute specifically, so the concrete
attribute-vs-declaration scenario that motivated it is now out of scope for arvo canon. The abstract rule
(strategies form a non-chain, so no join exists, so any future mechanism supplying two strategies to one
use site must use precedence-with-report or refusal) survives the notko correction because it is a fact
about the strategy set itself, not about any one supply mechanism.

## Casualties

### The build profile as an orthogonal axis multiplying the preset table

`142c`'s first section proposed that "every strategy cell is a function of the active profile, not a
constant... the strategy is what the consumer declares about intent, the profile is what the build
declares about the deployment" (`142c:59-68`). Killed inside the same file, same day, by op: "The notko
profiles are not cargo profiles" (`142c:128`). The correction that replaced it (the profile is a lexical
scope knob supplying a strategy, not an orthogonal build-condition axis multiplying cells) was itself later
narrowed to "not arvo's concern" by claim 7 above.

### Build-condition dependence as a property confined to `Warm`

`143`'s central finding, "build-condition dependence reaches `Warm` alone... because `Warm` alone is
defined by imitation" (`143:107-116`), and its proposed rule, "a cell is a constant unless the strategy's
definition is an imitation, in which case the cell is whatever the imitated thing does" (`143:119-121`).
Explicitly killed at `144:528-536`: "That sentence is built on the dichotomy op just dissolved... So the
rule does not survive as stated." Superseded by claim 6 above, which is broader than both `143`'s position
and the "general property of the axis" position `142c` had argued for.

### The three-tier consumer framework built on the notko `#[profile]`/`#[optimize_for]` attribute as arvo's own supply mechanism

`142c`'s tier-one/tier-two/tier-three consumer model (`142c:220-320`), `143`'s type-rewriting routes R1
through R3b for the attribute (`143:147-201`), the alias-reach limitation reported as an arvo problem
(`143:180-196`), and `144`'s "elision is a marker, resolved at the operation" mechanism built to fix that
limitation (`144:419-521`). Killed at `144b:9-19` ("no continuity as such... nothing more") and closed
finally at `144c:16-41`: the limitation "exists at the macro level... It does not exist at the type
level," it "is not arvo's," and `144`'s mechanism "stays in the record as a worked answer rather than as a
proposal, and nothing in arvo's canon should be built on it." Whatever the eventual mechanism for supplying
a strategy at a use site turns out to be, none of the concrete machinery explored across these three files
is arvo canon.

### The `Cold` name collision between notko and arvo

`144`'s finding that "notko `Cold` and arvo `Cold` are different behaviours under one name, and they
disagree on whether an operation can fail" (`144:226-238`), reported to op as something needing a call
"cheap to make now and expensive later" (`144:669-671`). Dissolved at `144b:26-29`: "Under the ruling these
are two words in two projects that happen to coincide. There is no shared vocabulary to reconcile."

### "No tier file can express an arvo posture" as a gap in the design

`144`'s framing of the notko extension mechanism's silence on arvo postures as a missing capability
("`Precise` has no tier... no tier file can express a posture arvo would recognise," `144:355-363`).
Dissolved at `144b:31-34`: "A notko tier file is not supposed to express an arvo posture. The absence is
the separation working."

## Coverage

Read in full: `70`, `70b`, `140b`, `142c`, `143`, `143b`, `144`, `144b`, `144c`. Read in substantial part
(targeted sections, not cover to cover): `124` (the consolidation, ~150 lines pulled across the D72 table,
the fixed-point and float preset tables, the envelope sentence at `124:3579`, and the view-lattice result
at `124:1386-1391`, out of a 576 KB file), `145b`, `145c`, `149`, `148` (sections bearing on the `143b`
citation and on where the preset tables live in the conversion order's own slice). Skimmed via targeted
grep only, not read: the remaining checkpoints (`04b` through `139b`) and the remaining expert files
(`01` through `69`, `71` through `139` excluding `70`/`140b`, `141`, `146`, `150`), checked for
strategy/profile/preset keyword density and read only where the density or a checkpoint's content pointed
back into this theme.

The concentration of this theme's real content in the terminal stretch (`142c` through `144c`, all dated
2026-08-07) is not an artefact of my search: `142c` itself states that it checked the two main
consolidations (`110`, `124`) for any prior mention of a build-profile mechanism and found none, and no
checkpoint before `142c` carries a ruling on the strategy/profile relationship. The fixed-point and float
preset tables themselves (survivor 4) were settled once, early, at `70`/`70b`, and every later mention
through `124` is either a citation of that ratification or, per `124`'s own audit, evidence that the
citation had stopped being carried forward accurately until `124` restored it. I am not confident I found
every incidental use of `Hot`/`Warm`/`Cold`/`Precise` as example strategies inside files whose actual
subject is a different theme (container derivation, numeral conversion, the erasure gate); where such uses
appeared in my targeted reads they were treated as downstream consumers of the ratified table rather than
as settlements of it, and are not reported here.

Survivor count: 10. Casualty count: 5.

The three survivors that most constrain what the canon can say: claim 5 (a cell is a function; constancy
must be established, not assumed), because it changes the required shape of every future sentence about a
strategy's behaviour; claim 3 (`Warm` is defined by imitation, as intent, outranking mechanism), because it
is the one fact every downstream derivation in this theme cites; and claim 7 (arvo strategies and notko
profiles are not the same concept), because it retroactively scopes out a full round and a half of expert
work as non-canon and sets the boundary any future profile-supply mechanism has to be designed inside.
