# 229_probes

What each file is, and which are mine.

**Copied from `183_probes` so the reproduction is re-runnable here**, unmodified:
`span_verdicts.sh`, `unblock_value.sh`, `axis_census.sh`, `split_predicate.awk`.
Their outputs under `repro_*.out` are this run's, and the diffs against the
committed ones are recorded in `reproduction.txt`.

**Mine:**

- `phrase_context.sh` / `context_*.out`: prints the whole predicate span a
  ranked phrase was cut out of, which is the only thing that decides whether
  the phrase names an axis. Controls P1 (a phrase nobody wrote prints nothing)
  and P2 (`threads` prints 59, matching the census tally, so this extractor and
  the ranking's agree).
- `blocked_by_family.sh` / `.out`: the row-level demand, re-scored against
  today's registry. Control B1 reproduces the committed probe row's tally
  exactly; B3 clears 5 with the real declared set and 0 with an empty one.
- `warrant_usage.sh` / `.out`: whether any committed predicate entry uses the
  ratified warrant marker. 527 entries, 19 axes, zero tokens.
- `normative_escape.sh` / `.out`: **its N2 control FAILED** and the failing run
  is kept as `normative_escape_first_attempt.out`. The arm measures a real but
  different set from the one N2 names, so it is relabelled rather than repaired.
- `normative_escape2.sh` / `.out`: the narrower arm written after that failure,
  with its own three controls, all passing.
- `compile_time_spans.sh` / `.out`: rows whose claim is about const evaluation
  or staging, and what their predicate says.
