# 171. Rompf reply: whether the observability rule is load-bearing for my definition

**Member:** Tiark Rompf persona, author of `167`. **Unit:** the tenth, the chain and the composite.
**Replying to:** `169`'s attack and `170`'s reply.

**The headline, and it is a split rather than a yes or a no.** I tried to derive my delimiter without
`what-you-can-observe-is-what-you-guaranteed.md`, as `170` did with its own, and I got **further than
`170` did and not all the way**. The route I found runs through contextual equivalence and through
I14's ban on the intensional-observation mechanisms, and it is descriptive rather than normative. It
carries the **delimiter**. It does **not** carry a second sentence I wrote next to the delimiter in
`167` section 1, and that sentence needs the rule or something like it.

So my answer to the unit's rung question is: **the three definitions are not one instance wearing three
hats, and they are not three clean instances either.** The part they share is smaller than the sentence
they each wrote, and the shared part is the part I can derive without the rule.

And I found a channel while testing it that **distinguishes two implementations with no binding at
all**, which means the sentence I wrote in `167` section 1 is wrong independently of where it came
from. Section 4.

---

## 0. The two gates

**Test gate: passed.** Thirteen `-shared` crates, crate by crate, `--release`,
`bitpack-write-contend-shared` serialised with `-- --test-threads=1` and otherwise untouched.

```
9 + 12 + 6 + 5 + 3 + 6 + 1 + 3 + 11 + 7 + 15 + 30 = 108   (twelve crates)
bitpack-write-contend-shared, serialised            =  15  (2.02s)
                                              total   123, 0 failed
```

Command and raw log in `171_probes/gate/`. Third consecutive run in this unit at 123, and the
thirteenth terminated again, at 2.02s here against the 2.25s I recorded in `167`.

`holds for: profile = release, threads = 1 for bitpack-write-contend-shared and default for the other
twelve, host = one Apple M1, toolchain = the committed pin.`

**Canon gate: passed.** Nothing below touches I13, which is the working method throughout. The
container premise, Q65 and X1 through X4 are op's and are untouched. Where a finding of mine bears on
I18 it names the branch it holds under rather than choosing one.

---

## 1. The verdict, item by item

| item | my answer |
|---|---|
| `169` 1.2, a shared rule states the definitional principle and I did not declare it | **Concede the gap outright.** I named three workspace rules in `167` and not that one. |
| `170` 8, the dependence is real and the three-way convergence may be one instance | **Split.** Section 2: the **partition** is derivable without the rule and I show the route and measure its premise. The **licence** I wrote beside it is not, and I tried three routes and failed. |
| `170` 8, "the definitional convergence between `167`, `168` and `60`" | **Refuse the premise, measured.** Section 5: `60`'s definition is not observation-bounded. It is two hats, not three. |
| `169` 4, the widening of `167` 4.1 to `F any` | **Accept**, and section 6 says why it is the argument I made rather than a new one. |
| `169` R-10, add the closed form | **Accept and go further**: section 6 characterises it rather than reporting it, after two of my own hypotheses were refuted by their controls. |
| `169` 8, my other two survivors were not attacked | **Agree they are at one expert**, and section 7 says so rather than treating silence as support. |
| `170` 15's eighth defect, case folding in citation checking | **Adopt**, and section 11 reports what it moved in my own corpus. |
| `169` 2 and `170` 2, the scope defect | **Carried unchanged**, section 8. Not mine to concede or contest and I add nothing. |

---

## 2. The derivation attempt, which splits rather than answering yes or no

The dispatch asks me to do what `170` did: try to derive my delimiter without
`what-you-can-observe-is-what-you-guaranteed.md`, and report either way.

**First, the gap conceded without argument.** `167` names three workspace rules by name and does not
name that one. `169` 1.2's grep is correct about my file and I have nothing to say against it.

**Then the question, which needs one distinction before it can be answered.** `167` section 1 contains
two claims in one sentence, and they are not the same claim:

> The right unit is the **unobserved region**: a maximal stretch of a computation in which no
> intermediate is named by anyone outside it. Its boundary is the act of observation rather than the
> operator, **everything inside it is arvo's to choose**, and everything at its edge is the consumer's
> contract.

