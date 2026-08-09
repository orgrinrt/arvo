# 47. What a consumer writes

**Member:** Casey Muratori. I wrote file 06, forty-one files back, on the consumer surface. The design
has been rebuilt underneath it since: the axis set it argued against is gone, the encoding it assumed
does not exist, and the fused-verdict question it left open has been answered by a mechanism nobody
had yet proposed when it was written. I carry none of its conclusions forward, and where anything
below agrees with it, it agrees by re-derivation. The habit of mind this dispatch wants is the one I
have argued for longer than anything else: a tool is not what its architecture diagram says, it is
what a person types into it and what it says back when they get it wrong, and those two things are
measurable. So I measured them.

**Gate:** run before this work, myself. `cargo test --workspace` from `mock/`: 654 passed, 0 failed, 9
ignored, summed from the per-binary `test result:` lines rather than trusted from a headline, matching
files 41 through 46 exactly. The shipped surface this dispatch is about is empty: `grep -rln
"Adjustment\|Bias\|Numeral" mock/crates/ --include="*.rs"` from the repo root returns nothing (exit
1), the corrected command file 45 established (`45:456-475`), so the test bodies to read are the
review's own probe files and I read them as source. The nearest shipped consumer surface, which this
file's question does bear on, I read in full: `mock/crates/arvo/tests/ufixed_ops.rs`,
`strategy_semantics.rs` and the head of `fixed_point_div.rs`. They are real assertions against real
containers with no tautology in them (`ufixed_ops.rs:21-23` asserts three distinct arithmetic results
against hand-computed values at a named container width), and they are the baseline for what a
consumer types today. Canon gate: `40_consolidation_three.md` and `44b_op_checkpoint_ten.md` in full
before a line of code. Nothing below overturns a ratified call. Section 3's proposals are additions at
the surface, and section 3.2 changes the shape of one unratified mechanism from file 37, which is
named as such.

**What I read:** `40_consolidation_three.md` in full, twice. `41` through `46` and `44b` in full, the
deliverables since it. By exception, at the source rather than through any file's paraphrase because I
build directly against them: `37_probes/probe_4_view_as_a_return_type_and_the_transfer.rs` and
`probe_4d` in full (the mechanism this file's section 3.2 reshapes), `37:225-243` (the
evaluation-strategy negative), `46_probes/vu_nat_sealed_adj.rs` and `vu_bias_sealed_adj.rs` in full
(the tower every probe here compiles against), `46_probes/OUTCOMES.md` (the reproduction commands the
second reads owe), and `41:95-131` (the `Bias` construction, for the `Int` read). `ls` of the review
directory once: 46 numbered deliverables plus probe directories before this one.

