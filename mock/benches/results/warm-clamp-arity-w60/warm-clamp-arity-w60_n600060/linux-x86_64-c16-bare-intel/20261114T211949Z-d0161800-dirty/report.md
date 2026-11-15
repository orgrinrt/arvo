# Clamping fold at 60 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-min-lanes dominates: 14% faster than the next best (warm-clamp-head)

warm-clamp-min-lanes (2.68 us) leads warm-clamp-head (3.06 us) by 14%, a clear separation rather than a photo finish. CV 82.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-accfit beats baseline by 53% (significant)

warm-clamp-accfit is -2.85 us (53%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-minimum is an outlier: 2.0x slower than the field

warm-clamp-minimum (5.37 us) is 2.0x the fastest (2.68 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-minimum shows alternating (throttle bounce) (autocorr -0.73)

warm-clamp-minimum's per-pass series has lag-1 autocorrelation -0.73, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-min-lanes, warm-clamp-head} vs {warm-clamp-accfit-dyn, warm-clamp-accfit, warm-clamp-acc64, warm-clamp-minimum} (39% apart)

The field splits into a fast tier {warm-clamp-min-lanes, warm-clamp-head} and a slow tier {warm-clamp-accfit-dyn, warm-clamp-accfit, warm-clamp-acc64, warm-clamp-minimum} with a 39% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-clamp-accfit is inconsistent: worst-20% is 3.9x its best-20%

warm-clamp-accfit's best 20% of batches run at 1.54 us but its worst 20% at 6.05 us (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-min-lanes** at 2675.9 ns median (-49.9% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 2.01x (fastest 2675.9 ns, slowest 5374.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 5719ns | 5433ns | 4475ns | 5419ns | 7864ns | base |
| warm-clamp-accfit | 3849ns | 4794ns | 1610ns | 3758ns | 6360ns | -32.70% |
| warm-clamp-accfit-dyn | 4156ns | 4426ns | 1962ns | 3992ns | 6842ns | -27.33% |
| warm-clamp-head | 4871ns | 3142ns | 2554ns | 4249ns | 9056ns | -14.82% |
| warm-clamp-min-lanes | 3836ns | 2764ns | 2197ns | 3012ns | 7945ns | -32.93% |
| warm-clamp-minimum | 5855ns | 5472ns | 4490ns | 5147ns | 9346ns | +2.38% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 5610ns | 4396ns | 7680ns | base | 1.460 |
| warm-clamp-accfit | 3676ns | 1542ns | 6050ns | -34.47% | 2.229 |
| warm-clamp-accfit-dyn | 3969ns | 1879ns | 6497ns | -29.26% | 2.064 |
| warm-clamp-head | 4728ns | 2486ns | 8744ns | -15.72% | 1.733 |
| warm-clamp-min-lanes | 3703ns | 2127ns | 7635ns | -33.99% | 2.212 |
| warm-clamp-minimum | 5741ns | 4413ns | 9120ns | +2.34% | 1.427 |

## Performance model

- Peak throughput: **5.313 Gops/s** (warm-clamp-accfit; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 1.535 | 28.9% |
| warm-clamp-accfit | 1.785 | 33.6% |
| warm-clamp-accfit-dyn | 1.934 | 36.4% |
| warm-clamp-head | 2.679 | 50.4% |
| warm-clamp-min-lanes | 3.061 | 57.6% |
| warm-clamp-minimum | 1.524 | 28.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 5719ns | 5719ns | base |
| warm-clamp-accfit | 3849ns | 3849ns | -32.70% |
| warm-clamp-accfit-dyn | 4156ns | 4156ns | -27.33% |
| warm-clamp-head | 4871ns | 4871ns | -14.82% |
| warm-clamp-min-lanes | 3836ns | 3836ns | -32.93% |
| warm-clamp-minimum | 5855ns | 5855ns | +2.38% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 5338ns | base | --- | [5305, 5388] | --- | --- | --- | --- |
| warm-clamp-accfit | 4589ns | -2845.8ns (-53.3%) | [-3352, -673]ns | [1884, 5214] | YES | 0.0005 | 0.0002 | 0 |
| warm-clamp-accfit-dyn | 4236ns | -2385.0ns (-44.7%) | [-2978, -289]ns | [1884, 5547] | YES | 0.0107 | 0.0064 | 0 |
| warm-clamp-head | 3058ns | -2070.8ns (-38.8%) | [-2267, -1097]ns | [3049, 5681] | YES | 0.0207 | 0.0166 | 0 |
| warm-clamp-min-lanes | 2676ns | -2451.0ns (-45.9%) | [-2644, -2257]ns | [2408, 2689] | YES | 0.0002 | 0.0000 | 0 |
| warm-clamp-minimum | 5375ns | no significant difference | [+0, +48]ns | [4522, 5391] | no | 0.1081 | 0.1081 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 5365ns | -38.9% | -1.3% | -42.8% | -50.0% | +0.4% |
| 1 | 2 | 5502ns | -39.9% | -37.5% | -54.7% | -47.3% | -17.8% |
| 1 | 3 | 5305ns | +1.8% | -35.0% | -42.4% | -49.3% | +2.3% |
| 1 | 4 | 4397ns | -58.6% | -39.0% | -15.8% | -51.6% | +0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.023 | ok |
| warm-clamp-accfit | -0.528 | HIGH- (thermal bounce) |
| warm-clamp-accfit-dyn | -0.023 | ok |
| warm-clamp-head | 0.010 | ok |
| warm-clamp-min-lanes | 0.029 | ok |
| warm-clamp-minimum | -0.734 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-clamp-accfit**: won 3/4, lost 1/4
- **warm-clamp-accfit-dyn**: won 4/4, lost 0/4
- **warm-clamp-head**: won 4/4, lost 0/4
- **warm-clamp-min-lanes**: won 4/4, lost 0/4
- **warm-clamp-minimum**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 27.1ns | 5609.7ns | 0.5% |  |
| warm-clamp-accfit | 48.1ns | 3675.9ns | 1.3% |  |
| warm-clamp-accfit-dyn | 52.5ns | 3968.5ns | 1.3% |  |
| warm-clamp-head | 37.9ns | 4727.6ns | 0.8% |  |
| warm-clamp-min-lanes | 35.1ns | 3702.7ns | 0.9% |  |
| warm-clamp-minimum | 27.0ns | 5741.1ns | 0.5% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 4396.4-7679.6 ns)
   4396.4 |##########
   4560.5 |
   4724.7 |
   4888.8 |
   5053.0 |
   5217.2 |########################################
   5381.3 |#########################
   5545.5 |##
   5709.7 |
   5873.8 |
   6038.0 |
   6202.1 |
   6366.3 |
   6530.5 |
   6694.6 |
   6858.8 |
   7023.0 |
   7187.1 |
   7351.3 |
   7515.4 |
  (5 below, 4 above range)

warm-clamp-accfit (n=40, range 1541.9-6050.2 ns)
   1541.9 |##########
   1767.4 |########################################
   1992.8 |##########
   2218.2 |
   2443.6 |
   2669.0 |
   2894.4 |
   3119.8 |
   3345.3 |
   3570.7 |
   3796.1 |
   4021.5 |
   4246.9 |
   4472.3 |##########
   4697.8 |###############
   4923.2 |#####
   5148.6 |##############################
   5374.0 |##########
   5599.4 |##########
   5824.8 |##########
  (7 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 1879.0-6496.9 ns)
   1879.0 |########################################
   2109.9 |
   2340.8 |
   2571.7 |
   2802.6 |
   3033.5 |##
   3264.4 |##
   3495.3 |
   3726.2 |
   3957.1 |
   4188.0 |
   4418.9 |
   4649.8 |
   4880.6 |#####
   5111.5 |#####
   5342.4 |#####
   5573.3 |########
   5804.2 |#####
   6035.1 |########
   6266.0 |########
  (4 below, 3 above range)

warm-clamp-head (n=40, range 2486.2-8744.4 ns)
   2486.2 |#####################
   2799.1 |########################################
   3112.0 |###
   3424.9 |
   3737.8 |
   4050.8 |
   4363.7 |
   4676.6 |###
   4989.5 |
   5302.4 |
   5615.3 |
   5928.2 |
   6241.1 |###
   6554.0 |
   6866.9 |###
   7179.8 |######
   7492.7 |###
   7805.7 |###
   8118.6 |#########
   8431.5 |######
  (4 below, 3 above range)

warm-clamp-min-lanes (n=40, range 2126.8-7634.7 ns)
   2126.8 |########################################
   2402.1 |####################################
   2677.5 |####################
   2952.9 |
   3228.3 |
   3503.7 |####
   3779.1 |
   4054.5 |####
   4329.9 |
   4605.3 |
   4880.7 |
   5156.1 |
   5431.5 |
   5706.9 |########
   5982.3 |####
   6257.7 |####
   6533.1 |####
   6808.5 |####
   7083.9 |
   7359.3 |
  (4 below, 4 above range)

warm-clamp-minimum (n=40, range 4412.9-9119.9 ns)
   4412.9 |#####################################
   4648.2 |
   4883.6 |
   5118.9 |##
   5354.3 |########################################
   5589.6 |
   5825.0 |
   6060.3 |
   6295.7 |##
   6531.0 |
   6766.4 |
   7001.7 |
   7237.1 |
   7472.4 |
   7707.8 |
   7943.1 |
   8178.5 |
   8413.8 |
   8649.2 |
   8884.5 |##
  (1 below, 3 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=27.4% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: CV=51.5% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **warm-clamp-accfit-dyn**: CV=50.5% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **warm-clamp-head**: CV=54.5% (high variance, measurements may be unstable)
- **warm-clamp-head**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **warm-clamp-min-lanes**: CV=59.7% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=42.6% (high variance, measurements may be unstable)
