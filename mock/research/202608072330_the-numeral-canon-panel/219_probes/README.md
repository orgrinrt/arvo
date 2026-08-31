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

## Added in the reply to seat 220

`reply_to_220.rs` and `reply_to_220.out` correct two counts in the blind census
and independently reproduce two of seat 220's claims. Run it the same way. The
census is left exactly as committed, defects included, because a corrected
measurement and a rewritten one are different things and only one of them can be
audited.

What it establishes: the universal count is 41 rather than the census's 38, the
multi-binding count is 6 rather than 5, seat 220's table of the eight
`sentence_kind = "theorem"` rows reproduces exactly on a separately written
reader, and the parameterised span `fraction_width: in 0..=W-1` occurs three
times rather than the five its prose claims.

`vacuity_spike/` answers the question seat 220 left open with a stated predicate.
It found that eight of the crate's thirty finding-returning arms say nothing
about an empty registry; this runs all thirty.

```
cd vacuity_spike && cargo run -q
```

Result: 21 of 30 say nothing on either an empty input or the real one, so no test
written over those 21 can tell an empty registry from a clean one. Nine fire on
the real corpus and would notice.

Its `Cargo.toml` carries an empty `[workspace]` table so the parent mock
workspace does not absorb it. `target/` and `Cargo.lock` are not committed.

**Both of this spike's controls fired on defects of mine before it produced a
number, and both are worth knowing about.** The first version called
`arvo_checks::canon()` and got zero rows in silence, which is seat 220's finding
reproducing itself against somebody who had already read it. The second version
reported all five directory arms vacuous, because `corpus::panel_dir()` is
crate-relative in the same way and both of my columns were naming a directory
that did not exist; three of those five actually fire. The whole-run control
caught the first and could not catch the second, which is why there is now a
per-column control as well.
