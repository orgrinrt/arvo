# p2, first run, kept because it measured the wrong thing

`p2_frontier_BROKEN_FIRST_RUN.py` and its output are the first attempt at the
compile-time law-validation frontier. They are wrong, they are kept, and the way
they are wrong is itself the finding that produced the corrected instrument.

**The defect: the law check returned early on the first counterexample.** So for
every arity at which signed saturating addition is *not* associative (3 and up),
`law_holds()` found a violation within the first few tuples and returned, and the
const evaluator never enumerated the domain the probe claimed it was enumerating.
The headline output reads `arity=3 width=16 tuples=2^48 accept 1.00s`, which is
not a measurement of anything: 2^48 tuples were never visited.

At arity 2 the law is true, so no early return was available, the evaluator did
exhaust the domain, and that row is the only honest one in the table. It refuses
at W = 9, 2^18 tuples.

**Second defect**, visible in the same output: the tuple count was carried in a
`u64` linear index, so every configuration past 2^63 failed at parse time with
`literal out of range for u64` and was reported as a frontier point. Four of the
six "first refused" rows in the summary are that, not a const-eval wall.

This is the "setup that helps" failure the workspace's test gate names: the
instrument fed the implementation exactly the inputs on which the expensive path
is never entered, so the expensive path was never measured, and every number it
printed looked reasonable.

The corrected instrument counts violations rather than returning early, so the
enumeration is unconditional, and generates a nest of K loops rather than a
linear index, so no counter overflows.
