# Test gate for 175

Host: Apple M1. Profile `--release` throughout. Run from `mock/benches/variants`.

```sh
for c in bitpack-carrier-shared bitpack-contend-shared bitpack-footprint-shared \
         bitpack-plan-shared bitpack-shared bitpack-wide-shared \
         quantiser-fadd-shared quantiser-radix-shared satfold-shared \
         warm-clamp-shared warm-container-shared wide-rung-shared; do
  (cd "$c" && cargo test --release)
done
(cd bitpack-write-contend-shared && cargo test --release -- --test-threads=1)
```

108 across twelve, 15 serialised (finished in 2.43s), **123 across 13, 0 failed, 0 ignored**.
Raw log in `thirteen_crates_release.log`. Third independent confirmation of the thirteenth crate's
count, and the second from a seat other than `174`'s.
