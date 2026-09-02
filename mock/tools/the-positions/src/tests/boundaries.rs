//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The positions where the host type is a contract rather than a choice.

use crate::walk::walk;

#[test]
fn an_extern_c_function_is_a_boundary() {
    let found = walk(
        "t",
        "src/lib.rs",
        "pub extern \"C\" fn go(code: i32) -> u32 { 0 }",
    );
    assert!(!found.is_empty());
    assert!(found.iter().all(|f| f.boundary), "{found:?}");
}

#[test]
fn a_no_mangle_export_is_a_boundary() {
    let source = "#[no_mangle]\npub fn vehje_runtime_execute(n: usize) -> i32 { 0 }";
    let found = walk("t", "src/lib.rs", source);
    assert!(found.iter().all(|f| f.boundary), "{found:?}");
}

#[test]
fn a_repr_c_struct_is_a_boundary() {
    let source = "#[repr(C)]\npub struct Frame { pub code: u32 }";
    let found = walk("t", "src/lib.rs", source);
    assert!(found.iter().all(|f| f.boundary), "{found:?}");
}

#[test]
fn a_file_under_a_platform_directory_is_a_boundary_whatever_it_declares() {
    for path in [
        "mock/crates/c/src/platform/os.rs",
        "mock/crates/c/src/backend/unix.rs",
        "mock/crates/c/src/thread/parking.rs",
        "mock/crates/vehje-runtime-abi/src/exports.rs",
    ] {
        let found = walk("t", path, "pub fn f(n: usize) {}");
        assert!(!found.is_empty(), "`{path}` yielded nothing at all");
        assert!(found[0].boundary, "`{path}` was not read as a boundary");
    }
}

#[test]
fn the_control_an_ordinary_function_in_an_ordinary_file_is_not_a_boundary() {
    // Without this the axis could be answering "yes" everywhere, which would
    // excuse every position in the stack and report the demand at zero.
    let found = walk(
        "t",
        "mock/crates/c/src/plan/graph.rs",
        "pub fn f(n: usize) -> u32 { 0 }",
    );
    assert_eq!(found.len(), 2);
    assert!(found.iter().all(|f| !f.boundary), "{found:?}");
}

#[test]
fn the_control_the_boundary_read_moves_with_one_attribute() {
    let plain = walk("t", "src/lib.rs", "pub fn f(n: usize) {}");
    let marked = walk("t", "src/lib.rs", "#[no_mangle]\npub fn f(n: usize) {}");
    assert!(!plain[0].boundary);
    assert!(marked[0].boundary);
}

#[test]
fn the_control_a_path_component_is_matched_and_not_a_substring() {
    // `os` and `abi` are two letters and three, so a substring match would take
    // half the stack: `mock/crates/hilavitkutin-providers` contains neither as
    // a component and contains `os` inside `providers`.
    let found = walk(
        "t",
        "mock/crates/hilavitkutin-providers/src/interner.rs",
        "pub fn f(n: usize) {}",
    );
    assert!(
        !found[0].boundary,
        "a substring match took an ordinary crate"
    );
    let also = walk(
        "t",
        "mock/crates/composite/src/lib.rs",
        "pub fn f(n: usize) {}",
    );
    assert!(!also[0].boundary);
}

#[test]
fn a_free_position_is_in_the_demand_and_off_a_boundary() {
    let free = walk(
        "t",
        "mock/crates/c/src/plan/graph.rs",
        "pub fn f(n: usize) {}",
    );
    assert!(free[0].is_demand());
    assert!(free[0].is_free());
    let bound = walk(
        "t",
        "mock/crates/c/src/platform/os.rs",
        "pub fn f(n: usize) {}",
    );
    assert!(
        bound[0].is_demand(),
        "a boundary position is still a position"
    );
    assert!(!bound[0].is_free());
}
