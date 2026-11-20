# The ratio bench after the step moved to the format

`NOTES.md` beside this file reads run `20261119T224053Z-1dd38711`, which timed the arm while it lived in
`arvo-numeral/src/room.rs`. Round `202611200734` deleted that file and moved the step into `arvo-format`
as `Member::product`, `Member::quotient` and `Member::converted`, and the value now lands its slots with
`Member::landed` and calls the step. So the code the run timed is gone, and its factors, 1.12 times the
hand product and 1.58 times the hand conversion at `Fixed<16, -8>`, are not a measure of anything the
tree holds now. The arms named `room` keep their name and now time `Member::landed` plus the member step.

Those factors were what the round leaned on when it removed the three
`FIXME(arvo_numeral_s_fixed_point_product_goes_through_the_128_bit)` markers. The rounding after the
step still goes through `adapt`, whose `Exact` and `Remainder` hold the four-limb `Wide`, so the
markers' worry is answered only as far as the step's own division goes, and that answer is from the
deleted code.

The bench is to be run again on the tree as it stands before
`member_adapted_lands_an_on_grid_slot_through_the_four_limb_a` closes, and its numbers quoted from that
run. Until then nothing here says how fast the format's step is.

Found by the second review of `work/arvo-numeral-exact-step-clean`,
`mock/research/202611201150_review-arvo-numeral-exact-step/PASS_2.md`, finding 2.
