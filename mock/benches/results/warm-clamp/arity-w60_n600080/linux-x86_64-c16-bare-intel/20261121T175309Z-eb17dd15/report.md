# Clamping fold at 60 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-clamp-acc64) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-clamp-acc64 has the worst median (3.61 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-clamp-accfit at 2.09 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-clamp-accfit beats baseline by 34% (significant)

warm-clamp-accfit is -1.22 us (34%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-accfit is fastest but the noisiest (CV 108.7%)

warm-clamp-accfit wins on median (2.09 us) yet has the highest variance (CV 108.7%), while warm-clamp-acc64 is the steadiest (CV 59.5%, 3.61 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.52 us) is smaller than the fastest variant's own run-to-run std-dev (2.27 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### warm-clamp-accfit is inconsistent: worst-20% is 4.1x its best-20%

warm-clamp-accfit's best 20% of batches run at 1.79 us but its worst 20% at 7.37 us (4.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-accfit** at 2089.4 ns median (-42.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.73x (fastest 2089.4 ns, slowest 3609.7 ns)

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
| warm-clamp-acc64 | 4247ns | 3694ns | 3016ns | 3476ns | 7790ns | base |
| warm-clamp-accfit | 3562ns | 2169ns | 1858ns | 2744ns | 7721ns | -16.13% |
| warm-clamp-accfit-dyn | 4381ns | 2775ns | 1966ns | 3984ns | 7988ns | +3.15% |
| warm-clamp-head | 4304ns | 3164ns | 2589ns | 3233ns | 9234ns | +1.35% |
| warm-clamp-min-lanes | 3842ns | 2304ns | 2204ns | 2965ns | 8110ns | -9.54% |
| warm-clamp-minimum | 4328ns | 3138ns | 3043ns | 3417ns | 8349ns | +1.91% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 4141ns | 2948ns | 7563ns | base | 1.978 |
| warm-clamp-accfit | 3417ns | 1790ns | 7373ns | -17.47% | 2.397 |
| warm-clamp-accfit-dyn | 4207ns | 1893ns | 7638ns | +1.60% | 1.947 |
| warm-clamp-head | 4179ns | 2520ns | 8933ns | +0.93% | 1.960 |
| warm-clamp-min-lanes | 3716ns | 2136ns | 7815ns | -10.27% | 2.205 |
| warm-clamp-minimum | 4222ns | 2975ns | 8111ns | +1.97% | 1.940 |

## Performance model

- Peak throughput: **4.576 Gops/s** (warm-clamp-accfit; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 2.269 | 49.6% |
| warm-clamp-accfit | 3.921 | 85.7% |
| warm-clamp-accfit-dyn | 3.068 | 67.0% |
| warm-clamp-head | 2.659 | 58.1% |
| warm-clamp-min-lanes | 3.664 | 80.1% |
| warm-clamp-minimum | 2.669 | 58.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 4247ns | 4247ns | base |
| warm-clamp-accfit | 3562ns | 3562ns | -16.13% |
| warm-clamp-accfit-dyn | 4381ns | 4381ns | +3.15% |
| warm-clamp-head | 4304ns | 4304ns | +1.35% |
| warm-clamp-min-lanes | 3842ns | 3842ns | -9.54% |
| warm-clamp-minimum | 4328ns | 4328ns | +1.91% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3610ns | base | --- | [3015, 3638] | --- | --- | --- | --- |
| warm-clamp-accfit | 2089ns | -1220.4ns (-33.8%) | [-1458, -1152]ns | [1791, 2909] | YES | 0.0017 | 0.0007 | 0 |
| warm-clamp-accfit-dyn | 2670ns | no significant difference | [-1438, +1150]ns | [2202, 5454] | no | 0.4470 | 0.2682 | 0 |
| warm-clamp-head | 3081ns | no significant difference | [-537, +77]ns | [3078, 3092] | no | 1.0000 | 1.0000 | 0 |
| warm-clamp-min-lanes | 2236ns | -864.2ns (-23.9%) | [-1426, -547]ns | [2218, 2672] | YES | 0.0009 | 0.0002 | 0 |
| warm-clamp-minimum | 3069ns | no significant difference | [-90, +119]ns | [3063, 3776] | no | 0.5370 | 0.4296 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2949ns | +26.4% | -25.1% | +4.7% | -8.2% | +3.9% |
| 1 | 2 | 3658ns | -32.3% | -50.5% | -31.1% | -39.1% | +3.5% |
| 1 | 3 | 3617ns | -50.5% | -39.8% | -14.8% | -39.1% | -15.3% |
| 1 | 4 | 3015ns | -40.6% | +80.9% | +2.4% | -26.0% | -0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.252 | moderate- |
| warm-clamp-accfit | 0.179 | ok |
| warm-clamp-accfit-dyn | -0.033 | ok |
| warm-clamp-head | -0.415 | moderate- |
| warm-clamp-min-lanes | -0.059 | ok |
| warm-clamp-minimum | -0.350 | moderate- |

**Consistency summary:**

- **warm-clamp-accfit**: won 3/4, lost 1/4
- **warm-clamp-accfit-dyn**: won 3/4, lost 1/4
- **warm-clamp-head**: won 2/4, lost 2/4
- **warm-clamp-min-lanes**: won 4/4, lost 0/4
- **warm-clamp-minimum**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 26.8ns | 4140.7ns | 0.6% |  |
| warm-clamp-accfit | 36.1ns | 3417.5ns | 1.1% |  |
| warm-clamp-accfit-dyn | 44.0ns | 4207.2ns | 1.0% |  |
| warm-clamp-head | 31.8ns | 4179.1ns | 0.8% |  |
| warm-clamp-min-lanes | 33.1ns | 3715.5ns | 0.9% |  |
| warm-clamp-minimum | 26.7ns | 4222.2ns | 0.6% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 2947.8-7562.6 ns)
   2947.8 |########################################
   3178.5 |
   3409.3 |##############################
   3640.0 |#######################
   3870.8 |
   4101.5 |
   4332.3 |
   4563.0 |###
   4793.7 |
   5024.5 |
   5255.2 |
   5486.0 |
   5716.7 |
   5947.4 |######
   6178.2 |
   6408.9 |
   6639.7 |
   6870.4 |###
   7101.2 |
   7331.9 |
  (5 below, 3 above range)

warm-clamp-accfit (n=40, range 1790.2-7372.8 ns)
   1790.2 |########################################
   2069.4 |############
   2348.5 |
   2627.6 |##
   2906.7 |##
   3185.9 |
   3465.0 |
   3744.1 |##
   4023.3 |##
   4302.4 |
   4581.5 |
   4860.7 |
   5139.8 |
   5418.9 |#####
   5698.1 |##
   5977.2 |##
   6256.3 |##
   6535.4 |##
   6814.6 |
   7093.7 |##
  (4 below, 4 above range)

warm-clamp-accfit-dyn (n=40, range 1893.0-7637.8 ns)
   1893.0 |######################
   2180.3 |########################################
   2467.5 |
   2754.7 |####
   3042.0 |
   3329.2 |
   3616.5 |
   3903.7 |
   4190.9 |
   4478.2 |
   4765.4 |
   5052.6 |#################
   5339.9 |########
   5627.1 |########
   5914.3 |####
   6201.6 |####
   6488.8 |
   6776.1 |########
   7063.3 |########
   7350.5 |####
  (6 below, 4 above range)

warm-clamp-head (n=40, range 2520.1-8933.3 ns)
   2520.1 |#######
   2840.8 |########################################
   3161.4 |
   3482.1 |
   3802.7 |
   4123.4 |
   4444.1 |
   4764.7 |
   5085.4 |#
   5406.1 |
   5726.7 |
   6047.4 |
   6368.0 |
   6688.7 |
   7009.4 |#
   7330.0 |
   7650.7 |
   7971.4 |#
   8292.0 |#
   8612.7 |
  (5 below, 5 above range)

warm-clamp-min-lanes (n=40, range 2135.6-7815.2 ns)
   2135.6 |########################################
   2419.6 |##############
   2703.6 |##
   2987.6 |
   3271.5 |
   3555.5 |
   3839.5 |
   4123.5 |
   4407.4 |
   4691.4 |
   4975.4 |
   5259.4 |
   5543.3 |
   5827.3 |
   6111.3 |####
   6395.3 |
   6679.2 |
   6963.2 |##
   7247.2 |####
   7531.2 |####
  (5 below, 4 above range)

warm-clamp-minimum (n=40, range 2974.8-8111.2 ns)
   2974.8 |########################################
   3231.6 |
   3488.4 |##
   3745.2 |#################
   4002.1 |
   4258.9 |##
   4515.7 |
   4772.5 |##
   5029.3 |
   5286.2 |
   5543.0 |
   5799.8 |
   6056.6 |
   6313.4 |
   6570.3 |
   6827.1 |
   7083.9 |
   7340.7 |
   7597.5 |
   7854.4 |##
  (5 below, 5 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=51.9% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: CV=66.5% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **warm-clamp-accfit-dyn**: CV=55.2% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **warm-clamp-head**: CV=58.6% (high variance, measurements may be unstable)
- **warm-clamp-head**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **warm-clamp-min-lanes**: CV=63.0% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=52.4% (high variance, measurements may be unstable)
