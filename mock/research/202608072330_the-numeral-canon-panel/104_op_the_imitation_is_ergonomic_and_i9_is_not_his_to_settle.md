# 104. Op: the imitation is ergonomic, the panic is a bounded intent, and I9 is not his to settle

**Op's own file.** Required reading for every member after it. Recorded at the close of the
strategy-axis unit's eight experts, before the consolidation, in the first batch of a queue-draining
session he called for.

**He was choosing among stated options in each case**, and the options are reproduced with each answer
per `record-the-options-a-decision-chose-among.md`, because an answer without them is unusable later.

---

## 1. What I3's imitation targets at a width Rust has no primitive for

**The options put to him:** (1) the **declared width**, so a 13-bit type behaves as if Rust had a
13-bit primitive, wrapping at `2^13`, with the container invisible; (2) the **container**, so a 13-bit
value stored in a `u16` wraps at `2^16`; (3) **neither, it is ergonomics**, the imitation being about
how it feels to use rather than about where arithmetic boundaries land.

**His answer:**

> Neither, it's ergonomics

**The intent, named separately from the answer.** I3 is not a statement about arithmetic boundaries at
all. "Behave like native primitives in regular old rust would" is about **the experience of using the
type**: that it is unsurprising, that it does the expected thing, that a reader who knows Rust's
primitives is not caught out. Where the boundaries land is a different question, answered by the
overflow policy and the width, not by I3.

**What this does to the panel's work on it.** `93`'s F8 measured that the declared-width and container
readings disagree at all fourteen non-native widths it swept. That measurement stands and is correct;
it is simply **measuring something I3 does not range over**. The two readings were both wrong about
what the intent is, so the fork between them was never a fork about I3.

It also retires the framing this file was asked under. Q47 part one asked which of two arithmetic
readings is meant. The answer is that neither is, and the question was built on a premise nobody had
checked with him.

## 2. Whether the imitation covers the native debug-overflow panic

**The options put to him:** (1) **no, values only**, so I3 covers what values come out and never the
runtime signalling, with I15 winning outright and no tension existing; (2) **the same guarantee at
compile time**, so what the panic catches is still caught, but the imitation is of the guarantee rather
than the mechanism; (3) **yes, I15 bends here**, a debug-only panic being the one acceptable exception.

**His answer, and then his own correction to it minutes later.** Both are recorded, in order, because
the second is what fixes the first.

> Option 3 but strategy bound and never on release outside of warm. As a rule of thumb, not an
> absolute, more of an intent

> Q2 might be never on hot outside of dev/debug. It's the intent inferrable

**Do not read the marker names literally, and he says so himself.** The first statement bounds the
panic by naming the imitate-the-native-primitive concern; the second bounds it by naming the
speed-first concern and says the intent is the inferrable thing rather than the wording. Per I1 the
strategy set is open and per I17 a marker name is vehicle, so **neither "warm" nor "hot" is doing work
here as a fixed member of a fixed set.** They name concerns, and his correction is what makes that
unmistakable.

**The intent, named separately from both quotations.** A native-primitive-style overflow panic **is**
permitted, and I15 bends for it, bounded on two axes and held as an intent rather than a gate:

- **By build.** Dev and debug only. It does not survive into a release artifact. Both of his statements
  carry this bound, worded differently.
- **By concern.** It belongs where imitating the native primitive is the point, and it must not appear
  where cost is the point. A path chosen for speed does not carry a check that exists for familiarity.
- **As a rule of thumb.** His words, and unretracted by the correction. It is not written into the canon
  as a gate, and an arm that cannot honour it is not thereby wrong.

**What this does to I15.** I15's "never any runtime checks, ever" is unchanged as the governing
posture. This is a declared, bounded exception inside it rather than a hole in it, and the reason it
does not undermine I15 is the build bound: the shipped program still contains no runtime validation.

**One thing left open on purpose.** Whether the panic may appear in dev builds of the speed-first
concern, or is excluded from that concern entirely, is not settled by either statement read literally,
and he has said the intent is what to infer rather than the wording. Under the intent above it is
excluded from any path chosen for cost, because a check that exists for familiarity has no business on
one. That reading is the coordinator's and is marked as such; it is cheap to correct and costs nothing
to act on, since no code exists yet.

## 3. Whether I9 describes the strategy pair or only its policy half

**The options put to him:** (1) **only the policy half**, the weighting picking among arms that already
agree and therefore being implementation; (2) **the pair, both halves**; (3) **the pair framing is
wrong**.

**His answer:**

> I think the intent is clear and this is impl detail that already had answer: optimal and converged to
> by experts (plural, iterative)

**He declined the question rather than picking, and the decline is the answer.** I9's intent is clear
as stated. Which of the two halves the word attaches to is an implementation decomposition, and the
mechanism for settling it already exists: **the experts converge on it, plural and iteratively, and the
answer is whichever is optimal.** It is not a call he owes.

**So Q50 is closed as not-his and returns to the panel**, where `102` already states both readings and
says it can build either. The consolidation carries it as an open implementation question with a
decision procedure attached, not as a question awaiting op.

**And this is the third time in this arc a question was put to him that the experts should have
settled.** The other two were category-wide policy forks, which
`never-ask-which-single-rule-governs.md` names. This one is a different shape and the rule does not
cover it: it asks op to adjudicate **which component of a decomposition his own word attaches to**,
where the decomposition is the panel's invention and the answer changes nothing he cares about. The
test that would have caught it: **if both answers leave the intent intact and differ only in what the
panel calls things, it is not his.**

## 4. The 21 numbered decisions outside the intent catalogue

**The options put to him:** (1) **premature, discard**; (2) **still his, fold into `INTENTS.md`**;
(3) **mixed, needs a pass**.

**His answer:**

> Already answered.

**He is right, and this question should never have been asked.** It is derivable from I12, which is his
own statement at `01` section 0:

> we don't need to settle this with so loose base. We can explore more ... Until that time, my word is
> only thing that ratifies shit, and the last panel process taught me that I shouldn't go and ratify
> anything before the experts actually agree and have a converged thing to bring to me

The 21 decisions were made **before** this panel's experts converged on anything. By I12 they are
**acks, meaning the direction checked out at the time**, and they are not ratifications. That is the
answer, it required no new input, and asking for it spent his attention on something already on the
record.

Compounding it: `87` already settles what happens to them operationally. The canon is written once at
the end from every consolidation, so anything in that document that still holds will be reachable then,
and anything that does not will not have been imported in the meantime.

**The pruning failure, stated so it is checkable.** The pre-ask filter he asked for is not only "did he
answer this exact question". It is **"is the answer already implied by an intent he has stated"**. I12
is a general rule about the status of his own prior calls, and it answers every question of the form
"is this old call of yours still binding" without being asked again.

---

## What this file changes

- **I3 is amended** in `INTENTS.md`: its subject is the experience of using the type, not the
  arithmetic boundary. The two arithmetic readings the panel built are both outside it.
- **A new entry records the bounded panic exception**, as an intent rather than a gate, with its three
  bounds.
- **Q47 is closed.** Part one rested on a premise he rejected; part two is answered by the new entry.
- **Q50 is closed as not-his** and returns to the panel with a decision procedure.
- **Nothing about the D-numbered decisions changes**, because I12 already governed them.
