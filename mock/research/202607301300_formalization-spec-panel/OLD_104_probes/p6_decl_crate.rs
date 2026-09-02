// p6 (b): the same surface as an ordinary library crate exporting a
// `macro_rules!`, which is what the declarative form costs.
#[macro_export]
macro_rules! bitfield_decl {
    ($name:ident : $n:literal { $($f:ident : $w:literal),* $(,)? }) => {
        pub struct $name(u64);
        impl $name { pub const OCCUPANCY: u32 = 0 $( + $w )*; }
    };
}
