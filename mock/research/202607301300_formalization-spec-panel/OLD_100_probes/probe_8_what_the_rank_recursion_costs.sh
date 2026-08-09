#!/usr/bin/env bash
# Probe 8. What the rank-generic recursion costs at compile time, and what it
# emits at runtime. The fourth design rule makes compile time a bucket to pour
# into rather than a cost to minimise, so the question is not whether it is free
# but whether it is bounded and what the runtime buys.
#
# Toolchain: the tree's pin. Run from the repo root so `rustc` resolves through
# `rust-toolchain.toml` (outside the tree the same command resolves to stable
# and the const-position work does not build).
set -euo pipefail
cd "$(dirname "$0")/../../../.."   # repo root
OUT=$(mktemp -d)
trap 'rm -rf "$OUT"' EXIT

echo "toolchain: $(rustc --version)  target: $(rustc -vV | sed -n 's/^host: //p')"
echo

gen() {  # $1 = rank
  local n=$1
  {
    echo '#![no_std]'
    echo 'use core::marker::PhantomData;'
    echo 'mod seal { pub trait Sealed {} }'
    echo 'pub struct H; pub struct O<P>(PhantomData<P>); pub struct I<P>(PhantomData<P>);'
    echo 'pub struct Z; pub struct Pz<P>(PhantomData<P>);'
    echo 'impl seal::Sealed for H {} impl<P: Pos> seal::Sealed for O<P> {}'
    echo 'impl<P: Pos> seal::Sealed for I<P> {} impl seal::Sealed for Z {}'
    echo 'impl<P: Pos> seal::Sealed for Pz<P> {}'
    echo 'pub trait Pos: seal::Sealed { const VAL: usize; }'
    echo 'impl Pos for H { const VAL: usize = 1; }'
    echo 'impl<P: Pos> Pos for O<P> { const VAL: usize = 2 * P::VAL; }'
    echo 'impl<P: Pos> Pos for I<P> { const VAL: usize = 2 * P::VAL + 1; }'
    echo 'pub trait Nat: seal::Sealed { const VAL: usize; }'
    echo 'impl Nat for Z { const VAL: usize = 0; }'
    echo 'impl<P: Pos> Nat for Pz<P> { const VAL: usize = P::VAL; }'
    echo 'pub trait Capacity: Nat { type Array<T: Copy>: AsRef<[T]> + AsMut<[T]> + Copy;'
    echo '  const AGREES: bool; fn filled<T: Copy>(v: T) -> Self::Array<T>; }'
    echo 'pub struct Slot<N, const K: usize>(PhantomData<N>);'
    echo 'impl<N: Nat, const K: usize> seal::Sealed for Slot<N, K> {}'
    echo 'impl<N: Nat, const K: usize> Nat for Slot<N, K> { const VAL: usize = N::VAL; }'
    echo 'impl<N: Nat, const K: usize> Capacity for Slot<N, K> {'
    echo '  type Array<T: Copy> = [T; K];'
    echo '  const AGREES: bool = { assert!(N::VAL == K); true };'
    echo '  fn filled<T: Copy>(v: T) -> [T; K] { const { assert!(<Self as Capacity>::AGREES) }; [v; K] } }'
    echo 'pub struct Scalar; pub struct Axis<Hd, Tl>(PhantomData<(Hd, Tl)>);'
    echo 'pub trait Shape { const RANK: usize; const COUNT: usize; }'
    echo 'impl Shape for Scalar { const RANK: usize = 0; const COUNT: usize = 1; }'
    echo 'impl<Hd: Capacity, Tl: Shape> Shape for Axis<Hd, Tl> {'
    echo '  const RANK: usize = 1 + Tl::RANK;'
    echo '  const COUNT: usize = { assert!(Hd::AGREES); <Hd as Nat>::VAL * Tl::COUNT }; }'
    echo 'pub trait Dense: Shape { type Store<E: Copy>: Copy;'
    echo '  fn build<E: Copy>(v: E) -> Self::Store<E>;'
    echo '  fn fold<E: Copy>(s: &Self::Store<E>, a: &mut usize, f: fn(&mut usize, E)); }'
    echo 'impl Dense for Scalar { type Store<E: Copy> = E;'
    echo '  fn build<E: Copy>(v: E) -> E { v }'
    echo '  fn fold<E: Copy>(s: &E, a: &mut usize, f: fn(&mut usize, E)) { f(a, *s) } }'
    echo 'impl<Hd: Capacity, Tl: Dense> Dense for Axis<Hd, Tl> {'
    echo '  type Store<E: Copy> = <Hd as Capacity>::Array<Tl::Store<E>>;'
    echo '  fn build<E: Copy>(v: E) -> Self::Store<E> { Hd::filled(Tl::build(v)) }'
    echo '  fn fold<E: Copy>(s: &Self::Store<E>, a: &mut usize, f: fn(&mut usize, E)) {'
    echo '    for i in s.as_ref() { Tl::fold(i, a, f) } } }'
    # extent 2 per axis, so COUNT = 2^rank and the storage stays buildable
    echo -n 'pub type S = '
    for ((k = 0; k < n; k++)); do echo -n 'Axis<Slot<Pz<O<H>>, 2>, '; done
    echo -n 'Scalar'
    for ((k = 0; k < n; k++)); do echo -n '>'; done
    echo ';'
    echo 'pub const RANK: usize = <S as Shape>::RANK;'
    echo 'pub const COUNT: usize = <S as Shape>::COUNT;'
    echo '#[inline(never)] pub fn total(v: u32) -> usize {'
    echo '  let s = <S as Dense>::build(v); let mut a = 0usize;'
    echo '  <S as Dense>::fold(&s, &mut a, |x, e| *x += e as usize); a }'
  } > "$OUT/r$n.rs"
}

