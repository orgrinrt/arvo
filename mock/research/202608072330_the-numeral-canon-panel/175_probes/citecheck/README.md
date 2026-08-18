# Citation check for 175, all four layers counted

```sh
python3 check.py > check.out
```

**23 citations, 0 not found, 2 of 2 negative controls caught.**

| layer | citations it was the first to find |
|---|---|
| L0 raw | 16 |
| L1 whitespace | 5 |
| **L2 markup** | **2** |
| L3 case | 0 |

L2 is load-bearing here for the first time in my own corpus: `171`'s check reported L2 at zero, and the
difference is that this file quotes from `173`'s statement, which is a **blockquote with bolded terms
inside it**, and from `174`'s amendment headings. That is `174`'s own observation about why L2 is its
largest bucket, reproduced from a different seat.

L3 is zero here and was 1 in `174`. Exposure to the case layer depends on whether an author lifts a
quotation mid-sentence and lowercases its leading capital; I quote from sentence starts, so I do not.
**A zero on a layer is a fact about the quoting author, not about the layer**, which is why the report
prints a per-layer count rather than a pass or fail.

`holds for: the 23 citations enumerated in check.py, against 173, 174, 60, 167, 171 and the workspace
rule, at this branch, threads any`
