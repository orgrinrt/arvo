#!/usr/bin/env bash
# Does the gate the second option needs reach a consumer?
#
# The obligation the second option is bounded by says a consumer naming arvo's
# types needs no unstable features of its own, and that row's own `gap` field
# records the containment question as unmeasured. This measures it for one gate.
#
# Three steps, and the middle one is what makes the first mean anything: a
# consumer that builds could be a consumer that has the feature by some other
# route, and the control is what separates those.
set -u
cd "$(dirname "$0")"

echo "=== 1. the consumer builds, naming the door's const parameter ==="
if out=$(cargo build --quiet 2>&1); then
    echo "    built, with no \`#![feature(...)]\` in the consumer."
else
    echo "    DID NOT BUILD:"; echo "$out" | sed 's/^/    /'; exit 1
fi
echo

echo "=== 2. and 3. the control refusal, and the value crossing ==="
# The control is a `trybuild` case rather than an example: as an example it
# broke `cargo test` by failing to build, which is what it is for.
out=$(cargo test --quiet 2>&1); status=$?
echo "$out" | grep -E '^test |test result' | sed 's/^/    /'
echo

echo "=== verdict ==="
if [ "$status" -ne 0 ]; then
    echo "RUN INVALID: the suite did not pass, so neither the control nor the"
    echo "value assertions held."
    exit 1
fi
echo "On this pin, an \`adt_const_params\` parameter in a public signature is"
echo "contained: a consumer with no feature attribute of its own names it,"
echo "reads the associated constant back with its value, and writes a generic"
echo "function over the parameter. The control confirms that consumer cannot"
echo "declare such a parameter itself, so it really is ungated."
echo
echo "Predicate: gate = adt_const_params, toolchain = the pinned"
echo "nightly-2026-05-28, position = a const generic parameter in a public"
echo "signature. It says nothing about any other gate and nothing about"
echo "generic_const_exprs, which is the one the obligation's gap names."
