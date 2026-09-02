//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! What a position means, read off the identifier attached to it.
//!
//! **The grammatical kind says whether a replacement is owed. The role says what
//! the replacement would have to be.** A `usize` naming a length and a `usize`
//! naming an index into that length are the same type and want different
//! primitives, because one is a cardinality closed under addition and the other
//! is a coordinate that is only meaningful against a bound. Counting them
//! together answers "how many positions" and nothing about "what arvo has to
//! supply", which is the question the obligation actually asks.
//!
//! # This is a reading and not a measurement
//!
//! An identifier is evidence of intent and not a statement of it. `n` is a
//! cardinality most of the time and an exponent sometimes; `size` is a byte
//! count here and an element count there. The classification is by keyword over
//! the identifier and the enclosing item's name, it is wrong on individual rows,
//! and it is reported as a distribution rather than as a fact about any one row.
//!
//! What makes it worth having anyway: the shape of the distribution is robust to
//! individual misreadings, and the alternative on a corpus of this size is
//! reading several thousand identifiers by hand, which nobody will redo and
//! which therefore cannot be checked.
//!
//! **`--role <role>` lists a group in full so a reader can check it**, which is
//! the whole reason the tool reports the identifier beside the count.

/// A semantic role, in the order a report lists them.
///
/// Ordered by how sharply the role constrains a replacement rather than by
/// frequency: a bit width admits one shape, a raw cardinality admits several,
/// and an unclassified position admits nothing until somebody reads it.
pub const ROLES: &[(&str, &[&str])] = &[
    // --- roles a single primitive could plausibly serve ---
    (
        "bit-width",
        &[
            "width",
            "bits",
            "bit_width",
            "bitwidth",
            "nbits",
            "num_bits",
            "bit_count",
            "bitcount",
        ],
    ),
    (
        "count",
        &[
            "count",
            "len",
            "length",
            "num",
            "n",
            "size",
            "arity",
            "rank",
            "degree",
            "cardinality",
            "total",
            "amount",
            "quantity",
        ],
    ),
    (
        "index",
        &[
            "index", "idx", "position", "offset", "cursor", "slot", "which", "nth", "ordinal",
        ],
    ),
    (
        "capacity",
        &[
            "capacity", "cap", "limit", "max_", "bound", "budget", "ceiling", "quota",
        ],
    ),
    (
        "stride",
        &[
            "stride",
            "align",
            "alignment",
            "pitch",
            "step",
            "granularity",
            "pad",
            "padding",
        ],
    ),
    (
        "exponent",
        &["exponent", "exp", "shift", "scale", "log", "pow", "power"],
    ),
    ("radix", &["radix", "base"]),
    (
        "identity",
        &[
            "id",
            "key",
            "handle",
            "token",
            "tag",
            "hash",
            "seed",
            "fingerprint",
            "digest",
            "uid",
            "ident",
            "symbol",
            "name_id",
            "salt",
        ],
    ),
    (
        "opaque-bits",
        &[
            "mask", "raw", "word", "payload", "repr", "encoded", "packed", "byte",
        ],
    ),
    (
        "truth",
        &[
            "is_", "has_", "was_", "can_", "should_", "flag", "enabled", "allow", "present",
            "found", "ok", "valid", "dirty", "ready", "done", "signed", "empty",
        ],
    ),
    (
        "real",
        &[
            "ratio",
            "factor",
            "weight",
            "fraction",
            "coefficient",
            "gain",
            "rate",
            "delta",
            "epsilon",
            "tolerance",
            "threshold",
            "probability",
            "score",
            "cost",
        ],
    ),
    (
        "code",
        &["errno", "code", "status", "errcode", "os_error", "exit"],
    ),
    (
        "address",
        &["addr", "address", "ptr", "pointer", "vaddr", "physical"],
    ),
    (
        "version",
        &["version", "abi", "revision", "generation", "epoch", "vers"],
    ),
    (
        "time",
        &[
            "nanos", "micros", "millis", "seconds", "duration", "instant", "elapsed", "timeout",
            "deadline", "tick",
        ],
    ),
];

/// The role a position's identifier reads as, or `unclassified`.
///
/// Checks the position's own identifier first and the enclosing item second, so
/// a field called `raw` on a type called `BitWidth` reads as a bit width rather
/// than as opaque bits. A tie inside one string goes to the earliest role in
/// `ROLES`, which is the one that constrains a replacement hardest.
#[must_use]
pub fn of(name: &str, owner: &str, primitive: &str) -> &'static str {
    // `bool` is truth whatever it is called. Nothing else can be, and a `bool`
    // named `count` is a naming defect rather than a cardinality.
    if primitive == "bool" {
        return "truth";
    }
    let lowered = normalise(name);
    if let Some(role) = matches(&lowered) {
        return role;
    }
    let owner_lowered = normalise(owner);
    if let Some(role) = matches(&owner_lowered) {
        return role;
    }
    // A float names a real quantity by default. There is no width or index in
    // this stack carried as a float, so the fallback is not a guess in the way
    // an integer's would be.
    if primitive == "f32" || primitive == "f64" {
        return "real";
    }
    "unclassified"
}

fn matches(haystack: &str) -> Option<&'static str> {
    for (role, needles) in ROLES {
        for needle in *needles {
            if contains_token(haystack, needle) {
                return Some(role);
            }
        }
    }
    None
}

/// Snake case, lowered, with a leading and trailing separator so a needle
/// anchored with `_` matches at either end.
fn normalise(s: &str) -> String {
    let mut out = String::from("_");
    let mut prev_lower = false;
    for ch in s.chars() {
        if ch.is_ascii_uppercase() {
            if prev_lower {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
            prev_lower = false;
        } else if ch.is_ascii_alphanumeric() {
            out.push(ch);
            prev_lower = ch.is_ascii_lowercase() || ch.is_ascii_digit();
        } else {
            out.push('_');
            prev_lower = false;
        }
    }
    out.push('_');
    out
}

/// Whether `needle` occurs in `haystack` on a token boundary.
///
/// A needle already carrying a trailing `_` is a prefix match on a token, which
/// is what `is_` and `max_` want. One without is a whole-token match, so `cap`
/// does not fire on `capture` and `base` does not fire on `basement`.
///
/// **A one or two character needle is never written in the prefix form**, and
/// the suite pins why: `n_` matched `name` and `e_` matched `errno`, so the two
/// commonest identifiers in a numeric library landed in `count` and `exponent`
/// before anything else got a look. Both are whole tokens now.
fn contains_token(haystack: &str, needle: &str) -> bool {
    if let Some(stem) = needle.strip_suffix('_') {
        let pat = format!("_{stem}");
        return haystack.contains(&pat);
    }
    let pat = format!("_{needle}_");
    haystack.contains(&pat)
}

/// Every role name, in order, with `unclassified` last, so a report enumerates
/// rather than listing only what it happened to find.
#[must_use]
pub fn all() -> Vec<&'static str> {
    let mut v: Vec<&'static str> = ROLES.iter().map(|(name, _)| *name).collect();
    v.push("unclassified");
    v
}
