#!/bin/bash
# PROBE 4: an axis marker's DEFINING CRATE is in the symbol.
#
# File 20's contract requires arvo to promise that marker names are public
# interface. This measures something stronger and cheaper: the symbol names the
# crate each generic argument came from, so a verifier scopes itself
# structurally ("arguments defined in an axis crate") instead of carrying a
# hand-written list of marker names that drifts every time an axis gains an
# instance.
#
# The difference is measured, not argued: probe 3 run over the real tree with a
# name list produces sixteen findings, every one of them in `syn` and none of
# them about arvo. With crate scoping, zero.
set -u
D=$(mktemp -d)
N="+nightly-2026-05-28"
F="-O -Cno-prepopulate-passes -Zinline-mir=no -Cpanic=abort"

cat > "$D/axes.rs" <<'EOF'
#![crate_type="lib"] #![crate_name="arvo_lowering"]
pub trait Layout { const PACKED: bool; }
pub struct Dense; pub struct Bitpacked;
impl Layout for Dense { const PACKED: bool = false; }
impl Layout for Bitpacked { const PACKED: bool = true; }
EOF
cat > "$D/num.rs" <<'EOF'
#![crate_type="lib"] #![crate_name="arvo_numeric"]
extern crate arvo_lowering as ax;
use ax::Layout;
pub fn load<const I: u16, L: Layout>(w: &[u64], i: usize) -> u64 {
    if L::PACKED { (w[i * I as usize / 64] >> (i % 64)) & 0xff } else { w[i] }
}
EOF
cat > "$D/user.rs" <<'EOF'
#![crate_type="lib"]
extern crate arvo_numeric as an; extern crate arvo_lowering as ax;
#[no_mangle] pub extern "C" fn u1(w: &[u64], i: usize) -> u64 { an::load::<13, ax::Dense>(w, i) }
#[no_mangle] pub extern "C" fn u2(w: &[u64], i: usize) -> u64 { an::load::<13, ax::Bitpacked>(w, i) }
EOF

(cd "$D" && rustc $N $F axes.rs 2>/dev/null &&
 rustc $N $F -L . --extern arvo_lowering=libarvo_lowering.rlib num.rs 2>/dev/null &&
 rustc $N $F -L . --extern arvo_numeric=libarvo_numeric.rlib \
   --extern arvo_lowering=libarvo_lowering.rlib --emit=llvm-ir user.rs -o u.ll 2>/dev/null)

echo "=== three crates: axes, a generic operation, a consumer instantiating it ==="
grep -o '^define[^@]*@_RIN[A-Za-z0-9_.$]*' "$D/u.ll" | sed 's/.*@/  /'
echo
echo "The argument is 'NtCs<hash>_13arvo_lowering5Dense'. The crate that DEFINED"
echo "the marker is in the name. Scoping needs no list of marker names, and an"
echo "axis gaining an instance costs the verifier nothing."
rm -rf "$D"
