#!/usr/bin/env bash
# Build one source for every installed target and record what `USize` denotes.
#
# Two arms in the crate, and the whole result is that they disagree across
# targets:
#
#   THE_CLAIM_ABOUT_THE_FAMILY      always holds. The control: it says the alias
#                                   resolves and the coordinates read back, so a
#                                   REFUSED in that column means the target is
#                                   unbuildable and its other column says nothing.
#   THE_UNIVERSAL_CLAIM_ABOUT_USIZE holds at 64-bit and must fail elsewhere.
#
# The build is run twice per target, once with the universal arm present and once
# with it cut out, so a target that fails the first and passes the second has
# failed for that arm rather than for the target. Cutting rather than cfg-gating,
# because a cfg would put the arm's absence under test instead of its content.
#
# `cargo` is not used: it resolves one target per invocation and the point here is
# one source across several. Every dependency is therefore rebuilt per target,
# including `arvo-format` itself, because reusing the host rlib would hand every
# arm the host's pointer width and the probe would report agreement it never saw.
#
# Note on one diagnostic, because it cost real time: omitting `-L dependency`
# makes rustc report `E0463 can't find crate for 'arvo_format'` when what it
# actually could not find is `notko`, which `arvo-format` depends on. The parent
# is named and the missing crate is not.
#
# Run from this directory, after `cargo build`.
set -uo pipefail

ARVO_SRC="../../../../crates/arvo-format/src/lib.rs"
NOTKO_SRC=$(ls -d ~/.cargo/git/checkouts/notko-*/*/src/lib.rs 2>/dev/null | head -1)
[ -n "$NOTKO_SRC" ] || { echo "no notko checkout found; run 'cargo build' first"; exit 2; }

XT="target/xt"
mkdir -p "$XT" out
rm -f "$XT"/*.rlib

echo "toolchain: $(rustc --version)"
TARGETS=$(rustup target list --installed)
echo "targets:"; echo "$TARGETS" | sed 's/^/  /'

# The universal arm removed, by cutting it out of a copy of the source.
mkdir -p src/generated
awk '/^pub const THE_UNIVERSAL_CLAIM_ABOUT_USIZE/{skip=1} skip&&/^};$/{skip=0;next} !skip' \
    src/lib.rs > src/generated/without_the_universal_arm.rs

: > measured.tsv
printf 'target\tpointer_width\tuniversal_arm\tfamily_arm_only\n' >> measured.tsv

for t in $TARGETS; do
    NOTKO_LIB="$XT/libnotko-$t.rlib"
    ARVO_LIB="$XT/libarvo_format-$t.rlib"

    rustc --edition 2024 --crate-type lib "$NOTKO_SRC" \
        --target "$t" --crate-name notko -o "$NOTKO_LIB" \
        > /dev/null 2> "out/notko-$t.stderr"

    rustc --edition 2024 --crate-type lib "$ARVO_SRC" \
        --target "$t" --crate-name arvo_format \
        --extern "notko=$NOTKO_LIB" -L "dependency=$XT" \
        -o "$ARVO_LIB" > /dev/null 2> "out/arvo-$t.stderr"

    if [ ! -s "$ARVO_LIB" ]; then
        printf '%s\t?\tSKIPPED\tSKIPPED\n' "$t" >> measured.tsv
        continue
    fi

    PW=$(rustc --target "$t" --print cfg 2>/dev/null \
        | sed -n 's/^target_pointer_width="\(.*\)"$/\1/p')

    rustc --edition 2024 --crate-type lib src/lib.rs --target "$t" \
        --crate-name p03_with \
        --extern "arvo_format=$ARVO_LIB" --extern "notko=$NOTKO_LIB" \
        -L "dependency=$XT" --out-dir "$XT" \
        > /dev/null 2> "out/with-universal-$t.stderr"
    WITH=$?

    rustc --edition 2024 --crate-type lib src/generated/without_the_universal_arm.rs \
        --crate-name p03_without --target "$t" \
        --extern "arvo_format=$ARVO_LIB" --extern "notko=$NOTKO_LIB" \
        -L "dependency=$XT" --out-dir "$XT" \
        > /dev/null 2> "out/without-universal-$t.stderr"
    WITHOUT=$?

    printf '%s\t%s\t%s\t%s\n' "$t" "$PW" \
        "$([ $WITH -eq 0 ] && echo builds || echo REFUSED)" \
        "$([ $WITHOUT -eq 0 ] && echo builds || echo REFUSED)" >> measured.tsv
done

echo
column -t -s $'\t' measured.tsv
echo
echo "The universal arm must build exactly on the 64-bit rows, and the family arm on"
echo "every row. One source, one name, two verdicts, which is the finding. A run"
echo "where the universal arm builds everywhere means the alias never read the target"
echo "and the probe is void."
