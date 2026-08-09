// p4: can a declarative macro derive a bitfield's placement map?
//
// File 61 established that `macro_rules!` cannot ingest a decimal literal's
// digits, which is why the notation vehicle must be a proc-macro. This probe
// asks the same question of the bitfield's own arithmetic: offsets as a prefix
// sum over declared widths.
//
// The answer is different, and the reason is that the widths arrive as separate
// tokens. Nothing needs decomposing; the sum is accumulated as a token sequence
// and handed to the const evaluator unevaluated.
//
// NO FEATURE GATES. Edition 2024. Plain rustc.

macro_rules! bitfield_derived {
    // entry
    (
        $(#[$m:meta])*
        pub struct $name:ident : $container:ty , $n:literal {
            $( $f:ident : $w:literal ),* $(,)?
        }
    ) => {
        $(#[$m])*
        #[derive(Copy, Clone, PartialEq, Eq, Debug)]
        pub struct $name($container);

        impl $name {
            pub const BITS: u32 = $n;
            pub const fn zero() -> Self { let _ = Self::_FITS; Self(0) }
            pub const fn from_raw(v: $container) -> Self { Self(v) }
            pub const fn to_raw(self) -> $container { self.0 }

            // occupancy: the placement map's union. With derived offsets and no
            // declared holes this is the plain sum, and it is the quantity the
            // datum-keyed digest masks to.
            pub const OCCUPANCY: u32 = 0 $( + $w )*;
            // total by construction: an over-committed declaration must reach
            // the stated assertion below, not overflow on the way to it.
            pub const OCCUPANCY_MASK: $container =
                if Self::OCCUPANCY == 0 { 0 }
                else if Self::OCCUPANCY >= <$container>::BITS { !0 }
                else { ((1 as $container) << Self::OCCUPANCY) - 1 };

            // containment, as a fact about the type: no door, no use required.
            const _FITS: () = {
                assert!(Self::OCCUPANCY <= $n, "declared fields exceed the container");
                assert!($n <= <$container>::BITS, "container narrower than N");
            };
        }

        bitfield_derived!(@emit $name, $container, [] $(, $f : $w)*);
    };

    // recursive arm: `$($acc)*` carries the prefix sum as unevaluated tokens.
    (@emit $name:ident, $container:ty, [$($acc:tt)*] , $f:ident : $w:literal $(, $rest:ident : $rw:literal)*) => {
        impl $name {
            #[inline(always)]
            pub const fn $f(self) -> $container {
                (self.0 >> (0 $($acc)*)) & (((1 as $container) << $w) - 1)
            }
        }
        bitfield_derived!(@emit $name, $container, [$($acc)* + $w] $(, $rest : $rw)*);
    };

    // base case
    (@emit $name:ident, $container:ty, [$($acc:tt)*]) => {};
}

bitfield_derived! {
    /// heterogeneous widths, offsets never written by the consumer
    pub struct Pixel: u32, 32 {
        r: 11,
        g: 11,
        b: 10,
    }
}

bitfield_derived! {
    /// a declared hole is an ordinary named field; nothing is lost by
    /// dropping hand-written offsets.
    pub struct Reg: u16, 16 {
        enable: 1,
        reserved: 4,
        divisor: 9,
    }
}

fn main() {
    // offsets are 0, 11, 22 by construction; nothing declared them.
    let p = Pixel::from_raw(0b_10_1010_1010__101_0101_0101__111_1111_1111);
    println!("r = {:#013b}", p.r());
    println!("g = {:#013b}", p.g());
    println!("b = {:#012b}", p.b());
    println!(
        "OCCUPANCY = {}, mask = {:#034b}",
        Pixel::OCCUPANCY,
        Pixel::OCCUPANCY_MASK
    );

    let q = Reg::from_raw(0b_1_0101_0101__1010__1);
    println!(
        "enable = {}, reserved = {:#06b}, divisor = {:#011b}",
        q.enable(),
        q.reserved(),
        q.divisor()
    );
    println!(
        "Reg OCCUPANCY = {}, mask = {:#018b}",
        Reg::OCCUPANCY,
        Reg::OCCUPANCY_MASK
    );

    // exhaustive round-trip over a 32-bit container is out of reach; sweep the
    // whole 16-bit one instead, every value, both directions.
    let mut bad = 0u32;
    for v in 0u16..=u16::MAX {
        let r = Reg::from_raw(v);
        let rebuilt =
            (r.enable() as u16) | ((r.reserved() as u16) << 1) | ((r.divisor() as u16) << 5);
        if rebuilt != (v & Reg::OCCUPANCY_MASK) {
            bad += 1;
        }
    }
    println!("16-bit exhaustive rebuild mismatches: {}", bad);
}
