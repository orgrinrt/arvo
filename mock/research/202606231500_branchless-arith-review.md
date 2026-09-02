# Branchless review of the arith op bodies (deferred audit)

Raised during round 202606231229 (Precise logical-bound clamp + the >64-bit fixed-point widen). The new
op bodies use `if` clamps (`if v < lo { lo } else if v > hi { hi } else { v }`) and the `shr256_lo` shift
cases. Question: should these be branchless, given the arith ops run a lot and the bodies are monomorphized
across hundreds of `(strategy, width)` impls, so any per-site compile cost compounds?

Decision for this round: keep the `if` form. Recorded here and as task #4 for a later audit pass.

## Why keeping the branches is the right call now

- Runtime is already branchless. LLVM lowers a scalar `if v < lo { lo } else if v > hi { hi } else { v }`
  to `cmov` / `csel` (conditional-move) at any `-O`. The shipped binary has no branch regardless of source
  shape, so a branchless source rewrite changes nothing in codegen.
- These bodies are predominantly runtime fns, not const-eval'd at scale. The `Mul` / `Div` operator paths
  call `i_mul_fixed::<FRAC>` / the clamp ops at runtime; const-evaluation happens only at the few const-use
  sites (`Identity::ONE`, const tests). There is no pile-up of const branch-processing across many sites.
- This crate's compile time is dominated by the const-trait + `generic_const_exprs` solving and the
  existence of the hundreds of monomorphized impls, not by a 2-way select inside each body. Flattening the
  branch would not move the needle measurably.
- `shr256_lo`'s branches are correctness, not optimization. `<<` / `>>` by >= 128 (or the `128 - frac`
  underflow) is UB on `u128`; the `frac == 0` / `< 128` / `== 128` / `> 128` cases cannot collapse into one
  arithmetic expression without reintroducing masking conditionals. These stay.

## Cost of forcing branchless (why it would be a net negative today)

- The unsigned upper-clamp has a clean branchless form: `v - v.saturating_sub(hi)` (unsigned
  `saturating_sub` floors at 0, giving the lower bound for free). Zero readability cost; could adopt.
- The signed two-sided clamp has no clean branchless form on the pinned nightly. There is no const
  `min` / `max` / `clamp`, and the unsigned saturating trick fails for signed (`saturating_sub` does not
  floor negatives). A branchless signed clamp needs sign-bit bit-hacks (`x & (x >> (BITS - 1))` style),
  which add bare-numeric `lint:allow` noise and make the floor / clamp logic harder to verify, for output
  identical to the `cmov` LLVM already emits.

## Revisit conditions (when the audit becomes consequential)

- A large const table (compile-time-built) comes to depend on these arith fns. Then the const-eval branch
  cost would compound across the table's entries, and the branchless arithmetic forms would const-fold
  faster.
- A bench or a `rustc` self-profile / `--timings` run flags the arith op bodies (not the const-trait / GCE
  machinery) as a compile-time or runtime hotspot.

Until one of those holds, the readable `if` form stands. Task #4 carries the audit.

## Related: 256-bit widen per-target perf path (TODO, not a correctness gap)

The >64-bit fixed-point widen (`umul256` / `i_mul_fixed_128` in `arith.rs`) uses `u128::carrying_mul`. This
is portable: LLVM lowers `u128` multiply on every target (native 128-bit multiply where the ISA has it,
software multi-limb or a `__multi3` libcall where it does not), so the path is correct on all hardware with
no fallback required. It is NOT gated to modern hardware.

The open item is perf-optimal per target, not correctness. On constrained targets (32-bit, or any without a
fast 128-bit multiply) the generic i128 lowering can be suboptimal, and a cfg-gated explicit-limb or
intrinsic route may beat it. That is arvo's always-optimal-internals discipline (Kind 1 structural lowering,
cfg-gated, bench-driven): add per-target cfg arms when a real target's bench shows the generic lowering is a
hotspot. Carried under task #4 alongside the branchless audit. Inline `TODO(perf, ...)` marks the use site.
