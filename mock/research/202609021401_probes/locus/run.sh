#!/usr/bin/env bash
# Every arm under both verbs, and a run where a build survives.
#
# stderr is folded in on purpose. A refusal is the result here, and a discarded
# stderr turns every refusal into a silent pass.
set -u
cd "$(dirname "$0")"
echo "toolchain: $(rustc --version 2>&1)"
echo "cargo:     $(cargo --version 2>&1)"
echo

ARMS="a_pair_roundtrip
b_zero_den_runtime_call
c_zero_den_const_call
d_zero_den_verdict_only
e_control_half_step
f_inadmissible_reaches_a_binary
g_unused_const_item
h_phase_const_alone
i_implementor_overrides_the_obligation
j_constructor_assert_costs_a_runtime_panic
j2_constructor_assert_refuses_the_const_site
k_const_generic_denominator
k2_const_generic_denominator_refuses
l_a_blanket_obligation_cannot_be_disarmed
l2_the_blanket_obligation_refuses_the_disarmed_impl
l3_the_blanket_obligation_at_a_runtime_call_site
l4_the_blanket_obligation_has_no_second_impl
m_membership_does_not_force_the_obligation"

for arm in $ARMS; do
    for verb in check build; do
        out=$(cargo "$verb" --bin "$arm" 2>&1)
        rc=$?
        echo "=== $arm | cargo $verb | exit=$rc"
        if [ "$rc" -ne 0 ]; then
            echo "$out" | grep -E '^(error|note:)' | head -6
        fi
    done
    out=$(cargo run --quiet --bin "$arm" 2>&1)
    rc=$?
    echo "=== $arm | cargo run | exit=$rc"
    echo "$out" | grep -vE '^(   Compiling|    Finished|     Running|    Blocking|   Locking)' | head -6
    echo
done
