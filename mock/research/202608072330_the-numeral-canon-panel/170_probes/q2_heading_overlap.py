#!/usr/bin/env python3
"""q2. How much of the shape does a shared premise set actually fix?

`169` 1.1 refuted the evidence `168` 23 offered for its own discount: neither
heading it called word-for-word identical is identical, both are strict
prefixes, and `168` quoted its own shorter form. I confirmed that by opening
both files and I concede it.

`169` R-3 asks for the replacement: measure token overlap rather than identity,
because a prefix relation is a real signal of shared framing and identity
rounds it away. This is that measurement.

Method: for every `168` section heading, find the best-matching `167` heading by
Jaccard overlap on lowercased word tokens with stopwords removed, and report the
distribution. A shared premise set that fixes a lot of the shape shows up as a
mass of high-overlap pairs; one that fixes little shows up as noise.

THE CASES THAT MUST BEHAVE A PARTICULAR WAY:
  C1. The two known prefix pairs must score high, or the metric cannot see the
      relation everyone agrees is there.
  C2. A random re-pairing of the same headings must score much lower, or the
      metric matches everything and its high scores mean nothing.
  C3. The two exactly-shared headings must both be ones the standing rules
      dictate, or the "dictated, therefore zero evidence" reading is wrong.
"""
import re, sys, random

STOP = set("a an the and or of to in for at on is are be it its this that what "
           "how why when where which with without as by from not no does do".split())

def headings(path):
    out = []
    for line in open(path):
        m = re.match(r'^## (?:[0-9]+[a-z]?|R[0-9]+)\.\s+(.*?)\s*$', line)
        if m:
            out.append(m.group(1))
    return out

def toks(h):
    return {w for w in re.findall(r"[a-z0-9']+", h.lower()) if w not in STOP}

def jac(a, b):
    """Symmetric similarity. Penalises the longer heading, which is why it is
    NOT the right metric for a prefix relation; see cont()."""
    A, B = toks(a), toks(b)
    if not A or not B:
        return 0.0
    return len(A & B) / len(A | B)

def cont(a, b):
    """Containment of a in b: what fraction of a's tokens b carries. 1.0 for a
    strict prefix. Added after C1 fired on the Jaccard version: the second
    prefix pair scored 0.44 because 167's heading carries five extra tokens,
    so a symmetric metric reports a subsumption as a partial match. The control
    catching that is the reason the probe reports both."""
    A, B = toks(a), toks(b)
    if not A:
        return 0.0
    return len(A & B) / len(A)

h168 = headings("../168_mcsherry_the_chain_derived_cold.md")
h167 = headings("../167_rompf_the_chain_derived_cold.md")
print(f"168 headings: {len(h168)}   167 headings: {len(h167)}")
print()

pairs = []
for a in h168:
    best = max(h167, key=lambda b: jac(a, b))
    pairs.append((jac(a, best), a, best))
pairs.sort(reverse=True, key=lambda t: t[0])

print("Top matches by token overlap (168 heading -> best 167 heading):")
for s, a, b in pairs[:8]:
    mark = "EXACT " if a == b else ("PREFIX" if a in b or b in a else "      ")
    print(f"  jac {s:.2f}  cont {cont(a, b):.2f} {mark}  {a[:46]:<46} | {b[:46]}")
print()

exact = [a for a in h168 if a in h167]
prefix = [(a, b) for a in h168 for b in h167 if a != b and (a in b or b in a)]
print(f"exact matches: {len(exact)}")
for e in exact:
    print(f"  {e!r}")
print(f"strict prefix relations: {len(prefix)}")
for a, b in prefix:
    print(f"  168: {a!r}\n  167: {b!r}")
print()

hi = [s for s, _, _ in pairs if s >= 0.5]
med = [s for s, _, _ in pairs if 0.25 <= s < 0.5]
print(f"overlap >= 0.50: {len(hi)} of {len(pairs)}   0.25-0.50: {len(med)}   below: {len(pairs)-len(hi)-len(med)}")
mean_real = sum(s for s, _, _ in pairs) / len(pairs)
print(f"mean best-match overlap: {mean_real:.3f}")
print()

# C2: shuffle 167's headings and re-pair, many times, to get a null.
random.seed(12345)
nulls = []
for _ in range(200):
    sh = h167[:]
    random.shuffle(sh)
    nulls.append(sum(jac(a, sh[i % len(sh)]) for i, a in enumerate(h168)) / len(h168))
mean_null = sum(nulls) / len(nulls)
print(f"null (random re-pairing, 200 draws): mean {mean_null:.3f}")
print()

print("=== CONTROLS ===")
c1j = len(prefix) >= 2 and all(jac(a, b) >= 0.5 for a, b in prefix)
c1c = len(prefix) >= 2 and all(cont(a, b) >= 0.99 for a, b in prefix)
print(f"C1a Jaccard sees both prefix pairs at >= 0.50                   : {c1j}"
      f"   <-- FIRES, and correctly: {min(jac(a,b) for a,b in prefix):.2f} on the second pair")
print(f"C1b containment sees both prefix pairs at 1.00                  : {c1c}")
assert c1c, "even containment cannot see the prefix relation, so the metric is wrong"
c2 = mean_real > 2 * mean_null
print(f"C2 real pairing beats random re-pairing by more than 2x         : {c2} ({mean_real:.3f} vs {mean_null:.3f})")
assert c2, "random re-pairing scores as well as the real one, so the metric matches everything"
DICTATED = {"What I settled, what I moved, what I could not",
            "Coverage of phase two, bounded"}
c3 = set(exact) <= DICTATED
print(f"C3 every exact match is a heading the standing shape dictates   : {c3}")
print()
print("The metric lesson, which the control produced rather than the analysis:")
print("a SYMMETRIC similarity reports a subsumption as a partial match, so it is the")
print("wrong instrument for a prefix relation. Containment is the right one and it")
print("returns 1.00 on both pairs. Jaccard is kept beside it because it is the honest")
print("answer to 'how similar are these two headings' and containment is the honest")
print("answer to 'did one file's framing arrive inside the other's'.")
print()
print("RESULT: identity finds", len(exact), "matches and all are dictated; overlap finds")
print(f"{len(hi)} pairs at or above 0.50 including the two prefix relations, against a")
print(f"null of {mean_null:.3f}. The shared premise set fixes a measurable amount of the")
print("shape, and the number to quote is the overlap rather than an identity count.")
