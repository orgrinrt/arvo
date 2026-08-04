# Spectral bisection: Fiedler vector + sign-cut partition

1 variants, 40 samples per variant.
Baseline: **fiedler-bisect-dense**

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fiedler-bisect-dense | 88055ns | 88131ns | 84748ns | 88067ns | 91325ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| fiedler-bisect-dense | 87840ns | 84535ns | 91071ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fiedler-bisect-dense | 88055ns | 88055ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fiedler-bisect-dense | 87936ns | base | --- | [87564, 88371] | --- | --- | --- | --- |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | fiedler-bisect-dense |
|---|---|
| 1 | 84203ns |
| 2 | 87642ns |
| 3 | 89501ns |
| 4 | 85010ns |
| 5 | 84716ns |
| 6 | 84140ns |
| 7 | 83832ns |
| 8 | 89118ns |
| 9 | 84066ns |
| 10 | 87832ns |
| 11 | 88081ns |
| 12 | 91738ns |
| 13 | 88415ns |
| 14 | 87680ns |
| 15 | 92197ns |
| 16 | 88383ns |
| 17 | 88058ns |
| 18 | 85923ns |
| 19 | 85801ns |
| 20 | 87503ns |
| 21 | 91750ns |
| 22 | 88140ns |
| 23 | 92329ns |
| 24 | 88325ns |
| 25 | 89275ns |
| 26 | 84593ns |
| 27 | 87560ns |
| 28 | 87575ns |
| 29 | 88041ns |
| 30 | 91208ns |
| 31 | 85719ns |
| 32 | 88570ns |
| 33 | 88359ns |
| 34 | 86978ns |
| 35 | 87425ns |
| 36 | 88684ns |
| 37 | 87804ns |
| 38 | 89353ns |
| 39 | 90490ns |
| 40 | 87568ns |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fiedler-bisect-dense | 0.129 | ok |

**Consistency summary:**


## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fiedler-bisect-dense | 2.8ns | 87839.6ns | 0.0% |  |

## Distribution (algo ns)

```
fiedler-bisect-dense (n=40, range 84534.8-91070.8 ns)
  84534.8 |#############
  84861.6 |######
  85188.4 |
  85515.2 |#############
  85842.0 |######
  86168.8 |
  86495.6 |
  86822.4 |######
  87149.2 |######
  87476.0 |########################################
  87802.8 |#################################
  88129.6 |#################################
  88456.4 |#############
  88783.2 |
  89110.0 |####################
  89436.8 |######
  89763.6 |
  90090.4 |
  90417.2 |######
  90744.0 |
  (4 below, 5 above range)

```
