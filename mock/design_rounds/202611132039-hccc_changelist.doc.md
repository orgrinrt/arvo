# Doc changelist: the quotient, the conversion and the door at a host width

**Date:** 2026-11-13
**Round:** 202611132039-hccc, branch `work/arvo-values`, PLAN step 7 and stack audit finding 1.
**Topics consumed:** `202611132039-hccc_topic.the-quotient-the-conversion-and-the-door-at-a-host-width.md`,
the only flat topic on this branch or on `origin/dev`.
**Sketches:** none new. The audit's timing of the door is
`mock/research/202611131215_stack-audit/AUDIT.md` finding 1, an ad hoc spike and cited as that.
**Scope:** `mock/crates/arvo-numeral/DESIGN.md.tmpl`, `mock/crates/arvo-numeral/README.md.tmpl`. No
change to `arvo-format` and no registry file.

## Canon

`ruling::the_format_spine_is_canon`: the quotient and the conversion are the format's `div` and
`convert` by equality on every member.
`ruling::never_a_runtime_check_and_one_lowered_path`: the host door validates nothing; a host integer
that names no member is completed by the declared policy, and `index` is refused at build where it
would truncate.
`ruling::the_overflow_panic_is_permitted_and_bounded` and
`ruling::the_debug_abort_is_configured_by_the_consumer`: the abort under `Abort` at `div`, `convert`
and `from_index`.
`ruling::behaviour_is_stated_per_declared_signature_and_the_premise_dissolves`: the door names the host
integer the consumer asks for and never the carrier.
`proposal::a_value_is_the_sole_occupancy_placement_of_a_declared_signature`, at one expert and on a
provisional stamp: built on as provisional and not promoted.

## CHANGE: `arvo-numeral`, the quotient and the conversion

File: `mock/crates/arvo-numeral/DESIGN.md.tmpl`.

The canon list at the top names the quotient and the conversion beside the four ring operations.
`## What this crate is` says the same in its contract paragraph, against the format's `div` and
`convert`.

`## The operations` gains two parts after the product.

`div(by)` takes `Divisor<S::Format>`, the format's type, re-exported, and returns the quotient, equal
to the format's `div` on every member and every divisor, with `fits_div` its verdict, asked of
`panic_on_overflow` of `quotient_position`. The text says a divisor is a constant today, that a value
cannot yet divide a value because the format's run-time constructor exists only where no slot names
zero, and names the format's ticket for it. It says there is no remainder because the format ships
none, with its ticket.

`convert::<T>()` takes a value of `S` to a value of any `T: Sole`, equal to the format's `convert` from
`S`'s format onto `T`, with `fits_convert::<T>()` its verdict, asked of `converted_position`. On one
format it gives what `under` gives, and `under` stays the door for that case.

Both call the format's function on slots as the fractional product does, under the same `FIXME` and
ticket, and the paragraph on the wide step is widened from the product to all three sites.

`## The debug check` names `fits_div` and `fits_convert` among the verdicts the check asks.

## CHANGE: `arvo-numeral`, the door at a host width

`## The doors` gains `from_index::<H>(host)`, `holds_index::<H>(host)` and `index::<H>()`, over the
sealed trait `HostInteger`, implemented for the host's eight machine integers of 8 to 64 bits and its
two pointer-width ones. The text states the contract of each: `from_index` equals
`adapted(Exact::on_grid(Slot::at(host)))` for every host value and asks the debug check the same
verdict; `holds_index` is that verdict; `index` is the slot index as an `H` and is a failed constant
where the range is not inside `H`'s. It states that the carrier is not named, that the arithmetic is
at 64 bits with the two cross-sign cases answered before any comparison, and that `from_host` and
`host` on the pointer-width aliases are kept.

`## Where the host integers are named`, item 3, gains the three functions and `HostInteger`.

The paragraph in `## The operations` that says there is no `From` of a host integer keeps that and
names `from_index` among the doors in: it is stated over the declared signature, its policy and its
range, and `From` would be a door that completes silently under a trait the consumer did not read.

## CHANGE: `arvo-numeral`, proofs and what is left

`## What is proved, and how` gains: the quotient through the value type against the format's `div`, at
widths 1 to 4 for every member divisor but zero and at widths 5 to 8 for a named set, both families,
both policies, five modes, every dividend, and the two fixed-point formats at exponent `-3`; the
conversion across widths, signs and exponents against the format's `convert`, and against `under` on
one format; the door against `adapted` on every 8-bit host value, the 16-bit ones within 1024 of zero
and the ends of each host integer, with `index` returning what `from_index` was given; the mutants
`mutant-div`, `mutant-convert` and `mutant-index`; the compile-fail pairs for `index`; the debug check
at the three new sites; and the consumer reaching each new item.

`## What is not here` drops the quotient and the conversion from the first sentence, and gains the
remainder, a divisor that is a value, and the per-width adaptation as the format's, each by its
ticket.

File: `mock/crates/arvo-numeral/README.md.tmpl`. The operations sentence names the quotient and the
conversion, and the doors sentence the host integer.

## What the source changelist will carry

In `arvo-numeral`: `div`, `fits_div`, `convert`, `fits_convert`, the re-export of `Divisor`, a module
`host` with `HostInteger`, `from_index`, `holds_index` and `index`, the three mutant features, the
tests named above, the mutants' red runs under
`mock/research/sketches/202611132039-hccc_value-mutants/`, and the consumer crate reaching each new
item.