echo "CLAIM A. Compile time and metadata size against rank. Trait-solver work"
echo "is what this measures; --emit=metadata skips codegen. Best of three."
printf "%6s %8s %10s %12s %12s\n" rank COUNT "ms (min)" "meta bytes" "solver depth"
for n in 1 2 4 6 8 10 12 14 16; do
  gen "$n"
  best=999999
  for _ in 1 2 3; do
    s=$(python3 -c 'import time;print(int(time.time()*1000))')
    rustc --edition 2024 --crate-type lib --emit=metadata \
      -o "$OUT/r$n.rmeta" "$OUT/r$n.rs" 2>"$OUT/err$n" || { cat "$OUT/err$n"; exit 1; }
    e=$(python3 -c 'import time;print(int(time.time()*1000))')
    d=$((e - s)); [ "$d" -lt "$best" ] && best=$d
  done
  sz=$(wc -c < "$OUT/r$n.rmeta" | tr -d ' ')
  cnt=$((1 << n))
  printf "%6s %8s %10s %12s %12s\n" "$n" "$cnt" "$best" "$sz" "$n"
done

echo
echo "CLAIM B. What the rank-generic fold EMITS. The body is one function for"
echo "every rank; the question is whether monomorphisation folds it away."
for n in 3 8; do
  gen "$n"
  rustc --edition 2024 --crate-type lib -O --emit=asm \
    -o "$OUT/a$n.s" "$OUT/r$n.rs" 2>/dev/null
  # the mangled `total` body, instruction lines only
  lines=$(awk '/total/{f=1} f&&/^\t/{c++} f&&/^\s*\.cfi_endproc/{print c; exit}' "$OUT/a$n.s")
  echo "  rank $n (COUNT $((1 << n))): total() emits ${lines:-?} instruction lines"
  awk '/total/{f=1} f{print} f&&/ret/{exit}' "$OUT/a$n.s" | head -12 | sed 's/^/      /'
  echo
done
