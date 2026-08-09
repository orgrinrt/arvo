#!/usr/bin/env bash
# Regenerates output.txt on the pin. Every probe here is a spike: it checks one
# thing, its names and arities are scaffolding, and it is cited for what it
# established and never for how it was written.
set -u
T=+nightly-2026-05-28
cd "$(dirname "$0")"
run() { echo "########## $1"; shift; "$@" 2>&1; echo; }
{
  rustc $T --version
  echo
  run "p02 scope shadowing (runs)"            sh -c "rustc $T --edition 2021 -O p02_scope_shadowing.rs -o /tmp/p02 && /tmp/p02"
  run "p03 boundary is a type error (E0308)"  rustc $T --edition 2021 p03_boundary_is_a_type_error.rs -o /tmp/p03
  run "p04 op-level retarget (runs)"          sh -c "rustc $T --edition 2021 -O p04_op_level_retarget.rs -o /tmp/p04 && /tmp/p04"
  run "p05 posture param erases (asm)"        sh -c "rustc $T --edition 2021 -O --emit=asm p05_posture_param_erases.rs -o p05_asm.s && grep -E '= _' p05_asm.s"
  run "p06a fn default type param (refused)"  rustc $T --edition 2021 p06a_fn_default_type_param.rs -o /tmp/p06a
  run "p06b output via mode marker (runs)"    sh -c "rustc $T --edition 2021 p06b_output_via_mode_marker.rs -o /tmp/p06b && /tmp/p06b"
  run "p06c method call ambiguity (E0283)"    rustc $T --edition 2021 p06c_method_call_ambiguity.rs -o /tmp/p06c
  run "p06d mode marker at a method call"     rustc $T --edition 2021 p06d_mode_method_call.rs -o /tmp/p06d
  echo "########## p07 notko granularity: see p07_notko_granularity/NOTES.md"; echo
  run "p08 ambient marker reaches tier two"   sh -c "rustc $T --edition 2021 -O p08_ambient_marker_reaches_tier_two.rs -o /tmp/p08 && /tmp/p08"
  run "p09 resolve laws + control (control MUST fail)" rustc $T --edition 2021 p09_resolve_laws_and_control.rs -o /tmp/p09
  run "p10 resolve erases (asm aliasing)"     sh -c "rustc $T --edition 2021 -O --emit=asm p10_resolve_erases.rs -o p10_asm.s && grep -E '= _' p10_asm.s"
  run "p11 postures do not fold (asm)"        sh -c "rustc $T --edition 2021 -O --emit=asm p11_postures_do_not_fold.rs -o p11_asm.s && grep -nA4 -E '^_under_(hot|cold):' p11_asm.s"
} > output.txt
echo "wrote output.txt"
