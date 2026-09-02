# How to rebuild and re-run seat 222's probes

Four probes, each one file, no dependencies, exact integer arithmetic throughout.
Every one prints its predictions and its controls before its result, and every
control is printed firing rather than asserted to fire.

```sh
for p in a1_the_variance_forms_across_fraction_widths \
         a2_monotonicity_on_one_construction_across_three_keyings \
         a3_the_layout_observation_is_const_and_gateable \
         a4_which_precision_reading_makes_the_sign_domains_a_chain; do
    rustc --edition 2024 -O -o "/tmp/$p" "$p.rs" && "/tmp/$p" > "${p%%_*}_output.txt"
done
```

`--edition 2024` is required rather than decorative: `a3` uses `core::mem` rather
than `std::mem` on purpose, because the question it answers is whether the
observation is available to a `no_std` crate, and the 2015 extern prelude does
not carry `core`.

Toolchain: `rustc 1.98.0-nightly (57d06900f 2026-05-27)`, the workspace pin.

`a2` and `a3` walk 67 million pairs each and take a few seconds. Nothing here
times anything, so none of it is a bench and none of it may be called one.

| probe | question it is about |
|---|---|
| `a1` | `question::does_the_rounding_variance_form_hold_at_a_second_fraction_width` |
| `a2` | `question::does_the_position_keyed_members_monotonicity_failure_rate_differ_from_the_independent_members` |
| `a3` | `question::the_container_premise` |
| `a4` | `question::does_precision_count_the_sign_digit` |
