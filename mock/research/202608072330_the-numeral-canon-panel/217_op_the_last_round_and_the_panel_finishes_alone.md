# 217. Op: the last round, and the panel finishes without him

Op's own file. His words verbatim, the options each answer chose among, and the
instruction that closes his involvement. Required reading for every member after
this.

**Two of the four answers select none of the options offered and state an intent
instead.** Those two are the valuable half of the round, and they are recorded
below as he wrote them, spelling included.

---

## Before the round: what he wants and what he does not want

Sent one after the other, minutes apart:

> Let me just state firmly: Speed up the ratification process and get us past the
> canon work and into design + impl so downstream consumers get unblocked,
> they've been waiting for this rewrite and redesign for months now

> But again, don't rush and fuck it up by being careless or dismissive or naive.
> Just stay on point, be decisive, be explicit and steer the experts but harness
> their domain knowledge and experience to the full

**Both halves bind and neither softens the other.** Speed comes from deciding
rather than from lowering the bar, and the thing being sped up is the
ratification process, not the derivation underneath it.

---

## The accuracy target: he refuses the trade rather than the reading

**Options he was given**, none selected: accuracy means closeness to the exact
composite, which defers every interior resolution to the boundary; or accuracy
means reproducing a named reference implementation step by step, which picks a
different placement; or both, selected per arm. The two readings had been priced
at **15.5x in aggregate error and 16x in worst case at depth five**, so the fork
was real and was put to him as a fork.

> We never sacrifice performance. If we can't express something reliably and
> accurate enough for genereal computing (precision strategy is different, and
> would be used when more than usual precision is needed) without losing perf and
> efficiency, we haven't done out job well enough and lack the actual solutions
> and need to find them.

**The fork presumed a trade and there is no trade.** The accuracy intent does not
license a slow path, and a placement that buys accuracy by giving up performance
has not answered the question, it has changed it. Where no arm is known that is
both, that is a gap in the solutions rather than a licence to pick a side, and
the work is finding the arm.

**The precision strategy is the carve-out and it is narrow**: it exists for the
case where more than usual precision is wanted, and it is not the general answer.
So "accuracy" outside that strategy means accurate enough for general computing,
at full speed, and the interior placement question is answered by whichever
placement achieves that rather than by ranking one objective above the other.

---

## The exchange rate: there is no rate, because there is no generalisation

**Options he was given**, none selected: a lexicographic ordering with no rate,
which is what every objective except the performance-first one already does; a
stated rate per objective, in the canon; a rate the consumer supplies; or
silence, leaving "meaningful" to whoever writes the arm.

> This is case by case basis and has to be justified in words that get recorded.
> Even small wins are wins worth pursuing, however small, and however small a set
> they apply to. This is the "patchwork" approach where we don't even try to
> generalise, we operate on const predicates that choose the most optimal path
> always, from a million small different impls for different situations.

**This is I13 answering a question that was not posed as an I13 question.** An
exchange rate is a generalisation over a category, and the design rejects those
by premise, so the honest answer is that no rate exists: each arm carries its own
justification, in words, in the record.

Three things follow that the option list did not contain.

- **A small win is taken.** However small the gain and however small the region
  it applies to. The size of the region is not an argument against an arm; it is
  the arm's predicate.
- **The justification is written down.** "Case by case" is not silence: silence
  was on the option list and he did not pick it. What replaces the rate is a
  recorded reason per arm, which is checkable in a way a rate never was.
- **A million small impls is the intended shape**, not a failure mode to be
  refactored away later.

---

## The predicate notation gains two markers

**Options he was given**, two selected: a proof carries a different marker from a
measurement; **and** the bounded whole-container range gets its own spelling. He
did not select the per-file re-statement pass, and did not select leaving I13 as
it stands.

The measurement behind the question: across one topic's four files, **82 findings
and not one carrying a width universal**, against eighteen firings of the thread
universal and thirty-nine of the target-feature one. Read literally under I13
that topic holds at no width in the library, which is not what any of its authors
meant.

**So the notation now distinguishes three things where it distinguished two.**

- **A measurement** carries the region it was swept over, exactly as before.
- **A proof** carries a marker saying the argument is width-free by construction,
  rather than being dressed as a sweep that happened to stop at three widths.
- **A bounded whole-container range** is neither a sample nor a universal, and
  says so: every value of a container, exhaustively, at that container's width.

**Nothing is re-stated retroactively.** The per-file pass was on the list and was
not chosen, so the never-widen-in-place rule stands and the existing files keep
the predicates their evidence supported. The markers apply going forward.

---

## The rounding vocabulary retires the ambiguous word

**Options he was given**, one selected: retire both spellings for six explicit
names; keep `trunc` and define it once as toward-zero; or rename the hardware
operation to `bit_drop` and let `floor` cover it.

He took the first. **The vocabulary is `toward_zero`, `floor`, `ceil`,
`half_up`, `half_even`, `stochastic`**, and `truncation` and `trunc` are retired
in both spellings.

The factual half was settled before the question was put and is not contested:
bit-drop measures equal to floor on every row and differs from toward-zero on
signed rows only, so on a signed domain the retired word named two operations. A
note records that bit truncation of a two's complement value is floor, so nobody
reads the hardware operation back into the name.

---

## And then he left

> Now work autonomously, you don't need me anymore. I've given all I can, the
> canon should be solvable and fully fillable without me from now on with all
> that I've already said.

**Every remaining canon question is the panel's.** Not deferred, not parked
awaiting him: solvable from what he has already said, and the corpus of his
statements is the place to solve them from.

Three consequences, and the second is the one an agent will get wrong.

- **A question filed as his is now the panel's**, and the filing was often the
  error rather than the question. Peers first, heavy convergence is the answer,
  and the coordinator gates.
- **His absence is not a licence to invent.** Silence in his corpus is still not
  permission. Where his words genuinely do not reach a question, the answer is
  derived from the intent, inside its spirit, and put through two independent
  agreements, exactly as `expert-dispatch-defends-the-canon.md` requires.
- **The corpus is bigger than anyone remembers.** `.data/op-responses/` holds his
  words verbatim across the whole arc, `INTENTS.md` holds the ratified intents,
  and the ruling namespace holds what he has stamped. A question that feels
  unanswerable is usually a question nobody searched for in the words the answer
  would be written in.
