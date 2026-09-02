# Graph structural decomposition: RCM reorder

1 variants, 40 samples per variant.
Baseline: **rcm-bits64**

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rcm-bits64 | 677ns | 506ns | 470ns | 577ns | 1185ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| rcm-bits64 | 577ns | 397ns | 1010ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm-bits64 | 677ns | 677ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm-bits64 | 431ns | base | --- | [419, 505] | --- | --- | --- | --- |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | rcm-bits64 |
|---|---|
| 1 | 427ns |
| 2 | 431ns |
| 3 | 427ns |
| 4 | 447ns |
| 5 | 395ns |
| 6 | 423ns |
| 7 | 429ns |
| 8 | 432ns |
| 9 | 419ns |
| 10 | 395ns |
| 11 | 958ns |
| 12 | 971ns |
| 13 | 974ns |
| 14 | 968ns |
| 15 | 960ns |
| 16 | 960ns |
| 17 | 958ns |
| 18 | 918ns |
| 19 | 418ns |
| 20 | 412ns |
| 21 | 1165ns |
| 22 | 1121ns |
| 23 | 500ns |
| 24 | 510ns |
| 25 | 492ns |
| 26 | 491ns |
| 27 | 518ns |
| 28 | 533ns |
| 29 | 515ns |
| 30 | 491ns |
| 31 | 401ns |
| 32 | 400ns |
| 33 | 419ns |
| 34 | 395ns |
| 35 | 400ns |
| 36 | 404ns |
| 37 | 393ns |
| 38 | 395ns |
| 39 | 398ns |
| 40 | 402ns |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm-bits64 | 0.676 | HIGH+ (drift/warm-up) |

**Consistency summary:**


## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm-bits64 | 2.1ns | 576.6ns | 0.4% |  |

## Distribution (algo ns)

```
rcm-bits64 (n=40, range 396.5-1009.7 ns)
    396.5 |########################################
    427.2 |############
    457.8 |
    488.5 |#####################
    519.1 |###
    549.8 |
    580.5 |
    611.1 |
    641.8 |
    672.4 |
    703.1 |
    733.7 |
    764.4 |
    795.1 |
    825.7 |
    856.4 |
    887.0 |
    917.7 |###
    948.4 |#####################
    979.0 |
  (5 below, 2 above range)

```

## Diagnostics

- **rcm-bits64**: CV=43.0% (high variance, measurements may be unstable)
- **rcm-bits64**: autocorrelation=0.68 (measurement drift or warm-up artifact)
