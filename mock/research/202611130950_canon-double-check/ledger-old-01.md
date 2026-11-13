## mock/DESIGN.md.tmpl   (root document)

- purpose: "Numeric primitives + analysis algorithms with Hot / Warm / Cold / Precise strategy markers. `#![no_std]`, no alloc. Zero dependency on platform. Designed so every downstream consumer shares one numeric layer." (mock/DESIGN.md.tmpl:3-6)
- layers or crate list as stated: "arvo splits into a layered set of small crates with a downstream facade. Dependency direction runs strictly upward; the facade sits at the bottom." (mock/DESIGN.md.tmpl:28-30)
  - L0a `arvo-transparent`, L0a `arvo-strategy`, L0 `arvo-storage`, L0.5 `arvo-bits-contracts`, L0.5 `arvo-mask-contracts`, L0.5 `arvo-numeric-contracts`, L1 `arvo-bits`, L1 `arvo-bitmask`, L1 `arvo-refit`, L1 `arvo-tensor`, L2 `arvo-hash`, L2 `arvo-graph`, L2 `arvo-sparse`, L2 `arvo-comb`, L3 `arvo-spectral`, facade `arvo` (mock/DESIGN.md.tmpl:32-49)
- stated behaviour:
  - "1. **Strategy-parameterised primitives.** Numeric containers declared at type level as `Hot` / `Warm` / `Cold` / `Precise`, driving storage width and codegen choices without runtime dispatch." (mock/DESIGN.md.tmpl:10-12)
  - "2. **no_std, no alloc.** Every crate is const-sized. Consumers pre-allocate; arvo never calls into an allocator." (mock/DESIGN.md.tmpl:13-14)
  - "4. **Monomorphisation is the dispatch.** No `dyn`, no `TypeId`. The monomorphised function IS the type identity." (mock/DESIGN.md.tmpl:19-20)
  - "5. **Const-trait-driven.** Every trait whose methods are sensibly callable in const context is `pub const trait` (RFC 3762 line)." (mock/DESIGN.md.tmpl:21-22)
  - "`UFixed<I, F, S>` and `IFixed<I, F, S>`: `repr(transparent)` newtypes over `Bits<{I+F}, S, Unsigned>` and `Bits<{1+I+F}, S, Signed>`." (mock/DESIGN.md.tmpl:75-77)

## mock/PRINCIPLES.md.tmpl   (root document)

- purpose: "`arvo` is the **downstream facade** of the foundations, not the L0 layer." (mock/PRINCIPLES.md.tmpl:5-6)
- layers or crate list as stated: "Trait declarations split into five contract crates: `arvo-transparent`, `arvo-strategy`, `arvo-bits-contracts` (hosting bit-access plus the narrow and widen surfaces), `arvo-mask-contracts`, `arvo-numeric- contracts`. Concrete impls live in domain-organised implementation crates (`arvo-bits`, `arvo-bitmask`, `arvo-refit`, `arvo-tensor`)." (mock/PRINCIPLES.md.tmpl:8-12); "Algorithm crates (`arvo-graph`, `arvo-sparse`, `arvo-comb`, `arvo-spectral`, `arvo-hash`) sit at L2 / L3." (mock/PRINCIPLES.md.tmpl:16-17)
- stated behaviour:
  - "The nightly feature flag set: `feature(adt_const_params)`, `feature(generic_const_exprs)`, `feature(const_trait_impl)`, `feature(const_index)`, `feature(const_param_ty_trait)`, `feature(macro_metavar_expr_concat)`, `feature(try_trait_v2)`." (mock/PRINCIPLES.md.tmpl:32-36)
  - "Strategy markers (`Hot`, `Warm`, `Cold`, `Precise`) are type parameters, not runtime config." (mock/PRINCIPLES.md.tmpl:271-272)
  - "A `Mass<Hot>` is explicitly lossy-but-fast; a `Mass<Precise>` is explicitly exact-but-expensive." (mock/PRINCIPLES.md.tmpl:273-274)
  - "Default strategy is `Warm` unless called out." (mock/PRINCIPLES.md.tmpl:294)
  - "Every crate is `no_std` with no allocation. All collections are const-sized." (mock/PRINCIPLES.md.tmpl:283-284)
  - "All arvo types are `repr(transparent)` over their chosen container." (mock/PRINCIPLES.md.tmpl:277)

## arvo-bitmask   (files read: 10 non-test; 9 test files counted by grep only)

- purpose: "Fixed-width bitmask types, bit-matrix adjacency, and set operations. Foundation for arvo-graph, arvo-sparse, arvo-comb, and hilavitkutin scheduling. `#![no_std]`, no alloc. Uses arvo numeric types internally." (README.md.tmpl:3-5)
- depends on: arvo, arvo-bits, arvo-bits-contracts, arvo-mask-contracts, arvo-tensor (all `.workspace = true`); `[features] default = []` (no other features) (Cargo.toml:7-15)
- attributes: `#![no_std]`, `#![feature(const_trait_impl)]`, `#![feature(const_ops)]`, `#![allow(incomplete_features)]` (src/lib.rs:19-22)

### Public items
src/lib.rs:
- `pub mod dirty;` `pub mod mask;` `pub mod matrix;` `pub mod node;` `pub mod ops;` (src/lib.rs:30-34) : no doc
- `pub use arvo_bits_contracts::{BitAccess, BitLogic, BitSequence};` (src/lib.rs:36) : no doc
- `pub use dirty::propagate_dirty;` `pub use mask::Mask;` `pub use matrix::{BitMatrix, cap_size};` `pub use node::NodeId;` `pub use ops::SetBitsIter;` (src/lib.rs:37-41) : no doc

src/dirty.rs:
- `pub fn propagate_dirty<W, C: Capacity>(matrix: &BitMatrix<W, C>, dirty: &mut Mask<W>) where W: BitSequence + BitAccess + BitLogic + Copy + Default + PartialEq` (src/dirty.rs:29) : "Propagate dirty bits through a `BitMatrix<W, C>` adjacency matrix."

src/mask.rs:
- `pub struct Mask<W> where W: BitSequence + BitAccess + Copy + Default { pub word: W }` (src/mask.rs:33) : "Generic fixed-width bitmask."
- `pub const fn from_word(word: W) -> Self` (src/mask.rs:47) : "Construct a mask from a raw word."
- `pub const fn to_word(self) -> W` (src/mask.rs:53) : "Extract the raw word."
- `pub const fn width() -> USize` (src/mask.rs:59) : "Logical bit width of the mask (from `W::WIDTH`)."
- `pub fn empty() -> Self` (impl bound `W: [...] + Identity<Additive>`) (src/mask.rs:71) : "Empty mask (all bits cleared). Routes through"
- `pub fn full() -> Self` (impl bound `W: [...] + Bounded`) (src/mask.rs:83) : "Full mask (all bits set). Routes through"
- (trait impls, not `pub` items: `Default for Mask<W>`, `const impl<W> Bounded for Mask<W>`, `const impl<W, Op> Identity<Op> for Mask<W>`)

src/matrix.rs:
- `pub use arvo_tensor::cap_size;` (src/matrix.rs:16) : no doc
- `pub struct BitMatrix<W, C: Capacity> where W: BitSequence + BitAccess + Copy + Default { pub rows: C::Array<Mask<W>> }` (src/matrix.rs:29) : "Generic adjacency matrix chassis."
- `pub fn empty() -> Self` (src/matrix.rs:43) : "Empty matrix (no edges)."
- `pub fn edge(&self, i: NodeId, j: NodeId) -> Bool` (src/matrix.rs:51) : "`Bool::TRUE` when edge `i -> j` is set."
- `pub fn set_edge(&mut self, i: NodeId, j: NodeId)` (src/matrix.rs:62) : "Set edge `i -> j`. Leaves self unchanged when `i` or `j` is"
- `pub fn clear_edge(&mut self, i: NodeId, j: NodeId)` (src/matrix.rs:73) : "Clear edge `i -> j`. Leaves self unchanged when `i` or `j` is"
- `pub fn successors(&self, i: NodeId) -> Mask<W>` (src/matrix.rs:83) : "Successor mask of node `i` (all outgoing edges)."
- `pub fn predecessors(&self, j: NodeId) -> Mask<W>` (src/matrix.rs:96) : "Predecessor mask of node `j` (all incoming edges)."
- `pub fn transitive_closure(&mut self)` (src/matrix.rs:112) : "Transitive closure via Warshall's algorithm."
- (trait impls: `Copy`, `Clone`, `PartialEq`, `Eq`, `Default` for `BitMatrix<W, C>`)

src/node.rs:
- `pub struct NodeId(pub USize);` (src/node.rs:21) : "Node index newtype."
- `pub const fn new(u: USize) -> Self` (src/node.rs:26) : "Construct a `NodeId` from a `USize`."
- (trait impls: `Ord`, `PartialOrd`, `Hash`)

src/ops.rs:
- `pub fn union(self, other: Self) -> Self` (src/ops.rs:33) : "Union (bitwise OR)."
- `pub fn intersection(self, other: Self) -> Self` (src/ops.rs:39) : "Intersection (bitwise AND)."
- `pub fn difference(self, other: Self) -> Self` (src/ops.rs:45) : "Difference (`self & !other`)."
- `pub fn complement(self) -> Self` (src/ops.rs:52) : "Complement (bitwise NOT)."
- `pub fn is_empty(self) -> Bool` (src/ops.rs:58) : "`Bool::TRUE` when every bit is zero."
- `pub fn intersects(self, other: Self) -> Bool` (src/ops.rs:64) : "`Bool::TRUE` when `self` and `other` share any bit."
- `pub fn contains(self, pos: USize) -> Bool` (src/ops.rs:71) : "`Bool::TRUE` when bit at `pos` is set."
- `pub fn insert(&mut self, pos: USize)` (src/ops.rs:77) : "Set bit at `pos`."
- `pub fn remove(&mut self, pos: USize)` (src/ops.rs:83) : "Clear bit at `pos`."
- `pub fn count(self) -> USize` (src/ops.rs:89) : "Popcount."
- `pub fn lowest_set(self) -> USize` (src/ops.rs:96) : "Lowest set bit index. Returns `W::WIDTH` if the mask is empty"
- `pub fn highest_set(self) -> USize` (src/ops.rs:102) : "Highest set bit index. Returns `W::WIDTH` if the mask is empty."
- `pub fn iter_set_bits(self) -> SetBitsIter<W>` (src/ops.rs:113) : "Iterator over set bit indices, lowest-first."
  (the eleven methods above sit in `impl<W> Mask<W> where W: BitSequence + BitAccess + BitLogic + Copy + Default`)
- `pub struct SetBitsIter<W> where W: BitSequence + BitAccess + BitLogic + Copy + Default` (src/ops.rs:125) : "Generic iterator over set bits of a `Mask<W>`."
- (trait impls: `Iterator for SetBitsIter<W>` with `type Item = USize`; `const impl BitAnd, BitOr, BitXor, Not for Mask<W>`; `const impl<W> MaskOps for Mask<W>` with `empty, full, set, clear, test, count, union, intersection, difference, complement`)

### Stated behaviour
- "Bitmasks are not numeric types. They do bitwise operations (OR, AND, popcount), not arithmetic (Add, Mul, Sqrt). The type families are different." (DESIGN.md.tmpl:12-14)
- "`W` is a bit-bearing word that satisfies `BitSequence + BitAccess + Copy + Default`. The bitmask width follows from `<W as HasBitWidth>::WIDTH`." (DESIGN.md.tmpl:38-40)
- "Single-instruction on the backing primitive: `bitor` / `bitand` / `bitxor` / `bitnot` on `Bits<64, Hot, Unsigned>` map to `or` / `and` / `xor` / `not` on x86 (identical on aarch64)." (DESIGN.md.tmpl:116-118)
- "Warm / Cold / Precise project to `WideBits<32>` at 1-byte alignment" (DESIGN.md.tmpl:75-76)
- "Termination: `dirty` only grows, and it is bounded by the mask's width." (src/dirty.rs:7-8)
- "Leaves self unchanged when `i` or `j` is out of range." (src/matrix.rs:59-60)
- "Lowest set bit index. Returns `W::WIDTH` if the mask is empty (matches `trailing_zeros` semantics on a zero word)." (src/ops.rs:93-94)
- "Two sites retain compiler-mandated primitive types, annotated with `lint:allow(...) reason:...; tracked: #N`" (DESIGN.md.tmpl:124-125)
- "One blanket impl covers every concrete `Mask<W>` instantiation." (DESIGN.md.tmpl:186)
- "the blanket is the arvo's correct-by-construction baseline; concrete W specialisations land later only when bench evidence shows a win" (DESIGN.md.tmpl:190-192)
- Strategy markers: "`Bits<64, Hot, Unsigned>`" canonical single-word W; "`AlignedWideBits16<32>` via `BitsContainerFor<256, Unsigned>` (32 bytes, 16-byte aligned for the Hot SIMD baseline" (DESIGN.md.tmpl:73-75)
- External standards named: RFC 3762 not named in this crate; none named.
- Consumer bound: "The `W` bound, `BitSequence + BitAccess + BitLogic + Copy + Default`, names three traits declared in arvo-bits-contracts (L0.5)." (DESIGN.md.tmpl:246-247)

### Backlog and known gaps
BACKLOG.md.tmpl (no SHAME.md.tmpl):
- `## Wider-than-256 mask widths` : "Mask4096 / Mask1024 / Mask128 ergonomic instantiations are deferred post-1.0 if a consumer surfaces the need." (BACKLOG.md.tmpl:10-11)
- `## MaskOps Width-typed const-generic on consumers` : "When a consumer binds generically via the abstract trait, the call-site spelling takes a Width const value rather than a u16 literal." (BACKLOG.md.tmpl:20-22)
- `## Concrete MaskOps W specialisations` : "This ships only the blanket `impl<W: [const] BitPrim> const MaskOps<...> for Mask<W>`." (BACKLOG.md.tmpl:27-28)
- `## BitMatrix iter contract in arvo-bits-contracts` : "A BitMatrix-shaped contract in arvo-bits-contracts (paralleling BitSequence for flat bitmasks) would let row-access algorithms live in the contract layer and be reused without depending on the concrete arvo-bitmask chassis." (BACKLOG.md.tmpl:38-41)

### Tests
Test files: tests/bitmatrix.rs (11), tests/capacity_threading.rs (1), tests/dirty.rs (7), tests/mask256.rs (13), tests/mask_const_arith.rs (1), tests/mask_ops.rs (10), tests/mask_ops_blanket_const.rs (1), tests/mask_scan.rs (9), tests/out_of_range.rs (5). src files: 0 `#[test]`. (counts by grep of `#[test]`; test bodies not read)

## arvo-bits-contracts   (files read: 13 non-test; tests/bitprim_const_access.rs counted by grep only)

- purpose: "Bit-level trait contracts for arvo. Declarations of `HasBitWidth`, `BitAccess`, `BitSequence`, `BitLogic`, `BitPrim`, `UBitContainer`, `IBitContainer`. All `pub const trait`." (README.md.tmpl:3-5); DESIGN: "The bit-level trait declarations. Every trait here is `pub const trait`." (DESIGN.md.tmpl:3)
- depends on: notko, arvo-transparent, arvo-strategy, arvo-storage (all `.workspace = true`); `[features] default = []` (Cargo.toml:7-14)
- attributes: `#![no_std]`, `#![feature(adt_const_params)]`, `#![feature(const_trait_impl)]`, `#![feature(const_ops)]`, `#![allow(incomplete_features)]` (src/lib.rs:1-5)

