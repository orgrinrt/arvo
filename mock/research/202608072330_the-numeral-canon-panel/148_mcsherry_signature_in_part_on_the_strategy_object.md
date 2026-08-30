# 148. Signature in part on the strategy object candidate

I am `140`, resumed. `146` asks me for a signature on two specific things: whether the closure asymmetry
and the declared-width companion rule are represented at one expert correctly, and on section 6.2, which
repairs my own baseline obligation. The coordinator adds four more.

**Verdict: I sign in part.** Everything asked of me by name I confirm. I refute one clause, on my own
instrument, and I qualify a second.

- **Refuted: one third of the contamination scoping in section 1.1.** "`140`'s own refuted P3" is not a
  place where the congruence is the mechanism, and the congruence is not available as an explanation
  there at all. `148_probes/p1`, both controls firing.
- **Qualified: section 6.3.** Its closure of the count's second argument is correct and it **inherits
  section 6.1's status**, because the step that closes it is the firewall. `148_probes/p2`, three
  controls firing. That is a composition rather than a supersession, and the candidate presents the two
  sections as independent.
- **One clause I would change:** section 5.5's container predicate omits `signedness`, which under the
  notation the candidate itself applies means the clause holds in no region where signedness exists.
  A stronger and equally honest form is available from material already in the topic.

---

## 0. Gates

**Canon gate: passed.** Same check as `140` and `143`. `INTENTS.md:51-61` demotes I1 to open;
`INTENTS.md:369-371` says the count is beside the point of the intent. Nothing here proposes a design
decision or presumes the set closed. Section 3 below argues a candidate clause depends on another
candidate clause, which is a structural claim about the document rather than a design proposal.

**Test gate: inherited, and I add nothing.** `146:21-24` inherits 123 across 13 from
`137_probes/g0_test_gate.out` and lists four reproductions of the `--test-threads=1` requirement,
including mine at 2.57s. That count corrects my own `140`, which reported 124 from a loose grep, and
`141` found the extra hit. I have nothing to add and I do not re-run.

---

## 1. The two findings about my own blindness

### 1.1 The commit ordering: confirmed, and there is a sharper fact the ledger did not use

`146:37-43` says the ordering "runs the wrong way" for me and establishes nothing about the between-file
half. **Confirmed, and the times are right.**

```
$ git log --format='%h %cd | %s' --date=iso -- <139 and 140 phase one>
a60f1a47 2026-08-15 10:19:20 +0300 | panel: 140, the strategy set derived cold, phase one
861f89bd 2026-08-15 10:17:07 +0300 | panel: 139 phase one, the strategy set derived cold
```

Two minutes and thirteen seconds, with `861f89bd` adding `139`'s 672-line file to the tree before mine
landed. The ledger is right and I am not going to argue with a timestamp about my own commit.

**But there is a third commit and it changes what the ordering can carry.** My four phase-one probes
landed in one commit, `33c9b212`, at **10:14:56**, which is two minutes and eleven seconds **before**
`139`'s file existed:

```
$ git show --stat --oneline 33c9b212
33c9b212 research: probes for the strategy-set cold derivation
 140_probes/p1_assignment_partition.rs        | 579 +++++
 140_probes/p1_out.txt                        | 183 +++
 140_probes/p2_independent_partition.py       | 228 +++
 140_probes/p2_out.txt                        |  51 ++
 140_probes/p3_container_is_not_observable.rs | 399 +++++
 140_probes/p3_out.txt                        |  17 +
 140_probes/p4_baseline_rebase.py             | 131 +++
 140_probes/p4_out.txt                        |  31 ++
```

So the ordering splits three ways rather than two, and the middle case is the one the ledger flattens:

- **`139`'s probes** were committed between 09:51 and 10:11, so they were in the tree the whole time I
  worked. Blindness to those rests **entirely** on my disclaimer, and the ledger is right to say so.
- **`139`'s prose** did not exist when every measured result in my phase one was committed. The 24-class
  partition, the two-model agreement, the container sweep and the baseline arithmetic were all in
  history before `861f89bd`. For those, **the ordering runs the right way**.
- **My write-up** was committed after `861f89bd`. For the prose, the ordering runs the wrong way and the
  disclaimer carries it.

**I am not asking for a stronger rung out of this**, and it does not rescue the between-file half in
general, because probes are not where framing leaks and `139`'s probes were readable throughout. What it
changes is a sentence: the ordering establishes nothing about my **prose** and it does establish
something about my **measurements**, and those are the part a later reader would most want dated. I offer
it as a correction to the ledger's precision rather than to its conclusion.

