# Spectral bisection: Fiedler vector + sign-cut partition

1 variants, 40 samples per variant.
Baseline: **fiedler-bisect-dense**

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fiedler-bisect-dense | 34717ns | 32460ns | 25506ns | 32442ns | 50755ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| fiedler-bisect-dense | 34475ns | 25355ns | 50429ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fiedler-bisect-dense | 34717ns | 34717ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fiedler-bisect-dense | 32272ns | base | --- | [27448, 36735] | --- | --- | --- | --- |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | fiedler-bisect-dense |
|---|---|
| 1 | 25350ns |
| 2 | 25632ns |
| 3 | 41245ns |
| 4 | 33712ns |
| 5 | 33155ns |
| 6 | 52472ns |
| 7 | 26021ns |
| 8 | 37876ns |
| 9 | 33002ns |
| 10 | 25138ns |
| 11 | 43318ns |
| 12 | 45443ns |
| 13 | 25805ns |
| 14 | 27340ns |
| 15 | 30784ns |
| 16 | 32501ns |
| 17 | 28278ns |
| 18 | 32043ns |
| 19 | 36390ns |
| 20 | 27555ns |
| 21 | 37080ns |
| 22 | 44178ns |
| 23 | 46436ns |
| 24 | 47802ns |
| 25 | 56567ns |
| 26 | 27932ns |
| 27 | 38821ns |
| 28 | 54418ns |
| 29 | 56115ns |
| 30 | 25335ns |
| 31 | 25149ns |
| 32 | 25641ns |
| 33 | 25343ns |
| 34 | 29958ns |
| 35 | 34243ns |
| 36 | 25893ns |
| 37 | 37562ns |
| 38 | 25254ns |
| 39 | 25648ns |
| 40 | 26575ns |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fiedler-bisect-dense | 0.263 | moderate+ |

**Consistency summary:**


## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fiedler-bisect-dense | 5.7ns | 34475.2ns | 0.0% |  |

## Distribution (algo ns)

```
fiedler-bisect-dense (n=40, range 25355.1-50428.9 ns)
  25355.1 |########################################
  26608.8 |###########
  27862.5 |###########
  29116.2 |#####
  30369.9 |#####
  31623.6 |###########
  32877.3 |#################
  34131.0 |#####
  35384.6 |#####
  36638.3 |#################
  37892.0 |#####
  39145.7 |
  40399.4 |#####
  41653.1 |
  42906.8 |#####
  44160.5 |#####
  45414.2 |###########
  46667.9 |#####
  47921.6 |
  49175.3 |
  (6 below, 4 above range)

```

## Diagnostics

- **fiedler-bisect-dense**: CV=27.7% (high variance, measurements may be unstable)
