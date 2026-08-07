//! What rustc bakes: array stride and field offset are emitted constants.
#![no_std]
#![crate_type = "lib"]
#[derive(Clone, Copy)]
pub struct Narrow(u8);
#[derive(Clone, Copy)]
pub struct Wide32([u8; 32]);

#[unsafe(no_mangle)]
pub fn idx_narrow(a: &[Narrow; 4096], i: usize) -> u8 {
    a[i].0
}
#[unsafe(no_mangle)]
pub fn idx_wide(a: &[Wide32; 4096], i: usize) -> u8 {
    a[i].0[0]
}
#[unsafe(no_mangle)]
pub fn sz_narrow() -> usize {
    core::mem::size_of::<[Narrow; 4096]>()
}
#[unsafe(no_mangle)]
pub fn sz_wide() -> usize {
    core::mem::size_of::<[Wide32; 4096]>()
}
