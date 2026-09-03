//! E. Can a reasoned claim about the canon's own contents state its region?
//!
//! Asked of the shipped checkers rather than of the corpus. The three lints
//! that decide it are pulled into the library crate beside this by `#[path]`
//! from `mock/lints/`, the same way the engine's generated pack pulls them, so
//! what runs below is the lint code itself and not a restatement of it.
//!
//! The claim used throughout is the layering derivation already in the registry
//! as `proposal::the_topics_form_a_stack_a_frame_and_the_canons_own_machinery`,
//! whose own `note` says it was "filed `normative` after being written
//! `argument`" because "no predicate, because none in this registry can express
//! it".
//!
//! Every prediction is asserted rather than printed, so a run in which a lint
//! stops firing fails rather than reading as agreement.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use arvo_259_lintdrive::{
    a_region_agrees_with_the_sentence_kind,
    an_imposition_rests_on_no_instrument,
    canon_rows,
    every_predicate_names_a_declared_axis,
};
use canon_rows::JOIN;
use mockspace::{RegistryView, RepoContext, RepoLint, RowFields};

/// A registry holding exactly the rows named.
///
/// The construction `mock/lints/canon_lint_testkit.rs:52` uses, rebuilt here
/// because that testkit also reaches for `crate::__mockspace_collect_lints`,
/// which only the engine's generated pack has.
fn view_owned(rows: &[(String, Vec<(String, String)>)]) -> RegistryView {
    let mut map: BTreeMap<String, RowFields> = BTreeMap::new();
    for (q, fields) in rows {
        map.insert(q.clone(), fields.iter().cloned().collect());
    }
    RegistryView::new(map, BTreeMap::new())
}

fn ctx(registry: &RegistryView) -> RepoContext<'_> {
    static HERE: OnceLock<PathBuf> = OnceLock::new();
    static CRATES: OnceLock<BTreeSet<String>> = OnceLock::new();
    static DIRS: OnceLock<Vec<PathBuf>> = OnceLock::new();
    static STRINGS: OnceLock<Vec<String>> = OnceLock::new();
    let here: &Path = HERE.get_or_init(|| PathBuf::from("."));
    RepoContext {
        mock_dir: here,
        repo_root: here,
        all_crates: CRATES.get_or_init(BTreeSet::new),
        src_dirs: DIRS.get_or_init(Vec::new),
        invocation: None,
        canon_paths: STRINGS.get_or_init(Vec::new),
        open_panels: STRINGS.get_or_init(Vec::new),
        registry,
    }
}

/// Every finding one lint reports, as `(finding kind, message)`.
fn run(lint: &dyn RepoLint, reg: &RegistryView) -> Vec<(String, String)> {
    lint.check_repo(&ctx(reg))
        .into_iter()
        .map(|e| {
            (
                e.finding_kind.unwrap_or("<none>").to_string(),
                e.message.clone(),
            )
        })
        .collect()
}

/// The 25 declared axes.
///
/// Read off `mock/registry/dimension.toml` at this base. The control
/// `the_declared_axis_list_here_matches_the_registry` re-reads that file and
/// fails if the two ever differ, so this list cannot go stale silently.
const AXES: [&str; 25] = [
    "integer_width",
    "fraction_width",
    "total_width",
    "signedness",
    "overflow_policy",
    "rounding",
    "operation",
    "arity",
    "chain_length",
    "container",
    "alignment",
    "access_pattern",
    "target_features",
    "threads",
    "strategy",
    "ambient_domain",
    "radix",
    "accumulator_width",
    "toolchain",
    "build_profile",
    "operand_window",
    "occupancy",
    "association",
    "leaf_aliasing",
    "phase",
];

/// The 25 axes as `dimension` rows, plus one planted proposal.
fn with_axes(id: &str, fields: &[(&str, &str)]) -> Vec<(String, Vec<(String, String)>)> {
    let mut rows: Vec<(String, Vec<(String, String)>)> = AXES
        .iter()
        .map(|a| {
            (format!("dimension::{a}"), vec![(
                "what".to_string(),
                "an axis".to_string(),
            )])
        })
        .collect();
    rows.push((
        id.to_string(),
        fields
            .iter()
            .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
            .collect(),
    ));
    rows
}

fn region_lint() -> Box<dyn RepoLint> {
    a_region_agrees_with_the_sentence_kind::repo_lint()
}
fn axis_lint() -> Box<dyn RepoLint> {
    every_predicate_names_a_declared_axis::repo_lint()
}
fn instrument_lint() -> Box<dyn RepoLint> {
    an_imposition_rests_on_no_instrument::repo_lint()
}

/// The layering claim's own words, so every arm plants one sentence and only
/// the filing differs.
const SAYS: &str = "The twenty topics are not one order; eleven form a stack, four a frame, \
                    four the canon's own machinery.";
const BECAUSE: &str = "Derived from the `what` sentence of each topic row by one test: can \
                       this topic's subject be stated at all without a decision belonging to \
                       that one.";

// -------------------------------------------------------------------------
// Controls. A run in which these do not hold says nothing about the six
// candidates, so they are asserted rather than reported.
// -------------------------------------------------------------------------

