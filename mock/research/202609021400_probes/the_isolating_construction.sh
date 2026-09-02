#!/usr/bin/env bash
# Seat q31a. Run the missing test against the real crate and against the mutant,
# to show it separates them. Naming a gap without showing the closer fires is
# half a finding.
set -uo pipefail
cd "$(dirname "$0")/../../.." || exit 1
F=mock/crates/arvo-format/src/slots.rs
BAK=mock/target/q31a_slots_backup2.rs
mkdir -p mock/target
cp "$F" "$BAK"
restore() { cp "$BAK" "$F"; touch "$F"; }
trap restore EXIT

build_and_run() { # $1 = label
  cargo build --manifest-path mock/Cargo.toml -p arvo-format >/dev/null 2>&1 || {
    echo "  [$1] arvo-format did not build"; return 2; }
  DEPS=mock/target/debug/deps
  FMT=$(ls -t "$DEPS"/libarvo_format-*.rlib | head -1)
  NOTKO=$(ls -t "$DEPS"/libnotko-*.rlib 2>/dev/null | head -1)
  rustc --edition 2024 -L "$DEPS" --extern arvo_format="$FMT" \
    ${NOTKO:+--extern notko="$NOTKO"} \
    -o mock/target/q31a_isolating "$(dirname "$0")/the_isolating_construction.rs" 2>/dev/null || {
      echo "  [$1] probe did not compile"; return 2; }
  mock/target/q31a_isolating
  return $?
}

echo "== A. against the crate as it stands =="
build_and_run real; a=$?
echo "  exit status: $a  (0 = the missing test passes here)"

echo
echo "== B. against the mutant, width clause deleted =="
perl -0pi -e 's/\s*assert!\(\s*Self::WIDTH\.count\(\) <= 62,.*?\);//s' "$F"
perl -0pi -e 's/\s*&& S::WIDTH\.count\(\) <= 62//s' "$F"
touch "$F"
[ "$(grep -c '<= 62' "$F")" -eq 0 ] || { echo "  mutation did not apply"; exit 1; }
cargo clean -p arvo-format --manifest-path mock/Cargo.toml >/dev/null 2>&1
build_and_run mutant; b=$?
echo "  exit status: $b  (non-zero = the missing test fails here)"

echo
echo "== C. verdict =="
if [ "$a" -eq 0 ] && [ "$b" -ne 0 ]; then
  echo "  The construction separates the crate from the mutant. It is a real test"
  echo "  of the width clause and it is the only one there would be."
else
  echo "  It does NOT separate them (A=$a, B=$b), so it is not the closer either."
fi
restore
cargo clean -p arvo-format --manifest-path mock/Cargo.toml >/dev/null 2>&1
