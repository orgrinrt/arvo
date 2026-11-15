# Round-first software quantiser add vs native hardware fadd, subnormal fraction swept

2 variants, 40 samples per variant.
Baseline: **quantiser-hardware**

## Highlights

Baseline for all deltas below: **quantiser-hardware**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### quantiser-hardware dominates: 1111% faster than the next best (quantiser-software)

quantiser-hardware (800 ns) leads quantiser-software (9.69 us) by 1111%, a clear separation rather than a photo finish. CV 21.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### quantiser-software shows alternating (throttle bounce) (autocorr -0.72)

quantiser-software's per-pass series has lag-1 autocorrelation -0.72, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (quantiser-hardware)

The baseline quantiser-hardware is the fastest (800 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 12.1x the fastest

Fastest quantiser-hardware (800 ns) to slowest quantiser-software (9.69 us): 12.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### quantiser-software is inconsistent: worst-20% is 3.8x its best-20%

quantiser-software's best 20% of batches run at 2.82 us but its worst 20% at 10.68 us (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (quantiser-hardware) is the fastest** at 800.2 ns median
- 1 variant significantly slower than baseline
- Spread: 12.11x (fastest 800.2 ns, slowest 9690.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| quantiser-hardware | 1137ns | 1121ns | 812ns | 1117ns | 1521ns | base |
| quantiser-software | 7609ns | 9797ns | 2875ns | 8086ns | 10911ns | +569.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-hardware | 803ns | 578ns | 1062ns | base | 0.319 |
| quantiser-software | 7489ns | 2819ns | 10685ns | +833.06% | 0.034 |

## Performance model

- Peak throughput: **0.443 Gops/s** (quantiser-hardware; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-hardware | 0.320 | 72.2% |
| quantiser-software | 0.026 | 6.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-hardware | 1137ns | 1137ns | base |
| quantiser-software | 7609ns | 7609ns | +569.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-hardware | 800ns | base | --- | [738, 843] | --- | --- | --- | --- |
| quantiser-software | 9690ns | +8705.3ns (+1087.8%) | [+4820, +8849]ns | [5395, 9698] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | quantiser-hardware | quantiser-software |
|---|---|---|---|
| 1 | 1 | 810ns | +1096.9% |
| 1 | 2 | 570ns | +530.9% |
| 1 | 3 | 940ns | +933.8% |
| 1 | 4 | 800ns | +252.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-hardware | -0.515 | HIGH- (thermal bounce) |
| quantiser-software | -0.716 | HIGH- (thermal bounce) |

**Consistency summary:**

- **quantiser-software**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-hardware | 117.6ns | 802.7ns | 14.7% | HIGH |
| quantiser-software | 36.7ns | 7489.3ns | 0.5% |  |

## Distribution (algo ns)

```
quantiser-hardware (n=40, range 578.1-1062.3 ns)
    578.1 |
    602.3 |#################
    626.5 |
    650.7 |
    674.9 |
    699.1 |#################
    723.3 |######################
    747.6 |
    771.8 |###########
    796.0 |########################################
    820.2 |###########
    844.4 |###########
    868.6 |#####
    892.8 |
    917.1 |######################
    941.3 |#####
    965.5 |
    989.7 |
   1013.9 |
   1038.1 |#####
  (6 below, 4 above range)

quantiser-software (n=40, range 2818.7-10684.7 ns)
   2818.7 |########
   3212.0 |############
   3605.3 |
   3998.6 |
   4391.9 |
   4785.2 |
   5178.5 |
   5571.8 |
   5965.1 |
   6358.4 |
   6751.7 |
   7145.0 |##
   7538.3 |
   7931.6 |##
   8324.9 |
   8718.2 |
   9111.5 |##
   9504.8 |########################################
   9898.1 |
  10291.4 |##
  (4 below, 3 above range)

```

## Diagnostics

- **quantiser-hardware**: CV=21.3% (high variance, measurements may be unstable)
- **quantiser-hardware**: bridge=15.1% of algo (FFI overhead may distort results)
- **quantiser-software**: CV=44.2% (high variance, measurements may be unstable)
- **quantiser-software**: worst_20/best_20 = 3.8x (possible bimodal distribution)