### 1.2 The disclaimer carries the weight, and it says what the ledger claims

`146:42-43` rests my blindness on "`140:6`'s specific disclaimer" and says it is now carrying the whole
weight. I opened it, because a sentence carrying the whole weight should be read rather than recalled.

```
$ grep -n "139_probes/\` exists and did not open it" 140_...md
5:messages. I checked that `139_probes/` exists and did not open it. Everything below is derived from op's
```

**The sentence exists and says what the ledger claims it says.** The full disclaimer at `140:3-7` is
wider: it names `INTENTS.md`, `RULES.md` and the brief as what I read, and rules out "No panel file, no
`OPTIONS.md`, no register, no probes of anyone else's, no git log, no commit messages" before the
specific clause about `139_probes/`.

**One correction, small and worth making because of what the sentence is now doing.** It is on **line 5**,
not line 6. `146:34` cites the containing range `140:3-7` correctly; only the point citation at `146:42`
is off by one. A citation that resolves to the wrong line is the failure mode this panel has already
written down five times, and it matters more here than usual precisely because the ledger elevated this
one sentence to load-bearing.

### 1.3 The contamination: I confirm the principle and refute one third of the scoping

`146:45-56` says the workspace rules are a shared auto-loaded input, that
`arvo-always-optimal-internals.md` contains the one-sided-clamp congruence, and that wherever that
congruence is the mechanism the two cold derivations are one instance. **The principle is right and I
confirm the input.** `140:5` records reading "the workspace rules" and that is exactly what it means; the
rules load automatically in this repository and I did not enumerate them.

The scoping sentence names three places. I was asked to confirm or refute on my side, and **the third is
wrong**.

> That reaches the unsigned half of the fusion result, the unsigned accumulator cells, and `140`'s own
> refuted P3.

**Two things merge at unsigned `F = 0` and they are not the same merge.**

`139:173-176` is explicit about what merged in its table: "The two saturating **intermediate** values
merged at unsigned `F = 0`. The mechanism is that unsigned saturation clips on one side only, and
one-sided clipping of a monotone operation is a congruence, so reducing early and reducing late land in
the same place." That is the intermediate axis, it is a **relocation** question, and it is the workspace
rule's sentence. `139` says so itself and marks it as corroboration rather than discovery.

**My P3 was a different axis.** It predicted that at unsigned `F = 0` addition the class count would equal
the **overflow** axis cardinality, three, and measured two. What merged was `SaturateBoth` against
`SaturateHighOnly`, and the mechanism I gave in `140` was that two non-negative values never sum below
zero, so the low clamp is unreachable.

`148_probes/p1_two_merges_two_mechanisms.rs`, output at `148_probes/p1_out.txt`, `W = 5`, exhaustive:

```
=== A. the OVERFLOW merge at unsigned addition (140's refuted P3) ===
low branch taken: 0 of 1024 operand pairs
SaturateBoth and SaturateHighOnly agree: yes, on every pair
  -> mechanism is REACHABILITY: the branch that distinguishes them is never entered

=== control (i): the same two positions at unsigned SUBTRACTION ===
low branch taken: 496 of 1024 operand pairs
the two positions agree: no, they separate
  control fires

=== B. does the congruence proposition even apply at a single addition? ===
  single addition, Wrap: early vs late differ at 0 triples
  single addition, SaturateBoth: early vs late differ at 0 triples
  single addition, SaturateHighOnly: early vs late differ at 0 triples

=== C. where the congruence IS the mechanism: two reduction sites ===
  multiply-add, Unsigned, SaturateBoth: early vs late differ at 0 triples
  multiply-add, Signed, SaturateBoth: early vs late differ at 11986 triples
  control (ii) fires
```

**Part B is the decisive one.** The congruence is a statement about relocating a reduction across an
operation, so it needs two reduction sites. A single addition has one, and the two intermediate positions
coincide there at **every** overflow position **including wrapping**, where the congruence says nothing at
all. A zero that appears identically under a policy the congruence does not cover is structural, not
congruential.

So at the cell P3 names there is nothing to relocate, and the congruence is not available as an
explanation rather than merely not being the one I reached for. **The scoping's third item is refuted**,
and my P3 is not contaminated. The two controls establish that the sweep can see both effects when they
exist: the low branch is taken 496 times at subtraction and the positions separate there, and the
congruence visibly fails at 11986 triples at signed saturating.

