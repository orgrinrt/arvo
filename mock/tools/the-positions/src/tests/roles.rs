//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The role reading, and the names it must not take.
//!
//! A classifier that answers one bucket for everything looks, in a distribution,
//! exactly like a stack that writes one kind of number. So the tests here come
//! in pairs: a name that must land in a role, and a name that must not.

use crate::role::{all, of};

#[test]
fn a_rationals_halves_are_not_cardinalities() {
    // `numerator` and `denominator` are the two commonest integers in an exact
    // arithmetic and neither counts anything. They read as `count` under any
    // vocabulary that has `num` in it and does not have these.
    assert_eq!(of("numerator", "", "i64"), "rational");
    assert_eq!(of("denominator", "", "i64"), "rational");
    assert_eq!(of("den", "", "i64"), "rational");
    assert_ne!(of("numerator", "", "i64"), of("count", "", "i64"));
}

#[test]
fn the_control_the_rational_needles_do_not_swallow_a_plain_number() {
    assert_eq!(
        of("num", "", "u32"),
        "count",
        "`num` is a count and not a rational half"
    );
    assert_ne!(of("denominator", "", "i64"), "count");
}

#[test]
fn a_bit_width_reads_as_a_bit_width_under_every_spelling_in_use() {
    for name in [
        "width",
        "bit_width",
        "bits",
        "nbits",
        "num_bits",
        "BITS",
        "BitWidth",
    ] {
        assert_eq!(
            of(name, "", "u32"),
            "bit-width",
            "`{name}` did not read as a bit width"
        );
    }
}

#[test]
fn the_control_a_width_and_a_count_are_not_the_same_role() {
    assert_ne!(
        of("width", "", "u32"),
        of("count", "", "u32"),
        "the two roles that most obviously want different primitives collapsed into one"
    );
}

#[test]
fn a_cardinality_reads_as_a_count_and_a_coordinate_reads_as_an_index() {
    for name in ["count", "len", "length", "arity", "rank"] {
        assert_eq!(of(name, "", "usize"), "count", "`{name}`");
    }
    for name in ["index", "idx", "offset", "cursor", "slot"] {
        assert_eq!(of(name, "", "usize"), "index", "`{name}`");
    }
}

#[test]
fn the_control_a_token_match_does_not_fire_on_a_longer_word() {
    // Each of these contains a role keyword as a substring and must not take it.
    let must_not = [
        ("capture", "capacity"),
        ("name", "count"),
        ("basement", "radix"),
        ("scaled_thing_name", "count"),
        ("identifier_of_nothing_here", "count"),
    ];
    for (name, forbidden) in must_not {
        assert_ne!(
            of(name, "", "u32"),
            forbidden,
            "`{name}` matched `{forbidden}` on a substring rather than a token"
        );
    }
}

#[test]
fn a_bool_is_truth_whatever_it_is_called() {
    // A `bool` named `count` is a naming defect and not a cardinality, and the
    // replacement it wants is decided by its type rather than by its name.
    for name in ["count", "width", "index", "is_ready", "", "xyzzy"] {
        assert_eq!(of(name, "", "bool"), "truth", "`{name}`");
    }
}

#[test]
fn the_control_an_integer_named_like_a_flag_is_not_silently_truth() {
    // The converse of the rule above: the type decides for `bool` and the name
    // decides for an integer, so a `u8` named `flags` must not be filed as truth
    // just because the word looks like one.
    assert_ne!(
        of("flags", "", "u8"),
        "truth",
        "an integer took `bool`'s shortcut"
    );
}

#[test]
fn a_float_with_no_readable_name_falls_back_to_a_real_quantity() {
    assert_eq!(of("xyzzy", "", "f64"), "real");
    assert_eq!(of("", "", "f32"), "real");
}

#[test]
fn the_control_an_integer_with_no_readable_name_stays_unclassified() {
    // The float fallback must not leak to integers, where it would invent a
    // distribution out of nothing. Unclassified is the honest answer and it is
    // the one that tells a reader where the work is.
    assert_eq!(of("xyzzy", "", "u32"), "unclassified");
    assert_eq!(of("", "", "usize"), "unclassified");
}

#[test]
fn the_enclosing_item_is_read_when_the_identifier_says_nothing() {
    assert_eq!(
        of("raw", "BitWidth", "u32"),
        "opaque-bits",
        "the row's own name wins"
    );
    assert_eq!(
        of("xyzzy", "BitWidth", "u32"),
        "bit-width",
        "the owner did not supply the role"
    );
}

#[test]
fn the_control_the_owner_does_not_override_the_rows_own_name() {
    assert_eq!(
        of("index", "BitWidth", "u32"),
        "index",
        "the enclosing item overrode a name that said what the position is"
    );
}

#[test]
fn every_declared_role_is_reachable_by_at_least_one_name() {
    // A role nobody can ever reach is a bucket that will read as zero in every
    // report forever, which is indistinguishable from a stack that has none.
    let probes: &[(&str, &str)] = &[
        ("bit_width", "bit-width"),
        ("count", "count"),
        ("numerator", "rational"),
        ("index", "index"),
        ("capacity", "capacity"),
        ("stride", "stride"),
        ("exponent", "exponent"),
        ("radix", "radix"),
        ("id", "identity"),
        ("mask", "opaque-bits"),
        ("is_ready", "truth"),
        ("ratio", "real"),
        ("errno", "code"),
        ("addr", "address"),
        ("version", "version"),
        ("nanos", "time"),
        ("xyzzy", "unclassified"),
    ];
    for (name, want) in probes {
        assert_eq!(
            of(name, "", "u32"),
            *want,
            "`{name}` did not reach `{want}`"
        );
    }
    let reached: Vec<&str> = probes.iter().map(|(_, r)| *r).collect();
    for role in all() {
        assert!(
            reached.contains(&role),
            "`{role}` is declared and unreachable"
        );
    }
}
