//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The consumer crates under `tests/consumer/`, each built by `trybuild` as a crate of its own with
//! no feature line and run to its end, so a gate leaking out of `arvo-contracts` or `arvo-numeral`
//! into a consumer stops this test.

#[test]
fn a_consumer_reaches_every_contract_with_no_feature_line() {
    let t = trybuild::TestCases::new();
    t.pass("tests/consumer/every_item_without_a_feature_line.rs");
}
