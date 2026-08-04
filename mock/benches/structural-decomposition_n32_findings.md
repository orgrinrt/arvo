# Graph structural decomposition: RCM reorder

1 variants, 40 samples per variant.
Baseline: **rcm-bits64**

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rcm-bits64 | 3264ns | 2825ns | 2717ns | 2825ns | 5127ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| rcm-bits64 | 3179ns | 2641ns | 5003ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm-bits64 | 3264ns | 3264ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm-bits64 | 2753ns | base | --- | [2690, 2804] | --- | --- | --- | --- |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | rcm-bits64 |
|---|---|
| 1 | 2652ns |
| 2 | 2662ns |
| 3 | 2692ns |
| 4 | 2681ns |
| 5 | 2711ns |
| 6 | 2745ns |
| 7 | 2664ns |
| 8 | 2688ns |
| 9 | 2720ns |
| 10 | 2826ns |
| 11 | 2658ns |
| 12 | 2633ns |
| 13 | 2651ns |
| 14 | 2654ns |
| 15 | 2625ns |
| 16 | 2626ns |
| 17 | 2655ns |
| 18 | 2663ns |
| 19 | 2635ns |
| 20 | 2738ns |
| 21 | 2775ns |
| 22 | 2855ns |
| 23 | 2824ns |
| 24 | 2798ns |
| 25 | 2740ns |
| 26 | 2769ns |
| 27 | 2803ns |
| 28 | 2805ns |
| 29 | 2782ns |
| 30 | 2815ns |
| 31 | 2947ns |
| 32 | 4315ns |
| 33 | 5162ns |
| 34 | 10884ns |
| 35 | 4923ns |
| 36 | 3674ns |
| 37 | 5265ns |
| 38 | 2853ns |
| 39 | 2813ns |
| 40 | 2760ns |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm-bits64 | 0.487 | moderate+ |

**Consistency summary:**


## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm-bits64 | 2.1ns | 3178.5ns | 0.1% |  |

## Distribution (algo ns)

```
rcm-bits64 (n=40, range 2641.4-5003.0 ns)
   2641.4 |########################################
   2759.5 |################################
   2877.6 |##
   2995.6 |
   3113.7 |
   3231.8 |
   3349.9 |
   3468.0 |
   3586.1 |##
   3704.1 |
   3822.2 |
   3940.3 |
   4058.4 |
   4176.5 |
   4294.5 |##
   4412.6 |
   4530.7 |
   4648.8 |
   4766.9 |
   4885.0 |##
  (4 below, 3 above range)

```

## Diagnostics

- **rcm-bits64**: CV=44.3% (high variance, measurements may be unstable)
