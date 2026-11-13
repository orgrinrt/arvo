//! Where a file's code names one crate as the root of a path.
//!
//! `no-std-enforcer` and `no-alloc-enforcer` ask whether `std` or `alloc` is
//! reached from code. A line scan answered it by looking at how a line began,
//! which a visibility, a second statement on the line or an alias walks around:
//! `pub extern crate std;` and `pub extern crate alloc;` began with `pub`,
//! `alloc::rc::Rc::new` was not a `use`, and `extern crate stdx;` began with
//! `extern crate std`. This answers from the one parse the engine holds
//! (`ctx.tree()`), so the question is what the nodes say and not how the line
//! happens to start.
//!
//! A hit is one of three things. An `extern crate` declaration naming the
//! crate, whatever precedes it and whatever it is aliased to. A path whose first
//! segment is the crate (or an alias a declaration or a `use` gave it), in a
//! `use`, an expression, a type, a pattern or a macro's own path (`alloc::vec![]`),
//! with or without a leading `::`. And the same inside a macro's arguments, which
//! the grammar leaves as raw tokens and a path walk would never reach.
//!
//! A path that merely contains the name is not a hit. `facade::std::x` has a
//! root of `facade`, `my_std::x` has a root of `my_std`, and `extern crate stdx;`
//! names a different crate: each is decided by comparing a whole segment, never a
//! prefix of text.
//!
//! Each line is reported once, however many nodes on it name the crate.

use std::collections::BTreeSet;

use mockspace::LintContext;
use mockspace::tree_sitter::Node;

/// The node kinds that are a path written with `::`, whose `path` field is the
/// part before the last segment.
const SCOPED: [&str; 3] = ["scoped_identifier", "scoped_type_identifier", "scoped_use_list"];

/// The 1-based lines on which the file's code names `krate` as a path root.
///
/// Sorted and without repeats. A comment and a string literal are never code and
/// are never read: the tree holds them as single nodes, and a path spelled inside
/// one is no path node.
pub fn lines_naming(ctx: &LintContext, krate: &str) -> Vec<usize> {
    let src = ctx.source().as_bytes();
    let mut names: BTreeSet<String> = BTreeSet::from([krate.to_string()]);
    // Aliases first, so a use above the declaration that gave the alias is still
    // found: `extern crate std as s;` may come after `s::process::exit(0);`.
    // A pass sees the aliases the pass before it found, so `use s as t;` after
    // `extern crate std as s;` gives `t` too, whichever comes first in the file.
    loop {
        let known = names.clone();
        collect_aliases(ctx.tree().root_node(), src, krate, &known, &mut names);
        if names.len() == known.len() {
            break;
        }
    }
    let mut lines = BTreeSet::new();
    walk(ctx.tree().root_node(), src, krate, &names, &mut lines);
    lines.into_iter().collect()
}

fn text<'a>(node: Node, src: &'a [u8]) -> &'a str {
    let t = node.utf8_text(src).unwrap_or("");
    // `r#std` names `std`: a raw identifier is the same name with the prefix dropped.
    t.strip_prefix("r#").unwrap_or(t)
}

/// The names the crate is known by: its own and every alias a declaration or a
/// `use` gives it.
fn collect_aliases(
    node: Node,
    src: &[u8],
    krate: &str,
    known: &BTreeSet<String>,
    names: &mut BTreeSet<String>,
) {
    let aliased = match node.kind() {
        "extern_crate_declaration" => {
            node.child_by_field_name("name")
                .filter(|n| text(*n, src) == krate)
        },
        "use_as_clause" => {
            node.child_by_field_name("path").filter(|p| {
                if p.kind() == "self" {
                    in_a_list_of_the_crate(node, src, known)
                } else {
                    names_the_crate_alone(*p, src, known) && !is_nested_under_a_path(node)
                }
            })
        },
        _ => None,
    };
    if aliased.is_some()
        && let Some(alias) = node.child_by_field_name("alias")
    {
        names.insert(text(alias, src).to_string());
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_aliases(child, src, krate, known, names);
    }
}

