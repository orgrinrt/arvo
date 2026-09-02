# Spectral bisection: Fiedler vector + sign-cut partition

1 variants, 40 samples per variant.
Baseline: **fiedler-bisect-dense**

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fiedler-bisect-dense | 6821ns | 6599ns | 6188ns | 6631ns | 8023ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| fiedler-bisect-dense | 6667ns | 6075ns | 7757ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fiedler-bisect-dense | 6821ns | 6821ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fiedler-bisect-dense | 6477ns | base | --- | [6464, 6515] | --- | --- | --- | --- |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | fiedler-bisect-dense |
|---|---|
| 1 | 6491ns |
| 2 | 6472ns |
| 3 | 6462ns |
| 4 | 6482ns |
| 5 | 6471ns |
| 6 | 6508ns |
| 7 | 6741ns |
| 8 | 6485ns |
| 9 | 6468ns |
| 10 | 6522ns |
| 11 | 6470ns |
| 12 | 6536ns |
| 13 | 6568ns |
| 14 | 6508ns |
| 15 | 6457ns |
| 16 | 6451ns |
| 17 | 6438ns |
| 18 | 6438ns |
| 19 | 6502ns |
| 20 | 6449ns |
| 21 | 6472ns |
| 22 | 6451ns |
| 23 | 6466ns |
| 24 | 7630ns |
| 25 | 6944ns |
| 26 | 6528ns |
| 27 | 8252ns |
| 28 | 7620ns |
| 29 | 10944ns |
| 30 | 6632ns |
| 31 | 6916ns |
| 32 | 6771ns |
| 33 | 6978ns |
| 34 | 6271ns |
| 35 | 5927ns |
| 36 | 5945ns |
| 37 | 6228ns |
| 38 | 5936ns |
| 39 | 5930ns |
| 40 | 5926ns |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fiedler-bisect-dense | 0.315 | moderate+ |

**Consistency summary:**


## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fiedler-bisect-dense | 2.5ns | 6667.2ns | 0.0% |  |

## Distribution (algo ns)

```
fiedler-bisect-dense (n=40, range 6075.1-7756.9 ns)
   6075.1 |
   6159.2 |##
   6243.3 |##
   6327.4 |
   6411.5 |########################################
   6495.6 |#################
   6579.7 |##
   6663.8 |##
   6747.9 |##
   6831.9 |##
   6916.0 |#####
   7000.1 |
   7084.2 |
   7168.3 |
   7252.4 |
   7336.5 |
   7420.6 |
   7504.7 |
   7588.7 |#####
   7672.8 |
  (5 below, 2 above range)

```
