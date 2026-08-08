//! One monomorphisation per arm per swept width, exported so each loop can be
//! disassembled in isolation.
//!
//! The bench's arm functions inline thirty monomorphisations each, so counting
//! instructions in one of them mixes every width together. Each function here
//! is exactly one arm at one width, at `D = 3`, which is the density the width
//! sweep runs at.
//!
//! The **whole** swept width set is exported rather than the ratified numeral
//! alone. A count taken at one width is what produced the claim under test.
//!
//! Not a bench. It has no timer, and by this workspace's own standard a
//! measurement outside `mock/benches/` on the harness is an ad-hoc quick spike
//! with no substance. An instruction count is a structural fact, which is the
//! one kind of claim a spike can carry.

use bench_wide_rung_shared::load::{Ragged, WordRound};
use bench_wide_rung_shared::run;

/// # Safety
/// `base` must satisfy the loader's contract for `n` elements at `W = 129`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn walk_ragged_w129(base: *const u8, n: usize, out: *mut u64) {
    let v = run::<Ragged, 129, 0>(base, n);
    unsafe { core::ptr::write(out as *mut [u64; 4], v) };
}

/// # Safety
/// `base` must satisfy the loader's contract for `n` elements at `W = 129`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn walk_wordround_w129(base: *const u8, n: usize, out: *mut u64) {
    let v = run::<WordRound, 129, 0>(base, n);
    unsafe { core::ptr::write(out as *mut [u64; 4], v) };
}

/// # Safety
/// `base` must satisfy the loader's contract for `n` elements at `W = 160`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn walk_ragged_w160(base: *const u8, n: usize, out: *mut u64) {
    let v = run::<Ragged, 160, 0>(base, n);
    unsafe { core::ptr::write(out as *mut [u64; 4], v) };
}

/// # Safety
/// `base` must satisfy the loader's contract for `n` elements at `W = 160`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn walk_wordround_w160(base: *const u8, n: usize, out: *mut u64) {
    let v = run::<WordRound, 160, 0>(base, n);
    unsafe { core::ptr::write(out as *mut [u64; 4], v) };
}

/// # Safety
/// `base` must satisfy the loader's contract for `n` elements at `W = 192`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn walk_ragged_w192(base: *const u8, n: usize, out: *mut u64) {
    let v = run::<Ragged, 192, 0>(base, n);
    unsafe { core::ptr::write(out as *mut [u64; 4], v) };
}

/// # Safety
/// `base` must satisfy the loader's contract for `n` elements at `W = 192`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn walk_wordround_w192(base: *const u8, n: usize, out: *mut u64) {
    let v = run::<WordRound, 192, 0>(base, n);
    unsafe { core::ptr::write(out as *mut [u64; 4], v) };
}

/// # Safety
/// `base` must satisfy the loader's contract for `n` elements at `W = 200`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn walk_ragged_w200(base: *const u8, n: usize, out: *mut u64) {
    let v = run::<Ragged, 200, 0>(base, n);
    unsafe { core::ptr::write(out as *mut [u64; 4], v) };
}

/// # Safety
/// `base` must satisfy the loader's contract for `n` elements at `W = 200`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn walk_wordround_w200(base: *const u8, n: usize, out: *mut u64) {
    let v = run::<WordRound, 200, 0>(base, n);
    unsafe { core::ptr::write(out as *mut [u64; 4], v) };
}

/// # Safety
/// `base` must satisfy the loader's contract for `n` elements at `W = 232`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn walk_ragged_w232(base: *const u8, n: usize, out: *mut u64) {
    let v = run::<Ragged, 232, 0>(base, n);
    unsafe { core::ptr::write(out as *mut [u64; 4], v) };
}

/// # Safety
/// `base` must satisfy the loader's contract for `n` elements at `W = 232`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn walk_wordround_w232(base: *const u8, n: usize, out: *mut u64) {
    let v = run::<WordRound, 232, 0>(base, n);
    unsafe { core::ptr::write(out as *mut [u64; 4], v) };
}

/// # Safety
/// `base` must satisfy the loader's contract for `n` elements at `W = 256`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn walk_ragged_w256(base: *const u8, n: usize, out: *mut u64) {
    let v = run::<Ragged, 256, 0>(base, n);
    unsafe { core::ptr::write(out as *mut [u64; 4], v) };
}

/// # Safety
/// `base` must satisfy the loader's contract for `n` elements at `W = 256`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn walk_wordround_w256(base: *const u8, n: usize, out: *mut u64) {
    let v = run::<WordRound, 256, 0>(base, n);
    unsafe { core::ptr::write(out as *mut [u64; 4], v) };
}