**What I do not contest.** The first item is `139`'s and `141`'s territory; I have no fusion result and
nothing to say. On the second, the unsigned accumulator cells, I note only that **neither cold derivation
has an accumulator dimension**, which is `141`'s whole rescoping, so whatever contamination lives there is
not a contamination of the cold **pair**. My own `143` derived its unsigned-accumulator predictions from
the congruence and said so at the time, marking the agreement as weak rather than counting it; that stands
and I do not upgrade it.

**And the correction cuts in the ledger's favour once.** `139` and I made the *same wrong prediction* at
that shape, three where the answer is two. That coincidence is worth a later reader's attention even
though the mechanisms differ, because it is the shape a shared input produces.

---

## 2. My claims as represented, checked one at a time

Asked to check each is at the strength I established and no higher.

**The 24-class collapse on two instruments.** Represented in section 5.3's monotonicity predicate and in
the 1.6 intersection table. Not overstated: the candidate never says 24 is a property of the design, and
5.3 explicitly makes the count a function of the observation set. **Correct.**

**The witness-set relativity.** Section 5.3 and section 6.3. Represented as one claim jointly with
`139`'s shape variation, which is `141`'s proof and which I accepted in `143` section 3. **Correct, and
this is the representation I asked for.**

**The accumulator concession, and the "silent rather than refuted" reading.** `146:89-93` states it as a
scope refutation rather than a claim refutation, quotes `141:209-223` for it, and names both files'
overreaching sentences including mine at `140:577-579`. **Correct.** I said in `143` that under I13 my F3
listed no accumulator dimension and therefore claimed nothing about one, so it was silent rather than
wrong, and the candidate does not upgrade that into either direction.

**F2'.** `146:225-226`: "'Strictly increasing' is false, with 714 counterexamples on `141`'s axis set and
134 on `143`'s own. The content survives as F2' and the original stands unedited." **Correct, including
the detail that the original stands unedited**, which is the part a compression usually loses.

**The closure asymmetry and the declared-width companion rule, at one expert.** Section 1.5 lists both.
The closure asymmetry entry says `141:780-782` "carries it forward and says its own count function is
consistent with it rather than evidence for it", which is exactly the distinction I drew in `143` section
5 item 9. The declared-width entry says every later instrument is built on that reading, "which makes it
load-bearing and still one expert". **Both correct, and the second is stated better than I stated it.**

**The prose overreach.** Represented at `146:89-93` via `141`'s reason: neither predicate carried a
dimension that would have narrowed it, because neither model had one. That is true and it is not the
reason I gave. Mine, in `143`, was that the sentence carried **no predicate at all** because it was prose,
and prose is not audited by the notation.

I am not asking for that to be added as a correction, because both readings are true and the candidate is
entitled to compress. I raise it because **it is the same failure as section 6.1**, and the candidate does
not connect them. The firewall carries no predicate because it is a proposition rather than a measurement;
my sentence carried no predicate because it was prose rather than a finding. In both cases a claim
travelled without the thing that would have bounded it, and in both cases nothing in the notation fires,
because the notation audits findings. If op reads 6.1 and asks how a canon ends up with an unpredicated
sentence in it, my overreach is the cheap worked example, and it is already in the record.

---

## 3. The count's second argument: it composes, and it inherits 6.1

`146:589-602` says I was right that the table is not writable and wrong about why: not because the
denotation line is open, but because the operation set is not named, which is a decision rather than
evidence. The coordinator asks whether that supersedes my reading or composes with it.

**It supersedes the specific worry and composes with the general shape, and there is a third thing that
neither of us stated.**

**What it supersedes, and I concede this cleanly.** My `143` F4 worried that the assignment set moves
depending on where the denotation line falls, and cited my own p6: 24 classes with the intermediate axis
in, 12 or 14 with it out. `145`'s saturation result answers it. Visibility is monotone in the observation
set and saturates, so an axis is component one exactly when it is visible under the maximal set. My p6
already measured the intermediate axis as visible under a five-operation set, so it is visible under any
larger one, so it is component one and cutting it is not available. **My specific worry is closed and the
closure is better than my framing of the problem.**

**What composes.** The count still has the two arguments I named. What changed is that the second one's
value is fixed by a decision instead of being open-ended, which is a much better position than "open" and
is not the same as "one argument".

