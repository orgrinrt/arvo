#!/usr/bin/env nutshell
# Which results in this repository can name the build profile they were taken at.
#
# `build_profile` is a declared axis in `mock/registry/dimension.toml`. Under the
# absence rule an axis a finding does not name is an axis the finding does not
# hold over at all, so a result that cannot state its profile holds at no
# profile. This counts who can state it.
#
# Three populations, because they fail differently:
#
#   1. `mock/benches/*.meta.json`, the harness artifacts. These are the only
#      measurements in the repository entitled to the word bench.
#   2. probe files that invoke `rustc` and therefore choose an optimisation
#      level, whether or not they say so.
#   3. probe files that read a clock.
#
# Required outcomes, written before the run:
#
#   C1  `168_probes/p5_run.sh` must land in the naming set. It compiles at two
#       named profiles and prints the flag string for each.
#   C2  at least one compiling probe must land in the silent set, and the set
#       must be non-empty. Otherwise the matcher says yes to everything.
#   C3  a bare `-O` counts as naming an optimisation level and `--emit asm` does
#       not, because emitting assembly says nothing about how it was optimised.
#       Both planted.
#   C4  the meta arm must be able to report a nonzero. Planted: a synthetic meta
#       carrying `"profile"` must be counted, or a zero over the real ones is a
#       fact about the grep.
#
# C2 and C4 are the arms that matter.
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
panel="$(cd "$here/.." && pwd)"
repo="$(cd "$panel/../../.." && pwd)"
out="$here/p5_build_profile.out"

# Naming a profile means naming an optimisation level, an LTO setting, a
# codegen-unit count, or the cargo profile. `--emit asm` is not one.
PROF='(^|[^A-Za-z0-9-])-O([^A-Za-z0-9-]|$)|opt-level|--release|debug-assertions|-Clto|-C lto|lto *=|codegen-units|\[profile'
COMPILES='rustc|cargo build|cargo test|cargo run|cargo bench'
TIMES='Instant::now|perf_counter|time\.time\(|clock_gettime|hyperfine|std::time'

{
  printf '=== p5 build profile, %s\n\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"

  printf '## population 1: the bench harness artifacts\n'
  metas=$(ls "$repo"/mock/benches/*.meta.json 2>/dev/null | wc -l | tr -d ' ')
  withprof=$( { grep -lE 'profile|lto|codegen|opt-level' "$repo"/mock/benches/*.meta.json 2>/dev/null || true; } | wc -l | tr -d ' ')
  dirty=$( { grep -l 'dirty' "$repo"/mock/benches/*.meta.json 2>/dev/null || true; } | wc -l | tr -d ' ')
  printf 'meta files:                       %s\n' "$metas"
  printf 'recording any profile field:      %s\n' "$withprof"
  printf 'recording a dirty git tree:       %s\n' "$dirty"
  printf 'fields a meta actually carries:   '
  head -1 "$(ls "$repo"/mock/benches/*.meta.json | head -1)" | tr ',' '\n' | sed 's/[{}"]//g' | cut -d: -f1 | tr '\n' ' '
  printf '\n\n'
  printf 'C4: plant a meta that does carry one, and confirm the grep can see it\n'
  tmpm=$(mktemp -d)
  printf '{"cpu":"x","profile":"release-fat-lto"}\n' > "$tmpm/planted.meta.json"
  if grep -lE 'profile|lto|codegen|opt-level' "$tmpm"/*.meta.json >/dev/null 2>&1; then
    printf 'C4 PASS: the planted meta is found, so %s of %s is a fact about the files\n' "$withprof" "$metas"
  else
    printf 'C4 FAIL: the grep cannot see a profile field even when one is there\n'
  fi
  rm -rf "$tmpm"
  printf '\n'

  printf '## C3: what counts as naming a profile\n'
  tmp=$(mktemp)
  for line in 'rustc -O foo.rs' 'rustc --emit asm foo.rs' 'rustc -C opt-level=3 foo.rs' \
              'cargo test -p x' 'rustc --crate-type lib foo.rs' 'nm -g libfoo.dylib'; do
    printf '%s\n' "$line" > "$tmp"
    if grep -qE "$PROF" "$tmp"; then printf '  names a profile:     %s\n' "$line"
    else printf '  names none:          %s\n' "$line"; fi
  done
  rm -f "$tmp"
  printf '  (expected: -O yes, --emit asm no, opt-level yes, cargo test no,\n'
  printf '   --crate-type no, nm -g no. A yes on nm -g would mean the -O arm\n'
  printf '   is matching a bare hyphen-letter anywhere.)\n\n'

  printf '## population 2: probe files that compile something\n'
  compilers=$( { cd "$panel" && grep -rlE "$COMPILES" --include='*.sh' --include='*.py' --include='*.rs' . 2>/dev/null || true; } \
    | grep '_probes/' | grep -v '^\./185_probes/' | sed 's|^\./||' | sort )
  nc=$(printf '%s\n' "$compilers" | grep -c . || true)
  named=''; silent=''
  for f in $compilers; do
    if grep -qE "$PROF" "$panel/$f" 2>/dev/null; then named="$named$f"$'\n'; else silent="$silent$f"$'\n'; fi
  done
  n_named=$(printf '%s' "$named" | grep -c . || true)
  n_silent=$(printf '%s' "$silent" | grep -c . || true)
  printf 'probe files invoking a compiler:  %s\n' "$nc"
  printf 'naming an optimisation setting:   %s\n' "$n_named"
  printf 'naming none:                      %s\n' "$n_silent"
  printf '\n'
  printf 'C1: 168_probes/p5_run.sh must be in the naming set\n'
  if printf '%s' "$named" | grep -qx '168_probes/p5_run.sh'; then printf 'C1 PASS\n'
  else printf 'C1 FAIL: a script that prints its own flag string per profile is not counted\n'; fi
  printf 'C2: the silent set must be non-empty\n'
  if [ "$n_silent" -gt 0 ]; then printf 'C2 PASS: %s\n' "$n_silent"
  else printf 'C2 FAIL: everything counted\n'; fi
  printf '\n'
  printf 'the silent ones, in full:\n'
  printf '%s' "$silent" | sed 's/^/  /'
  printf '\n'

  printf '## population 3: probe files that read a clock\n'
  timers=$( { cd "$panel" && grep -rlE "$TIMES" --include='*.sh' --include='*.py' --include='*.rs' . 2>/dev/null || true; } \
    | grep '_probes/' | grep -v '^\./185_probes/' | sed 's|^\./||' | sort )
  for f in $timers; do
    if grep -qE "$PROF" "$panel/$f" 2>/dev/null; then p=names; else p=SILENT; fi
    printf '  %-8s %s\n' "$p" "$f"
  done
  printf 'timing probe files: %s\n' "$(printf '%s\n' "$timers" | grep -c . || true)"
} > "$out" 2>&1
cat "$out"
