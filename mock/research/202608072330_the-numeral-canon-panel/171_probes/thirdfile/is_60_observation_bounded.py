#!/usr/bin/env python3
"""P4. Is 60's definition observation-bounded at all?

169 1.2 and 170 8 both treat 167, 168 and 60 as three files whose DEFINITIONS
turn on observability, and 170 concludes the convergence may be "one instance
wearing three hats". That is a checkable claim about what 60's definition says.

An earlier shell version of this probe found nothing in 60 because its defining
sentence wraps mid-phrase ("**A\nchain is a composition..."), which is exactly
the defect 169 section 9 and 170 section 15 catalogued. This version normalises
whitespace and strips markup before matching, per 169's seventh defect and 170's
eighth.

CASES THAT MUST FAIL
  C-I  167 and 168 must MATCH the observation pattern. A pattern that finds
       nothing anywhere is a bad pattern, not a fact about 60.
  C-J  60 must match SOME definitional pattern, else it has no definition and
       the comparison is malformed.
  C-K  The normalisation must be load-bearing: at least one match must be found
       ONLY after normalising. Otherwise this version is the shell version.
"""
import re, os, sys, glob

os.chdir(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..'))

def norm(t):
    t = re.sub(r'^\s*>\s?', ' ', t, flags=re.M)   # blockquote markers
    t = t.replace('**', '').replace('`', '')       # emphasis and code ticks
    return " ".join(t.split())

FILES = {
    '60':  '60_stam_the_chain_derived_cold.md',
    '167': '167_rompf_the_chain_derived_cold.md',
    '168': glob.glob('168_*.md')[0],
}

DEFN = {
    '60':  'A chain is a composition of exact operations together with a schedule of adaptation points',
    '167': 'the unobserved region: a maximal stretch of a computation in which no intermediate is named',
    '168': 'A chain is a maximal run of operations whose intermediates are not observable',
}

OBS = re.compile(r'observ|unobserved|not named by anyone', re.I)

raw, normed = {}, {}
for k, f in FILES.items():
    raw[k] = open(f, encoding='utf-8').read()
    normed[k] = norm(raw[k])

print("--- each file's own defining sentence, located after normalisation ---")
found_only_after_norm = 0
for k in ('60', '167', '168'):
    d = DEFN[k]
    in_raw = d in raw[k]
    in_norm = norm(d) in normed[k]
    if in_norm and not in_raw:
        found_only_after_norm += 1
    print(f"  {k:>4}: located = {in_norm}   (found in raw text without normalising = {in_raw})")
    print(f"        \"{d}\"")

print()
print("--- does each defining sentence itself use the observation vocabulary? ---")
for k in ('60', '167', '168'):
    hit = bool(OBS.search(DEFN[k]))
    print(f"  {k:>4}: {hit}")

print()
print("--- observation vocabulary density across each whole file ---")
for k in ('60', '167', '168'):
    n = len(OBS.findall(normed[k]))
    w = len(normed[k].split())
    print(f"  {k:>4}: {n:>3} occurrences in {w:>6} words  ({1000*n/w:.2f} per 1000)")

print()
print("--- C-J: 60 states a definition, of a different shape ---")
print(f"  'schedule of adaptation points' in 60: {normed['60'].count('schedule of adaptation points')} occurrences")
print(f"  'schedule of adaptation points' in 167: {normed['167'].count('schedule of adaptation points')}")
print(f"  'schedule of adaptation points' in 168: {normed['168'].count('schedule of adaptation points')}")

print()
ci = OBS.search(DEFN['167']) and OBS.search(DEFN['168'])
cj = 'schedule of adaptation points' in normed['60']
ck = found_only_after_norm > 0
print(f"C-I  167 and 168 defining sentences match the pattern: {bool(ci)}   (must be True)")
print(f"C-J  60 states a definition of another shape: {cj}   (must be True)")
print(f"C-K  normalisation was load-bearing on {found_only_after_norm} of 3 sentences: {ck}   (must be True)")
print()
print("VERDICT")
print(f"  60's defining sentence is observation-bounded: {bool(OBS.search(DEFN['60']))}")
sys.exit(0 if (ci and cj and ck) else 1)
