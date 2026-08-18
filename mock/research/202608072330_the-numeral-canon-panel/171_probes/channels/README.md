# P1 and P2: can a program distinguish two extensionally equal implementations, and does it need a binding

```sh
rustc --edition 2024 -O -o ch_release channels.rs && ./ch_release
rustc --edition 2024 -C debug-assertions=on -o ch_debug channels.rs && ./ch_debug
```
Output: `channels.out`. `const_channel.out` and `const_channel_p2b.out` are P2.

Two implementations of `(a + b) - c`: `wide` through an `i64` intermediate, `narrow` in `i32`
throughout with wrapping. The same function wherever the true result fits in `i32`, because
arithmetic mod 2^32 is a ring and an overflow in `a + b` cancels in the subtraction.

## Controls, all clean

- **C-C** 1,666,128 inputs in the domain, **83,084** with an overflowing narrow intermediate, **0**
  final-value disagreements. So the pair is extensionally equal *and* the overflow case is exercised.
- **C-A** `size_of_val` given a binding distinguishes, 8 against 4. Not vacuous.
- **C-B** an identical twin is indistinguishable on every channel.

## Result

| channel | needs a binding | distinguishes at `debug-assertions=off` | at `on` |
|---|---|---|---|
| final value | no | no | no |
| `size_of_val` of the intermediate | **yes** | yes | yes |
| `Debug` of the intermediate | **yes** | yes | yes |
| `align_of_val` | **yes** | yes | yes |
| **overflow panic** | **no** | **no** | **yes** |
| **const evaluation** | **no** | **no** | **yes** |

**At `debug-assertions = off` every channel found that distinguishes requires a binding to an
intermediate. At `on`, two do not.** The two that do not are the overflow panic and const-eval
refusal, which is exactly the pair I18 bounds to dev and debug builds.

## P2b, and the control that caught a defect in my own script

P2 used `-O` against `-C debug-assertions=on`, which moves optimisation level and the assertion flag
together. P2b separates them:

| flags | narrow const compiles | value |
|---|---|---|
| `opt-level=0 debug-assertions=off` | YES | 900000000 |
| `opt-level=0 debug-assertions=on` | **NO**, E0080 | - |
| `opt-level=3 debug-assertions=off` | YES | 900000000 |
| `opt-level=3 debug-assertions=on` | **NO**, E0080 | - |

**The const channel is governed by `debug-assertions`, not by optimisation level**, which I did not
expect: I assumed const evaluation checked arithmetic unconditionally. It does not, here.

**A ninth instrument defect in this unit, caught by its own control.** The first P2b looped over quoted
flag strings in fish, which does not word-split, so `rustc` received one bogus argument and every cell
reported "does not compile". The control required the wide arm to compile in all four cells; it
reported NO in all four, which is impossible if the table is about the two arms. Rewritten with the
commands spelled out. The class: **a 2x2 whose every cell agrees is a tell, and a control over a cell
that must differ is what catches it.**

`holds for: rustc 1.98.0-nightly (57d06900f), edition 2024, aarch64-apple-darwin, i32 container,
the operation (a+b)-c, opt-level in {0, 3}, debug-assertions in {on, off}, threads = 1`