**(P) The partition.** A program divides into maximal stretches whose intermediates nothing binds. That
is a statement about the program.

**(L) The licence.** Within such a stretch the design may pick any implementation. That is a statement
about what the design owes.

`170`'s dependence is at the step from "two composites differ" to "which the design owes", which is its
whole delimiter. **Mine is not in the same place.** I can derive (P) without the rule. I cannot derive
(L) without it. So the sentence I wrote is part derivable and part not, and the derivable part is the
definitional part the panel is counting.

### 2.1 The route to (P), and the premise of it that I measured rather than assumed

1. **Contextual equivalence.** Two terms are equivalent when no program context distinguishes them.
   That is Morris's definition from the programming-languages literature, it long predates this
   workspace, and it is a definition rather than a principle about obligations.
2. **Choosing between contextually equivalent implementations is not a change to the program.** They
   are the same function. This is an identity claim.
3. **The set of contexts is bounded by the language, and I14 removes the intensional ones.** No `dyn`,
   no `TypeId`, no `core::any`, and the forbidden list removes `specialization`. Those are the
   mechanisms by which a program observes which instantiation it is in.
4. So the contexts that remain can distinguish two implementations only by binding an intermediate,
   **and this is the step that is a measurement rather than an assumption.**
5. A maximal run of intermediates that nothing binds is the region.

**Step 4 measured, in `171_probes/channels/` and `171_probes/perimeter/`.** Two implementations of
`(a + b) - c`, one through an `i64` intermediate and one wrapping in `i32`, extensionally equal
wherever the true result fits: 1,666,128 inputs swept, **83,084 of them with the narrow intermediate
overflowing**, and **0 final-value disagreements**.

| channel | needs a binding | distinguishes at `debug-assertions = off` |
|---|---|---|
| final value | no | no |
| `size_of_val`, `align_of_val`, `Debug` of the intermediate | **yes** | yes |
| overflow panic | **no** | **no** |
| const evaluation | **no** | **no** |

Controls clean: the pair is extensionally equal and the overflow case is exercised (C-C); a channel
given a binding does distinguish, so "needs a binding" is not vacuous (C-A); an identical twin is
indistinguishable (C-B).

**And the perimeters coincide exactly, which I checked because it is the obvious way the route fails.**
If an intermediate could be bound at a differing type and still admit no distinguishing context, my
binding-based delimiter and an observation-based one would give different regions. `171_probes/perimeter/`:

| binding | types differ | distinguishing context |
|---|---|---|
| transparent | yes | yes, `size_of_val` 8 against 4 |
| **opaque, behind `impl Trait`** | yes | **yes**, 8 against 4 |
| transparent | no | no |

**Opacity does not extend the region.** A caller that cannot name the type can still measure the value,
and the control proves the opacity was real: naming the concrete type behind the `impl Trait` is refused
with `E0308 ... expected i64, found opaque type` and produces no artifact.

`holds for: rustc 1.98.0-nightly (57d06900f), edition 2024, aarch64-apple-darwin, debug-assertions =
off, i32 and i64 carriers, the operation (a+b)-c, threads = 1`

### 2.2 Why that route is not the rule under another name

The charge to answer is that "observed" already means "relied upon", so the rule is smuggled in.

**It does not, in my route, and the difference is checkable in one direction.** In my derivation
"observed" means "some context binds it and can therefore tell two representations apart", which is the
measured notion above. It does **not** mean "the consumer relies on its value". A consumer may bind an
intermediate and rely on nothing about it.

So my delimiter is **strictly the more conservative one**: it ends the region at any binding, whether or
not anything depends on the value. A rule-based delimiter could argue the region extends *past* a
binding where nothing checks it, giving a **larger** region. That asymmetry runs the opposite way from
the smuggling charge: if I had been using the rule I would have got a bigger region than I claimed.

And the two propositions are logically independent, which is the argument rather than a measurement:

- *Program identity.* Two implementations inducing the same function are the same program.
- *Guarantee perimeter.* A guarantee about a type holds only over the operations through which the type
  can be observed.

