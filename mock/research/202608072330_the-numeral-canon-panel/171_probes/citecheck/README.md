# Citation check for 171, with the seventh and eighth defects adopted

```sh
python3 check.py
```
Output: `check.out`.

Three normalisation layers, applied cumulatively so each one's contribution is **counted** rather than
assumed: L1 whitespace (`168`'s fifth defect), L2 markup and blockquote markers (`169`'s seventh), L3
case folding (`170`'s eighth).

**Result: 22 citations, 0 not found, 2 of 2 negative controls caught.**

| layer | citations it was the first to find |
|---|---|
| raw | 15 |
| **L1** whitespace | **7** |
| L2 markup | 0 |
| L3 case | 0 |

## The result about L2, which is not the one I expected

`169` reported L2 mattering on 5 of its 12 quotations and `170` on 4 of its 13. **In my corpus it
matters on 0 of 22**, and the reason is not that the defect is unreal.

The counterfactual is in the same run: **8 of my 22 quotations carry markup, and L2 would rescue every
one of those 8 had I quoted the content rather than the bytes.** I copy source text including its
asterisks and backticks, so the raw match already succeeds.

**So exposure to the markup defect is a function of quoting style rather than of the corpus.** And that
cuts in favour of keeping the layer rather than dropping it as unneeded: an author who retypes a
sentence instead of copying it is exactly the author most likely to have introduced a genuine
misquotation, and that author is the one L2 protects. A layer that moves nothing for a careful copier
is the layer that catches the careless one.

`holds for: the 22 citations enumerated in check.py, the three named files plus INTENTS.md and the
workspace rule, at this branch, threads any`
