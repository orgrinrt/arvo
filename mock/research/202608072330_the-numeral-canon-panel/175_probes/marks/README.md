# P1: is the candidate's kind-marking convention leading or trailing

```sh
python3 marks.py > marks.out
```

`174`'s A3 rests entirely on the convention being **trailing**, and `174` states plainly that it
inferred that from clauses 5 and 7 and that the legend does not state it, so "if the convention is
leading then clause 4's `[measured]` attaches to the no-threshold sentence instead and the defect is
worse rather than absent".

## Result: the trailing convention is forced, not inferred

| clause | opens with a mark | total marks | non-opening marks |
|---|---|---|---|
| 1, 2, 3, 6, 8, 9, 10, 11, 12 | yes | 1 | 0 |
| 4 | yes | 3 | **2** |
| 5 | yes | 2 | **1** |
| 7 | yes | 3 | **2** |

**Clause 4's final mark `[theorem]` and clause 5's final mark `[measured]` each have no successor
sentence: the text after them is empty.** Under a leading convention those marks would mark nothing,
which is not a possible reading. So the convention is trailing and `174`'s A3 holds unconditionally
rather than "either way a mark is wrong".

Controls: C-A three clauses carry a non-opening mark, so there is a convention question (must be > 0);
C-B nine carry an opening mark only, so "opening mark for the primary kind" is also part of the
convention (must be > 0); C-C the legend was searched for the words trailing, opening mark, attaches
to, preceding sentence, and returned **False**, which is the negative claim with its place and search.

`holds for: 173 as committed at this branch, the mark vocabulary theorem/measured/enumeration/normative/argument`
