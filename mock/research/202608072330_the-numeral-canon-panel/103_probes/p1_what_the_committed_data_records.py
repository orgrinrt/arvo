#!/usr/bin/env python3
"""p1. What the committed CSV corpus actually records, column by column.

The question this probe exists for: `102` claims every committed region is
answer-equivalent, meaning all arms compute one value. Before asking whether
that is true, ask a prior question that costs one pass over the data: does the
committed corpus record an answer at all?

The harness CSV schema has a `digest` column, a `score` column and an
`input_tag` column. If `digest` is populated and equal across the arms of a
family, answer-equivalence is a measured property of the committed artifact and
can be checked without reading a line of variant source. If it is not
populated, the corpus is silent about answers and any claim about answers is a
claim about the code that produced the corpus, not about the corpus.

Run from `mock/benches/`.
"""

import csv
import collections
import glob
import os
import sys


def main() -> int:
    here = os.path.dirname(os.path.abspath(__file__))
    benches = os.path.abspath(os.path.join(here, "..", "..", "..", "benches"))
    if not os.path.isdir(benches):
        print("bench directory not found at", benches)
        return 1
    os.chdir(benches)

    files = sorted(glob.glob("*.csv"))
    print("corpus root:", benches)
    print("csv files:", len(files))

    header_shapes = collections.Counter()
    rows_total = 0
    per_column = collections.defaultdict(collections.Counter)
    variants = set()
    families = collections.defaultdict(set)

    for f in files:
        with open(f, newline="") as fh:
            reader = csv.DictReader(fh)
            header_shapes[tuple(reader.fieldnames or ())] += 1
            for row in reader:
                rows_total += 1
                for k, v in row.items():
                    per_column[k][v] += 1
                variants.add(row.get("variant"))
                families[f].add(row.get("variant"))

    print("rows:", rows_total)
    print("distinct header shapes:", len(header_shapes))
    for shape, n in header_shapes.items():
        print("  header used by", n, "files:")
        print("   ", ",".join(shape))

    print()
    print("per-column cardinality over the whole corpus:")
    for col in sorted(per_column):
        vals = per_column[col]
        card = len(vals)
        top = vals.most_common(3)
        constant = " CONSTANT" if card == 1 else ""
        print(f"  {col:14s} distinct={card:6d}{constant}  top={top}")

    print()
    answer_cols = ["digest", "score", "input_tag"]
    print("the columns that could carry an answer:")
    for col in answer_cols:
        vals = per_column.get(col, collections.Counter())
        card = len(vals)
        only = list(vals)[0] if card == 1 else None
        verdict = (
            f"CARRIES NOTHING (single value {only!r} in all {rows_total} rows)"
            if card == 1
            else f"carries {card} distinct values"
        )
        print(f"  {col:14s} {verdict}")

    print()
    print("distinct arms named anywhere in the corpus:", len(variants))
    multi = sum(1 for f, vs in families.items() if len(vs) > 1)
    print("csv files naming more than one arm:", multi, "of", len(files))

    digest_card = len(per_column.get("digest", {}))
    print()
    if digest_card <= 1:
        print("VERDICT: the committed corpus records no per-arm answer.")
        print("  Answer-equivalence is therefore not a property this data")
        print("  attests, in either direction. It can only be a claim about")
        print("  the variant sources, or about the convention they follow.")
    else:
        print("VERDICT: the corpus records per-arm digests; equivalence is checkable here.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
