# Graph structural decomposition: RCM reorder

1 variants, 40 samples per variant.
Baseline: **rcm-bits64**

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rcm-bits64 | 654ns | 480ns | 431ns | 534ns | 1235ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| rcm-bits64 | 553ns | 363ns | 1035ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm-bits64 | 654ns | 654ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm-bits64 | 410ns | base | --- | [376, 442] | --- | --- | --- | --- |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | rcm-bits64 |
|---|---|
| 1 | 365ns |
| 2 | 383ns |
| 3 | 379ns |
| 4 | 375ns |
| 5 | 368ns |
| 6 | 363ns |
| 7 | 373ns |
| 8 | 383ns |
| 9 | 375ns |
| 10 | 401ns |
| 11 | 360ns |
| 12 | 356ns |
| 13 | 371ns |
| 14 | 370ns |
| 15 | 362ns |
| 16 | 358ns |
| 17 | 371ns |
| 18 | 377ns |
| 19 | 382ns |
| 20 | 372ns |
| 21 | 420ns |
| 22 | 460ns |
| 23 | 443ns |
| 24 | 430ns |
| 25 | 442ns |
| 26 | 442ns |
| 27 | 431ns |
| 28 | 456ns |
| 29 | 434ns |
| 30 | 435ns |
| 31 | 995ns |
| 32 | 1175ns |
| 33 | 1004ns |
| 34 | 1007ns |
| 35 | 1012ns |
| 36 | 1013ns |
| 37 | 1024ns |
| 38 | 1020ns |
| 39 | 1010ns |
| 40 | 1019ns |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm-bits64 | 0.896 | HIGH+ (drift/warm-up) |

**Consistency summary:**


## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm-bits64 | 1.9ns | 553.0ns | 0.4% |  |

## Distribution (algo ns)

```
rcm-bits64 (n=40, range 362.8-1035.0 ns)
    362.8 |########################################
    396.4 |########
    430.0 |#####################
    463.6 |
    497.2 |
    530.8 |
    564.4 |
    598.1 |
    631.7 |
    665.3 |
    698.9 |
    732.5 |
    766.1 |
    799.7 |
    833.3 |
    866.9 |
    900.5 |
    934.1 |
    967.7 |##
   1001.3 |#####################
  (4 below, 1 above range)

```

## Diagnostics

- **rcm-bits64**: CV=50.1% (high variance, measurements may be unstable)
- **rcm-bits64**: autocorrelation=0.90 (measurement drift or warm-up artifact)