/// L1. The axis list here is the axis list the registry declares.
#[test]
fn control_the_declared_axis_list_here_matches_the_registry() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../registry/dimension.toml"
    );
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("cannot read {path}: {e}"));
    let declared: Vec<&str> = text
        .lines()
        .filter_map(|l| l.strip_prefix("id = \""))
        .filter_map(|l| l.strip_suffix('"'))
        .collect();
    assert_eq!(
        declared.len(),
        AXES.len(),
        "the registry declares {} axes and this probe carries {}: {declared:?}",
        declared.len(),
        AXES.len()
    );
    for a in AXES {
        assert!(
            declared.contains(&a),
            "`{a}` is not declared in the registry"
        );
    }
}

/// L2. A correctly formed numeric row draws nothing from any of the three.
///
/// Without it, a run reporting every candidate cannot be told apart from three
/// lints that report everything.
#[test]
fn control_an_ordinary_numeric_row_draws_nothing() {
    let region = ["fraction_width: F = 0", "threads: threads = 1"].join(JOIN);
    let rows = with_axes("proposal::an_ordinary_numeric_row", &[
        ("sentence_kind", "argument"),
        ("says", SAYS),
        ("predicate", &region),
    ]);
    let v = view_owned(&rows);
    for l in [region_lint(), axis_lint(), instrument_lint()] {
        assert!(run(l.as_ref(), &v).is_empty(), "{:?}", run(l.as_ref(), &v));
    }
}

/// L3. Each lint fires on the breach it was written for.
#[test]
fn control_each_lint_fires_on_its_own_canonical_breach() {
    let rows = with_axes("proposal::a_theorem_with_no_region", &[(
        "sentence_kind",
        "theorem",
    )]);
    assert_eq!(run(region_lint().as_ref(), &view_owned(&rows)).len(), 1);

    let rows = with_axes("proposal::an_undeclared_axis", &[
        ("sentence_kind", "argument"),
        ("predicate", "no_such_axis: anything"),
    ]);
    assert_eq!(run(axis_lint().as_ref(), &view_owned(&rows)).len(), 1);

    let rows = with_axes("proposal::an_imposition_with_evidence", &[
        ("sentence_kind", "normative"),
        ("evidence", "some_probe"),
    ]);
    assert_eq!(run(instrument_lint().as_ref(), &view_owned(&rows)).len(), 1);
}

// -------------------------------------------------------------------------
// The candidates.
// -------------------------------------------------------------------------

/// D1. Reasoned, and no region. The state the layering row was in.
#[test]
fn d1_argument_with_no_region_is_refused_as_claiming_the_whole_space() {
    let rows = with_axes("proposal::layering_as_argument_no_region", &[
        ("sentence_kind", "argument"),
        ("says", SAYS),
        ("because", BECAUSE),
    ]);
    let f = run(region_lint().as_ref(), &view_owned(&rows));
    assert_eq!(f.len(), 1, "{f:?}");
    assert_eq!(f[0].0, "an-established-claim-carries-no-region", "{f:?}");
}

/// D2. The region written in the vocabulary the subject actually has.
///
/// Two entries, two undeclared axes, two findings. This is the arm that says
/// the dimension vocabulary is closed against a structural coordinate.
#[test]
fn d2_a_structural_axis_is_refused_because_nothing_declares_it() {
    let region = [
        "registry_state: the twenty topic rows as they stand at this commit",
        "namespace: namespace = topic",
    ]
    .join(JOIN);
    let rows = with_axes("proposal::layering_over_structural_axes", &[
        ("sentence_kind", "argument"),
        ("says", SAYS),
        ("because", BECAUSE),
        ("predicate", &region),
    ]);
    let f = run(axis_lint().as_ref(), &view_owned(&rows));
    assert_eq!(f.len(), 2, "{f:?}");
    for e in &f {
        assert_eq!(e.0, "undeclared-axis", "{e:?}");
    }
}

/// D3. The whole space on every declared axis, tokenless.
///
/// The arm the question turns on. Under
/// `ruling::the_warrant_is_a_token_and_a_clause_on_the_values_side`, "An entry
/// carrying no token claims no warrant", so this claims the widest region on
/// each axis and claims nothing about how it was earned.
///
/// If it is silent, a structural claim's region is expressible today and the
/// question's premise is about idiom rather than about expressibility.
#[test]
fn d3_every_declared_axis_at_any_is_accepted_by_every_lint() {
    let entries: Vec<String> = AXES.iter().map(|a| format!("{a}: {a} any")).collect();
    let region = entries.join(JOIN);
    let rows = with_axes("proposal::layering_over_every_axis_at_any", &[
        ("sentence_kind", "argument"),
        ("says", SAYS),
        ("because", BECAUSE),
        ("predicate", &region),
    ]);
    let v = view_owned(&rows);
    for l in [region_lint(), axis_lint(), instrument_lint()] {
        let f = run(l.as_ref(), &v);
        assert!(
            f.is_empty(),
            "{} reported the universal spelling: {f:?}",
            l.name()
        );
    }
}

