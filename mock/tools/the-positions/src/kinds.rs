//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! What a position is, and the vocabulary the report is written in.

/// Where in the grammar a host primitive sits.
///
/// **Grammatical rather than semantic.** What a position wants is decided by
/// what it means, and the grammar cannot see that; what the grammar can see is
/// whether an outside implementor has to write the type themselves, which is
/// what decides whether a replacement is owed at all. The two axes are reported
/// separately for that reason.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Position {
    /// A parameter of a function a caller can call.
    FnParam,
    /// The return type of a function a caller can call.
    FnReturn,
    /// A named field of a public struct, itself public.
    StructField,
    /// A field of an enum variant. As public as the enum, always.
    VariantField,
    /// An associated constant on a trait. **The contract coordinate**: an
    /// implementor outside the defining crate writes this type themselves, so
    /// it is the position with the least room to be excused.
    TraitConst,
    /// An associated constant in an `impl` block.
    ImplConst,
    /// A free `const` or `static` item.
    ItemConst,
    /// The right-hand side of a `type` alias.
    AliasTarget,
    /// A default or a bound on an associated type.
    AssocType,
    /// A type argument in an implemented trait, `impl Trait<u32> for X`.
    ImplTraitArg,
    /// The type of a const generic parameter. **Excepted by op, in his own
    /// words, and the exception is bounded a second time by "only when truly
    /// painful otherwise".**
    ConstGenericParam,
    /// A generic argument in a bound, `where T: Holds<u32>`.
    BoundArg,
    /// Inside a function body: a local, a cast, a turbofish, a literal's type.
    /// **Not an API position.** Reported because the shipped lint refuses these
    /// too, so the demand on arvo is wider than the obligation's own wording,
    /// and the difference is worth being able to see rather than argue about.
    Interior,
}

impl Position {
    #[must_use]
    pub fn token(self) -> &'static str {
        match self {
            Self::FnParam => "fn-param",
            Self::FnReturn => "fn-return",
            Self::StructField => "struct-field",
            Self::VariantField => "variant-field",
            Self::TraitConst => "trait-const",
            Self::ImplConst => "impl-const",
            Self::ItemConst => "item-const",
            Self::AliasTarget => "alias-target",
            Self::AssocType => "assoc-type",
            Self::ImplTraitArg => "impl-trait-arg",
            Self::ConstGenericParam => "const-generic-param",
            Self::BoundArg => "bound-arg",
            Self::Interior => "interior",
        }
    }

    /// Whether an outside caller or implementor has to write this type.
    ///
    /// The obligation is over "public API positions", and this is what that
    /// phrase means once it is made checkable: a position is one when somebody
    /// who is not the defining crate has to name the type to use or implement
    /// the item.
    #[must_use]
    pub fn is_api(self) -> bool {
        !matches!(self, Self::Interior | Self::ConstGenericParam)
    }

    /// Every kind, so a report can enumerate rather than only what it happened
    /// to find. A kind at zero is a result.
    #[must_use]
    pub fn all() -> &'static [Self] {
        &[
            Self::FnParam,
            Self::FnReturn,
            Self::StructField,
            Self::VariantField,
            Self::TraitConst,
            Self::ImplConst,
            Self::ItemConst,
            Self::AliasTarget,
            Self::AssocType,
            Self::ImplTraitArg,
            Self::ConstGenericParam,
            Self::BoundArg,
            Self::Interior,
        ]
    }
}

/// What the primitive is wrapped in at the point it is written.
///
/// **This is the axis that decides whether arvo owes a numeral at all.** A
/// `usize` standing alone is a number and wants one. The `u8` in `*mut u8` is
/// not a number, it is the unit memory is addressed in, and no width, no
/// signedness and no rounding mode is the answer to it. The same is true of the
/// `u8` in `&[u8]`, which is a byte string.
///
/// Read off the parse: the first shaping node between the leaf and the position
/// it occupies. Nothing about the identifier is involved, so unlike the
/// semantic role this is a measurement rather than a reading.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Carrier {
    /// The primitive is the type. `count: usize`.
    Scalar,
    /// Behind a reference. `&u32`.
    Reference,
    /// The element of a slice or an array. `&[u8]`, `[u32; 4]`.
    Element,
    /// The target of a raw pointer. `*mut u8`.
    Pointer,
    /// A type argument. `Outcome<u32, E>`, `Option<usize>`.
    Argument,
}

