# Test gate for 171: exact commands

Host: Apple M1, darwin. Profile: `--release` throughout. Run from `mock/benches/variants`.

```sh
for c in bitpack-carrier-shared bitpack-contend-shared bitpack-footprint-shared \
         bitpack-plan-shared bitpack-shared bitpack-wide-shared \
         quantiser-fadd-shared quantiser-radix-shared satfold-shared \
         warm-clamp-shared warm-container-shared wide-rung-shared; do
  (cd "$c" && cargo test --release)
done
(cd bitpack-write-contend-shared && cargo test --release -- --test-threads=1)
```

Raw output in `thirteen_crates_release.log`. 108 across twelve, 15 serialised, 123 total, 0 failed.
The serialised crate finished in 2.02s.
