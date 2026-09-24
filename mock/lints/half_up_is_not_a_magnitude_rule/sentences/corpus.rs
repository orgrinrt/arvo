//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The pairs the reader gets right on both sides.
//!
//! Eight of the entries are the sentences a review measured against the reader
//! that stood before the rewrite: six it was silent on and two it fired on.
//! They are marked, so the corpus says where it came from. The pairs from the
//! one on a relative binding back to the mode onward answer the third review of
//! the attribution: which rule a reading belongs to when a clause names two.
//! The last two are the segment bounds of the list and of the contrast, the
//! first of them the oracle comment in `arvo-format` word for word.

use super::Pair;

/// The corpus. A pair is added here rather than as a test of its own, so the
/// count below is what says whether one was dropped.
pub(super) const PAIRS: &[Pair] = &[
    Pair {
        fires:    "Unlike `half_even`, `half_up` sends a tie away from zero, as MATLAB's Round does.",
        silent:   "Unlike `half_up`, MATLAB's Round sends a tie away from zero.",
        subject:  None,
        differs:  "which of the two names the contrast is against",
        measured: true,
    },
    Pair {
        fires:    "With no dither to read, `half_up` sends a tie away from zero.",
        silent:   "With no dither to read, `half_up` sends no tie away from zero.",
        subject:  None,
        differs:  "whether the negator sits in the reading's own segment",
        measured: true,
    },
    Pair {
        fires:    "Except for the unsigned formats, `half_up` takes a tie away from zero.",
        silent:   "`half_up` takes a tie toward positive infinity, except that the alias takes it \
                  away from zero.",
        subject:  None,
        differs:  "whether the clause with the reading in it names the mode",
        measured: true,
    },
    Pair {
        fires:    "`half_up` rounds to nearest and, for negative inputs, rounds the tie to the \
                  larger magnitude.",
        silent:   "`half_up` rounds to nearest and, for negative inputs, never rounds the tie to \
                  the larger magnitude.",
        subject:  None,
        differs:  "a negator in the reading's segment, with a parenthetical between the two either \
                  way",
        measured: true,
    },
    Pair {
        fires:    "For `half_up` the tie at -2.5 goes to -3, away from zero, like Java.",
        silent:   "For `half_up` the tie at -2.5 goes to -2, not away from zero, unlike Java.",
        subject:  None,
        differs:  "a negator in front of the reading, in a sentence cut into three by commas",
        measured: true,
    },
    Pair {
        fires:    "`half_up` is nearest. Its tie goes away from zero.",
        silent:   "`half_up` is nearest. The alias's tie goes away from zero.",
        subject:  None,
        differs:  "whether the second sentence opens with a pronoun for the mode",
        measured: true,
    },
    Pair {
        fires:    "The error of `half_up` is at most half a quantum, and its tie goes up in \
                  magnitude.",
        silent:   "The error of `half_up` is at most half a quantum in magnitude.",
        subject:  None,
        differs:  "whether a direction word stands in front of the magnitude",
        measured: true,
    },
    Pair {
        fires:    "`half_up` bounds the error by `q/2` and sends a tie away from zero at every sign.",
        silent:   "`half_up` bounds the error by `q/2` in magnitude at every sign.",
        subject:  None,
        differs:  "whether the magnitude is the error's or the tie's",
        measured: true,
    },
    Pair {
        fires:    "`half_up` is ties away from zero.",
        silent:   "`half_up` is not ties away from zero.",
        subject:  None,
        differs:  "the negator, with nothing else between the name and the reading",
        measured: false,
    },
    Pair {
        fires:    "`HalfUp` reads the sign at a tie.",
        silent:   "`HalfUp` never reads the sign at a tie.",
        subject:  None,
        differs:  "the negator, on the sign vocabulary rather than the magnitude one",
        measured: false,
    },
    Pair {
        fires:    "`HalfUp` commutes with reflection.",
        silent:   "`HalfUp` never commutes with reflection.",
        subject:  None,
        differs:  "the negator, on the symmetry vocabulary",
        measured: false,
    },
    Pair {
        fires:    "Nearest, and a tie goes away from zero.",
        silent:   "Nearest, and a tie goes toward positive infinity.",
        subject:  Some("HalfUp"),
        differs:  "the reading itself, in a doc block whose item names the mode",
        measured: false,
    },
    Pair {
        fires:    "| `half_up` | ties away from zero |",
        silent:   "| `half_up` | not ties away from zero |",
        subject:  None,
        differs:  "the negator, inside a table cell",
        measured: false,
    },
    Pair {
        fires:    "In `{floor, half_up away from zero, ceil}` the second is wanted.",
        silent:   "In `{floor, away from zero, half_up, ceil}` the second is wanted.",
        subject:  None,
        differs:  "whether the name and the reading are one item of the list or two",
        measured: false,
    },
    Pair {
        fires:    "The rule `half_up` takes a tie away from zero.",
        silent:   "The planted rule `half_up` takes a tie away from zero.",
        subject:  None,
        differs:  "the marker saying the clause is about another rule",
        measured: false,
    },
    Pair {
        fires:    "`half_up` (the nearest mode) sends a tie away from zero.",
        silent:   "`half_up` (the nearest mode) never sends a tie away from zero.",
        subject:  None,
        differs:  "a negator in the reading's segment, with a parenthetical between the name and \
                  the reading",
        measured: false,
    },
    Pair {
        fires:    "`half_up` is not the even rule but sends a tie away from zero.",
        silent:   "`half_up` is not the even rule and never sends a tie away from zero.",
        subject:  None,
        differs:  "whether a conjunction stands between the negator and the reading",
        measured: false,
    },
    Pair {
        fires:    "It is `half_up` which sends a tie away from zero.",
        silent:   "It is not `half_up` which sends a tie away from zero.",
        subject:  None,
        differs:  "a negator ahead of a segment the reading's clause hangs off",
        measured: false,
    },
    Pair {
        fires:    "Java's rule and `half_up` both send a tie away from zero.",
        silent:   "Java's rule, and not `half_up`, sends a tie away from zero.",
        subject:  None,
        differs:  "whether the negator sits in the name's own segment",
        measured: false,
    },
    Pair {
        fires:    "So a tie at -2.5 goes to -3, which is away from zero.",
        silent:   "So a tie at -2.5 goes to -2. That one is ties away from zero, and it is reached \
                  by shifting the position half a step away from zero and rounding toward zero.",
        subject:  Some("HalfUp"),
        differs:  "whether the clause with the reading is about the item or about the rule it was \
                  just contrasted with",
        measured: false,
    },
    Pair {
        fires:    "`half_up` takes a tie away from zero at a negative slot.",
        silent:   "`half_up` takes a tie toward positive infinity at a negative slot, where the \
                  alias goes away from zero.",
        subject:  None,
        differs:  "which clause the mode's name sits in",
        measured: false,
    },
    Pair {
        fires:    "The rounding region commutes with reflection for toward zero and half-even, \
                  and for floor, ceil, half-up or a stochastic decision at a fixed dither.",
        silent:   "The rounding region commutes with reflection for toward zero and half-even, \
                  and not for floor, ceil, half-up or a stochastic decision at a fixed dither.",
        subject:  None,
        differs:  "whether a negator opens the list the name sits in, behind the reading",
        measured: false,
    },
    Pair {
        fires:    "`half_up`, unlike the even rule, sends a tie away from zero.",
        silent:   "`half_up`, unlike the even rule, never sends a tie away from zero.",
        subject:  None,
        differs:  "whether the negator is in the reading's own segment or in an aside between the \
                  name and the reading",
        measured: false,
    },
    Pair {
        fires:    "In `{floor, half_up, a tie away from zero, ceil}` the second is wanted.",
        silent:   "In `{floor, half_up, away from zero, ceil}` the second is wanted.",
        subject:  None,
        differs:  "whether the item holding the reading holds anything besides it",
        measured: false,
    },
    Pair {
        fires:    "`half_up`: a tie goes away from zero.",
        silent:   "`half_even`: a tie goes away from zero.",
        subject:  None,
        differs:  "which mode the clause before the colon names",
        measured: false,
    },
    Pair {
        fires:    "`half_up`; a tie goes away from zero.",
        silent:   "The alias; a tie goes away from zero.",
        subject:  None,
        differs:  "whether the clause before the semicolon names the mode",
        measured: false,
    },
    Pair {
        fires:    "`half_up`? a tie goes away from zero.",
        silent:   "`half_up`? a tie never goes away from zero.",
        subject:  None,
        differs:  "a negator in the reading's segment, across a question mark",
        measured: false,
    },
    Pair {
        fires:    "`half_up` is a mode. Ties go away from zero under it.",
        silent:   "`half_up` is a mode. Ties go away from zero under `Round`.",
        subject:  None,
        differs:  "whether the clause carrying the reading names another rule, where neither opens \
                  with a pronoun",
        measured: false,
    },
    Pair {
        fires:    "`half_up` is a mode. It is nearest. Its tie goes away from zero.",
        silent:   "`half_up` is a mode. `HalfEven` is another. Its tie goes away from zero.",
        subject:  None,
        differs:  "whether the sentence between the name and the pronoun names another rule",
        measured: false,
    },
    Pair {
        fires:    "`half_up` pairs with `q/2`; it sends a tie away from zero.",
        silent:   "`half_up` pairs with `HalfEven`; it sends a tie away from zero.",
        subject:  None,
        differs:  "whether the code span standing after the name is another rule",
        measured: false,
    },
    Pair {
        fires:    "Nearest. A tie goes away from zero.",
        silent:   "Nearest. A tie goes away from zero under `HalfEven`.",
        subject:  Some("HalfUp"),
        differs:  "whether the clause names another rule, where the doc block's item would otherwise \
                  reach it",
        measured: false,
    },
    Pair {
        fires:    "`half_up` goes up. And takes ties away from zero.",
        silent:   "`half_up` goes up. And `HalfEven` takes ties away from zero.",
        subject:  None,
        differs:  "whether the clause after the boundary names another rule",
        measured: false,
    },
    Pair {
        fires:    "`half_up` is a mode. It is read up the number line. A tie goes away from zero.",
        silent:   "`half_up` is a mode. Read it as up the number line. A tie goes away from zero.",
        subject:  None,
        differs:  "whether the clause between the name and the reading points back, which is what \
                  carries the name past one clause",
        measured: false,
    },
    Pair {
        fires:    "`half_up` sends a tie to the nearest slot, where ties away from zero would give \
                  -3.",
        silent:   "`half_up` sends a tie toward positive infinity, where ties away from zero would \
                  give -3.",
        subject:  None,
        differs:  "whether the clause states the settled denotation ahead of the reading",
        measured: false,
    },
    Pair {
        fires:    "`half_up` adds half a step and floors. Either way a tie goes away from \
                  zero.",
        silent:   "`half_up` adds half a step and floors. Toward zero sends a tie away from \
                  zero.",
        subject:  None,
        differs:  "whether the clause after it opens by naming another rule, capitalised",
        measured: false,
    },
    Pair {
        fires:    "`half_up` is nearest, which goes away from zero.",
        silent:   "`half_up` is not Java's `HALF_UP`, which goes away from zero.",
        subject:  None,
        differs:  "whether the rule standing last before a relative is the mode or another rule",
        measured: false,
    },
    Pair {
        fires:    "`half_up` is a nearest rule, the one that goes away from zero.",
        silent:   "`half_up` is not `ROUND_HALF_UP`, the rule that goes away from zero.",
        subject:  None,
        differs:  "the same, with the relative standing behind an apposition",
        measured: false,
    },
    Pair {
        fires:    "`half_up` is nearest, and it sends a tie away from zero.",
        silent:   "`half_up` is nearest, and `HalfEven` sends a tie away from zero.",
        subject:  None,
        differs:  "whether the reading's own segment names another rule ahead of it",
        measured: false,
    },
    Pair {
        fires:    "`half_up`, like `floor`, sends a tie away from zero.",
        silent:   "`half_up` is like `floor`, which sends a tie away from zero.",
        subject:  None,
        differs:  "whether the other rule sits in an aside or is what a relative binds to",
        measured: false,
    },
    Pair {
        fires:    "`half_up` has the reading `away from zero`.",
        silent:   "`half_up`'s row has the entry `away from zero`.",
        subject:  None,
        differs:  "whether the word beside the span names what the sentence gives or a piece of a \
                  corpus",
        measured: false,
    },
    Pair {
        fires:    "`half_up` takes the `away from zero` reading at a tie.",
        silent:   "`half_up` takes the `away from zero` spelling from its rows.",
        subject:  None,
        differs:  "the same, with the word behind the span",
        measured: false,
    },
    Pair {
        fires:    "`half_up` goes toward positive infinity and away from zero.",
        silent:   "`half_up` goes toward positive infinity, where away from zero would give -3.",
        subject:  None,
        differs:  "whether a conjunction or a contrast stands between the denotation and the \
                  reading",
        measured: false,
    },
    Pair {
        fires:    "The operation is `half_up`. It goes away from zero.",
        silent:   "The operation is not `half_up`. It goes away from zero.",
        subject:  None,
        differs:  "whether the clause before the pronoun denies that its subject is the mode",
        measured: false,
    },
    Pair {
        fires:    "`half_up` is nearest, but `HalfEven` like `half_up` sends a tie away from zero.",
        silent:   "`half_up` is nearest, but `HalfEven` like `floor` sends a tie away from zero.",
        subject:  None,
        differs:  "whether the reading's segment names the mode as well as another rule",
        measured: false,
    },
    Pair {
        fires:    "`HalfEven` is next to `half_up`, which goes away from zero.",
        silent:   "`half_up` is next to `HalfEven`, which goes away from zero.",
        subject:  None,
        differs:  "which of the two names stands last before the relative",
        measured: false,
    },
    Pair {
        fires:    "Next to `HalfEven` stands `half_up`, and it sends a tie away from zero.",
        silent:   "Next to `half_up` stands `HalfEven`, and it sends a tie away from zero.",
        subject:  None,
        differs:  "which of the two names a pronoun behind a conjunction points at",
        measured: false,
    },
    Pair {
        fires:    "`half_up` is a mode. And it is nearest. Its tie goes away from zero.",
        silent:   "`HalfEven` is a mode. And it sends a tie away from zero.",
        subject:  Some("HalfUp"),
        differs:  "whether a clause opening on a conjunction and a pronoun points back, so the doc \
                  block's item does not fill in",
        measured: false,
    },
    Pair {
        fires:    "| `half_up` | nearest, and it sends a tie away from zero |",
        silent:   "| `half_up` | nearest, and `HalfEven` sends a tie away from zero |",
        subject:  None,
        differs:  "whether a table cell's segment names another rule ahead of the reading",
        measured: false,
    },
    Pair {
        fires:    "The half-up rows are the ruling's own examples: a tie goes to the nearest slot \
                  at every sign, so `-2.5` goes to `-2` and `-0.5` to `0`, where ties away from \
                  zero would give `-3` and `-1`.",
        silent:   "The half-up rows are the ruling's own examples: a tie goes toward positive \
                  infinity at every sign, so `-2.5` goes to `-2` and `-0.5` to `0`, where ties away \
                  from zero would give `-3` and `-1`.",
        subject:  None,
        differs:  "whether the denotation stands ahead, with conjunctions in the segments between \
                  it and the reading's own",
        measured: false,
    },
    Pair {
        fires:    "`half_up` sends a tie (see below), away from zero.",
        silent:   "The modes floor (see below), away from zero, and half_up.",
        subject:  None,
        differs:  "whether the nearest item holding text beside the reading, past the empty \
                  segment a bracket and a comma cut, is a bare name",
        measured: false,
    },
];
