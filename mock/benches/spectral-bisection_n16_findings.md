# Spectral bisection: Fiedler vector + sign-cut partition

1 variants, 40 samples per variant.
Baseline: **fiedler-bisect-dense**

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fiedler-bisect-dense | 5771ns | 5692ns | 5631ns | 5721ns | 6060ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| fiedler-bisect-dense | 5668ns | 5532ns | 5953ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fiedler-bisect-dense | 5771ns | 5771ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fiedler-bisect-dense | 5589ns | base | --- | [5551, 5628] | --- | --- | --- | --- |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | fiedler-bisect-dense |
|---|---|
| 1 | 5616ns |
| 2 | 5596ns |
| 3 | 5625ns |
| 4 | 5645ns |
| 5 | 5589ns |
| 6 | 5572ns |
| 7 | 5575ns |
| 8 | 5890ns |
| 9 | 6089ns |
| 10 | 5992ns |
| 11 | 5804ns |
| 12 | 5630ns |
| 13 | 5608ns |
| 14 | 5572ns |
| 15 | 5540ns |
| 16 | 5560ns |
| 17 | 5661ns |
| 18 | 5589ns |
| 19 | 5530ns |
| 20 | 5533ns |
| 21 | 5535ns |
| 22 | 5602ns |
| 23 | 5534ns |
| 24 | 5530ns |
| 25 | 5533ns |
| 26 | 5535ns |
| 27 | 5532ns |
| 28 | 5533ns |
| 29 | 5531ns |
| 30 | 5534ns |
| 31 | 5542ns |
| 32 | 5539ns |
| 33 | 5564ns |
| 34 | 5921ns |
| 35 | 5927ns |
| 36 | 5923ns |
| 37 | 5926ns |
| 38 | 5924ns |
| 39 | 5921ns |
| 40 | 5921ns |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fiedler-bisect-dense | 0.804 | HIGH+ (drift/warm-up) |

**Consistency summary:**


## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fiedler-bisect-dense | 2.0ns | 5668.1ns | 0.0% |  |

## Distribution (algo ns)

```
fiedler-bisect-dense (n=40, range 5532.1-5952.8 ns)
   5532.1 |########################################
   5553.2 |################
   5574.2 |############
   5595.2 |################
   5616.3 |########
   5637.3 |####
   5658.3 |####
   5679.4 |
   5700.4 |
   5721.5 |
   5742.5 |
   5763.5 |
   5784.6 |####
   5805.6 |
   5826.6 |
   5847.7 |
   5868.7 |####
   5889.7 |
   5910.8 |############################
   5931.8 |
  (4 below, 2 above range)

```

## Diagnostics

- **fiedler-bisect-dense**: autocorrelation=0.80 (measurement drift or warm-up artifact)
