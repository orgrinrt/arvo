//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! Do the two untested enforcers actually have the holes claimed, or does the
//! claim just read well?
//!
//! Run: `rustc --edition 2021 -O --test p4_the_untested_enforcers_have_holes.rs -o /tmp/p4 && /tmp/p4 --test-threads=1`
//!
//! The predicates below are transcribed verbatim from `mock/lints/no_std_enforcer.rs`
//! and `mock/lints/no_alloc_enforcer.rs` as they stand on this branch. Transcribed
//! rather than linked because the lints compile against the mockspace lint ABI and
//! this probe is checking the string matching alone, which is where the holes are.
//! If a later reader finds the lint source and this file disagree, the lint source
//! is the truth and this probe is stale.
//!
//! What must fail, stated before the run: `the_controls_catch_what_the_lints_do_catch`
//! asserts the transcribed predicates fire on the cases they are built for. If that
//! test fails, the transcription is wrong and every miss below is an artefact of my
//! copying rather than a hole in the lint.
//!
//! **The first cut of this file overstated the false-positive class** and this probe
//! caught it: it claimed any identifier ending in the word would fire, and named
//! `CString`. It does not. The token is a space then the word, so `" CString"` has a
//! `C` where the token wants `S` and no match occurs. What actually fires is an
//! identifier *beginning* with the word behind whitespace. The narrower claim is
//! what is asserted below, and the wrong one is recorded here rather than quietly
//! fixed, because the difference is the whole reason to run the thing.

#![allow(dead_code)]

/// Verbatim from `no_std_enforcer.rs`: the comment skip, then the five prefixes.
fn no_std_enforcer_fires(line: &str) -> bool {
    let trimmed = line.trim_start();
    if trimmed.starts_with("//") {
        return false;
    }
    trimmed.starts_with("use std::")
        || trimmed.starts_with("use ::std::")
        || trimmed.starts_with("pub use std::")
        || trimmed.starts_with("pub use ::std::")
        || trimmed.starts_with("extern crate std")
}

/// Verbatim from `no_alloc_enforcer.rs`: the import arm, then the token arm. The
/// token arm tests the untrimmed line, which is the detail that decides two cases.
fn no_alloc_enforcer_fires(line: &str) -> bool {
    let trimmed = line.trim_start();
    if trimmed.starts_with("//") {
        return false;
    }
    if trimmed.starts_with("use alloc::")
        || trimmed.starts_with("pub use alloc::")
        || trimmed.starts_with("extern crate alloc")
    {
        return true;
    }
    let line_body = line;
    for (token, _display) in [
        ("Vec<", "Vec<T>"),
        (" String", "String"),
        ("Box<", "Box<T>"),
    ] {
        if line_body.contains(token) {
            if trimmed.starts_with("///") || trimmed.starts_with("//!") {
                continue;
            }
            return true;
        }
    }
    false
}

// --- the control, which decides whether anything below means anything ---------

#[test]
fn the_controls_catch_what_the_lints_do_catch() {
    // no-std-enforcer, the cases it is built for.
    assert!(no_std_enforcer_fires("use std::fmt;"));
    assert!(no_std_enforcer_fires("    use std::collections::BTreeMap;"));
    assert!(no_std_enforcer_fires("pub use std::fmt::Debug;"));
    assert!(no_std_enforcer_fires("use ::std::fmt;"));
    assert!(no_std_enforcer_fires("extern crate std;"));
    assert!(!no_std_enforcer_fires("// use std::fmt;"));

    // no-alloc-enforcer, the cases it is built for.
    assert!(no_alloc_enforcer_fires("use alloc::vec::Vec;"));
    assert!(no_alloc_enforcer_fires("extern crate alloc;"));
    assert!(no_alloc_enforcer_fires("    let v: Vec<u8> = todo!();"));
    assert!(no_alloc_enforcer_fires(
        "    let b: Box<dyn Trait> = todo!();"
    ));
    assert!(no_alloc_enforcer_fires("    let s: String = todo!();"));
    assert!(!no_alloc_enforcer_fires("// Vec<u8> in a comment"));
}

