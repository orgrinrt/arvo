// ARM 6, must BUILD on every target, and its mutant beside it must fail on every
// target.
//
// The question this answers is Q31's third option: whether "arvo can carry it"
// has to be scoped to a compilation target, which would buy a quantifier over
// compilations. If the admitted set moved with the target, one of these
// assertions would fail on one of them.
//
// `const _` items rather than a `main`, so the obligations are forced at check
// time and this can be run for a bare-metal target with no runtime.
#![no_std]

use arvo_format::slots::{is_admissible, Signed, Slots, Unsigned, ADMITTED_WIDTHS};
use q31_probes::Grid;

// The boundary, both sides of it.
const _: () = assert!(is_admissible::<Grid<-2305843009213693952, 2305843009213693951, 62>>().get());
const _: () = assert!(!is_admissible::<Grid<-4611686018427387904, 4611686018427387903, 63>>().get());

// The exact-product range from arm 1, refused here too and on every target.
const _: () = assert!(!is_admissible::<Grid<-4611686016279904256, 4611686018427387904, 63>>().get());

// The whole shipped set, counted rather than sampled.
const _: () = assert!(ADMITTED_WIDTHS.len() == 62);
const _: () = assert!(ADMITTED_WIDTHS[0].count() == 1);
const _: () = assert!(ADMITTED_WIDTHS[61].count() == 62);

// The widest and narrowest shipped impls, both signs.
const _: () = assert!(<Signed<62> as Slots>::WIDTH.count() == 62);
const _: () = assert!(<Unsigned<62> as Slots>::WIDTH.count() == 62);
const _: () = assert!(<Signed<1> as Slots>::WIDTH.count() == 1);

// The exact endpoints of the widest admitted grid, so a target that changed the
// arithmetic underneath would be caught rather than only a target that changed
// the bound.
const _: () = assert!(<Signed<62> as Slots>::MIN.index() == -2305843009213693952);
const _: () = assert!(<Signed<62> as Slots>::MAX.index() == 2305843009213693951);
const _: () = assert!(<Unsigned<62> as Slots>::MAX.index() == 4611686018427387903);
