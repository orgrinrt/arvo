# Container fork, operation-density sweep at 64 bits (8192 elements, wrapping)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-minimum dominates: 10% faster than the next best (warm-container-kernel)

warm-container-minimum (6.99 us) leads warm-container-kernel (7.70 us) by 10%, a clear separation rather than a photo finish. CV 9.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-kernel beats baseline by 60% (significant)

warm-container-kernel is -10.29 us (60%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 2.5x slower than the field

warm-container-plusone (17.22 us) is 2.5x the fastest (6.99 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-headroom shows alternating (throttle bounce) (autocorr -0.79)

warm-container-headroom's per-pass series has lag-1 autocorrelation -0.79, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-minimum, warm-container-kernel, warm-container-native} vs {warm-container-headroom, warm-container-plusone} (121% apart)

The field splits into a fast tier {warm-container-minimum, warm-container-kernel, warm-container-native} and a slow tier {warm-container-headroom, warm-container-plusone} with a 121% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: warm-container-minimum** at 6992.9 ns median (-59.2% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.46x (fastest 6992.9 ns, slowest 17221.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Container fork, operation-density sweep at 64 bits (8192 elements, wrapping) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112747662 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 16916ns | 17224ns | 14332ns | 17098ns | 18952ns | base |
| warm-container-kernel | 7412ns | 7784ns | 6084ns | 7711ns | 7840ns | -56.18% |
| warm-container-minimum | 7453ns | 7060ns | 7048ns | 7194ns | 8634ns | -55.94% |
| warm-container-native | 7769ns | 7854ns | 6702ns | 7816ns | 8696ns | -54.07% |
| warm-container-plusone | 17335ns | 17324ns | 15552ns | 17339ns | 19105ns | +2.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 16835ns | 14263ns | 18855ns | base | 8.272 |
| warm-container-kernel | 7329ns | 6019ns | 7737ns | -56.46% | 19.001 |
| warm-container-minimum | 7378ns | 6981ns | 8546ns | -56.18% | 18.876 |
| warm-container-native | 7691ns | 6634ns | 8604ns | -54.32% | 18.108 |
| warm-container-plusone | 17255ns | 15482ns | 19019ns | +2.49% | 8.071 |

## Performance model

- Peak throughput: **23.138 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 139264

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 8.121 | 35.1% |
| warm-container-kernel | 18.079 | 78.1% |
| warm-container-minimum | 19.915 | 86.1% |
| warm-container-native | 17.951 | 77.6% |
| warm-container-plusone | 8.087 | 34.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 16916ns | 16916ns | base |
| warm-container-kernel | 7412ns | 7412ns | -56.18% |
| warm-container-minimum | 7453ns | 7453ns | -55.94% |
| warm-container-native | 7769ns | 7769ns | -54.07% |
| warm-container-plusone | 17335ns | 17335ns | +2.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 17149ns | base | --- | [15385, 18812] | --- | --- | --- | --- |
| warm-container-kernel | 7703ns | -10286.7ns (-60.0%) | [-11108, -9369]ns | [7692, 7704] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 6993ns | -9384.1ns (-54.7%) | [-10280, -8392]ns | [6991, 6995] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 7758ns | -9386.6ns (-54.7%) | [-10267, -8397]ns | [6989, 8542] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 17221ns | +662.4ns (+3.9%) | [+142, +1297]ns | [15556, 18973] | YES | 0.0022 | 0.0022 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 15388ns | -60.9% | -54.6% | -54.6% | +23.3% |
| 1 | 2 | 18816ns | -59.1% | -54.6% | -64.7% | +1.0% |
| 1 | 3 | 14257ns | -46.0% | -51.0% | -40.0% | +9.1% |
| 1 | 4 | 18812ns | -59.0% | -62.8% | -54.6% | -17.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.789 | HIGH- (thermal bounce) |
| warm-container-kernel | -0.083 | ok |
| warm-container-minimum | -0.415 | moderate- |
| warm-container-native | 0.187 | ok |
| warm-container-plusone | 0.250 | moderate+ |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 23.5ns | 16835.4ns | 0.1% |  |
| warm-container-kernel | 24.2ns | 7329.4ns | 0.3% |  |
| warm-container-minimum | 22.4ns | 7377.7ns | 0.3% |  |
| warm-container-native | 23.6ns | 7690.6ns | 0.3% |  |
| warm-container-plusone | 23.1ns | 17255.1ns | 0.1% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 14263.1-18854.9 ns)
  14263.1 |#######
  14492.7 |
  14722.3 |
  14951.9 |
  15181.5 |######################
  15411.1 |##
  15640.7 |
  15870.3 |
  16099.8 |
  16329.4 |
  16559.0 |
  16788.6 |
  17018.2 |
  17247.8 |
  17477.4 |
  17707.0 |
  17936.5 |
  18166.1 |
  18395.7 |
  18625.3 |########################################
  (7 below, 4 above range)

warm-container-kernel (n=40, range 6018.7-7737.1 ns)
   6018.7 |########
   6104.6 |
   6190.6 |
   6276.5 |
   6362.4 |
   6448.3 |
   6534.2 |
   6620.2 |
   6706.1 |
   6792.0 |
   6877.9 |
   6963.8 |
   7049.8 |
   7135.7 |
   7221.6 |
   7307.5 |
   7393.4 |
   7479.4 |
   7565.3 |
   7651.2 |########################################
  (3 below, 1 above range)

warm-container-minimum (n=40, range 6981.2-8545.9 ns)
   6981.2 |########################################
   7059.5 |
   7137.7 |
   7215.9 |
   7294.2 |
   7372.4 |
   7450.6 |
   7528.9 |
   7607.1 |
   7685.3 |
   7763.6 |
   7841.8 |
   7920.0 |
   7998.3 |
   8076.5 |
   8154.7 |
   8233.0 |
   8311.2 |
   8389.4 |
   8467.7 |#######
  (2 below, 5 above range)

warm-container-native (n=40, range 6633.9-8603.9 ns)
   6633.9 |############
   6732.4 |
   6830.9 |
   6929.4 |#####################
   7027.9 |
   7126.4 |
   7224.9 |
   7323.4 |
   7421.9 |
   7520.4 |
   7618.9 |
   7717.4 |
   7815.9 |
   7914.4 |
   8012.9 |
   8111.4 |
   8209.9 |
   8308.4 |
   8406.9 |
   8505.4 |########################################
  (4 below, 1 above range)

warm-container-plusone (n=40, range 15482.0-19018.9 ns)
  15482.0 |########################################
  15658.9 |
  15835.7 |
  16012.6 |
  16189.4 |
  16366.3 |
  16543.1 |
  16720.0 |
  16896.8 |
  17073.6 |
  17250.5 |
  17427.3 |
  17604.2 |
  17781.0 |
  17957.9 |
  18134.7 |
  18311.6 |
  18488.4 |
  18665.2 |
  18842.1 |###################################
  (2 below, 4 above range)

```
