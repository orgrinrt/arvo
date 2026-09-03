#!/usr/bin/env bash
# Seat 258. Why `cargo mock test` reports the benches tree failed while
# `cargo test` in that tree passes.
#
# The test gate ran the whole suite. `cargo test --workspace` under `mock/` is
# green, 178 passing and 3 ignored, and the lints tree is green at 676 passing and
# 16 ignored. `cargo mock test`, which runs every tree mockspace owns, reports
# "FAILED benches" and "1 of 8 tree(s) failed" and prints none of that tree's
# output, so the failure arrives with no diagnosis attached.
#
# Attacked rather than reported. `cargo test`, `--workspace`, `--all-targets`,
# `--locked`, `--offline` and `--manifest-path` from the repository root all pass
# in that tree. What fails it is denied warnings: two nested `unsafe` blocks in one
# bench variant are `unused_unsafe`, which is a warning under `cargo test` and an
# error under `RUSTFLAGS=-D warnings`.
#
# Not repaired here. The file is a bench variant whose committed CSV artifacts
# record what built them, and `a-committed-timing-records-what-built-it` is a lint
# over exactly that pairing, so editing the kernel is a bench-harness act rather
# than a panel one.
#
# THE CASES THAT MUST FAIL, run before the verdict is reported:
#   C1  The ordinary run must pass, or the finding is not about the flag.
#   C2  The denied-warnings run must fail, or the flag is not what separates them.
#   C3  The two lines the error names must be the two nested `unsafe` blocks, read
#       out of the source rather than taken from the message.
set -u
cd "$(dirname "$0")/../../../.." || exit 1 # the repository root
fail() {
	echo "CONTROL FAILED: $1"
	exit 2
}

TREE=mock/benches
SRC=$TREE/variants/bitpack-contend-shared/src/kernels.rs

echo "tree: $(git rev-parse HEAD)"
echo "toolchain: $(rustc --version 2>&1)"
echo

# --- C1 ------------------------------------------------------------------------
if (cd "$TREE" && cargo test --workspace >/dev/null 2>&1); then
	echo "C1 cargo test --workspace in $TREE            : passes"
else
	fail "C1: the ordinary run failed, so the finding is not about the flag"
fi

# --- C2 ------------------------------------------------------------------------
out=$(cd "$TREE" && RUSTFLAGS="-D warnings" cargo test --workspace 2>&1)
if printf '%s\n' "$out" | grep -q '^error'; then
	echo "C2 the same run under RUSTFLAGS=-D warnings   : fails"
else
	fail "C2: the denied-warnings run passed, so the flag is not the separator"
fi
echo
printf '%s\n' "$out" | grep -E '^error|^ *--> ' | sed 's/^/    /'
echo

# --- C3 ------------------------------------------------------------------------
echo "the two lines, read out of the source:"
for l in 291 348; do
	printf '    %s:%s: %s\n' "$SRC" "$l" "$(sed -n "${l}p" "$SRC")"
done
for l in 291 348; do
	sed -n "${l}p" "$SRC" | grep -q 'unsafe' || fail "C3: $SRC:$l is not an unsafe block"
done
echo "C3 both named lines are unsafe blocks         : yes"
echo "    and each is nested inside an outer one:"
for l in 279 340; do
	printf '    %s:%s: %s\n' "$SRC" "$l" "$(sed -n "${l}p" "$SRC")"
done
echo
echo "VERDICT: \`cargo mock test\` denies warnings and the benches tree carries two."
echo "The suite itself is green. Not repaired here: the variant's committed artifacts"
echo "record what built them and the repair belongs to whoever owns the bench harness."
