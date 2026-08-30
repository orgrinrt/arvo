#!/bin/sh
# File 62. Rebuilds file 08's union crate from the committed trail, exactly per
# the recipe `08_probes/README.md:8-11` states, then compiles `e_codegen.rs`
# against it and emits asm for the five delivery shapes. Run from this directory
# with the workspace pin installed. The one deviation from the README: the
# deliberately-wrong consumer file `f_error_surface.rs` is excluded from the
# build, since its documented outcome is a compile failure.
set -e
PIN=+nightly-2026-05-28
SRC=../08_probes
WORK="${1:-/tmp/union-rebuild-62}"
mkdir -p "$WORK/src/bin"
cat > "$WORK/Cargo.toml" <<'EOF'
[package]
name = "union"
version = "0.0.0"
edition = "2024"
[profile.release]
opt-level = 3
EOF
cp "$SRC/a_union.rs"                         "$WORK/src/lib.rs"
cp "$SRC/b_spare_pattern_decides_delivery.rs" "$WORK/src/spare.rs"
cp "$SRC/c_split_does_not_bind.rs"            "$WORK/src/fusion.rs"
cp "$SRC/e_codegen.rs"                        "$WORK/src/bin/e_codegen.rs"
cp "$SRC/g_classification_table.rs"           "$WORK/src/bin/g_classification_table.rs"
( cd "$WORK" && cargo $PIN build --release --offline )
"$WORK/target/release/g_classification_table"
RLIB=$(ls "$WORK"/target/release/deps/libunion-*.rlib | head -1)
rustc $PIN --edition 2024 --crate-type lib -C opt-level=3 \
  --extern union="$RLIB" -L "$WORK/target/release/deps" \
  --emit asm -o "$WORK/e.s" "$WORK/src/bin/e_codegen.rs"
echo "asm at $WORK/e.s; count with count_shapes.py"