**And what neither of us said: the step that closes it is the firewall, so the closure is as settled as
the firewall is.**

Section 6.3 closes the argument by declaring that an axis visible under the maximal observation set **is**
component one. That declaration is not forced by the visibility measurement. It is forced by the
proposition that nothing outside the declared policy may move an answer, which is section 6.1's firewall,
which section 6.1 records as carrying no predicate in any of the three files that endorse it.

Drop the firewall and an answer-visible choice may sit in component two. That is not hypothetical: it is
exactly `139`'s slack repair, which section 1.8 retires on **measured** grounds, that the residue it
served is empty, rather than on the ground that it is incoherent.

`148_probes/p2_the_closure_rests_on_the_firewall.py`, `W = 5`, `F = 1`, signed saturating, exhaustive:

```
LIVE axis (stepwise vs fused): differ at 9722 triples, max gap 16
DEAD axis (stepwise vs a second spelling): differ at 0 triples, max gap 0

rule             live axis        dead axis
FIREWALL         component one    component two
SLACK(0)         component one    component two
SLACK(8)         component one    component two
SLACK(16)        component two    component two
SLACK(64)        component two    component two
```

**The same axis, the same measurement, two classifications.** It is component one under the firewall and
component two once the declared slack reaches 16, with nothing about the arms changing. Three controls
fire: slack zero agrees with the firewall exactly, so the two rules are one family; a dead axis whose two
positions are extensionally identical is component two under every rule tried, so the classifier does not
manufacture policy content; and the live axis is component one under the firewall, so the sweep contains a
case where the rules could differ.

**So which axes are in the assignment set is a function of the classification rule, and the rule is the
unpredicated proposition.** That does not weaken 6.3 and I am not asking for it to be withdrawn. The
firewall is endorsed by three files, I endorse it, and `141` and `142` between them closed the one live
alternative. What it changes is a dependency the candidate does not draw: **6.3 is not an independent
closure sitting beside 6.1's open question. It rests on it.** If op decides in 6.1 that a canon may not
carry an unpredicated proposition, 6.3 does not survive unchanged, and a reader should know that before
answering the 6.1 question rather than after.

**This is one expert's and it wants a second read**, in the specific form of somebody checking that my
SLACK rule is a fair rendering of what a non-firewall design would actually do rather than a strawman
built to flip. The probe's slack-zero control is what makes it not a strawman, but a control is not a
substitute for someone disagreeing with the construction.

---

## 4. Section 6.2, which repairs my obligation: I sign it

`146:569-587` reconciles `144`'s sections 6 and 4.3 by reading the per-coordinate division as a
declaration of the weighting's units rather than a transformation of the cost table, with `145`'s `z5` E4
reproducing the 24.6% independently at 894 of 3640.

**I sign it, and I want to be precise about what my signature is worth here**, because `146:587` names me
as one of the signatures that decides whether it holds.

What I can attest: the reading is consistent with the shared-baseline obligation as I stated it in `140`
section 11.4 and `143` section 6, and it repairs a gap I left. My obligation said every cost claim is
stated against the same named arm. It did not say **in what units**, and 6.2 is right that one named
baseline names two operations. That is a real hole in my formulation and the units reading fills it.

What I cannot attest: I did not build an instrument against 6.2 and I did not reproduce `145`'s E4. The
candidate calls 6.2 "one expert's and an argument rather than a measurement", which is the correct
strength, and my signature does not raise it. **I am confirming that it repairs my obligation coherently,
not that it is measured.** The distinction matters because 6.2 reconciles two of `144`'s own sections, and
`144` is the signature that decides that part, not me.

---

## 5. The one clause I would change

**Section 5.5's container predicate omits `signedness`, and under the notation the candidate applies that
means the clause holds nowhere.**

The predicate reads: `W = 4; F in {0, 1, 2}; operations in {add, subtract, multiply}; overflow in {wrap,
saturating}; rounding in {toward zero, floor}; overflow limit read at the declared width; threads = 1.`

Section 1.6 explains why: the intersection of the three instruments omits it, and the table lists
`signedness` among the ten dimensions "in the union only". That is honest and it is the rule working as
written.

**The problem is that intersecting a dimension the instances *partition* rather than *overlap* yields an
empty region.** `139` swept signed, `140` swept unsigned, `141` swept both. Their intersection on that
axis is empty, so the dimension drops out, and `RULES.md:519-520` is explicit that an absent dimension
means the finding "does not hold in any situation involving that dimension at all". Every fixed-point
numeral has a signedness. So the clause as predicated holds in no region a consumer can instantiate,
which is not what any of the three instruments measured and not what the candidate means.

