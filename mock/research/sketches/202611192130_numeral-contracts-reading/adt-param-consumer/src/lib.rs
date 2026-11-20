//! Question: can a consumer that names no gate write a bound and a call over a
//! const parameter of the stack's own type, so the bare `i128` the shape puts
//! at `Constant<const V: i128>` is not the only position the obligation
//! leaves open? Op's bound on the exception reads "only when truly painful
//! otherwise", so this decides whether it is.

use adt_param_lib::{Constant, Index, Small};

pub fn three<T: Constant<{ Index::at(3) }>>() -> T {
    T::MEMBER
}

pub const THREE: Small = <Small as Constant<{ Index::at(3) }>>::MEMBER;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_member_is_the_one_named() {
        assert_eq!(three::<Small>(), Small(Index::at(3)));
        assert_eq!(THREE.0.get(), 3);
    }
}
