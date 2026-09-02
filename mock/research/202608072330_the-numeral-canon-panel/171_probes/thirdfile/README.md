# P4: is `60`'s definition observation-bounded

```sh
python3 is_60_observation_bounded.py
```
Output: `is_60_observation_bounded.out`.

`169` 1.2 establishes that none of `60`, `167`, `168` names the observability rule. `170` 8 then
speaks of "the definitional convergence between `167`, `168` and `60`" as possibly "one instance
wearing three hats". That second step assumes all three definitions are the same kind of definition.

## Result

| file | its own defining sentence | observation-bounded | observation vocabulary density |
|---|---|---|---|
| `60` | "A chain is a composition of exact operations together with a schedule of adaptation points" | **no** | 2 in 7,299 words, 0.27 per 1000 |
| `167` | "the unobserved region: a maximal stretch ... in which no intermediate is named" | yes | 22 in 14,690, 1.50 per 1000 |
| `168` | "A chain is a maximal run of operations whose intermediates are not observable" | yes | 26 in 16,986, 1.53 per 1000 |

**`60`'s definition is not observation-bounded.** It is scheduled: what a chain *contains*, not what
*bounds* one. `167` R7 had already recorded that split and neither `169` nor `170` had it in view.

## Controls, all clean

- **C-I** `167` and `168`'s defining sentences do match the pattern, so a zero at `60` is a fact about
  `60` rather than about the pattern.
- **C-J** `60` does state a definition, of another shape, twice.
- **C-K** normalisation was load-bearing on **2 of 3** defining sentences: `60`'s and `167`'s are found
  only after stripping markup and collapsing whitespace. An earlier shell version of this probe found
  nothing in `60` for exactly that reason, which is `169`'s seventh defect biting a third file.

`holds for: the three files as committed at this branch, the patterns in the source, threads any`