impl Carrier {
    #[must_use]
    pub fn token(self) -> &'static str {
        match self {
            Self::Scalar => "scalar",
            Self::Reference => "reference",
            Self::Element => "element",
            Self::Pointer => "pointer",
            Self::Argument => "argument",
        }
    }

    /// Whether this is a position where a number is what is meant.
    ///
    /// A pointer target and a slice element are not: they name the unit of
    /// memory or of a byte string, and the thing that would replace them is a
    /// buffer type rather than a numeral. Everything else carries a value.
    #[must_use]
    pub fn is_a_number(self) -> bool {
        !matches!(self, Self::Pointer | Self::Element)
    }

    #[must_use]
    pub fn all() -> &'static [Self] {
        &[
            Self::Scalar,
            Self::Reference,
            Self::Element,
            Self::Pointer,
            Self::Argument,
        ]
    }
}

/// The host types the obligation names, plus the two the grammar groups with
/// them.
///
/// `str` and `char` are **outside** the obligation's wording, which names
/// "integers, floats, bool or usize". They are counted and kept apart, because
/// arvo's own `a-contract-coordinate-is-not-a-host-primitive` refuses them in
/// the exempt crate and the stack's `no-bare-static-str` refuses `str`
/// separately, so the demand on arvo is not the same for them and a single
/// total would hide that.
pub const NUMERIC: &[&str] = &[
    "u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128", "isize", "f32",
    "f64",
];

/// Truth, which the obligation names and which no width or signedness reaches.
pub const TRUTH: &[&str] = &["bool"];

/// Text, which the obligation does not name.
pub const TEXTUAL: &[&str] = &["str", "char"];

/// Which of the three families a primitive name is in, or `None` if it is not
/// one at all.
#[must_use]
pub fn family(name: &str) -> Option<&'static str> {
    if NUMERIC.contains(&name) {
        Some("numeric")
    } else if TRUTH.contains(&name) {
        Some("truth")
    } else if TEXTUAL.contains(&name) {
        Some("textual")
    } else {
        None
    }
}

/// One host primitive at one place.
#[derive(Clone, Debug)]
pub struct Found {
    /// The tree this came from, `<repo>@<ref>`.
    pub tree: String,
    /// Path inside that tree.
    pub path: String,
    /// One-based, so it matches what an editor shows.
    pub line: usize,
    /// Where in the grammar.
    pub position: Position,
    /// The primitive's own spelling.
    pub primitive: String,
    /// The identifier the position is attached to: the parameter's name, the
    /// field's name, the constant's name, the function's name for a return.
    ///
    /// **This is the only evidence of what the position means**, and what the
    /// position means is what decides which primitive could serve it. A walk
    /// that reported the grammar alone would answer "how many" and nothing
    /// about "what shape a replacement would have to have".
    pub name: String,
    /// The enclosing item's name, where there is one: the trait, the struct,
    /// the enum, the impl's type.
    pub owner: String,
    /// Whether the item is reachable from outside its crate as far as the parse
    /// can tell. See `walk::visible` for what that can and cannot establish.
    pub public: bool,
    /// Whether the file is part of a crate's compiled library, as against a
    /// test, an example, a bench variant or a research probe. `corpus::is_shipped`.
    pub shipped: bool,
    /// What the primitive is wrapped in, which decides whether a number is what
    /// the position means at all.
    pub carrier: Carrier,
    /// `None` where the type is the host's own, which is what the obligation is
    /// about. `Some("arvo")` or `Some("notko")` where the position already
    /// carries one of the stack's, which is the denominator the count needs.
    pub supplier: Option<&'static str>,
}

impl Found {
    /// A position the obligation is about: an outside crate has to write this
    /// type, the item is reachable, and the file ships.
    ///
    /// All three, and dropping any one of them is how the first run of this
    /// tool reported 39,529 where the answer is a fraction of that.
    #[must_use]
    pub fn is_demand(&self) -> bool {
        self.supplier.is_none() && self.position.is_api() && self.public && self.shipped
    }

    /// A position already carrying one of the stack's own types, at the same
    /// bar. The other half of the fraction.
    #[must_use]
    pub fn is_supply(&self) -> bool {
        self.supplier.is_some() && self.position.is_api() && self.public && self.shipped
    }
}
