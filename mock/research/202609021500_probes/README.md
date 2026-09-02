# Q31 probes: is the admission obligation one word or two, and can you tell by measuring

Run them all with `./run.sh`. It builds every arm, records the full compiler output
under `out/`, and prints one line per arm. The script encodes no expectation, so it
cannot pass an arm by believing it should; what each arm was supposed to do is in
that arm's own header comment and in its file name.

`cargo build` throughout and never `cargo check`. `Slots::ADMITTED` is a const, a
const is evaluated at codegen, and `check` skips codegen. `arvo-format/src/slots.rs`
says so in its own doc comment. Without that, the first five arms report five silent
passes.

Everything here is a spike. It checks one thing each. None of the naming, none of
the shapes, and none of the incidental choices is a design decision, and the
`Grid` type exists only because the shipped impl set stops at 62 and there is
otherwise no way to ask what the obligation says at 63.

## What each arm establishes

| Arm | Expectation | What it establishes |
|---|---|---|
| `arms/a1_exact_product_of_admitted_operands_is_refused` | refuses | Both operands are `Signed<32>`, which arvo admits. The set the exact product of two of them occupies is refused by arvo's own admission obligation, at `slots.rs:219`, on the ground that a slot index is a signed 64-bit integer. Its endpoints both fit that integer and its span fits a count, so no other assertion can be the cause. |
| `arms/a2_control_the_same_shape_at_sixteen_bits_builds` | builds | The identical construction at sixteen bits, whose exact product needs 31 bits. So a1's refusal is the width and not `Grid`, not the arithmetic in the const parameters, and not anything else in the program. |
| `arms/a3_a_wellformed_grid_past_the_ladder_is_refused` | refuses | A 63-bit two's complement grid: contiguous, ordered, addressed exactly by its width. Refused at `slots.rs:219`. |
| `arms/a4_control_the_same_grid_at_sixtytwo_builds` | builds | The same shape one bit narrower. |
| `arms/a5_negative_control_a_malformed_grid_is_also_refused` | refuses | An inverted range, refused at `slots.rs:211` with a message about admitting nothing. Two different kinds of refusal out of one const, through one door, under one error code. |
| `carrier/` lib | builds | Two copies of the shipped `slots.rs` side by side, differing in exactly one thing that is not a design choice: the machine type a slot index is carried in. The same 63-bit declaration is refused by one and admitted by the other, and an inverted range is refused by both. |
| `carrier/arms/c1`, `c2` | both refuse | The controls on that. Each asserts the opposite of one half, and each fails, so the lib's assertions are not vacuous in either direction. |
| `cross/invariance` | builds on every target | The admitted set, its exact boundary either side, and the endpoints of the widest admitted grid, asserted at check time and identical on four targets spanning two pointer widths, three operating-system values and two architecture families. |
| `cross/mutant` | fails on every target | Asserts the 63-bit grid is admitted. It fails everywhere, so the invariance arm could have reported a difference had there been one. |

## The classification instrument

`carrier/src/lib.rs`, module `classify`. It restates the five slot assertions over
plain numbers so each can be asked on its own, and then asks each one twice: once
with the shipped carrier and once with the widened one.

The test it applies reads no string. **Hold the declaration fixed, change only the
machine type a slot index is carried in. An assertion whose verdict moves is about
the machine. An assertion whose verdict does not is about the grid.** Under it, the
ladder bound moves and the inverted, zero-width and width-does-not-cover conditions
do not.

`ASSERTION_FOUR_IS_IMPLIED` in the same module is exhaustive over the only free
dimension rather than a sample: for every one of the 62 admitted widths, the span
bound at `slots.rs:232` already excludes everything the count bound at `slots.rs:228`
would have excluded. `AT_SIXTY_THREE_IT_FAILS` beside it is the control saying that
sweep is not vacuous, and the module `span_too_wide` puts the crate's own pinned
case for that assertion through the same instrument.

## What is not here, and what was chased down instead

Nothing, now. `msp430-none-elf` has no prebuilt artifacts on the pinned toolchain, so
`run.sh` builds core from source for it with `-Z build-std=core`, which needs the
`rust-src` component the pin already lists. So the invariance result covers pointer
widths 16, 32 and 64, and the mutant fails on all five targets.

`avr-none` was tried and needs a CPU selected through a form `cargo build` would not
take on this pin. Not chased, because msp430 already carries the 16-bit case.