### Public items
src/lib.rs:
- `pub use narrow_from::NarrowFromU64;` `pub use widen::{Widen, Widened};` (src/lib.rs:35-36) : no doc
- (private modules: `bits_impl`, `cross_domain`, `narrow_from`, `prim_impls`, `widebits_impl`, `widen`; `mod sealed` holds `pub(crate) trait Bit`, `IBit`, `UBridge<const BITS: u16>`, `IBridge<const BITS: u16>`) (src/lib.rs:28-54)
- `pub const trait HasBitWidth` (src/lib.rs:62) : "Logical bit width at the type level."
  - `const WIDTH: USize;` (src/lib.rs:64) : "Logical bit width."
- `pub const trait BitAccess: HasBitWidth + Copy` (src/lib.rs:73) : "Individual bit read / write."
  - `fn bit(self, idx: USize) -> Bool;` (:75) : "Read bit at position `idx`."
  - `fn with_bit_set(self, idx: USize) -> Self;` (:77) : "Produce a copy with bit `idx` set to 1."
  - `fn with_bit_cleared(self, idx: USize) -> Self;` (:79) : "Produce a copy with bit `idx` cleared to 0."
  - `fn with_bit_toggled(self, idx: USize) -> Self;` (:81) : "Produce a copy with bit `idx` flipped."
- `pub const trait BitSequence: HasBitWidth + Copy` (src/lib.rs:90) : "Bulk bit-scanning / popcount."
  - `fn trailing_zeros(self) -> USize;` (:92) : "Count trailing (LSB) zero bits."
  - `fn leading_zeros(self) -> USize;` (:94) : "Count leading (MSB) zero bits."
  - `fn count_ones(self) -> USize;` (:96) : "Count set bits."
  - `fn count_zeros(self) -> USize;` (:98) : "Count cleared bits."
  - `fn is_zero(self) -> Bool;` (:100) : "`Bool::TRUE` when every bit is zero."
- `pub const trait BitLogic: HasBitWidth + Copy` (src/lib.rs:115) : "Whole-word bitwise logic."
  - `fn bitor(self, other: Self) -> Self;` (:117) : "Whole-word OR."
  - `fn bitand(self, other: Self) -> Self;` (:119) : "Whole-word AND."
  - `fn bitnot(self) -> Self;` (:121) : "Whole-word NOT."
  - `fn bitxor(self, other: Self) -> Self;` (:123) : "Whole-word XOR."
  - `fn clear_lowest_set_bit(self) -> Self where Self: [const] BitAccess + [const] BitSequence` (default body) (:133) : "Clear the lowest set bit."
- `pub const trait BitPrim: sealed::Bit + Copy + 'static` (src/lib.rs:156) : "Sealed unsigned primitive bit bridge."
  - `const WIDTH: USize;` (:158) : "Bit width of this primitive (8, 16, 32, 64, or 128)."
  - `const ZERO: Self;` (:160) : "Zero value."
  - `const ONE: Self;` (:162) : "One value."
  - `fn count_ones(self) -> USize;` (:165) : "Count set bits."
  - `fn trailing_zeros(self) -> USize;` (:167) : "Count trailing zero bits (LSB-first)."
  - `fn leading_zeros(self) -> USize;` (:169) : "Count leading zero bits (MSB-first)."
  - `fn get_bit(self, idx: USize) -> Bool;` (:172) : "Read bit `idx`. Returns `Bool::FALSE` for `idx >= WIDTH`."
  - `fn with_bit_set(self, idx: USize) -> Self;` (:174) : "Set bit `idx`. Leaves self unchanged for `idx >= WIDTH`."
  - `fn with_bit_cleared(self, idx: USize) -> Self;` (:176) : "Clear bit `idx`. Leaves self unchanged for `idx >= WIDTH`."
  - `fn with_bit_toggled(self, idx: USize) -> Self;` (:178) : "Toggle bit `idx`. Leaves self unchanged for `idx >= WIDTH`."
  - `fn bitor(self, other: Self) -> Self;` (:181) : "Whole-word OR."
  - `fn bitand(self, other: Self) -> Self;` (:183) : "Whole-word AND."
  - `fn bitnot(self) -> Self;` (:185) : "Whole-word NOT."
  - `fn bitxor(self, other: Self) -> Self;` (:187) : "Whole-word XOR."
  - `fn clear_lowest_set_bit(self) -> Self;` (:189) : "Clear the lowest set bit. `self & (self.wrapping_sub(1))`."
  - `fn is_zero(self) -> Bool;` (:198) : "`Bool::TRUE` when every bit of the primitive is zero."
  - `fn mask_low(n: USize) -> Self;` (:210) : "Mask with the lowest `n` bits set, all higher bits cleared."
- `pub const trait IBitPrim: sealed::IBit + Copy + 'static` (src/lib.rs:222) : "Sealed signed primitive bit bridge."
  - items identical in signature and doc to `BitPrim` (`WIDTH`, `ZERO`, `ONE`, `count_ones`, `trailing_zeros`, `leading_zeros`, `get_bit`, `with_bit_set`, `with_bit_cleared`, `with_bit_toggled`, `bitor`, `bitand`, `bitnot`, `bitxor`, `clear_lowest_set_bit`, `is_zero`, `mask_low`) at src/lib.rs:224-264; `mask_low` doc first line: "Mask with the lowest `n` bits set, all higher bits cleared."
- `pub const trait BitsBitPrim<Sign: Signedness>: Copy + 'static` (src/lib.rs:287) : "Sign-keyed primitive bridge."
  - `const WIDTH: USize;` `const ZERO: Self;` `const ONE: Self;` `fn count_ones(self) -> USize;` `fn trailing_zeros(self) -> USize;` `fn leading_zeros(self) -> USize;` `fn is_zero(self) -> Bool;` `fn get_bit(self, idx: USize) -> Bool;` `fn with_bit_set(self, idx: USize) -> Self;` `fn with_bit_cleared(self, idx: USize) -> Self;` `fn with_bit_toggled(self, idx: USize) -> Self;` `fn bitor(self, other: Self) -> Self;` `fn bitand(self, other: Self) -> Self;` `fn bitnot(self) -> Self;` `fn bitxor(self, other: Self) -> Self;` `fn clear_lowest_set_bit(self) -> Self;` (src/lib.rs:289-322)
  - impls: `const impl<T: [const] BitPrim> BitsBitPrim<Unsigned> for T` (:325); `const impl<T: [const] IBitPrim> BitsBitPrim<Signed> for T` (:384)
- `pub const trait UBitContainer<const BITS: u16>: sealed::UBridge<BITS> + [const] BitsContainerFor<BITS, Unsigned>` (src/lib.rs:454) : "Sealed bridge: `(S, BITS)` where `S: BitsContainerFor<BITS, Unsigned>` **and**"
  - `type Prim: [const] BitPrim;` (:458) : "The container primitive for this `(S, BITS)` pair."
  - `fn to_prim(t: <Self as BitsContainerFor<BITS, Unsigned>>::T) -> Self::Prim;` (:463) : "Coerce the strategy-selected container into the bridge's"
  - `fn from_prim(p: Self::Prim) -> <Self as BitsContainerFor<BITS, Unsigned>>::T;` (:465) : "Reverse of `to_prim`."
- `pub const trait IBitContainer<const BITS: u16>: sealed::IBridge<BITS> + [const] BitsContainerFor<BITS, Signed>` (src/lib.rs:469) : "Signed counterpart of `UBitContainer`."
  - `type Prim: [const] IBitPrim;` (:473) : "The container primitive for this `(S, BITS)` pair."
  - `fn to_prim(t: <Self as BitsContainerFor<BITS, Signed>>::T) -> Self::Prim;` (:475) : "See `UBitContainer::to_prim`."
  - `fn from_prim(p: Self::Prim) -> <Self as BitsContainerFor<BITS, Signed>>::T;` (:477) : "Reverse of `to_prim`."
  - blanket impls: `impl<S, const BITS: u16> sealed::UBridge<BITS> for S` (:489), `const impl<S, const BITS: u16> UBitContainer<BITS> for S` (:497), `impl ... sealed::IBridge<BITS> for S` (:521), `const impl ... IBitContainer<BITS> for S` (:529)
- `pub type Narrowed<const N: u16, T> = T;` (src/lib.rs:568) : "Narrowed-result expression alias."
- `pub const trait Narrow<T>` (src/lib.rs:584) : "Narrow `Self` to the lowest `N` bits as type `T`."
  - `fn narrow_to<const N: u16>(self) -> T where Self: Sized;` (:586) : "Truncate to the lowest `N` bits and return as `T`."
  - `fn narrow_to_unmasked<const N: u16>(self) -> T where Self: Sized;` (:598) : "Cast to `T` without masking the high bits."
  - impls by macro `impl_narrow_u!`: `u8 => u8`; `u16 => u8, u16`; `u32 => u8, u16, u32`; `u64 => u8, u16, u32, u64`; `u128 => u8, u16, u32, u64, u128` (:643-647); by `impl_narrow_i!`: `i8 => i8`; `i16 => i8, i16`; `i32 => i8, i16, i32`; `i64 => i8, i16, i32, i64`; `i128 => i8, i16, i32, i64, i128` (:678-682); `const impl<const M: u16, const N: u16, S: Strategy, Sign: Signedness> Narrow<Bits<N, S, Sign>> for Bits<M, S, Sign>` (:692)

src/widen.rs:
- `pub type Widened<T> = T;` (src/widen.rs:31) : "Widened-result expression alias."
- `pub const trait Widen<T>` (src/widen.rs:49) : "Widen `Self` into a wider bit-width carrier `T`."
  - `fn widen_to(self) -> T where Self: Sized;` (:58) : "Widen `Self` into `T`."
  - `fn widen_to_unmasked(self) -> T where Self: Sized;` (:69) : "Hot-path widen for symmetry with `Narrow::narrow_to_unmasked`."
  - impls by `impl_widen_u!`: `u8 => u16, u32, u64, u128`; `u16 => u32, u64, u128`; `u32 => u64, u128`; `u64 => u128`; by `impl_widen_i!`: `i8 => i16, i32, i64, i128`; `i16 => i32, i64, i128`; `i32 => i64, i128`; `i64 => i128` (:91-94, :111-114); `const impl<const M: u16, const N: u16, S: Strategy, Sign: Signedness> Widen<Bits<N, S, Sign>> for Bits<M, S, Sign>` (:123)

src/narrow_from.rs:
- `pub const trait NarrowFromU64<const N: u16, S: Strategy, Sign: Signedness>: Sized` (src/narrow_from.rs:36) : "Narrow a u64 to `Self` under the `(N, S, Sign)` projection."
  - `fn narrow_u64(raw: u64) -> Self;` (:38) : "Mask the low `N` bits of `raw` and project into `Self`."
  - impls by `impl_narrow_for_prims!`: `(Hot, Unsigned, u8, u16, u32, u64, u128)`, `(Hot, Signed, i8, i16, i32, i64, i128)`, `(Cold, Unsigned, u8..u128)`, `(Cold, Signed, i8..i128)`, `(Warm, Unsigned, u16, u32, u64, u128)`, `(Warm, Signed, i16, i32, i64, i128)`, `(Precise, Unsigned, u16, u32, u64, u128)`, `(Precise, Signed, i16, i32, i64, i128)` (:77-84); `const impl<const N: u16, S: Strategy, Sign: Signedness, const BYTES: usize, A: Align> NarrowFromU64<N, S, Sign> for WideBits<BYTES, A>` (:98)

src/bits_impl.rs (no `pub` items; trait impls on `Bits<N, S, Sign>` generic over `Sign: Signedness`): `HasBitWidth`, `BitAccess`, `BitSequence`, `BitLogic` (:20, :27, :61, :98)

src/cross_domain.rs (no `pub` items): macros `impl_narrow_pri_to_bits_unsigned!(u8, u16, u32, u64, u128)` (:42), `impl_narrow_pri_to_bits_signed!(i8, i16, i32, i64, i128)` (:68), `impl_widen_bits_to_pri_unsigned!(u8, u16, u32, u64, u128)` (:96), `impl_widen_bits_to_pri_signed!(i8, i16, i32, i64, i128)` (:122)

src/prim_impls.rs (no `pub` items): `impl_bit_prim_u!` for `u8` 8, `u16` 16, `u32` 32, `u64` 64, `u128` 128 (:124-131); `impl_bit_prim_i!` for `i8`/`u8` 8, `i16`/`u16` 16, `i32`/`u32` 32, `i64`/`u64` 64, `i128`/`u128` 128 (:246-252)

src/widebits_impl.rs (no `pub` items): `impl<const BYTES: usize, A: Align + 'static> sealed::Bit for WideBits<BYTES, A> {}` (:35); `const impl<const BYTES: usize, A: Align + 'static> BitPrim for WideBits<BYTES, A>` (:38)

### Stated behaviour
- "All mutators take `self` and return `Self` (functional style, no interior mutation). `idx` is LSB-first (bit 0 is least significant). Indices `>= WIDTH` do not panic: `bit` returns `Bool::FALSE` and the three `with_bit_*` mutators return `self` unchanged." (src/lib.rs:69-72)
- "Hot-only surface. Other strategies (`Warm`, `Precise`, `Cold`) have wider containers that make whole-word ops meaningless at the logical-width level (NOT would flip surplus container bits)." (src/lib.rs:112-114)
- "Saturates at `WIDTH`: if `n >= WIDTH`, returns the all-ones value. If `n == 0`, returns zero. Otherwise returns `(ONE << n) - ONE`." (src/lib.rs:203-204)
- "Operates on the raw container bits; the logical-width contract is implicit in the types' construction." (src/lib.rs:88-89)
- "Bit operations reinterpret the bits through the corresponding unsigned type so signed-shift semantics do not leak in." (src/lib.rs:216-217)
- "Sound only when the caller knows the high bits above `N` are already zero [...] Calling with non-zero high bits produces silent garbage in the unmasked path." (src/lib.rs:591-597)
- "The precondition is the consumer's responsibility; calling with non-zero high bits produces silent garbage in the unmasked path." (DESIGN.md.tmpl:276-278)
- "For sub-carrier-width `Bits<M, S, Sign>` the contract is **bit-pattern-preserving**, not logical-value-preserving, on both `Narrow` and `Widen`." (DESIGN.md.tmpl:333-335)
- "Sign-extends at the carrier's high bit for signed primitive sources. Zero-extends for unsigned primitive sources." (src/widen.rs:52-53)
- "Where M < N is not enforced at the type level (parallel to Narrow's M > N non-enforcement)" (DESIGN.md.tmpl:405-406)
- "bit 0 is the LSB of `bytes[0]`. `trailing_zeros` walks from byte 0 upward; `leading_zeros` walks from byte BYTES-1 downward." (src/widebits_impl.rs:20-21)
- "The container may be wider than the logical width N; the raw `leading_zeros` includes the container's surplus bits above N, which the contract subtracts back out." (src/bits_impl.rs:70-72)
- "The Sign axis on `Bits<N, S, Sign>` is invisible at the trait surface; bit-level operations act on raw bit patterns which are sign-agnostic by construction." (DESIGN.md.tmpl:9-11)
- Strategy markers: `Hot, Cold` Unsigned natives `u8, u16, u32, u64, u128`; "`Warm, Precise` ... Unsigned | u16, u32, u64, u128 (no u8; 2x-logical ladder)" (DESIGN.md.tmpl:426-428); "Hot-only surface" for `BitLogic` (src/lib.rs:112); `AlignedWideBits16<const BYTES: usize>` "for the SIMD-aligned shape at N > 128 (Hot)" (DESIGN.md.tmpl:171-173)
- Named standards: "matches `u128::trailing_zeros` / `u128::leading_zeros` semantics viewed as bit-significance order" (DESIGN.md.tmpl:231-232); "RFC 3762" not named here.
- Consumer bound: "consumer call sites pull this crate to bind to the trait surface, then reach the concrete impls in `arvo-bits` via blanket dispatch." (DESIGN.md.tmpl:5-7)

