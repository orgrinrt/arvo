// PROBE A (section 1): what a real arvo consumer reads today.
//
// Needs path deps on arvo, arvo-storage, arvo-strategy, notko. Three cases, run
// separately because rustc stops at the first hard error in some of them.
//
// The question: 04 verified that rustc expands type aliases in diagnostics and
// concluded the spec's alias story destroys the error surface. arvo's dominant
// consumer spelling is ALREADY an alias (`Uint<N, S>`, 27 sites in hilavitkutin
// against 9 for the raw `UFixed` form), so the damage should already be visible.
// It is, and it is worse than expansion.

// ---- A1: the dominant consumer alias, without the GCE gate ----------------
//
//   use arvo::strategy::{Hot, SignedIdentity};
//   pub fn b() { let _ = <arvo::Uint<12, Hot> as SignedIdentity>::NEG_ONE; }
//
// RESULT, verbatim:
//
//   error[E0277]: the trait bound `UFixed<IBits(MetaCarrier(12)),
//                 FBits(MetaCarrier(0)), ...>: SignedIdentity` is not satisfied
//     |
//   4 |     let _ = <arvo::Uint<12, Hot> as SignedIdentity>::NEG_ONE;
//     |              ^^^^^^^^^^^^^^^^^^^ the trait `SignedIdentity` is not
//     |              implemented for `UFixed<IBits(MetaCarrier(12)),
//     |              FBits(MetaCarrier(0)), Hot>`
//     = note: the full name for the type has been written to '...long-type-....txt'
//
// Note the `...` truncation in the message line and the long-type spill, at
// THREE type parameters, today.

// ---- A2: the same, with `#![feature(generic_const_exprs)]` on the consumer --
//
// Which is the environment arvo's real consumer is in: hilavitkutin/src/lib.rs:24
// carries that gate, as do six of its test files.
//
// RESULT, verbatim:
//
//   error[E0277]: the trait bound `UFixed<arvo::::aliases::Uint::{constant#0},
//                 arvo::::aliases::Uint::{constant#1}, ...>: SignedIdentity`
//                 is not satisfied
//
// The consumer wrote 12 and the compiler cannot tell them the width. The const
// arguments render as anonymous const-item paths, with a doubled `::`.

// ---- A3: does the on_unimplemented note survive alias expansion? -----------
//
//   use arvo::strategy::{Hot, Identity, Multiplicative};
//   pub fn a() { let _ = <arvo::Fixed<0, 8, Hot> as Identity<Multiplicative>>::IDENTITY; }
//
// RESULT: it does, at the primary span.
//
//   error[E0277]: this type has no multiplicative identity
//     |
//   4 |     let _ = <arvo::Fixed<0, 8, Hot> as Identity<Multiplicative>>::IDENTITY;
//     |              ^^^^^^^^^^^^^^^^^^^^^^ the trait `OneRepresentable<1>` is
//     |              not implemented for `Picker`
//     = note: A purely fractional fixed-point type has zero integer bits, ...
//     = note: required for `UFixed<IBits(MetaCarrier(0)), FBits(MetaCarrier(8)),
//             Hot>` to implement `Identity<Multiplicative>`
//     = note: the full name for the type has been written to '...'
//
// So the alias survives where the consumer looks (the span and the caret) and is
// expanded in the trailing normalisation note. Which of the two a consumer gets
// depends on whether the failing obligation's trait carries
// `#[diagnostic::on_unimplemented]`: with the attribute the message is the
// authored text and the expansion is demoted to a note; without it the message
// line IS the expanded bound. 04's probe used a bare trait and therefore
// measured the second case only.
