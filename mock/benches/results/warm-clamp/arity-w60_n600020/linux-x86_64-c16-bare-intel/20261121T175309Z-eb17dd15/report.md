# Clamping fold at 60 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-clamp-acc64) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-clamp-acc64 has the worst median (3.87 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-clamp-min-lanes at 2.48 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-clamp-min-lanes dominates: 23% faster than the next best (warm-clamp-head)

warm-clamp-min-lanes (2.48 us) leads warm-clamp-head (3.06 us) by 23%, a clear separation rather than a photo finish. CV 87.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-min-lanes is fastest but the noisiest (CV 87.5%)

warm-clamp-min-lanes wins on median (2.48 us) yet has the highest variance (CV 87.5%), while warm-clamp-acc64 is the steadiest (CV 29.4%, 3.87 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.39 us) is smaller than the fastest variant's own run-to-run std-dev (2.17 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### warm-clamp-accfit-dyn is inconsistent: worst-20% is 4.5x its best-20%

warm-clamp-accfit-dyn's best 20% of batches run at 1.01 us but its worst 20% at 4.59 us (4.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-min-lanes** at 2484.0 ns median (-35.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.56x (fastest 2484.0 ns, slowest 3872.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Clamping fold at 60 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112891686 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3659ns | 4182ns | 1426ns | 4028ns | 4784ns | base |
| warm-clamp-accfit | 3396ns | 3570ns | 1608ns | 3541ns | 4750ns | -7.18% |
| warm-clamp-accfit-dyn | 3137ns | 3788ns | 1082ns | 3216ns | 4958ns | -14.25% |
| warm-clamp-head | 4136ns | 3147ns | 2325ns | 3436ns | 8046ns | +13.04% |
| warm-clamp-min-lanes | 3841ns | 2575ns | 1973ns | 3266ns | 7437ns | +5.00% |
| warm-clamp-minimum | 3007ns | 3360ns | 1094ns | 3187ns | 4381ns | -17.80% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3409ns | 1332ns | 4444ns | base | 2.403 |
| warm-clamp-accfit | 3164ns | 1506ns | 4421ns | -7.18% | 2.589 |
| warm-clamp-accfit-dyn | 2920ns | 1011ns | 4586ns | -14.35% | 2.806 |
| warm-clamp-head | 4013ns | 2258ns | 7778ns | +17.71% | 2.042 |
| warm-clamp-min-lanes | 3693ns | 1899ns | 7119ns | +8.34% | 2.218 |
| warm-clamp-minimum | 2800ns | 1015ns | 4078ns | -17.85% | 2.925 |

## Performance model

- Peak throughput: **8.102 Gops/s** (warm-clamp-accfit-dyn; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.115 | 26.1% |
| warm-clamp-accfit | 2.449 | 30.2% |
| warm-clamp-accfit-dyn | 2.326 | 28.7% |
| warm-clamp-head | 2.673 | 33.0% |
| warm-clamp-min-lanes | 3.298 | 40.7% |
| warm-clamp-minimum | 2.603 | 32.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3659ns | 3659ns | base |
| warm-clamp-accfit | 3396ns | 3396ns | -7.18% |
| warm-clamp-accfit-dyn | 3137ns | 3137ns | -14.25% |
| warm-clamp-head | 4136ns | 4136ns | +13.04% |
| warm-clamp-min-lanes | 3841ns | 3841ns | +5.00% |
| warm-clamp-minimum | 3007ns | 3007ns | -17.80% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3873ns | base | --- | [3556, 4044] | --- | --- | --- | --- |
| warm-clamp-accfit | 3344ns | -355.0ns (-9.2%) | [-557, -18]ns | [2892, 3649] | YES (adj: no) | 0.0962 | 0.0385 | 0 |
| warm-clamp-accfit-dyn | 3521ns | no significant difference | [-268, +3]ns | [1249, 3960] | no | 0.2564 | 0.1539 | 0 |
| warm-clamp-head | 3064ns | no significant difference | [-582, +1290]ns | [2513, 3083] | no | 1.0000 | 1.0000 | 0 |
| warm-clamp-min-lanes | 2484ns | no significant difference | [-1190, +1170]ns | [2024, 4385] | no | 0.5370 | 0.4296 | 0 |
| warm-clamp-minimum | 3148ns | -626.9ns (-16.2%) | [-863, -26]ns | [2842, 3547] | YES | 0.0321 | 0.0064 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 4027ns | +4.5% | -13.2% | -37.1% | -53.3% | -14.0% |
| 1 | 2 | 3948ns | -14.8% | +4.4% | -35.9% | -37.0% | +0.1% |
| 1 | 3 | 3379ns | -16.6% | +3.0% | -8.9% | +13.2% | -69.7% |
| 1 | 4 | 3972ns | -4.5% | -31.9% | -36.9% | -33.3% | -24.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.339 | moderate- |
| warm-clamp-accfit | -0.158 | ok |
| warm-clamp-accfit-dyn | 0.026 | ok |
| warm-clamp-head | -0.447 | moderate- |
| warm-clamp-min-lanes | -0.066 | ok |
| warm-clamp-minimum | -0.327 | moderate- |

**Consistency summary:**

- **warm-clamp-accfit**: won 3/4, lost 1/4
- **warm-clamp-accfit-dyn**: won 2/4, lost 2/4
- **warm-clamp-head**: won 4/4, lost 0/4
- **warm-clamp-min-lanes**: won 3/4, lost 1/4
- **warm-clamp-minimum**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 72.4ns | 3408.9ns | 2.1% |  |
| warm-clamp-accfit | 67.0ns | 3164.2ns | 2.1% |  |
| warm-clamp-accfit-dyn | 63.2ns | 2919.9ns | 2.2% |  |
| warm-clamp-head | 33.6ns | 4012.7ns | 0.8% |  |
| warm-clamp-min-lanes | 37.7ns | 3693.3ns | 1.0% |  |
| warm-clamp-minimum | 60.0ns | 2800.3ns | 2.1% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1332.3-4444.4 ns)
   1332.3 |
   1487.9 |
   1643.5 |####
   1799.1 |####
   1954.7 |####
   2110.3 |
   2265.9 |
   2421.5 |
   2577.1 |
   2732.7 |
   2888.3 |
   3043.9 |
   3199.5 |#############
   3355.1 |####
   3510.7 |#################
   3666.3 |########
   3821.9 |##########################
   3977.5 |#############
   4133.1 |####
   4288.7 |########################################
  (6 below, 2 above range)

