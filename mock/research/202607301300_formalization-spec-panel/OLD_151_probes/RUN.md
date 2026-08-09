# How every number in 151 was produced

Pin: nightly-2026-05-28. Verified with:
    $ rustc +nightly-2026-05-28 --version
    rustc 1.98.0-nightly (57d06900f 2026-05-27)

Every command below was run from this directory. Exit codes are recorded because two of
the Rust probes are EXPECTED to fail, and a reader has to be able to tell an expected
refusal from a broken probe.

    $ python3 sign_domain.py > sign_domain.out
      exit 0  (expected 0)
    $ python3 sign_domain2.py > sign_domain2.out
      exit 0  (expected 0)
    $ python3 sign_domain3.py > sign_domain3.out
      exit 0  (expected 0)
    $ python3 sign_domain4.py > sign_domain4.out
      exit 0  (expected 0)

    $ rustc +nightly-2026-05-28 --edition 2024 --crate-type=lib p1_target_keyed.rs --out-dir out
      exit 0  (expected 0), diagnostics in p1.out
    $ rustc +nightly-2026-05-28 --edition 2024 --crate-type=lib p2_adjudicator_as_free_key.rs --out-dir out
      exit 1  (expected NONZERO, the refusal is the finding), diagnostics in p2.out
    $ rustc +nightly-2026-05-28 --edition 2024 --crate-type=lib p3_adjudicator_derived.rs --out-dir out
      exit 0  (expected 0), diagnostics in p3.out
    $ rustc +nightly-2026-05-28 --edition 2024 --crate-type=lib p4_two_rules.rs --out-dir out
      exit 1  (expected NONZERO, the refusal is the finding), diagnostics in p4.out
    $ rustc +nightly-2026-05-28 --edition 2024 --crate-type=lib p5_domain_derives_range.rs --out-dir out
      exit 0  (expected 0), diagnostics in p5.out

    $ rustc +nightly-2026-05-28 --edition 2024 p6_run_checks.rs --out-dir out
      exit 0  (expected 0, three warnings about #![no_std] in a non-root module)
    $ ./out/p6_run_checks > p6_run.out
      exit 0  (expected 0)

The counts in section 2.2 are in counts.out, with the grep that produced each and a note
recording that a first version of the possessive pattern reported a false zero.