### Backlog and known gaps
BACKLOG.md.tmpl:
- `## Status, #316 redesign` : "The const-generic position lift to Width (Round 2 / #312) landed for Bits in arvo-storage." (BACKLOG.md.tmpl:5-6)
- `## Removed entries` : "- "BitPrim impl on MultiContainer", MultiContainer is deleted entirely in #316 per no-legacy-shims-pre-1.0; the impl moves to WideBits + AlignedWideBits16." (BACKLOG.md.tmpl:25-27)

SHAME.md.tmpl (title `# arvo-bits-contracts. Known gaps`):
- `## IBit` : "Private sealing trait gating `IBitPrim`." (SHAME.md.tmpl:12)
- `## UBridge` : "Private sealing trait gating the `UBitContainer` blanket impl." (SHAME.md.tmpl:24)
- `## IBridge` : "Private sealing trait gating the `IBitContainer` blanket impl." (SHAME.md.tmpl:35)

### Tests
tests/bitprim_const_access.rs (1). src files: 0 `#[test]`.

## arvo-bits   (files read: 6 non-test; 8 test files counted by grep only)

- purpose: "Concrete trait impls for the bit-level contracts." (README.md.tmpl:3); DESIGN: "Bit-level **implementation** crate." (DESIGN.md.tmpl:3); lib.rs: "This crate is the domain-alias surface, nothing else." (src/lib.rs:17)
- depends on: arvo-storage, arvo-strategy, arvo-bits-contracts; dev-dependency arvo (all `.workspace = true`); `[features] default = []` (Cargo.toml:7-16)
- attributes: `#![no_std]`, `#![feature(adt_const_params)]`, `#![feature(const_trait_impl)]`, `#![allow(incomplete_features)]` (src/lib.rs:1-4)

### Public items
src/lib.rs:
- `pub use arvo_bits_contracts::{BitAccess, BitLogic, BitPrim, BitSequence, HasBitWidth, IBitContainer, IBitPrim, Narrow, Narrowed, UBitContainer, Widen, Widened};` (src/lib.rs:22-25) : no doc
- `pub use arvo_storage::Bits;` (src/lib.rs:26) : no doc
- `pub use arvo_strategy::{Hot, Strategy};` (src/lib.rs:27) : no doc
- `pub use refit_constructors::BitsRefitCtor;` (src/lib.rs:87) : no doc
- (in private `mod refit_constructors`) `pub const trait BitsRefitCtor<const N: u16, S: Strategy, Sign: Signedness> where S: BitsContainerFor<N, Sign>` (src/lib.rs:41) : "Sealed extension trait providing `from_narrowed` and"
  - `fn from_narrowed<Src>(src: Src) -> Bits<N, S, Sign> where Src: [const] Narrow<<S as BitsContainerFor<N, Sign>>::T>;` (:49) : "Construct from a wider-or-equal-bit-width source via `Narrow`."
  - `fn from_widened<Src>(src: Src) -> Bits<N, S, Sign> where Src: [const] Widen<<S as BitsContainerFor<N, Sign>>::T>;` (:59) : "Construct from a narrower source via `Widen`."
  - impl: `const impl<const N: u16, S: Strategy, Sign: Signedness> BitsRefitCtor<N, S, Sign> for Bits<N, S, Sign>` (:64)
- `pub type Bit<S = Hot> = Bits<1, S>;` (src/lib.rs:93) : "1-bit opaque bit-pattern."
- `pub type Nibble<S = Hot> = Bits<4, S>;` (:96) : "4-bit opaque bit-pattern (half-byte)."
- `pub type Byte<S = Hot> = Bits<8, S>;` (:99) : "8-bit opaque bit-pattern."
- `pub type Word<S = Hot> = Bits<16, S>;` (:102) : "16-bit opaque bit-pattern (x86 "word")."
- `pub type DWord<S = Hot> = Bits<32, S>;` (:105) : "32-bit opaque bit-pattern (x86 "dword")."
- `pub type QWord<S = Hot> = Bits<64, S>;` (:109) : "64-bit opaque bit-pattern (x86 "qword"; arvo's widest logical"

src/traits.rs (the file src/lib.rs does not contain a `mod traits;` line):
- `pub use arvo_bits_contracts::{BitAccess, BitLogic, BitSequence, HasBitWidth};` (src/traits.rs:13) : no doc

### Stated behaviour
- "Default strategy is `Hot`: bit work typically wants the minimum container and wrapping semantics." (src/lib.rs:10-11)
- "The bit-level trait declarations (`HasBitWidth`, `BitAccess`, `BitSequence`, `BitLogic`) live in `arvo-bits-contracts`. The blanket impls on `Bits<N, S>` also live there" (src/lib.rs:13-15)
- "Other strategies (`Warm`, `Precise`, `Cold`) do not impl `BitLogic`: their wider containers make whole-word ops meaningless at the logical-width level. `BitLogic` is a Hot-only surface." (DESIGN.md.tmpl:162-165)
- "The width is the logical bit count, not the container size. Container size depends on the strategy marker and may be larger." (DESIGN.md.tmpl:77-79)
- "All methods take `self` and return `Self` (functional style, no mutation). `idx` is from LSB (bit 0 = least significant)." (DESIGN.md.tmpl:94-95)
- "`Bool` implements `Try` with `Residual = Infallible`:" (DESIGN.md.tmpl:198)
- "The `?` always takes the `Continue` path (Infallible residual is uninhabited). Zero runtime cost." (DESIGN.md.tmpl:223-224)
- "`Bit` is data. `Bool` is control flow. The boundary between them is explicit conversion." (DESIGN.md.tmpl:193-194)
- "Requires `adt_const_params` (nightly)." (DESIGN.md.tmpl:58)
- "`BitLogic` is a Hot-only surface." (DESIGN.md.tmpl:164-165)
- Hardware mapping: "`trailing_zeros` -> `cttz` (x86: `tzcnt`, aarch64: `rbit` + `clz`)" (DESIGN.md.tmpl:114)
- Consumers: "The chassis works uniformly for `W = Bits<64, Hot, Unsigned>` (single-word) and `W = Bits<256, Hot, Unsigned>` (multi-word via WideBits storage)." (DESIGN.md.tmpl:304-306)
- Named external standards or overflow/rounding statements: none found in these files beyond the x86/aarch64 instruction mapping above.

### Backlog and known gaps
BACKLOG.md.tmpl (no SHAME.md.tmpl):
- `## Bits extensions` : "- Wider backing for Bits (u128 storage, supporting N in 65..=128)." (BACKLOG.md.tmpl:10)
- `## bitfield! macro extensions` : "- Overlap detection between sub-ranges." (BACKLOG.md.tmpl:18)

### Tests
tests/aliases.rs (4), tests/bit_access.rs (6), tests/bit_logic.rs (10), tests/bit_sequence.rs (6), tests/bit_width.rs (3), tests/bitfield.rs (8), tests/bits.rs (9), tests/bits_const_arith.rs (1). src files: 0 `#[test]`. (counts by grep; bodies not read)

## arvo-comb   (files read: 9 non-test; 4 test files counted by grep only)

- purpose: "Combinatorial optimisation on const-generic structures. Matrix chain DP, greedy interval grouping, bin-packing. `#![no_std]`, no alloc. All types are arvo numerics." (README.md.tmpl:3-5)
- depends on: arvo, arvo-bitmask, arvo-tensor (`.workspace = true`); `[features] default = []` (Cargo.toml:7-13)
- attributes: `#![no_std]`; no feature gates: "No `generic_const_exprs` / `adt_const_params` gates" (src/lib.rs:15-16)

### Public items
src/lib.rs:
- `pub mod binpack;` `pub mod dp;` `pub mod greedy;` `pub mod range;` (src/lib.rs:23-26) : no doc
- `pub use binpack::bin_pack;` `pub use dp::matrix_chain_dp;` `pub use greedy::greedy_group;` `pub use range::Range;` (src/lib.rs:28-31) : no doc

src/binpack.rs:
- `pub fn bin_pack<N: Capacity, B: Capacity, T, W>(items: &Array<T, N>, capacity: W, weight_of: impl Fn(&T) -> W, affinity: impl Fn(&T, &T) -> W) -> (USize, Array<NUSize, N>) where W: Add<Output = W> + TotalOrd + Copy + FromConstant` (src/binpack.rs:37) : "First-fit bin packing with affinity-based pre-ordering."

src/dp.rs:
- `pub fn matrix_chain_dp<N: Capacity, W>(cost: impl Fn(USize, USize) -> W, feasible: impl Pred2<USize, USize>) -> (W, Array<USize, N>) where W: Add<Output = W> + TotalOrd + Copy + FromConstant, N::Array<W>: Copy, N::Array<Bool>: Copy, N::Array<USize>: Copy` (src/dp.rs:36) : "Minimise the total cost of splitting `[0..N]` into intervals."
- (private) `fn fill_splits<N: Capacity>(...)` (src/dp.rs:148)

src/greedy.rs:
- `pub fn greedy_group<N: Capacity, M: Capacity, A, T>(items: &Array<T, N>, feasible: impl Pred2<A, T>, merge: impl Fn(A, &T) -> A, init: impl Fn() -> A) -> (USize, Array<Range, M>)` (src/greedy.rs:35) : "Walk `items` in order, accumulating into a single group while the"

src/range.rs:
- `pub struct Range { pub start: USize, pub end: USize }` (src/range.rs:15) : "Half-open interval `[start, end)` over `USize` indices."
  - fields: `pub start: USize` (:17) "Inclusive start index."; `pub end: USize` (:19) "Exclusive end index."
  - derives `Copy, Clone, Eq, PartialEq, Debug`; `impl Default for Range` (:22)

### Stated behaviour
- "The numeric bound is `Add + TotalOrd + Copy + FromConstant`; no subtraction." (src/binpack.rs:8-9)
- "Items that do not fit any bin map to `NUSize::NONE`; items that placed map to `NUSize::some(bin_id)`." (src/binpack.rs:13-14)
- "`bin_count` is the number of bins that received at least one item (never greater than `B`)." (src/binpack.rs:35-36)
- "Classic `O(N^3)` interval DP" (src/dp.rs:3)
- "If no feasible leaf exists for some sub-interval, the DP falls back on composing feasible children; if even that fails, the returned cost reflects the best feasible split discovered, and `splits` contains `USize(0)` for unreachable entries." (src/dp.rs:32-35)
- "The returned `(count, groups)` has `count <= M`; only the first `count` entries of `groups` are meaningful. If the algorithm would exceed `M`, it returns early with `count == M` and the tail of the input is dropped from the result." (src/greedy.rs:27-30)
- "If a just-reset accumulator still reports an item as infeasible, that item is skipped to avoid an infinite loop." (src/greedy.rs:32-33)
- "`M` is the caller-supplied upper bound on the number of groups. It must be ≥ ceil(N / min_group_size); the algorithm asserts this at compile time by bounding `M >= 1` and at runtime by returning early if the group array fills." (DESIGN.md.tmpl:74-77)
- "Researched threshold: use DP when problem size exceeds 10 units; fall back to greedy's left-to-right sequential join below that." (DESIGN.md.tmpl:103-104)
- "Generic over numeric trait bounds for cost and weight types. Does not import UFixed or IFixed directly." (DESIGN.md.tmpl:12-13)
- "`N` / `M` / `B` are `Capacity` type parameters (arvo-tensor's `Capacity`)" (DESIGN.md.tmpl:16-17)
- Strategy markers, rounding, overflow, named standards: none stated in these files.

### Backlog and known gaps
BACKLOG.md.tmpl (no SHAME.md.tmpl):
- `## Morton / Hilbert dispatch ordering` : "Space-filling curve ordering for dispatch interleaving." (BACKLOG.md.tmpl:10)
- `## Wider-W bin_pack variant` : "`bin_pack` ships with a `Cap` up to 64-ish items per bin (bounded by internal `Mask<Bits<64, Hot, Unsigned>>` helpers where consumers pair it with hierarchical co-firing flags)." (BACKLOG.md.tmpl:16-18)

### Tests
tests/binpack.rs (5), tests/capacity_threading.rs (2), tests/dp.rs (4), tests/greedy.rs (5). src files: 0 `#[test]`. (counts by grep; bodies not read)

## arvo-graph   (files read: 12 non-test; 9 test files counted by grep only)

- purpose: "DAG and graph algorithms on const-generic bitmask structures. Topological sort, connected components, rank computation, critical path, waist detection. `#![no_std]`, no alloc." (README.md.tmpl:3-5)
- depends on: arvo, arvo-bits-contracts, arvo-bitmask, arvo-tensor (`.workspace = true`); `[features] default = []` (Cargo.toml:7-14)
- attributes: `#![no_std]`, `#![feature(const_trait_impl)]` (src/lib.rs:22-23)

### Public items
src/lib.rs:
- `pub mod components;` `pub mod path;` `pub mod rank;` `pub mod spanning;` `pub mod topo;` `pub mod waist;` `pub mod waist_const;` (src/lib.rs:28-34) : no doc
- `pub use components::components;` `pub use path::longest_path;` `pub use rank::{downward_rank, upward_rank};` `pub use spanning::{SpanningTree, spanning_tree};` `pub use topo::{renumber, topo_sort};` `pub use waist::waist_detect;` `pub use waist_const::waist_detect_const;` (src/lib.rs:36-42) : no doc

src/components.rs:
- `pub fn components<C: Capacity, B>(dag: &BitMatrix<B, C>) -> C::Array<USize> where B: BitSequence + BitAccess + BitLogic + Copy + Default + Identity<Additive>, C::Array<USize>: Copy, C::Array<NodeId>: Copy` (src/components.rs:23) : "Assign a component ID to every node."

src/path.rs:
- `pub fn longest_path<C: Capacity, W, B>(dag: &BitMatrix<B, C>, weights: &C::Array<W>, topo_order: &C::Array<NodeId>) -> (W, Mask<B>, C::Array<NodeId>) where W: Add<Output = W> + TotalOrd + Copy + FromConstant, B: BitSequence + BitAccess + BitLogic + Copy + Default + Identity<Additive>, C::Array<W>: Copy, C::Array<NodeId>: Copy` (src/path.rs:30) : "Longest path in a DAG as a DP over forward topo order."

