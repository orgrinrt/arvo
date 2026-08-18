// Probe H. Is the third carrier buildable at all, gate-free?
//
// Section 6 names three carriers for a region-level guarantee and says the third,
// a description of the region that lowers at the observation point, is the only
// one that can see its consumer. A canon may only state an intent it has
// established is doable, so this probe asks the doability question and nothing
// else.
//
// CONSTRAINTS TAKEN FROM I14 AND THE FORBIDDEN LIST
//   no_std, no alloc, no Box, no Vec
//   no dyn, no TypeId, no core::any
//   no generic_const_exprs, no specialization, no auto_traits
//   in fact: NO #![feature(...)] AT ALL. If it needs a gate, that is the finding.
//
// THE CASE THAT MUST FAIL
//   NC18 The narrowed evaluation must AGREE with the wide one on the congruence
//        operators and DISAGREE on at least one non-congruence operator. If it
//        agreed everywhere, the description would not actually be using the
//        consumer's demand and the probe would prove nothing.
//   NC19 Two observations of the SAME description at different demands must
//        produce different working widths. If the width did not move, the
//        backward information is not reaching the intermediate.
//   NC20 The forward width must be computable from the description alone, with
//        no observation. If it needed the sink, the description would not be a
//        description.

// ---------------------------------------------------------------------------
// The description. Every node is a compile-time type; the values it closes over
// are the only run-time part.
// ---------------------------------------------------------------------------

pub trait Expr: Copy {
    /// Width the forward rule alone assigns to this node. Computable with no
    /// observation, which is NC20.
    const FWD_W: u32;
    /// Does a demand of `d` low bits reduce to a demand of `d` on the operands?
    /// The partition probe B established empirically.
    const PASSES_DEMAND: bool;

    /// Evaluate under a demand of `D` low bits. The sink supplies `D`.
    fn eval<const D: u32>(self) -> i64;

    /// The width this node actually computes at, under a demand of `D`.
    fn work_w<const D: u32>(self) -> u32 {
        let d = if D < Self::FWD_W { D } else { Self::FWD_W };
        d
    }
}

const fn mask(w: u32) -> i64 {
    if w >= 63 {
        i64::MAX
    } else {
        (1i64 << w) - 1
    }
}

#[derive(Copy, Clone)]
pub struct Lit<const W: u32>(pub i64);

impl<const W: u32> Expr for Lit<W> {
    const FWD_W: u32 = W;
    const PASSES_DEMAND: bool = true;
    fn eval<const D: u32>(self) -> i64 {
        let w = if D < W { D } else { W };
        self.0 & mask(w)
    }
}

#[derive(Copy, Clone)]
pub struct Add<A, B>(pub A, pub B);

impl<A: Expr, B: Expr> Expr for Add<A, B> {
    const FWD_W: u32 = if A::FWD_W > B::FWD_W {
        A::FWD_W + 1
    } else {
        B::FWD_W + 1
    };
    const PASSES_DEMAND: bool = true;
    fn eval<const D: u32>(self) -> i64 {
        let w = if D < Self::FWD_W { D } else { Self::FWD_W };
        (self.0.eval::<D>().wrapping_add(self.1.eval::<D>())) & mask(w)
    }
}

#[derive(Copy, Clone)]
pub struct Mul<A, B>(pub A, pub B);

impl<A: Expr, B: Expr> Expr for Mul<A, B> {
    const FWD_W: u32 = A::FWD_W + B::FWD_W;
    const PASSES_DEMAND: bool = true;
    fn eval<const D: u32>(self) -> i64 {
        let w = if D < Self::FWD_W { D } else { Self::FWD_W };
        (self.0.eval::<D>().wrapping_mul(self.1.eval::<D>())) & mask(w)
    }
}

/// A non-congruence node: the demand stops here and the operand is evaluated at
/// its own full forward width.
#[derive(Copy, Clone)]
pub struct Shr<A, const K: u32>(pub A);

impl<A: Expr, const K: u32> Expr for Shr<A, K> {
    const FWD_W: u32 = if A::FWD_W > K { A::FWD_W - K } else { 1 };
    const PASSES_DEMAND: bool = false;
    fn eval<const D: u32>(self) -> i64 {
        let w = if D < Self::FWD_W { D } else { Self::FWD_W };
        // the operand is NOT narrowed to D: the demand does not pass.
        (self.0.eval::<{ 64 }>() >> K) & mask(w)
    }
}

/// The observation point. This is where the region ends and the consumer's
/// contract begins, and it is the only place the demand exists.
pub fn observe<const K: u32, E: Expr>(e: E) -> i64 {
    e.eval::<K>()
}
