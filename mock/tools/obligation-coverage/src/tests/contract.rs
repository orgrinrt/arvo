use super::*;

// --- the contract it declares about itself ------------------------------------

#[test]
fn it_declares_itself_as_the_shape_it_is_and_no_run_returns_a_blocking_finding() {
    // The contract's own enforcement on a `no-failing-case` tool: a run may not
    // return a finding that blocks a gate. Driven over the registries above
    // rather than asserted about the declaration alone, because the declaration
    // is what the tool says and the outcome is what it does.
    assert!(matches!(
        ObligationCoverage.not_a_lint(),
        mockspace::tool::NotALint::NoFailingCase
    ));
    for v in [
        view(&[]),
        alone(),
        ruling_at("ratified"),
        ruling_at("stated"),
        ruling_at("open"),
        stamped_by("ratified"),
        stamped_by("stated"),
        unstamped(),
        retired(),
    ] {
        let (outcome, text) = run(&v, &[]);
        assert!(
            !matches!(outcome, Outcome::Findings(_)),
            "a no-failing-case tool returned findings: {text}"
        );
    }
}

#[test]
fn it_answers_to_the_name_the_subcommand_uses() {
    assert_eq!(ObligationCoverage.name(), "obligation-coverage");
}
