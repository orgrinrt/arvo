//! Dense `N × N` matrix wrapper.
//!
//! `Matrix<W, N>` + `cap_size(c: Cap)` live in `arvo-tensor` as of
//! round 202604280000. This module is the thin forwarder that keeps
//! the `arvo_spectral::Matrix` and `arvo_spectral::matrix::cap_size`
//! import paths working for existing consumers.

pub use arvo_tensor::{Matrix, cap_size};
