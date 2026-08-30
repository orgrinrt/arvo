#[derive(Clone, Copy)]
pub struct Bool(pub bool);
pub struct Budget;
pub struct Parity;

// Attempt: a const fn that varies by predicate type, the way an associated const does.
const fn is_monotone<P>() -> bool {
    // there is no way to inspect P here; const fns cannot dispatch on a type
    true
}

#[inline(always)]
fn bisect(v: &[u32]) -> usize {
    v.len()
}
#[inline(always)]
fn scan(v: &[u32]) -> usize {
    v.len()
}

pub fn select<P>(v: &[u32]) -> usize {
    if is_monotone::<P>() {
        bisect(v)
    } else {
        scan(v)
    }
}

pub fn demo() {
    // Both take the SAME branch, because the const fn cannot see P.
    println!(
        "{} {}",
        select::<Budget>(&[1, 2]),
        select::<Parity>(&[1, 2])
    );
}
