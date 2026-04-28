//! Smoke tests for the `Enumerator` blanket impl.

use arvo::USize;
use arvo_tensor::Enumerator;

#[test]
fn enumerates_slice_with_typed_usize() {
    let xs = [10u32, 20, 30];
    let mut first_key = USize(99);
    let mut total: u32 = 0;
    for (i, t) in xs.iter().enumerated() {
        if total == 0 {
            first_key = i;
        }
        total += *t;
    }
    assert_eq!(first_key.0, 0);
    assert_eq!(total, 60);
}

#[test]
fn enumerates_produces_ascending_indices() {
    let xs = [1u32, 2, 3, 4];
    let mut seen: [usize; 4] = [99; 4];
    for (i, t) in xs.iter().enumerated() {
        seen[(*t - 1) as usize] = i.0;
    }
    assert_eq!(seen, [0, 1, 2, 3]);
}