**The repair is available from material already in the topic and it is stronger, not weaker.** `141`'s
own instrument spans both signednesses by itself, at `141:149-152`:

```
| W=4 F=0 unsigned | 12 configs, 2 classes | 2 | 4, control fires |
| W=4 F=0 signed   | 12 configs, 2 classes | 2 | 4, control fires |
```

So the honest form is two sentences rather than one: the three-instrument intersection as written, plus a
statement that **one of the three establishes both signednesses on its own**, with `signedness in
{unsigned, signed}` in its own predicate. That gives a clause a design can actually gate on, and it keeps
the intersection discipline intact by not pretending the three agree about something they did not jointly
range over.

**I flag this as a general hazard rather than a one-off**, because section 1.6 is a good instrument and
this is its blind spot: it computes an intersection over dimension **names**, and a dimension every
instance lists but with **disjoint values** intersects to nothing while looking present in every input.
Section 1.6 already warns that the intersection is an upper bound and that it cannot check that two files
mean the same thing by a name. This is the adjacent case: the same name, different values, empty overlap.

I have not checked the other rows of the 1.6 table for the same pattern and I am not claiming they carry
it.

---

## 6. What I checked, and what I did not

**Opened at source rather than recalled:** `140:3-7` and `140:5` for the disclaimer wording and its line;
`140:577-579` for my own overreach; `139:172-180` for its merge and mechanism; `141:147-152` for the
signedness span of its container instrument; `141:209-223`, `141:780-782`; `146` in full; `INTENTS.md`
at I1 and I17; `RULES.md` at the predicate section; the commit log and `git show --stat` for the
phase-one ordering including my own probe commit.

**Read in full:** `146`, `141`, my own `140` and `143`.

**Read in part:** `139` at its sections 3 and 4 and its class table, which is what `143` also did, so
where `146` cites `139` outside those I am taking the candidate's account. `145` not read; its `z1`, `z4`,
`z5` and `z6` results reach me through `146` only, which means section 3's reliance on the saturation
result is reliance on `146`'s account of `145`. `144` not read; section 4's signature is on `146`'s
rendering of it, and I say so there.

**Not read:** `142` in full, `147` if it exists, and the probe sources of `139`, `141`, `142`, `144` and
`145`. I did not open `141_probes/p4` in `143` and I did not open it now.

**Built:** two probes, five controls, all firing. One prediction of mine in them and it held; I do not
count that as informative, because both probes were built to test somebody else's claim rather than mine
and neither had far to fall.

**Not done.** I did not reproduce `145`'s saturation theorem, which section 3 leans on. I did not attack
the firewall proposition itself, which I endorse and have never instrumented. I priced nothing: both
probes are exhaustive enumeration over a small domain and neither times anything. Both are at `W = 5`
with no transfer argument to 64 bits, and both are `threads = 1`, so under the notation neither holds
anywhere threads exist.

**The thing I would most want a second reader on** is section 3, because it is the only place I claim a
structural dependency between two clauses of the candidate rather than reporting a measurement, and
because the SLACK rule in `p2` is my own construction of what a non-firewall design would do. If somebody
builds a different non-firewall classification and the live axis stays component one under it, my section
3 is wrong and 6.3 stands alone after all.

---

## 7. The signature

**Sign in part.**

I sign: section 1.1's ordering finding, with the three-way refinement in 1.2 offered as precision rather
than dissent. Section 1.2's rescoping of the convergence as a narrowing rather than a refutation. Section
1.5's placement of the closure asymmetry and the declared-width companion rule at one expert, both stated
correctly. Section 1.8's treatment of my F2'. Section 5.3's count clause. Section 6.2, at the strength the
candidate itself gives it and no higher.

I refute: the third item of section 1.1's contamination scoping, on `148_probes/p1`.

I qualify: section 6.3, which is correct and which inherits section 6.1's status, on `148_probes/p2`.

I ask for one change: section 5.5's container predicate, per section 5 above.

**And one thing I will not sign, because it is not mine to sign.** Section 8 lists "whether a canon may
carry an unpredicated proposition at all" as op's. I agree it is op's, and section 3 above means that
question reaches further than section 6.1 states: it decides section 6.3 as well. That is a fact about the
candidate's own structure and op should have it before answering.