src/rank.rs:
- `pub fn upward_rank<C: Capacity, W, B>(dag: &BitMatrix<B, C>, weights: &C::Array<W>) -> C::Array<W> where W: Add<Output = W> + TotalOrd + Copy + FromConstant, B: BitSequence + BitAccess + BitLogic + Copy + Default, C::Array<W>: Copy, C::Array<USize>: Copy, C::Array<NodeId>: Copy` (src/rank.rs:34) : "Upward rank: weight plus the max successor rank."
- `pub fn downward_rank<C: Capacity, W, B>(dag: &BitMatrix<B, C>, weights: &C::Array<W>) -> C::Array<W>` (same where-clause as `upward_rank`) (src/rank.rs:95) : "Downward rank: weight plus the max predecessor rank."

src/spanning.rs:
- `pub struct SpanningTree<C: Capacity, B: BitSequence + BitAccess + Copy + Default>` (src/spanning.rs:31) : "Flat spanning-tree decomposition."
  - `pub on_trunk: Mask<B>` (:33) : "Nodes that sit on a trunk (main or branch)."
  - `pub trunk_next: C::Array<NodeId>` (:35) : "Successor of each on-trunk node. Undefined off the trunk."
  - `pub branch_roots: Mask<B>` (:37) : "Starting nodes of branches."
  - `pub bridges: Mask<B>` (:39) : "Fan-in nodes (predecessor count >= 2)."
  - `impl ... Copy`, `impl ... Clone` where `C::Array<NodeId>: Copy` (:42, :47)
- `pub fn empty() -> Self` (impl bound `B: BitSequence + BitAccess + Copy + Default + Identity<Additive>`, `C::Array<NodeId>: Copy`) (src/spanning.rs:64) : "Empty decomposition."
- `pub fn spanning_tree<C: Capacity, W, B>(dag: &BitMatrix<B, C>, ranks: &C::Array<W>) -> SpanningTree<C, B> where W: TotalOrd + Copy, B: BitSequence + BitAccess + BitLogic + Copy + Default + Identity<Additive>, C::Array<NodeId>: Copy` (src/spanning.rs:87) : "Decompose a DAG into trunks, branches, and bridges."

src/topo.rs:
- `pub fn topo_sort<C: Capacity, B>(dag: &BitMatrix<B, C>) -> (USize, C::Array<NodeId>) where B: BitSequence + BitAccess + BitLogic + Copy + Default, C::Array<USize>: Copy, C::Array<NodeId>: Copy` (src/topo.rs:36) : "Topologically sort a DAG via Kahn's algorithm."
- `pub fn renumber<C: Capacity>(topo_order: &C::Array<NodeId>) -> C::Array<NodeId> where C::Array<NodeId>: Copy` (src/topo.rs:108) : "Inverse permutation: map new topo position to old `NodeId`."

src/waist.rs:
- `pub fn waist_detect<C: Capacity, B>(dag: &BitMatrix<B, C>, topo_order: &C::Array<NodeId>) -> Mask<B> where B: BitSequence + BitAccess + BitLogic + Copy + Default + Identity<Additive>, C::Array<USize>: Copy, C::Array<Bool>: Copy` (src/waist.rs:27) : "Detect waist levels in a DAG."

src/waist_const.rs:
- `pub const fn waist_detect_const<C, W>(succ_rows: &C::Array<W>, topo_order: &C::Array<NodeId>) -> C::Array<Bool> where C: [const] ConstCapacity, W: [const] BitAccess` (src/waist_const.rs:29) : "Detect waist levels in a DAG at compile time."

### Stated behaviour
- "If W is `UFixed<{IBits(16)}, {FBits(0)}, Hot>`, rank accumulation wraps on overflow. If the same type with `Precise`, it saturates. The algorithm is identical. The consumer chooses the safety level by choosing W." (DESIGN.md.tmpl:62-65)
- "The algorithm needs addition (accumulate path weights), ordering (find maximum), copy (store in arrays), and a zero value (initialise). It does not need multiplication, division, or square root." (DESIGN.md.tmpl:58-60)
- "On a cyclic graph, `valid_count < N` and only the first `valid_count` entries of `topo_order` are meaningful; the rest are defaulted to `NodeId::new(USize(0))`." (src/topo.rs:29-31)
- "Cycle detection built-in: `valid_count < node_count` means a cycle was detected." (DESIGN.md.tmpl:79-80)
- "Unreached sources are skipped, a cyclic input silently yields an empty decomposition for the cyclic remainder." (src/spanning.rs:84-85)
- "Nodes outside the valid prefix (e.g. when a cycle clipped the topo sort) contribute nothing." (src/waist.rs:24-25)
- "`pred_of[i]` is the chosen predecessor of node `i` when `has_predecessor` bit `i` is set; otherwise undefined." (src/path.rs:27-28)
- "The node ceiling is the chosen word's width, not a fixed 64; the algorithm bodies are the same across widths." (DESIGN.md.tmpl:29-30)
- "The runtime `waist_detect` is not const: it builds arrays through `Capacity::filled` + `AsRef`/`AsMut` and iterates predecessors via `Mask::iter_set_bits`, none const-callable." (DESIGN.md.tmpl:207-209)
- "`const_trait_impl` (WATCH-tier ALLOWED in the unstable-feature sweep; same gate arvo-tensor uses for `ConstCapacity`) gates `waist_detect_const`" (src/lib.rs:24-26)
- Strategy markers: `Hot` / `Precise` appear only in the overflow example above; no other strategy statement. Named standards: none.
- Consumer bound: "bounded on the bit-sequence traits the bodies use (`BitSequence + BitAccess + BitLogic + Copy + Default + Identity<Additive>`)" (DESIGN.md.tmpl:23-24)

### Backlog and known gaps
BACKLOG.md.tmpl (no SHAME.md.tmpl; one untitled leading paragraph, then):
- `## Wider-W ergonomic helpers` : "Common pipeline workloads in observed research stay well under 64 nodes, so the 64-wide instantiation covers the common case." (BACKLOG.md.tmpl:17-18)

### Tests
tests/capacity_threading.rs (3), tests/components.rs (5), tests/generic_width.rs (3), tests/path.rs (4), tests/rank.rs (5), tests/spanning.rs (4), tests/topo.rs (6), tests/waist.rs (3), tests/waist_const.rs (3). src files: 0 `#[test]`. (counts by grep; bodies not read)

## arvo-hash   (files read: 9 non-test; 3 test files counted by grep only)

- purpose: "L2 hash-domain types and algorithm contracts. `ContentHash` aliases arvo `Bits<64, Hot>` for call-site domain naming." (README.md.tmpl:3-4)
- depends on: arvo, arvo-bits-contracts (`.workspace = true`); external `xxhash-rust = { version = "0.8", default-features = false, features = ["xxh3", "const_xxh3"] }`; `[features] default = []` (Cargo.toml:7-13)
- attributes: `#![no_std]`, `#![feature(adt_const_params)]`, `#![feature(const_trait_impl)]`, `#![allow(incomplete_features)]` (src/lib.rs:18-21)

### Public items
src/lib.rs:
- `pub mod algo;` `pub mod aliases;` `pub mod fnv1a;` `pub mod xxhash3;` (src/lib.rs:23-26) : no doc
- `pub use algo::{ConstHash, Hasher, fnv1a_64};` `pub use aliases::ContentHash;` `pub use fnv1a::Fnv1a;` `pub use xxhash3::{XxHash3, xxhash3_64};` (src/lib.rs:28-31) : no doc

src/algo.rs:
- `pub trait Hasher<const N: u16> where Hot: BitsContainerFor<N, Unsigned>` (src/algo.rs:29) : "Streaming N-bit hasher. Feed bytes via `update`, finalise to a"
  - `fn update(&mut self, bytes: &[u8]);` (:34) : "Feed a byte chunk into the hasher."
  - `fn finalize(self) -> Bits<N, Hot>;` (:37) : "Consume the hasher and produce the final N-bit digest."
- `pub const trait ConstHash<const N: u16, S: Strategy, Sign: Signedness>: Sized where S: BitsContainerFor<N, Sign>, <S as BitsContainerFor<N, Sign>>::T: [const] NarrowFromU64<N, S, Sign>` (src/algo.rs:56) : "Compile-time, one-shot hash construction."
  - `fn hash_const(bytes: &[u8]) -> Bits<N, S, Sign>;` (:62) : "Hash the byte slice and produce the typed N-bit digest."
- `pub const fn fnv1a_64(bytes: &[u8]) -> u64` (src/algo.rs:73) : "FNV-1a-64 over a byte slice (free const fn)."

src/aliases.rs:
- `pub type ContentHash = Bits<64, Hot>;` (src/aliases.rs:12) : "64-bit content-hash identity."

src/fnv1a.rs:
- `pub struct Fnv1a<const N: u16> where Hot: BitsContainerFor<N, Unsigned> { state: u64 }` (src/fnv1a.rs:33) : "Streaming FNV-1a-64 hasher with N-bit output."
- `pub const fn new() -> Self` (src/fnv1a.rs:55) : "Construct a fresh hasher in its initial state."
- (trait impls: `Default for Fnv1a<N>`; `impl<const N: u16> Hasher<N> for Fnv1a<N>` with `NarrowFromU64` bound; `const impl<const N: u16> ConstHash<N, Hot, Unsigned> for Fnv1a<N>`) (:62, :77, :110)

src/xxhash3.rs:
- `pub const fn xxhash3_64(bytes: &[u8]) -> u64` (src/xxhash3.rs:29) : "XxHash3-64 over a byte slice (free const fn)."
- `pub struct XxHash3<const N: u16> where Hot: BitsContainerFor<N, Unsigned> { buffer: [u8; 256], pos: usize }` (src/xxhash3.rs:52) : "Streaming XxHash3-64 hasher with N-bit output."
- `pub const fn new() -> Self` (src/xxhash3.rs:72) : "Construct a fresh hasher in its initial state."
- (trait impls: `Default for XxHash3<N>`; `impl<const N: u16> Hasher<N> for XxHash3<N>`; `const impl<const N: u16> ConstHash<N, Hot, Unsigned> for XxHash3<N>`) (:81, :93, :134)

### Stated behaviour
- "`N` must satisfy `1 <= N <= 64`. Wider widths require a different state width (`Fnv1a128`, deferred)." (src/fnv1a.rs:23-24)
- "`N` must satisfy `1 <= N <= 64`. Wider widths require a different state width (`XxHash3_128`, deferred)." (src/xxhash3.rs:36-37)
- "Width is constrained to `1..=64` implicitly by `Hot: BitsContainerFor<N, Unsigned>` plus FNV-1a-64's 64-bit state." (src/fnv1a.rs:10-11)
- "Buffer overflow is a contract violation (the streaming surface bounds at 256 bytes by design; consumers above that band reach for `ConstHash::hash_const`). Visible in debug, silent in release to match the no-overhead substrate philosophy." (src/xxhash3.rs:100-103)
- "The streaming `update` is bounded by a 256-byte stack buffer. Bytes beyond the buffer trigger `debug_assert!` in debug builds (silent in release" (DESIGN.md.tmpl:149)
- "The streaming impl buffers bytes and computes the hash on `finalize()`. xxhash-rust's streaming Xxh3 is alloc-using; the const-friendly API is one-shot only." (src/xxhash3.rs:47-49)
- "`update` runs the FNV-1a round byte-by-byte; `finalize` narrows the 64-bit state to N bits via `NarrowFromU64`." (src/fnv1a.rs:75-76)
- "The `[const]` host-effect bound on the supertrait constraint is required for cross-crate const dispatch." (src/algo.rs:46-47)
- "`XxHash3<const N: u16>` is the default hash family. `Fnv1a<const N: u16>` remains as an explicit option for known-good fits (small inputs in the 8-16 byte band where FNV-1a stays competitive on bandwidth, or algorithm-locked test fixtures)." (DESIGN.md.tmpl:101-104)
- "XxHash3 wins on bandwidth above the 32-byte input mark; FNV-1a holds at 8-16 bytes." (DESIGN.md.tmpl:108-109)
- Strategy markers: `Hasher` fixed to `Hot` (`Bits<N, Hot>`); "The `ConstHash<N, Hot, Unsigned>` impl ships; non-Hot strategy impls are BACKLOG." (DESIGN.md.tmpl:151)
- Named standards: "FNV-1a-64 internal state width is fixed by the algorithm specification" (src/fnv1a.rs:37); "xxhash3 state is u64 by algorithm spec" (src/xxhash3.rs:28); `xxhash-rust` crate.
- Rounding / overflow: `fnv1a_64` body uses `hash.wrapping_mul(FNV_PRIME)` (src/algo.rs:80); no prose statement.
- "Cryptographic hashes (SHA, BLAKE); out of scope entirely." (DESIGN.md.tmpl:248)

### Backlog and known gaps
BACKLOG.md.tmpl (no SHAME.md.tmpl):
- `## Concrete algorithms` : "Shipping order as consumers surface need:" (BACKLOG.md.tmpl:10)
- `## Fnv1a128 algorithm` : "`Fnv1a<const N: u16>` produces at most 64 bits of state (FNV-1a-64 algorithm with low-N masking)." (BACKLOG.md.tmpl:21)
- `## Hilavitkutin-str migration` : "hilavitkutin-str const_fnv1a is the existing FNV implementation." (BACKLOG.md.tmpl:25)
- `## Wider bit widths` : "If arvo-bits grows u128 backing for Bits (its own BACKLOG item), arvo-hash gains Hash128-domain semantic aliases." (BACKLOG.md.tmpl:31-32)
- `## Cryptographic hashes` : "Out of scope." (BACKLOG.md.tmpl:36)
- `## Semantic aliases` : "NameHash, KeyHash, or similar named aliases beyond ContentHash land once consumer patterns stabilise." (BACKLOG.md.tmpl:42-43)

### Tests
tests/algo.rs (3), tests/aliases.rs (2), tests/fnv1a.rs (6). src files: 0 `#[test]`. (counts by grep; bodies not read)

## arvo-mask-contracts   (files read: 5)

- purpose: "The mask operations contract. One trait declaration: `MaskOps`." (DESIGN.md.tmpl:3); lib.rs: "`MaskOps` is the abstract const trait over a bitmask." (src/lib.rs:7)
- depends on: notko, arvo-storage, arvo-strategy, arvo-bits-contracts (`.workspace = true`); `[features] default = []` (Cargo.toml:7-14)
- attributes: `#![no_std]`, `#![feature(const_trait_impl)]`, `#![allow(incomplete_features)]` (src/lib.rs:1-3)

### Public items
src/lib.rs:
- `pub const trait MaskOps: Sized + Copy` (src/lib.rs:45) : "Bitmask predicate."
  - `fn empty() -> Self;` (:47) : "Mask with no bits set."
  - `fn full() -> Self;` (:49) : "Mask with every bit set."
  - `fn set(self, idx: USize) -> Self;` (:51) : "Set bit `idx`. Returns `self` unchanged for `idx >= width`."
  - `fn clear(self, idx: USize) -> Self;` (:53) : "Clear bit `idx`. Returns `self` unchanged for `idx >= width`."
  - `fn test(self, idx: USize) -> Bool;` (:55) : "Test bit `idx`. Returns `Bool::FALSE` for `idx >= width`."
  - `fn count(self) -> USize;` (:57) : "Count of set bits."
  - `fn union(self, other: Self) -> Self;` (:59) : "Set-theoretic union."
  - `fn intersection(self, other: Self) -> Self;` (:61) : "Set-theoretic intersection."
  - `fn difference(self, other: Self) -> Self;` (:63) : "Set-theoretic difference: `self & !other`."
  - `fn complement(self) -> Self;` (:65) : "Set-theoretic complement."