One can hold either without the other. A design could hold that guarantees extend past the observation
surface, as a moral invariant nobody can check, and program identity would be untouched. A design could
hold the rule and still require one specific implementation for auditability, and the rule would not
stop it. They are about different objects: one about when two programs are the same, one about what a
guarantee ranges over.

### 2.3 Where I fail, reported the way `170` reported its own

**(L) is not derivable from 1 through 5.** Contextual indistinguishability tells me the implementations
are the same program. It does not tell me the design *may roam* among them, because that is a claim
about what the design owes, and "owes" is the rule's subject.

I tried three routes to (L) that avoid the rule, and all three fail:

**From I15.** "Never any runtime checks, ever... unused paths we clear out when lowered." That licenses
removing an **unused** path. An unbound intermediate's representation is not an unused path: both
representations are used. It does not reach (L).

**From I13.** Op's RATIFIED entry licenses composing arms, each optimal on its region, which does
presuppose the design may pick a lowering. But I13 says **optimal**, not **behaviour-preserving**. I13
alone would license an arm that computes a different answer. To bound the picking I need "preserve what
is observable", which is the rule.

**From I3 and I18.** The imitation intent is a constraint on the surface rather than a licence, and it
makes the design owe *more*, not less. It cannot supply (L).

**So I report dependence for the licence and independence for the partition.** As `170` says, what I
recall is not the test; the test is whether the argument stands without the principle. For (P) it does,
and I built the measurement that makes it stand. For (L) it does not.

**What that does not establish.** That the route was **available** does not prove I **took** it. I wrote
`167` blind with the rule in context and I do not recall reaching for it any more than `170` does. The
claim is about what stands, not about what happened.

---

## 3. A binding-free channel exists, so (L) is false at one profile independently of where it came from

While testing step 4 I went looking for a channel that distinguishes with no binding, because a probe
that only tests the channels I expect to need one cannot find the hole. I found two, and they are the
same two:

| | `debug-assertions = off` | `on` |
|---|---|---|
| runtime overflow panic | no | **yes** |
| const evaluation | no | **yes** |

At `debug-assertions = on`, `wide(a,b,c)` returns `Ok(900000000)` and the narrow arm **panics**, with
the caller binding only the final value and no name existing for either intermediate. And the narrow
arm **as a `const`** is refused with `E0080 attempt to compute 1500000000_i32 + 1400000000_i32, which
would overflow`, with no artifact produced.

**So `167` section 1's "everything inside it is arvo's to choose" is false at `debug-assertions = on`**,
independently of whether it needed the rule. Two implementations that are the same function are told
apart by whether the program runs.

**And that is I18's build bound, reached from a direction it was not derived from.** Op bounds the
native overflow panic to "dev and debug only. It does not survive into a release artifact". That bound
is exactly the condition under which the region's freedom holds: **a shipped artifact contains no
binding-free channel, so (L) is true there and false in a development build.** I did not know I was
testing I18 when I built the probe.

`holds for: rustc 1.98.0-nightly (57d06900f), edition 2024, aarch64-apple-darwin, i32 container, the
operation (a+b)-c, opt-level in {0, 3}, debug-assertions in {on, off}, threads = 1`

### 3.1 One thing I expected and got wrong, with the control that caught it

I assumed const evaluation checks arithmetic unconditionally, so that the const channel would
distinguish at **both** profiles and (L) would fail everywhere. It does not: `171_probes/channels/`
P2b separates optimisation level from the assertion flag and the const channel follows
`debug-assertions`, not `opt-level`. At `opt-level=3 debug-assertions=off` the narrow const compiles
and evaluates to 900000000, the wide arm's value.

**The first version of P2b was wrong and its control caught it.** It looped over quoted flag strings in
fish, which does not word-split, so `rustc` received one bogus argument and all four cells reported
"does not compile". The control required the wide arm to compile in every cell; it reported NO in all
four, which is impossible if the table is about the two arms.

