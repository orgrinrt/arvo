# 219 probes

Two self-contained scanners over the committed registry, for seat 219's
derivation of the warrant marker's spelling. No dependencies, no build system,
and neither writes anything.

Run either from this directory:

```
rustc -O warrant_census.rs -o /tmp/wc && /tmp/wc ../../../registry
rustc -O values_side_admissibility.rs -o /tmp/vsa && /tmp/vsa ../../../registry
```

Run with no argument and each does its planted control alone, which is the mode
that shows the scanner can report a non-zero at all. Both refuse to run against
the registry when their control fails, because a census whose arms have never
returned anything but zero measures nothing and reads exactly like a clean
result.

`census.out` and `values_side.out` are the committed outputs of the two commands
above, control section included.

The compiled binaries are deliberately not committed. The sources build in one
command on the workspace's pinned toolchain and the outputs are here to diff
against.

## What each answers

`warrant_census.rs` measures the shape of the predicate corpus on the axes a
warrant marker would touch: how many entries there are, how the universal is
spelled, how many rows carry a universal and a bounded region together, whether
the three proposed tokens collide with anything already written, whether the
slug side is lexically clean enough to carry a marker, and how many entries a
given enforcement arm would fire on today.

`values_side_admissibility.rs` measures what it costs that the checker reads the
slug side of an entry and not the values side. Two questions: does the corpus
write a spelling its own axis declares inadmissible, and does any values side
bind more than one thing.

## Read the numbers with one correction

`values_side_admissibility`'s "binds an undeclared name" column reports five and
overstates. Three of the five are the left-hand-side extractor reading `and
arity`, `and W` and `and F` as names in entries of the form `X for the first
run, and Y for the second`. Those three are double-region entries rather than
undeclared axes. The honest split is two undeclared axes and three double
regions, and the multi-binding count of five is exact.
