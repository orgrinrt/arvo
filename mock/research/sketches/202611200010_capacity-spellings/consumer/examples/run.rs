//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital

//! Runs the consumer's generic code at two budgets and arm B's best case.

use consumer::{DefaultPlanDims, SmallPlanDims, WINDOW_COUNT, build_and_walk};

fn main() {
    let (default_sum, default_cores) = build_and_walk::<DefaultPlanDims>();
    let (small_sum, small_cores) = build_and_walk::<SmallPlanDims>();
    // 0 + 1 + ... + 255 = 32640, plus the 7 written into the nested array.
    assert_eq!(default_sum, 32640 + 7);
    assert_eq!(default_cores, 8);
    // 0 + 1 + 2 + 3 = 6, plus 7.
    assert_eq!(small_sum, 6 + 7);
    assert_eq!(small_cores, 2);
    assert_eq!(WINDOW_COUNT, 4);
    println!(
        "WORKS: default sum={default_sum} cores={default_cores}; small sum={small_sum} cores={small_cores}; window count={WINDOW_COUNT}"
    );
}
