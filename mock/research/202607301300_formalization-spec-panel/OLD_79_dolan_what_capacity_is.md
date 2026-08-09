# 79. What capacity is: a parameter, not an operation's answer, and the far point fires one layer down

Stephen Dolan, file 79. I wrote file 14 (which algebra this is), file 35 (whether widening
collapses), and file 60 (value or datum), the last of which corrected its own brief before
answering it. This one does the same, in a smaller way: op's question has two readable
literalisms, they differ by exactly one, and the difference is the whole finding.

**What I read.** `78_consolidation_eight.md` in full, the only required document per the
standing instruction, and `77b_op_checkpoint_nineteen.md` in full, which states this dispatch
in op's own words. Behind the consolidation, for the specific derivation this file needs
rather than as general background: `76_probes/OUTCOMES.md` and the probe sources
`a1_naive_unification.rs`, `b2_split_by_layer.rs`, `b2b_disagreement_refused.rs`, to see the
compiled construction the consolidation summarises rather than trust the summary alone;
`74_lattner_the_taxonomy_rechecked.md` section 3, the capacity row's own reasoning; `71_
smith_the_far_point_without_infinity.md` sections 1 to 2, the far-point rule's derivation in
the words that produced it, since `78` compresses it to two sentences and the compression
loses the one clause I need (the NaN exclusion is a theorem of taking the supremum over the
*ordered* values, not a case written in). One `ls` of the panel directory, current through
file `78`. I checked the shipped `arvo-tensor/src/capacity.rs` and `arvo-storage/src/
platform.rs` for one purpose only, confirming what the capacity-as-a-type migration actually
produced before reasoning about it (a `Capacity` trait with a `type Array<T>` GAT and a
`const CAP: Cap`, `Cap(pub USize)` as a separate runtime mirror), tagged `tree-fact` below;
no claim rests on what those files' comments say the design means.

**Gates.** Canon gate, reproduced fresh from the repo root: `grep -rln "Adjustment\|Bias\|
Numeral" mock/crates/ --include="*.rs"` and the same with `FullRange\|UTerm\|AddWidth`, both
exit 1, empty. Test gate: `cargo test --offline --workspace` from `mock/`, summed per binary
this session by parsing every `test result:` line rather than trusting a printed headline,
**661 passed, 0 failed, 9 ignored**, matching `78`'s own count exactly. Toolchain `rustc
1.98.0-nightly (57d06900f 2026-05-27)`, `aarch64-apple-darwin`, confirmed by `rustc
--version` run inside the tree this session. Every compiled claim below traces to
`79_probes/`, written and run fresh this session; nothing under `mock/crates/` was touched.

## 1. The trap in op's own two sentences, tested rather than assumed

Op's checkpoint states the claim in one breath: "it is also the same as infinity on infinite
number sets, and the lastmost number in finite sets." Read as literally as the far-point rule
itself is stated (`68:275-286`, "the supremum of a numeral's ordered representable values"),
this says capacity *is* a supremum, of the finite set of valid indices a collection admits.

Taken at face value that identifies two things that are off by one, and worth naming exactly
because it is the kind of trap a design misses precisely by trusting the analogy that
motivated it. A capacity-`N` collection's valid index set, under the zero-indexed convention
every shipped construction in this domain already uses (`arvo-tensor/src/capacity.rs:56`,
`core::array::from_fn(|i| f(USize(i)))`, tree-fact, its existence and shape, not its meaning),
is `{0, 1, ..., N-1}`. Its supremum, its lastmost member, is `N-1`. But "capacity" as the word
is used everywhere in the corpus (`arvo-tensor/src/capacity.rs:24`, `const CAP: Cap`; the
dispatch's own "denotes a fixed length") names `N`, the count, not `N-1`, the last index. The
two are related by exactly one predecessor step, and conflating them would silently shift
every bound check in the tower by one, which is the sharpest possible argument for treating
them as two distinct facts before writing a single line of spec text that unifies them.

So before answering "does it map directly, or become an alias," the honest first move is to
separate two questions op's one sentence compresses into one: **is capacity itself a
supremum**, or **is capacity the parameter whose predecessor a supremum-shaped rule then
answers questions about?** Section 2 argues it is the second, and section 3 to 5 build and
compile what that costs.

## 2. Capacity is a parameter; the far point is what an operation resolves to

The numeral tower already draws exactly this line, and it is worth stating plainly because it
is the same line, one layer down. `Precision`, `Exponent`, `StoredWidth` are type-level
parameters: they *establish* what a numeral's representable set is. `OverRange`'s far-point
resolution is what an *operation* (an addition, a widen-then-narrow) answers when its true
result falls outside the set those parameters established. The far-point rule is stated over
"a numeral's ordered representable values" precisely because it needs a value set to take a
supremum of, and a value set is exactly what a parameter like `Precision` defines, not what
`Precision` itself is.

Capacity is the same kind of thing as `Precision`, not the same kind of thing as an `OverRange`
event. It is a type-level fact that *establishes* an index domain (`{0, ..., N-1}`), the way
`Precision` establishes a representable magnitude range. It is not itself a value flowing
through an operation that can land outside that domain and need resolving; the *index* is
that value, and indexing is the operation. So the far-point rule's own subject, read
precisely, is never Capacity. Its subject, one layer down, is index arithmetic: what happens
when an index reaches or exceeds a capacity, which is `arvo-container`'s contract (already
re-keyed to the numeral side once this stretch, `74:185-206`, for the unrelated reason that
saturation clamps to a *value* fact rather than a *storage* fact; the same re-keying argument
applies here without alteration, because an index clamp is equally a fact about the index
domain, not about the carrier).

This is not a rejection of op's analogy. It is a sharper reading of it than either offered
answer captures, and it is the "third answer" the dispatch itself predicted. The analogy
holds exactly, one layer down from where the dispatch's own phrasing places it: **the far
point of a capacity-bounded index space is its predecessor**, `N-1`, by the identical
supremum-over-the-ordered-set logic the numeral rule already states, with one added
sharpness the numeral case never has to confront: an index space can be *empty* (a capacity
of zero), where a numeral's value set never is. Section 4 builds this and the empty case is
where the parallel gets interesting rather than where it breaks.

## 3. Capacity's value maps directly onto the shared Nat carrier: no wrapper, no second arithmetic

Reasoned first, then compiled. If capacity is a parameter of the same kind as `Precision`,
the question "does it map directly onto the theory side" has an answer independent of the
far-point question entirely: does the *count itself* need its own ordering, its own
arithmetic, its own comparison machinery, or does it borrow the tower's wholesale? The
answer the tower's own shape already implies, and file 76's b2 already built without stating
it this way, is that it borrows wholesale, because a count and a precision are the same
mathematical object (a natural number) playing two different roles, and the design's own
carrier-at-birth rule says a closed vocabulary gets one seal, not one seal per role.

**Compiled** (`79_probes/probe_1_capacity_is_a_nat.rs`, zero feature gates, `no_std`): `Capacity`
is declared as a direct subtrait of `Nat` (`pub trait Capacity: Nat { type Array<T>: ...;
const SIZE: usize = Self::VAL; }`), not a wrapper struct holding a `Nat` and not a copy of its
arithmetic. `SIZE` is a default associated const reading straight through the supertrait,
asserted equal to the underlying `Nat::VAL` at the concrete instance `C13` (`SIZE == 13 ==
<C13 as Nat>::VAL`, both checked at compile time). No comparison, ordering, or `Gcd`
machinery is declared for `Capacity` anywhere in the probe; none is needed, because whatever
the shared carrier crate eventually builds for `Nat`/`Pos` (`Cmp`, the eleven-firing seal, the
value-uniqueness proof) is inherited automatically the moment a capacity's type *is* a `Nat`,
with zero duplicated machinery and zero divergence risk between "compare two counts" and
"compare two precisions."

One precision on top of what the probe shows, because it corrects a shape the dispatch's own
predecessor material got half right. File 76's `a1` probe tried a *blanket* impl,
`impl<N: Nat> Capacity for N`, and it is the blanket impl specifically, not the subtrait
relationship, that fails: `type Array<T> = [T; <N as Nat>::VAL]` needs the array length
computed from a generic type parameter in type position, which is exactly the refused
`generic_const_exprs` step (`76_probes/OUTCOMES.md`, part A). `probe_1` here keeps `Capacity:
Nat` (the relationship a1 also declared) but does **not** blanket-impl it over every `N: Nat`;
each concrete capacity pairs its count with a companion literal (`Slot<N, const K: usize>`,
identical shape to file 76's `b2`), and the pairing is what section 4 covers. So "capacity
maps directly onto Nat" is true of the *value*, exactly, and was never going to be true of an
automatic blanket impl covering the *array grammar* too; conflating those two claims is
exactly how the naive unification failed the first time, and keeping them separate here is
deliberate, not an oversight repeating the same mistake under a different name.

**The four uses the dispatch names, tested against the same count, no fourth vocabulary
anywhere.** `probe_1` builds and asserts, at compile time, all four: an array's length
(`Slot::<..., 13>::build`, the paired grammar fact); an index-bound membership check
(`in_bounds::<C13>(12) == true`, `in_bounds::<C13>(13) == false`, both against `C::SIZE`);
an iteration terminator (`count_live::<C13>(0) == 13`, a `while i < C::SIZE` loop); and an
arity (`FixedArity<Pz<O<H>>>::ARITY == 2`, reading `C::VAL` the identical way a capacity's
`SIZE` is read). All four consume the same `Nat`-typed count through the same projection.
The brief's own caveat, that a prior arity marker "needed its own sealed carrier for reasons
a length does not obviously share," is real but is a caveat about a *different kind* of
arity than the one these four uses need. I could not locate the specific prior artifact the
brief points at inside this panel's own corpus (searched `[Aa]rity` across every file; the
hits are all fold-arity, an unrelated subject in files 18 and 19), so I reason this from the
design's own vocabulary rather than from a citation: a marker that must witness *which*
position a value occupies (a proof of identity at a slot, the shape `D16`'s derived-safe /
asserted-`unsafe impl` split already names, `74:683`) is an ordinal-with-identity fact and
needs its own carrier for exactly the reason a plain count does not, because two positions
can share a count without sharing an identity witness. Capacity, and the four uses tested
here, never ask "which slot," only "how many" and "build or read slot `i`." That is a
cardinal fact, and a cardinal fact is precisely what `Nat` already is.

## 4. The array grammar stays a paired, non-derived fact, forced by the language, not chosen for taste

This half is not new; I confirm it rather than re-derive it, because op's dispatch asks
whether capacity maps directly or becomes an alias, and an honest answer has to say plainly
where the direct half stops. File 76's probes `a1` through `a3b` establish, compiled, that
no expression of `[T; K]` computed from a type-level `Nat` exists under the permitted feature
set: the naive form refuses citing `generic_const_exprs`, the const-block escape refuses
(`use of const in the type system not defined as type const`), and following rustc's own
suggested rewrite all the way to `type const` still refuses on the inductive step `2 *
P::VAL`, which `min_generic_const_args` cannot express (`76_probes/OUTCOMES.md`, part A,
all four probes). This is a fact about what the array-length grammar admits, not a design
preference; there is no construction on the other side of it to discover.

`probe_1` here reuses `b2`'s shape rather than reinventing it: `Slot<N, const K: usize>`
carries the `Nat` (direct) and a bare `usize` literal `K` (the language-forced position,
paired, checked to agree with `N::VAL` in an inline const block at the one construction door,
`agrees::<N, K>()`), never derived from the other. This is the crossing contract's own shape,
restated at a different boundary: a numeral's carrier and its decoded value are two things
paired and checked, never one deriving the other purely in type position (`Encoding`, section
1.3 of the base document), and capacity's split between its `Nat` value and its `Array`
grammar is the identical split, not a special case invented for this domain. The honest
statement of "maps directly, or becomes an alias" therefore has two different answers at two
different layers of the same type, and both are precise rather than approximate: the value
layer maps directly (no alias, no second arithmetic, `probe_1`, section 3); the lowering
layer is a paired, alias-shaped companion fact, forced by the grammar rather than chosen
(`b2`, confirmed rather than rebuilt here).

## 5. Where the far-point rule actually fires: a predecessor operation, genuinely new, genuinely cheap

Section 2 located the far-point rule's real subject one layer down from Capacity itself, at
index-arithmetic resolution. This section builds that layer rather than asserting it exists.

**Compiled** (`79_probes/probe_2_predecessor_and_the_far_point.rs`, zero feature gates,
`no_std`). Nothing in the panel's prior probes defines a predecessor on `Pos`/`Nat`; this is
genuinely new construction, and I say so plainly rather than let it read as machinery the
tower already had. It costs no new sealed vocabulary and no forbidden feature: it is ordinary
structural recursion over the closed `H | O<P> | I<P>` grammar, the same shape the tower
already uses for `VAL`, `Cmp`, and `Gcd` (`68:654-657`). The derivation: `I<P> = 2P+1`, so its
predecessor is `O<P>`, no recursion. `O<P> = 2P >= 2` always, so its predecessor is always
representable as `Pos`, regardless of `P`'s own shape, which is the fact that makes the whole
construction total. `O<H> = 2`'s predecessor is `H`, the one base case. `O<O<Q>>`'s
predecessor recurses through the identical three cases one level down, a carry chain through
trailing zero bits, exactly binary decrement by hand. `O<I<Q>>`'s predecessor reduces after
one step through the trivial `I` case. `H`'s predecessor is `Z`, the one place the
construction crosses from `Pos` to `Nat`. Four disjoint trait impls express this with no
specialization and no overlap, because `I<Q>`, `O<H>`, `O<O<Q>>`, `O<I<Q>>` are four
structurally distinct types under the sealed grammar's own closure.

Checked exhaustively rather than sampled, over every structural shape the grammar admits
through five and six bits (twenty values, `V1` through `V32`, covering the trivial `I` case,
the `O<H>` base case, carry chains one to four trailing-zero bits deep, and every `O<I<_>>`
reduction), each predecessor asserted against its known decimal value at compile time; every
one of the twenty assertions holds (`probe_2`). On top of it, `last_index<C: Pos + Dec>()`
projects the last valid index below a capacity, total over `Pos`, `const`-callable, no gates,
matching file 71's own far-point projection in exact shape (a total const projection over a
closed product, `71_probes/probe_1_far_point_total.rs`), applied to a different set.

**Compiled, the boundary** (`79_probes/probe_2b_predecessor_of_zero_refused.rs`). An empty
capacity has no valid index and so no far point. The far-point rule's own NaN exclusion is a
theorem of taking the supremum over the *ordered* values rather than a case written into the
rule (`71:85-87`, "NaN needs no exclusion clause... the `NanOnly`/`NoSpecials` agreement is a
theorem of the definition, not a case in it"); the index domain has the identical shape but a
sharper instance of it, because where a numeral's ordered value set is never empty, a
capacity's index set genuinely can be. `last_index` is generic over `C: Pos + Dec`; `Z`
implements neither, so `last_index::<Z>()` fails at the trait bound, before monomorphisation,
with rustc naming the missing bound directly (`error[E0277]: the trait bound Z: Dec is not
satisfied`, `help: the trait Dec is not implemented for Z`). This is not a runtime refusal
standing in for a missing case; it is the same shape as the far-point rule's own exclusion,
matched exactly rather than merely resembled, because there genuinely is no representable
answer and the type system says so at the same point the numeral rule's own exclusion says
it, before any value exists to be wrong.

## 6. The answer, stated once, spec-ready

**Capacity maps directly onto the shared Nat carrier for its value, and needs a paired,
non-derived companion fact for its array-length representation. It is not, itself, the far
point of anything; the far-point rule fires one layer downstream, as the resolution for
index arithmetic that runs past a capacity, and that resolution is a predecessor operation on
the same shared carrier, newly built here, costing no new sealed vocabulary.**

Spelled out as the three sentences a canon revision could take close to verbatim:

1. **Capacity is a parameter, not an event.** Like `Precision`, `Exponent`, and `StoredWidth`,
   a capacity is a type-level fact establishing a domain (here, an index range), not a value
   that flows through an operation and can land outside its own domain. The far-point rule's
   stated subject ("a numeral's ordered representable values," `68:275-286`) is a value set an
   operation can exceed; a capacity is what establishes such a set for indexing, not a member
   of one.
2. **Capacity's value is a direct instance of the tower's `Nat`, not a second encoding and not
   an alias.** `Capacity: Nat`, one seal, one ordering, one arithmetic, inherited wholesale;
   `SIZE` reads straight through. This closes the two-encodings finding (`74`, section 3) at
   the value layer completely: there is no capacity-specific comparison, no capacity-specific
   `Gcd`, nothing for a second machinery to diverge on.
3. **Capacity's array-length grammar is a paired, declared fact, checked to agree with the
   value at the one construction door, never derived.** This is forced by the array-length
   grammar under the permitted feature set (`76`, part A), not chosen; it is the crossing
   contract's own value-versus-carrier split, restated at this boundary rather than invented
   for it.
4. **The far-point rule's shape recurs at the index domain, one layer downstream of
   Capacity, as `arvo-container`'s out-of-bounds resolution: the last valid index below a
   capacity is its predecessor, total over nonzero capacities, undefined (refused at the type
   level, not clamped to a sentinel) over an empty one.** This is genuinely new construction
   (a `Dec`/`PosPred` pair on `Pos`), not machinery the tower already had; it belongs in the
   same shared bottom carrier crate proposed for `Nat`/`Pos`/`Bias` (`74`, section 3's own
   sorting test: does the type say what a number *is*, which `Dec` does, so it goes down),
   because it is pure `Pos` content with no capacity-specific or numeral-specific meaning.

This is offered as one of the review's required two independent reads, not as the ratified
answer; op's own standing discipline names the call his. Where it differs from the narrowing
op declined (a bare shared carrier with nothing else) is that it locates a genuinely useful,
buildable extension the narrowing never reached (the predecessor, and the index-domain
resolution it makes total), while agreeing with op's own instinct against inventing a second
encoding to hold it.

## 7. What this leaves open

The predecessor construction (`Dec`/`PosPred`) has not been checked against the shared
carrier crate's actual final shape, only against the standalone vocabulary every prior probe
in this stretch reuses; if the carrier crate's `Pos` grammar changes shape before it lands,
`Dec` needs re-deriving against the new shape, not merely re-pasting. Whether `arvo-container`
wants `last_index` itself, or a `clamp_index`/`wrap_index` family built on top of it mirroring
the numeral preset table's own `TowardNegative`/`ReduceModulo`/`Refuse` triad, is downstream
design this file does not attempt; section 6's item 4 states only that the resolution is total
and where it lives, not its full preset shape. And the "which arity marker" question in
section 3 is flagged rather than settled: I reasoned it from the design's own cardinal-versus-
witness vocabulary because I could not locate the specific prior instance the brief points at;
a second read should check whether such an instance exists in a repo or research note outside
this panel's own corpus before treating my reading as confirmed.

*Grounded on: ratified (`68b`, `70b`, `74b`, `77b:68-100`, the far-point rule at `68:275-286`),
settled shapes (`71:64-93`, the far-point rule's own derivation; `74` section 3, the two-
encodings finding), tree-fact (`arvo-tensor/src/capacity.rs:19-59`, `arvo-storage/src/
platform.rs:73`, existence and shape of the shipped `Capacity`/`Cap` split, not their meaning),
compiled (`79_probes/probe_1_capacity_is_a_nat.rs`, `79_probes/probe_2_predecessor_and_the_
far_point.rs`, `79_probes/probe_2b_predecessor_of_zero_refused.rs`, all three run fresh this
session), reasoned (the parameter-versus-event distinction, the arity-versus-cardinal
distinction, section 2 through section 6 in full).*
