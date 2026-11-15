# Graph structural decomposition: RCM reorder

1 variants, 40 samples per variant.
Baseline: **rcm-bits64**

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rcm-bits64 | 28869ns | 28015ns | 23250ns | 27564ns | 38403ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| rcm-bits64 | 28722ns | 23135ns | 38226ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm-bits64 | 28869ns | 28869ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm-bits64 | 27886ns | base | --- | [25454, 28575] | --- | --- | --- | --- |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | rcm-bits64 |
|---|---|
| 1 | 24285ns |
| 2 | 27225ns |
| 3 | 23617ns |
| 4 | 24024ns |
| 5 | 23168ns |
| 6 | 23758ns |
| 7 | 26306ns |
| 8 | 30088ns |
| 9 | 29175ns |
| 10 | 22676ns |
| 11 | 40124ns |
| 12 | 35952ns |
| 13 | 28305ns |
| 14 | 28248ns |
| 15 | 28071ns |
| 16 | 27958ns |
| 17 | 27553ns |
| 18 | 29077ns |
| 19 | 31935ns |
| 20 | 27540ns |
| 21 | 30105ns |
| 22 | 25203ns |
| 23 | 44195ns |
| 24 | 32038ns |
| 25 | 28453ns |
| 26 | 50849ns |
| 27 | 31560ns |
| 28 | 27913ns |
| 29 | 28696ns |
| 30 | 32457ns |
| 31 | 24628ns |
| 32 | 25009ns |
| 33 | 23622ns |
| 34 | 25020ns |
| 35 | 22890ns |
| 36 | 25705ns |
| 37 | 27860ns |
| 38 | 21994ns |
| 39 | 38257ns |
| 40 | 23352ns |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm-bits64 | 0.086 | ok |

**Consistency summary:**


## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm-bits64 | 5.6ns | 28722.3ns | 0.0% |  |

## Distribution (algo ns)

```
rcm-bits64 (n=40, range 23134.6-38225.8 ns)
  23134.6 |#################################
  23889.2 |####################
  24643.7 |####################
  25398.3 |######
  26152.8 |######
  26907.4 |####################
  27662.0 |########################################
  28416.5 |####################
  29171.1 |######
  29925.7 |#############
  30680.2 |
  31434.8 |####################
  32189.3 |######
  32943.9 |
  33698.5 |
  34453.0 |
  35207.6 |######
  35962.2 |
  36716.7 |
  37471.3 |
  (3 below, 4 above range)

```

## Diagnostics

- **rcm-bits64**: CV=20.8% (high variance, measurements may be unstable)