/// Whether the path is the crate and nothing more: `std` or `::std`.
fn names_the_crate_alone(path: Node, src: &[u8], names: &BTreeSet<String>) -> bool {
    match path.kind() {
        "identifier" => names.contains(text(path, src)),
        "scoped_identifier" => {
            path.child_by_field_name("path").is_none()
                && path
                    .child_by_field_name("name")
                    .is_some_and(|n| names.contains(text(n, src)))
        },
        _ => false,
    }
}

/// Whether the clause sits directly in the braces of `std::{ .. }` or
/// `::std::{ .. }`, so a `self` in it is the crate itself.
fn in_a_list_of_the_crate(clause: Node, src: &[u8], names: &BTreeSet<String>) -> bool {
    clause
        .parent()
        .filter(|list| list.kind() == "use_list")
        .and_then(|list| list.parent())
        .filter(|outer| outer.kind() == "scoped_use_list" && !is_nested_under_a_path(*outer))
        .and_then(|outer| outer.child_by_field_name("path"))
        .is_some_and(|path| names_the_crate_alone(path, src, names))
}

/// Whether the path node stands inside the braces of a use that has a path in
/// front of them, `facade::{std}`, so its first segment is not the root.
///
/// Braces with nothing in front, `use {std::fmt};` and `use ::{std};`, hold
/// roots, and so do the braces of `std::{..}` itself, which the walk reaches as
/// the `scoped_use_list` node and not as a member of a list.
fn is_nested_under_a_path(node: Node) -> bool {
    let mut cur = node;
    while let Some(parent) = cur.parent() {
        match parent.kind() {
            "use_list" | "use_as_clause" => cur = parent,
            "scoped_use_list" => {
                return parent.child_by_field_name("list") == Some(cur)
                    && parent.child_by_field_name("path").is_some();
            },
            _ => return false,
        }
    }
    false
}

fn walk(
    node: Node,
    src: &[u8],
    krate: &str,
    names: &BTreeSet<String>,
    lines: &mut BTreeSet<usize>,
) {
    let hit = match node.kind() {
        "extern_crate_declaration" => {
            node.child_by_field_name("name")
                .is_some_and(|n| text(n, src) == krate)
        },
        kind if SCOPED.contains(&kind) => {
            !is_a_path_segment_of_a_longer_path(node)
                && !is_nested_under_a_path(node)
                && root_segment(node, src).is_some_and(|r| names.contains(r))
        },
        "use_wildcard" => {
            !is_a_path_segment_of_a_longer_path(node)
                && !is_nested_under_a_path(node)
                && first_named_child(node)
                    .and_then(|c| root_segment(c, src))
                    .is_some_and(|r| names.contains(r))
        },
        // `use std;`, `use std as s;`: the root is the whole path.
        "identifier" => {
            let parent_is_a_bare_use = node.parent().is_some_and(|p| {
                matches!(p.kind(), "use_declaration" | "use_list")
                    || (p.kind() == "use_as_clause" && p.child_by_field_name("path") == Some(node))
            });
            parent_is_a_bare_use && !is_nested_under_a_path(node) && names.contains(text(node, src))
        },
        "token_tree" => {
            if names_a_crate_in_tokens(node, src, names) {
                lines.insert(node.start_position().row + 1);
            }
            false
        },
        _ => false,
    };
    if hit {
        lines.insert(node.start_position().row + 1);
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        walk(child, src, krate, names, lines);
    }
}

/// Whether the node is the `path` of a longer scoped path, so the longer one is
/// the node that is counted and this one is not counted twice.
fn is_a_path_segment_of_a_longer_path(node: Node) -> bool {
    node.parent().is_some_and(|p| {
        (SCOPED.contains(&p.kind()) || p.kind() == "use_wildcard")
            && (p.child_by_field_name("path") == Some(node)
                || (p.kind() == "use_wildcard" && first_named_child(p) == Some(node)))
    })
}

