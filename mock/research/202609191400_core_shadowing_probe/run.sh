#!/usr/bin/env bash
# Which ways of binding the name `core` reach the leading-:: path the
# platform-width aliases read, and which only shadow a bare `core::...` path.
#
# One command, from anywhere: `./run.sh` in this directory. Every arm in this
# directory is run, each with the outcome it is expected to have: `builds`, or
# `refused` together with the text the compiler's refusal has to contain, so an
# arm refused for some other reason counts as unexpected rather than as a pass.
# The single-file arms are checked with `rustc --emit=metadata`, the part of a
# build `cargo check` runs, and the manifest-rename fixtures under
# `manifest_rename/` with `cargo check --locked`. All of it runs at the host on
# the pinned toolchain (`rust-toolchain.toml`); this is a name-resolution
# question rather than a target one, so no cross target is needed.
#
# The whole output of every run, exit statuses and verdicts included, goes to
# `run.out` beside this script, so the committed file is the run rather than a
# transcription of it. The script exits non-zero when any arm's outcome is not
# the expected one, and the last line of `run.out` says how many were.
set -uo pipefail
cd "$(dirname "$0")"

OUT_DIR=$(mktemp -d)
trap 'rm -rf "$OUT_DIR"' EXIT

unexpected=0

# judge <status> <expected> <log> [<text the refusal contains>]
judge() {
  local status=$1 expected=$2 log=$3 text=${4-}
  echo "exit ${status}"
  if [ "$expected" = builds ] && [ "$status" -eq 0 ]; then
    echo "verdict: as expected, builds"
  elif [ "$expected" = refused ] && [ "$status" -ne 0 ] && grep -qF -- "$text" "$log"; then
    echo "verdict: as expected, refused with: ${text}"
  else
    echo "verdict: UNEXPECTED, expected ${expected}${text:+ with: ${text}}"
    unexpected=$((unexpected + 1))
  fi
  echo
}

# check <arm> <expected> [<text>]: one single-file arm.
check() {
  local arm=$1 expected=$2 text=${3-}
  local log="${OUT_DIR}/${arm}.log"
  echo "== ${arm}.rs"
  rustc --edition 2024 --crate-type lib --crate-name "${arm}" --emit=metadata \
    --out-dir "$OUT_DIR" "${arm}.rs" >"$log" 2>&1
  local status=$?
  cat "$log"
  judge "$status" "$expected" "$log" "$text"
}

# check_manifest <crate> <expected> [<text>]: one fixture under manifest_rename/.
check_manifest() {
  local crate=$1 expected=$2 text=${3-}
  local log="${OUT_DIR}/${crate}.log"
  echo "== manifest_rename/${crate}"
  (cd "manifest_rename/${crate}" && cargo check --quiet --locked) >"$log" 2>&1
  local status=$?
  cat "$log"
  judge "$status" "$expected" "$log" "$text"
}

main() {
  echo "tool: $(rustc --version)"
  echo "tool: $(cargo --version)"
  echo

  # The bare spelling is shadowed by a module and by a `use` alias.
  check a_mod_core_shadows_the_bare_path builds
  check a_use_alias_shadows_the_bare_path builds
  check a_bare_mod_usize_confirms_the_original_premise builds

  # The leading `::` resists both.
  check the_leading_double_colon_resists_both_shadows builds
  check the_use_alias_does_not_reach_the_leading_colon_path builds

  # An `extern crate self as core` at the crate root reaches it, plain or raw.
  check extern_crate_self_as_core_hijacks_the_absolute_path builds
  check the_raw_ident_extern_crate_self_hijacks_the_leading_colon_path builds

  # The same alias out of a macro is refused by the compiler.
  check a_macro_expanded_extern_crate_self_as_core_is_refused refused \
    'macro-expanded `extern crate` items cannot shadow names passed with `--extern`'

  # A dependency renamed to `core` in the manifest reaches it whenever the
  # renamed crate carries the path the edition's prelude import names: the
  # real `core` re-exported whole, the real `prelude` module alone, or an empty
  # hand-written `prelude::rust_2024`. Without that path it stops at the
  # prelude import.
  check_manifest user builds
  check_manifest user_of_the_crate_with_only_the_prelude builds
  check_manifest user_of_the_crate_with_an_empty_prelude builds
  check_manifest user_of_the_crate_without_the_glob refused \
    'cannot resolve a prelude import'

  # The positive control and the negative control.
  check the_shipped_spelling_alone_is_silent builds
  check the_bare_spelling_without_the_leading_colon_is_shadowed refused \
    "the bare spelling read the \`mod core\` shadow's 8 rather than the host's pointer width"

  echo "unexpected outcomes: ${unexpected}"
  [ "$unexpected" -eq 0 ]
}

main 2>&1 | tee run.out
exit "${PIPESTATUS[0]}"
