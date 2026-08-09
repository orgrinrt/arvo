// P7. Granularity of the shipped attribute, established by putting it on things
// other than a free fn rather than by reading `parse2::<ItemFn>`.
// Uncomment one case at a time; each is recorded in output.txt.
use notko_macros::profile;

#[derive(Debug)]
pub struct E;

#[profile(Hot)]
pub fn ok_free_fn(x: u32) -> Result<u32, E> {
    if x > 10 {
        return Err(E);
    }
    Ok(x * 2)
}

pub struct S;

// CASE_IMPL
// #[profile(Hot)]
// impl S { pub fn m(&self, x: u32) -> Result<u32, E> { Ok(x) } }

// CASE_MOD
// #[profile(Hot)]
// pub mod inner { pub fn g(x: u32) -> Result<u32, super::E> { Ok(x) } }

// CASE_TRAIT_METHOD
// pub trait T { fn t(&self, x: u32) -> Result<u32, E>; }
// impl T for S { #[profile(Hot)] fn t(&self, x: u32) -> Result<u32, E> { Ok(x) } }

// CASE_UNKNOWN_TIER
// #[profile(Precise)]
// pub fn precise_fn(x: u32) -> Result<u32, E> { Ok(x) }

fn main() {
    println!("{:?}", ok_free_fn(3));
}
