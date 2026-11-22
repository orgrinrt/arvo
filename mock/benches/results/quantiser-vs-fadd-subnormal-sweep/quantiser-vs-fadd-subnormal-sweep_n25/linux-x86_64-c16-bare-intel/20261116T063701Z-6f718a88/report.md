# Round-first software quantiser add vs native hardware fadd, subnormal fraction swept

2 variants, 40 samples per variant.
Baseline: **quantiser-hardware**

## Highlights

Baseline for all deltas below: **quantiser-hardware**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### quantiser-hardware dominates: 412% faster than the next best (quantiser-software)

quantiser-hardware (825 ns) leads quantiser-software (4.22 us) by 412%, a clear separation rather than a photo finish. CV 12.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### quantiser-hardware shows alternating (throttle bounce) (autocorr -0.59)

quantiser-hardware's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (quantiser-hardware)

The baseline quantiser-hardware is the fastest (825 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 5.1x the fastest

Fastest quantiser-hardware (825 ns) to slowest quantiser-software (4.22 us): 5.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### quantiser-software is inconsistent: worst-20% is 3.5x its best-20%

quantiser-software's best 20% of batches run at 3.48 us but its worst 20% at 12.12 us (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (quantiser-hardware) is the fastest** at 825.1 ns median
- 1 variant significantly slower than baseline
- Spread: 5.12x (fastest 825.1 ns, slowest 4221.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Round-first software quantiser add vs native hardware fadd, subnormal fraction swept |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112891188 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| quantiser-hardware | 1161ns | 1122ns | 985ns | 1131ns | 1428ns | base |
| quantiser-software | 5767ns | 4294ns | 3543ns | 4294ns | 12411ns | +396.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| quantiser-hardware | 843ns | 728ns | 1016ns | base | 0.304 |
| quantiser-software | 5655ns | 3483ns | 12123ns | +570.74% | 0.045 |

## Performance model

- Peak throughput: **0.351 Gops/s** (quantiser-hardware; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| quantiser-hardware | 0.310 | 88.3% |
| quantiser-software | 0.061 | 17.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| quantiser-hardware | 1161ns | 1161ns | base |
| quantiser-software | 5767ns | 5767ns | +396.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| quantiser-hardware | 825ns | base | --- | [787, 839] | --- | --- | --- | --- |
| quantiser-software | 4222ns | +3422.8ns (+414.8%) | [+3112, +3438]ns | [3844, 4265] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | quantiser-hardware | quantiser-software |
|---|---|---|---|
| 1 | 1 | 758ns | +369.2% |
| 1 | 2 | 988ns | +252.0% |
| 1 | 3 | 809ns | +421.7% |
| 1 | 4 | 833ns | +412.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| quantiser-hardware | -0.590 | HIGH- (thermal bounce) |
| quantiser-software | 0.236 | moderate+ |

**Consistency summary:**

- **quantiser-software**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| quantiser-hardware | 107.7ns | 843.1ns | 12.8% | HIGH |
| quantiser-software | 32.0ns | 5655.2ns | 0.6% |  |

## Distribution (algo ns)

```
quantiser-hardware (n=40, range 728.4-1016.2 ns)
    728.4 |###########
    742.8 |#####
    757.2 |############################
    771.6 |###########
    785.9 |###########
    800.3 |###########
    814.7 |######################
    829.1 |########################################
    843.5 |
    857.9 |#####
    872.3 |#####
    886.7 |#####
    901.0 |
    915.4 |
    929.8 |###########
    944.2 |#####
    958.6 |
    973.0 |
    987.4 |###########
   1001.8 |
  (3 below, 4 above range)

quantiser-software (n=40, range 3483.2-12122.6 ns)
   3483.2 |####################
   3915.2 |########################################
   4347.2 |##
   4779.1 |
   5211.1 |
   5643.1 |
   6075.0 |
   6507.0 |
   6939.0 |
   7370.9 |##
   7802.9 |
   8234.9 |
   8666.8 |
   9098.8 |
   9530.8 |
   9962.7 |
  10394.7 |#####
  10826.7 |
  11258.6 |#####
  11690.6 |##
  (6 below, 3 above range)

```

## Diagnostics

- **quantiser-hardware**: bridge=12.9% of algo (FFI overhead may distort results)
- **quantiser-software**: CV=59.6% (high variance, measurements may be unstable)
- **quantiser-software**: worst_20/best_20 = 3.5x (possible bimodal distribution)
