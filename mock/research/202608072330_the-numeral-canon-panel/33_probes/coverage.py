#!/usr/bin/env python3
"""Coverage instrument: what a member file flags, and whether the register carries it.

Built after the register missed a flagged-as-op's question TWICE, in two
independent passes, each of which checked coverage by reading. Reading is what
failed, so this measures instead.

High recall on purpose. It is meant to over-report and never to miss: every hit
is a candidate a human or a later member confirms or dismisses. A precise
extractor that silently drops one line would reproduce the defect it exists to
catch.

Usage:  python3 33_probes/coverage.py [--verbose]
Exit:   1 if any flagged item has no plausible echo in the register.
"""
import re, sys, pathlib

HERE = pathlib.Path(__file__).resolve().parent.parent
REGISTER = HERE / "OPTIONS.md"
DROPLIST = HERE / "DROPLIST.md"

# Phrases a member uses when it hands something to op or names it undetermined.
# Each is a claim the register is supposed to carry.
FLAGS = [
    r"is op'?s\b",
    r"genuinely undetermined",
    r"undetermined, and is op'?s",
    r"question for op\b",
    r"for op rather than",
    r"only op can",
    r"op'?s call\b",
    r"asked for a second read",
    r"owed a second read",
    r"no second read has run",
]
FLAG_RE = re.compile("|".join(FLAGS), re.I)

STOP = set("""the a an and or of to in on for with that this those these is are was were be been
it its as at by from not no nor but if then than which who whom whose what when where how why
we our us you your they their them he she his her i me my one two three four five six seven eight
nine ten first second third fourth own same other another each every all any both few more most
some such only just also very can may might must should would could will shall do does did done
here there now then thus so because since while during before after above below up down out off
over under again further once here's op op's rather into within without across against between""".split())

def split_claims(line):
    """Split a flagged line into independently-testable claims.

    A member writing "what is undetermined: whether X; and whether Y" has named
    two things, and the register can carry one without the other. Splitting on
    the connectives that join such claims is what makes a half-carried
    conjunction visible.
    """
    body = re.sub(r"^\s*[-*]?\s*\**[^:]{0,80}:\*{0,2}", "", line, count=1)
    parts = re.split(r";|\band whether\b|\bor whether\b|\band that\b", body)
    return [p for p in parts if len(p.strip()) > 20] or [body]

def content_words(text, n=40):
    """Distinctive words from a line, longest first: the search keys."""
    words = re.findall(r"[A-Za-z_][A-Za-z0-9_'-]{3,}", text.lower())
    seen, out = set(), []
    for w in words:
        w = w.strip("'-")
        if w in STOP or len(w) < 4 or w in seen:
            continue
        seen.add(w)
        out.append(w)
    out.sort(key=len, reverse=True)
    return out[:n]

def main():
    verbose = "--verbose" in sys.argv
    if not REGISTER.exists():
        print("no OPTIONS.md", file=sys.stderr)
        return 2
    reg = REGISTER.read_text().lower()
    drop = DROPLIST.read_text().lower() if DROPLIST.exists() else ""
    haystack = reg + "\n" + drop

    members = sorted(
        p for p in HERE.glob("[0-9][0-9]_*.md")
        if not p.name.startswith("00_")
    )

    missing, checked = [], 0
    for m in members:
        text = m.read_text()
        # PARAGRAPHS, not lines. These files are hard-wrapped at about 100
        # columns, so a claim routinely spans three physical lines and only the
        # first carries the flag phrase. A line-based scan sees the flag and the
        # first fragment, and never sees the rest of the sentence. That is why
        # the first two cuts of this instrument could not catch `02:49-51`.
        offset = 1
        for para in re.split(r"\n\s*\n", text):
            nlines = para.count("\n") + 1
            start = offset
            offset += nlines + 1
            if not FLAG_RE.search(para):
                continue
            flat = " ".join(para.split())
            if len(flat) < 25:
                continue
            for clause in split_claims(flat):
                keys = content_words(clause)
                if len(keys) < 3:
                    continue
                checked += 1
                # Score on ABSENT DISTINCTIVE terms, not on average overlap.
                # A register of 800 lines about numerals contains "shapes",
                # "value", "denote" everywhere, so a genuinely missing claim
                # still scores 85% on word overlap. The absent half of
                # `02:49-51` scored exactly that, with only `order's` and
                # `amended` missing. The distinguishing vocabulary is the
                # signal; the shared vocabulary is noise.
                absent = [k for k in keys if k not in haystack]
                if len(absent) >= 2:
                    ratio = 1 - len(absent) / len(keys)
                    missing.append(
                        (m.name, start, ratio, clause.strip()[:150], absent[:8])
                    )
                elif verbose:
                    print(f"  ok  {m.name}:{start}  {clause.strip()[:60]}")

    print(f"members scanned : {len(members)}")
    print(f"flagged lines   : {checked}")
    print(f"weak echo       : {len(missing)}")
    if missing:
        print("\nFlagged in a member, weak or no echo in the register or droplist:\n")
        for name, ln, ratio, text, keys in sorted(missing, key=lambda r: r[2]):
            print(f"{name}:{ln}  echo {ratio:.0%}")
            print(f"    {text}")
            print(f"    absent: {', '.join(keys)}\n")
    return 1 if missing else 0

if __name__ == "__main__":
    sys.exit(main())
