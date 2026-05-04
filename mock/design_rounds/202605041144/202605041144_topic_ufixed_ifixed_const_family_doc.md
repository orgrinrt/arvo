**Date:** 2026-05-04
**Phase:** TOPIC
**Scope:** arvo (DESIGN.md.tmpl)
**Source topics:** Round 7 (#325) PR #50 reviewer NIT 2; new task #326

# Round 8 Topic 1: const-callable trait family subsection for UFixed and IFixed

This topic adds a dedicated subsection to `mock/crates/arvo/DESIGN.md.tmpl` enumerating the canonical const-callable trait family that UFixed and IFixed impl via the predicate-bundling pattern. Round 7 closed the Bounded gap on impl side; the doc surface is the remaining piece.

The subsection lives under "## Fixed-point types", after "### Const arithmetic surface (round 202605021200)". It mirrors the framing already used by the float side at "### Float-side const surface (round 202605021800, relocated 202605030400)" (lines 1032-1090). Same shape: enumerate the trait family with code samples, document the predicate-bundling rationale, point to the inner Bits forward, name what is intentionally omitted.

Doc-only. No source change. Closes Round 7 reviewer NIT 2.

## Decisions

### Decision 1: Subsection placement

Insert "### Const-callable trait family (rounds 202605021800, 202605041128)" between line 751 (end of "Const arithmetic surface") and line 753 (start of "## Strategy markers"). The two round IDs reflect the multi-round provenance: 202605021800 introduced Identity / ConstPartialEq / ConstEq / ConstBitEq / ConstOrd / ConstDefault blanket forwards on UFixed and IFixed; 202605041128 (#325) added Bounded.

### Decision 2: Subsection content

Three structured blocks:

1. **The trait family.** Bullet list naming each trait the predicate-bundling blanket lands for, with a one-line semantic anchor each. Identity (ZERO / ONE), Bounded (MIN / MAX), ConstPartialEq + ConstEq (value equality at const time), ConstBitEq (bit-pattern equality), ConstOrd (total ordering at const time), ConstDefault (typed default).

2. **The predicate-bundling shape.** One code-sample showing the Bounded blanket on UFixed (the latest addition, mirrors all earlier ones). Comment lines name the cycle-avoidance reason: bound on the inner Bits trait projection bundles the container requirement, sidestepping the generic_const_exprs cycle that two-predicate forms tripped.

3. **What is omitted on UFixed and IFixed.** One short paragraph naming SignedIdentity::MINUS_ONE on IFixed only (signed counterpart, separate from Identity). Bounded MIN / MAX semantics inherited as container bounds (not logical bit-width bounds); whether logical-width bounds should ship as a separate trait is BACKLOG, not Round 8 scope.

### Decision 3: BACKLOG entry for logical-width bounds

Add one BACKLOG entry under `mock/crates/arvo/BACKLOG.md.tmpl` capturing the logical-width-bounds question Round 7 surfaced: today `<U16Warm as Bounded>::MAX` returns u32::MAX (the container bound) because the Warm 16-bit container is u32. A `LogicalBounds` trait or similar surface returning the logical-width bound would be a separate substrate decision; it is not Round 8 scope but worth tracking so it does not get lost.

## Sketches needed

None. The doc subsection mirrors the float-side template that already lives in DESIGN.md.tmpl (lines 1032-1090). No design question; only documentation.

## Lock criterion

This topic locks (frozen) when:

1. Decision 1 names the exact insertion point.
2. Decision 2 enumerates the three blocks the subsection contains.
3. Decision 3 captures the BACKLOG entry shape.
4. The doc CL is opened referencing this topic.

## Out of scope

- Source change. Round 7 already shipped Bounded; Round 8 only adds the doc subsection.
- LogicalBounds trait. Captured as BACKLOG only.
- Other DESIGN.md.tmpl subsection reorganization. Kept minimal so the round stays focused.

## Cross-references

- `mock/crates/arvo/DESIGN.md.tmpl:1032-1090` (float-side template the new subsection mirrors).
- `mock/crates/arvo/DESIGN.md.tmpl:717-751` (Const arithmetic surface; insertion point sibling).
- `mock/crates/arvo/src/ufixed.rs:60-90` (Identity + Bounded blankets; pattern reference).
- `mock/crates/arvo/src/ifixed.rs:62-92` (Identity + Bounded blankets; pattern reference for signed).
- `.claude/rules/cl-claim-sketch-discipline.md` (no sketches needed; doc-only).
- `.claude/rules/no-legacy-shims-pre-1.0.md` (clean addition; no deprecation).
- Round 7 PR #50 review (NIT 2 origin).
- Task #326 (this round closes it).
