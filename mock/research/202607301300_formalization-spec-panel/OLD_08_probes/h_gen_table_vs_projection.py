import sys, os
# Two encodings of "is this exponent integral", at N widths.
# TABLE: one impl per exponent value (the spec's D73 macro-expanded table).
# PROJ : one blanket impl reading an associated const (06 sec 7's shape).
n = int(sys.argv[1]); mode = sys.argv[2]; out = sys.argv[3]
L = ["#![allow(dead_code)]",
     "pub trait TruthMarker { const VALUE: bool; }",
     "pub struct True; pub struct False;",
     "impl TruthMarker for True { const VALUE: bool = true; }",
     "impl TruthMarker for False { const VALUE: bool = false; }",
     "pub struct Implicit<const E: i32>;",
     "pub trait Integral { type Out: TruthMarker; }"]
if mode == "table":
    for e in range(-n//2, n//2):
        L.append(f"impl Integral for Implicit<{e}> {{ type Out = {'True' if e>=0 else 'False'}; }}")
else:
    L.append("pub trait ExpVal { const E: i32; }")
    L.append("impl<const E: i32> ExpVal for Implicit<E> { const E: i32 = E; }")
    L.append("pub trait IntegralC { const OUT: bool; }")
    L.append("impl<T: ExpVal> IntegralC for T { const OUT: bool = T::E >= 0; }")
# consumer sites: one bound per exponent, so the solver actually runs
if mode == "table":
    L.append("pub fn need<T: Integral>() {}")
    for e in range(-n//2, n//2):
        L.append(f"pub fn u{e+n}() {{ need::<Implicit<{e}>>() }}")
else:
    L.append("pub fn need<T: IntegralC>() -> bool { T::OUT }")
    for e in range(-n//2, n//2):
        L.append(f"pub fn u{e+n}() -> bool {{ need::<Implicit<{e}>>() }}")
open(out,"w").write("\n".join(L)+"\n")
