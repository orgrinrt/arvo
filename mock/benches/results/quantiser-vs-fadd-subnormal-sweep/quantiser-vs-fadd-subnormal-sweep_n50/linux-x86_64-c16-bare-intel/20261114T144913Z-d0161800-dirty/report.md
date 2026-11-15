# Round-first software quantiser add vs native hardware fadd, subnormal fraction swept

2 variants, 40 samples per variant.
Baseline: **quantiser-hardware**

## Highlights

Baseline for all deltas below: **quantiser-hardware**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### quantiser-hardware dominates: 466% faster than the next best (quantiser-software)

quantiser-hardware (699 ns) leads quantiser-software (3.96 us) by 466%, a clear separation rather than a photo finish. CV 20.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### quantiser-hardware shows alternating (throttle bounce) (autocorr -0.63)

quantiser-hardware's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (quantiser-hardware)

The baseline quantiser-hardware is the fastest (699 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 5.7x the fastest

Fastest quantiser-hardware (699 ns) to slowest quantiser-software (3.96 us): 5.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### quantiser-software is inconsistent: worst-20% is 3.4x its best-20%

quantiser-software's best 20% of batches run at 3.39 us but its worst 20% at 11.69 us (3.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (quantiser-hardware) is the fastest** at 699.4 ns median
- 1 variant significantly slower than baseline
- Spread: 5.66x (fastest 699.4 ns, slowest 3960.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| quantiser-hardware | 1004ns | 980ns | 838ns | 951ns | 1331ns | base |
| quantiser-software | 5866ns | 4027ns | 3447ns | 4634ns | 11979ns | +484.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-hardware | 702ns | 598ns | 908ns | base | 0.365 |
| quantiser-software | 5748ns | 3387ns | 11685ns | +718.98% | 0.045 |

## Performance model

- Peak throughput: **0.428 Gops/s** (quantiser-hardware; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-hardware | 0.366 | 85.4% |
| quantiser-software | 0.065 | 15.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-hardware | 1004ns | 1004ns | base |
| quantiser-software | 5866ns | 5866ns | +484.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-hardware | 699ns | base | --- | [624, 701] | --- | --- | --- | --- |
| quantiser-software | 3960ns | +3277.5ns (+468.6%) | [+3267, +3407]ns | [3928, 3973] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | quantiser-hardware | quantiser-software |
|---|---|---|---|
| 1 | 1 | 702ns | +465.9% |
| 1 | 2 | 644ns | +505.3% |
| 1 | 3 | 700ns | +362.5% |
| 1 | 4 | 623ns | +537.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-hardware | -0.632 | HIGH- (thermal bounce) |
| quantiser-software | -0.395 | moderate- |

**Consistency summary:**

- **quantiser-software**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-hardware | 101.3ns | 701.8ns | 14.4% | HIGH |
| quantiser-software | 34.6ns | 5747.6ns | 0.6% |  |

## Distribution (algo ns)

```
quantiser-hardware (n=40, range 597.5-907.6 ns)
    597.5 |##
    613.0 |################################
    628.5 |
    644.1 |##
    659.6 |##
    675.1 |
    690.6 |########################################
    706.1 |
    721.6 |
    737.1 |
    752.6 |
    768.1 |
    783.6 |
    799.1 |##
    814.6 |
    830.1 |
    845.6 |
    861.1 |
    876.6 |
    892.1 |
  (3 below, 4 above range)

quantiser-software (n=40, range 3387.2-11685.5 ns)
   3387.2 |
   3802.1 |########################################
   4217.0 |
   4631.9 |
   5046.9 |
   5461.8 |
   5876.7 |#
   6291.6 |#
   6706.5 |
   7121.4 |
   7536.3 |#
   7951.2 |
   8366.2 |
   8781.1 |
   9196.0 |
   9610.9 |#
  10025.8 |#
  10440.7 |
  10855.6 |#
  11270.5 |#####
  (6 below, 3 above range)

```

## Diagnostics

- **quantiser-hardware**: bridge=15.1% of algo (FFI overhead may distort results)
- **quantiser-software**: CV=56.2% (high variance, measurements may be unstable)
- **quantiser-software**: worst_20/best_20 = 3.4x (possible bimodal distribution)