fn first_named_child(node: Node) -> Option<Node> {
    node.named_child(0)
}

/// The first segment of a path node: what stands before the first `::`.
///
/// A leading `::` gives a node with no `path` and the segment is its `name`, so
/// `::std::x` has the root `std` and `crate::std::x` has the root `crate`.
fn root_segment<'a>(mut node: Node, src: &'a [u8]) -> Option<&'a str> {
    loop {
        match node.kind() {
            "identifier" | "crate" | "self" | "super" | "metavariable" => {
                return Some(text(node, src));
            },
            "use_wildcard" => node = first_named_child(node)?,
            kind if SCOPED.contains(&kind) => {
                match node.child_by_field_name("path") {
                    Some(path) => node = path,
                    None => return node.child_by_field_name("name").map(|n| text(n, src)),
                }
            },
            _ => return None,
        }
    }
}

/// Whether the raw tokens of a macro's arguments hold `<crate>::` at a path
/// start. The grammar does not parse a macro's arguments, so the paths in them
/// are an identifier token and a `::` token side by side.
fn names_a_crate_in_tokens(tree: Node, src: &[u8], names: &BTreeSet<String>) -> bool {
    let mut cursor = tree.walk();
    let tokens: Vec<Node> = tree.children(&mut cursor).collect();
    tokens.iter().enumerate().any(|(i, tok)| {
        if tok.kind() != "identifier" || !names.contains(text(*tok, src)) {
            return false;
        }
        if tokens.get(i + 1).map(|t| t.kind()) != Some("::") {
            return false;
        }
        // `facade::std::x`: a `::` before it that follows another segment means
        // this is not the root. `::std::x` with nothing before the colons is.
        let after_a_segment = i >= 2
            && tokens[i - 1].kind() == "::"
            && matches!(
                tokens[i - 2].kind(),
                "identifier" | "crate" | "self" | "super" | "metavariable"
            );
        let after_a_member = i >= 1 && tokens[i - 1].kind() == ".";
        !after_a_segment && !after_a_member
    })
}

#[cfg(test)]
mod tests {
    use mockspace::testkit::LintFixture;

    use super::lines_naming;

    fn std_lines(source: &str) -> Vec<usize> {
        lines_naming(&LintFixture::new(source).ctx(), "std")
    }

    fn alloc_lines(source: &str) -> Vec<usize> {
        lines_naming(&LintFixture::new(source).ctx(), "alloc")
    }

    // --- the shapes the line scan walked around ---

    #[test]
    fn an_extern_crate_declaration_is_found_whatever_precedes_it() {
        assert_eq!(std_lines("extern crate std;\n"), vec![1]);
        assert_eq!(std_lines("pub extern crate std;\n"), vec![1]);
        assert_eq!(std_lines("pub(crate) extern crate std;\n"), vec![1]);
        assert_eq!(alloc_lines("pub extern crate alloc;\n"), vec![1]);
        assert_eq!(std_lines("#[macro_use] extern crate std;\n"), vec![1]);
    }

    #[test]
    fn an_aliased_declaration_and_the_paths_through_the_alias_are_found() {
        let src = "pub extern crate std as s;\nfn f() { s::process::exit(0); }\n";
        assert_eq!(std_lines(src), vec![1, 2]);
    }

    #[test]
    fn an_alias_is_found_above_the_declaration_that_gave_it() {
        let src = "fn f() { s::process::exit(0); }\nextern crate std as s;\n";
        assert_eq!(std_lines(src), vec![1, 2]);
    }

    #[test]
    fn a_use_alias_names_the_crate_for_the_paths_after_it() {
        let src = "use std as s;\nfn f() { s::process::exit(0); }\n";
        assert_eq!(std_lines(src), vec![1, 2]);
    }

