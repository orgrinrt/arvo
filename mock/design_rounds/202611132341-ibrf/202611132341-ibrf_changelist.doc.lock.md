# Doc changelist: `land` corrected, and completed past the ends

**Date:** 2026-11-13
**Round:** `202611132341-ibrf`, job `arvo-format`.
**Topics consumed:** `202611132341-ibrf_topic.land-corrected-and-completed-past-the-top.md`, the only flat
topic on this branch and on every remote ref.
**Scope:** `mock/crates/arvo-format/DESIGN.md.tmpl`, `mock/crates/arvo-format/README.md.tmpl`.

## Canon

`proposal::wrapping_is_admitted_exactly_where_the_representable_set_is_one_progression`, provisional,
stamped by experts, with question `29bd8e5f1b6c` to op: wrapping exactly on one progression, refused at
compile time elsewhere; saturation on any bounded set, to the end a value is past. Its gap leaves what
saturation needs from the applied map at more than one magnitude to design, which is what the landing's
past-the-ends subsection now says. `ruling::never_a_runtime_check_and_one_lowered_path`, ratified: the
denominator of `Scaled` stops being checked at run time. The membership predicate is the crate's own
design (`Format`, the phase in units of the magnitude-zero quantum); the position at a magnitude is
derived from it.

## CHANGE: `Scaled`'s denominator is non-zero by type

The landing's second paragraph: the denominator is a `NonZeroU128`, so a point always names a ratio and
building one checks nothing. The zero-reads-as-zero sentence goes.

## CHANGE: the nearest-member argument names where it holds

"The finest grid in range is the nearest member" says the phase is one offset shared by every magnitude,
and that the rule reaches the nearest member where the slot range holds zero. A new paragraph gives the
gap a range not holding zero can leave (slots 8 to 15 at quanta 1 and 4), and the compile-time refusal
of such a format over more than one magnitude under `arvo_land_refuses_a_format_whose_slot_range_does_not_hold_ze`.

## CHANGE: the position at a magnitude reads the phase in its own unit

"The position at a magnitude" is rewritten from the membership predicate: `num/den radix^t - p/q
radix^b` with `b = BASE - s`, over one denominator by `c = max(0, -t, -b)`, `b` left out at a zero
phase. A floor past the slot index is past the range; a `D` past the limbs holds no position; a
numerator past them holds none unless provably past the index, which is stated with its bound.

## CHANGE: the walk's start bound is true for a negative shift

"Where the walk starts" takes `ceil(log2 radix)` for `t < 0`, states the phase-term condition the bound
needs (`b <= 0` at every magnitude skipped), and walks a falling law with a nonzero phase from its finest
magnitude.

## CHANGE: past the ends of the union

"What it does not land" becomes "Past the ends of the union": saturation and clamping land at the top
magnitude and `adapt` pins there; wrapping is refused at compile time off one progression, asked of the
set by `is_one_progression`; a magnitude-indexed one-progression set under wrapping is refused as not yet
admitted under `arvo_land_does_not_wrap_on_a_one_progression_set_over_severa`; the operations keep their
slope test, which is about their missing magnitude coordinate. The FIXME's ticket
`arvo_landing_a_value_on_a_magnitude_indexed_format_is_a_call` is no longer named.

## CHANGE: the owed tests and the surface

The oracle is written from the membership predicate; the member sweep, the review's two points, the ends
under saturation and clamping, wrapping on a constant quantum, `is_one_progression`'s cases with the
degenerate laws as the slope test's miss, and three compile-fail pairs are owed. The IEEE suites land the
overflow region through `land` too. The surface adds `is_one_progression`.

## CHANGE: the standing claims the wrapping row falsifies

Addition's paragraph on the magnitude-indexed family no longer says no row defines wrapping there; it
says the row does and the missing coordinate is what holds the family back. The product's paragraph and
the README's sentence on floating point formats say the same.

## Comparison to the first review

Findings 1, 2, 4 and 6 of `mock/research/202611132340_review-arvo-format-1/REVIEW.md` are answered here
in the design; 5 and 8 are answered in source and the generated documents; 3 is answered in the topic as
this author's reading, for the second review; 7 is ticket `ee02b2a4561b`.
