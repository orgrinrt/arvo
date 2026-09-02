// a02: does Zig have arvo's bridge problem?
//
// Zig answer: types ARE comptime values, so a function from a width to a type is
// an ordinary function. @Int builds an integer type of exactly N bits, so the
// container derivation is not even a case split: the language ships arbitrary
// widths natively.
//
// Zig 0.16.0. In this version the reification builtin is @Int(signedness, bits);
// std.meta.Int is a deprecated wrapper over it (std/meta.zig:754).
//
// Compile: zig build-obj a02_zig_comptime_derivation.zig -O ReleaseFast -femit-bin=out/a02.o

const std = @import("std");

// the ladder: one function, no table, arithmetic on the width freely
fn Container(comptime n: u16) type {
    if (n <= 64) return @Int(.unsigned, n);
    return [(n + 63) / 64]u64;
}

fn Fixed(comptime i: u16, comptime f: u16) type {
    return struct { raw: Container(i + f) };
}

comptime {
    // arbitrary widths, no declaration per width
    std.debug.assert(@sizeOf(Fixed(3, 0)) == 1);
    std.debug.assert(@sizeOf(Fixed(13, 3)) == 2);
    std.debug.assert(@sizeOf(Fixed(13, 13)) == 4);
    std.debug.assert(@sizeOf(Fixed(100, 100)) == 32);
    std.debug.assert(@sizeOf(Fixed(4711, 1)) == 8 * ((4712 + 63) / 64));
    // and the exact-width integer is a first-class type
    std.debug.assert(@bitSizeOf(Container(13)) == 13);
    std.debug.assert(@bitSizeOf(Container(3)) == 3);
}

// erasure check: the derived container is the native one, so the two bodies
// below must be identical. Passed by container type because Zig refuses an
// auto-layout struct across the C calling convention.
const F16 = Fixed(13, 3);
export fn arvo16(ar: Container(16), br: Container(16)) u16 {
    const a: F16 = .{ .raw = ar };
    const b: F16 = .{ .raw = br };
    return a.raw +% b.raw;
}
export fn native16(a: u16, b: u16) u16 {
    return a +% b;
}