    #[test]
    fn a_rooted_path_in_an_expression_a_type_and_a_macro_name_is_found() {
        assert_eq!(
            alloc_lines("fn f() { let _ = alloc::rc::Rc::new(1); }\n"),
            vec![1]
        );
        assert_eq!(
            alloc_lines("fn f() { let _ = alloc::sync::Arc::new(1); }\n"),
            vec![1]
        );
        assert_eq!(alloc_lines("fn f() { let _ = alloc::vec![1]; }\n"), vec![1]);
        assert_eq!(alloc_lines("struct S(alloc::boxed::Box<u8>);\n"), vec![1]);
        assert_eq!(std_lines("fn f() { std::process::exit(0); }\n"), vec![1]);
        assert_eq!(std_lines("fn f(x: std::fmt::Error) {}\n"), vec![1]);
    }

    #[test]
    fn every_use_shape_is_found() {
        assert_eq!(std_lines("use std::fmt;\n"), vec![1]);
        assert_eq!(std_lines("use ::std::fmt;\n"), vec![1]);
        assert_eq!(std_lines("pub use std::fmt::Debug;\n"), vec![1]);
        assert_eq!(std_lines("use std::{fmt, io};\n"), vec![1]);
        assert_eq!(std_lines("use std::collections::*;\n"), vec![1]);
        assert_eq!(std_lines("use std;\n"), vec![1]);
        assert_eq!(std_lines("use {std::fmt, core::fmt::Debug};\n"), vec![1]);
        assert_eq!(alloc_lines("use alloc::vec::Vec;\n"), vec![1]);
        assert_eq!(alloc_lines("use ::alloc::boxed::Box;\n"), vec![1]);
    }

    /// A macro's arguments are raw tokens, which a path walk never reaches and the
    /// line scan did.
    #[test]
    fn a_path_inside_a_macros_arguments_is_found() {
        assert_eq!(std_lines("fn f() { m!(std::process::id()); }\n"), vec![1]);
        assert_eq!(
            alloc_lines("fn f() { m!(1, alloc::vec::Vec::new()); }\n"),
            vec![1]
        );
    }

    // --- what must stay silent ---

    #[test]
    fn control_a_different_crate_with_the_same_prefix_is_no_hit() {
        assert!(std_lines("extern crate stdx;\n").is_empty());
        assert!(std_lines("use stdx::fmt;\n").is_empty());
        assert!(std_lines("fn f() { my_std::x(); ministd::y(); }\n").is_empty());
        assert!(alloc_lines("extern crate allocator;\nuse allocator::Vec;\n").is_empty());
    }

    #[test]
    fn control_a_segment_that_is_not_the_root_is_no_hit() {
        assert!(std_lines("use facade::std::fmt;\n").is_empty());
        assert!(std_lines("fn f() { facade::std::x(); crate::std::y(); }\n").is_empty());
        assert!(std_lines("fn f() { m!(facade::std::x()); }\n").is_empty());
        assert!(std_lines("fn f(v: &V) { v.std::x; }\n").is_empty());
    }

    #[test]
    fn control_comments_strings_and_a_bare_name_are_no_hit() {
        assert!(std_lines("// use std::fmt;\n/* std::x */\nfn f() {}\n").is_empty());
        assert!(std_lines("/// std::process::exit\nfn f() { let s = \"std::fmt\"; }\n").is_empty());
        assert!(std_lines("fn f() { let std = 1; let _ = std + 1; }\n").is_empty());
        assert!(std_lines("fn f(x: Std) -> u8 { 0 }\n").is_empty());
    }

    /// A name inside the braces of a use that has a path in front of them is a
    /// member of that path, not a root: `facade::{std}` imports `facade::std`.
    #[test]
    fn control_a_name_inside_braces_after_a_path_is_no_hit() {
        assert!(std_lines("use facade::{std, core};\n").is_empty());
        assert!(std_lines("use facade::{std::fmt};\n").is_empty());
        assert!(std_lines("use facade::{a::{std::fmt}};\n").is_empty());
        assert!(std_lines("use facade::{std::*};\n").is_empty());
        assert!(std_lines("use facade::{std as s};\nfn f() { s::x(); }\n").is_empty());
        assert!(alloc_lines("use my_crate::{alloc::Pool, x};\n").is_empty());
    }

