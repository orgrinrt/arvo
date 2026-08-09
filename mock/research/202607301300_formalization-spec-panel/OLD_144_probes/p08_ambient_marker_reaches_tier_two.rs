// P8. The wall: an injected scope selection cannot reach a module-scope domain
// alias (p02 case D), and a module-scope domain alias is op's tier two, the
// wholesale-adoption path. So a scope selection does nothing for the tier where
// most arvo code will live.
//
// Attack: make the ELISION itself a marker. `UInt<5>` with no posture written
// means `Num<5, Ambient>`, one type everywhere, so the alias keeps one identity
// and nothing breaks at a function boundary. The posture is resolved AT THE
// OPERATION, by a type-level function that maps Ambient to the scope's posture
// and leaves any DECLARED posture alone. Precedence stops being a macro
// heuristic and becomes a total function on types.
#![allow(dead_code)]

#[derive(Copy, Clone, Debug)]
pub struct Ambient;
#[derive(Copy, Clone, Debug)]
pub struct Hot;
#[derive(Copy, Clone, Debug)]
pub struct Warm;
#[derive(Copy, Clone, Debug)]
pub struct Precise;

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct Num<const N: u8, S>(u32, core::marker::PhantomData<S>);
impl<const N: u8, S> Num<N, S> {
    #[inline]
    pub const fn new(v: u32) -> Self {
        Num(v, core::marker::PhantomData)
    }
    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }
}

/// Precedence, as a total function on the four markers plus the elision.
/// `Ambient` yields to the scope; anything declared wins over the scope.
pub trait Resolve<P> {
    type Out: Posture;
}
impl<P: Posture> Resolve<P> for Ambient {
    type Out = P;
}
impl<P> Resolve<P> for Hot {
    type Out = Hot;
}
impl<P> Resolve<P> for Warm {
    type Out = Warm;
}
impl<P> Resolve<P> for Precise {
    type Out = Precise;
}

pub enum Refused {
    OverRange,
}

/// The posture, as the result shape of an operation performed under it.
pub trait Posture {
    type Of<T>;
    fn go(a: u32, b: u32) -> Self::Of<u32>;
}
impl Posture for Hot {
    type Of<T> = T;
    #[inline]
    fn go(a: u32, b: u32) -> u32 {
        a.wrapping_add(b)
    }
}
impl Posture for Warm {
    type Of<T> = T;
    #[inline]
    fn go(a: u32, b: u32) -> u32 {
        a.wrapping_add(b)
    }
}
impl Posture for Precise {
    type Of<T> = Result<T, Refused>;
    #[inline]
    fn go(a: u32, b: u32) -> Result<u32, Refused> {
        a.checked_add(b).ok_or(Refused::OverRange)
    }
}

/// The operation entry point the macro rewrites `a + b` into, inside a scope
/// annotated with posture `P`. The operands keep their own type.
pub type Chosen<S, P> = <S as Resolve<P>>::Out;
#[inline]
pub fn add<P: Posture, const N: u8, S: Resolve<P>>(
    a: Num<N, S>,
    b: Num<N, S>,
) -> <Chosen<S, P> as Posture>::Of<Num<N, S>>
where
    <Chosen<S, P> as Posture>::Of<u32>: IntoNum<N, S, Chosen<S, P>>,
{
    <Chosen<S, P> as Posture>::go(a.raw(), b.raw()).into_num()
}

/// Re-wrap whatever shape the posture produced, without naming the shape twice.
pub trait IntoNum<const N: u8, S, P: Posture> {
    fn into_num(self) -> <P as Posture>::Of<Num<N, S>>;
}
impl<const N: u8, S, P: Posture<Of<u32> = u32, Of<Num<N, S>> = Num<N, S>>> IntoNum<N, S, P>
    for u32
{
    #[inline]
    fn into_num(self) -> Num<N, S> {
        Num::new(self)
    }
}
impl<const N: u8, S, P> IntoNum<N, S, P> for Result<u32, Refused>
where
    P: Posture<Of<u32> = Result<u32, Refused>, Of<Num<N, S>> = Result<Num<N, S>, Refused>>,
{
    #[inline]
    fn into_num(self) -> Result<Num<N, S>, Refused> {
        self.map(Num::new)
    }
}

// ---- tier two: the domain alias, no posture written, ONE type everywhere ----
pub type UInt<const N: u8> = Num<N, Ambient>;
pub type StrHandle = UInt<5>;

// ---- tier three: an explicit declaration, at the alias definition ----------
pub type Checked = Num<5, Precise>;

// a plain scope: the elision resolves to the ambient posture the scope names
fn plain(a: StrHandle, b: StrHandle) -> StrHandle {
    add::<Warm, 5, Ambient>(a, b)
}
// what the macro emits under `#[profile(Hot)]`: SAME operand type, different op
fn hot_scope(a: StrHandle, b: StrHandle) -> StrHandle {
    add::<Hot, 5, Ambient>(a, b)
}
// the same scope over an explicitly declared operand: the declaration wins, and
// the result shape follows the DECLARATION, not the scope
fn hot_scope_over_declared(a: Checked, b: Checked) -> Result<Checked, Refused> {
    add::<Hot, 5, Precise>(a, b)
}

fn main() {
    let a = StrHandle::new(3);
    let b = StrHandle::new(4);
    println!("plain      = {}", plain(a, b).raw());
    println!("hot scope  = {}", hot_scope(a, b).raw());
    let c = Checked::new(u32::MAX);
    let d = Checked::new(1);
    println!(
        "declared under a Hot scope refuses = {}",
        hot_scope_over_declared(c, d).is_err()
    );
    println!(
        "alias identity unchanged: {}",
        core::any::type_name::<StrHandle>()
    );
}
