//--------------------------------------------------------------------------------------------------
// Copyright (c) 2026                   orgrinrt                 ort@hiisi.digital
// SPDX-License-Identifier: MPL-2.0     https://mozilla.org/MPL/2.0        contact@hiisi.digital
//--------------------------------------------------------------------------------------------------

//! The walk: every host primitive in a file, with the position it occupies.
//!
//! **From the leaf upward rather than from the item downward.** tree-sitter
//! gives `primitive_type` as a node kind, so every occurrence of a host type is
//! one node kind and the walk cannot miss a shape it did not think to look for.
//! Going the other way, enumerating item kinds and reading their types, misses
//! whatever the enumeration left out, and the enumeration is exactly what is in
//! question here.
//!
//! What the leaf cannot tell you is where it sits, so each one walks up its own
//! ancestors until it reaches a node that defines a position. A type is a tree
//! (`Option<[u32; 4]>` is four nodes above the `u32`), and the transparent
//! shapes are stepped through rather than enumerated: anything that is not a
//! position-defining kind is passed.
//!
//! # What this cannot establish
//!
//! **Reachability, as against declared visibility.** `pub` on an item inside a
//! private module is not reachable, and a private item re-exported by a `pub
//! use` is. The walk reads `pub` and the chain of enclosing modules, which gets
//! the common cases and is wrong in both directions at the edges. It is stated
//! rather than papered over, and the direction of the error is knowable: a
//! facade crate that re-exports is undercounted, a crate with a private module
//! full of `pub` items is overcounted.
//!
//! **A macro's expansion.** A position written inside `macro_rules!` is found
//! where the macro is defined, once, rather than at each expansion. Under-counts
//! a crate that generates its surface, which several here do.

use mockspace::tree_sitter::{Node, Parser};

use crate::kinds::{family, Found, Position};

/// Parse and collect. Returns empty for a file that will not parse at all,
/// which for Rust is effectively unreachable, since tree-sitter recovers.
pub fn walk(tree_label: &str, path: &str, source: &str) -> Vec<Found> {
    let shipped = crate::corpus::is_shipped(path);
    let mut parser = Parser::new();
    if parser
        .set_language(&mockspace::tree_sitter_rust::LANGUAGE.into())
        .is_err()
    {
        return Vec::new();
    }
    let Some(tree) = parser.parse(source, None) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    collect(
        tree.root_node(),
        source,
        tree_label,
        path,
        shipped,
        &mut out,
    );
    out
}

/// Depth-first over every node, picking out the primitive leaves.
fn collect(node: Node, src: &str, tree: &str, path: &str, shipped: bool, out: &mut Vec<Found>) {
    if node.kind() == "primitive_type" {
        let text = text_of(node, src);
        if family(&text).is_some() {
            let (position, name, owner) = classify(node, src);
            out.push(Found {
                tree: tree.to_string(),
                path: path.to_string(),
                line: node.start_position().row + 1,
                position,
                primitive: text,
                name,
                owner,
                public: visible(node, src),
                shipped,
            });
        }
        return;
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect(child, src, tree, path, shipped, out);
    }
}