    #[test]
    fn braces_with_nothing_in_front_hold_roots() {
        assert_eq!(std_lines("use {std::fmt};\n"), vec![1]);
        assert_eq!(std_lines("use ::{std};\n"), vec![1]);
        assert_eq!(std_lines("use std::{fmt, io::{self}};\n"), vec![1]);
    }

    #[test]
    fn a_self_alias_inside_the_crates_braces_names_the_crate() {
        let src = "use std::{self as s};\nfn f() { s::process::exit(0); }\n";
        assert_eq!(std_lines(src), vec![1, 2]);
        let rooted = "use ::std::{self as s};\nfn f() { s::process::exit(0); }\n";
        assert_eq!(std_lines(rooted), vec![1, 2]);
    }

    #[test]
    fn a_leading_colon_use_alias_names_the_crate() {
        let src = "use ::std as s;\nfn f() { s::process::exit(0); }\n";
        assert_eq!(std_lines(src), vec![1, 2]);
    }

    #[test]
    fn control_a_self_alias_inside_another_crates_braces_is_no_alias() {
        let src = "use other::{self as s};\nfn f() { s::process::exit(0); }\n";
        assert!(std_lines(src).is_empty());
    }

    #[test]
    fn a_raw_identifier_names_the_same_crate() {
        assert_eq!(std_lines("extern crate r#std;\n"), vec![1]);
        assert_eq!(std_lines("fn f() { r#std::process::exit(0); }\n"), vec![1]);
        assert_eq!(
            std_lines("extern crate std as r#s;\nfn f() { s::y(); }\n"),
            vec![1, 2]
        );
    }

    #[test]
    fn an_alias_of_an_alias_names_the_crate() {
        let src = "extern crate std as s;\nuse s as t;\nfn f() { t::x(); }\n";
        assert_eq!(std_lines(src), vec![1, 2, 3]);
        let reversed = "fn f() { t::x(); }\nuse s as t;\nextern crate std as s;\n";
        assert_eq!(std_lines(reversed), vec![1, 2, 3]);
    }

    #[test]
    fn control_a_macro_metavariable_root_is_no_hit() {
        let src = "macro_rules! m { ($x:ident) => { $x::std::y() } }\n";
        assert!(std_lines(src).is_empty());
    }

    /// An alias is collected over the whole file, so one module's `use std as s`
    /// reads as the crate in a module where `s` is another crate.
    #[test]
    #[ignore = "catalogue: aliases are not scoped to their module; tracked #crate_root_paths_reads_a_use_alias_in_one_module_as_the_crat"]
    fn an_alias_in_one_module_is_not_the_crate_in_another() {
        let src = "mod m { use std as s; }\nmod n { use other as s; fn f() { s::x(); } }\n";
        assert_eq!(std_lines(src), vec![1]);
    }

    /// Aliases are read by name over the whole file, not by what they point to.
    /// `extern crate self as std` makes `std` the current crate, and a later `std::x()`
    /// is then not the standard library.
    #[test]
    #[ignore = "catalogue: aliases are read by name over the whole file, not by what they point to; tracked #crate_root_paths_reads_a_use_alias_in_one_module_as_the_crat"]
    fn an_alias_of_self_is_not_the_crate() {
        let src = "extern crate self as std;\nfn f() { std::x(); }\n";
        assert_eq!(std_lines(src), Vec::<usize>::new());
    }

    #[test]
    fn a_line_naming_the_crate_twice_is_one_line() {
        assert_eq!(std_lines("fn f() { std::a(); std::b(); }\n"), vec![1]);
    }

    #[test]
    fn the_two_crates_are_asked_apart() {
        let src = "use std::fmt;\nuse alloc::vec::Vec;\n";
        assert_eq!(std_lines(src), vec![1]);
        assert_eq!(alloc_lines(src), vec![2]);
    }
}
