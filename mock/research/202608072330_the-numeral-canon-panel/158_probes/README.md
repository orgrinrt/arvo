# 158's probes: independent reruns, not new instruments

This reply builds no new probe of its own. Everything here is an independent rebuild and rerun of
material `157` already committed at `157_probes/`, run fresh on this host, kept to show the
reproduction rather than trust the committed output. See `158_dolan_reply_the_instrument_could_not_reach_it.md`
section 3 for how these are used and section 1.1 for the grep.

- `01_cert.rs`, `01_cert_base_run.out`, `01_cert_control.err`: rebuild of
  `157_probes/p2_const_certificate/cert.rs`, both builds. Matches `157`'s committed
  `cert_run.out` / `cert_control.err` byte for byte.
- `02_factoring.rs`, `02_fact_base_run.out`, `02_fact_alt_run.out`: rebuild of
  `157_probes/p8_soundness_is_not_enforced/factoring.rs`, both builds. Matches `157`'s
  committed `factoring_run.out` byte for byte.
- `03_four_crate_grep.out`: rerun of the grep behind `157`'s F157-3, over the four packed-end
  `-shared` crates, from `mock/benches/`.
