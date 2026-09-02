#!/usr/bin/env bash
# Probe: does the committed bench corpus (mock/benches/) encode a fraction-width
# (F, as distinct from integer/declared width W) axis anywhere, the way it encodes
# W, NC, OP, D as key fields?
#
# Positive control: the same search style MUST find the width axis, which we
# already know from reading warm-container-shared/src/lib.rs is real
# (`key_w`, "declared width in bits"). If the control fails, the search
# methodology itself is broken and the negative result below is worthless.
set -uo pipefail
cd "$(git rev-parse --show-toplevel)"

echo "=== control: width axis (key_w) must be found ==="
grep -rn "fn key_w" mock/benches/variants/ | wc -l

echo "=== target: any fraction-width axis (key_f / frac_bits / FRAC) ==="
grep -rniE "key_f\(|frac_bits|const F:|FRAC_BITS|fraction.?width" mock/benches/variants/ | wc -l

echo "=== target detail (if any) ==="
grep -rniE "key_f\(|frac_bits|const F:|FRAC_BITS|fraction.?width" mock/benches/variants/ || echo "(none found)"

echo "=== signedness axis: is signed vs unsigned ever a swept KEY field? ==="
grep -rniE "key_sign|is_signed|SIGNED:|const SIGN" mock/benches/variants/ || echo "(none found)"

echo "=== does any variant crate depend on a real arvo crate (I,F,S generic type)? ==="
grep -rln '"arvo"\|arvo-strategy\|arvo_strategy\|UFixed\|IFixed' mock/benches/variants/*/Cargo.toml mock/benches/variants/*/src/*.rs 2>/dev/null || echo "(none found)"
