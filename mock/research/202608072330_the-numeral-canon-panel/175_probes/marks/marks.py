#!/usr/bin/env python3
"""P1. Is the candidate's kind-marking convention leading or trailing, and is it
consistent?

174's A3 rests entirely on the convention being TRAILING: under that reading
clause 4's bare [measured] attaches to the statability sentence, which 60
disclaims as not a measurement. 174 says the legend does not state the
convention and that it inferred it from clauses 5 and 7. If the convention is
LEADING, a different mark is wrong instead.

This enumerates every mark in the statement, its position within its clause, and
whether the clause opens with one, so the convention is measured rather than
inferred from two examples.

CASES THAT MUST FAIL
  C-A  at least one clause must carry a NON-opening mark, else there is no
       convention question to settle and 174's A3 is about nothing.
  C-B  at least one clause must carry ONLY an opening mark, else every mark is
       trailing and "opening mark for the clause's primary kind" is not the
       convention either.
  C-C  the legend must be searched for a statement of the convention and the
       search reported, since "the legend does not state it" is a negative claim
       about a place.
"""
import re, os, sys

os.chdir(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..'))
SRC = '173_leroy_the_canon_candidate_for_the_chain.md'
txt = open(SRC, encoding='utf-8').read()

# The statement is the blockquote block in section 4.
sec4 = txt.split('## 4. The statement')[1].split('\n**Permanence.**')[0]
quoted = "\n".join(l[2:] if l.startswith('> ') else ('' if l.strip() == '>' else l)
                   for l in sec4.splitlines() if l.startswith('>'))

# split into clauses on the "**N. [kind]**" opener
parts = re.split(r'\n(?=\*\*\d+\.\s)', "\n" + quoted)
parts = [p for p in parts if re.match(r'\s*\*\*\d+\.\s', p)]

MARK = re.compile(r'\[(theorem|measured|enumeration|normative|argument)[^\]]*\]')
print(f"{'clause':>7} {'opens with a mark':>18} {'total marks':>12} {'non-opening marks':>18}  positions (chars into clause)")
open_only = 0
has_trailing = 0
rows = []
for p in parts:
    n = re.match(r'\s*\*\*(\d+)\.', p).group(1)
    ms = list(MARK.finditer(p))
    head = p[:60]
    opens = bool(MARK.search(head))
    trailing = [m for m in ms if m.start() > 60]
    if opens and not trailing:
        open_only += 1
    if trailing:
        has_trailing += 1
    rows.append((n, opens, len(ms), len(trailing)))
    print(f"{n:>7} {str(opens):>18} {len(ms):>12} {len(trailing):>18}  {[m.start() for m in ms]}")

print()
print("--- clause 4, the disputed one, with its marks located in context ---")
c4 = next(p for p in parts if p.strip().startswith('**4.'))
for m in MARK.finditer(c4):
    before = " ".join(c4[max(0, m.start()-170):m.start()].split())[-150:]
    after = " ".join(c4[m.end():m.end()+150].split())[:120]
    print(f"  mark {m.group(0)!r} at char {m.start()}")
    print(f"    preceded by: ...{before}")
    print(f"    followed by: {after}...")
    print()

print("--- C-C: does the legend state the convention? ---")
legend = txt.split('## 4. The statement')[1].split('>')[0]
print("  legend text:", " ".join(legend.split())[:400])
states = any(w in legend.lower() for w in ('trailing', 'opening mark', 'attaches to', 'preceding sentence'))
print(f"  legend states the convention: {states}")

print()
print(f"C-A  clauses with a non-opening mark: {has_trailing}  (must be > 0)")
print(f"C-B  clauses with an opening mark only: {open_only}  (must be > 0)")
print(f"C-C  the legend search was run and returned: {states}")
# ---------------------------------------------------------------------------
# P1b, appended after the first run: is the trailing convention FORCED rather
# than inferred? A mark with no successor sentence cannot be a leading mark.
# ---------------------------------------------------------------------------
print()
print("--- P1b: is the trailing reading forced? ---")
forced = []
for p in parts:
    n = re.match(r'\s*\*\*(\d+)\.', p).group(1)
    ms = list(MARK.finditer(p))
    if not ms:
        continue
    last = ms[-1]
    tail = p[last.end():].replace('*', '').strip()
    if last.start() > 60 and tail == '':
        forced.append((n, last.group(0)))
    print(f"  clause {n}: last mark {last.group(0):<15} text after it: {tail[:60]!r}")
print()
if forced:
    for n, m in forced:
        print(f"  FORCED: clause {n}'s final mark {m} has NO successor sentence, so it cannot")
        print(f"          be a leading mark. The convention is trailing, not inferred.")
else:
    print("  Not forced by this argument: every final mark has a successor.")

sys.exit(0 if (has_trailing > 0 and open_only > 0) else 1)
