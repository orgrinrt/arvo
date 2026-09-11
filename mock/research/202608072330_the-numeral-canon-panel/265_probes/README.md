# 265_probes: what a platform-width type is, by instrument

Every probe here is a spike. It checks one thing, its incidental spellings are
scaffolding, and nothing in it is a design answer. Each probe directory carries
its source, the script that built it, and one output file per target under
`out/`, exit code first, so the claim in `265_lattner_what_a_platform_width_type_is.md`
that cites it can be re-run rather than believed.

## The matrix

`run_matrix.sh <probe-dir>` builds a cargo probe for six targets spanning the
three pointer widths the Rust reference admits, and records the pointer width
from `rustc --print cfg` in every output file:

| target | pointer width | how core was obtained |
|---|---|---|
| `aarch64-apple-darwin` | 64 | precompiled (host) |
| `x86_64-unknown-linux-gnu` | 64 | precompiled |
| `i686-unknown-linux-gnu` | 32 | precompiled |
| `thumbv6m-none-eabi` | 32 | precompiled |
| `wasm32-unknown-unknown` | 32 | precompiled |
| `msp430-none-elf` | 16 | `-Zbuild-std=core` from the pinned `rust-src` |

Toolchain: the workspace pin, `nightly-2026-05-28`, recorded in each output.

## The probes

| probe | checks | pairs with | result |
|---|---|---|---|
| `p1_target_bound_width` | a `Format` whose `Slots` is `Unsigned<{ usize::BITS }>` and `Signed<{ isize::BITS }>`, with the width, the cardinality and the top of the set asserted at compile time | `p1c_literal_width`, one token different per format (`32` for `usize::BITS`), diff in `p1_vs_p1c.diff` | builds at 16 and 32 with every assertion holding; refused at 64 by exactly `E0277: Unsigned<64> / Signed<64> is not an admitted slot range` and nothing else. The control builds at all six. |
| `p2a_const_bound_width` | three const spellings of the pointer width (`usize::BITS`, `cfg(target_pointer_width)`, `size_of::<usize>()`) asserted equal at compile time; an arm `match usize::BITS` lowered to assembly in `asm/` | `p2b_runtime_bound_width`, one difference: the count comes from a non-const function | p2a builds at all six; the arm folds to a bare constant on every lowered target (`mov w0, #3` at 64, `movs r0, #2` / `i32.const 2` at 32). p2b refused at all six by `E0015: cannot call non-const function in constants`. |
| `p3_one_name_two_realisations` | the name `PlatformUnsigned` with its width observation pinned at 32 by a `const` assertion | its own three outcomes | builds at 32; refused at 16 by the assertion text; refused at 64 by p1's `E0277`. |
| `p4_obligation_range` | whether the platform-width unsigned set covers a unix errno (bounded 4095) and a Windows `GetLastError` `DWORD` (`2^32 - 1`) | its own three outcomes | builds at 32; refused at 16 by the `DWORD` assertion (the errno one holds); refused at 64 by p1's `E0277`, so the shipped tree cannot host the obligation on the consumer's own hosts. |
| `p5_const_generic_position` | `Buf<const N: usize>` (p5a), `Buf<const N: Cap>` with `Cap` a newtype (p5b), the same with `adt_const_params` and `ConstParamTy` (p5c); bare `rustc` per precompiled target | a, b, c differ pairwise in one thing | a builds, b refused (`Cap is forbidden as the type of a const generic parameter`), c builds, at all five targets. |
| `p6_placement_reads_the_bound_width` | `derive_sole` over the platform-width signature under both objectives, plus `narrowest_carrier(pointer width)` | `p6a` | carrier equals the pointer width and one output at sole, at 16 and 32; refused at 64 by p1's `E0277`. |
| `p6a_ladder_over_the_pointer_width` | the ladder half alone, no format in the crate | `p6` | builds at all six: the narrowest shipped carrier covering the pointer width is the machine word at 16, 32 and 64, and `LADDER` is the same list at every target. |

## Outside sources

`fetched/` holds the two Rust reference pages relied on, with the fetch date in
`SOURCES.txt`. SerpAPI's budget was spent (`"Your account has run out of
searches."`), so the pages were fetched directly, which the search skill lists
as the next step.