/// Walk up from the leaf to the node that decides what position this is.
///
/// Returns the position, the identifier it is attached to, and the enclosing
/// item's name.
fn classify(leaf: Node, src: &str) -> (Position, String, String) {
    let mut cur = leaf;
    while let Some(parent) = cur.parent() {
        match parent.kind() {
            // ---- the excepted one, and it is checked before anything else,
            // because a const parameter's type sits inside `type_parameters`
            // which sits inside the item that would otherwise claim it.
            "const_parameter" => {
                return (
                    Position::ConstGenericParam,
                    named_child_text(parent, "identifier", src),
                    enclosing_item_name(parent, src),
                );
            }

            "parameter" => {
                return (
                    Position::FnParam,
                    pattern_text(parent, src),
                    enclosing_item_name(parent, src),
                );
            }

            "field_declaration" => {
                let is_variant = ancestor_kind(parent, "enum_variant").is_some();
                return (
                    if is_variant {
                        Position::VariantField
                    } else {
                        Position::StructField
                    },
                    named_child_text(parent, "field_identifier", src),
                    enclosing_item_name(parent, src),
                );
            }

            "ordered_field_declaration_list" => {
                let is_variant = ancestor_kind(parent, "enum_variant").is_some();
                return (
                    if is_variant {
                        Position::VariantField
                    } else {
                        Position::StructField
                    },
                    "<tuple field>".to_string(),
                    enclosing_item_name(parent, src),
                );
            }

            "const_item" | "static_item" => {
                let name = named_child_text(parent, "identifier", src);
                let owner = enclosing_item_name(parent, src);
                let pos = match ancestor_kind(parent, "declaration_list").and_then(|d| d.parent()) {
                    Some(g) if g.kind() == "trait_item" => Position::TraitConst,
                    Some(g) if g.kind() == "impl_item" => Position::ImplConst,
                    _ => Position::ItemConst,
                };
                return (pos, name, owner);
            }

            "type_item" => {
                // `type Carrier = u16;` is a `type_item` wherever it is
                // written. Inside a trait or an impl it is an associated type,
                // which is a contract coordinate an outside implementor writes;
                // at module level it is a free alias, which nobody else has to
                // name. tree-sitter spells both the same, so the body it sits in
                // is what tells them apart.
                let inside = ancestor_kind(parent, "declaration_list")
                    .and_then(|d| d.parent())
                    .map(|g| g.kind().to_string())
                    .unwrap_or_default();
                let pos = if inside == "trait_item" || inside == "impl_item" {
                    Position::AssocType
                } else {
                    Position::AliasTarget
                };
                return (
                    pos,
                    named_child_text(parent, "type_identifier", src),
                    enclosing_item_name(parent, src),
                );
            }

            "associated_type" => {
                return (
                    Position::AssocType,
                    named_child_text(parent, "type_identifier", src),
                    enclosing_item_name(parent, src),
                );
            }

            "function_item" | "function_signature_item" => {
                // Reached only from the return type: a parameter would have
                // stopped at `parameter`, and a body would have stopped at
                // one of the interior kinds below.
                return (
                    Position::FnReturn,
                    named_child_text(parent, "identifier", src),
                    enclosing_item_name(parent, src),
                );
            }

            "impl_item" => {
                // The type argument of the trait being implemented, or of the
                // type it is implemented for.
                return (
                    Position::ImplTraitArg,
                    "<impl head>".to_string(),
                    enclosing_item_name(parent, src),
                );
            }

            "trait_bounds"
            | "where_predicate"
            | "higher_ranked_trait_bound"
            | "constrained_type_parameter"
            | "optional_type_parameter" => {
                return (
                    Position::BoundArg,
                    "<bound>".to_string(),
                    enclosing_item_name(parent, src),
                );
            }

            // Anything with a body is interior by definition: a local, a cast,
            // a turbofish, a closure parameter, a const block.
            "block"
            | "let_declaration"
            | "type_cast_expression"
            | "closure_parameters"
            | "const_block"
            | "match_arm"
            | "call_expression"
            | "unit_expression" => {
                return (
                    Position::Interior,
                    "<interior>".to_string(),
                    enclosing_item_name(parent, src),
                );
            }

            _ => {}
        }
        cur = parent;
    }
    (
        Position::Interior,
        "<unclassified>".to_string(),
        String::new(),
    )
}

/// The nearest enclosing named item, for context in a report.
fn enclosing_item_name(from: Node, src: &str) -> String {
    let mut cur = Some(from);
    while let Some(node) = cur {
        match node.kind() {
            "trait_item" | "struct_item" | "enum_item" | "union_item" => {
                return named_child_text(node, "type_identifier", src);
            }
            "impl_item" => {
                let mut c = node.walk();
                let mut names: Vec<String> = Vec::new();
                for child in node.children(&mut c) {
                    if matches!(
                        child.kind(),
                        "type_identifier" | "generic_type" | "scoped_type_identifier"
                    ) {
                        names.push(text_of(child, src));
                    }
                }
                return format!("impl {}", names.join(" for "));
            }
            "function_item" | "function_signature_item" => {
                let n = named_child_text(node, "identifier", src);
                if !n.is_empty() {
                    return n;
                }
            }
            _ => {}
        }
        cur = node.parent();
    }
    String::new()
}

