// The control for `tests/the_proof.rs`: the container it asserts the proof
// against is not `Send`, so the proof being `Send` there says something.

use core::marker::PhantomData;

use arvo_bits::Bits;

#[allow(dead_code)]
struct Pinned(Bits<8>, PhantomData<*const ()>);

const fn send<T: Send>() {}

const _: () = send::<Pinned>();

fn main() {}
