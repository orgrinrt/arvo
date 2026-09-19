#!/usr/bin/env bash
# Whether the leading-:: path resists the ways this crate could bind the name
# `core`, and whether a bare `core::...` path does not.
#
# One command, from anywhere: `./run.sh` in this directory. Each arm is checked
# at the host, on the pinned toolchain (`rust-toolchain.toml`), with
# `--emit=metadata`, which is the part of a build `cargo check` runs. This is a
# name-resolution question rather than a target one, so no cross target is
# needed. The whole output of every run, its exit status included, goes to
# `run.out` beside this script, so the committed file is the run rather than a
# transcription of it.
set -uo pipefail
cd "$(dirname "$0")"

OUT_DIR=$(mktemp -d)
trap 'rm -rf "$OUT_DIR"' EXIT

check() {
  local arm=$1
  shift
  echo "== ${arm}"
  rustc --edition 2024 --crate-type lib --crate-name "${arm}" --emit=metadata \
    --out-dir "$OUT_DIR" "$@" "${arm}.rs" 2>&1
  echo "exit $?"
  echo
}

{
  echo "tool: $(rustc --version)"
  echo
  check a_mod_core_shadows_the_bare_path
  check a_use_alias_shadows_the_bare_path
  check the_leading_double_colon_resists_both_shadows
  check extern_crate_self_as_core_hijacks_the_absolute_path
  check a_bare_mod_usize_confirms_the_original_premise
  check the_shipped_spelling_alone_is_silent
} | tee run.out
