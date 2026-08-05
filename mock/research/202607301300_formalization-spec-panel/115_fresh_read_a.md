# Fresh read A: 110_consolidation_eleven.md, cold

Read start to finish, once, with no other file opened, per the constraint given. The report below answers
the five questions in the order given, question 4 first as instructed.

## 4. Could I implement from this

No, not tomorrow, and the document does not want me to. Its own governing statement is that implementation
is explicitly out of bounds until the canon is complete: "the panel produces canon, not source... `mock/crates`
is out of bounds until the canon is complete and earmarked as arvo's first full canon" (line 4419-4421). Op's
own four-phase sequence, quoted at line 338-344, puts "implementing anything" after "settle the canon in
full" and two further phases. So the honest answer to "where would you start" is that the document itself
says: not yet, and names roughly twenty items still waiting on a human call before the canon it describes is
even finished (the "loudest for op" list, line 4423-4470).

If told to start anyway, here is what would actually happen.

I could transcribe a handful of trait declarations directly, because they are given as literal Rust code
blocks: the `Numeral` identity contract (line 787-797), `Encoding` (line 869-875), `Policy`/`Quantisation`/
`Resolution`/`Direction` (line 2905-2916, 2963-2988), and the `Nat`/`Pos`/`Bias`/`Exponent`/`Radix` sealed
encoding (line 2889-2896). Those would compile as stub traits with no bodies.

