# Clamping fold at 13 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-head dominates: 109% faster than the next best (warm-clamp-accfit-dyn)

warm-clamp-head (810 ns) leads warm-clamp-accfit-dyn (1.69 us) by 109%, a clear separation rather than a photo finish. CV 24.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-head beats baseline by 78% (significant)

warm-clamp-head is -2.55 us (78%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-minimum is an outlier: 7.0x slower than the field

warm-clamp-minimum (5.65 us) is 7.0x the fastest (810 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-min-lanes shows alternating (throttle bounce) (autocorr -0.57)

warm-clamp-min-lanes's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 7.0x the fastest

Fastest warm-clamp-head (810 ns) to slowest warm-clamp-minimum (5.65 us): 7.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-clamp-acc64 is inconsistent: worst-20% is 3.9x its best-20%

warm-clamp-acc64's best 20% of batches run at 1.06 us but its worst 20% at 4.17 us (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-head** at 809.5 ns median (-75.1% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 6.99x (fastest 809.5 ns, slowest 5654.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Clamping fold at 13 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112761915 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3163ns | 3504ns | 1122ns | 3408ns | 4471ns | base |
| warm-clamp-accfit | 1926ns | 1926ns | 1587ns | 1933ns | 2243ns | -39.12% |
| warm-clamp-accfit-dyn | 1947ns | 1917ns | 1542ns | 1953ns | 2336ns | -38.45% |
| warm-clamp-head | 1099ns | 1035ns | 858ns | 1030ns | 1545ns | -65.26% |
| warm-clamp-min-lanes | 3338ns | 3434ns | 2578ns | 3325ns | 4135ns | +5.52% |
| warm-clamp-minimum | 5363ns | 5728ns | 4622ns | 5274ns | 6370ns | +69.55% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 2973ns | 1059ns | 4167ns | base | 2.756 |
| warm-clamp-accfit | 1687ns | 1403ns | 1934ns | -43.24% | 4.855 |
| warm-clamp-accfit-dyn | 1706ns | 1359ns | 2016ns | -42.63% | 4.803 |
| warm-clamp-head | 866ns | 689ns | 1205ns | -70.85% | 9.455 |
| warm-clamp-min-lanes | 3126ns | 2423ns | 3850ns | +5.17% | 2.620 |
| warm-clamp-minimum | 5288ns | 4564ns | 6257ns | +77.88% | 1.549 |

## Performance model

- Peak throughput: **11.891 Gops/s** (warm-clamp-head; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.517 | 21.2% |
| warm-clamp-accfit | 4.819 | 40.5% |
| warm-clamp-accfit-dyn | 4.844 | 40.7% |
| warm-clamp-head | 10.120 | 85.1% |
| warm-clamp-min-lanes | 2.538 | 21.3% |
| warm-clamp-minimum | 1.449 | 12.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3163ns | 3163ns | base |
| warm-clamp-accfit | 1926ns | 1926ns | -39.12% |
| warm-clamp-accfit-dyn | 1947ns | 1947ns | -38.45% |
| warm-clamp-head | 1099ns | 1099ns | -65.26% |
| warm-clamp-min-lanes | 3338ns | 3338ns | +5.52% |
| warm-clamp-minimum | 5363ns | 5363ns | +69.55% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3255ns | base | --- | [2866, 3811] | --- | --- | --- | --- |
| warm-clamp-accfit | 1700ns | -1589.2ns (-48.8%) | [-1992, -1143]ns | [1691, 1700] | YES | 0.0008 | 0.0007 | 0 |
| warm-clamp-accfit-dyn | 1691ns | -1459.9ns (-44.9%) | [-1978, -1146]ns | [1682, 1766] | YES | 0.0008 | 0.0007 | 0 |
| warm-clamp-head | 810ns | -2550.7ns (-78.4%) | [-2836, -2162]ns | [759, 856] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 3227ns | no significant difference | [-241, +54]ns | [2732, 3432] | no | 0.6358 | 0.6358 | 0 |
| warm-clamp-minimum | 5655ns | +1889.5ns (+58.1%) | [+1801, +2131]ns | [4659, 5691] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3849ns | -55.3% | -51.2% | -70.3% | -7.7% | +18.6% |
| 1 | 2 | 2932ns | -42.0% | -42.1% | -76.0% | -0.0% | +59.0% |
| 1 | 3 | 3768ns | -52.9% | -62.8% | -77.9% | -5.3% | +50.7% |
| 1 | 4 | 3769ns | -62.4% | -53.2% | -79.9% | -32.7% | +51.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.463 | moderate- |
| warm-clamp-accfit | -0.257 | moderate- |
| warm-clamp-accfit-dyn | -0.187 | ok |
| warm-clamp-head | -0.322 | moderate- |
| warm-clamp-min-lanes | -0.573 | HIGH- (thermal bounce) |
| warm-clamp-minimum | 0.274 | moderate+ |

**Consistency summary:**

- **warm-clamp-accfit**: won 4/4, lost 0/4
- **warm-clamp-accfit-dyn**: won 4/4, lost 0/4
- **warm-clamp-head**: won 4/4, lost 0/4
- **warm-clamp-min-lanes**: won 3/4, lost 0/4
- **warm-clamp-minimum**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 66.6ns | 2972.7ns | 2.2% |  |
| warm-clamp-accfit | 84.4ns | 1687.3ns | 5.0% | HIGH |
| warm-clamp-accfit-dyn | 84.3ns | 1705.5ns | 4.9% |  |
| warm-clamp-head | 80.8ns | 866.4ns | 9.3% | HIGH |
| warm-clamp-min-lanes | 74.2ns | 3126.3ns | 2.4% |  |
| warm-clamp-minimum | 22.8ns | 5287.9ns | 0.4% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1058.5-4167.1 ns)
   1058.5 |##########################
   1214.0 |
   1369.4 |
   1524.8 |
   1680.3 |
   1835.7 |
   1991.1 |####
   2146.5 |
   2302.0 |
   2457.4 |####
   2612.8 |########
   2768.2 |####
   2923.7 |#############
   3079.1 |#############
   3234.5 |####
   3389.9 |####
   3545.4 |
   3700.8 |########################################
   3856.2 |#################
   4011.6 |
  (3 below, 5 above range)

warm-clamp-accfit (n=40, range 1403.0-1934.4 ns)
   1403.0 |######################
   1429.6 |
   1456.2 |####
   1482.7 |
   1509.3 |####
   1535.9 |########
   1562.5 |
   1589.0 |
   1615.6 |
   1642.2 |
   1668.7 |###############################
   1695.3 |########################################
   1721.9 |########
   1748.5 |
   1775.0 |
   1801.6 |####
   1828.2 |####
   1854.7 |########
   1881.3 |##########################
   1907.9 |
  (2 below, 1 above range)

warm-clamp-accfit-dyn (n=40, range 1358.6-2015.8 ns)
   1358.6 |
   1391.5 |############################
   1424.4 |
   1457.2 |
   1490.1 |
   1522.9 |#####
   1555.8 |
   1588.7 |
   1621.5 |
   1654.4 |############################
   1687.2 |########################################
   1720.1 |######################
   1752.9 |#####
   1785.8 |#####
   1818.7 |
   1851.5 |########################################
   1884.4 |
   1917.2 |#####
   1950.1 |#####
   1983.0 |#####
  (4 below, 2 above range)

warm-clamp-head (n=40, range 688.9-1205.4 ns)
    688.9 |########################################
    714.8 |
    740.6 |########################
    766.4 |####
    792.2 |
    818.1 |########################
    843.9 |
    869.7 |########
    895.5 |########
    921.3 |
    947.2 |
    973.0 |
    998.8 |########
   1024.6 |####
   1050.4 |
   1076.3 |
   1102.1 |
   1127.9 |################
   1153.7 |
   1179.5 |
  (3 below, 3 above range)

warm-clamp-min-lanes (n=40, range 2423.5-3850.5 ns)
   2423.5 |
   2494.8 |########################################
   2566.2 |#####
   2637.5 |###########
   2708.9 |######################
   2780.2 |
   2851.6 |#####
   2922.9 |###########
   2994.3 |#####
   3065.6 |
   3137.0 |
   3208.3 |############################
   3279.7 |###########
   3351.0 |
   3422.4 |
   3493.7 |############################
   3565.1 |######################
   3636.4 |
   3707.8 |#####
   3779.1 |
  (1 below, 4 above range)

warm-clamp-minimum (n=40, range 4563.7-6257.0 ns)
   4563.7 |#####################
   4648.3 |##############
   4733.0 |
   4817.7 |
   4902.3 |
   4987.0 |
   5071.7 |
   5156.3 |
   5241.0 |
   5325.7 |
   5410.3 |
   5495.0 |
   5579.7 |####
   5664.3 |########################################
   5749.0 |
   5833.7 |
   5918.3 |
   6003.0 |
   6087.7 |
   6172.3 |
  (4 below, 2 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=38.9% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-clamp-head**: CV=22.8% (high variance, measurements may be unstable)