### Stated behaviour
- "Width follows from the implementor's underlying word type; the trait is uniform across the width axis." (src/lib.rs:38-40)
- "The trait is uniform across mask-width: width information lives on the implementor's underlying word type, not as a trait parameter." (DESIGN.md.tmpl:27-28)
- "Default-method bodies compose `BitLogic` and `BitAccess` from `arvo-bits-contracts`; concrete impls in `arvo-bitmask` may override with hardware-specific implementations." (src/lib.rs:42-44)
- "Per-W concrete overrides are deferred to a future bench-driven round when a measured win justifies the override." (DESIGN.md.tmpl:69-71)
- Overflow, rounding, strategy markers, named standards: none stated.

### Backlog and known gaps
BACKLOG.md.tmpl (no SHAME.md.tmpl):
- `# arvo-mask-contracts. Backlog` has body "(none currently)" (BACKLOG.md.tmpl:3)
- `## Removed entries` : "(none)" (BACKLOG.md.tmpl:7)

### Tests
No test files. src files: 0 `#[test]`.

## arvo-numeric-contracts   (files read: 6)

- purpose: "Numeric-op trait contracts: `Abs`, `Recip`, `Sqrt`, `TotalOrd`, `FromConstant`, the `Predicate` family. All `pub const trait`." (README.md.tmpl:3-4)
- depends on: notko, arvo-transparent, arvo-strategy, arvo-storage, arvo-bits-contracts (`.workspace = true`); `[features] default = []` (Cargo.toml:7-15)
- attributes: `#![no_std]`, `#![feature(adt_const_params)]`, `#![feature(const_trait_impl)]`, `#![allow(incomplete_features)]` (src/lib.rs:1-4)

### Public items
src/lib.rs:
- `pub const trait Abs` (src/lib.rs:27) : "Absolute value."
  - `type Output;` (:29) : "Output type of the absolute value."
  - `fn abs(self) -> Self::Output;` (:31) : "Return the absolute value of `self`."
- `pub const trait Recip` (src/lib.rs:40) : "Multiplicative inverse `1 / x`."
  - `type Output;` (:42) : "Output type of the reciprocal."
  - `fn recip(self) -> Self::Output;` (:44) : "Return `1 / self`."
- `pub const trait Sqrt` (src/lib.rs:52) : "Square root."
  - `type Output;` (:54) : "Output type of the square root."
  - `fn sqrt(self) -> Self::Output;` (:56) : "Return the square root of `self`."
- `pub const trait TotalOrd` (src/lib.rs:65) : "Total ordering."
  - `fn total_cmp(self, other: Self) -> Ordering;` (:67) : "Return the total ordering of `self` vs `other`."
- `pub const trait FromConstant` (src/lib.rs:85) : "Compile-time constant constructor."
  - `fn from_constant<const C: USize>() -> Self;` (:87) : "Construct the value `C` in the target type."
- `pub const trait Predicate` (src/lib.rs:97) : "Boolean predicate over a numeric value."
  - `fn test(self) -> Bool;` (:99) : "Run the predicate and return its boolean result."
- `pub const trait IsZero: Predicate {}` (src/lib.rs:103) : "Predicate that holds when `self` is the zero value of its type."
- `pub const trait IsPositive: Predicate {}` (:106) : "Predicate that holds when `self` is strictly greater than zero."
- `pub const trait IsNonZero: Predicate {}` (:109) : "Predicate that holds when `self` is not the zero value."
- `pub const trait IsNonNegative: Predicate {}` (:112) : "Predicate that holds when `self` is not strictly less than zero."
- `pub const trait IsZeroOrPositive: Predicate {}` (:115) : "Predicate that holds when `self` is zero or strictly positive."
- `mod predicates;` (private) with `pub use predicates::{ConstSign, IsNonNegativeOf, IsNonZeroOf, IsPositiveOf, IsZeroOf, IsZeroOrPositiveOf};` (src/lib.rs:117-121) : no doc

src/predicates.rs:
- `pub const trait ConstSign` (src/predicates.rs:64) : "Const-callable sign predicates over a numeric type."
  - `fn is_positive(self) -> Bool;` (:66) : "True iff `self > 0`."
  - `fn is_non_negative(self) -> Bool;` (:68) : "True iff `self >= 0`."
  - `fn is_zero_or_positive(self) -> Bool;` (:72) : "True iff `self >= 0` (semantic alias of `is_non_negative`;"
  - blanket: `const impl<T> ConstSign for T where T: Copy + [const] ConstOrd + [const] Identity<Additive>` (:75)
- `pub struct IsZeroOf<T: Copy>(pub T);` (:96) : "Wraps a value and tests whether it equals `<T as Identity<Additive>>::IDENTITY`."
- `pub struct IsPositiveOf<T: Copy>(pub T);` (:101) : "Wraps a value and tests whether it is strictly greater than zero."
- `pub struct IsNonZeroOf<T: Copy>(pub T);` (:106) : "Wraps a value and tests whether it is not equal to zero."
- `pub struct IsNonNegativeOf<T: Copy>(pub T);` (:111) : "Wraps a value and tests whether it is not less than zero."
- `pub struct IsZeroOrPositiveOf<T: Copy>(pub T);` (:116) : "Wraps a value and tests whether it is zero or strictly positive."
  - each `#[repr(transparent)]`, derives `Copy, Clone, PartialEq, Eq, Default, Debug`; `const unsafe impl<T: Copy> Transparent` with `type Inner = T` (:120-134); `const impl<T> Predicate` and marker impls per wrapper (:138-216)

### Stated behaviour
- "`Output` is `Self` for unsigned types and for signed types whose domain represents the absolute value of its full range without loss. Per-impl contracts state how edge values are handled (saturate vs wrap) where relevant." (src/lib.rs:23-26)
- "Domain restrictions are per-impl. For `UFixed<I, F, S>` with `F == 0` this returns the fractional dual `UFixed<0, I, S>`; for `F > 0` it returns the same shape via fixed-point reciprocal. Float impls follow IEEE semantics." (src/lib.rs:36-39)
- "Domain is non-negative values. Signed-input impls clamp negatives to zero or panic per per-impl contract. Integer impls return the floor of the true square root." (src/lib.rs:49-51)
- "Distinct from `core::cmp::Ord` so float-bearing arvo types can implement a strict-NaN-policy total order without conflicting with the partial `core::PartialOrd`." (src/lib.rs:61-63)
- "For fixed-point types, `C` is placed at the integer-bit position (multiplied by `2^F`), so `from_constant::<{USize(1)}>()` gives one whole unit. Out-of-range constants truncate at the container. This trait is compile-time only; runtime conversions belong to a different surface." (src/lib.rs:80-84)
- "Floats cannot impl `ConstOrd` (NaN breaks reflexivity), so they impl `ConstSign` directly in the arvo facade with bare-primitive comparison against 0.0." (src/predicates.rs:13-15)
- "`IsNonNegative` and `IsZeroOrPositive` share a body but the marker distinction lets consumers name the semantic they care about. For unsigned types the predicate is trivially true" (src/predicates.rs:33-35)
- "Default-impl bodies for the standard cases live in the `arvo` facade" (DESIGN.md.tmpl:4)
- Named standards: "IEEE semantics" (src/lib.rs:39). Strategy markers: none stated.

### Backlog and known gaps
BACKLOG.md.tmpl (no SHAME.md.tmpl):
- `# arvo-numeric-contracts. Backlog` body: "(none currently)" (BACKLOG.md.tmpl:3)
- `## Removed entries` : "(none)" (BACKLOG.md.tmpl:7)

### Tests
No test files. src files: 0 `#[test]`.

## arvo-refit   (files read: 5)

- purpose: "Re-export crate for the bit-width refit trait family (`Narrow` and `Widen`)." (README.md.tmpl:3-4); lib.rs: "arvo-refit. Re-export gateway for the bit-width refit family." (src/lib.rs:1)
- depends on: notko, arvo-storage, arvo-bits-contracts, arvo-mask-contracts, arvo-bitmask (`.workspace = true`); `[features] default = []` (Cargo.toml:7-15)
- attributes: `#![no_std]` (src/lib.rs:15)

### Public items
src/lib.rs:
- `pub use arvo_bits_contracts::{Narrow, Narrowed, Widen, Widened};` (src/lib.rs:17) : no doc

### Stated behaviour
- "`Narrow<T>` and `Widen<T>` are const traits. `Narrowed<const N: u16, T>` and `Widened<T>` are cosmetic aliases for return positions in consumer signatures." (DESIGN.md.tmpl:20-22)
- "`Narrow<T>` for shrinking a value into a narrower bit-width (with high-bit masking), and `Widen<T>` for promoting a value into a wider bit-width (zero-extending for unsigned, sign-extending for signed)." (DESIGN.md.tmpl:4-7)
- "Depends on `arvo-bits-contracts`." (DESIGN.md.tmpl:11)
- Overflow, strategy markers, named standards: none stated.

### Backlog and known gaps
BACKLOG.md.tmpl (no SHAME.md.tmpl):
- `# arvo-refit. Backlog` body: "(none currently)" (BACKLOG.md.tmpl:3)
- `## Removed entries` : "(none)" (BACKLOG.md.tmpl:7)

### Tests
No test files. src files: 0 `#[test]`.

## arvo-sparse   (files read: 11 non-test; 6 test files counted by grep only)

- purpose: "Sparse matrix algorithms on const-generic structures. CSR format, RCM reordering, block diagonal detection, Dulmage-Mendelsohn decomposition. `#![no_std]`, no alloc." (README.md.tmpl:3-5)
- depends on: arvo, arvo-bitmask, arvo-bits-contracts, arvo-tensor, notko (`.workspace = true`); `[features] default = []` (Cargo.toml:7-15)
- attributes: `#![no_std]`, `#![feature(const_trait_impl)]` (src/lib.rs:18-19)

### Public items
src/lib.rs:
- `pub mod adjacency;` `pub mod block;` `pub mod csr;` `pub mod dm;` `pub mod rcm;` (src/lib.rs:21-25) : no doc
- `pub use adjacency::{BandwidthReducer, BidirectionalSparseAdjacency, BipartiteStructuralAnalysis, BlockPartitioner, SparseAdjacency};` `pub use block::{block_diagonal, block_diagonal_via};` `pub use csr::{Csr, CsrBidirectional};` `pub use dm::{DulmageMendelsohn, classification_to_mask, dulmage_mendelsohn, dulmage_mendelsohn_via};` `pub use rcm::{rcm_reorder, rcm_reorder_via};` (src/lib.rs:27-36) : no doc

src/adjacency.rs:
- `pub const trait SparseAdjacency<C: Capacity>` (src/adjacency.rs:38) : "Sealed const trait naming the interface for sparse adjacency."
  - `type Successors<'a>: Iterator<Item = NodeId> where Self: 'a;` (:44) : "Iterator over the successors of a given node."
  - `fn successors<'a>(&'a self, node: NodeId) -> Self::Successors<'a>;` (:49) : "Walk the successor set of `node` as an iterator."
  - `fn node_count(&self) -> USize;` (:52) : "Number of nodes in the graph."
- `pub const trait BidirectionalSparseAdjacency<C: Capacity>: SparseAdjacency<C>` (src/adjacency.rs:64) : "Sealed const trait extending `SparseAdjacency<C>` with cheap"
  - `type Predecessors<'a>: Iterator<Item = NodeId> where Self: 'a;` (:66) : "Iterator over the predecessors of a given node."
  - `fn predecessors<'a>(&'a self, node: NodeId) -> Self::Predecessors<'a>;` (:71) : "Walk the predecessor set of `node` as an iterator."
- `pub struct BitMatrixSuccessors<W> where W: BitSequence + BitAccess + BitLogic + Copy + Default { inner: SetBitsIter<W> }` (src/adjacency.rs:83) : "Iterator yielding `NodeId` for set bits of a `Mask<W>`."
- (impls: `Iterator for BitMatrixSuccessors<W>`; `impl<W, C: Capacity> SparseAdjacency<C> for BitMatrix<W, C>`; `impl<W, C: Capacity> BidirectionalSparseAdjacency<C> for BitMatrix<W, C>`) (:90, :105, :127)
- `pub trait BandwidthReducer<C: Capacity>: BidirectionalSparseAdjacency<C>` (src/adjacency.rs:156) : "Bandwidth-reduction reorder. Default impl runs RCM."
  - `fn reduce_bandwidth(&self) -> C::Array<NodeId> where Self: Sized` (default body) (:160) : "Return a permutation `C::Array<NodeId>` where"
- `pub trait BlockPartitioner<C: Capacity>: BidirectionalSparseAdjacency<C>` (:169) : "Block-diagonal partitioner. Default impl runs connected-components."
  - `fn partition_blocks(&self) -> (USize, C::Array<USize>) where Self: Sized` (default body) (:172) : "Return `(block_count, per_node_block_ids)`."
- `pub trait BipartiteStructuralAnalysis<C: Capacity>: BidirectionalSparseAdjacency<C>` (:181) : "Bipartite structural analyser. Default impl runs Dulmage-Mendelsohn."
  - `fn analyse_structure(&self) -> crate::dm::DulmageMendelsohn<C> where Self: Sized` (default body) (:184) : "Return a `DulmageMendelsohn<C>` classification."
- blanket impls of the three problem-shaped traits for `T: BidirectionalSparseAdjacency<C>` (:194-198)

src/block.rs:
- `pub fn block_diagonal<W, C: Capacity>(adjacency: &BitMatrix<W, C>) -> (USize, C::Array<USize>) where W: BitSequence + BitAccess + BitLogic + Identity<Additive> + Copy + Default` (src/block.rs:29) : "Assign a component (block) ID to every node."
- `pub fn block_diagonal_via<T, C: Capacity>(adjacency: &T) -> (USize, C::Array<USize>) where T: BidirectionalSparseAdjacency<C>` (src/block.rs:95) : "Trait-driven variant of `block_diagonal`."

src/csr.rs:
- `pub struct Csr<R: Capacity, NNZ: Capacity, W: Copy>` (src/csr.rs:40) : "Compressed sparse row matrix."
  - `pub row_ptr: R::Array<USize>` (:43) : "Row start offsets. `row_ptr[r]` is the index of row `r`'s"
  - `pub col_idx: NNZ::Array<NodeId>` (:45) : "Column index of each non-zero, flattened row-major."
  - `pub values: NNZ::Array<W>` (:47) : "Value of each non-zero, flattened row-major."
  - `pub live_rows: USize` (:52) : "Live row count. Rows `0..live_rows` carry data; rows at or"
  - `pub live_nnz: USize` (:57) : "Live non-zero count. The last live row ends at `live_nnz`"
  - `impl ... Copy`, `impl ... Clone` (:60, :68); `impl ... Default for Csr` (:217)
