// PROBE F (section 8): what a fallible call site costs, decided by the orphan
// rule rather than by taste. Needs a path dep on notko; nothing else.
//
// The spec puts the fallible return in `type Fallibility<T>: notko::ConstTry`
// (spec:156), with `Just<T>` for total rules and `Outcome<T, _>` where `Refuse`
// appears, and prices it at "call sites unwrap" (spec:269-271). The question my
// lens asks is how MANY times, and the answer is not a design choice.
//
// If a consumer writes `let t = a + b + c;` on a `Precise` composition, then
// `a + b` is an `Outcome` and `Outcome + c` needs `impl Add for Outcome<..>`.
// arvo does not own `Outcome`. Body under test:
//
//   pub struct MyNum(pub u16);        // stands in for an arvo primitive
//   pub struct OutOfRange;
//   impl Add for Outcome<MyNum, OutOfRange> { ... }
//
// RESULT, verbatim:
//
//   error[E0117]: only traits defined in the current crate can be implemented
//                 for types defined outside of the crate
//     --> src/lib.rs:14:1
//      |
//   14 | impl Add for Outcome<MyNum, OutOfRange> {
//      | ^^^^^---^^^^^--------------------------
//      |      |       |
//      |      |       `Outcome` is not defined in the current crate
//      |      `Outcome` is not defined in the current crate
//      |
//      = note: impl doesn't have any local type before any uncovered type
//              parameters
//
// So under a foreign-carrier delivery there is one unwrap PER OPERATION, not one
// per expression, and arvo has no way to change that. Three-term accumulation is
// three `?` or three matches. That is a per-call-site cost on the preset whose
// stated intent is care, and it is settled by the orphan rule rather than by the
// spec's choice.
//
// The escape is an arvo-owned carrier, which can implement `Add` and short
// circuit, so the chain is written once and settled once. That is the same
// conclusion 05 reaches from layout (`c_layout.rs`, the doubling) and from
// codegen (`d_delivery_codegen.rs`, the per-element exits), reached here from
// coherence, which is a third independent road to it.
