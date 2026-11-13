//! The source of one file with everything that is not code blanked out.
//!
//! The enforcer lints (`no-alloc-enforcer`, `no-dynamic-dispatch`,
//! `no-runtime-grow`, `no-std-enforcer`) ask whether a forbidden name is
//! written in code. A line scan cannot answer that: it takes a trailing `//`
//! comment, an interior line of a block comment and the text of a string
//! literal, raw or not, for code. This module answers it from the one
//! tree-sitter parse the engine already holds for the file (`ctx.tree()`),
//! so the four lints share one reading and none keeps a scanner of its own.
//!
//! What is blanked is every `line_comment` (doc comments included),
//! `block_comment`, `string_literal`, `raw_string_literal` and `char_literal`
//! node. Each byte of such a node becomes a space except a line break, so the
//! result has the byte length and the line numbering of the source and a
//! finding can still quote the original line. A lifetime is not a char
//! literal in the grammar, so a line carrying `'a` ticks loses nothing.

use mockspace::LintContext;

/// The node kinds that are never code, and are blanked whole.
const BLANKED: [&str; 5] = [
    "line_comment",
    "block_comment",
    "string_literal",
    "raw_string_literal",
    "char_literal",
];

/// The file's source with every comment and literal blanked.
///
/// The result is the same length as `ctx.source()` and breaks lines in the same
/// places, so `result.lines()` and `ctx.source().lines()` run in step.
pub fn code_only(ctx: &LintContext) -> String {
    let mut bytes = ctx.source().as_bytes().to_vec();
    let mut cursor = ctx.tree().walk();
    'walk: loop {
        let node = cursor.node();
        if BLANKED.contains(&node.kind()) {
            for b in &mut bytes[node.start_byte() .. node.end_byte()] {
                if *b != b'\n' && *b != b'\r' {
                    *b = b' ';
                }
            }
        } else if cursor.goto_first_child() {
            continue;
        }
        loop {
            if cursor.goto_next_sibling() {
                continue 'walk;
            }
            if !cursor.goto_parent() {
                break 'walk;
            }
        }
    }
    // Every blanked range is a whole node, so it starts and ends on a char
    // boundary and every byte inside it was replaced by an ASCII space.
    String::from_utf8(bytes).expect("a blanked node range is a whole number of chars")
}

#[cfg(test)]
mod tests {
    use mockspace::testkit::LintFixture;

    use super::code_only;

    fn blanked(source: &str) -> String {
        code_only(&LintFixture::new(source).ctx())
    }

    #[test]
    fn a_trailing_comment_is_blanked_and_the_code_before_it_is_not() {
        let out = blanked("let n = 1; // a Box<T> here\n");
        assert_eq!(out, "let n = 1;                 \n");
    }

    #[test]
    fn a_doc_comment_is_blanked() {
        assert!(blanked("/// a `Vec<T>` here\nfn f() {}\n").starts_with("                  "));
        assert!(!blanked("//! a `String` here\n").contains("String"));
    }

    #[test]
    fn every_interior_line_of_a_block_comment_is_blanked_and_the_breaks_stay() {
        let out = blanked("/*\nuse std::fmt;\n*/\nfn f() {}\n");
        assert!(!out.contains("std"));
        assert_eq!(out.lines().count(), 4);
        assert_eq!(out.lines().nth(3), Some("fn f() {}"));
    }

    #[test]
    fn a_string_literal_is_blanked() {
        let out = blanked("let msg = \"expected a Vec<T> here\";\n");
        assert!(!out.contains("Vec"));
        assert!(out.starts_with("let msg = "));
        assert!(out.trim_end().ends_with(';'));
    }

    #[test]
    fn a_raw_multi_line_string_is_blanked_on_every_line() {
        let out = blanked("let example = r#\"\nuse std::fmt;\n\"#;\n");
        assert!(!out.contains("std"));
        assert_eq!(out.lines().count(), 3);
    }

    #[test]
    fn a_char_literal_is_blanked() {
        assert!(!blanked("let c = '\"';\n").contains('\''));
    }

    /// The control for the char literal arm: a lifetime is a different node, so
    /// the code between two lifetime ticks on one line is still code. The line
    /// scanner this replaces stripped from the second tick to the third.
    #[test]
    fn lifetimes_are_not_char_literals() {
        let source = "pub struct S<'a, 'b>(&'a Vec<u8>, &'b Bool);\n";
        assert_eq!(blanked(source), source);
    }

    #[test]
    fn a_multi_byte_character_in_a_blanked_node_keeps_the_length() {
        let source = "let s = \"héllo\"; // ünï\nfn f() {}\n";
        let out = blanked(source);
        assert_eq!(out.len(), source.len());
        assert!(out.ends_with("fn f() {}\n"));
    }

    #[test]
    fn code_alone_comes_back_unchanged() {
        let source = "fn f(x: Bool) -> Bool {\n    x\n}\n";
        assert_eq!(blanked(source), source);
    }

    #[test]
    fn an_empty_source_is_empty() {
        assert_eq!(blanked(""), "");
    }

    /// A source the grammar cannot place still yields a tree, and the
    /// comment in it is still a comment.
    #[test]
    fn a_comment_in_an_unparseable_source_is_still_blanked() {
        let out = blanked("fn f( { // Box<T>\n");
        assert!(!out.contains("Box"));
    }
}
