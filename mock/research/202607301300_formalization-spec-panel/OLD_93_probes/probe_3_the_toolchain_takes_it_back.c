/* Probe 3: the compiler contract between the design and the target voids the "the target
 * defines it" route through ordinary codegen. LLVM's `sdiv` is undefined behavior on a
 * zero divisor (LLVM LangRef, 'sdiv'/'udiv': "Division by zero is undefined behavior"),
 * so the optimizer is licensed to assume every divisor reaching a division is nonzero,
 * and it uses that licence: a zero-divisor check placed AFTER a division is deleted.
 *
 * WHAT THIS MODEL SEPARATES (`86b:8-10`). It separates the ISA's semantics (probe 1:
 * aarch64 defines x/0 = 0) from the toolchain's semantics (here: the IR the design
 * actually lowers through defines x/0 as UB on every target, including aarch64). The
 * distinction is nonvacuous because on aarch64 the two give different answers to "is the
 * cell free": the silicon says yes, the IR contract says the cell does not exist, and a
 * lowering wanting the silicon's answer must pay an asm-opacity barrier to get past the
 * IR. A probe reading only the emitted `sdiv` instruction would conflate them.
 *
 * `f` divides first and checks the divisor afterward. If LLVM honoured x/0 as a value,
 * the check would have to survive. It does not: at -O2 on aarch64 the emitted body of `f`
 * is `sdiv w0, w0, w1; ret`, no compare, no branch, no -999 path. The same holds at
 * x86-64 (`idivl` with no compare). `g` is the control: checking BEFORE dividing
 * survives, because the division no longer dominates the check.
 *
 * Build: clang -O2 -S probe_3_the_toolchain_takes_it_back.c -o out/probe_3_aarch64.s
 *        clang -target x86_64-apple-darwin -isysroot $(xcrun --show-sdk-path) -O2 -S \
 *          probe_3_the_toolchain_takes_it_back.c -o out/probe_3_x86_64.s
 * Outcome: WORKS. The `d == 0` arm of `f` is deleted on both targets; `g` keeps its.
 */
int f(int x, int d) {
    int q = x / d;          /* llvm sdiv: UB if d == 0, so the optimizer assumes d != 0 */
    if (d == 0) return -999; /* dead by the assumption; deleted at -O2 */
    return q;
}

int g(int x, int d) {
    if (d == 0) return -999; /* control: the check dominates the division and survives */
    return x / d;
}
