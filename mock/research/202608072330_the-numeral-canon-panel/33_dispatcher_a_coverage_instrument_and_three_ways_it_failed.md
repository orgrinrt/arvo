# A coverage instrument for the register, and the three ways it failed first

**Position:** after `31`. **Author:** the dispatching agent.
**Standing:** tooling plus its validation record. Carries no authority over any design question.

The register has now been built twice and missed a flagged-as-op's question both times. `30` rebuilt
it from the member files and recovered one; `31` checked that rebuild and found another. Both passes
were careful and both checked coverage by **reading**.

Reading is what failed, so this measures instead. `33_probes/coverage.py` extracts every claim a
member file hands to op and asks whether the register carries it.

## What it does

For each member file, find every paragraph containing a phrase a member uses when it hands something
over (`is op's`, `genuinely undetermined`, `question for op`, `asked for a second read`, and the rest
in `FLAGS`). Split each into independently-testable claims. For each claim, take its distinctive
words and report the ones the register and droplist do not contain anywhere.

It is **high recall on purpose**. It over-reports, and every hit is a candidate to confirm or dismiss.
A precise extractor that silently dropped one line would reproduce the defect it exists to catch.

**Read the delta, not the count.** Thirty-four standing candidates is not thirty-four defects; most
are boilerplate and provenance notes. What carries information is what changes across a register edit.
Restoring the missing question moved it from 36 to 34, and the two that vanished were exactly the
restoration.

## Its validation, which is the part that makes it worth anything

Run against the register **as it stood before the missing question was restored**, it must flag that
question. Run against the register after, it must not.

```
before:  02_carried_...:49  absent: order's, amended
         "the order's own predicate is amended to identify shapes that denote
          the same value set, which turns out to be a precondition for ..."
after:   0 occurrences
```

Both hold. A seeded known defect is caught and the negative control is clean.

## The three designs that failed, and why each is instructive

Recorded in full because the next person to build a coverage check will reach for all three in this
order, and because each failure is a fact about this corpus rather than about the code.

**One: line-level word overlap.** Score each flagged line by the fraction of its distinctive words the
register contains, flag below a threshold. Run against the seeded defect it produced **the identical
output** as against the clean register: seven hits, none of them the real one.

The reason is that the flagged line names **two** things, `whether Precision counts the sign digit;
and whether the order's predicate is amended`. One was carried and one was not, and averaging over the
whole line let the carried half lift the absent half over the threshold. **A whole-line test cannot
see a half-carried conjunction**, and a half-carried conjunction is exactly what happened.

**Two: clause splitting, still line-based.** Split on `;`, `and whether`, `or whether` and test each
clause alone. This is the right idea and it still missed, for a reason that has nothing to do with
claims: **these files are hard-wrapped at about 100 columns**, so the claim spans three physical lines
and only the first carries the flag phrase. A line-based scanner sees the flag and the first fragment
and never reads the rest of the sentence. Fixed by scanning paragraphs.

**Three: paragraph-level clause overlap.** Now the whole claim is visible and split correctly, and it
**still** scored 85 percent and passed. Its distinctive words are `identify`, `shapes`, `denote`,
`value`, `set`, `precondition`, and every one of them appears elsewhere in an 838-line document about
numerals, shapes and value sets. Only `order's` and `amended` were absent.

**That is the general lesson and it outlives this script: a bag-of-words echo test cannot find a
missing claim inside a large document that shares its domain vocabulary.** The shared vocabulary is
noise and swamps the signal. What discriminates is the handful of terms the missing claim does not
share with anything else, so the score has to be built from **absent distinctive terms** rather than
from average overlap. Two or more absent distinctive terms is the current trigger.

## What it does not do, stated so nobody trusts it further than it goes

It finds claims a member **flagged**. A member that names an option without any of the flag phrases is
invisible to it, and the register's job is broader than op-flagged items. So this closes one hole
rather than the class.

It cannot tell a genuine gap from a paraphrase. `order's` and `amended` being absent is evidence, not
proof, and every hit still needs a reader.

It compares against the register and droplist only. A claim carried somewhere else in the panel and
not in either still reports as absent, which is the intended bias.

It has no test suite of its own beyond the seeded-defect run above, which is one instance rather than
three.

## How to use it

```
python3 33_probes/coverage.py            # candidates
python3 33_probes/coverage.py --verbose  # and what passed
```

Run it before and after any register edit and read the difference. Exit status is 1 when anything is
flagged, so a gate can consume it, though nothing consumes it today and claiming otherwise would be
the unimplemented-gate failure this workspace has already paid for once.
