//! The loader says so when it read nothing, instead of handing back an empty
//! registry that every other arm here reads as a clean one.
//!
//! Almost every check in this crate sweeps `canon()` and asserts the finding
//! list is empty. That shape is right, and it has one failure mode: a registry
//! that did not load is indistinguishable from a registry with nothing wrong in
//! it. The suite goes green over zero rows and reports the canon healthy.
//!
//! It was reachable. `walk` returned `Ok(())` for a path that is not a
//! directory, so `load` on a wrong or missing path returned an empty registry
//! rather than an error, and the `.expect` in `canon()` could not fire. Found
//! by a probe pointed at the wrong directory getting a clean run.
//!
//! Two guards now, because they catch different things. `load` refuses a path
//! that is not a directory, which is the wrong-path case. `canon()` refuses an
//! empty result, which is the directory-exists-and-is-empty case that no error
//! from the filesystem would report.

use std::path::Path;

/// The wrong-path case: an error rather than an empty registry.
#[test]
fn a_path_that_is_not_a_directory_is_an_error() {
    let err = arvo_checks::load(Path::new("mock/registry/there-is-no-such-place"))
        .expect_err("a missing directory returned a registry");
    assert_eq!(err.kind(), std::io::ErrorKind::NotFound, "{err}");
}

/// A file is not a directory either, and this is the likelier typo: naming one
/// document instead of the tree it sits in.
#[test]
fn a_file_where_a_directory_was_meant_is_an_error() {
    // Built from the manifest directory rather than from `file!()`, which is
    // relative to the workspace root and does not resolve from where the test
    // binary runs.
    let owned = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let file = owned.as_path();
    assert!(file.is_file(), "the fixture for this test moved: {file:?}");

    let err = arvo_checks::load(file).expect_err("a file returned a registry");
    assert_eq!(err.kind(), std::io::ErrorKind::NotFound, "{err}");
}

/// The positive control, without which the two above prove only that the
/// function can fail.
#[test]
fn the_real_registry_still_loads_and_is_not_empty() {
    let reg = arvo_checks::canon();
    assert!(
        !reg.rows.is_empty(),
        "the canon loaded zero rows, which is what this whole file exists to stop"
    );
    assert!(
        reg.of("ruling").count() > 0,
        "rows loaded but no `ruling` namespace among them, so the walk reached \
         something other than the registry"
    );
}
