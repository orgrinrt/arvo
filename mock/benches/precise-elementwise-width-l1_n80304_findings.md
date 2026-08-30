# Container fork under saturating semantics, elementwise, declared-width sweep (8192 elements, 4 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (436 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-minimum at 142 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-minimum dominates: 16% faster than the next best (warm-container-native)

warm-container-minimum (142 ns) leads warm-container-native (165 ns) by 16%, a clear separation rather than a photo finish. CV 12.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-minimum beats baseline by 64% (significant)

warm-container-minimum is -279 ns (64%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 3.1x slower than the field

warm-container-headroom (436 ns) is 3.1x the fastest (142 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-native shows warm-up / thermal drift (autocorr +0.90)

warm-container-native's per-pass series has lag-1 autocorrelation +0.90, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-minimum, warm-container-native, warm-container-kernel} vs {warm-container-plusone, warm-container-headroom} (99% apart)

The field splits into a fast tier {warm-container-minimum, warm-container-native, warm-container-kernel} and a slow tier {warm-container-plusone, warm-container-headroom} with a 99% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.1x the fastest

Fastest warm-container-minimum (142 ns) to slowest warm-container-headroom (436 ns): 3.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-kernel is inconsistent: worst-20% is 5.2x its best-20%

warm-container-kernel's best 20% of batches run at 131 ns but its worst 20% at 679 ns (5.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-minimum** at 141.9 ns median (-67.5% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 3.07x (fastest 141.9 ns, slowest 435.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 516ns | 517ns | 392ns | 473ns | 769ns | base |
| warm-container-kernel | 410ns | 250ns | 192ns | 301ns | 952ns | -20.62% |
| warm-container-minimum | 216ns | 210ns | 189ns | 212ns | 253ns | -58.22% |
| warm-container-native | 314ns | 241ns | 189ns | 265ns | 585ns | -39.11% |
| warm-container-plusone | 437ns | 403ns | 394ns | 424ns | 521ns | -15.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 437ns | 333ns | 650ns | base | 93.778 |
| warm-container-kernel | 286ns | 131ns | 679ns | -34.41% | 142.986 |
| warm-container-minimum | 146ns | 128ns | 172ns | -66.49% | 279.848 |
| warm-container-native | 218ns | 129ns | 417ns | -50.01% | 187.576 |
| warm-container-plusone | 370ns | 334ns | 441ns | -15.22% | 110.618 |

## Performance model

- Peak throughput: **319.314 Gops/s** (warm-container-minimum; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 93.977 | 29.4% |
| warm-container-kernel | 240.376 | 75.3% |
| warm-container-minimum | 288.756 | 90.4% |
| warm-container-native | 248.544 | 77.8% |
| warm-container-plusone | 120.612 | 37.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 516ns | 516ns | base |
| warm-container-kernel | 410ns | 410ns | -20.62% |
| warm-container-minimum | 216ns | 216ns | -58.22% |
| warm-container-native | 314ns | 314ns | -39.11% |
| warm-container-plusone | 437ns | 437ns | -15.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 436ns | base | --- | [334, 440] | --- | --- | --- | --- |
| warm-container-kernel | 170ns | -264.2ns (-60.6%) | [-269, -202]ns | [169, 171] | YES | 0.0001 | 0.0000 | 0 |
| warm-container-minimum | 142ns | -264.8ns (-60.8%) | [-271, -206]ns | [130, 156] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 165ns | -268.4ns (-61.6%) | [-273, -204]ns | [156, 171] | YES | 0.0001 | 0.0000 | 0 |
| warm-container-plusone | 340ns | no significant difference | [-6, +1]ns | [339, 352] | no | 0.2682 | 0.2682 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|
| 1 | 333ns | +97.4% | -61.2% | +25.0% | +1.9% |
| 2 | 333ns | +97.0% | -61.1% | +24.9% | +1.3% |
| 3 | 334ns | +98.6% | -61.4% | +25.0% | +1.5% |
| 4 | 335ns | +96.0% | -61.3% | +24.4% | +1.0% |
| 5 | 334ns | +97.1% | -61.6% | +25.1% | +1.2% |
| 6 | 342ns | +90.7% | -62.4% | +21.4% | -0.5% |
| 7 | 344ns | +90.0% | -62.3% | +22.0% | -0.8% |
| 8 | 996ns | -34.0% | -86.9% | -58.1% | -65.7% |
| 9 | 998ns | -16.4% | -87.1% | -58.4% | -65.8% |
| 10 | 994ns | -34.4% | -87.1% | -58.1% | -65.8% |
| 11 | 439ns | -61.3% | -64.8% | -61.4% | -22.9% |
| 12 | 438ns | -60.7% | -64.7% | -60.7% | -22.8% |
| 13 | 443ns | -61.5% | -64.4% | -61.4% | -23.4% |
| 14 | 438ns | -60.9% | -64.3% | -61.0% | -22.3% |
| 15 | 440ns | -61.7% | -64.8% | -61.3% | -6.7% |
| 16 | 437ns | -60.9% | -64.5% | -61.2% | -22.3% |
| 17 | 442ns | -62.0% | -64.2% | -61.3% | -23.3% |
| 18 | 442ns | -61.8% | -64.7% | -61.2% | -23.3% |
| 19 | 441ns | -61.3% | -65.3% | -61.3% | -23.3% |
| 20 | 442ns | -61.3% | -65.0% | -61.8% | -23.5% |
| 21 | 438ns | -60.7% | -60.8% | -64.4% | +0.3% |
| 22 | 434ns | -60.7% | -60.3% | -63.0% | +1.6% |
| 23 | 442ns | -61.4% | -61.3% | -63.9% | -1.7% |
| 24 | 435ns | -60.9% | -60.6% | -64.4% | +1.1% |
| 25 | 440ns | -60.9% | -61.3% | -64.9% | -0.1% |
| 26 | 435ns | -60.6% | -59.6% | -64.2% | +1.2% |
| 27 | 442ns | -62.0% | -60.9% | -64.6% | -0.3% |
| 28 | 443ns | -61.4% | -61.2% | -64.5% | -0.8% |
| 29 | 442ns | -61.7% | -61.5% | -65.0% | -1.0% |
| 30 | 443ns | -61.7% | -61.1% | -64.6% | -0.3% |
| 31 | 333ns | -60.6% | -61.2% | -61.6% | -0.1% |
| 32 | 333ns | -59.9% | -61.7% | -61.2% | +0.4% |
| 33 | 333ns | -61.4% | -61.0% | -60.9% | +0.3% |
| 34 | 333ns | -60.7% | -61.6% | -60.8% | +0.1% |
| 35 | 334ns | -60.5% | -61.6% | -60.5% | +8.7% |
| 36 | 333ns | -60.4% | -61.3% | -61.3% | +34.3% |
| 37 | 334ns | -60.7% | -61.6% | -61.3% | +25.3% |
| 38 | 333ns | -60.4% | -60.8% | -61.0% | +0.6% |
| 39 | 334ns | -60.9% | -61.3% | -61.6% | -0.5% |
| 40 | 333ns | -60.9% | -61.1% | -61.2% | -0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.656 | HIGH+ (drift/warm-up) |
| warm-container-kernel | 0.887 | HIGH+ (drift/warm-up) |
| warm-container-minimum | 0.869 | HIGH+ (drift/warm-up) |
| warm-container-native | 0.898 | HIGH+ (drift/warm-up) |
| warm-container-plusone | 0.705 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-container-kernel**: won 33/40, lost 7/40
- **warm-container-minimum**: won 40/40, lost 0/40
- **warm-container-native**: won 33/40, lost 7/40
- **warm-container-plusone**: won 23/40, lost 16/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 3.1ns | 436.8ns | 0.7% |  |
| warm-container-kernel | 5.5ns | 286.5ns | 1.9% |  |
| warm-container-minimum | 3.0ns | 146.4ns | 2.1% |  |
| warm-container-native | 4.6ns | 218.4ns | 2.1% |  |
| warm-container-plusone | 2.5ns | 370.3ns | 0.7% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 333.2-650.1 ns)
    333.2 |############################
    349.0 |
    364.9 |
    380.7 |
    396.6 |
    412.4 |
    428.2 |########################################
    444.1 |
    460.0 |
    475.8 |
    491.6 |
    507.5 |
    523.4 |
    539.2 |
    555.0 |
    570.9 |
    586.8 |
    602.6 |
    618.5 |
    634.3 |
  (3 below, 3 above range)

warm-container-kernel (n=40, range 130.8-679.4 ns)
    130.8 |############
    158.3 |########################################
    185.7 |
    213.1 |
    240.5 |
    268.0 |
    295.4 |
    322.8 |
    350.3 |
    377.7 |
    405.1 |
    432.6 |
    460.0 |
    487.4 |
    514.8 |
    542.3 |
    569.7 |
    597.1 |
    624.6 |
    652.0 |##################
  (4 below, 1 above range)

warm-container-minimum (n=40, range 128.3-172.2 ns)
    128.3 |########################################
    130.5 |##
    132.7 |
    134.9 |
    137.1 |
    139.3 |
    141.5 |
    143.7 |
    145.9 |
    148.1 |
    150.3 |
    152.5 |#######
    154.7 |###########
    156.9 |####
    159.0 |
    161.2 |
    163.4 |
    165.6 |
    167.8 |##
    170.0 |################
  (2 below, 2 above range)

warm-container-native (n=40, range 129.1-417.1 ns)
    129.1 |#######################
    143.5 |##########################
    157.9 |########################################
    172.3 |
    186.7 |
    201.1 |
    215.5 |
    229.9 |
    244.3 |
    258.7 |
    273.1 |
    287.5 |
    301.9 |
    316.3 |
    330.7 |
    345.1 |
    359.5 |
    373.9 |
    388.3 |
    402.7 |#######################
  (3 below, 3 above range)

warm-container-plusone (n=40, range 334.1-441.1 ns)
    334.1 |########################################
    339.4 |###############
    344.8 |
    350.1 |
    355.5 |
    360.8 |##
    366.2 |
    371.5 |
    376.9 |
    382.2 |
    387.6 |
    392.9 |
    398.3 |
    403.6 |
    409.0 |##
    414.3 |##
    419.7 |
    425.0 |
    430.4 |##
    435.7 |####################
  (4 below, 2 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=38.2% (high variance, measurements may be unstable)
- **warm-container-headroom**: autocorrelation=0.66 (measurement drift or warm-up artifact)
- **warm-container-kernel**: CV=78.9% (high variance, measurements may be unstable)
- **warm-container-kernel**: worst_20/best_20 = 5.2x (possible bimodal distribution)
- **warm-container-kernel**: autocorrelation=0.89 (measurement drift or warm-up artifact)
- **warm-container-minimum**: autocorrelation=0.87 (measurement drift or warm-up artifact)
- **warm-container-native**: CV=52.9% (high variance, measurements may be unstable)
- **warm-container-native**: worst_20/best_20 = 3.2x (possible bimodal distribution)
- **warm-container-native**: autocorrelation=0.90 (measurement drift or warm-up artifact)
- **warm-container-plusone**: autocorrelation=0.71 (measurement drift or warm-up artifact)
