//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! An adapted binary operation over every pair of members, as a table.
//!
//! A triple law over a range of eight bits is sixteen million triples, and each
//! is two or four applications of the operation. Built once per pair, a triple
//! is lookups. Every operation's suite that counts a triple law builds one, so
//! it lives here rather than in any one of them.

use crate::slots::Slot;

/// The adapted answer for every pair of members, as offsets from the lowest
/// slot.
///
/// Sized for the widest format the sweeps run, eight bits.
#[derive(PartialEq, Eq)]
pub(crate) struct Table {
    members: usize,
    cells:   [[u8; 256]; 256],
}

impl Table {
    /// Every pair of members of `[min, max]` put through `op`.
    ///
    /// Requires every answer to be a member, so a table is also a totality check
    /// over the pairs it holds.
    pub(crate) fn of(min: Slot, max: Slot, op: impl Fn(Slot, Slot) -> Slot) -> Self {
        let members = (max.index() - min.index() + 1) as usize;
        assert!(
            members <= 256,
            "the table holds eight-bit ranges and narrower"
        );
        let mut cells = [[0u8; 256]; 256];
        for (i, row) in cells.iter_mut().enumerate().take(members) {
            for (j, cell) in row.iter_mut().enumerate().take(members) {
                let a = Slot::at(min.index() + i as i128);
                let b = Slot::at(min.index() + j as i128);
                let got = op(a, b);
                assert!(
                    got.is_within(min, max).get(),
                    "{a:?}, {b:?} adapted to {got:?}, outside [{min:?}, {max:?}]"
                );
                *cell = (got.index() - min.index()) as u8;
            }
        }
        Self {
            members,
            cells,
        }
    }

    /// How many members the range has.
    pub(crate) fn members(&self) -> usize {
        self.members
    }

    /// The answer for the members at offsets `i` and `j`.
    pub(crate) fn at(&self, i: usize, j: usize) -> usize {
        self.cells[i][j] as usize
    }

    /// How many triples `(a . b) . c` and `a . (b . c)` disagree on.
    pub(crate) fn divergent(&self) -> u64 {
        let mut count = 0u64;
        for a in 0 .. self.members {
            for b in 0 .. self.members {
                let ab = self.at(a, b);
                for c in 0 .. self.members {
                    if self.at(ab, c) != self.at(a, self.at(b, c)) {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}
