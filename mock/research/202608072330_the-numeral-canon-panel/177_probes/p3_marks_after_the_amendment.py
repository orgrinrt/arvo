#!/usr/bin/env python3
"""
177 P3. Does the marking convention 176 states classify every mark in the statement AS
AMENDED, including the mark 176 itself introduces?

THE CONVENTION, quoted from 176's amended legend:
  "A clause's opening mark gives its primary kind; a trailing mark attaches to the
   sentence immediately before it."

  175 proved the trailing reading FORCED from clauses 4 and 5, each of which ends with a
  mark having no successor text. This probe asks whether those are the only shapes. A mark
  that is neither at the start nor at the end has text after it, and "the sentence
  immediately before it" does not identify a referent when the mark sits between
  semicolon-separated clauses of one sentence.

METHOD
  Reuses 175's own mark pattern (which already lists `argument`) and its blockquote
  extraction, so a disagreement with 175's table is my defect and not a finding.
  Clauses 1, 2 and 5 as amended are taken from 176's quoted blocks. Clause 4 is NOT
  quoted in full anywhere in 176, only described, so it is synthesised here by applying
  176 section 3's stated change to 173's text; that synthesis is declared, not hidden.

NEGATIVE CONTROLS, declared before the run
  M1. Every clause must be found to open with a mark, 12 of 12, matching 175's table.
      A lower number means my extractor is short and no count below is usable.
  M2. Clauses 4 and 5 of the UNAMENDED statement must show a terminal mark with no
      successor text. That is 175's forced-convention result, re-armed.
  M3. The mid-clause class must be non-empty in the UNAMENDED text. If it is empty
      before the amendment I have invented the class rather than found it.
"""
import re, os

D = os.path.dirname(os.path.abspath(__file__)) + "/.."
MARK = re.compile(r'\[(theorem|measured|enumeration|normative|argument)[^\]]*\]')

def statement_clauses(path, start, end):
    txt = open(os.path.join(D, path), encoding='utf-8').read()
    sec = txt.split(start)[1].split(end)[0]
    quoted = "\n".join(l[2:] if l.startswith('> ') else ('' if l.strip() == '>' else l)
                       for l in sec.splitlines() if l.startswith('>'))
    parts = re.split(r'\n(?=\*\*\d+\.\s)', "\n" + quoted)
    parts = [p for p in parts if re.match(r'\s*\*\*\d+\.\s', p)]
    return {int(re.match(r'\s*\*\*(\d+)\.', p).group(1)): " ".join(p.split()) for p in parts}

un = statement_clauses("173_leroy_the_canon_candidate_for_the_chain.md",
                       "## 4. The statement", "\n**Permanence.**")

t176 = open(os.path.join(D, "176_leroy_the_candidate_revised_against_two_signatures.md"),
            encoding='utf-8').read()
def quoted_block(n):
    m = re.search(r'^> \*\*%d\.\s\[.*?(?=\n\n)' % n, t176, re.S | re.M)
    if not m: return None
    return " ".join(" ".join(l[1:].strip() for l in m.group(0).split("\n")
                             if l.startswith(">")).split())

am = dict(un)
taken, synthesised = [], []
for n in (1, 2, 4, 5):
    q = quoted_block(n)
    if q:
        am[n] = q; taken.append(n)
    else:
        synthesised.append(n)
# clause 4: apply 176 section 3's described change to 173's text
if 4 in synthesised:
    c4 = un[4]
    assert c4.count("**[measured]**") == 1, "clause 4's single trailing [measured] not found"
    am[4] = c4.replace("**[measured]**",
                       '**[argument]** ("That is a statability argument, not a benchmark", `60:210`)')

def classify(text):
    res = []
    for m in MARK.finditer(text):
        pre, post = text[:m.start()], text[m.end():]
        if len(re.sub(r'[^A-Za-z]', '', pre)) <= 3:
            kind = "opening"
        elif re.sub(r'[^A-Za-z0-9]', '', post) == "":
            kind = "terminal"
        else:
            # does the mark sit at a SENTENCE boundary (the convention's case) or
            # INSIDE a sentence between a semicolon or comma (the case it does not name)?
            tail = post.strip()
            tail = tail[2:].strip() if tail.startswith("**") else tail
            kind = "mid, sentence boundary" if (tail[:1].isupper() or
                    re.sub(r'[^A-Za-z]', '', pre)[-0:] == "" ) and not tail.startswith((";", ",")) \
                   else "MID-SENTENCE"
        res.append((m.group(0), kind, post.strip()[:52]))
    return res

print(f"amended clauses taken from 176's quoted blocks : {taken}")
print(f"amended clauses synthesised from 176's prose    : {synthesised}")
print()
for label, src in (("UNAMENDED (173)", un), ("AS AMENDED (173 + 176)", am)):
    opens = terminal = mid = 0
    mids = []
    for n in sorted(src):
        cs = classify(src[n])
        if cs and cs[0][1] == "opening":
            opens += 1
        for mk, kind, post in cs:
            if kind == "opening":  opens_dummy = 0
            terminal += kind == "terminal"
            mid += kind == "MID-SENTENCE"
            if kind.startswith("MID") or kind.startswith("mid"):
                mids.append((n, mk, kind, post))
    print(f"=== {label} ===")
    print(f"  clauses {len(src)}   clauses opening with a mark {opens}   "
          f"terminal marks {terminal}   intra-sentential marks {mid}")
    for n, mk, kd, post in mids:
        print(f"    clause {n:>2}  {mk[:44]:<44} {kd:<22} followed by: {post!r}")
    print()

o_un = sum(1 for n in un if classify(un[n]) and classify(un[n])[0][1] == "opening")
term45 = [n for n in (4, 5) if any(k == "terminal" for _, k, _ in classify(un[n]))]
mid_un = sum(1 for n in un for _, k, _ in classify(un[n]) if k == "MID-SENTENCE")
mid_am = sum(1 for n in am for _, k, _ in classify(am[n]) if k == "MID-SENTENCE")
print("CONTROLS")
print(f"  M1 every clause opens with a mark         : {'PASS' if o_un == len(un) else 'FAIL'}  ({o_un} of {len(un)})")
print(f"  M2 clauses 4 and 5 have terminal marks    : {'PASS' if term45 == [4, 5] else 'FAIL'}  {term45}")
print(f"  M3 the intra-sentential class exists before 176 : {'PASS' if mid_un > 0 else 'FAIL'}  ({mid_un})")
print()
print("READING")
print("  Clause 4's amended text is NOT quoted anywhere in 176, only described, so its mark")
print("  position depends on where a reader puts the quoted qualifier. My synthesis put it")
print("  after the mark, which makes it read intra-sentential. THAT IS MY ARTIFACT and is")
print("  excluded from the counts below; a finding resting on my own splice would be the")
print("  scope-not-mechanism class this unit named.")
q_un = sum(1 for n in un if n != 4 for _, k, _ in classify(un[n]) if k == "MID-SENTENCE")
q_am = sum(1 for n in am if n != 4 for _, k, _ in classify(am[n]) if k == "MID-SENTENCE")
print(f"  excluding clause 4: intra-sentential marks before {q_un}, after {q_am}.")
print(f"  intra-sentential marks before the amendment: {mid_un}; after: {mid_am}.")
print("  The convention as worded names two positions, opening and trailing. Every")
print("  mid-clause mark is a third position it does not classify.")