- `pub fn new() -> Self` (impl `W: Copy + Default`) (:87) : "Empty matrix: all offsets zero, all columns `NodeId(USize(0))`,"
- `pub fn with_live_counts(live_rows: USize, live_nnz: USize) -> Self` (:108) : "Empty matrix with explicit live counts."
- `pub fn get(&self, row: USize, col: NodeId) -> Maybe<W>` (:144) : "Value at `(row, col)` if present, otherwise `Maybe::Isnt`."
- `pub fn row_values(&self, row: USize) -> &[W]` (:170) : "Slice of value entries for `row`."
- `pub fn row_col_indices(&self, row: USize) -> &[NodeId]` (:187) : "Slice of column indices for `row`."
- `pub fn nnz(&self, row: USize) -> USize` (:204) : "Number of non-zeros in `row`."
- (private) `fn row_end(&self, r: USize) -> USize` (:122)
- `impl<R: Capacity, NNZ: Capacity, W: Copy> SparseAdjacency<R> for Csr<R, NNZ, W>` (:231)
- `pub struct CsrBidirectional<R: Capacity, NNZ: Capacity, W: Copy>` (:278) : "Bidirectional compressed sparse row matrix."
  - `pub forward: Csr<R, NNZ, W>` (:280) : "Forward CSR (carries values)."
  - `pub transpose_row_ptr: R::Array<USize>` (:283) : "Transpose row start offsets. `transpose_row_ptr[j]` indexes"
  - `pub transpose_col_idx: NNZ::Array<NodeId>` (:285) : "Transpose column indices: the predecessor rows themselves."
  - `pub live_rows: USize` (:289) : "Live row count, carried from the forward CSR. Predecessor"
  - `pub live_nnz: USize` (:292) : "Live non-zero count, carried from the forward CSR. The last"
  - `impl ... Copy`, `impl ... Clone` (:295, :303)
- `pub fn with_transpose(self) -> CsrBidirectional<R, NNZ, W> where R::Array<USize>: Copy, NNZ::Array<NodeId>: Copy` (impl `W: Copy + Default` on `Csr`) (:334) : "Consume this CSR and build a bidirectional view with the"
- `impl ... SparseAdjacency<R> for CsrBidirectional`; `impl ... BidirectionalSparseAdjacency<R> for CsrBidirectional` (:471, :493); (private) `transpose_row_end`, `predecessors_slice` (:439, :457)

src/dm.rs:
- `pub struct DulmageMendelsohn<C: Capacity>` (src/dm.rs:36) : "Dulmage-Mendelsohn decomposition result."
  - `pub class_count: USize` (:38) : "Number of distinct classes used."
  - `pub class: C::Array<USize>` (:40) : "Class ID per node: `0` horizontal, `1` vertical, `2` square."
  - `impl ... Copy`, `Clone`, `Default` (:43, :45, :55)
- `pub fn dulmage_mendelsohn<W, C: Capacity>(adjacency: &BitMatrix<W, C>) -> DulmageMendelsohn<C> where W: BitSequence + BitAccess + BitLogic + Copy + Default` (:67) : "Classify each node in the adjacency into one of the three classes."
- `pub fn dulmage_mendelsohn_via<T, C: Capacity>(adjacency: &T) -> DulmageMendelsohn<C> where T: BidirectionalSparseAdjacency<C>` (:107) : "Trait-driven variant of `dulmage_mendelsohn`."
- `pub fn classification_to_mask<W, C: Capacity>(dm: &DulmageMendelsohn<C>, class_id: USize) -> Maybe<Mask<W>> where W: BitPrim + BitSequence + BitAccess + BitLogic + arvo::Identity<arvo::Additive> + Copy + Default` (:147) : "Project a class into a fits-in-W bitmask."

src/rcm.rs:
- `pub fn rcm_reorder<W, C: Capacity>(adjacency: &BitMatrix<W, C>) -> C::Array<NodeId> where W: BitSequence + BitAccess + BitLogic + Identity<Additive> + Copy + Default` (src/rcm.rs:36) : "Reverse Cuthill-McKee permutation."
- `pub fn rcm_reorder_via<T, C: Capacity>(adjacency: &T) -> C::Array<NodeId> where T: BidirectionalSparseAdjacency<C>` (:182) : "Trait-driven variant of `rcm_reorder`."
- (private) `degree`, `min_degree_unvisited`, `degree_via`, `min_degree_unvisited_via` (:136, :146, :292, :317)

### Stated behaviour
- "Absence is modelled with `notko::Maybe<T>`, not bare `Option<T>`, per the workspace primitive-vocabulary discipline." (DESIGN.md.tmpl:14-15)
- "Every public algorithm signature here is W-generic. The bitmask path takes `&BitMatrix<W, C>` with `W: BitPrim`; the trait-driven path takes any `&T` where `T: SparseAdjacency<C>`." (DESIGN.md.tmpl:23-25)
- DESIGN lists in `SparseAdjacency` a method `fn successor_mask<W: BitPrim>(&self, node: NodeId) -> Mask<W>;` and says "`successor_mask` has a default impl from the `successors` iterator" (DESIGN.md.tmpl:148, :162); src/adjacency.rs:38-53 declares `successors` and `node_count` in that trait and doc comment text at src/adjacency.rs:34-37 mentions `successor_mask`.
- DESIGN names trait methods `reduce`, `partition`, `analyse` (DESIGN.md.tmpl:201, :207, :213); src/adjacency.rs names `reduce_bandwidth`, `partition_blocks`, `analyse_structure` (:160, :172, :184).
- "Class IDs: `0 = horizontal`, `1 = vertical`, `2 = square`." (DESIGN.md.tmpl:309)
- "The three classes partition the `N` node indices exactly: every node `i` in `0..N` carries exactly one class ID." (src/dm.rs:15-16)
- "Returns `Maybe::Isnt` when the node count exceeds `W::WIDTH`" (DESIGN.md.tmpl:343)
- "Reverse Cuthill-McKee bandwidth minimisation. O(n + nnz)." (DESIGN.md.tmpl:227)
- "The pseudo-peripheral heuristic (BFS-diameter) is deferred; min-degree start is adequate for this round's scope." (src/rcm.rs:14-16)
- "This round ships read-only storage. The constructor fills every slot with a default value." (src/csr.rs:18-19)
- "Preconditions: the input CSR is well-formed. [...] Out-of-range column indices are silently skipped (treated as having no transpose contribution); a malformed `row_ptr` may truncate the scatter at the per-column cursor bound." (src/csr.rs:324-329)
- "The type system is the contract: substrate algorithms trust their input shapes per `arvo-toolbox-not-policer`." (src/csr.rs:330-331)
- "`r >= live_rows` yields `USize(0)` (empty range)." (src/csr.rs:120)
- "Predecessor queries cost O(N + NNZ) per query and are not supported on `Csr` directly" (DESIGN.md.tmpl:88-89)
- "The `pub const trait` declarations stay `const` across the capacity-type migration; the impls are plain (non-const) impls carrying the runtime graph-query bodies" (DESIGN.md.tmpl:174-176)
- "Design intent: run unconditionally during planning when the consumer is a pipeline scheduler. Sub-microsecond at typical pipeline scale." (DESIGN.md.tmpl:258-260)
- Strategy markers: the alias `pub type Adjacency<C: Capacity> = BitMatrix<Bits<64, Hot, Unsigned>, C>;` (DESIGN.md.tmpl:29-30). Named standards: none.

### Backlog and known gaps
BACKLOG.md.tmpl:
- `## SpMV` : "Sparse matrix-vector multiply patterns." (BACKLOG.md.tmpl:16)
- `## arvo-graph W-generic sweep` : "`arvo-graph` carries the same `Bits<64, Hot, Unsigned>` hardcoding in `topo_sort`, `upward_rank`, `downward_rank`, `longest_path`, `components`, `waist_detect`, `spanning_tree`." (BACKLOG.md.tmpl:22-24)

SHAME.md.tmpl (title `# arvo-sparse. Known gaps`; no `##` headings):
- "The entries below silence push-gate `design-doc-source-mismatch` lint warnings for types documented in the round-202605111719 DOC-phase that do not yet exist in source." (SHAME.md.tmpl:3-5)

### Tests
tests/block.rs (5), tests/capacity_threading.rs (3), tests/csr.rs (5), tests/dm.rs (5), tests/live_counts.rs (7), tests/rcm.rs (4). src files: 0 `#[test]`. (counts by grep; bodies not read)

## arvo-spectral   (files read: 11 non-test; 10 test files counted by grep only, `tests/common/mod.rs` has 0)

- purpose: "Spectral graph methods on const-generic structures. Laplacian construction, power iteration, Fiedler vector, spectral bisection. `#![no_std]`, no alloc. All types are arvo numerics." (README.md.tmpl:3-5)
- depends on: arvo, arvo-bitmask, arvo-sparse, arvo-tensor (`.workspace = true`); `[features] default = []` (Cargo.toml:7-14)
- attributes: `#![no_std]` (src/lib.rs:15); no feature gates.

### Public items
src/lib.rs:
- `pub mod fiedler;` `pub mod laplacian;` `pub mod matrix;` `pub mod operator;` `pub mod partition;` `pub mod power;` (src/lib.rs:17-22) : no doc
- `pub use fiedler::{dense_laplacian_lambda_max_bound, fiedler_vector};` `pub use laplacian::laplacian;` `pub use matrix::Matrix;` `pub use operator::{LinearOperator, SparseLaplacian};` `pub use partition::{SpectralBipartitioner, k_way_partition, spectral_bisection};` `pub use power::power_iteration;` (src/lib.rs:24-29) : no doc

src/fiedler.rs:
- `pub fn fiedler_vector<Op, C: Capacity, F>(operator: &Op, sigma: F, iterations: USize) -> C::Array<F> where Op: LinearOperator<F, C>, F: Add<Output = F> + Sub<Output = F> + Mul<Output = F> + Sqrt<Output = F> + Recip<Output = F> + TotalOrd + Copy + FromConstant` (src/fiedler.rs:54) : "Compute the Fiedler vector via shifted, deflated power iteration on"
- `pub fn dense_laplacian_lambda_max_bound<C: Capacity, F>(lap: &Matrix<F, C>) -> F where F: Add<Output = F> + Mul<Output = F> + TotalOrd + Copy + FromConstant` (src/fiedler.rs:160) : "Gershgorin upper bound on `lambda_max(L)` for a dense Laplacian."

src/laplacian.rs:
- `pub fn laplacian<C: Capacity, W, F>(weights: &Matrix<W, C>) -> Matrix<F, C> where W: Into<F> + Copy, F: Add<Output = F> + Sub<Output = F> + Copy + FromConstant` (src/laplacian.rs:31) : "Build the graph Laplacian `L = D - W`."

src/matrix.rs:
- `pub use arvo_tensor::{Matrix, cap_size};` (src/matrix.rs:8) : no doc

src/operator.rs:
- `pub trait LinearOperator<F, C: Capacity>` (src/operator.rs:30) : "Linear operator `y = A * x` on a fixed-capacity vector space."
  - `fn apply(&self, x: &C::Array<F>, y: &mut C::Array<F>);` (:32) : "Write `A * x` into `y`. `y` must not alias `x`."
  - `fn live_dim(&self) -> USize;` (:43) : "Number of nodes the operator spans."
  - impl: `impl<F, C: Capacity> LinearOperator<F, C> for Matrix<F, C> where F: Add<Output = F> + Mul<Output = F> + Copy + FromConstant` (:47)
- `pub struct SparseLaplacian<'data, R: Capacity, NNZ: Capacity, W, F> where W: Copy + Into<F> { csr: &'data Csr<R, NNZ, W>, _phantom: PhantomData<F> }` (src/operator.rs:96) : "Graph Laplacian over a CSR-backed weighted adjacency."
- `pub fn new(csr: &'data Csr<R, NNZ, W>) -> Self` (:110) : "Build a Laplacian operator over the borrowed CSR."
- `pub fn gershgorin_lambda_max(&self) -> F where F: Add<Output = F> + Mul<Output = F> + TotalOrd + Copy + FromConstant` (:133) : "Gershgorin upper bound on `lambda_max(L)`."
- `impl<'data, R: Capacity, NNZ: Capacity, W, F> LinearOperator<F, R> for SparseLaplacian<'data, R, NNZ, W, F>` (:167)

src/partition.rs:
- `pub fn spectral_bisection<C: Capacity, F>(fiedler: &C::Array<F>, live_n: USize) -> (USize, C::Array<USize>) where F: TotalOrd + Copy + FromConstant` (src/partition.rs:45) : "Partition `N` nodes into two classes by the sign of their Fiedler"
- `pub fn k_way_partition<Op, C: Capacity, K: Capacity, F>(operator: &Op, sigma: F, iterations: USize) -> (USize, C::Array<USize>) where Op: LinearOperator<F, C>, C::Array<F>: Copy, F: Add<Output = F> + Sub<Output = F> + Mul<Output = F> + Sqrt<Output = F> + Recip<Output = F> + TotalOrd + Copy + FromConstant` (:85) : "Recursive spectral k-way partitioning over a Laplacian operator."
- `pub trait SpectralBipartitioner<C: Capacity, F>: LinearOperator<F, C> where C::Array<F>: Copy, F: Add<Output = F> + Sub<Output = F> + Mul<Output = F> + Sqrt<Output = F> + Recip<Output = F> + TotalOrd + Copy + FromConstant` (:239) : "Spectral bipartition contract."
  - `fn lambda_max_bound(&self) -> F;` (:256) : "Return the operator's Gershgorin upper bound on"
  - `fn bipartition(&self, iterations: USize) -> (USize, C::Array<USize>) where Self: Sized` (default body) (:264) : "Run Fiedler iteration and sign-partition."
  - impl: `impl<'data, R: Capacity, NNZ: Capacity, W, F> SpectralBipartitioner<R, F> for SparseLaplacian<'data, R, NNZ, W, F>` (:276)

src/power.rs:
- `pub fn power_iteration<Op, C: Capacity, F>(operator: &Op, iterations: USize) -> C::Array<F> where Op: LinearOperator<F, C>, F: Add<Output = F> + Mul<Output = F> + Sqrt<Output = F> + Recip<Output = F> + TotalOrd + Copy + FromConstant` (src/power.rs:38) : "Run power iteration on `operator` for `iterations` rounds."