warm-clamp-accfit (n=40, range 1506.1-4420.6 ns)
   1506.1 |######
   1651.8 |
   1797.5 |
   1943.3 |
   2089.0 |
   2234.7 |######
   2380.4 |######
   2526.2 |####################
   2671.9 |#############
   2817.6 |####################
   2963.3 |######
   3109.1 |######
   3254.8 |##########################
   3400.5 |######
   3546.2 |##########################
   3692.0 |######
   3837.7 |####################
   3983.4 |######
   4129.1 |######
   4274.9 |########################################
  (5 below, 1 above range)

warm-clamp-accfit-dyn (n=40, range 1011.2-4585.9 ns)
   1011.2 |
   1189.9 |########################################
   1368.6 |
   1547.4 |####
   1726.1 |
   1904.8 |
   2083.6 |
   2262.3 |
   2441.1 |
   2619.8 |
   2798.5 |
   2977.3 |
   3156.0 |########
   3334.7 |########
   3513.5 |####
   3692.2 |########
   3871.0 |#################
   4049.7 |####
   4228.4 |###############################
   4407.2 |########
  (6 below, 3 above range)

warm-clamp-head (n=40, range 2257.6-7777.9 ns)
   2257.6 |########################################
   2533.6 |###
   2809.6 |##############################
   3085.6 |###
   3361.6 |
   3637.6 |
   3913.7 |###
   4189.7 |
   4465.7 |
   4741.7 |
   5017.7 |
   5293.7 |
   5569.7 |
   5845.8 |
   6121.8 |
   6397.8 |###
   6673.8 |######
   6949.8 |###
   7225.8 |#############
   7501.8 |
  (5 below, 3 above range)

warm-clamp-min-lanes (n=40, range 1899.0-7118.9 ns)
   1899.0 |########################################
   2160.0 |
   2421.0 |################################
   2682.0 |
   2943.0 |
   3204.0 |####
   3465.0 |
   3726.0 |
   3987.0 |
   4248.0 |
   4509.0 |
   4770.0 |
   5031.0 |
   5292.0 |####
   5553.0 |################
   5813.9 |########
   6074.9 |########
   6335.9 |####
   6596.9 |
   6857.9 |####
  (7 below, 3 above range)

warm-clamp-minimum (n=40, range 1015.3-4078.2 ns)
   1015.3 |##############################
   1168.5 |#####
   1321.6 |
   1474.7 |
   1627.9 |
   1781.0 |
   1934.2 |
   2087.3 |
   2240.5 |
   2393.6 |
   2546.7 |#####
   2699.9 |##########
   2853.0 |###############
   3006.2 |###############
   3159.3 |##########
   3312.5 |##########
   3465.6 |########################################
   3618.7 |##########
   3771.9 |
   3925.0 |##########
  (4 below, 4 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=33.4% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **warm-clamp-accfit**: CV=32.8% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: CV=51.4% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: worst_20/best_20 = 4.5x (possible bimodal distribution)
- **warm-clamp-head**: CV=55.4% (high variance, measurements may be unstable)
- **warm-clamp-head**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **warm-clamp-min-lanes**: CV=58.8% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=41.4% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 4.0x (possible bimodal distribution)
