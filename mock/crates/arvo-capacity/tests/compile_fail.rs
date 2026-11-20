//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital

//! What must not compile, each beside a twin that differs in the offending
//! part and does.
//!
//! `trybuild` builds each file and holds the diagnostic, so a refusal cannot go
//! away without a test saying so, and a twin that stopped compiling would make
//! its pair say nothing, which is why each pair is one test.

fn pair(refused: &str, twin: &str) {
    let t = trybuild::TestCases::new();
    t.compile_fail(format!("tests/ui/{refused}.rs"));
    t.pass(format!("tests/ui/{twin}.rs"));
}

#[test]
fn an_outside_impl_of_capacity_is_refused() {
    pair(
        "an_outside_impl_of_capacity_is_refused",
        "dim_used_from_outside_compiles",
    );
}

#[test]
fn the_seal_s_length_cannot_be_read_from_outside() {
    pair(
        "the_seal_s_length_is_read_from_outside",
        "the_count_is_read_through_count",
    );
}

#[test]
fn the_seal_s_length_field_cannot_be_read_from_outside() {
    pair(
        "the_seal_s_length_field_is_read_from_outside",
        "the_count_is_read_through_count",
    );
}

#[test]
fn a_dim_cannot_be_constructed() {
    pair("a_dim_is_constructed", "a_dim_is_named_as_a_type");
}
