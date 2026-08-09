// probe 1: the far point is definable as a TOTAL projection over the whole
// four-member Specials product, with no HasInfinity gate and no refusal
// anywhere. NaN never participates because it is not in the order, and that
// falls out of the definition rather than needing a special case.
//
// hypothesis: "the far point of a numeral on the overflow side is the
// supremum of its ORDERED representable values" is expressible as a
// const-callable, type-level fact for all four Specials members, under the
// permitted feature set (no gates at all, in fact), and it distinguishes the
// two kinds the grade needs: an absorbing far point (infinity, self-
// witnessing in the datum) and a finite far point (saturation, silent in the
// datum).
//
// expected: compiles clean, all four const assertions hold, no feature line.

// the four-member Specials product, as the review carries it (68:578).
pub struct NoSpecials;
pub struct NanOnly;
pub struct InfOnly;
pub struct IeeeSpecials;

pub trait Specials {
    const HAS_INF: bool;
    const HAS_NAN: bool;
}
impl Specials for NoSpecials {
    const HAS_INF: bool = false;
    const HAS_NAN: bool = false;
}
impl Specials for NanOnly {
    const HAS_INF: bool = false;
    const HAS_NAN: bool = true;
}
impl Specials for InfOnly {
    const HAS_INF: bool = true;
    const HAS_NAN: bool = false;
}
impl Specials for IeeeSpecials {
    const HAS_INF: bool = true;
    const HAS_NAN: bool = true;
}

// the kind of the far point. Absorbing: the far point is an infinity, and a
// value that reached it witnesses the overflow in-band forever. Finite: the
// far point is the largest finite magnitude, and reaching it is silent in
// the datum.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FarPointKind {
    Absorbing,
    Finite,
}

// the projection. TOTAL: one blanket impl, no bound beyond Specials itself,
// so there is no member of the product it refuses. NaN plays no role in the
// value of KIND because NaN is not in the order; only HAS_INF is consulted.
pub trait FarPoint: Specials {
    const KIND: FarPointKind = if Self::HAS_INF {
        FarPointKind::Absorbing
    } else {
        FarPointKind::Finite
    };
}
impl<S: Specials> FarPoint for S {}

// const-callable form, as the dispatch's constraints require.
pub const fn far_point_kind<S: FarPoint>() -> FarPointKind {
    S::KIND
}

// the whole product, not a sample. compile-time, no runtime assertion.
const _: () = assert!(matches!(
    far_point_kind::<NoSpecials>(),
    FarPointKind::Finite
));
const _: () = assert!(matches!(far_point_kind::<NanOnly>(), FarPointKind::Finite));
const _: () = assert!(matches!(
    far_point_kind::<InfOnly>(),
    FarPointKind::Absorbing
));
const _: () = assert!(matches!(
    far_point_kind::<IeeeSpecials>(),
    FarPointKind::Absorbing
));

// NanOnly and NoSpecials agree on the far point even though they differ on
// HAS_NAN: the order-theoretic definition never consults the NaN axis.
const _: () = assert!(matches!(
    (far_point_kind::<NanOnly>(), far_point_kind::<NoSpecials>()),
    (FarPointKind::Finite, FarPointKind::Finite)
));

fn main() {
    println!("NoSpecials   -> {:?}", far_point_kind::<NoSpecials>());
    println!("NanOnly      -> {:?}", far_point_kind::<NanOnly>());
    println!("InfOnly      -> {:?}", far_point_kind::<InfOnly>());
    println!("IeeeSpecials -> {:?}", far_point_kind::<IeeeSpecials>());
    println!("total over the product, zero refusals, zero feature gates");
}
