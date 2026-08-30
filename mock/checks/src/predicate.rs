//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! A predicate names axes somebody declared, in the form the axis declared.
//!
//! The notation is only exact if the vocabulary is closed. Three states per axis
//! and no fourth: listed with a range or `any`, listed with a fixed value, or
//! absent, and absent says the claim holds in no situation where that axis
//! exists at all. **That last reading is what makes the closed set load-bearing
//! rather than tidy**: an axis nobody declared cannot be absent from anything,
//! because nobody knew to look for it, so an undeclared axis silently converts
//! the strongest negative statement in the notation into a shrug.
//!
//! An entry is `<dimension slug>: <values>`. The slug side is checked here. The
//! values side is not, and deliberately: the grammars differ per axis and are
//! prose on the `dimension` row rather than a pattern, because `I in 1..=64`,
//! `operation in {add, mul}` and `target features = host default` have nothing
//! in common a regex would capture without also accepting everything.

use crate::{Finding, Registry, Row};

/// The fields that hold a predicate, and which namespace each belongs to.
///
/// Named rather than discovered, because a field's type is `string[]` and a
/// walker over every `string[]` would check `keywords` and `options` too.
const PREDICATE_FIELDS: &[(&str, &str)] = &[
    ("proposal", "predicate"),
    ("law", "holds"),
    ("law", "fails"),
];

/// Every predicate entry across the canon, as `(row, field, entry)`.
pub fn entries(reg: &Registry) -> Vec<(&Row, &'static str, &str)> {
    let mut out = Vec::new();
    for (namespace, field) in PREDICATE_FIELDS {
        for row in reg.of(namespace) {
            for entry in row.list(field) {
                out.push((row, *field, entry.as_str()));
            }
        }
    }
    out
}

/// The axis slug an entry names, and the values it gives it.
///
/// `None` where the entry has no colon at all, which is the malformed case
/// rather than an unknown axis, and the two are reported separately because the
/// fixes differ: one is a typo and the other is an axis to declare.
pub fn split(entry: &str) -> Option<(&str, &str)> {
    let (slug, values) = entry.split_once(':')?;
    Some((slug.trim(), values.trim()))
}

/// Predicate entries that are malformed or that name no declared axis.
pub fn undeclared_dimensions(reg: &Registry) -> Vec<Finding> {
    let declared = reg.slugs("dimension");
    let mut out = Vec::new();
    for (row, field, entry) in entries(reg) {
        let Some((slug, values)) = split(entry) else {
            out.push(Finding::new(
                "predicate-entry-is-malformed",
                row.addr(),
                format!(
                    "`{field}` carries `{entry}`, which has no `<dimension>: <values>` split. \
                     An entry nothing can parse is a sentence in a field a checker reads, \
                     and it passes every schema."
                ),
            ));
            continue;
        };
        if !declared.contains(&slug) {
            out.push(Finding::new(
                "predicate-names-an-undeclared-dimension",
                row.addr(),
                format!(
                    "`{field}` names the axis `{slug}`, which no `dimension` row declares. \
                     Declare it, or use the slug of the axis that already means this. \
                     An undeclared axis cannot be absent from any other predicate, so \
                     admitting one here weakens every predicate in the canon."
                ),
            ));
            continue;
        }
        if values.is_empty() {
            out.push(Finding::new(
                "predicate-entry-has-no-values",
                row.addr(),
                format!(
                    "`{field}` names `{slug}` and gives it nothing. An axis listed with no \
                     values is neither of the two positive states the notation has, and it \
                     reads as a region while naming none."
                ),
            ));
        }
    }
    out
}

/// One axis named twice in one predicate.
///
/// Two entries for one axis are two regions, and nothing says which governs.
/// The schema cannot see it: a `string[]` with two entries is a valid array.
pub fn repeated_dimensions(reg: &Registry) -> Vec<Finding> {
    let mut out = Vec::new();
    for (namespace, field) in PREDICATE_FIELDS {
        for row in reg.of(namespace) {
            let mut seen: Vec<&str> = Vec::new();
            for entry in row.list(field) {
                let Some((slug, _)) = split(entry) else {
                    continue; // reported by the arm above
                };
                if seen.contains(&slug) {
                    out.push(Finding::new(
                        "predicate-names-one-axis-twice",
                        row.addr(),
                        format!(
                            "`{field}` names `{slug}` more than once. Two regions on one axis \
                             and nothing says which holds; write the one region the claim was \
                             established in."
                        ),
                    ));
                } else {
                    seen.push(slug);
                }
            }
        }
    }
    out
}
