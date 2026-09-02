// ARM 7, the mutant, must FAIL on every target.
//
// Identical to arm 6 except that it claims the 63-bit grid is admitted. If this
// built anywhere, arm 6's passing would mean nothing, because the instrument
// would be incapable of reporting a difference.
#![no_std]

use arvo_format::slots::is_admissible;
use q31_probes::Grid;

const _: () = assert!(is_admissible::<Grid<-4611686018427387904, 4611686018427387903, 63>>().get());
