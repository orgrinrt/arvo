# 159. Reply: five instruments are three, the container premise closes one of my options and splits two, and my own file carries no universal on any axis

Kiselyov, resumed from `154`. Read for this file: `157` in full, `158` in full, `157_probes/`
(`p3_predicate_audit.sh` and its output, `p4_output.txt`, `p1b_literal_ties.out`,
`p8_soundness_is_not_enforced/factoring_run.out`, and the `p3` script itself rather than its table),
`109` sections 2 and 13 and its predicate blocks, `155` section 5, and the five bench crates at
source. Two probes in `159_probes/`, committed as each ran.

**Three outcomes below, and they are not all the same.** I **concede** on S-8 and synthesise with it.
I **hold**, with a reason, that F157-3's count of five is three. I **correct** an attribution the
dispatch made rather than `157`.

---

## 0. The gates

**Test gate: passed.** Thirteen `-shared` crates, each run separately, `--release`, on
`nightly-2026-05-28`:

```
9 + 12 + 6 + 5 + 3 + 6 + 1 + 3 + 11 + 7 + 15 + 30 = 108   (twelve crates)
bitpack-write-contend-shared, -- --test-threads=1        = 15
                                                    total  123
```

All green. `bitpack-write-contend-shared` is run serialised and otherwise untouched, per the brief;
its hang and its soundness bug are handled elsewhere and nothing below rests on them. The profile is
named because I am the one who established this week that a bare timing without it is uninterpretable.

**Canon gate: passed.** Nothing here touches the RATIFIED rung. I13 is used as an instrument in
section 5 and not argued with. `156` item 1 is op's and I do not answer it; section 3 says which of my
options it reaches, which is a different act from answering it.

---

## 1. F6 went from one instrument to three, not five, and the count was never the strong part

`157` F157-3 (`157:288-294`) takes `154` F6 "from one instrument to five" and settles the
`154`/`155` disagreement in my favour. **I accept the settlement and I do not accept the arithmetic.**

The brief asks whether the count leans on the shape my own withdrawals were about. It does, and
`RULES.md:116-118` is the test: "independence means arrived at differently, not three probes sharing
one model." `159_probes/p1_are_the_five_instruments_independent.sh`, with its control:

```
bitpack-carrier-shared     -> bench-bitpack-plan-shared
bitpack-footprint-shared   -> bench-bitpack-plan-shared
bitpack-shared             -> <none: bench-core only>
bitpack-wide-shared        -> bench-bitpack-carrier-shared bench-bitpack-contend-shared bench-bitpack-plan-shared
crates in FOUR depending on another bitpack crate: 3 of 4

struct SplitMix64(u64);                    in 6 crates
pub type Plan13 = Pack<LOGICAL_BITS>;      in 2 crates
```

Three of the four depend on `bitpack-plan-shared`, and `bitpack-footprint-shared/src/lib.rs:9` says
in its own header that it "Reuses `bench-bitpack-plan-shared`'s transform, unmodified". The family
root, which `157` did not count, declares `PlanColumn<const N: usize>` and `MacColumn<const N: usize>`
and no element type either: the same shape, one level up, in the crate the others inherit from.

**So the honest count is three:** the `Carrier` bound, `bitpack-shared` (which depends on nothing but
`bench-core`), and the `plan-shared` family as **one** instance. Three meets the bar. Five overstates
it by counting one model four times, which is the exact defect that made me withdraw F11, F12 and F13.

**And `SplitMix64` is declared identically in 6 of 6 crates examined, `warm-container-shared`
included.** That generalises past this finding: every bench crate in this corpus descends from one
template, so **"N bench crates agree" is worth much less than N anywhere in this panel**, and a
vocabulary count over them is one instance until the dependency graph and the copied symbols have been
checked. I did not know that when I wrote `154` and it bears on any future count of this kind.

**Why this does not weaken F6, and this is the part I most want carried.** The strongest support
`157` produced is not in the count at all. It is that `Carrier` is `pub trait Carrier: Copy + 'static`
(`warm-container-shared/src/lib.rs:187`), implemented for exactly `u8`, `u16`, `u32`, `u64`, `u128`
(`:279-283`). `Copy` implies `Sized`, and `Sized` is what a 13-bit element cannot be. **That is a
proof about the target, not a sample of crates**, and it needs no count: it says `155`'s instrument
could not have refuted F6 whatever it found. The four-crate vocabulary is corroboration on top of a
structural argument, and corroboration is where the double-counting happened. `157`'s own result is
stronger stated without its number.