I would stop hard at the very next step, because **the type consumers actually hold, `Number<N, S>` itself,
is never declared anywhere in 5867 lines.** Every mechanism in the document (the crossing contract, the
quantiser, the fold, the digest, the algebra) is stated in terms of "a numeral," "a policy," "a lowering,"
but the concrete generic struct that ties `Numeral` + `Policy` + `Lowering` + a strategy marker together into
one value type is never written down as code, not once. Line 743-745 writes "A value of `Number<N: Numeral,
S>` is an integer `k`..." (two parameters), but line 3159 shows working example code reading
`Number<Fix13_3Signed, Warm, MinWidth>` (three parameters, with the third one naming a `StoredWidth`
value directly). Section 1.3 (line 877-881) states as a settled, measured design decision that `Encoding` is
"nested inside `Lowering` rather than carried as a third type parameter on `Number<N, S>`," specifically to
avoid a three-parameter form, and prices the alternative at 1.8x worse diagnostics. Nowhere does the document
reconcile the two-parameter framing with the three-parameter example. My first question to whoever wrote
this would be: what is `Number`'s actual generic parameter list, and does a `Strategy` marker like `Warm`
expand into a `(Policy, Lowering)` pair, or is `S` itself a `Lowering`?

Past that, two more load-bearing decisions are explicitly marked as still pending a human call, and the
document explicitly declines to pick for me:

- The array/capacity storage grammar. Three competing designs (a paired numeral-plus-literal form, a bare
  const parameter, a numeral-derived-storage form) are all built, priced, and compared in a table (line
  3581-3596), and the document states outright: "This document does not pick; op does" (line 3579). I could
  not write the type that backs a fixed-size array without this being settled.
- Whether `Numeral`'s own shape is the flat four-member form op ratified as D68, or the nested
  `Implicit`/`Ranged` form the document actually uses everywhere else. Section 1.2's own correction says "two
  op calls, contrary shapes... **Whether op accepts the supersession is op's**, and it is on the open list"
  (line 843-851, 4480-4481). This is the single most central trait in the whole design and its shape is
  disputed between two of op's own decisions.
- The crate split (D72, section 1.25) has a compiled counterexample (file 09's dishonest impl, line
  3154-3168) showing the split does not actually stop a hostile crate from writing a law-breaking impl, and
  op's own reserved call on whether the split is worth it "has never been made" (line 3170-3182, 4474-4479).
  I would not know whether I am writing one crate or six.

So the concrete answer: I would get as far as a handful of stub trait declarations and the sealed number
encoding, then stop at the first attempt to write the actual numeral type, and the question I would have to
ask is the `Number<N, S>` arity question above, followed immediately by the array-grammar and `Numeral`-shape
questions, because none of the three has an answer this document is willing to give.

## 1. What could not be reconstructed

The document's central claim is that a reader can reconstruct the design from it alone (line 33-35). That
claim does not survive contact with the document's own citation apparatus. Nearly every substantive
paragraph carries a citation of the form `file:line` into one of roughly thirty numbered member dispatches,
op checkpoints, or "probe" directories that this file does not contain and I was not permitted to open. The
document is honest about this dependency in its own framing (it opens by saying it is "required to absorb"
two other files, line 4-5, in the same paragraph that later claims to stand alone), but the dependency is
real and total: strike every citation and what remains is prose describing conclusions with no derivation.

Below are the specific, load-bearing gaps, ranked by cost to an implementer.

**`Number<N, S>` itself.** Covered under question 4. Every downstream mechanism assumes it exists and knows
its shape; the shape is never given. This is the single largest gap in the document.

**The decision register.** Roughly forty "D-numbers" (D5 through D75) are cited throughout as the ratified
ground for design choices, e.g. "D72, op's own" (line 3104), "D69... ratified" (line 749-750), "D53" (line
3006), "D63" (line 3025). A handful are quoted in full when the document wants to make a point (D56 at line
3273-3278, D67 at line 94-101, D71's consequences at line 2532-2539). The rest are named and used as
premises with no text given: D5, D6, D7, D9, D18b, D10, D11, D15, D17, D23, D27, D28, D31, D32, D33, D38,
D39, D40, D41, D42, D43, D44, D45, D48, D49, D50 are all cited by number, in the crate-table section alone
(line 3204-3214), with no more than a clause of gloss. A reader cannot check whether the document's use of,
say, D42 or D45 is faithful, because D42 and D45 are never quoted. Worse: the document tells me the
identifiers are not even unique. "The identifiers are not unique. The inherited-state file carries two
overlapping D1 through D4 sequences" (line 267-269), and "a reader following that into the talk file reads a
forbidden-feature ruling as a shape-crate ratification" (line 273-274). So even the citation key into the
missing material is stated, by the document itself, to be ambiguous.

**`Folded<N>`.** Used repeatedly as the type-level carrier of the site count (line 1869-1870, 1934-1935,
3343, 4864): "a caller needing a definedness-faithful fold takes `Folded<0>`... refused by `E0308`" (line
1324-1325). Never declared. I inferred it is a phantom marker on a fold's result type, refused by ordinary
type mismatch when it disagrees with a caller's declared expectation, but I do not know if it wraps a value,
what its other type parameters are, or how it relates to `Number<N, S>`.

**`notko-hlist` / `Cardinal` / `Cons` / `Empty` / `Length`.** Cited as an existing prior-art crate whose
"binding-time sentence" is still owed (line 3214, 4760-4761, 4966-4972), and used as the underlying mechanism
for the bitfield's heterogeneous product (line 3747, 4100-4101). What an "hlist" is, or what `Cardinal`
computes as opposed to `Length`, is never stated in this file; the reader is assumed to already know.

**`Reduce`.** The tower's own machinery for reducing a rational to lowest terms, invoked repeatedly as the
thing that makes a declaration expensive when forced ("the type checker to run the tower's own `Reduce`
machinery on an unreduced pair, costs 13.80 ms," line 2276-2277; "a direct `Reduce` bound," line 4746), never
declared or given a signature.

**`Dec` / `PosPred`.** The predecessor operation on the sealed `Pos` encoding, load-bearing for the capacity
work (line 3470-3475), described only informally: "`I<Q>` steps to `O<Q>` with no recursion; `O<O<Q>>`
recurses through a carry chain bounded by the number of trailing zero bits" (line 3472-3474). No code given.

**The `Bool` / `Cap` / `USize` / `NUSize` types.** Used constantly as concrete named types (`Bool`'s six
doors are enumerated at line 3991-3995 down to file:line citations in the shipped tree), but never declared
in this document as Rust structs. I inferred `Bool` wraps `bool` and `USize`/`Cap` wrap `usize`, from usage
(`Deref<Target = bool>`, `From<Bool> for bool`), not from a definition.

**IEEE/OCP vocabulary assumed known.** `E4M3`, `E4M3FNUZ`, OCP, BID vs DPD (line 2172, "BID against DPD
cannot itself change any of the three statements"), decimal cohort mechanics: all used as settled background
knowledge. A reader who does not already know IEEE 754-2019's decimal encoding or the OCP FP8 spec cannot
reconstruct what these terms denote from the document alone; the primary-source reads are explicitly listed
as still owed (line 4696-4703).

**Geometric-algebra vocabulary in section 1.28.** `Spin(n)`, "the even subalgebra of `Cl(3)`," rotor
components as elements of `Spin(n)` versus degrees of freedom (line 3679-3688): used to overturn D10's own
storage argument, with no definition of the algebra given in this file.

**`SameFaceAs<Q37>`.** Appears only inside a quoted compiler diagnostic (line 2317) as the trait whose
absence produces the review's best diagnostic message. Never declared as a trait anywhere else in the
document.

## 2. What had to be guessed

**Whether `S` in `Number<N: Numeral, S>` denotes a `Strategy` marker (`Hot`/`Warm`/`Cold`/`Precise`) or a
`Lowering` directly.** Section 1.23 (line 3006-3021) frames `UFixed`, `IFixed`, `FastFloat`, `StrictFloat`
as "four names for four compositions" and states "compositions are public and bindable... strategy presets
are the default documented path, not the only path" (D52, line 3016-3018). This implies `S` is not literally
a preset name but something a preset expands into, but the expansion itself (does `Warm` desugar to a
`(Policy, Lowering)` pair bound at one generic slot, or is `S` genuinely the strategy tag with Policy and
Lowering derived from it via associated types) is never stated as a rule. I read it the first way; a
different, equally careful reader could read it the second way, and the two readings have different
consequences for how a consumer would write a divergent preset.

**Whether the `Nat`/`Pos`/`Bias` structural encoding lives in a crate of its own that everything else
depends on, or is scattered per-crate.** File 74b is credited with adopting "one sealed bottom carrier crate...
for capacity with `Capacity` kept as a named semantic alias over it" (line 4372-4373), and the same encoding
is separately reused for `Bias`, `Exponent`, and the notation macro. The crate table in section 1.25 (line
3110-3118) does not list this bottom-carrier crate at all among its six rows, and section 1.27 (line
3475-3476) says the `Dec`/`PosPred` construction "belongs in the same shared bottom carrier crate proposed
for `Nat`/`Pos`/`Bias`," present tense, as if it is not yet placed. I could not tell whether this crate is a
seventh member of D72's table that the table simply omits, or a distinct, still-unlocated crate.

**Whether "the site count" (`Folded<N>`'s `N`) is carried at the value level at all, or purely at the type
level.** The text treats it as something a caller "takes" as a type argument and something the compiler
"refuses" on mismatch (E0435, E0308), which reads as purely type-level, but section 1.14 also says the site
count is "one instruction outside the loop" and "what gets published" (line 1867-1868), language that
suggests a real, computed value exists somewhere too. I read this as: the type-level marker is checked
statically and a runtime value tracking the same count may or may not also exist depending on which
`Door`/delivery mechanism a consumer picked (Thread B, section 5, line 4870-4914 is explicitly still open on
exactly this "how does a refusal arrive" question), but the document never states this reconciliation in one
place; I inferred it by triangulating three separate sections.

**Whether "declaring" and "shipping" a rung of the algebra ladder (Magma, Semigroup, Monoid, Dioid, ...) is
the same act as implementing it.** D75 (line 1490-1493) is quoted as separating the two, "D47's sketch-and-
bench obligation attaching to implementations... and not to declarations," but the document never states
which of the two acts is what a `pub const trait Semigroup` in source would actually correspond to. I read
"declaring" as "the trait exists in the crate, unconditionally, for every rung the theory has a name for,"
and "implementing" as "a concrete numeral has an impl of it," but this is my own gloss, not a quoted
definition.

## 3. What contradicts what

**The subsection count is wrong by one, and it is checkable.** Line 736-737: "Twenty-nine subsections. Every
one states its content." The section headers actually run `1.1` through `1.30` continuously (line 741, 785,
867, 916, 1021, 1152, 1243, 1359, 1464, 1523, 1602, 1622, 1739, 1857, 1942, 1962, 2145, 2216, 2346, 2358,
2464, 2655, 2886, 3042, 3090, 3267, 3437, 3624, 3739, 3941), which is thirty subsections, not twenty-nine.
Small on its own, but this is exactly the kind of count-not-a-list the document itself warns against
elsewhere ("a count cannot be checked and a list can," line 1647, restated at line 1778-1779), and here the
count is simply miscounted against the list sitting directly beneath it in the same document.

**The "eleven operations" enumeration does not total eleven.** Line 1580-1582, describing the closed
`Growth`-removal proof: "The positive enumeration: eleven operations drawn from the design's surface (in-
numeral add, sub, mul, div; `mul_full`; `mulnum` over `Ranged`; `div_exact`; the `div_floor`/`rem` pair;
`fold`, `fold_sequential`, `fold_compensated`; `quantize`)." Counting the named operations in that
parenthetical: add, sub, mul, div (4) + `mul_full` (5) + `mulnum` over `Ranged` (6) + `div_exact` (7) +
`div_floor`/`rem` as one item (8) + `fold`, `fold_sequential`, `fold_compensated` (11) + `quantize` (12).
That is twelve named operations under the most charitable reading (treating the `div_floor`/`rem` pair as one
item), or thirteen if the pair is counted as two. Either way, not eleven. The document leans hard on this
sentence as "the stronger of the two results" backing a universal claim ("no operation expressible in this
type system's dispatch discipline can have policy-dependent growth," line 1587-1589), so a miscount at the
premise is not decorative.

**`Number`'s arity: two parameters versus three, unreconciled.** Covered fully under question 4. Line
743-745 states the type as `Number<N: Numeral, S>` (two parameters); section 1.3 (line 877-881) states, as
a ratified, measured design decision, that a third parameter is specifically avoided; and section 1.25's own
worked example at line 3159 writes `Number<Fix13_3Signed, Warm, MinWidth>` and
`Number<Fix13_3Signed, Warm, DoubleWidth>`, three parameters, with the third naming a concrete `StoredWidth`
value. I could not find an assumption inside the document that reconciles these without either treating the
two-parameter statement as informal shorthand (never said) or treating the three-parameter example as a
deliberately non-standard, hostile spelling used only to prove a point about crate boundaries (plausible, but
never stated as such; the surrounding prose calls it simply "the impl" with no flag that its spelling departs
from the design's own ratified arity).

**Op's ground for `Cold`'s nearest-rounding choice cites an axis the same document says was ratified out.**
Line 2524-2530: "Op's own ground at `talk:1674-1678` was that `Cold`... 'is already paying a widen and a
narrow per operation'... **That ground depends on the `Widening` axis, which was ratified out at `39b`**, so
the substituted ground above... is a correct re-derivation... rather than a restatement." The document
catches and flags this one itself (it is not a silent contradiction), but it is worth naming as evidence of
how load-bearing citations to op's own words can rest on premises the design has since deleted; the document
is transparent about this specific instance but does not claim to have swept for others of the same shape.

## 5. The document's own shape

This is not one coherent design write-up. By its own account it is the eleventh in a chain of "consolidations"
of a much longer review process (roughly one hundred numbered member dispatches, several dozen "op
checkpoints," and a running audit trail of at least three prior self-audits, files 111 through 114, whose
corrections are spliced inline throughout as blockquotes beginning "Correction, file 114"). Structurally, the
substantive design content (section 1, roughly half the file by line count) alternates between two very
different registers within the same subsection, often within the same paragraph: settled, multiply-verified
material carrying op's own verbatim quotes and compiled counterexamples (the crossing contract in 1.4, the
strategy-door tables in 1.21, the division dissolution in 1.13), sitting beside freshly reopened, single-pass
findings explicitly awaiting a human decision (the array grammar in 1.27, "this document does not pick; op
does," line 3579; the truth-contract fork's alternative form in 1.30, "priced rather than recommended," line
4303-4304).

The other half of the document (sections 0, 2, 5, 6, 7, 8, 9, roughly 2900 of 5867 lines) is not design
content at all. It is meta-bookkeeping: a registry of who ratified what and when (section 0.4), a cumulative
log of every sentence a prior consolidation silently dropped (section 6, "the droplist"), a separate log of
proposals born and killed within one stretch (section 7), a full accounting of what this specific document
restored and by what method (section 8), and a verification appendix re-running the document's own search
claims (section 9). Section 8 states outright why: "Every consolidation from the third onward opens with a
variant of 'this document replaces it... no file in the panel directory is assumed read.' The claim was true
for the second, third and fourth. It has been false since the sixth" (line 5421-5424), and diagnoses the
mechanism as a phrase, "Unchanged this stretch," that is "a true statement about the stretch" while being
false about the document (line 13-15, 5424-5426).

So the document's own self-description is accurate: it is an assembly of positions taken at different times,
by different personas standing in for an absent human lead in most cases (the "persona-decided" checkpoints
named at line 248-252 as nine files where "a persona stood in for op during his absence," explicitly graded
lower on the provenance ladder than op's own words), stitched together under heavy editorial narration about
its own provenance failures. It reads less like a specification and more like the working transcript of a
committee that has spent several of its later sessions auditing whether its earlier sessions' minutes were
honest, with the design itself, when it does surface, alternating between confident and visibly unsettled
from one paragraph to the next with no typographic signal distinguishing the two other than close reading of
words like "adopted," "ratified," "confirmed," "op's call," "priced rather than recommended," and "not
adopted."

## What worked

Three things are genuinely complete and buildable as stated, without needing anything outside this file:

**The sealed number encoding** (`Nat`, `Pos`, `Bias`, `Exponent`, `Radix`, line 2889-2896) is given as full
constructor lists with the sealing rationale (line 817-826) and the const-eval ceiling behaviour bisected to
an exact bit (line 2244-2249). A reader could write this module today.

**The crossing contract's three statements plus its precondition** (section 1.4, line 916-1019) is
internally complete: the maps are named (`encode`, `decode`), the three statements are given as formulas, the
precondition is derived rather than asserted, and the escape family is quantified with a worked numeric table
(line 961-970). This section does not lean on an external file to be understood.

**The strategy-door tables** (section 1.21, line 2502-2653) are reproduced as literal markdown tables with a
stated derivation for every cell and the ratifying checkpoint named. This is the one place in the document
where the promise in the opening paragraph (state the content, not a pointer to it) is fully kept, and it
shows: it is also the densest, most citation-light, most immediately usable section in the file.