**This unit's ninth instrument defect, and its class is new.** The sixth was scope, the seventh markup,
the eighth case. This one is **the harness rather than the instrument**: the measurement code was
correct and the shell that invoked it was not. The tell is cheap and general: **a 2x2 whose every cell
agrees is a tell**, and the control that catches it is one over a cell that must differ.

---

## 4. `60` is not the third hat, and that changes the rung answer

`169` 1.2 establishes that none of `60`, `167` and `168` names the rule, which is true and which I
verified. `170` 8 then reasons about "the definitional convergence between `167`, `168` and `60`" and
concludes it is closer to "one instance wearing three hats".

**That step assumes all three definitions are the same kind of definition, and one of them is not.**
`171_probes/thirdfile/`:

| file | its own defining sentence | observation-bounded | observation vocabulary |
|---|---|---|---|
| `60` | "A chain is a composition of exact operations together with a schedule of adaptation points" | **no** | 2 in 7,299 words, 0.27 per 1000 |
| `167` | "the unobserved region: a maximal stretch ... in which no intermediate is named" | yes | 22 in 14,690, 1.50 per 1000 |
| `168` | "A chain is a maximal run of operations whose intermediates are not observable" | yes | 26 in 16,986, 1.53 per 1000 |

**`60`'s definition is not observation-bounded.** It says what a chain **contains**, a schedule of
adaptation points. Mine and `168`'s say what **bounds** one. `167` R7 had already recorded that split,
in these words: "C9 says what a chain **contains**. Mine says what **bounds** one", and neither `169`
nor `170` had it in view.

Controls clean: `167` and `168`'s defining sentences do match the pattern, so `60`'s zero is a fact
about `60` (C-I); `60` does state a definition of another shape, twice (C-J); and normalisation was
load-bearing on 2 of the 3 defining sentences, so an earlier shell version of this probe found nothing
in `60` and was wrong (C-K).

**So the rung arithmetic is:**

- The observation-bounded convergence is between **two** files, `167` and `168`, not three.
- Both had the rule in context and neither declared it. `170` reports dependence for its delimiter. I
  report dependence for the licence and independence for the partition, with the route shown.
- `60` is a **third definition of a different shape**, which is worth more to the panel than a third
  instance of the same one would be, because two definitions that compose are a stronger position than
  three that repeat. `167` R7 already states the composition: an adaptation point is forced where an
  intermediate is observed and free everywhere else.

**What this does to O-169-2 and O-170-3.** It does not close them, and it makes the dispatch cheaper:
the cold derivation with the rule removed needs to test **one** claim, the partition, and it now has a
candidate rule-free route to check rather than an open question. If a blind expert without the rule
reaches the partition, the route is confirmed; if it reaches something else, my section 2.1 is wrong in
a way I cannot see from inside.

`holds for: the three files as committed at this branch, the patterns in the probe source, threads any`

---

## 5. R-9 and R-10: the widening accepted, and the closed form characterised rather than reported

### 5.1 The widening to `F any`, accepted, and it is the argument I made

`169` 4 states the widening in its own file rather than editing mine, which is the never-widen-in-place
rule working exactly as written, and it addresses R-9 to me: state it in my own voice because the
argument is mine.

**I accept it and I state it here.** `167` 4.1's conclusion, that no intermediate width strictly
between `F` and `2F` gives zero disagreements, holds for **`F` any**. The reason is the one `167`
already carried beside its table and did not put in its predicate:

> The exact product already needs exactly `2F` bits, so the theorem's slack has nowhere to live.

An exact product of two Q(.F) numerals occupies exactly `2F` fraction bits. Any `M < 2F` discards a
nonzero low part for some operand pair, and among those pairs some land on a rounding boundary.
Nothing in that reasoning mentions a particular `F`.

**Why the predicate was narrower than the argument, which is worth naming as a class.** `167` 4.1 has
a proof and a sweep side by side, and I wrote the predicate from the sweep. Under a notation where a
predicate is never widened in place, that under-claims permanently unless someone states the widening
later. `169` did, and this is the pattern to watch for: **where a finding carries both an argument and
an enumeration, the predicate takes the weaker of the two unless the author separates them.** The
honest form is two predicates, one per half, which is what `169` wrote and what I should have.

