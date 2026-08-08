# Op: arvo adapts to the cores it finds, and takes no stance on how many there are

**Date:** 2026-08-08. **Position:** after `31` was dispatched, answering the question `27` returned.
**Required reading.** Op marks this as **a ratifiable intent behind arvo**, which is the first time
anything in this panel has been named that way.

> **Corrected by `34`, and this file may not be read without it.** The phrase "without sacrificing the
> soundness" below reads as a uniform condition over every strategy. It is not. Op: it "is property of
> all of them except Hot. Hot *can* sacrifice soundness, that is its explicit purpose, but it should
> not lose it for nothing, instead, provable meaningful gains." The correction is recorded in full at
> `34_op_hot_may_sacrifice_soundness_for_proven_gain.md`.

## The question this answers, and the options it was choosing among

`27` measured packing under memory-system contention and found the break-even carrier moves by a
factor of two and a half to four between one core and four, with the answer against a four-byte
carrier **changing sign**. It returned one question rather than a menu: is the canon's claim about one
core, or about the declared parallel workload?

The options on the table were: **one core**, the regime every prior panel measurement was taken in;
**the declared parallel workload**, which arvo's own rules assert and which inverts several answers;
or **the inequality alone**, carrying no regime and no threshold.

## His answer, verbatim

> Yes, arvo will be multi-threadanle wherever it is proven to improve performance without sacrificing
> the soundness. We are a library, not a program, so we don't know how end users will use us, however,
> our main selling point are the algo crates that hilavitkutin, vehje, pretty much every single repo
> and project I have, downstream, use. As well as the contracts for things that compose to bigger
> units than just numerals alone. But we need this base to work, to build the bigger things. So that
> in mind, it should be implicit that we do what is most efficient and best performance in each
> different situation. We will run in threads = 1, threads = 2, threads = n where n can be any finite.
> We don't take stances on these. If it gives juice and proves more efficient than the alternatives,
> we should do that, when we can detect we have several cores available. When we don't, we do what is
> the most efficient thing in a single-threaded realm. This is something to put into the canon in
> fact, this is a ratifiable intent behind arvo.

## It does not select an option. It dissolves the question.

The question assumed the canon names **a** regime. His answer is that **arvo takes no stance on the
regime**, because a library does not know how it will be used. So neither "one core" nor "the parallel
workload" is the claim, and the third option is closer but still wrong: the inequality alone is
regime-free, while what he describes is regime-**sensitive** and resolved at the point where the core
count becomes knowable.

Four things follow that are load-bearing, and they are separable.

**Adaptation is conditional on proof, both ways.** "Wherever it is proven to improve performance
without sacrificing the soundness." Two conditions, not one, and the soundness condition is the one an
optimisation pass will be tempted to trade against. **Per `34`, that second condition is per strategy
rather than global**: hard for every strategy except `Hot`, and for `Hot` a price (a proven meaningful
gain) rather than a prohibition, because trading soundness away is what `Hot` is for.

**The core count is detected, not declared.** "When we can detect we have several cores available",
and otherwise "what is the most efficient thing in a single-threaded realm". So there is a
detection step, and there are at least two arms behind it. Whether detection is a build-time fact, a
runtime fact, or a consumer-supplied one is **not** stated here and is not to be assumed.

**The thread count is unbounded and unranked.** "threads = 1, threads = 2, threads = n where n can be
any finite. We don't take stances on these." No privileged n, and in particular one core is not the
default case with parallelism as an extension, nor the reverse.

**"Most efficient in each different situation" is the general form**, of which the packing carrier is
one instance. That generalises well past this question and touches every measured fork the panel
holds.

## The identity claim, which is separate and larger

Half of what he wrote is not about threading at all, and it is the part with the widest blast radius:

> We are a library, not a program [...] our main selling point are the algo crates that hilavitkutin,
> vehje, pretty much every single repo and project I have, downstream, use. As well as the contracts
> for things that compose to bigger units than just numerals alone. But we need this base to work, to
> build the bigger things.

Three claims about what arvo **is**, none of which the panel has been reasoning from:

- **A library, not a program.** The consumer's usage is unknown by construction, which is the premise
  under the no-stance position above.
- **The selling point is the algorithm crates and the composition contracts**, not the numerals. The
  numerals are the base that has to work so the bigger things can be built.
- **Composition to units bigger than a numeral is a first-class concern**, named in the same breath as
  the algorithm crates.

The panel has spent thirty-one files on the numeral. On his account the numeral is the **base**, and
what it is a base **for** is where the value is. That is worth carrying into every remaining question,
because "which shape serves all other parts of arvo best" is his selection criterion, and this sentence
says which parts those are.

## Standing

He calls this **a ratifiable intent**, and that wording is deliberate and is not the same as ratified.
His own correction governs: an opinion given before the experts converge is an ack meaning the
direction checks out, and ratification is the last step, reached only when a converged thing is brought
to him. So this is direction of unusually high confidence, marked by him as canon-bound, and **it is
not yet in the canon and no member may cite it as settled.**

What it does change immediately: the Q7 question as `27` posed it is answered, the option "the canon
names a regime" is dead in both its forms, and a new option is live in their place. The register is not
edited here because a member is reading it; the edit follows when that dispatch lands.