**F159-1. The four packed-end crates behind F157-3 are one dependency family and not four independent
instances; the independent count supporting `154` F6 is three.** Three of four depend on
`bitpack-plan-shared`; `SplitMix64` is declared identically in 6 of 6 crates examined; the family root
shows the same column-and-no-element shape and was not counted. `corpus = the six -shared crates
named in the probe at commit db6710b1, pattern = the greps in
159_probes/p1_are_the_five_instruments_independent.sh, threads any, target features any`. Evidence:
`159_probes/p1_are_the_five_instruments_independent.out`. Refines F157-3's count; does not touch its
conclusion.

---

## 2. Q157-D: the clause at `109:649-651` is `109`'s, not mine, and here is what I can add

**A correction, and it is the dispatch's rather than `157`'s.** The brief says "Q157-D widens **your**
target-independence clause at `109:649-651`". That clause is `109`'s. `157:858-861` attributes it
correctly and quotes it as `109`'s; `109:649-651` is inside `109` section 13's alternative B, in
`109`'s voice, reached from I6's disk-storage argument. I did not write it, `154` does not contain it,
and the reply to a widening of it is owed by `109` rather than by me. I flag this because acting on a
misattribution is how a claim ends up with no author who actually holds it.

**What I can add, from what I did derive, and `157` did not use it.** `154` section 2 established from
the assembly that I15 **entails** saturation: a parameter left runtime forces a `cmp` the compiler
cannot remove (`154_probes/p1_saturation/sat.s:31-39`), and I15 forbids exactly that check.

The `cfg` hazard passes straight through that. A `cfg`-gated realisation map is resolved at compile
time, so each build emits **one** lowered path and **no** runtime check. `157`'s own output shows it:

```
=== build base ===              === build alt ===
HAZARD    R(MAX+1) = 8191       HAZARD    R(MAX+1) = 0
CERT      separates = false     CERT      separates = true
```

Both builds satisfy I15 completely. So **I15 cannot catch this and neither can the entailment I
derived from it**: the property I15 buys is "one lowered path per build", and the hazard is that the
*denotation* differs across builds, which is a relation between builds that no single build's lowering
can witness. That is an independent reason for `157`'s conclusion that the residual obligation is a
lint rather than a type, arrived at from the intent side rather than from the `const fn` side, and it
sharpens the conclusion: the obligation is not merely *not* enforceable by a signature, it is not
enforceable by anything that inspects one build.

**F159-2. I15 does not constrain a `cfg`-varying realisation map, because every build of one
satisfies it.** Each build emits one lowered path and no runtime check, so the property I15 names
holds in both builds of `157_probes/p8_soundness_is_not_enforced/factoring.rs` while the denotation
differs between them. `builds = 2 (base, --cfg alt), toolchain = nightly-2026-05-28, W = 13, threads
= 1, target features any`. Evidence: `157_probes/p8_soundness_is_not_enforced/factoring_run.out`, read
rather than re-run; I did not rebuild it and I say so.

---

## 3. The container premise against my five options: it closes one, splits two, and does not reach two

The brief asks whether Q157-A is what my O-A through O-E were turning on. **It is not my concession
wearing different clothes, and it is not unrelated either.** My concession was about the *equivalence
test on one sentence*; Q157-A is about *the identity relation*. They intersect on three of five
options and miss the other two. Taken one at a time, which is the only honest way to answer it:

**O-D is closed by Q157-A, outright.** O-D was "the index deliberately over-counts, so a
currently-degenerate point can be split later". Q157-A decides whether the container axis is a real
coordinate. If footprint is observable, the container splits classes 32 to 64 (`157_probes/p4_output.txt`)
and the index is **not** over-counting: it is counting correctly and O-D dissolves. If footprint is
not observable, the index over-counts by exactly that axis and O-D is the live description of it.
Either answer resolves O-D and no other evidence is needed.

**O-A and O-B are split by it, in opposite directions.** Under *observable*, a packed 13-bit column
and a dense 13-bit value differ in identity and not merely in realisation, so they are two things and
**O-A**'s two vocabularies is the honest shape. Under *not observable*, the container is internals,
the two ends are one primitive differing only in how the target can address it, and **O-B** (or S-8's
lens, below) is right. So Q157-A does not pick between O-A and O-B; it makes each the answer under one
branch. That is worth stating exactly because it means neither can be argued for until the branch is
taken, and arguing for either now is arguing for a premise rather than a design.

**O-C is not reached by it.** O-C was "define the primitive as the saturated construction and stay
silent on type-versus-lens", and my worry was `RULES.md:79-83`'s equivalence test. Whether footprint is
observable does not change whether two teams handed that sentence converge. O-C is moved by S-5
instead, and section 4 answers S-5.

**O-E is not reached by it.** Retiring the word is about the three senses in `154` section 1, which
Q157-A does not touch.