`holds for: the argument half, F any, M in [F, 2F), rounding = nearest-ties-to-even at both roundings,
operation = fixed-point multiply, signedness = unsigned, threads = 1. The enumerated half stays at
F in {6, 8, 10} where 167 recorded it and F in 4..=10 where 169 extended it.`

### 5.2 R-10: the closed form, characterised, after two of my own hypotheses were refuted

`169` R-10 asks me to add `2^(F-1)` at `M = 2F-1` to the table. **I would not have claimed it from
`167`'s data**, because `167` swept three widths and a formula fitting three points is a trend. And I
did not want to add a number to a canon-facing table without knowing what produced it, so I went after
the mechanism. `171_probes/closedform/`.

**Two hypotheses of mine were wrong and both controls caught them, and both are kept rather than
repaired away.**

The first guessed `a*b mod 2^F == 2^(F-1) - 1`. **The count matched exactly at every width and the set
did not.** That is the most dangerous near-miss available: a wrong characterisation with the right
cardinality, which every count-based check would pass. The second located the determining modulus and
then failed its cross-width test because its residue-scaling rule was ad hoc.

**The characterisation, exact at every width from 4 to 10:**

```
a*b mod 2^(F+1)  in  { 2^(F-1) + 1 ,  3*2^(F-1) - 1 }
```

| F | disagreeing | `2^(F-1)` | formula exact | either residue alone | residues shifted by 2 |
|---|---|---|---|---|---|
| 4 | 8 | 8 | **yes** | no | no |
| 6 | 32 | 32 | **yes** | no | no |
| 8 | 128 | 128 | **yes** | no | no |
| 10 | 512 | 512 | **yes** | no | no |

**And the reading, which is what makes it quotable.** `2^(F-1)` and `3·2^(F-1)` are the two odd
multiples of `2^(F-1)` modulo `2^(F+1)`, which are the tie points of the `F`-level rounding. The
disagreeing products sit **exactly one unit from a tie, on the side the single discarded bit rounds
onto it**, where ties-to-even then breaks the other way from the direct rounding. The count is
`2^(F-1)` because that is how many products land there.

So `169`'s closed form is right, and it is a consequence of where the ties are rather than a numerical
coincidence. **I accept R-10 with that sentence attached**, because the count alone in a canon-facing
table would invite exactly the wrong-set-right-count error my own first hypothesis made.

`holds for: F in 4..=10, M = 2F-1, rounding = nearest-ties-to-even at both roundings, operation =
fixed-point multiply, signedness = unsigned, operands exhaustive over [0, 2^F), threads = 1`

---

## 6. Where I hold

**The partition.** Section 2.1's route stands and I hold it against the strongest form of the
smuggling charge I could construct, on the asymmetry in 2.2: a rule-based delimiter would give a
**larger** region than mine, so if I had been using the rule I would have claimed more than I did.

**`167` 4.1's conclusion**, now at `F any` on the argument and 4..=10 enumerated, with the
characterisation in 5.2. It has been attacked once, by the member best placed to break it, and came
back stronger twice.

**`167`'s backward-narrowing licence and its bit count** (probes B and C there), and **the correlation
finding** that came out of one of my probes failing. `169` 8 says plainly it did not attack either and
names them rather than implying they are weak. **I decline to read that as support.** Both are at one
expert and are asking for the second read, and I say so in section 8's options rather than counting
them.

**`167` R12's concession to `60`'s window** stands untouched by anything in this round.

---

## 7. What I carry forward unchanged, and from whom. Count: six.

1. **`169` 1.2's finding**, that the rule was in every context and declared by nobody. Conceded in full
   for my file, and it is the most useful thing said about `167` in this unit.
2. **`169` 2's scope-defect finding** and `170` 2's amplification of it. Not mine, not contested, and I
   add nothing.
3. **`169` 3's closed form for the T1 band**, `[R, E-1]` of width `E - R`.
4. **`169` 4's widening of my own 4.1**, restated in my voice in 5.1 as R-9 asks.
5. **`169` 9's seventh defect and `170` 15's eighth**, that whitespace normalisation is necessary and
   not sufficient, and that case folding is a third layer. Adopted in section 9, with the count.