### Stated behaviour
- "The trait is plain (non-const). Power iteration and Fiedler run in non-const contexts" (DESIGN.md.tmpl:110-111)
- "The trait is plain `pub trait` rather than `pub const trait` because the underlying Fiedler iteration uses floating-point `Sqrt` / `Recip` operations that are not const-callable." (src/partition.rs:236-238)
- DESIGN shows `fiedler_vector<Op, C: Capacity, F>(operator: &Op, iterations: USize)` and `SpectralBipartitioner` as `pub const trait SpectralBipartitioner<C: Capacity>: SparseAdjacency<C>` with `fn bisect<F>` (DESIGN.md.tmpl:62-65, :202-216); src/fiedler.rs:54-58 takes `sigma: F` and src/partition.rs:239 declares `pub trait SpectralBipartitioner<C: Capacity, F>: LinearOperator<F, C>` with `lambda_max_bound` and `bipartition`.
- "Class IDs: `0 = positive`, `1 = non-positive`." (DESIGN.md.tmpl:187-188)
- "Ties go to the negative class; `F::from_constant(0)` compared strictly-greater-than via `TotalOrd`. `class_count` is always `2`." (src/partition.rs:7-9)
- "`partition_count` can be less than `K` if a component becomes unsplittable before the budget is exhausted." (src/partition.rs:16-17)
- "Recomputing per-pop is the documented approximation per arvo-spectral BACKLOG; restricted-Laplacian per-component eigen is a follow-up round's scope." (src/partition.rs:142-144)
- "No convergence ratio check this round: the caller picks the iteration count." (src/power.rs:5-6)
- "Behaviour when the vector collapses to zero [...]: the normalisation multiplies by `recip(sqrt(0))`, which for float types is infinity or NaN." (src/power.rs:31-34)
- "Only the sign pattern of the result is meaningful for `spectral_bisection`; magnitude is L2-normalised after the final step." (src/fiedler.rs:24-26)
- "`y` must not alias `x`." (src/operator.rs:11-12)
- "Assumes non-negative weights (the standard graph-Laplacian case)." (src/operator.rs:123)
- "Diagonal entries in the CSR are ignored (a self-edge does not contribute to either the off-diagonal or the degree, by the Laplacian's definition)." (src/operator.rs:93-95)
- "Diagonal weights in the input are ignored; only the off-diagonal edges contribute to the degree sum." (src/laplacian.rs:28-29)
- "The eigenvector computation uses float (FastFloat or StrictFloat). Weight-to-float conversion happens inside the operator's `apply`" (DESIGN.md.tmpl:78-80)
- "Spectral methods are typically used at small N (about 30 iterations, N around 32 in observed workloads)" (DESIGN.md.tmpl:52-54)
- "Researched threshold: engage spectral when the grouping problem has more than 5 nodes; below that a single trunk suffices." (DESIGN.md.tmpl:252-253)
- "`live_dim` [...] The method is required, not defaulted" (DESIGN.md.tmpl:114-119)
- Strategy markers (`Hot`/`Warm`/`Cold`/`Precise`): none named. Named standards: Gershgorin disc bound (src/operator.rs:119-120); no other external standard.

### Backlog and known gaps
BACKLOG.md.tmpl (no SHAME.md.tmpl):
- `## Other decompositions` : "SVD, QR, LU, Cholesky are all plausible additions if a consumer surfaces a concrete need." (BACKLOG.md.tmpl:10-11)
- `## Non-symmetric matrices` : "Current `fiedler_vector` assumes the weighted adjacency is symmetric (Laplacian derivation requires it)." (BACKLOG.md.tmpl:16-17)
- `## Restricted-Laplacian per-component eigen` : "`k_way_partition`'s recursive bisection re-computes a full-graph Laplacian per recursion." (BACKLOG.md.tmpl:23-24)

### Tests
tests/capacity_threading.rs (3), tests/common/mod.rs (0), tests/fiedler.rs (2), tests/laplacian.rs (3), tests/matrix.rs (4), tests/matrix_bounds.rs (5), tests/partition.rs (5), tests/partition_loose.rs (1), tests/partition_stack_full.rs (2), tests/power.rs (3). src files: 0 `#[test]`. (counts by grep; bodies not read)

## arvo-storage   (files read: 10; test files counted by grep only, bodies not read)

- purpose: "The minimal bare-primitive layer of arvo. Sits at the bottom of every dependency chain in the arvo." (DESIGN.md.tmpl:3-4); README: "Minimal L0 storage primitives for arvo." (README.md.tmpl:3)
- depends on: `notko.workspace = true`, `arvo-transparent.workspace = true`, `arvo-strategy.workspace = true` (Cargo.toml:8-10). Features: `default = []` (Cargo.toml:13), no others. `[lints.rust]`: `unexpected_cfgs = { level = "warn", check-cfg = ["cfg(arvo_full_layout_audit)"] }` (Cargo.toml:16). The cfg `arvo_full_layout_audit` gates `mod full_audit` in src/layout_assertions.rs:286-287.
- attributes: `#![no_std]` (lib.rs:1), `#![feature(adt_const_params)]` (lib.rs:2), `#![feature(const_trait_impl)]` (lib.rs:3), `#![feature(const_convert)]` (lib.rs:8), `#![feature(const_ops)]` (lib.rs:9), `#![feature(const_param_ty_trait)]` (lib.rs:10), `#![feature(try_trait_v2)]` (lib.rs:11), `#![feature(try_trait_v2_residual)]` (lib.rs:12), `#![allow(incomplete_features)]` (lib.rs:13). Comment at lib.rs:4-7: "const_convert is not stale: on the pinned nightly the feature is still named" / "`const_convert` and points at the live const-From/Into umbrella (rustc #143773)." / "The `const_from` successor name is not yet the gate name here. Reclassified by" / "the soundness sweep (task #624); keep the gate."

### Public items

src/lib.rs (modules `mod bits; mod bridges; mod layout_assertions; mod meta_bits; mod platform;` are private, lib.rs:23-27):
- `pub use arvo_strategy::{A1, A16, A32, A64, Align, Bounded, Identity, SignedIdentity, WideBits};` (lib.rs:29) : no doc
- `pub use bits::Bits;` (lib.rs:30) : no doc
- `pub use bridges::{ConstBitEq, ConstDefault, ConstEq, ConstOrd, ConstOrdering, ConstPartialEq};` (lib.rs:31-33) : no doc
- `pub use meta_bits::{FBits, IBits, MetaCarrier, Width, fbits, ibits, width};` (lib.rs:34) : no doc
- `pub use platform::{AsBool, Bool, BoolResidual, Cap, NUSize, USize};` (lib.rs:35) : no doc

src/bits.rs:
- `pub struct Bits<const N: u16, S: Strategy = Hot, Sign: Signedness = Unsigned>(<S as BitsContainerFor<N, Sign>>::T) where S: BitsContainerFor<N, Sign>;` (bits.rs:57-61) : "N-bit opaque bit-pattern. Transparent wrapper over the" (bits.rs:47); derives `PartialEq, Eq, Copy, Clone, Debug, Default, Hash`, `#[repr(transparent)]` (bits.rs:55-56)
  - `pub const fn from_raw(raw: <S as BitsContainerFor<N, Sign>>::T) -> Self` (bits.rs:86) : "Construct from the raw container value."
  - `pub const fn to_raw(self) -> <S as BitsContainerFor<N, Sign>>::T` (bits.rs:91) : "Project to the raw container value."
- Trait impls on `Bits` (no `pub` item of their own): `ConstParamTy_` (bits.rs:74), `const impl Bounded` (101), `const impl Identity<Op>` (113), `const impl SignedIdentity` (126), `const impl ConstPartialEq` (142), `const impl ConstEq` (155), `const impl ConstBitEq` (162), `const impl ConstOrd` (180), `const impl ConstDefault` (194), `core::ops::Deref` (209), `AsRef` (220), `const unsafe impl Transparent` (234).

src/bridges.rs:
- `pub const trait ConstPartialEq` (bridges.rs:39) : "Const-callable partial equality."
  - `fn const_eq(&self, other: &Self) -> Bool;` (bridges.rs:41) : "Const-callable `==`. Partial: `NaN == NaN` returns `FALSE`."
  - `fn const_ne(&self, other: &Self) -> Bool` (bridges.rs:45) : "Const-callable `!=`. Default body negates `const_eq`."
- `pub const trait ConstEq: [const] ConstPartialEq {}` (bridges.rs:62) : "Const-callable equality with reflexivity guarantee."
- `pub const trait ConstBitEq` (bridges.rs:72) : "Const-callable bit-pattern equality."
  - `fn const_bit_eq(&self, other: &Self) -> Bool;` (bridges.rs:74) : "Const-callable bit-pattern equality. Always reflexive."
  - `fn const_bit_ne(&self, other: &Self) -> Bool` (bridges.rs:79) : "Const-callable bit-pattern inequality. Default body negates"
- `pub enum ConstOrdering` (bridges.rs:93), `#[derive(Copy, Clone, Debug, PartialEq, Eq)] #[repr(i8)]` (bridges.rs:91-92) : "Total-ordering result for `ConstOrd::const_cmp`."
  - `Less = -1,` (bridges.rs:95) : "`self < other`."
  - `Equal = 0,` (bridges.rs:97) : "`self == other`."
  - `Greater = 1,` (bridges.rs:99) : "`self > other`."
- `pub const trait ConstOrd: [const] ConstEq` (bridges.rs:135) : "Const-callable total ordering."
  - `fn const_cmp(&self, other: &Self) -> ConstOrdering;` (bridges.rs:137) : "Const-callable `cmp`."
  - `fn const_lt(&self, other: &Self) -> Bool` (bridges.rs:141) : "Const-callable `<`."
  - `fn const_le(&self, other: &Self) -> Bool` (bridges.rs:146) : "Const-callable `<=`."
  - `fn const_gt(&self, other: &Self) -> Bool` (bridges.rs:151) : "Const-callable `>`."
  - `fn const_ge(&self, other: &Self) -> Bool` (bridges.rs:156) : "Const-callable `>=`."
- `pub const trait ConstDefault: Sized` (bridges.rs:166) : "Const-callable default construction."
  - `fn const_default() -> Self;` (bridges.rs:168) : "Const-callable default value."
- Impls by macro: `impl_partial_eq_eq_ord_default_int!(u8, u16, u32, u64, u128, usize);` (bridges.rs:201) and `impl_partial_eq_eq_ord_default_int!(i8, i16, i32, i64, i128, isize);` (bridges.rs:202), each giving `ConstPartialEq`, `ConstEq`, `ConstBitEq`, `ConstOrd`, `ConstDefault`. `impl_partial_eq_bit_eq_default_float!(f32, f64);` (bridges.rs:273), giving `ConstPartialEq`, `ConstBitEq`, `ConstDefault`. `bool` has `ConstPartialEq`, `ConstEq`, `ConstBitEq`, `ConstOrd`, `ConstDefault` (bridges.rs:209-241). `From<core::cmp::Ordering> for ConstOrdering` (bridges.rs:102), `From<ConstOrdering> for core::cmp::Ordering` (bridges.rs:113).

src/meta_bits.rs:
- `pub struct MetaCarrier(pub u16);` (meta_bits.rs:50), `#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug, Default, Hash)] #[repr(transparent)]` (48-49) : "Layout-stable companion to `Bits<9, Hot, Unsigned>` used as the" (meta_bits.rs:31)
  - `pub const fn from_raw(n: u16) -> Self` (meta_bits.rs:60) : "Construct from the raw u16 carrier value."
  - `pub const fn to_raw(self) -> u16` (meta_bits.rs:66) : "Project to the raw u16 carrier value."
  - `pub const fn as_bits(self) -> Bits<9, Hot, Unsigned>` (meta_bits.rs:76) : "Zero-cost view as `Bits<9, Hot, Unsigned>`."
- Impls on `MetaCarrier`: `const impl Bounded` (85), `const impl<Op> Identity<Op>` (90), `ConstPartialEq` (97), `ConstEq` (106), `ConstBitEq` (108), `ConstOrd` (117), `ConstDefault` (126), `const unsafe impl Transparent` with `type Inner = u16` (53).
- `macro_rules! meta_bits_wrapper` (meta_bits.rs:161), not exported; invoked three times (meta_bits.rs:284, 294, 303). Each invocation `pub struct $W(pub MetaCarrier);` (170) with derives `ConstParamTy, PartialEq, Eq, Copy, Clone, Debug` and `#[repr(transparent)]` (168-169), and for each `$W`:
  - `pub const ZERO: Self` (174) : "Zero value."
  - `pub const ONE: Self` (176) : "One value."
  - `pub const fn raw(self) -> u16` (181) : "Read the underlying u16 carrier. See `arvo_transparent`"
  - `pub const fn as_bits(self) -> Bits<9, Hot, Unsigned>` (185) : "Zero-cost view as `Bits<9, Hot, Unsigned>`."
  - `$vis const fn $helper(n: u16) -> $W` (278) : "Const-fn helper for const-generic-position construction."
  - trait impls emitted: `Transparent` (`Inner = u16`), `Deref<Target = u16>`, `AsRef<u16>`, `Bounded`, `Identity<Op>`, `ConstPartialEq`, `ConstEq`, `ConstBitEq`, `ConstOrd`, `ConstDefault`, `From<u8>`, `From<u16>`, `From<$W> for u8`, `From<$W> for u16` (meta_bits.rs:193-274).
- `pub struct IBits;` with helper `pub const fn ibits(n: u16) -> IBits` (meta_bits.rs:290-291) : "Integer-bit count for fixed-point types."
- `pub struct FBits;` with helper `pub const fn fbits(n: u16) -> FBits` (meta_bits.rs:299-300) : "Fractional-bit count for fixed-point types."
- `pub struct Width;` with helper `pub const fn width(n: u16) -> Width` (meta_bits.rs:310-311) : "Bit-width meta value (1..=128)."
- `pub const W64: Width = width(64u16);` in `impl Width` (meta_bits.rs:316) : "64-bit width (FNV-1a-64 cap)."

src/platform.rs:
- `pub struct USize(pub usize);` (platform.rs:45), `#[derive(ConstParamTy, PartialEq, Eq, Copy, Clone, Debug)] #[repr(transparent)]` (43-44) : "Index / count newtype wrapping `usize`."
- `pub struct Cap(pub USize);` (platform.rs:73), same derives and repr (71-72) : "Const-generic capacity for fixed-size structures."
- `macro_rules! impl_unsigned_integer_newtype` (platform.rs:99), not exported; invoked `impl_unsigned_integer_newtype!(USize, usize);` (251) and `impl_unsigned_integer_newtype!(Cap, USize);` (252). Emits `Bounded`, `Identity<Op>`, `ConstPartialEq`, `ConstEq`, `ConstBitEq`, `ConstOrd`, `ConstDefault`, `Add`, `Sub`, `Mul`, `Div`, `Rem`, `Shl`, `Shr`, `BitAnd`, `BitOr`, `BitXor`, `Not` (each `const impl ... <$outer> for $outer`), and `PartialOrd`, `Ord`. Also `Deref<Target = usize> for USize` (53), `const unsafe impl Transparent for USize` (`Inner = usize`, 49) and for `Cap` (`Inner = USize`, 77), `const impl From<Cap> for USize` (369), `const impl From<USize> for Cap` (376).
- `pub struct Bool(pub bool);` (platform.rs:261), `#[derive(PartialEq, Eq, Copy, Clone, Debug)] #[repr(transparent)]` (259-260) : "Control-flow boolean."
  - `pub const TRUE: Bool = Bool(true);` (270) : "Constant `Bool(true)`."
  - `pub const FALSE: Bool = Bool(false);` (272) : "Constant `Bool(false)`."
  - Impls: `Deref<Target = bool>` (275), `Try` with `Output = bool`, `Residual = BoolResidual` (293), `FromResidual<BoolResidual>` (308), `const impl From<bool> for Bool` (335), `const impl From<Bool> for bool` (342), `const impl ConstPartialEq/ConstEq/ConstBitEq/ConstOrd/ConstDefault` (388-420), `const impl ConstTry` with `Output = bool`, `Residual = Infallible` (435), `const impl ConstFromResidual<Infallible>` (450), `const unsafe impl Transparent` (264).
