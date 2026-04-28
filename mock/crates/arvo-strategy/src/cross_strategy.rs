//! Cross-strategy operation diagnostic surface.
//!
//! `Resolve<S1, S2>::Out` projects the more conservative of two
//! strategies whenever a cross-strategy op composes. The projection
//! is silent: a `Hot + Wrapping` value composed with a
//! `Precise + Saturating` value resolves to the `Precise + Saturating`
//! result without surfacing the semantic shift to the consumer.
//! Per the `arvo-toolbox-not-policer` workspace rule, the substrate
//! cannot refuse the op (the consumer's choice stands), but it can
//! warn at compile time when the resolution shifts overflow policy,
//! container width, or storage layout.
//!
//! `CrossStrategyOp<S1, S2>` is the marker trait carrying the
//! `#[diagnostic::on_unimplemented]` attribute. It is implemented
//! only for `(S, S)` same-strategy pairs. When a consumer composes a
//! cross-strategy op, code that bounds on
//! `where (S1, S2): CrossStrategyOp<S1, S2>` produces a compile-time
//! warning naming the shift; consumer opt-out goes through an
//! explicit `lint:allow(cross-strategy-resolution)` annotation or a
//! type-cast at the call site.
//!
//! This file ships the trait surface and the same-strategy impls.
//! Wiring the bound into every cross-strategy arithmetic op site is
//! tracked as a follow-up forward-looking entry under
//! Round 202604281000 in `arvo/BACKLOG.md.tmpl`. Until that wiring
//! lands, consumers can adopt the bound explicitly at their own
//! cross-strategy op sites.

use crate::{Cold, Hot, Precise, Strategy, Warm};

/// Marker trait that fires a compile-time diagnostic at cross-strategy
/// op sites.
///
/// Implemented for same-strategy pairs (`(Hot, Hot)`, `(Warm, Warm)`,
/// etc.). Cross-strategy pairs lack the impl; bounding on
/// `CrossStrategyOp<S1, S2>` at op sites surfaces the missing-impl
/// diagnostic.
///
/// The diagnostic message names the resolved-strategy semantics and
/// points the consumer at the explicit-cast escape hatch. Per
/// `arvo-toolbox-not-policer`, the substrate warns but never refuses;
/// silencing the warning is a one-line `lint:allow` annotation or an
/// explicit `.cast::<S>()` at the call site.
#[diagnostic::on_unimplemented(
    message = "cross-strategy operation between `{S1}` and `{S2}` shifts arithmetic semantics",
    label = "this op resolves to `Resolve<{S1}, {S2}>::Out` and may adopt different overflow / container / layout semantics than either operand",
    note = "consumer choice: insert an explicit `.cast::<S>()` to make the strategy shift visible at the call site, or annotate the surrounding scope with `// lint:allow(cross-strategy-resolution)` to silence the warning",
)]
pub trait CrossStrategyOp<S1: Strategy, S2: Strategy> {}

impl CrossStrategyOp<Hot, Hot> for () {}
impl CrossStrategyOp<Warm, Warm> for () {}
impl CrossStrategyOp<Cold, Cold> for () {}
impl CrossStrategyOp<Precise, Precise> for () {}
