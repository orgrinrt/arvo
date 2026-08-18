# Test gate: exact commands

Host: darwin arm64. Profile: `--release` throughout.

Twelve crates, unserialised, from `mock/benches/variants`:

```sh
for c in bitpack-carrier-shared bitpack-contend-shared bitpack-footprint-shared \
         bitpack-plan-shared bitpack-shared bitpack-wide-shared \
         quantiser-fadd-shared quantiser-radix-shared satfold-shared \
         warm-clamp-shared warm-container-shared wide-rung-shared; do
  (cd "$c" && cargo test --release)
done
```

Output: `twelve_crates_release.log`. Total 108 passed, 0 failed.

The thirteenth, serialised:

```sh
(cd bitpack-write-contend-shared && cargo test --release -- --test-threads=1)
```

Output: `write_contend_serialised_release.log`. 15 passed, 0 failed, finished in 2.25s.

Grand total 123 across 13.

The tautology scan (eighteen assertion-free bodies, all delegating) was a python walk over every
`#[test]` block in the thirteen crates; four of the eighteen were opened by hand and confirmed to
delegate to an asserting helper.