- `pub enum BoolResidual {}` (platform.rs:291) : "Uninhabited residual for [`Bool`]'s infallible `Try`." with `impl Residual<bool> for BoolResidual { type TryType = Bool; }` (315)
- `pub const trait AsBool` (platform.rs:323) : "Bridge trait for code paths that need a raw `bool`."
  - `fn as_bool(&self) -> bool;` (325) : "Extract the inner `bool`."
  - `const impl AsBool for Bool` (328)
- `pub struct NUSize(Slot<NonZeroUsize>);` (platform.rs:485), `#[derive(Copy, Clone, Eq, PartialEq, Debug)] #[repr(transparent)]` (483-484) : "Niche-filled optional `USize`."
  - `pub const NONE: Self = Self(Slot::NONE);` (489) : "The absent value. Equivalent to `Slot::NONE`."
  - `pub const fn some(logical: USize) -> Self` (495) : "Construct from a logical 0-indexed `USize`. Internally shifts"
  - `pub const fn into_maybe(self) -> Maybe<USize>` (510) : "Extract as a logical `Maybe<USize>`. Inner -1 shift maps"
  - `pub const fn unwrap_or(self, default: USize) -> USize` (518) : "Logical value or the supplied default when absent."
  - `pub const fn is_some(&self) -> Bool` (526) : "`Bool::TRUE` when carrying a logical value."
  - `pub const fn is_none(&self) -> Bool` (531) : "`Bool::TRUE` when absent."
  - Impls: `Default` returning `Self::NONE` (536), `const unsafe impl Transparent` with `type Inner = Slot<NonZeroUsize>` (548).

src/layout_assertions.rs: no `pub` items (module is private, "The module is private (no `pub use`); assertions fire at compile" layout_assertions.rs:18). Private `macro_rules! assert_layout_eq` (39), private `const fn _assert_const_param_ty<T: core::marker::ConstParamTy_ + Copy + Eq>() {}` (263), `#[cfg(arvo_full_layout_audit)] mod full_audit` (286-287).

### Stated behaviour

Overflow, wrapping, saturating:
- "Overflow semantics match `usize`'s primitive contract: panic on debug-assertion builds, wrap on release." (DESIGN.md.tmpl:384-385, line-wrapped in source)
- "Construction can fail when the logical value is `usize::MAX` (the +1 would overflow); construction returns `NUSize::NONE` in that case." (platform.rs:478-480, line-wrapped in source)
- "`some` cannot fail in practice. The single overflow case is `NUSize::some(USize(usize::MAX))`, which folds into `NUSize::NONE` because the +1 shift would wrap." (DESIGN.md.tmpl:478-480, line-wrapped in source)
- "- `Precise` mirrors Warm's container shape; saturating arithmetic distinguishes the strategy at the op layer." (DESIGN.md.tmpl:143-144, line-wrapped in source)
- "// Hot = Wrapping / Min / Dense" (layout_assertions.rs:228); "// Warm = Wrapping / DoubleLogical / Dense" (233); "// Cold = Wrapping / Min / Bitpacked" (240); "// Precise = Saturating / DoubleLogical / Dense" (245)
- "Bit patterns are identities, not arithmetic values." (bits.rs:26-27, line-wrapped in source)
- "but no `Add` / `Sub` / `Mul` / `Div` / `Ord`." (bits.rs:25-26, line-wrapped in source)
- Rounding: no statement found in the files read.
- Debug checks or assertions: "Every assertion in this module is a compile-time invariant the type system cannot verify on its own:" (layout_assertions.rs:4-5, line-wrapped in source); "Mixed-arity ops (`USize + bare-usize`) deliberately do not compile." (DESIGN.md.tmpl:378)

Strategy markers:
- "`Hot`, `Warm`, `Cold`, `Precise` are zero-sized types declared in `arvo-strategy`." (DESIGN.md.tmpl:124-125, line-wrapped in source)
- "- `Hot` picks the narrowest container per bit width. 1..=8 → `u8`, 9..=16 → `u16`,..., 65..=128 → `u128`. Above 128 → `WideBits<bytes_for(N), A16>` (align(16) byte sequence)." (DESIGN.md.tmpl:133-136, line-wrapped in source)
- "- `Warm` mirrors Hot's container shape below 128. Above 128 → `WideBits<bytes_for(N), A1>` (align-1 byte sequence). The previous "2x logical width" Warm contract collapsed once the arvo passed single native primitives in width." (DESIGN.md.tmpl:137-139, line-wrapped in source)
- "- `Cold` mirrors Warm's per-value container shape. Bitpacking happens at the column-store layer above the per-Bits container, not at the per-value container shape." (DESIGN.md.tmpl:140-142, line-wrapped in source)
- "Container projections: u8 for 1..=8 under Hot, u16 for 9..=16, u32 for 17..=32, u64 for 33..=64, u128 for 65..=128 (Hot/Cold). Warm/Precise use the 2x-logical ladder (u16 for 1..=8, u32 for 9..=16, u64 for 17..=32, u128 for 33..=64) and have no native bucket above N=64." (bits.rs:12-16, line-wrapped in source)
- "Above the per-Strategy native cap, all four strategies project to `WideBits<bytes_for(N), A>` where `A = A16` for Hot (SSE2 / NEON aligned baseline) and `A = A1` for Cold / Warm / Precise (align-1 byte-exact)." (bits.rs:16-19, line-wrapped in source)
- "// Precise mirrors Warm's 2x-logical contract across the entire 1..=64" "// range." (layout_assertions.rs:142-143)
- "// Cold wide bucket projects to WideBits<bytes_for(N), A1>: align-1" "// byte-exact (Cold is bitpacked column-store; alignment on the" "// per-value container is not load-bearing)." (layout_assertions.rs:111-113)
- "// Warm = ..." and "// --- Tier 2: Bits projection: Warm (2x logical, caps at 64) ---" (layout_assertions.rs:123)
- "`WideBits<BYTES, A1>` is the align-1 baseline (Warm / Cold / Precise above 128); `WideBits<BYTES, A16>` is the SSE2 + NEON SIMD-friendly default (Hot above 128)." (DESIGN.md.tmpl:206-208, line-wrapped in source)

Named standards and external semantics:
- "The byte-ordering convention for `WideBits<BYTES, A>` matches `u128::trailing_zeros` / `u128::leading_zeros` semantics viewed as bit-significance order:" (DESIGN.md.tmpl:214-216, line-wrapped in source)
- "`A32` and `A64` (AVX-2 and AVX-512) plug in by adding markers, not container types, behind cfg gates per #320." (DESIGN.md.tmpl:208-210, line-wrapped in source)
- "BYTES ranges from 1 to 8191 (Width=u16 → max 65535 logical bits → max 8192 physical bytes; one byte slack accounts for the type system's unsigned-arithmetic bookkeeping in the projection const fns)." (DESIGN.md.tmpl:241-243, line-wrapped in source)
- "For N=4096 (RSA-4096 modulus), BYTES = 512. For SHA3-512 digest (N=512), BYTES = 64." (DESIGN.md.tmpl:245-246)
- "Floats deliberately do NOT implement ConstEq, ConstOrd. NaN breaks reflexivity (NaN != NaN under PartialEq), and total ordering on NaN-bearing types routes through arvo-numeric-contracts::TotalOrd." (bridges.rs:245-247, line-wrapped in source)
- "`ConstPartialEq` gives NaN-aware partial equality (`a.const_eq(&a)` is FALSE when `a` is NaN)." (bridges.rs:5-7, line-wrapped in source)
- "`ConstOrdering` (mirror of `core::cmp::Ordering`, locally defined because constructing `core::cmp::Ordering` values inside `impl const` bodies is not const-stable on rustc 1.96.0-nightly)." (bridges.rs:18-20, line-wrapped in source)
- "Reading the full struct as bytes via transmute or raw-pointer cast is **undefined behaviour**." (DESIGN.md.tmpl:264-265, line-wrapped in source: "reading the full struct as bytes via transmute or" / "raw-pointer cast is **undefined behaviour**.")
- "`?` desugars to core::ops::Try and stays unchanged" (platform.rs:430-431, "`?` desugars to core::ops::Try and" / "stays unchanged;"); "the `?` syntax itself stays non-const because rustc desugars `?` directly to `core::ops::Try::branch`, which is not `pub const trait` as of 2026-05." (DESIGN.md.tmpl:294-297, line-wrapped in source)

Consumer bounds and selection:
- "Everything else in the arvo stack (bit-level contracts, mask types, narrowing, fixed-point arithmetic, algorithms) lives upstream and consumes these primitives via trait bounds." (DESIGN.md.tmpl:16-19, line-wrapped in source)
- "New types here are added only when the type genuinely cannot be expressed without wrapping a bare primitive." (DESIGN.md.tmpl:19-20, line-wrapped in source)
- "`HasBitWidth` / `BitAccess` / `BitSequence` / `BitLogic` (declared in `arvo-bits-contracts`)" (bits.rs:24-26, line-wrapped in source)
- "`from_raw` remains the unchecked entry point; `from_narrowed` and `from_widened` are the recommended path when the source type does not exactly match the carrier projection." (DESIGN.md.tmpl:102-104, line-wrapped in source)
- "Consumer code prefers `.into()` over `Transparent::raw` or `cap_size` for the typed projection case" (DESIGN.md.tmpl:410-411, line-wrapped in source)
- "The strategy axis and Width: The parameter space accommodates 1..=65535 logical bits." (not a single verbatim line: the sentence is "The parameter" / "space accommodates 1..=65535 logical bits." DESIGN.md.tmpl:39-40)
- "`USize` implements the const unsafe `Transparent` trait." (DESIGN.md.tmpl:349)
- "Per D-7, every per-size helper is gone." (DESIGN.md.tmpl:68)
- "Bits is NOT a UFixed alias and NOT a UFixed wrapper." (bits.rs:21)
- "`UFixed<I, F, S>` wraps `Bits<{I+F}, S, Unsigned>` (default Sign omitted at the call site). `IFixed<I, F, S>` wraps `Bits<{1+I+F}, S, Signed>`." (bits.rs:33-34, line-wrapped in source)

Doc-versus-source facts, stated neutrally: DESIGN.md.tmpl:36-38 reads "The const-generic param is `Width` (the typed newtype around `u16`), relocated from this crate to `arvo-strategy`. `arvo-storage` re-exports it via `pub use arvo_strategy::Width;`"; src/lib.rs:34 reads `pub use meta_bits::{FBits, IBits, MetaCarrier, Width, fbits, ibits, width};` and src/meta_bits.rs:310 holds `pub struct Width;`. DESIGN.md.tmpl:58-63 lists `from_narrowed` and `from_widened`; src/bits.rs holds `from_raw` and `to_raw` only (bits.rs:86, 91). DESIGN.md.tmpl:446 reads "`pub struct NUSize(Slot<NonZeroUSize>);`"; src/platform.rs:485 reads `pub struct NUSize(Slot<NonZeroUsize>);`. DESIGN.md.tmpl:327 reads "`ConstOrd` is skipped for `Bool`"; src/platform.rs:410 holds `const impl ConstOrd for Bool`. DESIGN.md.tmpl:528-559 says `Width` previously lived here as a third meta-newtype and was relocated; src/meta_bits.rs:303-312 holds the `Width` wrapper invocation.

### Backlog and known gaps
BACKLOG.md.tmpl (no SHAME.md.tmpl in the batch list for this crate):
- `## ConstParamTy on UFixed / IFixed` (BACKLOG.md.tmpl:6): "Manual ConstParamTy_ impl on UFixed and IFixed lets fixed-point types appear directly as const generic arguments." (8-9)
- `## USize as Bits-rooted` (13): "Currently parked." (17)
- `## Cold strategy bitpacking` (20): "Out of scope for." (23)
- `## Round 3 SHIPPED (post-#316 work)` (25): "Round 3 (#313) of the foundations work shipped in round 202605031748." (27-28)
- `## #316 follow-ups (post-redesign)` (38): "The #316 foundational redesign deletes MultiContainer entirely and introduces WideBits + AlignedWideBits16." (40-41)
- `## Const-arith follow-ups` (47): "- ISize: signed platform-pointer-width companion to USize." (49)
- `## NU8 / NU16 / NU32 / NU64 family` (53): "Niche-packed Maybe-of-primitive companions to NUSize." (55)
- `## Removed entries` (63): "(none)" (65)

### Tests
- `#[test]` counts per src file (grep): src/bits.rs 0, src/bridges.rs 0, src/layout_assertions.rs 0, src/lib.rs 0, src/meta_bits.rs 0, src/platform.rs 0.
- Test files (grep counts, bodies not read): tests/bool_consttry.rs 1, tests/canonical_const_surface.rs 3, tests/from_bridges.rs 4, tests/metacarrier_const_arith.rs 1, tests/nusize_const_arith.rs 3, tests/usize_const_arith.rs 1.

## arvo-strategy   (files read: 2; partial crate in this batch, listed files are BACKLOG.md.tmpl and Cargo.toml only)

- purpose: not stated in the listed files (no README or DESIGN listed for this crate in this batch)
- depends on: `notko.workspace = true`, `arvo-transparent.workspace = true` (Cargo.toml:8-9). Features: `default = []` (Cargo.toml:12), no others.
- attributes: not in the listed files (no lib.rs listed)

### Public items
Not in the listed files (no source file listed for this crate in this batch).

### Stated behaviour
Not in the listed files beyond the backlog lines below.

### Backlog and known gaps
BACKLOG.md.tmpl (no SHAME.md.tmpl in the batch list for this crate):
- `## Tracked bridge expansions` (BACKLOG.md.tmpl:3): "Const-trait bridges deferred from (Round 1) to later rounds within PR #42:" (5)
- `## Mask-over-Bits collapse (shipped)` (14): "Round 3 landed on top of #316: Mask\<W\> over Bits + BitMatrix\<W, N\> + MaskOps trait impl + dogfood sweep (BitPrim::mask_low method, FastFloat / StrictFloat Identity blanket, Mask\<W\>::empty via Identity::ZERO)." (16-19, line-wrapped in source)
- `## Hot tier expansion (#320)` (26): "AVX-2 align(32) and AVX-512 align(64) Hot tiers are deferred to #320 with bench-driven evidence per the audit's H1 finding." (28-29, line-wrapped in source)
- `## Asm microkernels (#321)` (36): "Hot-path bit ops on WideBits / AlignedWideBits16 (count_ones, trailing/leading_zeros, bitwise) get asm-microkernel implementations when bench evidence shows compiler-generated SIMD doesn't hit the inlined-instruction-count target." (38-41, line-wrapped in source)
- `## HotTruncate research (#322)` (44): "A possible future Hot variant that explicitly truncates wide widths to 128 for compute (lossy but native-fast)." (46-47, line-wrapped in source)
- `## Removed entries` (50): "- "UContainerFor / IContainerFor extension to wider widths", both traits are deleted entirely in #316 per no-legacy-shims-pre-1.0." (52-53, line-wrapped in source)

### Tests
Not in the listed files.

<!-- END -->
