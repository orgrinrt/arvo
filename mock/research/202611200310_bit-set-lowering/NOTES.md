# What the bit tier's set operations lower to

`obligation::set_operations_over_a_fixed_size_bit_set` asks for intersection, union and an emptiness test
over a set of a size fixed at build, each lowering to one instruction, and says the lowering is part of
the need. Round `202611200157` gives `arvo-bits` the bit tier, and this note reads what its three
operations compile to at `Bits<64>` against the same three written on the host's `u64`.

## What was measured

The bench `mock/benches/bit-set/` has two arms. `bit-set-host` writes `a & b != 0`, `a | b` and `a == 0`
on `u64`; `bit-set-tier` writes the same at `Bits<64>` through the trait: `meet` is `a.and(b).is_empty()` negated
in a function generic over `BitLogic`, and `union` and `is_empty` call `BitLogic::or` and
`BitLogic::is_empty` on `Bits<64>` by name. Each arm exports the
three as `bit_set_meet`, `bit_set_union` and `bit_set_is_empty`, and `cargo mock bench disasm` built each
arm as a run does, under the harness's pinned release profile, and wrote one function per file under
`mock/benches/results/bit-set/disasm/linux-x86_64-c4-container-intel/`, runs `20261120T030755Z-a69939c8`
through `20261120T030815Z-a69939c8`, tool `llvm-objdump`, commit `a69939c8`, not dirty. No timing was run.

## What it shows

| Operation | Host arm | Tier arm |
|---|---|---|
| `bit_set_meet` | `testq %rdi, %rsi`; `setne %al`; `retq` | the same |
| `bit_set_union` | `movq %rdi, %rax`; `orq %rsi, %rax`; `retq` | the same |
| `bit_set_is_empty` | `testq %rdi, %rdi`; `sete %al`; `retq` | the same |

The tier arm is instruction for instruction the host arm. The operation is one instruction in each, `test`
for the intersection test and the emptiness test and `or` for the union; the `setcc` puts the flag into
the C return register and the `mov` moves the first argument into it, both the cost of returning across
the C boundary the exports have, which an inlined use folds into a branch or a further operation.

## What it decides, and what it does not

holds for: `N = 64`, x86-64, the bench harness's release profile, each operation called across a C
boundary. It says the tier adds nothing over the host integer at sixty-four bits on this target, and that
the obligation's one-instruction lowering holds there for all three operations.

It says nothing of a set of 256 bits, which this crate does not hold and whose one-instruction lowering is
the obligation's own open `gap`; nothing of another target; and nothing of a width below sixty-four, where
`masked` puts an `and` with the mask at the boundary a value enters by, and nothing here reads what
becomes of it.