/// Whether an outside crate can see the item this sits in.
///
/// Three rules, and each is what the grammar can actually establish:
///
/// - An item inside a `trait_item` is as visible as the trait, because a trait
///   member carries no visibility of its own. Same for an `enum_variant`.
/// - An item inside an `impl_item` implementing a trait is as visible as the
///   trait, and an inherent one needs its own `pub`.
/// - Otherwise the item needs a `visibility_modifier`, and every enclosing
///   `mod_item` needs one too.
///
/// A `#[cfg(test)]` module is not visible, whatever it declares.
fn visible(leaf: Node, src: &str) -> bool {
    // Whether this sits inside a trait, or inside an impl of one, is decided
    // before the walk rather than during it. The walk runs leaf to root, so it
    // meets the member before the trait that gives it its visibility, and a
    // flag set on the way out arrives one item too late: the `const BASE: i32`
    // of a `pub trait` carries no `pub` of its own and was being called
    // private for exactly that reason.
    let mut inherited_from_trait = ancestor_kind(leaf, "trait_item").is_some()
        || ancestor_kind(leaf, "enum_item").is_some()
        || ancestor_kind(leaf, "impl_item").is_some_and(impl_is_trait_impl);
    let mut cur = Some(leaf);
    while let Some(node) = cur {
        match node.kind() {
            "mod_item" => {
                if is_cfg_test(node, src) || !has_pub(node, src) {
                    return false;
                }
            }
            "trait_item" | "enum_item" => {
                inherited_from_trait = true;
                if !has_pub(node, src) {
                    return false;
                }
            }
            "impl_item" => {
                // `impl Trait for T` members are as public as the trait, which
                // is not in this file's reach; treated as public, and the
                // inherent case still needs its own `pub` from the item below.
                if impl_is_trait_impl(node) {
                    inherited_from_trait = true;
                }
            }
            "function_item"
            | "function_signature_item"
            | "struct_item"
            | "union_item"
            | "const_item"
            | "static_item"
            | "type_item"
            | "associated_type" => {
                if !inherited_from_trait && !has_pub(node, src) {
                    return false;
                }
            }
            "field_declaration"
                // An enum variant's field carries no visibility of its own.
                if ancestor_kind(node, "enum_variant").is_none() && !has_pub(node, src) => {
                    return false;
                }
            _ => {}
        }
        cur = node.parent();
    }
    true
}

fn impl_is_trait_impl(node: Node) -> bool {
    let mut c = node.walk();
    node.children(&mut c).any(|ch| ch.kind() == "for")
}

fn has_pub(node: Node, src: &str) -> bool {
    let mut c = node.walk();
    node.children(&mut c)
        .any(|ch| ch.kind() == "visibility_modifier" && text_of(ch, src).starts_with("pub"))
}

fn is_cfg_test(node: Node, src: &str) -> bool {
    let mut prev = node.prev_sibling();
    while let Some(p) = prev {
        if p.kind() == "attribute_item" {
            let t = text_of(p, src).replace(' ', "");
            if t.contains("cfg(test)") || t.contains("cfg(feature=\"test") {
                return true;
            }
            prev = p.prev_sibling();
            continue;
        }
        break;
    }
    false
}

fn ancestor_kind<'a>(from: Node<'a>, kind: &str) -> Option<Node<'a>> {
    let mut cur = from.parent();
    while let Some(node) = cur {
        if node.kind() == kind {
            return Some(node);
        }
        cur = node.parent();
    }
    None
}

fn named_child_text(node: Node, kind: &str, src: &str) -> String {
    let mut c = node.walk();
    for child in node.children(&mut c) {
        if child.kind() == kind {
            return text_of(child, src);
        }
    }
    String::new()
}

/// A parameter's binding name, which is a pattern rather than an identifier.
fn pattern_text(node: Node, src: &str) -> String {
    let mut c = node.walk();
    for child in node.children(&mut c) {
        if matches!(
            child.kind(),
            "identifier" | "mut_pattern" | "ref_pattern" | "tuple_pattern"
        ) {
            return text_of(child, src);
        }
    }
    String::new()
}

fn text_of(node: Node, src: &str) -> String {
    src.get(node.byte_range()).unwrap_or_default().to_string()
}