// --- hole 1: a qualified path never looks like an import ----------------------

#[test]
fn no_std_enforcer_misses_every_qualified_path_expression() {
    // None of these is an import, all of them reach the platform, and the lint
    // is the only thing standing between them and a `#![no_std]` crate.
    let reaches_std_without_importing = [
        "    std::process::exit(0);",
        "    let n = std::mem::size_of::<u32>();",
        "    std::thread::sleep(d);",
        "    let t = std::time::Instant::now();",
        "    ::std::process::abort();",
        "    let _ = std::fs::read(\"/etc/passwd\");",
    ];
    for line in reaches_std_without_importing {
        assert!(
            !no_std_enforcer_fires(line),
            "the enforcer was expected to miss this and did not, so the hole is \
             narrower than claimed: {line}"
        );
    }
}

#[test]
fn no_std_enforcer_misses_a_renaming_import_and_a_braced_one() {
    // `use std as x` does not start with `use std::`.
    assert!(!no_std_enforcer_fires("use std as platform;"));
    // A brace-first import does not either.
    assert!(!no_std_enforcer_fires("use {std::fmt, core::mem};"));
    // Whitespace inside the path defeats the prefix.
    assert!(!no_std_enforcer_fires("use std :: fmt;"));
    // An attribute in front of the import defeats it.
    assert!(!no_std_enforcer_fires(
        "#[cfg(feature = \"x\")] use std::fmt;"
    ));
}

// --- hole 2: the space before the word ----------------------------------------

#[test]
fn no_alloc_enforcer_misses_string_at_column_zero() {
    // The token is `" String"`, matched against the untrimmed line, so the word
    // is only seen when something precedes it. At column zero nothing does.
    assert!(
        !no_alloc_enforcer_fires("String::from(\"x\");"),
        "expected the column-zero case to be missed"
    );
    assert!(
        !no_alloc_enforcer_fires("String::new();"),
        "expected the column-zero case to be missed"
    );

    // And the same expression indented is caught, purely because the indentation
    // supplies the space the token needs. The lint's verdict depends on layout.
    assert!(no_alloc_enforcer_fires("    String::from(\"x\");"));
}

#[test]
fn no_alloc_enforcer_false_positives_on_an_identifier_beginning_with_the_word() {
    // Whitespace then an identifier that starts with the word but is not it.
    for line in [
        "    let x: Stringify = todo!();",
        "    struct StringBuilder;",
        "    fn f(v: Strings) {}",
    ] {
        assert!(
            no_alloc_enforcer_fires(line),
            "expected a false positive here and did not get one, so the class is \
             narrower still: {line}"
        );
    }
}

#[test]
fn the_false_positive_class_is_narrower_than_ending_in_the_word() {
    // The claim this probe's first cut made, kept as a permanent negative so it
    // cannot come back. `" CString"` has a `C` where the token wants `S`.
    for line in [
        "    fn f(c: CString) {}",
        "    let e: OsString = todo!();",
        "    struct MyStringBuilder;",
    ] {
        assert!(
            !no_alloc_enforcer_fires(line),
            "an identifier merely containing the word was expected not to fire: {line}"
        );
    }
}

#[test]
fn no_alloc_enforcer_fires_inside_a_string_literal_and_a_block_comment() {
    // Neither is code. Both are reported.
    assert!(no_alloc_enforcer_fires(
        "    let msg = \"pass a Vec<u8> to the sink\";"
    ));
    assert!(no_alloc_enforcer_fires("    /* returns a Vec<u8> */"));
}

// --- what this establishes, and what it does not ------------------------------
//
// It establishes that the string matching in these two enforcers admits source
// that violates the constraint they exist to enforce, and reports source that
// does not. It does not establish that the other three untested enforcers have
// holes; nobody has looked, which is its own finding.
