# Clamping fold at 13 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-head dominates: 106% faster than the next best (warm-clamp-accfit-dyn)

warm-clamp-head (763 ns) leads warm-clamp-accfit-dyn (1.57 us) by 106%, a clear separation rather than a photo finish. CV 11.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-head beats baseline by 77% (significant)

warm-clamp-head is -2.88 us (77%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-minimum is an outlier: 5.8x slower than the field

warm-clamp-minimum (4.40 us) is 5.8x the fastest (763 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-head shows alternating (throttle bounce) (autocorr -0.67)

warm-clamp-head's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 5.8x the fastest

Fastest warm-clamp-head (763 ns) to slowest warm-clamp-minimum (4.40 us): 5.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-clamp-minimum is inconsistent: worst-20% is 2.1x its best-20%

warm-clamp-minimum's best 20% of batches run at 4.25 us but its worst 20% at 9.14 us (2.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-head** at 762.8 ns median (-79.5% vs baseline)
- 4 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.76x (fastest 762.8 ns, slowest 4395.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3598ns | 3941ns | 2079ns | 3822ns | 4444ns | base |
| warm-clamp-accfit | 1861ns | 1916ns | 1620ns | 1878ns | 2051ns | -48.27% |
| warm-clamp-accfit-dyn | 1818ns | 1865ns | 1601ns | 1828ns | 2005ns | -49.47% |
| warm-clamp-head | 977ns | 951ns | 877ns | 948ns | 1164ns | -72.84% |
| warm-clamp-min-lanes | 3105ns | 3080ns | 2778ns | 3065ns | 3552ns | -13.69% |
| warm-clamp-minimum | 5567ns | 4454ns | 4313ns | 4735ns | 9319ns | +54.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3385ns | 1962ns | 4148ns | base | 2.420 |
| warm-clamp-accfit | 1625ns | 1430ns | 1743ns | -51.99% | 5.041 |
| warm-clamp-accfit-dyn | 1573ns | 1375ns | 1736ns | -53.54% | 5.209 |
| warm-clamp-head | 772ns | 704ns | 903ns | -77.21% | 10.618 |
| warm-clamp-min-lanes | 2908ns | 2614ns | 3288ns | -14.09% | 2.817 |
| warm-clamp-minimum | 5482ns | 4254ns | 9143ns | +61.95% | 1.494 |

## Performance model

- Peak throughput: **11.633 Gops/s** (warm-clamp-head; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.202 | 18.9% |
| warm-clamp-accfit | 4.845 | 41.6% |
| warm-clamp-accfit-dyn | 5.217 | 44.8% |
| warm-clamp-head | 10.739 | 92.3% |
| warm-clamp-min-lanes | 2.822 | 24.3% |
| warm-clamp-minimum | 1.864 | 16.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3598ns | 3598ns | base |
| warm-clamp-accfit | 1861ns | 1861ns | -48.27% |
| warm-clamp-accfit-dyn | 1818ns | 1818ns | -49.47% |
| warm-clamp-head | 977ns | 977ns | -72.84% |
| warm-clamp-min-lanes | 3105ns | 3105ns | -13.69% |
| warm-clamp-minimum | 5567ns | 5567ns | +54.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3720ns | base | --- | [3436, 3811] | --- | --- | --- | --- |
| warm-clamp-accfit | 1691ns | -2041.6ns (-54.9%) | [-2120, -1754]ns | [1611, 1692] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 1570ns | -2080.3ns (-55.9%) | [-2151, -1784]ns | [1517, 1688] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 763ns | -2879.1ns (-77.4%) | [-3045, -2693]ns | [710, 765] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 2903ns | -784.3ns (-21.1%) | [-871, -423]ns | [2766, 2972] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 4395ns | +1238.4ns (+33.3%) | [+914, +1558]ns | [4374, 5349] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 3285ns | -48.3% | -54.7% | -78.5% | -9.2% | +29.5% |
| 1 | 2 | 3834ns | -62.7% | -62.9% | -78.3% | -23.9% | +14.4% |
| 1 | 3 | 3733ns | -54.7% | -54.7% | -80.5% | -22.0% | +17.7% |
| 1 | 4 | 3797ns | -55.5% | -55.5% | -79.9% | -27.5% | +41.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.221 | moderate- |
| warm-clamp-accfit | -0.431 | moderate- |
| warm-clamp-accfit-dyn | 0.156 | ok |
| warm-clamp-head | -0.674 | HIGH- (thermal bounce) |
| warm-clamp-min-lanes | -0.002 | ok |
| warm-clamp-minimum | -0.050 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 4/4, lost 0/4
- **warm-clamp-accfit-dyn**: won 4/4, lost 0/4
- **warm-clamp-head**: won 4/4, lost 0/4
- **warm-clamp-min-lanes**: won 4/4, lost 0/4
- **warm-clamp-minimum**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 76.5ns | 3385.3ns | 2.3% |  |
| warm-clamp-accfit | 82.1ns | 1625.2ns | 5.1% | HIGH |
| warm-clamp-accfit-dyn | 84.6ns | 1572.7ns | 5.4% | HIGH |
| warm-clamp-head | 72.1ns | 771.5ns | 9.3% | HIGH |
| warm-clamp-min-lanes | 70.5ns | 2908.2ns | 2.4% |  |
| warm-clamp-minimum | 25.8ns | 5482.5ns | 0.5% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1962.0-4147.8 ns)
   1962.0 |
   2071.3 |
   2180.6 |
   2289.9 |
   2399.2 |####
   2508.5 |
   2617.8 |
   2727.1 |
   2836.4 |####
   2945.6 |####
   3054.9 |####
   3164.2 |################
   3273.5 |####
   3382.8 |############
   3492.1 |############
   3601.4 |####
   3710.7 |########################################
   3820.0 |####################
   3929.2 |
   4038.5 |
  (4 below, 5 above range)

warm-clamp-accfit (n=40, range 1429.5-1743.3 ns)
   1429.5 |
   1445.2 |###
   1460.9 |
   1476.6 |
   1492.3 |###
   1507.9 |
   1523.6 |#############
   1539.3 |######
   1555.0 |
   1570.7 |
   1586.4 |
   1602.1 |######
   1617.8 |
   1633.5 |###
   1649.1 |
   1664.8 |###
   1680.5 |########################################
   1696.2 |################
   1711.9 |###
   1727.6 |######
  (5 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 1374.6-1735.8 ns)
   1374.6 |###
   1392.7 |##############
   1410.8 |##########
   1428.8 |
   1446.9 |
   1464.9 |###
   1483.0 |###
   1501.1 |###
   1519.1 |##############
   1537.2 |
   1555.2 |#######
   1573.3 |###
   1591.4 |###
   1609.4 |
   1627.5 |
   1645.5 |
   1663.6 |#######
   1681.7 |########################################
   1699.7 |
   1717.8 |
  (3 below, 5 above range)

warm-clamp-head (n=40, range 704.2-902.8 ns)
    704.2 |########################################
    714.1 |###
    724.0 |
    734.0 |
    743.9 |###
    753.8 |#########################
    763.8 |#########################
    773.7 |
    783.6 |###
    793.6 |
    803.5 |
    813.4 |
    823.3 |##############
    833.3 |#######
    843.2 |
    853.1 |
    863.1 |
    873.0 |
    882.9 |
    892.9 |
  (3 below, 3 above range)

warm-clamp-min-lanes (n=40, range 2614.2-3288.3 ns)
   2614.2 |
   2648.0 |#####
   2681.7 |###########
   2715.4 |#################
   2749.1 |########################################
   2782.8 |
   2816.5 |
   2850.2 |#####
   2883.9 |###########
   2917.6 |#####
   2951.3 |############################
   2985.0 |#################
   3018.7 |#####
   3052.4 |#####
   3086.1 |
   3119.8 |
   3153.5 |#####
   3187.2 |#####
   3220.9 |###########
   3254.6 |#####
  (5 below, 3 above range)

warm-clamp-minimum (n=40, range 4253.7-9142.8 ns)
   4253.7 |########################################
   4498.2 |##
   4742.6 |
   4987.1 |
   5231.5 |#################
   5476.0 |##
   5720.4 |
   5964.9 |##
   6209.3 |
   6453.8 |
   6698.2 |
   6942.7 |
   7187.1 |
   7431.6 |
   7676.1 |
   7920.5 |
   8165.0 |##
   8409.4 |
   8653.9 |
   8898.3 |
  (6 below, 4 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=25.8% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: CV=42.2% (high variance, measurements may be unstable)
