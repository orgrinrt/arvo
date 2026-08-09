/* Probe 2: x86-64 integer division by zero delivers no value at all: the `div`/`idiv`
 * instruction raises #DE and the process dies with SIGFPE. Run under Rosetta 2 on this
 * host, so the silicon-level fact is read rather than cited.
 *
 * WHAT THIS MODEL SEPARATES (`86b:8-10`). It separates "the target's own divide
 * instruction defines the answer" (true on aarch64, probe 1) from "every target's divide
 * instruction defines an answer" (false: x86 defines a trap, which is not a value). The
 * distinction is nonvacuous because the Door-placement argument quantifies over targets
 * and was checked at one.
 *
 * The division is routed through inline asm so neither C's UB rules nor the optimizer
 * can touch it: the executed instruction is the ISA's own `idiv`, exactly what a lowering
 * "letting the target define the cell" would emit.
 *
 * Build: clang -target x86_64-apple-darwin -isysroot $(xcrun --show-sdk-path) -O2 \
 *          probe_2_x86_does_not_define_a_value.c -o out/probe_2_x86
 * Run: ./out/probe_2_x86 5 0   (under Rosetta 2)
 * Outcome: WORKS (as a refusal): killed by SIGFPE, shell status 136. No value exists.
 */
#include <stdio.h>
#include <stdlib.h>

__attribute__((noinline)) long long raw_idiv(long long x, long long d) {
    long long q;
    /* cqto sign-extends rax into rdx:rax; idiv divides rdx:rax by the operand */
    __asm__ volatile("movq %1, %%rax\n\tcqto\n\tidivq %2\n\tmovq %%rax, %0"
                     : "=r"(q)
                     : "r"(x), "r"(d)
                     : "rax", "rdx");
    return q;
}

int main(int argc, char **argv) {
    if (argc != 3) return 2;
    long long x = atoll(argv[1]);
    long long d = atoll(argv[2]);
    long long q = raw_idiv(x, d);
    printf("%lld / %lld = %lld\n", x, d, q);
    return 0;
}
