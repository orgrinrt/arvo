#!/usr/bin/env bash
#
# TWO OUTPUTS, AND READ BOTH. `f_the_url_mismatch.out` is the run at engine
# revision `b57007c`, where every control passes and the defect is real.
# `f_the_url_mismatch_after_the_pin_moved.out` is the same script at `a7dd8223`,
# which the launcher resolved to mid-dispatch, and there F2 and F3 report FAIL.
# That is the instrument saying the defect is repaired: the engine now writes a
# `[patch]` table for BOTH spellings, the pack builds with no workaround, and
# F3 fails because rewriting the https table into a second ssh one is now a
# duplicate key. A control that fires on a fixed tree is the control working.
# F. Why `cargo mock` builds nothing at this base, isolated to one string.
#
# Found while trying to run `d_can_a_structural_claim_state_its_region.sh`,
# which plants rows and asks the shipped gate to judge them. The gate answered
# "BLOCKED: this repo's custom lints could not be built, so no lint below them
# ran. Nothing was checked." That output is `d_run_full.txt`.
#
# The engine writes the generated lint crate's manifest itself, on every run,
# and it writes the mockspace dependency and its `[patch]` table with the
# `https://` spelling:
#
#   mockspace = { package = "mockspace-lint-rules",
#                 git = "https://github.com/hiisi-digital/mockspace.git", rev = "..." }
#   [patch."https://github.com/hiisi-digital/mockspace.git"]
#
# while `mockspace.toml`'s `[lint-crates]`, `mockspace-extra-lints`' own
# manifest and all five of this repo's tool crates spell it `ssh://git@`. Cargo
# keys a git source by the literal URL string, so the patch never reaches the
# ssh dependents, two copies of `mockspace-lint-rules` enter one graph, and the
# `LintPack` one hands the other is a different type with the same name.
#
# The global `url."git@github.com:".insteadOf "https://github.com/"` git config
# is why the https spelling FETCHES fine and is exactly why nobody noticed: git
# rewrites the transport and cargo still keys the source by what the manifest
# said.
#
# It went unseen because the cdylib was cached. The commit hook, `cargo mock
# query` and `cargo mock test` all reused an object built before the engine pin
# moved, and reported green. `rm -rf mock/target/mockspace-lints/target` is what
# made it visible.
#
# Controls, written before the run:
#   F1  the two spellings must actually differ in the tree, read out of the two
#       files rather than asserted here.
#   F2  the build must FAIL with the engine's spelling. A run where it succeeds
#       means the graph has changed and this file describes nothing.
#   F3  the build must SUCCEED with the ssh spelling and nothing else changed.
#       One of the two has to move or the diagnosis is not isolated to the URL.
#   F4  the engine must rewrite the manifest back on the next `cargo mock`, or
#       the fix would simply be to edit the generated file and this would be a
#       local annoyance rather than an engine defect.
set -uo pipefail
cd "$(dirname "$0")"
ROOT=$(cd ../../../.. && pwd)
GEN=$ROOT/mock/target/mockspace-lints

echo "### F1, the two spellings, read out of the tree"
echo "  the engine's generated manifest:"
grep -n 'mockspace.git' "$GEN/Cargo.toml" | sed 's/^/    /'
echo "  what everything else uses:"
grep -n 'mockspace.git' "$ROOT/mockspace.toml" | sed 's/^/    /'
grep -Hn 'mockspace.git' "$ROOT"/mock/tools/*/Cargo.toml | sed "s|$ROOT/|    |"
https=$(grep -c 'https://github.com/hiisi-digital/mockspace.git' "$GEN/Cargo.toml")
echo "  https spellings in the generated manifest: $https"
[ "$https" -ge 2 ] && echo "  PASS, the dependency and the patch both use it" \
  || echo "  FAIL, the manifest no longer looks like this"
echo

echo "### F2, the build with the engine's spelling"
rm -rf "$GEN/target" "$GEN/Cargo.lock"
( cd "$GEN" && cargo build --release 2>&1 | tail -3 ) | sed 's/^/    /'
( cd "$GEN" && cargo build --release >/dev/null 2>&1 ) && f2=ok || f2=failed
echo "  build: $f2"
[ "$f2" = "failed" ] && echo "  PASS, it fails as described" || echo "  FAIL, it built"
echo

echo "### F3, the same manifest with the one string changed"
cp "$GEN/Cargo.toml" /tmp/f_gen_manifest.https
awk '{gsub(/https:\/\/github.com\/hiisi-digital\/mockspace.git/,"ssh://git@github.com/hiisi-digital/mockspace.git"); print}' \
  /tmp/f_gen_manifest.https > "$GEN/Cargo.toml"
diff /tmp/f_gen_manifest.https "$GEN/Cargo.toml" | sed 's/^/    /'
rm -rf "$GEN/target" "$GEN/Cargo.lock"
( cd "$GEN" && cargo build --release 2>&1 | tail -2 ) | sed 's/^/    /'
( cd "$GEN" && cargo build --release >/dev/null 2>&1 ) && f3=ok || f3=failed
echo "  build: $f3"
[ "$f3" = "ok" ] && echo "  PASS, one string is the whole of it" \
  || echo "  FAIL, something else is also wrong"
echo

echo "### F4, does the engine write it back"
( cd "$ROOT" && cargo mock --lint-only >/dev/null 2>&1 ) || true
after=$(grep -c 'https://github.com/hiisi-digital/mockspace.git' "$GEN/Cargo.toml")
echo "  https spellings after one \`cargo mock --lint-only\`: $after"
[ "$after" -ge 2 ] && echo "  PASS, the engine rewrites it, so this is not fixable in this repo" \
  || echo "  FAIL, the edit survived and the workaround is durable"