/// D4. The filing the row took, plus the instrument it ran.
///
/// What option two of the open question costs: a structural claim filed
/// `normative` may not name the instrument that produced it.
#[test]
fn d4_an_imposed_structural_claim_may_not_cite_its_instrument() {
    let rows = with_axes("proposal::layering_as_normative_with_evidence", &[
        ("sentence_kind", "normative"),
        ("says", SAYS),
        ("because", BECAUSE),
        ("evidence", "a_probe_that_walked_the_topic_rows"),
    ]);
    let f = run(instrument_lint().as_ref(), &view_owned(&rows));
    assert_eq!(f.len(), 1, "{f:?}");
    assert!(f[0].1.contains("normative"), "{f:?}");
}

/// D5. Keeping both the imposed kind and a region.
#[test]
fn d5_an_imposed_structural_claim_may_not_carry_a_region_either() {
    let rows = with_axes("proposal::layering_as_normative_with_a_region", &[
        ("sentence_kind", "normative"),
        ("says", SAYS),
        ("because", BECAUSE),
        ("predicate", "threads: threads = 1"),
    ]);
    let f = run(region_lint().as_ref(), &view_owned(&rows));
    assert_eq!(f.len(), 1, "{f:?}");
    assert_eq!(f[0].0, "an-imposed-proposition-carries-a-region", "{f:?}");
}

/// D6. The shape the four region-bearing machinery rows actually use.
///
/// Two of them write `threads: threads = 1` and nothing else; two write the
/// numeric region of the claims they are talking about. Every one passes. The
/// arm exists to show that passing is not the same as being right: no checker
/// here can tell a region belonging to the claim from one belonging to the run
/// or to somebody else's claim.
#[test]
fn d6_a_borrowed_region_passes_every_lint() {
    for borrowed in ["threads: threads = 1", "total_width: W in 3..=7", "fraction_width: F = 0"] {
        let rows = with_axes("proposal::layering_with_a_borrowed_region", &[
            ("sentence_kind", "argument"),
            ("says", SAYS),
            ("because", BECAUSE),
            ("predicate", borrowed),
        ]);
        let v = view_owned(&rows);
        for l in [region_lint(), axis_lint(), instrument_lint()] {
            let f = run(l.as_ref(), &v);
            assert!(f.is_empty(), "{} on `{borrowed}`: {f:?}", l.name());
        }
    }
}

/// D7. The region obligation over every declared sentence kind.
///
/// The whole matrix rather than the two kinds the question names, because a law
/// asserted at some of its shapes is a law nobody checked.
#[test]
fn d7_every_declared_sentence_kind_against_the_region_obligation() {
    const KINDS: [(&str, bool); 6] = [
        ("theorem", true),
        ("measured", true),
        ("enumeration", true),
        ("definition", false),
        ("normative", false),
        ("argument", true),
    ];
    for (kind, owes_a_region) in KINDS {
        let bare = with_axes("proposal::p", &[("sentence_kind", kind), ("says", SAYS)]);
        let f = run(region_lint().as_ref(), &view_owned(&bare));
        if owes_a_region {
            assert_eq!(
                f.len(),
                1,
                "`{kind}` with no region should be reported: {f:?}"
            );
        } else {
            assert!(f.is_empty(), "`{kind}` owes no region: {f:?}");
        }

        let regioned = with_axes("proposal::p", &[
            ("sentence_kind", kind),
            ("says", SAYS),
            ("predicate", "threads: threads = 1"),
        ]);
        let g = run(region_lint().as_ref(), &view_owned(&regioned));
        if owes_a_region {
            assert!(g.is_empty(), "`{kind}` with a region should pass: {g:?}");
        } else {
            assert_eq!(g.len(), 1, "`{kind}` may not carry a region: {g:?}");
        }
    }
}

/// D8. A structural claim's region cannot be narrowed on any declared axis.
///
/// The point of a predicate is that a reader can gate on it. This arm asks
/// whether narrowing any single axis away from `any` changes what the checker
/// says about a claim whose subject contains no numeral. It does not: all 25
/// narrowings pass exactly as the universal one does, so the notation gives a
/// structural claim 25 axes it may set to anything at all, and no way to be
/// wrong about any of them.
#[test]
fn d8_narrowing_any_single_axis_is_equally_acceptable_to_the_checker() {
    for pinned in AXES {
        let entries: Vec<String> = AXES
            .iter()
            .map(|a| {
                if *a == pinned {
                    format!("{a}: {a} = the one value this claim was never observed at")
                } else {
                    format!("{a}: {a} any")
                }
            })
            .collect();
        let rows = with_axes("proposal::layering_with_one_axis_pinned", &[
            ("sentence_kind", "argument"),
            ("says", SAYS),
            ("because", BECAUSE),
            ("predicate", &entries.join(JOIN)),
        ]);
        let v = view_owned(&rows);
        for l in [region_lint(), axis_lint(), instrument_lint()] {
            let f = run(l.as_ref(), &v);
            assert!(
                f.is_empty(),
                "{} on a claim pinned at `{pinned}`: {f:?}",
                l.name()
            );
        }
    }
}
