#!/usr/bin/env bash
# Builds the library, then each refusal case, keeping each output in outputs/.
set -u
cd "$(dirname "$0")"
export CARGO_INCREMENTAL=0
(cargo build --lib 2>&1; echo "exit $?") > outputs/lib.txt
for e in past_inherent past_generic past_runtime_generic; do
    (cargo build --example "$e" 2>&1; echo "exit $?") > "outputs/$e.txt"
done
