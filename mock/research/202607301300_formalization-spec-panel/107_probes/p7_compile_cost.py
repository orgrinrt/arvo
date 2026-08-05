#!/usr/bin/env python3
"""Probe 7. What the derivation costs the compiler, measured rather than guessed.

The pricing pillar (`102:90-95`, op's own words at `77b:10-22`) says compile
time is the bucket we pour into. That licenses a cost; it does not excuse
declining to measure one. This probe measures three shapes at growing capacity:

  const   the const-parameter capacity: `Dim<const N>` with `type Array<T> = [T; N]`.
          The shipped shape (`arvo-tensor/src/capacity.rs:44-48`, read as a
          factual check on what exists). Zero gates.
  derived probe 4's shape: an inductive numeral with storage recurred alongside
          it through two `repr(C)` combinators. Zero gates.
  paired  the ratified shape: an inductive numeral plus a companion literal,
          checked to agree. Zero gates.

Each is compiled with one capacity instantiated, the storage type named, and the
size const-asserted, so the layout is genuinely computed rather than skipped.

MUST BE RUN WITH CWD INSIDE THE TREE (pinned nightly-2026-05-28). Outside it,
rustc resolves to stable and two of the three shapes report different errors for
unrelated reasons.
"""

import subprocess
import sys
import time
import os

PRELUDE_NUMERAL = """#![no_std]
use core::marker::PhantomData;
pub struct H;
pub struct O<P>(PhantomData<P>);
pub struct I<P>(PhantomData<P>);
pub trait Pos { const VAL: usize; }
impl Pos for H { const VAL: usize = 1; }
impl<P: Pos> Pos for O<P> { const VAL: usize = 2 * P::VAL; }
impl<P: Pos> Pos for I<P> { const VAL: usize = 2 * P::VAL + 1; }
"""

DERIVED_IMPLS = """#[repr(C)] #[derive(Clone, Copy)] pub struct Twin<A>(A, A);
#[repr(C)] #[derive(Clone, Copy)] pub struct TwinOne<A, T>(A, A, T);
pub trait Capacity: Pos { type Array<T: Copy>: Copy; }
impl Capacity for H { type Array<T: Copy> = [T; 1]; }
impl<P: Capacity> Capacity for O<P> { type Array<T: Copy> = Twin<P::Array<T>>; }
impl<P: Capacity> Capacity for I<P> { type Array<T: Copy> = TwinOne<P::Array<T>, T>; }
"""

PAIRED_IMPLS = """pub struct Slot<P, const K: usize>(PhantomData<P>);
pub trait Capacity { const VAL: usize; type Array<T: Copy>: Copy; }
impl<P: Pos, const K: usize> Capacity for Slot<P, K> {
    const VAL: usize = { assert!(P::VAL == K, "AGREES"); K };
    type Array<T: Copy> = [T; K];
}
"""

CONST_IMPLS = """#![no_std]
pub struct Dim<const N: usize>;
pub trait Capacity { const VAL: usize; type Array<T: Copy>: Copy; }
impl<const N: usize> Capacity for Dim<N> {
    const VAL: usize = N;
    type Array<T: Copy> = [T; N];
}
"""


def numeral(n):
    """LSB-outermost binary spelling in the sealed grammar."""
    assert n >= 1
    if n == 1:
        return "H"
    inner = numeral(n // 2)
    return ("I<%s>" if n % 2 else "O<%s>") % inner


def source(shape, n):
    if shape == "const":
        return CONST_IMPLS + (
            "pub type C = Dim<%d>;\n"
            "pub type S = <C as Capacity>::Array<u32>;\n"
            "const _: () = assert!(core::mem::size_of::<S>() == %d * 4);\n"
            "const _: () = assert!(<C as Capacity>::VAL == %d);\n" % (n, n, n)
        )
    if shape == "derived":
        return PRELUDE_NUMERAL + DERIVED_IMPLS + (
            "pub type C = %s;\n"
            "pub type S = <C as Capacity>::Array<u32>;\n"
            "const _: () = assert!(core::mem::size_of::<S>() == %d * 4);\n"
            "const _: () = assert!(<C as Pos>::VAL == %d);\n" % (numeral(n), n, n)
        )
    if shape == "paired":
        return PRELUDE_NUMERAL + PAIRED_IMPLS + (
            "pub type C = Slot<%s, %d>;\n"
            "pub type S = <C as Capacity>::Array<u32>;\n"
            "const _: () = assert!(core::mem::size_of::<S>() == %d * 4);\n"
            "const _: () = assert!(<C as Capacity>::VAL == %d);\n" % (numeral(n), n, n, n)
        )
    raise ValueError(shape)


def time_compile(shape, n, reps=5):
    path = "/tmp/p7_%s_%d.rs" % (shape, n)
    with open(path, "w") as f:
        f.write(source(shape, n))
    best = None
    for _ in range(reps):
        t0 = time.perf_counter()
        r = subprocess.run(
            ["rustc", "--edition", "2024", "--crate-type=lib", path,
             "-o", "/tmp/p7out.rlib"],
            capture_output=True,
        )
        dt = time.perf_counter() - t0
        if r.returncode != 0:
            return None, r.stderr.decode()[:400]
        best = dt if best is None else min(best, dt)
    return best, None


def main():
    ver = subprocess.run(["rustc", "--version"], capture_output=True).stdout.decode().strip()
    print("toolchain: %s" % ver)
    print("cwd: %s" % os.getcwd())
    if "nightly" not in ver:
        print("REFUSING: not the pinned toolchain. Run with cwd inside the tree.")
        sys.exit(1)
    sizes = [1, 7, 13, 47, 64, 255, 256, 1023, 1024, 4095, 4096,
             16384, 65535, 65536, 262144, 1048576]
    print()
    print("%-10s %10s %10s %10s   %s" % ("N", "const", "derived", "paired", "numeral depth"))
    for n in sizes:
        row = []
        for shape in ("const", "derived", "paired"):
            t, err = time_compile(shape, n)
            row.append("%10.3f" % t if t is not None else "%10s" % "REFUSED")
            if err:
                print("   %s at N=%d: %s" % (shape, n, err.splitlines()[0] if err.splitlines() else ""))
        depth = numeral(n).count("<") + 1
        print("%-10d %s %s %s   %d" % (n, row[0], row[1], row[2], depth))


if __name__ == "__main__":
    main()