**F159-3. Q157-A closes O-D, splits O-A against O-B into one branch each, and does not reach O-C or
O-E.** `options = 154 section 8's five, premise = whether the container is in the observation set`.
Evidence: the option texts at `154` section 8 read against `157_probes/p4_output.txt`. This is an
argument over stated texts rather than a measurement, and I mark it as one.

---

## 4. S-5 through S-9, answered one at a time

**S-8 accepted, and it is the best thing in `157` for my question.** "A primitive's realisation is
always a lens `(carrier, position)`; where the position is const-zero and the carrier is one machine
word, the lens is an identity and the thing is a value." This **is** the synthesis I could not find.
It gives O-B's single vocabulary without O-B's stated cost, which was that `Bool` and `USize` become
one-element columns: under S-8 the native end never mentions the lens because the lens is the
identity there. I withdraw O-B's cost clause as I stated it and carry S-8 in its place. This is a
concession and I am not going to dress it as a refinement.

**S-5 partially accepted, and the residue is real.** S-5 says O-C closes positively because the wall
is external and binds every implementer equally, so two teams both ship a lens at the packed end.
**The forcing is shared and the resolution is not.** Both teams meet the same wall, which is more than
I credited; but a column type with index accessors and a borrowed `PackedRef<'a, W>` are different at
the boundary in ways a consumer observes (whether one can be stored, what aliasing it permits, whether
it outlives the column), and the equivalence test is about behaviour. So S-5 narrows my worry from
"the sentence says too little" to "the sentence says enough about the wall and not enough about the
shape at the wall". That is a genuine improvement and I record it as one. **O-C stays open, narrower.**
S-5's own discriminator, which it did not run, is still the thing that closes it.

**S-6 accepted.** The arity question is the concrete spelling of an implementation and fails the
permanence test; the reason survives a rewrite and the spelling does not. This is the better home for
what my section 6 was reaching at.

**S-7 accepted for one wall and not the other, and the difference matters.** S-7 says my wall is `156`
item 1 wearing different clothes and so is not mine to break. That is right about the **arity** wall,
and relocating it onto an existing question is better than breaking it, exactly as S-7 argues. It is
**not** right about O-C's wall, which is the equivalence test on a sentence and is not a question about
which observations ship. Two walls, one relocated and one still standing, and collapsing them would
lose the second.

**S-9 answered in section 1**: accepted as a settlement, corrected as a count.

---

## 5. My own predicate audit, as asked

`159_probes/p2_predicate_audit.sh`, with a control that tests whether the extractor can read my
predicate blocks rather than assuming what they say:

```
findings stated               : 17
carrying `W any`              : 0
carrying another spelling     : 0
carrying `W in 1..=64`        : 3
carrying `threads any`        : 0
carrying `target features any`: 0
carrying `threads = 1` (fixed): 13
predicate blocks found        : 18
```

**`154` carries no `any` on any axis at all.** Not width, not threads, not target features. So it is a
fifth instance of F157-10's pattern, and `157` did not count it because it audited `110`, `111`, `112`
and `114`. Read literally under I13 my findings hold at `threads = 1`, at one host's baseline target
features, at the widths listed, and nowhere else.

**Where a finding of mine is a proof rather than a sweep, per the brief.** Two are, and I state the
widening **here** rather than editing the original, per `RULES.md:552-560`:

- **F6** is recorded at `W = 13` and its argument is a proof. `size_of` is denominated in bytes, so
  every Rust type's bit size is a multiple of eight; therefore for any `W` not a multiple of eight no
  Rust type has exactly `W` bits. The finding widens to **`W any where W mod 8 != 0`**, on that
  argument and with no new measurement. `154_probes/p2_fibre/FINDINGS.md`'s predicate stays exactly
  where it is.
- **F1**'s mechanism is a proof: a width the compiler cannot know forces a range check, for any width
  whatever, because the compiler's ignorance is the premise. Recorded at `N in {13, 47}`. It widens to
  `N any` on the mechanism; the *magnitudes* in `sat.s` do not widen and I claim nothing about them.

**Three of mine carry `W in 1..=64`**, which is not `W any` and is also not a three-width sample: it
is exhaustive over every width a `u64` container can hold. Under the notation read literally those
findings hold at 64 widths and at no other, which for a claim about a `u64` container is the whole
domain. **That is a case Q65 should have in front of it**: the notation currently cannot distinguish
"sampled three widths" from "swept the entire domain of the thing" from "proved for all widths", and
all three land as a non-`any` predicate that reads as narrow.

**And a defect in F157-10's instrument, which does not change its verdict.** Its control is
"`threads any` and `target features any` are non-zero, so a zero in the `W any` column is the corpus
and not the pattern". That proves the word `any` is findable. It does not prove the **width axis's
spellings** were enumerated, and those are different conjuncts. `109` writes a universal over integer
width as `I any, F any` (`109:156`), which the pattern `\bW any\b` cannot see:

```
file                                         W any   I any   F any   N any
110, 111, 112, 114                               0       0       0       0
109_bellard_the_primitive_derived_cold.md        0       2       1       0
```

**So F157-10 stands exactly as stated** and my knife does not cut it: no spelling of a width universal
appears in the four files it audited, which I checked across four patterns. What does not stand is its
prose generalisation to "topic five's corpus, read under its own rules, says nothing whatever about any
width arvo ships" (`157:35-36`, restated at `157:1011-1013`). Topic five includes `109`, which the count excludes only because
its findings are marked `**Predicate.**` rather than `**F<n>**`, and whose very first predicate is a
universal over integer width. The sweeping sentence is false of topic five and true of the four files.

**F159-4. F157-10's count holds over the four files it audited and its generalisation to topic five
does not, because `109` carries a width universal in a spelling the instrument does not enumerate.**
Four patterns checked across five files; `109` returns 2 for `I any` and 1 for `F any` while all four
audited files return zero on every pattern. `corpus = 109, 110, 111, 112, 114 at commit db6710b1,
patterns = the greps in 159_probes/p2_predicate_audit.sh, threads any, target features any`.
Evidence: `159_probes/p2_predicate_audit.out`.

---

## 6. What I am carrying forward unchanged, and from whom

**Six, with a count, and two of them replace things of mine.**

1. **S-8's lens formulation**, from `157`. Replaces O-B's cost clause. Carried entire.
2. **S-6**, from `157`: the arity question is implementation spelling and fails the permanence test.
3. **F157-4**, from `157`: the saturation bound holds only over signatures whose operations are
   functions of `(V, R)`. I did not verify its 32-to-64 grid myself and I say so; I am carrying its
   conclusion and using it in section 3, on `157`'s measurement rather than on mine.
4. **`110`'s definitional-versus-reachability distinction**, carried unchanged from my own phase two
   and unchanged again here. It is what refuted my P4 and nothing in `157` or `158` disturbs it.
5. **`109`'s const-availability membership criterion**, carried from my phase two, and section 2 above
   is a further limit on it rather than a correction to it.
6. **F157-13's conclusion** that soundness needs a lint rather than a type, from `157`, to which
   section 2 adds a second and independent reason.

**Not carried:** F157-3's count of five, corrected to three in section 1. F157-10's generalisation to
topic five, corrected in section 5. Neither correction touches the conclusion the number was attached
to, and I want that said plainly, because a corrected count that reads as a refutation is how a true
finding gets retired, which is a thing I spent this week establishing.

---

## 7. Coverage, bounded

**Read in full:** `157` sections 0, 1, 2.3, 3.7, 5, 6, 7; `158` in full; the five bench crates'
manifests and type declarations; `109` sections 2 and 13 and every `**Predicate.**` block in it;
`RULES.md:79-83` and `:116-118`; `154` section 8 and my own probe FINDINGS files.

**Read by grep, not in full:** `157` sections 2.1, 2.2, 3.1 to 3.6, 4; `111` and `112` except where
`157` quotes them and I opened the quotation.

**Not opened:** `157_probes/p2_const_certificate/`, `p5`, `p6`, `p7`; `114`; `OPTIONS.md` since Q52.
So everything I say about the adequacy certificate rests on `157`'s prose and its `p8` output, and I
have not verified the certificate compiles at every width from 1 to 64 as claimed. Nothing in sections
1, 3 or 5 depends on that; section 2 depends on `p8`'s output, which I read and did not re-run.

**What would move if I am wrong.** Section 1's count rested on `SplitMix64` being copied rather than
independently typed, and `SplitMix64` **is** a published routine, so six authors could each have typed
it from one paper. I named that as the hole and then closed it rather than leaving it: section D of
the probe hashes each declaration body with whitespace removed, and **five of the six are identical
while the sixth is `bitpack-shared`**, which is also the one crate with no `bitpack` dependency. The
copying and the independence agree, so the objection is answered in both directions at once and
F159-1's count of three stands on two signals rather than one. Section
5's `109` finding rests on `I any` meaning a universal over integer width in `109`'s notation rather
than something else; I read both occurrences in context and they are predicate blocks, but I did not
ask `109`.

**What I settled:** that F157-3's five is three, and that `154` carries no universal on any axis.

**What I moved:** O-C, from "expected to close negatively" to "open and narrower", on S-5.

**What I could not:** I could not decide O-A against O-B, and section 3 says why that is not a failure
of mine to decide: it is one premise away, the premise is `156` item 1, and it is op's.
