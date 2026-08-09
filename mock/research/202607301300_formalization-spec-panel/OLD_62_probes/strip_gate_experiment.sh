#!/bin/sh
# File 62. The direct experiment behind section 2 of the panel file: copy a
# shipped crate out of the tree, delete its `#![feature(generic_const_exprs)]`
# line and nothing else, and build under the workspace pin. Run from the repo
# root. The copy goes outside the tree so no shipped source is edited; note
# that OUTSIDE the tree the toolchain file does not apply, so the pin must be
# named explicitly (a bare cargo resolves to stable and fails with E0554,
# which is the trap the dispatch brief warned about and which this experiment
# hit once before naming the pin).
#
# usage: strip_gate_experiment.sh arvo-strategy | arvo
set -e
PIN=+nightly-2026-05-28
CRATE="${1:-arvo-strategy}"
REPO="$(pwd)"
WORK="/tmp/strip-gate-62/$CRATE"
rm -rf "$WORK" && mkdir -p "$(dirname "$WORK")"
cp -R "$REPO/mock/crates/$CRATE" "$WORK"
cd "$WORK"
# rewrite workspace-inherited manifest fields and point deps back at the tree
python3 - "$REPO" <<'EOF'
import re, sys
repo = sys.argv[1]
m = open("Cargo.toml").read()
m = m.replace("version.workspace = true", 'version = "0.0.0"')
m = m.replace("edition.workspace = true", 'edition = "2024"')
m = m.replace("license.workspace = true", 'license = "MPL-2.0"')
m = m.replace(
    "notko.workspace = true",
    'notko = { path = "%s/../notko", default-features = false, features = ["const", "try_trait_v2"] }' % repo,
)
m = re.sub(
    r"arvo-([a-z-]+)\.workspace = true",
    lambda g: 'arvo-%s = { path = "%s/mock/crates/arvo-%s" }' % (g.group(1), repo, g.group(1)),
    m,
)
open("Cargo.toml", "w").write(m)
lines = open("src/lib.rs").read().splitlines(keepends=True)
idx = [i for i, l in enumerate(lines) if l.strip() == "#![feature(generic_const_exprs)]"]
assert len(idx) == 1, idx
del lines[idx[0]]
open("src/lib.rs", "w").write("".join(lines))
EOF
cargo $PIN check --offline 2>&1 | grep -cE "^error: generic parameters may not be used in const operations" || true
