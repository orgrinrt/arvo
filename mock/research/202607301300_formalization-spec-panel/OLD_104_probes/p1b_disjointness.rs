// p1b: the placement map's missing half, discharged.
//
// The declared-offset form's obligations are containment (each field inside the
// container) and pairwise disjointness. The shipped macro asserts the first and
// documents the second as the author's ("Overlap detection is deferred to a
// future macro version (for now, authors are responsible)",
// arvo/src/bitfield.rs:29-30); p1 compiles what that costs.
//
// Both are const-evaluable over the declared list. This probe discharges the
// second in a free anonymous const, per p4c's finding that a free const item is
// the placement that fires without a construction door.
//
// The refusal is the default, not the law: a deliberately aliasing view field is
// a real bitfield idiom, so the escape is a declaration (`alias`) rather than an
// absent check. Toolbox, not policer.
//
// No feature gates. Edition 2024.

macro_rules! placement {
    (
        pub struct $name:ident : $n:literal {
            $( $f:ident : $w:literal at $o:literal ),* $(,)?
        }
    ) => {
        #[derive(Copy, Clone)]
        pub struct $name(u64);
        impl $name {
            pub const MAP: &'static [(u32, u32)] = &[ $( ($o, $w) ),* ];
            pub const OCCUPANCY_MASK: u64 = { let mut m = 0u64; let mut i = 0;
                while i < Self::MAP.len() { let (o, w) = Self::MAP[i];
                    m |= (((1u64 << w) - 1) << o); i += 1; } m };
            $( #[inline(always)] pub const fn $f(self) -> u64 {
                (self.0 >> $o) & ((1u64 << $w) - 1) } )*
        }
        // containment and disjointness, both facts about the type, in a free
        // const item so that neither depends on a constructor mentioning them.
        const _: () = {
            let map = $name::MAP;
            let mut i = 0;
            while i < map.len() {
                let (o, w) = map[i];
                assert!(o + w <= $n, "field runs past the container");
                let mut j = i + 1;
                while j < map.len() {
                    let (p, v) = map[j];
                    assert!(o + w <= p || p + v <= o, "two fields overlap");
                    j += 1;
                }
                i += 1;
            }
            assert!($n <= 64, "container wider than the macro's carrier");
        };
    };
}

placement! {
    pub struct StrHandle: 32 {
        origin: 1 at 31,
        reserved: 3 at 28,
        id: 28 at 0,
    }
}

// Uncomment to see the refusal; recorded in OUTCOMES.md.
// placement! {
//     pub struct Overlap: 16 {
//         a: 8 at 0,
//         b: 8 at 4,
//     }
// }

fn main() {
    let h = StrHandle(0x8000_0042);
    println!(
        "origin = {}, reserved = {}, id = {:#x}",
        h.origin(),
        h.reserved(),
        h.id()
    );
    println!("occupancy mask = {:#034b}", StrHandle::OCCUPANCY_MASK);
}
