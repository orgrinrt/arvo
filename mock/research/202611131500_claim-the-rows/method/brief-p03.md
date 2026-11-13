# Reader brief: which unratified rows of arvo's registry the written history backs

Worktree: `/home/user/.worktrees/reader-b` (repo `orgrinrt/arvo`, detached at `0dc63874`). Work only there. Write one file, `/tmp/claude-0/-home-user/aa9282d6-0277-5105-ab7d-a32d329e33e6/scratchpad/claim/out-p03.md`, and commit nothing.

## The question

For each row in your lot (`/tmp/claude-0/-home-user/aa9282d6-0277-5105-ab7d-a32d329e33e6/scratchpad/lots/lot-p03.txt`, one `namespace::slug` per line), say which of three classes it is in, with the evidence. No class is preferred, and a lot may fall wholly in one.

- **a**: the written history shows the claim is the lead designer's own. One of: his own words saying it (a quote), a design he wrote that says it, or a written record of him ratifying it. Give the file path, the line range and the passage verbatim.
- **b**: the written history shows the claim is not backed by him, or it is against a `ruling` row at `rung = "ratified"` or against the record. Give what it conflicts with (`ruling::<slug>`, or path and lines), the conflicting passage verbatim, and two to four real options for settling it.
- **c**: neither of the two can be shown. Say in one line what was searched.

A row can be split: where one sentence of its `says` is class a and another is not, say which sentence and class each.

## What the row is checked on

- The row's `says` (and `instead`, where it has one), in full. A passage about the same subject that does not say what the row says is not evidence for class a.
- Who wrote the passage. His words are marked as quotes or sit in files of his own; a panel member's file restating him is that member's reading, and the passage it quotes is what counts.
- A passage found is opened and read around, twenty lines each side, before it is cited.

## Where the written history is

- `mock/research/202608072330_the-numeral-canon-panel/`: the files named `<n>_op_*.md` and `INTENTS.md` carry his words as the panel recorded them; the other files are panel members'.
- `mock/research/202607301300_formalization-spec-panel/`, and the other directories and files under `mock/research/`.
- `mock/design_rounds/`: design rounds as they are and were. The oldest are for a crate layout that is removed; a design there is evidence of who designed what, not of what the tree holds.
- `/home/user/.data/op-responses/`: captures of his own words across the workspace; the files with `arvo`, `panel`, `canon`, `numeral` or `ruling` in the name first.
- `mock/registry/ruling.toml`: each row's `quote` field is his exact words; `provenance` names where they are written in full.

Search with `grep -rn` over those roots in the words the passage would be written in, not only the row's own words. TOML strings break phrases over `\` at line ends; join them first (`perl -0pe 's/\\\n\s*//g' <file>`). Do not read a whole directory; search, then read the hits.

## Authority

- The governing canon is `mock/registry/*.toml`, declared as `canon_paths` in `mockspace.toml`. `ruling` rows at `rung = "ratified"` govern. `stated` is his direction. `proposal` rows are claims a panel made. `mock/research/` is argument and record.
- Authority comes from a human having ratified, not from a file looking finished. Whether a row should exist at all, and whether it sits in the right namespace, are both challengeable; say so in a line where it applies.
- The crates under `mock/crates/` are claims to test, not context.
- Where a row in your lot contradicts a ratified ruling, that is class b; do not weigh the two.

## Output

`/tmp/claude-0/-home-user/aa9282d6-0277-5105-ab7d-a32d329e33e6/scratchpad/claim/out-p03.md`, one block per row, in lot order, nothing before the first block:

```
## <namespace>::<slug>
class: a | b | c | split
evidence: <path>:<first line>-<last line>
passage: "<verbatim>"
conflict: <ruling::slug or path:lines>            (class b only)
options: 1. ... 2. ... 3. ...                      (class b only)
note: <one line, only where needed>
```

End the file with three counts, a, b and c. A row whose search found nothing is class c; saying so is a complete answer.

## Rules that bind here

- No python. Shell and perl only.
- Read only; write the one output file. No `cargo` runs, no git writes.
- No AskUserQuestion. Where blocked, write what blocked it at the top of the output file and return.
