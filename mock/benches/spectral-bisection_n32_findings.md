# Spectral bisection: Fiedler vector + sign-cut partition

1 variants, 40 samples per variant.
Baseline: **fiedler-bisect-dense**

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fiedler-bisect-dense | 22938ns | 23146ns | 21802ns | 23123ns | 23518ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| fiedler-bisect-dense | 22804ns | 21673ns | 23372ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fiedler-bisect-dense | 22938ns | 22938ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fiedler-bisect-dense | 23021ns | base | --- | [22993, 23050] | --- | --- | --- | --- |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | fiedler-bisect-dense |
|---|---|
| 1 | 22993ns |
| 2 | 23057ns |
| 3 | 23045ns |
| 4 | 23106ns |
| 5 | 23652ns |
| 6 | 23035ns |
| 7 | 23042ns |
| 8 | 23168ns |
| 9 | 23128ns |
| 10 | 23464ns |
| 11 | 23007ns |
| 12 | 23025ns |
| 13 | 22997ns |
| 14 | 23282ns |
| 15 | 23470ns |
| 16 | 23459ns |
| 17 | 22068ns |
| 18 | 21525ns |
| 19 | 21708ns |
| 20 | 21628ns |
| 21 | 21472ns |
| 22 | 21548ns |
| 23 | 23326ns |
| 24 | 22968ns |
| 25 | 23154ns |
| 26 | 23092ns |
| 27 | 22976ns |
| 28 | 22965ns |
| 29 | 22966ns |
| 30 | 23039ns |
| 31 | 23020ns |
| 32 | 23022ns |
| 33 | 23056ns |
| 34 | 22998ns |
| 35 | 22997ns |
| 36 | 21938ns |
| 37 | 21501ns |
| 38 | 22129ns |
| 39 | 22992ns |
| 40 | 23128ns |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fiedler-bisect-dense | 0.687 | HIGH+ (drift/warm-up) |

**Consistency summary:**


## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fiedler-bisect-dense | 2.1ns | 22803.7ns | 0.0% |  |

## Distribution (algo ns)

```
fiedler-bisect-dense (n=40, range 21673.3-23371.8 ns)
  21673.3 |###
  21758.2 |
  21843.2 |
  21928.1 |###
  22013.0 |###
  22097.9 |###
  22182.9 |
  22267.8 |
  22352.7 |
  22437.6 |
  22522.6 |
  22607.5 |
  22692.4 |
  22777.3 |
  22862.2 |
  22947.2 |########################################
  23032.1 |########################
  23117.0 |############
  23201.9 |###
  23286.9 |###
  (5 below, 4 above range)

```

## Diagnostics

- **fiedler-bisect-dense**: autocorrelation=0.69 (measurement drift or warm-up artifact)