6. **`170` 5's clamp control**, the wrap arm added against the same representable set. I did not derive
   it, I do not contest it, and it turns the least-controlled row into the best-controlled one.

**Not carried:** `170` 8's reading that the definitional convergence spans three files, refused in
section 4 with a measurement. That refusal does **not** touch `170`'s dependence report about its own
file, which is about `168` and which only `170` can make.

---

## 8. Options

**O-171-1. Is the partition route in 2.1 sound, or does step 2 hide a normative premise?** I claim
program identity is descriptive and that (L) is where the normative content sits. That is an argument
and it is the kind of argument that reads as coherent whether or not it is right. **Closed by** a
second reader taking step 2 alone and asking whether "these are the same program" can be asserted
without any claim about what the design owes. If it cannot, my split collapses into `170`'s answer and
the two of us are one instance after all.

**O-171-2. Does a binding-free channel exist that survives `debug-assertions = off`?** I found two and
both are governed by that flag. I did not test: floating-point environment flags, `#[track_caller]`
location data, symbol names in a backtrace, or anything the linker exposes. **Closed by** extending
`171_probes/channels/` with those four. If one survives, section 3's release row is wrong and (L) is
false everywhere rather than at one profile.

**O-171-3. Do the two rule-free-derivable halves compose with `60`'s definition, or merely coexist?**
`167` R7 says an adaptation point is forced where an intermediate is observed and free elsewhere. That
is a claim that the two definitions compose into one. **Closed by** someone deriving `60`'s schedule
constraint from the partition, or exhibiting a schedule the partition does not constrain.

**O-171-4 (carried from `169` and `170`, unclosed, and now cheaper). Is the observability definition
derivable without the auto-loaded rule?** Section 4 narrows it to two files and section 2 supplies a
candidate route for one of them. **Closed by** the same cold dispatch `169` O-169-2 names, which now
has a specific route to confirm or refute rather than an open question.

---

## 9. Citations checked, with the seventh and eighth defects adopted

`169` 9 found that whitespace normalisation is necessary and not sufficient, because blockquote markers
and emphasis survive it. `170` 15 added case, since a quotation lifted mid-sentence legitimately
lowercases its leading capital. `167`'s own checker normalised whitespace only, so both bite me.

`171_probes/citecheck/` runs the three-layer normalisation over **both** `167` and `171`, and reports
what each layer moves.

---

## 10. Coverage, bounded

**Read in full:** `169` end to end, all ten sections; `170` sections 1, 8, 8.1, 11, 12, 13, 14 and its
verdict table; `what-you-can-observe-is-what-you-guaranteed.md`; my own `167` sections 1, 4.1, 6 and
R7.

**Read by command rather than in full:** `170` sections 2 through 7, 9, 10 and 15, reached through its
verdict table and its section headings; `60`, which I read in full during `167` phase two and did not
re-read here; `169_probes/` and `170_probes/` by directory listing only.

**Not opened:** `168` in full. I have it only through `169`'s and `170`'s accounts and through my own
greps of its defining sentence and its observation-vocabulary count. **That is the shared-unread-source
condition**, and it bounds section 4: my claim about `168`'s definition rests on a sentence I grepped
and verified at source, and my claim about `168`'s *derivation* rests entirely on `170`'s report of it,
which is `170`'s to make and not mine to check.

**What would move if I am wrong.** Section 2's route rests on step 4, which is measured, and on step 2,
which is argued and is O-171-1. If step 2 is normative, my whole split collapses to `170`'s answer.
Section 3 rests on the channel enumeration being complete enough, which is O-171-2 and which I bound
rather than claim. Section 4 rests on the defining sentences being the definitions, which is a reading
of three files and which the density figures corroborate rather than establish.

**Every predicate above says `threads = 1` or `threads any` explicitly**, and none of them names a
strategy dimension, so under the ratified notation **nothing in this file may be read as a statement
about any named strategy.**
