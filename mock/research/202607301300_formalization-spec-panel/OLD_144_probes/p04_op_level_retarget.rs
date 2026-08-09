// P4. Retargeting the OPERATION rather than the TYPE.
// The alias case (P2 case D) is immune to an injected `use`, and the alias case
// is op's tier two. So try the other half: leave the stored type alone and
// retarget the arithmetic the annotated scope performs on it.
//
// Shape under test: `a + b` in the body is rewritten to `ops::add::<P, _>(a, b)`
// where P is the posture the attribute names, and the RESULT SHAPE is projected
// from P, so a refusing posture returns a fallible value and an infallible one
// does not. That is one decision driving both the numeric policy and the
// fallibility, which is the thing notko's tier axis and arvo's strategy axis
// have in common.
#![allow(dead_code)]

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Hot;
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Warm;
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Precise;

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct Num<const N: u8, S>(u32, core::marker::PhantomData<S>);

impl<const N: u8, S> Num<N, S> {
    pub const fn new(v: u32) -> Self {
        Num(v, core::marker::PhantomData)
    }
    pub const fn raw(self) -> u32 {
        self.0
    }
}

pub enum Refused {
    OverRange,
}

/// The posture, as a type-level function from the operand type to the result
/// shape of an operation performed under it.
pub trait Posture {
    type Of<T>;
    fn wrap<T>(v: T) -> Self::Of<T>;
}
impl Posture for Hot {
    type Of<T> = T;
    fn wrap<T>(v: T) -> T {
        v
    }
}
impl Posture for Warm {
    type Of<T> = T;
    fn wrap<T>(v: T) -> T {
        v
    }
}
impl Posture for Precise {
    type Of<T> = Result<T, Refused>;
    fn wrap<T>(v: T) -> Result<T, Refused> {
        Ok(v)
    }
}

/// One operation entry point, parameterised on the posture, generic over the
/// stored type. The stored type keeps whatever posture its ALIAS declared; only
/// the arithmetic is retargeted.
pub fn add<P: Posture, const N: u8, S>(a: Num<N, S>, b: Num<N, S>) -> P::Of<Num<N, S>> {
    P::wrap(Num::new(a.raw().wrapping_add(b.raw())))
}

// The tier-two consumer's alias, module scope, no posture written, immune to
// any injected `use` (established in p02 case D).
pub type StrHandle = Num<5, Warm>;

fn ambient(a: StrHandle, b: StrHandle) -> StrHandle {
    add::<Warm, 5, Warm>(a, b)
}

// what the macro would emit inside `#[profile(Hot)]`
fn hot_scope(a: StrHandle, b: StrHandle) -> StrHandle {
    add::<Hot, 5, Warm>(a, b)
}

// what the macro would emit inside `#[profile(Precise)]`: the result shape
// changed, so the signature changed with it, which is the notko rewrite's job.
fn precise_scope(a: StrHandle, b: StrHandle) -> Result<StrHandle, Refused> {
    add::<Precise, 5, Warm>(a, b)
}

fn main() {
    let a = StrHandle::new(3);
    let b = StrHandle::new(4);
    println!("ambient  = {}", ambient(a, b).raw());
    println!("hot      = {}", hot_scope(a, b).raw());
    println!(
        "precise  = {}",
        precise_scope(a, b).map(|v| v.raw()).unwrap_or(0)
    );
    // the stored type is unchanged in every arm: still Num<5, Warm>
    println!("stored   = {}", core::any::type_name::<StrHandle>());
}
