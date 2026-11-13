//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The catalogue's entries: every pair the reader is known to get wrong on one
//! named side, with the side.
//!
//! The data alone, so `catalogue.rs` holds the tests over it. Each entry's
//! test, ignore reason and ticket are there, at the same index. The entries
//! after the fourth are the residue of the closed lists the reader reads by,
//! one or more for each list, under the ticket of that list or shape.

#![cfg(test)]

use super::Pair;
use super::catalogue::{Known, Wrong};

/// Every known wrong reading, with its twin.
pub(super) const CATALOGUE: &[Known] = &[
    Known {
        pair:  Pair {
            fires:    "`half_up` is not Java's `HALF_UP`. It sends a tie away from zero.",
            silent:   "`half_up` is not the `HALF_UP` of Java. That one goes away from zero.",
            subject:  None,
            differs:  "whether the pronoun following a negated identity is `It`, referring to the \
                      sentence's subject, or `that one`, referring to what the identity denied",
            measured: false,
        },
        wrong: Wrong::LetsThrough,
    },
    Known {
        pair:  Pair {
            fires:    "`half_up` is nearest. Round halfway ties away from zero.",
            silent:   "`half_up` is nearest. Round sends halfway ties away from zero.",
            subject:  None,
            differs:  "whether `Round` opening the clause is an imperative whose object opens on a \
                      word of an open class, or the method followed by its verb",
            measured: false,
        },
        wrong: Wrong::LetsThrough,
    },
    Known {
        pair:  Pair {
            fires:    "`half_up` is a mode. Its decimal module sends a tie away from zero.",
            silent:   "`half_up` is a mode. Python's decimal module sends a tie away from zero.",
            subject:  None,
            differs:  "whether the clause's subject is the mode's own, by a pronoun, or a thing \
                      named in words the reader has no noun for",
            measured: false,
        },
        wrong: Wrong::Refuses,
    },
    // The same question as the Java pair, inside one clause: whether a pronoun
    // binds past a rule the clause denies the mode is. It stands under that
    // pair's question ticket, and the reader takes the rule standing last in
    // both until the ticket settles it.
    Known {
        pair:  Pair {
            fires:    "`half_up` differs from `floor`, and it sends a tie away from zero.",
            silent:   "`half_up` differs from `floor`. It sends a tie away from zero.",
            subject:  None,
            differs:  "whether the pronoun opens a segment joined by a comma and a conjunction \
                      within one clause, where the subject governs, or opens the next clause after \
                      a full stop, where the name standing last does",
            measured: false,
        },
        wrong: Wrong::LetsThrough,
    },
    // 4. The noun-phrase check: a rule named by a hyphenated phrase alone.
    Known {
        pair:  Pair {
            fires:    "`half_up` is not slow, which sends a tie away from zero.",
            silent:   "`half_up` is not round-half-away, which sends a tie away from zero.",
            subject:  None,
            differs:  "whether the denied thing is a quality of the mode or a rule named by a \
                      hyphenated phrase with no rule noun",
            measured: false,
        },
        wrong: Wrong::Refuses,
    },
    // 5 to 8. The pronoun call, under the Java pair's question ticket: `it`
    // later in a subject clause hands it to the mode, and a personal pronoun
    // in a segment keeps an alias spelling a reading.
    Known {
        pair:  Pair {
            fires:    "`half_up` is the ruled mode. Every test shows it sends ties away from zero.",
            silent:   "`half_up` is the ruled mode. Seat 270 first read it as ties away from zero.",
            subject:  None,
            differs:  "whether `it` in a clause opening on a subject is the mode the subject \
                      reports on or the thing the subject read",
            measured: false,
        },
        wrong: Wrong::Refuses,
    },
    Known {
        pair:  Pair {
            fires:    "`half_up` is the ruled mode. The crate shows it sends a tie away from zero.",
            silent:   "`half_up` is the ruled mode. The crate, as it ships, sends a tie away from \
                      zero.",
            subject:  None,
            differs:  "whether `it` stands for the mode or for the clause's own subject",
            measured: false,
        },
        wrong: Wrong::Refuses,
    },
    Known {
        pair:  Pair {
            fires:    "`half_up` is the ruled mode. The shipped crate shows it sends a tie away from \
                      zero.",
            silent:   "`half_up` is the ruled mode. The shipped crate sends a tie away from zero, \
                      which it should not.",
            subject:  None,
            differs:  "whether `it` stands for the mode or for the clause's own subject, behind \
                      the reading",
            measured: false,
        },
        wrong: Wrong::Refuses,
    },
    Known {
        pair:  Pair {
            fires:    "`half_up` is floor(x + q/2), and ties-away is what it does.",
            silent:   "`half_up` is floor(x + q/2), and ties-away is what Java calls it.",
            subject:  None,
            differs:  "whether the pronoun in the alias's segment is the mode or the alias itself",
            measured: false,
        },
        wrong: Wrong::Refuses,
    },
    // 9 to 11. `COUNTED`: a seat number before a word that also counts.
    Known {
        pair:  Pair {
            fires:    "`half_up` sends ties up. 5 in 10 go away from zero.",
            silent:   "`half_up` sends ties up. 270 in the panel read ties away from zero.",
            subject:  None,
            differs:  "whether `in` behind the number makes a proportion or places a seat",
            measured: false,
        },
        wrong: Wrong::Refuses,
    },
    Known {
        pair:  Pair {
            fires:    "`half_up` sends ties up. 5 in 10 go away from zero.",
            silent:   "`half_up` is the ruled mode. 270 in its first answer read ties away from \
                      zero.",
            subject:  None,
            differs:  "whether `in` behind the number makes a proportion or opens a phrase about \
                      the seat",
            measured: false,
        },
        wrong: Wrong::Refuses,
    },
    Known {
        pair:  Pair {
            fires:    "`half_up` sends ties up. 5 or 6 go away from zero.",
            silent:   "`half_up` is the ruled mode. 270 or 271 answered ties away from zero.",
            subject:  None,
            differs:  "whether `or` joins two counts or two seats",
            measured: false,
        },
        wrong: Wrong::Refuses,
    },
    // 12. A bare number before a verb.
    Known {
        pair:  Pair {
            fires:    "`half_up` sends ties up. 5 go away from zero.",
            silent:   "`half_up` sends ties up. 270 read ties away from zero.",
            subject:  None,
            differs:  "whether the number counts the ties or names a seat that reads",
            measured: false,
        },
        wrong: Wrong::LetsThrough,
    },
    // 13. `MOTIONS`: a motion gerund the list does not hold.
    Known {
        pair:  Pair {
            fires:    "`half_up` sends negative ties away from zero instead of drifting toward zero.",
            silent:   "`half_up` sends negative ties toward positive infinity instead of drifting \
                      away from zero.",
            subject:  None,
            differs:  "whether the reading is what the mode does or what it is set against",
            measured: false,
        },
        wrong: Wrong::LetsThrough,
    },
    // 14. The gerund rule: a gerund naming `toward zero` before a relative.
    Known {
        pair:  Pair {
            fires:    "`half_up` is computed by adding half and then rounding toward zero, which \
                      sends a tie away from zero.",
            silent:   "`half_up` is floor(x + q/2), and rounding toward zero sends a tie away from \
                      zero.",
            subject:  None,
            differs:  "whether the operation the gerund names is how the mode is computed or a \
                      rule beside it",
            measured: false,
        },
        wrong: Wrong::LetsThrough,
    },
    // 15. `COPULAS`: a copula that predicates rather than identifies.
    Known {
        pair:  Pair {
            fires:    "For `half_up`, away from zero is right rather than toward positive infinity.",
            silent:   "For `half_up`, away from zero is not toward positive infinity.",
            subject:  None,
            differs:  "whether the copula predicates something of the reading or denies that it \
                      is the denotation",
            measured: false,
        },
        wrong: Wrong::LetsThrough,
    },
];
