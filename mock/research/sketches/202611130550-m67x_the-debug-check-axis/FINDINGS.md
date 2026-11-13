# Where the choice of a debug abort sits

Outcome: WORKS for a type parameter with a default; a cargo feature works and carries one consumer's
choice into another's types.

The question is the vehicle for the axis that says whether a debug build aborts when a result leaves
the range, chosen by the consumer and separate from the overflow policy. Two arms over one four-bit
value whose sum wraps, `value::by_feature::U4` and `value::by_parameter::U4`:

- by feature, the abort is on unless the value crate's feature `quiet` is on;
- by parameter, the abort is a marker in the type, `U4<C: DebugCheck = Abort>`, with `Quiet` beside
  `Abort`.

Two consumers: `plain-user` names no feature and spells the parameter arm at its default, so it wants
the abort; `quiet-user` turns the feature on and spells `U4<Quiet>`. Two binaries, each run in the dev
profile on the toolchain in `outputs/toolchain.txt`, each computing fifteen plus one: `app-plain`
depends on `plain-user` alone, `app-both` on both consumers.

## What each run shows

`outputs/app-plain.txt`: with `plain-user` alone, both arms abort.

`outputs/app-both.txt`: with `quiet-user` in the same build, `plain-user`'s feature arm completes to
0 and no longer aborts, though `plain-user` changed nothing. Cargo unifies the features of one crate
across a build, so the choice one consumer made reached a type another consumer declared. The
parameter arm still aborts for `plain-user` and completes for `quiet-user`, because each choice is in
the type that consumer spelled.

So under a feature one declared type has two behaviours, picked by whichever other crate shares the
build, and a consumer with no feature line cannot choose at all. Under the parameter a consumer with
no feature line chooses by naming `Quiet`, one type keeps one behaviour in every build, and the
default costs a consumer that wants the abort nothing to spell.

## What it does not show

The cost of the parameter at the call site, which a four-bit toy does not measure, and how the
parameter sits beside a strategy parameter on an alias, which is
`202611130550-m67x_an-alias-that-takes-a-strategy-later`.
