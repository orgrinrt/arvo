# Graph structural decomposition: RCM reorder

1 variants, 40 samples per variant.
Baseline: **rcm-bits64**

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rcm-bits64 | 4293ns | 3403ns | 2899ns | 3701ns | 7461ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| rcm-bits64 | 4161ns | 2813ns | 7224ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm-bits64 | 4293ns | 4293ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm-bits64 | 3321ns | base | --- | [3224, 3937] | --- | --- | --- | --- |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | rcm-bits64 |
|---|---|
| 1 | 3048ns |
| 2 | 2926ns |
| 3 | 4023ns |
| 4 | 3030ns |
| 5 | 2947ns |
| 6 | 3889ns |
| 7 | 3800ns |
| 8 | 2922ns |
| 9 | 2734ns |
| 10 | 2680ns |
| 11 | 2704ns |
| 12 | 8948ns |
| 13 | 8277ns |
| 14 | 2665ns |
| 15 | 4811ns |
| 16 | 3109ns |
| 17 | 3754ns |
| 18 | 6615ns |
| 19 | 3985ns |
| 20 | 2925ns |
| 21 | 4033ns |
| 22 | 3051ns |
| 23 | 3012ns |
| 24 | 3234ns |
| 25 | 3270ns |
| 26 | 9712ns |
| 27 | 3270ns |
| 28 | 5630ns |
| 29 | 3218ns |
| 30 | 3265ns |
| 31 | 3230ns |
| 32 | 3852ns |
| 33 | 4864ns |
| 34 | 3308ns |
| 35 | 3335ns |
| 36 | 6146ns |
| 37 | 4410ns |
| 38 | 3349ns |
| 39 | 5405ns |
| 40 | 7062ns |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm-bits64 | 0.066 | ok |

**Consistency summary:**


## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm-bits64 | 3.1ns | 4161.2ns | 0.1% |  |

## Distribution (algo ns)

```
rcm-bits64 (n=40, range 2812.9-7224.3 ns)
   2812.9 |########################################
   3033.4 |########################################
   3254.0 |########################################
   3474.6 |
   3695.2 |##########################
   3915.7 |####################
   4136.3 |
   4356.9 |######
   4577.5 |
   4798.0 |#############
   5018.6 |
   5239.2 |######
   5459.7 |######
   5680.3 |
   5900.9 |
   6121.5 |######
   6342.0 |
   6562.6 |######
   6783.2 |
   7003.8 |######
  (4 below, 3 above range)

```

## Diagnostics

- **rcm-bits64**: CV=42.0% (high variance, measurements may be unstable)
