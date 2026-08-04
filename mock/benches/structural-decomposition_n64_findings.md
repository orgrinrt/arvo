# Graph structural decomposition: RCM reorder

1 variants, 40 samples per variant.
Baseline: **rcm-bits64**

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rcm-bits64 | 27859ns | 27318ns | 24503ns | 27436ns | 32484ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| rcm-bits64 | 27699ns | 24367ns | 32156ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm-bits64 | 27859ns | 27859ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm-bits64 | 27177ns | base | --- | [26395, 28662] | --- | --- | --- | --- |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | rcm-bits64 |
|---|---|
| 1 | 25308ns |
| 2 | 25182ns |
| 3 | 25057ns |
| 4 | 25295ns |
| 5 | 27220ns |
| 6 | 27177ns |
| 7 | 27177ns |
| 8 | 27172ns |
| 9 | 27166ns |
| 10 | 25762ns |
| 11 | 46732ns |
| 12 | 28634ns |
| 13 | 34279ns |
| 14 | 28690ns |
| 15 | 27067ns |
| 16 | 25488ns |
| 17 | 28837ns |
| 18 | 27371ns |
| 19 | 27359ns |
| 20 | 26174ns |
| 21 | 29448ns |
| 22 | 29357ns |
| 23 | 29375ns |
| 24 | 29322ns |
| 25 | 29422ns |
| 26 | 29301ns |
| 27 | 29304ns |
| 28 | 29314ns |
| 29 | 29289ns |
| 30 | 29304ns |
| 31 | 26431ns |
| 32 | 26396ns |
| 33 | 26395ns |
| 34 | 27443ns |
| 35 | 22450ns |
| 36 | 23991ns |
| 37 | 24908ns |
| 38 | 24919ns |
| 39 | 24385ns |
| 40 | 24043ns |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm-bits64 | 0.175 | ok |

**Consistency summary:**


## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm-bits64 | 2.9ns | 27698.6ns | 0.0% |  |

## Distribution (algo ns)

```
rcm-bits64 (n=40, range 24366.8-32156.3 ns)
  24366.8 |####
  24756.3 |#############
  25145.8 |#################
  25535.2 |####
  25924.7 |####
  26314.2 |#############
  26703.6 |####
  27093.1 |###################################
  27482.6 |
  27872.1 |
  28261.5 |####
  28651.0 |########
  29040.5 |########################################
  29430.0 |####
  29819.4 |
  30208.9 |
  30598.4 |
  30987.8 |
  31377.3 |
  31766.8 |
  (3 below, 2 above range)

```
