#!/usr/bin/env bash
# Seat q31a. The `WIDTH <= 62` clause in `Slots::ADMITTED` is the single clearest
# hosting condition in the crate: its own message says `2^63 does not fit a
# signed 64-bit integer`. Before citing it as the load-bearing example of a
# hosting bound welded into an admission obligation, find out whether the suite
# can tell if it is there.
#
# The method is mutation, which is the only thing that answers it: delete the
# clause from both the const and the verdict, run the whole suite, and see what
# goes red. Nothing red means no test isolates it, because the construction the
# suite points at it with, `RogueRange`, is also inverted and would be refused
# anyway.
#
# Restores by rewriting the file rather than by moving a backup over it, and
# touches it afterwards, because a restore that carries an older mtime leaves
# cargo believing the object built from the mutation is current.
set -uo pipefail
cd "$(dirname "$0")/../../.." || exit 1
F=mock/crates/arvo-format/src/slots.rs
BAK=mock/target/q31a_slots_backup.rs
mkdir -p mock/target
cp "$F" "$BAK"

restore() { cp "$BAK" "$F"; touch "$F"; cargo clean -p arvo-format --manifest-path mock/Cargo.toml >/dev/null 2>&1; }
trap restore EXIT

echo "== 0. baseline: the suite as it stands =="
cargo test --manifest-path mock/Cargo.toml -p arvo-format 2>&1 | grep -E '^test result|error\[' | sort | uniq -c

echo
echo "== 1. mutate: delete the WIDTH <= 62 clause from the const and the verdict =="
# The const's assertion, three lines.
perl -0pi -e 's/\s*assert!\(\s*Self::WIDTH\.count\(\) <= 62,.*?\);//s' "$F"
# The verdict's conjunct.
perl -0pi -e 's/\s*&& S::WIDTH\.count\(\) <= 62//s' "$F"
touch "$F"
echo "  clause still present in the file? $(grep -c '<= 62' "$F") occurrence(s) (0 means the mutation applied)"
if [ "$(grep -c '<= 62' "$F")" -ne 0 ]; then
  echo "  MUTATION DID NOT APPLY. Nothing below is evidence."; exit 1
fi

echo
echo "== 2. run the whole suite against the mutant =="
cargo clean -p arvo-format --manifest-path mock/Cargo.toml >/dev/null 2>&1
OUT=$(cargo test --manifest-path mock/Cargo.toml -p arvo-format 2>&1)
printf '%s\n' "$OUT" | grep -E '^test result|^error|FAILED|panicked' | sort | uniq -c
fails=$(printf '%s\n' "$OUT" | grep -cE '^test .* FAILED|test result: FAILED')

echo
echo "== 3. verdict =="
if [ "$fails" -eq 0 ]; then
  echo "  NO TEST FAILED. The clause can be deleted and the suite stays green."
  echo "  So no test in this crate isolates \`WIDTH <= 62\`. The construction the"
  echo "  suite points at it with, RogueRange, is inverted as well as 63 bits wide,"
  echo "  and the first clause refuses it whether or not the third exists."
else
  echo "  $fails test(s) failed, so the clause is isolated somewhere."
  printf '%s\n' "$OUT" | grep -E '^test .* FAILED' | head
fi

echo
echo "== 4. control: mutate a clause that IS isolated, to show the method fires =="
restore
perl -0pi -e 's/\s*assert!\(\s*Self::MIN\.index\(\) <= Self::MAX\.index\(\),.*?\);//s' "$F"
perl -0pi -e 's/S::MIN\.index\(\) <= S::MAX\.index\(\)\s*&&\s*/ /s' "$F"
touch "$F"
cargo clean -p arvo-format --manifest-path mock/Cargo.toml >/dev/null 2>&1
OUT2=$(cargo test --manifest-path mock/Cargo.toml -p arvo-format 2>&1)
f2=$(printf '%s\n' "$OUT2" | grep -cE '^test .* FAILED|test result: FAILED')
echo "  deleting the MIN <= MAX clause: $f2 failing test(s) / error(s)"
printf '%s\n' "$OUT2" | grep -E '^test .* FAILED|^error' | head -5
if [ "$f2" -eq 0 ]; then
  echo "  CONTROL FAILED: the method does not fire even on a clause with a dedicated"
  echo "  test, so section 3's zero is a fact about the method and not about the suite."
else
  echo "  The method fires. Section 3's zero is a fact about the suite."
fi
