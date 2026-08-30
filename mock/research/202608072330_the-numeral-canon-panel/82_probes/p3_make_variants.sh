#!/usr/bin/env bash
# Generates the two refusal variants from p3a by a single stated edit each, so
# the difference between the compiling case and the refused case is auditable
# rather than asserted.
#
#   p3b: the consumer declares a window that STRADDLES ZERO. Everything else is
#        identical to p3a. Expected: E0080 naming the reason.
#
#   p3c: the CLOSED FORM is perturbed to p2's mutant M1 (`lo >= -1 || hi <= 0`,
#        which allows one negative). The consumer's declaration is unchanged and
#        is one the honest closed form licenses. Expected: E0080 from the
#        cross-check, not from the consumer's declaration, because a wrong
#        closed form must be caught before any arm is gated on it.
set -euo pipefail
cd "$(dirname "$0")"

# p3b: add a straddling declaration and a consumer that instantiates it.
sed -e 's|^pub fn total_drawdown|pub type Straddles = Win<-128, 127>;\npub fn total_delta(xs: \&[i8]) -> i8 {\n    fold_reassociated::<Straddles>(xs)\n}\n\npub fn total_drawdown|' \
    p3a_the_construction.rs > p3b_straddling_declaration_refused.rs

# p3c: perturb the closed form only.
sed -e 's|^    lo >= 0 \|\| hi <= 0$|    lo >= -1 \|\| hi <= 0|' \
    p3a_the_construction.rs > p3c_perturbed_closed_form_refused.rs

echo "generated:"
echo "  p3b diff against p3a:"
diff p3a_the_construction.rs p3b_straddling_declaration_refused.rs || true
echo "  p3c diff against p3a:"
diff p3a_the_construction.rs p3c_perturbed_closed_form_refused.rs || true

# p3d_bad: a straddling declaration used in DEAD CODE (a plain `pub fn` that
# nothing reaches, which is exactly the shape that compiled clean under the
# const-assert construction in p3b).
sed -e 's|^#\[no_mangle\]\npub extern "C" fn total_drawdown|X|' p3d_structural_permission.rs > /dev/null
python3 - <<'PY'
src = open('p3d_structural_permission.rs').read()
add = '''
/// A straddling window, used only from a plain `pub fn` that nothing reaches.
/// Under the const-assert construction this compiled clean (p3b). Under the
/// structural construction it must not.
pub type Delta = Win<-128, 127>;
pub fn total_delta_dead_code(xs: &[i8]) -> i8 {
    fold_reassociated::<Delta>(xs)
}
'''
marker = '#[no_mangle]\npub extern "C" fn total_drawdown'
assert marker in src, "marker not found"
src = src.replace(marker, add + '\n' + marker, 1)
open('p3d_bad_straddling_dead_code.rs','w').write(src)
print("wrote p3d_bad_straddling_dead_code.rs")
PY
