//! E. Can a reasoned claim about the canon's own contents state its region?
//!
//! Asked of the shipped checkers rather than of the corpus. The three lints
//! that decide it are pulled in by `#[path]` from `mock/lints/`, the same way
//! the engine's generated pack pulls them, so what runs below is the lint code
//! itself and not a restatement of it.
//!
//! Six candidate spellings of one claim. The claim is the layering derivation
//! already in the registry as
//! `proposal::the_topics_form_a_stack_a_frame_and_the_canons_own_machinery`,
//! whose own `note` says it was "filed `normative` after being written
//! `argument`" because "no predicate, because none in this registry can express
//! it".
//!
//! The predictions are written in the test names and asserted, so a run in
//! which the lints stop firing fails rather than reads as agreement.

#![allow(unused)]

#[path = "../../../../../lints/canon_rows.rs"]
pub mod canon_rows;

#[path = "../../../../../lints/a_region_agrees_with_the_sentence_kind.rs"]
pub mod a_region_agrees_with_the_sentence_kind;

#[path = "../../../../../lints/every_predicate_names_a_declared_axis.rs"]
pub mod every_predicate_names_a_declared_axis;

#[path = "../../../../../lints/an_imposition_rests_on_no_instrument.rs"]
pub mod an_imposition_rests_on_no_instrument;
