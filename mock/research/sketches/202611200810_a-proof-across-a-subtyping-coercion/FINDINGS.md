# A proof carried across a subtyping coercion

**Outcome:** WORKS

The third review of the bit tier found that `Within<B, const I: u32>` held `PhantomData<B>` and so was
covariant in its container. This sketch asks whether that is a hole a caller can reach, and whether the
field `PhantomData<fn(B) -> B>` closes it, standing alone so it reads nothing of the crate.

`variance/src/lib.rs` declares a width trait, the two shapes of the proof, each made only by `NEW`,
which asserts the index below the width, and one container type `Wrap<T>` at two instances related by
subtyping: `Wrap<for<'a> fn(&'a ())>` declares 128 bits and `Wrap<fn(&'static ())>` declares 8, the
first a subtype of the second. The access function trusts the proof and checks nothing, as an
implementor of `BitAccessAt` does.

## The runs

`run.sh` builds and runs both examples under the repository's toolchain and keeps the first error line
or the run's last line, and the exit status, in `outputs/variance.txt`.

`a_covariant_proof` makes a proof of index 100 for the 128-bit container and binds it as a proof for
the 8-bit one. It builds, and at run time indexes the one byte the narrow container holds at byte 12:
exit 101, a panic at `src/lib.rs:52`, the slice index past the storage.

`an_invariant_proof` is the same carry with the invariant field. It does not build: `E0308`, mismatched
types, at the binding.

So the covariant proof is a hole any caller with two such containers can reach, and the invariant field
closes it at build. The tier's `Within` takes the invariant field, and the ui case
`a_proof_carried_to_a_narrower_container_is_refused` pins the mismatch against the crate itself.

## What this does not establish

Nothing about a container whose declared width disagrees with its storage: that is the implementor's
to keep, and no shape of the proof can see it.