**What I compiled or measured, separated from what I reasoned.** Everything load-bearing here is
compiled against the pin (`rustc 1.98.0-nightly (57d06900f 2026-05-27)`, confirmed with `rustc
--version` inside the repo; host `aarch64-apple-darwin`, confirmed with `rustc -vV`, and every
instruction-free figure below is a pin-and-host fact in file 45's sense). `47_probes/` holds two
tower copies, twelve numbered probes of which six are committed refusing (one of those in half),
and a `price/` sweep run twice.
Reasoned without an artifact: the recommendation in section 1.1 is a reading of three compiled facts
plus two cited measurements from files 05 and 11, not itself a measurement; the naming argument in
section 1.2 is judgment; the rewrite-cost estimates in section 3 are counts of what does not exist
yet rather than of a diff. **One hypothesis of my own was refuted by my own probe and I have left the
refutation in place rather than deleting the claim**: section 1.1 was written to argue that the short
circuit makes a term's report grouping-dependent, and an exhaustive check over all 81 four-leaf terms
says it does not. What survives is a different and worse property, and the argument I came in with is
void.

---

## 0. What this file is, and the one thing thirty files have not asked

Thirty deliverables have asked whether this design is correct, and it has become far more correct. In
the last stretch alone a coordinate system was inverted, an encoding was replaced with a value-unique
one, a nine-point lattice replaced a three-way fork, and a perimeter that had been reported closed was
found open twice. That is the review working.

Nobody has asked what it is like to use.

That is not a soft question and it is not a matter of taste. A tool whose correctness is provable and
whose surface is unbearable has failed, and the failure is invisible from inside every angle taken so
far, because every one of those angles reads the design's own vocabulary fluently. The consumer does
not. The consumer opens an editor and types.

So this file types. Section 1 answers the two sentences the design owes, both of which are questions
about what a person expects, by writing the person's code under each answer. Section 2 is what writing
that code found, which is worth more than the two sentences and is the reason this dispatch is a design
contribution rather than two closed items. Section 3 is three proposals with their costs. Section 4 is
the two second reads owed to files 45 and 46, kept out of the way as instructed.

Throughout, the workload is the one this design exists for and the one
`.claude/rules/arvo-toolbox-not-policer.md` names as its primary case: a contiguous column of millions
of elements, folded, in a program where every saved bit compounds across the element count. Three
million Q0.15 telemetry samples, summed, narrowed. Nothing exotic.

---

## 1. The two sentences

### 1.1 The evaluation strategy of a refusing operand's sibling: strict

**Recommended sentence, for the spec:**

> Every operand of an operation is evaluated. An operation's grade is the join of its operands' grades
> with whatever the operation itself contributes, whether or not any operand refused. A refusal does
> not suppress its sibling's quantisation events.

**And the corollary, which is the reason and belongs next to it:**

> A term's grade is therefore a function of the term. It is invariant under regrouping, which the
> transfer rule licenses, and under reordering, which commutativity licenses. Neither invariance
> survives a short circuit.

The consolidation records this as "a sentence the design owes, not a question awaiting more evidence"
(`40:639-641`), with file 39's standards test tilting toward strict without deciding. Here is what the
consumer's code says about it.

#### The argument I came in with, and its refutation

I expected the short circuit to make a term's report depend on its grouping. A regrouping refuses at a
different node, so it skips a different sibling, so it reports a different event multiset, so the
design would be publishing an object that is not invariant under the one transformation its own
transfer rule licenses. That would have been decisive.

It is false. `47_probes/probe_4_the_siblings_report.rs` checks all 3^4 = 81 four-leaf terms over the
alphabet {refuses, rounds, clean} under three groupings (left-nested, balanced, right-nested), under
both readings, as a const assertion:

```rust
const _: () = assert!(grouping_invariant_everywhere(STRICT));
const _: () = assert!(grouping_invariant_everywhere(SHORT));
```

Both hold. The reason is obvious once the check says so and I did not see it while arguing: every
grouping of a term visits its leaves left to right. A left-to-right short circuit therefore reports
exactly the prefix before the first refusing leaf, whatever the tree shape. Regrouping does not move
leaves. **Any objection to the short circuit on grouping-invariance grounds, including the one this
probe was written to make, is void**, and I would rather record that than quietly not mention it.

#### What survives, and it is worse

The short-circuit report is not invariant under **reordering**, and the delivered value is. Same
multiset of four channels (one refusing, two rounding, one clean), three orders, balanced grouping,
all const-asserted:

| order | short circuit, events reported | strict, events reported |
|---|---|---|
| refusing channel first | 0 | 2 |
| refusing channel second | 1 | 2 |
| refusing channel third | 2 | 2 |

The refusal is present in all six cases (`causes == 1` throughout, asserted), so the delivered outcome
is identical and only the report moves. That is what makes this a defect in the published object
rather than in the arithmetic.

Three things follow, and the third is the one that decides it for me.

**It contradicts the design's own droplist.** "A documented traversal order substituting for a law:
associativity is about grouping, not order" (`40:711`). Under the short circuit the grade *is* a
documented traversal order, promoted to something a consumer reads and acts on. The design has already
ruled that order is not a thing a law may rest on; the grade is the law's own content
(`40:265-267`), so it may not rest on order either.

**The stack reorders on purpose.** hilavitkutin's RCM renumbering is a permutation of a column's
traversal order, chosen for locality and nothing else. Under the short circuit, turning on a
cache-layout optimisation changes what a consumer's diagnostic says about unchanged data. There is no
sentence anyone can write that makes that acceptable to a person debugging a pipeline.

**The diagnostic degrades exactly when it is needed.** Same term shape, three inputs, increasingly
bad, asserted:

| input | short circuit, events | strict, events |
|---|---|---|
| nothing refuses | 2 | 2 |
| one channel refuses, late in the order | 1 | 2 |
| it refuses early | 0 | 2 |

The consumer's code that reads this is four lines and it is the entire reason a grade is published at
all:

```rust
// Read off a refused sample: which channels are drifting toward their limits?
pub const fn needs_rescale(r: Report) -> bool { r.events > 0 }
```

Under strict, a refused sample still names the drifting channels and the consumer widens them. Under
the short circuit, the worst sample in the run reports nothing to rescale, and the consumer ships a
pipeline that quietly keeps refusing. Both asserted at the end of probe 4. **That is the answer to
"which one will they misuse without noticing":** the short circuit, and they will not notice because
the mechanism that would have told them is the one that went quiet.

#### Two facts this review already had, pointing the same way

I did not measure these and I am citing them rather than re-deriving them. File 05 found that a
short-circuiting refusal is not constant time and leaks data through timing (`05:392-403`), and file
11 measured it: "two data-dependent branch exits per element in the compiled code, against none for
the bottom delivery" (`11:543-546`). For the workload this design is for, a per-element data-dependent
branch in a column loop is not a small thing; it is the difference between a loop that vectorises and
one that does not. So strict is also the shape the target workload wants, which is a pleasant place to
land: the reading that makes the consumer's diagnostic reliable is the one that makes the inner loop
branchless.

And file 39's standards test already tilted here from outside, in three standards' own vocabulary
(IEEE's sticky flags, SystemC's per-variable flags, MATLAB's overflow logging are all strict-shaped),
recorded at `40:322-327`.

**Suggested, not ruled.** The cost of strict is that an operand whose sibling has already refused is
still computed, which is real work thrown away on the refusing path. For the column workload that is
the cheap direction (it is what the branchless form does anyway); for a scalar path with an expensive
operand it is not free, and a member who wants to argue that case has a real case to argue. What is
not available any more, I think, is the argument that the two readings are equally reasonable and the
design may pick either.

### 1.2 `Precise`'s combinator surface: both forms, as two names

**Recommended sentence, for the spec:**

> The regrouping fold and the sequential fold are two named combinators. The regrouping one publishes
> what its law does not preserve; the sequential one regroups nothing, so it publishes nothing and is
> faithful by construction. The caller's own type decides which of the two typechecks, and the
> sequential one is named for what it costs rather than for what it delivers.

The open item asks whether the surface offers only the definedness-faithful form, or offers the
published-grade form as well and lets the caller's type decide (`40:632-637`). Written out as consumer
code in `47_probes/probe_5_precise_surface.rs`, there are three candidate shapes and one of them does
not exist.

**Shape C is not available, and this is compiled rather than asserted.** The shape that would keep both
doors and keep the common path free of ceremony is one combinator with the grouping as a defaulted
type parameter, so `fold(xs)` regroups and `fold::<Sequential>(xs)` does not. Rust refuses it, for a
free function and for an inherent method both (`probe_5b`, `error: defaults for generic parameters are
not allowed here`, future-incompatible, issue #36887). So the ergonomic argument for a single name
cannot be made that way, and the real choice is between one combinator and two.

**Shape A, one combinator, leaves a consumer with no program.** A consumer whose contract is
`Folded<Faithful>` and whose accumulator is not interior-safe has exactly one remedy: widen the
accumulator numeral. That is usually right and it is what the design should push people toward, since
`Warm` and `Cold` go from having no law at any view to having every one purely by widening
(`40:353-356`). It is not always available: interior safety for a three-million-element fold wants
twenty-two digits of headroom, which is fine over a fifteen-digit sample and crosses into multi-limb
over a sixty-four-digit one. When it is not available, the consumer reaches for the sequential
combinator and finds `error[E0425]: cannot find function fold_sequential in module mechanism`
(`probe_5c`, committed refusing). Their program is unwritable, and the tool has no opinion to offer,
which is the posture `arvo-toolbox-not-policer.md` names as the thing arvo does not do: "refusals to
expose a primitive because we think the consumer is misusing it".

**Shape B is the design's own idiom, applied a second time.** `40:199-201` already settled this
question in a neighbouring case: "The one genuinely shaped fold, compensated summation, gets its own
named combinator (`fold_compensated`) sitting beside `fold`. Two function names with different bounds
is a structural refusal that costs nothing new." A sequential fold sitting beside a regrouping one is
the same shape for the same reason, and adopting it here is consistency rather than a new mechanism.

**One compiled fact that decides who has to ship it.** `Folded`'s grade marker field is private, which
is correct under `what-you-can-observe-is-what-you-guaranteed.md`: a consumer that could construct
`Folded<Faithful>` from outside could mint any guarantee it liked. The consequence is that the
sequential combinator's constructor must live inside the perimeter, and a consumer cannot add the door
themselves (`probe_5`, which failed with E0423 until the constructor moved into the mechanism, and the
comment recording why is at `probe_3_the_grade_is_projected.rs`'s `Folded<Faithful>::sequential`).
**Shape B is something arvo ships or something nobody has.** That is the whole content of "letting the
caller's type decide": the caller's type decides which door they walk through, and only arvo can build
the doors.

**Name it for the price, not the property.** `fold_sequential`, not `fold_faithful`. The faithfulness
is already in the return type, where the compiler enforces it and no name is needed; the cost, which
is the loss of regrouping and therefore of vectorisation and of morsel splitting, appears nowhere else
at all. A reviewer scanning a diff should stop on this call site, and they will stop on
`fold_sequential` and not on `fold_faithful`.

**One honest cost, and section 3.3 is what I would do about it.** With two doors, a consumer who takes
the wrong one gets `error[E0308]: expected Folded<Faithful>, found Folded<RefusalsTransferred>`
(`probe_3c`), which is correct and tells them nothing about either remedy. E0308 has no customisation
surface. That is a real gap and it is fixable.

---

## 2. What writing the consumer's code found

This is the part I think is worth more than section 1.

### 2.1 A consumer cannot write a number

The ratified encoding is `Pos ::= H | O<P> | I<P>`, positional binary, `O<p>` doubling and `I<p>`
doubling-and-adding-one, terminated by the leading one. Its uniqueness is settled, its seal is
settled, its price is measured, and none of that is in question here. What is in question is that
there is no other way to name a number, and a consumer names numbers constantly: a precision, a
quantum, an exponent bound, an arity, an accumulator width.

`47_probes/probe_1_the_declaration_as_it_stands.rs` declares the numerals for the column workload,
verbatim, every value const-asserted so the file is checked rather than illustrative:

```rust
pub type SamplePrecision = Pz<I<I<I<H>>>>;                              // 15
pub type Pow2_15 = O<O<O<O<O<O<O<O<O<O<O<O<O<O<O<H>>>>>>>>>>>>>>>;      // the Q0.15 quantum's denominator
pub type AccumPrecision = Pz<I<O<I<O<O<H>>>>>>;                         // 37
pub type Binary64Precision = Pz<I<O<I<O<I<H>>>>>>;                      // 53
```

Four declarations. Each one required me to decompose a decimal number into binary by hand and write
the digits inside out, and I got two of them wrong on the first pass; the const assertions caught me,
which is exactly why they are in the probe. IEEE binary64's precision is six constructors deep. A
Q0.15 quantum is fifteen.

**And a mistyped numeral is a well-formed numeral.** `probe_1b` compiles clean, which is the finding:

```rust
pub type Intended = Pz<I<O<I<O<O<H>>>>>>;  // 37, the accumulator for a 3M-element fold
pub type Typo     = Pz<I<O<I<O<I<H>>>>>>;  // 53, one constructor changed
pub type Dropped  = Pz<I<O<I<O<H>>>>>;     // 21, one constructor dropped
```

All three are legal `Nat`s, all three are sealed, all three are unique, all three reach every
`Nat`-bounded position, all three const-asserted at those values. Value-uniqueness guarantees one type
per value. It cannot guarantee that the type a person typed is the value they meant, and the encoding
maximises the gap between those two things, because the edit distance between 37 and 21 is one
character in the middle of a nest six deep and an editor's bracket matching invites exactly that edit.

**And when it does surface, the consumer has to decode binary to read the message.** `probe_1c`,
verbatim:

```
error[E0308]: mismatched types
   = note: expected struct `Accumulator<Pz<I<O<I<O<O<H>>>>>>>`
              found struct `Accumulator<Pz<I<O<I<O<I<H>>>>>>>`
```

Two lines differing by one letter, forty characters in. Nothing in that message says 37 or 53. That is
the answer to "which one produces a comprehensible error when they get it wrong": as it stands, none
of them, because the error is about the encoding and the consumer is thinking about numbers.

I want to be exact about the blame here, because the encoding is not at fault. The encoding is doing a
job (uniqueness by construction, arithmetic by impl selection, no normalisation pass anywhere) that
nothing cheaper does, and file 36 measured that it does it 3.06x faster than typenum. The defect is
that **the design stopped at the representation and never wrote the notation**. Every serious
type-level numeric library in this language ships one: typenum has `consts::U37`, and it ships it for
precisely this reason. Section 3.1 is the smallest thing that closes it.

### 2.2 The published grade is the caller's homework

File 37's transfer rule is right, and the mechanism it is carried in has the caller do arithmetic the
library already did. This is what a consumer types today
(`37_probes/probe_4_view_as_a_return_type_and_the_transfer.rs:344`):

```rust
pub const PRECISE_BELOW: Folded<1> = regroup_fold::<0, 0, 1, 4, 0, 1>([1, 2, 3, 4]);
```

Six positional const parameters, no names, all integers. `0` in the first two positions is `Refuse`;
`1` in the third is `Signed`; `4` is the arity; `0` is the headroom, which the caller computed on
paper; and the last `1` is a bitmask of the grade generator classes this composition fails to
preserve, which the caller worked out by running the design's own law in their head. The const
assertion checks it, so an understating caller is refused, which is the mechanism working. File 37
states plainly why it is shaped this way (`37:452-456`): computing the grade in return position is an
expression over a generic const parameter in type position, and that is the `generic_const_exprs`
wall.

Three things about this from the consumer's side, in increasing order of how much they matter.

**Nobody can read a call site.** `regroup_fold::<0, 0, 1, 4, 0, 1>` is six magic numbers. A reviewer
cannot tell a correct one from a wrong one without opening the mechanism, and neither can the author
six months later.

**The caller has to know the answer to ask the question.** `PUBLISHED` is what `add_assoc_view` already
computes. The library knows it; the caller writes it down; the library checks the caller against
itself. That is the shape that always rots, because the two copies are maintained by different people
at different times.

**And here is the misuse nobody will notice.** Understating refuses, correctly. Overstating compiles
and is merely pessimistic (`37:456-458`, and it is the safe direction, correctly). So a caller who
cannot work out the right bitmask writes `3` and moves on. Now their fold's type says both classes may
disagree, their downstream signature refuses it, and the fix in front of them is to widen the
downstream signature to `Folded<3>` too. Nothing is unsound at any step, every step is locally
reasonable, and at the end of it the consumer has a codebase where every fold publishes everything and
the type therefore says nothing. **The transfer rule was designed to remove the waiver, and a
caller-declared grade puts the waiver back at the declaration site with worse ergonomics.** Section
3.2 removes it.

### 2.3 The three questions, answered

The dispatch asks which candidate a person can hold in their head, which produces a comprehensible
error, and which they will misuse without noticing. Consolidated:

| | hold in their head | comprehensible error | misused without noticing |
|---|---|---|---|
| strict evaluation | yes: the grade is the term's own leaf multiset | not applicable, no error | no |
| short-circuit evaluation | no: it is the prefix before the first refusal in a traversal order they do not control | not applicable | **yes**, silently, and worst on the worst data |
| `Precise`, one combinator | yes | E0425, name not found, no remedy offered | no, they are simply stuck |
| `Precise`, two combinators | yes | E0308, correct and remedy-free, fixable per 3.3 | no |
| numeral written as encoding | **no** | **no**, the message is in binary | **yes**, a wrong digit is silent |
| grade declared by the caller | no, six unnamed integers | yes when understated, nothing when overstated | **yes**, by overstating until everything typechecks |
| grade projected (3.2) | yes, four parameters they already have | yes, designed, remedies named | no route found |

---

## 3. Three proposals, with their costs

Each is compiled. Each is stated as a suggestion; only op's calls are final.

### 3.1 Ship the notation, not only the representation

**The proposal.** A generated table of aliases naming every `Nat` in a useful range, plus a one-rule
macro so a consumer writes the literal:

```rust
pub mod n {
    pub type N0  = Z;
    pub type N1  = Pz<H>;
    // ...
    pub type N37 = Pz<I<O<I<O<O<H>>>>>>;
}

macro_rules! nat {
    ($v:literal) => { $crate::n::${concat(N, $v)} };
}
```

`47_probes/probe_2_writing_a_number.rs`, compiled: `nat!(37)`, `n::N37` and `Pz<I<O<I<O<O<H>>>>>>` are
**the same type**, proved by passing all three to one function that admits exactly one type. Value
uniqueness is untouched, because these are spellings and not numerals: nothing new inhabits `Nat`.

The macro needs `macro_metavar_expr_concat`, which arvo already enables (`crates/arvo/src/lib.rs:26`),
and it resolves **by name**, not by type-level arithmetic, so it costs the trait solver nothing. That
is the point of routing through a table rather than building the number with the tower's own
`Dbl`/`DblInc`: a digit-munching macro would put a projection chain in every type, and projections
are what show up in diagnostics.

**Price** (`47_probes/price/`, `--emit=metadata`, min of three, counts 0 and 400, difference quotient,
scope stated in `OUTCOMES.md`): a 400-row table with no per-row check costs **no measurable compile
time** (one run came back negative, which is the honest way to say it is under the noise floor) and
**165 bytes of metadata per row**. With a const assertion per row, which is what makes the table
trustworthy and is what I would ship, 0.083 ms and 668 bytes per row. A 1024-row table is therefore
roughly 85 ms and 680 KB of metadata in the crate that declares it, paid once, and the aliases
themselves emit no symbols.

**What it does not fix, stated plainly.** rustc expands type aliases in diagnostics, so `probe_1c`'s
message is unchanged by this proposal: the consumer writes `nat!(37)` and still reads
`Pz<I<O<I<O<O<H>>>>>>` when it goes wrong. I looked for a way to get the decimal value into the message
and did not find one that keeps uniqueness (a distinct struct per number breaks the arithmetic; a
const assertion carries no formatting in const eval). The partial mitigation available is 3.3's shape:
where a numeral mismatch surfaces through a **bound** rather than a type equality, an
`on_unimplemented` note can carry the decoder ring (`H` is 1, `O<p>` is 2p, `I<p>` is 2p+1, outermost
first). I have not built that one and it is the weakest of the three; I flag it rather than propose it.

**Rewrite cost: zero.** Nothing exists to change. The table is generated by the same machinery that
already emits arvo's per-width impls, and the range is a judgment call (I would start at 0 through
1024, which covers every precision, width and exponent bound in every format the standards test
names).

### 3.2 Project the published grade instead of declaring it

**The proposal.** Make the grade a type rather than a const parameter. Then the combinator's return
type is an ordinary associated-type projection, the caller declares nothing, and the
`generic_const_exprs` wall is not near the problem.

```rust
pub const fn regroup_fold<Top, Bot, Dom, Hd, Am1>(xs: &[i32])
    -> Folded<<(<Hd as InteriorSafety<Am1>>::Out, Top, Bot, Dom) as FoldGrade>::Out>
```

`47_probes/probe_3_the_grade_is_projected.rs`, compiled clean, **no unstable feature at all**. File
37's `add_assoc_view` body transcribes into impl selection one arm at a time; the `Never` arm becomes
the absence of an impl. The five call sites file 37 measures become:

```rust
pub fn precise_below(xs: &[i32]) -> Folded<RefusalsTransferred> {
    regroup_fold::<Refuse, Refuse, Signed, NoHeadroom, Arity4Minus1>(xs)
}
```

Five parameters, every one a fact the consumer already has, named. `probe_3c` is the negative control
proving the annotation is checked rather than inferred: annotate that same call `Folded<Faithful>` and
it is E0308.

Four things this buys, three of them compiled.

**The caller declares nothing, so overstating stops being accidental.** It stays available, because
overstating is sound and sometimes wanted, but as an explicit `.weaken::<BothTransferred>()` bounded on
the lattice order, which is one grep away from a reviewer instead of one digit away from a typo.

**Interior safety is computed rather than passed.** `HEADROOM` was an integer the caller worked out on
paper. It becomes a comparison between two numerals the caller already declared, through the tower's
own `Cmp` (`47_probes/vu_nat.rs:153`), which is machinery file 36 built for the gcd and which costs
nothing new here.

**The grade gets names.** `Folded<RefusalsTransferred>` against `Folded<1>`. Every diagnostic in this
file's probe set reads better for it, and the consumer's own signature (`fn alarm(f:
Folded<Faithful>)`) states its contract in words.

**The composition with no law becomes a designed diagnostic instead of a const-eval panic**, and it
fires during type checking rather than const evaluation. Verbatim, `probe_3b`:

```
error[E0277]: this composition's fold has no associativity law at any view
    |                                                  ^ regrouping this fold changes the delivered value
    = note: no published grade makes the regrouping honest, because the values themselves diverge
    = note: widen the accumulator until the fold is interior-safe, or do not regroup
    = help: the following other types implement trait `FoldGrade`:
              (Safe, Top, Bot, Dom)
              (Unsafe, ReduceModulo, ReduceModulo, Signed)
              ...
```

The `help` enumerating every composition that does have a law is free and I did not ask for it.

**Price**, same harness, both kinds emitting const items so the comparison is matched: metadata is
**561.7 bytes per call site projected against 1493.7 declared**, a 2.66x reduction, and those byte
counts were byte-identical across runs. **Compile time does not separate the two** at this count on
this harness: across two sweeps the declared form came back at 0.139 and 0.100 ms per site and the
projected at 0.127 and 0.132, so the sign of the gap is not stable and I make no claim about it. The
ergonomic argument is the argument; the metadata is a bonus.

**Rewrite cost.** File 37's mechanism is unshipped probe material, so this is a rewrite of one probe
file, not of source. What it changes in the ratified statement is nothing: the transfer rule
(`40:300-303`) is quoted into the new shape unaltered, including "tolerance is a transfer, never a
waiver", which the projection enforces more strongly than the declaration did. What it changes in the
consolidation's prose is one clause: `40:305` says a caller "takes `Folded<0>`" and a `Precise`
regrouping "delivers `Folded<1>`", which becomes `Folded<Faithful>` and `Folded<RefusalsTransferred>`.
And it retires one honest limit file 37 recorded (`37:452-456`, the published grade is declared
because it cannot be computed): it can be computed, in the type system rather than in const eval.

I want to name what this generalises, because I think it is the more useful half. Op's tenth
checkpoint ratified moving every number in this design out of const-land into the value-unique
type-level encoding. **The grade is the one quantity that was left behind**, and every awkwardness in
file 37's call site is a symptom of that rather than of the mechanism. The rule worth carrying
forward: in this design, a quantity that has to be computed and then appear in a type is a type, and
a quantity that only has to be read is a const. `generic_const_exprs` is the wall you hit for putting
the first kind in the second place.

### 3.3 State the caller's contract as a bound, so the error can carry the remedy

**The proposal.** Where a consumer's own correctness depends on a fold's grade, they state it as a
bound rather than an exact type, and arvo attaches the remedies to the bound.

```rust
#[diagnostic::on_unimplemented(
    message = "this fold's definedness does not match the sequential fold's",
    label = "published grade `{Self}`",
    note = "this combinator may refuse where a sequential fold returned, or return where it refused",
    note = "to get a faithful fold: widen the accumulator numeral until the fold is interior-safe, \
            or call `fold_sequential`, which does not regroup and pays for it"
)]
pub trait Definite: Grade {}
impl Definite for Faithful {}
```

`47_probes/probe_6_the_caller_contract_diagnostic.rs`, compiled. The consumer writes `fn
alarm_threshold<G: Definite>(f: Folded<G>)`, one generic parameter more than `Folded<Faithful>`, and
in exchange the mismatch becomes:

```
error[E0277]: this fold's definedness does not match the sequential fold's
   |     --------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ published grade `RefusalsTransferred`
   = note: this combinator may refuse where a sequential fold returned, or return where it refused
   = note: to get a faithful fold: widen the accumulator numeral until the fold is interior-safe,
           or call `fold_sequential`, which does not regroup and pays for it
help: the trait `Definite` is implemented for `Faithful`
```

That message names the grade, says what it means in the consumer's own terms, and names both remedies
including the one section 1.2 recommends shipping. The bare E0308 says none of it and cannot be made
to.

**Rewrite cost: one trait and one impl**, and a convention in the docs. Nothing else moves, and a
consumer who prefers `Folded<Faithful>` keeps writing it.

This is the same move as `arvo-toolbox-not-policer.md`'s "diagnostic, not directive" and the same move
`.claude/rules/arvo-always-optimal-internals.md` records as already shipped for strategy-and-width
projection (`#[diagnostic::on_unimplemented]` errors that point the consumer at the right
alternative). It is arvo's existing house style, applied to the one surface where the design has
genuinely useful advice to give and currently gives none.

---

## 4. The two second reads

Kept short and out of the way, as instructed. Both were run before section 1, reading the probe source
and forming a reading before reading either file's conclusion. Full commands and outcomes in
`47_probes/OUTCOMES.md`.

**File 46, `probe_1b` and `probe_3`: reproduced exactly, second read given.**

`probe_1b` compiles clean (rc=0) against file 42's tower, which is the defect file 46 reports. My
independent read of why, formed from the source before reading file 46's table: `Adjustment` is
declared at `46_probes/vu_nat_sealed.rs:448` as `pub trait Adjustment` with no supertrait, so the
`Pos`/`Nat` seal is simply not on this route. A downstream crate implements it directly on a local
type (orphan-legal, local type), fabricates `NUM = 6, DEN = 12`, and reaches a fn-forced `A:
Adjustment` position. Both halves of the defect land: the pair is unreduced, so validity fails, and it
is a second type denoting the value `Reduced<H, O<H>>` already names, so value-uniqueness fails.

`probe_3` fails with four E0277s, one per sealed trait, including `EvilAdj: AdjustmentSealed`, which
is the refusal `probe_1b` shows was missing. The error heads match file 46's report exactly.

**I agree with file 46's finding and its fix**, and this is the second independent read the review's
own discipline requires before it is built on. Two notes I would add, neither of which changes the
conclusion. The seal's diagnostic is unusually good here (rustc's own "`Pos` is a sealed trait,
because to implement it you also need to implement ... which is not accessible" note fires
automatically), and that is worth knowing because it is one of the few places in this design where the
mechanism explains itself to a consumer without anyone writing the explanation. And file 46's own
`probe_3d` lesson deserves louder billing than a table cell: its first draft used bare type aliases,
compiled clean, and tested nothing, because a type alias defers its bound checks. **Any compile-fail
suite shipped for this perimeter must force through a fn signature or it will be green while asserting
nothing.** That is a tautological-test trap sitting in wait for whoever implements the owed seal, and
it should be a sentence in the implementation task rather than a footnote in a probe table.

**File 45, finding 4.1, `Int` has an empty grounding set: first read, agreeing.**

The ratified table lists `Int ::= Z0 | Zpos<P> | Zneg<P>` at `40:488` with the comment "biases,
corrected to a normalised rational (39)". I checked what actually consumes it rather than what the
comment says: `grep -rln "Zpos\|Zneg\|\bZ0\b"` across `36_probes`, `41_probes`, `42_probes` and
`46_probes` hits `36_probes` only. The towers everything since file 41 composes with build `Bias` as
`BZero | BPos<N, D> | BNeg<N, D>` over `Pos` pairs with the sign carried by the constructor
(`41:101-131`), which is a different construction that does not use `Int` at all. Precision, widths
and exponent bounds are `Nat`. So the tier is unconsumed, and the comment ratifying it cites the very
correction that removed its only consumer.

**This is a first read only and a second is owed before either resolution is taken.** File 45 offers
two and leans to the drop; I lean the same way and for a reason it does not state, which is a consumer
reason and therefore mine to add: a sealed tier that nothing inhabits at a bounded position is still
in the crate's public surface, still in its rustdoc, and still something a consumer reading the
identity contract has to work out the purpose of and fails to. An unconsumed export is a question in
every reader's head forever. If it is kept, it should be kept where the exponent fork can find it and
not where a consumer trying to declare a numeral trips over it.

---

## 5. What I leave open

**The decoder-ring diagnostic** for numeral mismatches (3.1's stated non-fix) is unbuilt. I found no
way to put a decimal value into an E0308, and the `on_unimplemented` route only works where the
mismatch is expressible as a bound. Someone who knows rustc's diagnostic surface better than I do
should spend twenty minutes on it before the design accepts that the message stays in binary.

**The alias table's range** is a judgment call I did not make. 0 through 1024 covers every format the
standards test names; whether it should also cover the powers of two a consumer needs for quantum
denominators past that (`Pow2_15` is fifteen constructors and a Q0.31 quantum is thirty-one) is a
separate question with a different answer, and a `pow2!(31)` alias family may be the better shape for
that half.

**Whether the sequential fold belongs on the same trait as the regrouping one** I did not test. Probe 5
puts them side by side as free functions because that is the shape `40:199-201` already blesses; a
member designing the actual combinator surface should check whether they want to be methods on a
column type, which changes the discoverability argument and might change the naming one.

**The consumer-side compile cost of the whole assembly** remains where file 37 and file 36 both left
it (`40:657-660`): every sweep in this file prices one mechanism in isolation and is a neighbour to
that question, never an answer for it. My alias-table and projection figures are two more neighbours.

**And one thing I did not do that this dispatch's own framing invites.** I wrote the consumer's code
for the fold and for the declaration, which is where the two open sentences live. I did not write the
consumer's code for the crossing contract, the quantiser's dithered entry point, or the membership
predicate, all three of which have consumer-facing surfaces nobody has typed either. If the finding in
section 2 is worth anything, it is worth repeating at those three, and I would expect it to find
something at each, because the same cause is available at each: a mechanism settled from the inside,
correct, and never once written down from the outside.
