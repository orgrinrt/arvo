#!/usr/bin/env nutshell
# Runs `p2_definition_site_refusal.rs`. H1 and H3 are the positive controls and
# must compile; H2, H4 and H5 must be refused. H4 and H5 are the ones `p1`
# could not achieve: naming the bad type without ever constructing or using it.
set -uo pipefail
src="p2_definition_site_refusal.rs"
[ -f "$src" ] || { echo "run me from 191_probes" >&2; exit 2; }
tmp=$(mktemp -d); trap 'rm -rf "$tmp"' EXIT
echo "### rustc: $(rustc --version)"
echo
run() {
  name="$1"; expect="$2"; shift 2
  out=$(rustc --edition 2021 -o "$tmp/a.out" "$src" "$@" 2>&1); rc=$?
  if [ $rc -eq 0 ]; then got="COMPILED"; else got="REFUSED "; fi
  if [ "$got" = "$expect" ]; then v="as required"; else v="*** NOT AS REQUIRED ***"; fi
  echo "######## $name"
  echo "         required=$expect  got=$got   $v"
  [ $rc -eq 0 ] || printf '%s\n' "$out" | head -14 | sed 's/^/    /'
  echo
}
run "BASE  no arms"                                    "COMPILED"
run "H1    capacity 200 in 8 bits  (POSITIVE CONTROL)" "COMPILED" --cfg arm_h1
run "H3    capacity 300 in 9 bits  (POSITIVE CONTROL)" "COMPILED" --cfg arm_h3
run "H2    capacity 300 in 8 bits, in a signature"     "REFUSED " --cfg arm_h2
run "H4    capacity 300 in 8 bits, only in a field"    "REFUSED " --cfg arm_h4
run "H5    capacity 300 in 8 bits, only aliased"       "REFUSED " --cfg arm_h5
run "H6    the alias then held in a field"                "REFUSED " --cfg arm_h6
run "H7    the alias then used in a signature"            "REFUSED " --cfg arm_h7

cat <<'NOTE'
### Reading H5, which is left failing on purpose.
###
### H5 is the one arm that does not do what the ideal wants: a bare `type`
### alias naming an ill-fitting shape compiles. That is Rust's general
### `type_alias_bounds` behaviour rather than anything about this relation, and
### H6 and H7 bound it: the moment the alias is used, in a field or in a
### signature, the refusal arrives with the same diagnostic. So the residue is a
### deferral to the use site and not a hole, and the arm is left red rather than
### relabelled, because relabelling it would hide the one place the mechanism is
### weaker than "refuses at the definition site" claims.
###
### The composition this run establishes, against `p1`:
###   const assertion        refuses only where the const is forced. Naming,
###                          returning, aliasing and holding all compile. (p1 G)
###   type-level relation    refuses at every position that uses the type,
###                          through an alias included. (p2 H2, H4, H6, H7)
###   where-clause const     refused terminally, names generic_const_exprs. (p1 D)
NOTE
