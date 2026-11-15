# Clamping fold at 64 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-accfit-dyn dominates: 14% faster than the next best (warm-clamp-minimum)

warm-clamp-accfit-dyn (1.88 us) leads warm-clamp-minimum (2.14 us) by 14%, a clear separation rather than a photo finish. CV 116.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-minimum beats baseline by 21% (significant)

warm-clamp-minimum is -472 ns (21%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-accfit-dyn is fastest but the noisiest (CV 116.8%)

warm-clamp-accfit-dyn wins on median (1.88 us) yet has the highest variance (CV 116.8%), while warm-clamp-accfit is the steadiest (CV 59.7%, 3.75 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-clamp-accfit-dyn shows alternating (throttle bounce) (autocorr -0.72)

warm-clamp-accfit-dyn's per-pass series has lag-1 autocorrelation -0.72, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.87 us) is smaller than the fastest variant's own run-to-run std-dev (2.20 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### warm-clamp-accfit is inconsistent: worst-20% is 4.5x its best-20%

warm-clamp-accfit's best 20% of batches run at 1.53 us but its worst 20% at 6.95 us (4.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-accfit-dyn** at 1879.3 ns median (-17.8% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.99x (fastest 1879.3 ns, slowest 3749.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 3766ns | 2382ns | 2354ns | 2860ns | 7893ns | base |
| warm-clamp-accfit | 4082ns | 3921ns | 1601ns | 3828ns | 7324ns | +8.40% |
| warm-clamp-accfit-dyn | 3767ns | 1966ns | 1687ns | 3301ns | 7245ns | +0.04% |
| warm-clamp-head | 4309ns | 3174ns | 3155ns | 3174ns | 8866ns | +14.42% |
| warm-clamp-min-lanes | 4001ns | 2396ns | 1940ns | 3183ns | 8516ns | +6.26% |
| warm-clamp-minimum | 3667ns | 2246ns | 1565ns | 3305ns | 6858ns | -2.60% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 3605ns | 2259ns | 7525ns | base | 2.272 |
| warm-clamp-accfit | 3888ns | 1530ns | 6949ns | +7.85% | 2.107 |
| warm-clamp-accfit-dyn | 3589ns | 1613ns | 6868ns | -0.45% | 2.283 |
| warm-clamp-head | 4181ns | 3069ns | 8573ns | +15.98% | 1.959 |
| warm-clamp-min-lanes | 3830ns | 1863ns | 8115ns | +6.23% | 2.139 |
| warm-clamp-minimum | 3481ns | 1490ns | 6484ns | -3.45% | 2.354 |

## Performance model

- Peak throughput: **5.499 Gops/s** (warm-clamp-minimum; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 3.582 | 65.1% |
| warm-clamp-accfit | 2.185 | 39.7% |
| warm-clamp-accfit-dyn | 4.359 | 79.3% |
| warm-clamp-head | 2.653 | 48.2% |
| warm-clamp-min-lanes | 3.558 | 64.7% |
| warm-clamp-minimum | 3.829 | 69.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 3766ns | 3766ns | base |
| warm-clamp-accfit | 4082ns | 4082ns | +8.40% |
| warm-clamp-accfit-dyn | 3767ns | 3767ns | +0.04% |
| warm-clamp-head | 4309ns | 4309ns | +14.42% |
| warm-clamp-min-lanes | 4001ns | 4001ns | +6.26% |
| warm-clamp-minimum | 3667ns | 3667ns | -2.60% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 2287ns | base | --- | [2282, 2300] | --- | --- | --- | --- |
| warm-clamp-accfit | 3749ns | -418.0ns (-18.3%) | [-721, -2]ns | [1870, 5320] | YES (adj: no) | 0.1009 | 0.0807 | 0 |
| warm-clamp-accfit-dyn | 1879ns | -414.6ns (-18.1%) | [-457, -400]ns | [1866, 5159] | YES | 0.0011 | 0.0007 | 0 |
| warm-clamp-head | 3088ns | +807.8ns (+35.3%) | [+790, +818]ns | [3082, 3094] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 2302ns | no significant difference | [-394, +29]ns | [2093, 3339] | no | 0.4296 | 0.4296 | 0 |
| warm-clamp-minimum | 2139ns | -471.7ns (-20.6%) | [-752, -442]ns | [1826, 4747] | YES | 0.0011 | 0.0007 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 2431ns | +115.9% | -36.9% | +27.0% | -21.9% | +69.6% |
| 1 | 2 | 2262ns | +12.2% | +49.5% | +36.3% | +1.3% | +104.8% |
| 1 | 3 | 2279ns | -25.5% | -17.8% | +35.8% | -18.4% | -34.4% |
| 1 | 4 | 2290ns | +62.0% | +12.7% | +34.7% | +5.6% | -20.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.181 | ok |
| warm-clamp-accfit | -0.130 | ok |
| warm-clamp-accfit-dyn | -0.716 | HIGH- (thermal bounce) |
| warm-clamp-head | -0.658 | HIGH- (thermal bounce) |
| warm-clamp-min-lanes | -0.681 | HIGH- (thermal bounce) |
| warm-clamp-minimum | 0.151 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 1/4, lost 3/4
- **warm-clamp-accfit-dyn**: won 2/4, lost 2/4
- **warm-clamp-head**: won 0/4, lost 4/4
- **warm-clamp-min-lanes**: won 2/4, lost 2/4
- **warm-clamp-minimum**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 39.7ns | 3605.2ns | 1.1% |  |
| warm-clamp-accfit | 59.7ns | 3888.2ns | 1.5% |  |
| warm-clamp-accfit-dyn | 48.2ns | 3588.9ns | 1.3% |  |
| warm-clamp-head | 33.0ns | 4181.2ns | 0.8% |  |
| warm-clamp-min-lanes | 42.0ns | 3829.7ns | 1.1% |  |
| warm-clamp-minimum | 46.9ns | 3480.8ns | 1.3% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 2259.2-7525.2 ns)
   2259.2 |########################################
   2522.5 |#
   2785.8 |
   3049.1 |
   3312.4 |
   3575.7 |
   3839.0 |
   4102.3 |
   4365.6 |
   4628.9 |
   4892.2 |#
   5155.5 |
   5418.8 |
   5682.1 |
   5945.4 |
   6208.7 |###
   6472.0 |#
   6735.3 |###
   6998.6 |#
   7261.9 |#
  (5 below, 3 above range)

warm-clamp-accfit (n=40, range 1530.3-6949.3 ns)
   1530.3 |############################
   1801.3 |########################################
   2072.2 |#####
   2343.2 |#####
   2614.1 |
   2885.1 |
   3156.0 |
   3427.0 |#####
   3697.9 |#####
   3968.9 |
   4239.8 |
   4510.8 |
   4781.7 |#####
   5052.7 |######################
   5323.6 |#####
   5594.6 |############################
   5865.5 |#################
   6136.5 |#####
   6407.4 |
   6678.4 |###########
  (5 below, 2 above range)

warm-clamp-accfit-dyn (n=40, range 1612.9-6868.4 ns)
   1612.9 |########################################
   1875.7 |#########
   2138.5 |
   2401.3 |
   2664.0 |###
   2926.8 |
   3189.6 |###
   3452.4 |
   3715.1 |###
   3977.9 |
   4240.7 |
   4503.4 |
   4766.2 |###
   5029.0 |
   5291.8 |###
   5554.5 |######
   5817.3 |###
   6080.1 |#########
   6342.9 |###
   6605.6 |############
  (6 below, 2 above range)

warm-clamp-head (n=40, range 3068.8-8572.7 ns)
   3068.8 |########################################
   3344.0 |
   3619.2 |
   3894.4 |
   4169.6 |
   4444.8 |
   4719.9 |#
   4995.1 |
   5270.3 |
   5545.5 |
   5820.7 |#
   6095.9 |
   6371.1 |
   6646.3 |
   6921.5 |
   7196.7 |
   7471.9 |#
   7747.1 |
   8022.3 |#
   8297.5 |
  (3 below, 4 above range)

warm-clamp-min-lanes (n=40, range 1862.8-8114.9 ns)
   1862.8 |################################
   2175.4 |########################################
   2488.0 |###
   2800.6 |
   3113.2 |###
   3425.8 |
   3738.4 |
   4051.0 |###
   4363.6 |
   4676.2 |
   4988.8 |
   5301.4 |
   5614.0 |
   5926.6 |###
   6239.2 |
   6551.8 |###
   6864.4 |#######
   7177.0 |###
   7489.7 |###
   7802.3 |###
  (5 below, 5 above range)

warm-clamp-minimum (n=40, range 1489.7-6484.1 ns)
   1489.7 |##############
   1739.4 |########################################
   1989.1 |
   2238.8 |###
   2488.6 |###
   2738.3 |
   2988.0 |###
   3237.7 |
   3487.4 |
   3737.2 |
   3986.9 |
   4236.6 |
   4486.3 |##########
   4736.0 |###
   4985.8 |###
   5235.5 |#######
   5485.2 |##########
   5734.9 |#######
   5984.6 |
   6234.4 |###
  (5 below, 4 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=61.3% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **warm-clamp-accfit**: CV=57.6% (high variance, measurements may be unstable)
- **warm-clamp-accfit**: worst_20/best_20 = 4.5x (possible bimodal distribution)
- **warm-clamp-accfit-dyn**: CV=61.2% (high variance, measurements may be unstable)
- **warm-clamp-accfit-dyn**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **warm-clamp-head**: CV=57.1% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: CV=66.9% (high variance, measurements may be unstable)
- **warm-clamp-min-lanes**: worst_20/best_20 = 4.4x (possible bimodal distribution)
- **warm-clamp-minimum**: CV=58.8% (high variance, measurements may be unstable)
- **warm-clamp-minimum**: worst_20/best_20 = 4.4x (possible bimodal distribution)
