# Layout::Bitpacked footprint: plan-driven sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-packed**

## Highlights

Baseline for all deltas below: **bitpack-footprint-packed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed dominates: 409% faster than the next best (bitpack-footprint-packed-naive)

bitpack-footprint-packed (530.86 us) leads bitpack-footprint-packed-naive (2.70 ms) by 409%, a clear separation rather than a photo finish. CV 4.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-footprint-packed-naive shows alternating (throttle bounce) (autocorr -0.70)

bitpack-footprint-packed-naive's per-pass series has lag-1 autocorrelation -0.70, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (bitpack-footprint-packed)

The baseline bitpack-footprint-packed is the fastest (530.86 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 5.1x the fastest

Fastest bitpack-footprint-packed (530.86 us) to slowest bitpack-footprint-packed-naive (2.70 ms): 5.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (bitpack-footprint-packed) is the fastest** at 530861.8 ns median
- 1 variant significantly slower than baseline
- Spread: 5.09x (fastest 530861.8 ns, slowest 2702147.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Layout::Bitpacked footprint: plan-driven sum swept past L1 and L2 |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112837746 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 538341ns | 531027ns | 516265ns | 533506ns | 574923ns | base |
| bitpack-footprint-packed-naive | 2720893ns | 2702375ns | 2695086ns | 2711077ns | 2776147ns | +405.42% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-packed | 538121ns | 516076ns | 574605ns | base | 7.794 |
| bitpack-footprint-packed-naive | 2720559ns | 2694845ns | 2775847ns | +405.57% | 1.542 |

## Performance model

- Peak throughput: **8.127 Gops/s** (bitpack-footprint-packed; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-packed | 7.901 | 97.2% |
| bitpack-footprint-packed-naive | 1.552 | 19.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-packed | 538341ns | 538341ns | base |
| bitpack-footprint-packed-naive | 2720893ns | 2720893ns | +405.42% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 530862ns | base | --- | [530013, 531224] | --- | --- | --- | --- |
| bitpack-footprint-packed-naive | 2702148ns | +2175532.1ns (+409.8%) | [+2164148, +2198434]ns | [2697015, 2728696] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|---|
| 1 | 1 | 530105ns | +414.8% |
| 1 | 2 | 574475ns | +369.4% |
| 1 | 3 | 516222ns | +438.7% |
| 1 | 4 | 531072ns | +407.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-packed | -0.486 | moderate- |
| bitpack-footprint-packed-naive | -0.698 | HIGH- (thermal bounce) |

**Consistency summary:**

- **bitpack-footprint-packed-naive**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-packed | 33.4ns | 538121.1ns | 0.0% |  |
| bitpack-footprint-packed-naive | 73.3ns | 2720559.2ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-packed (n=40, range 516076.0-574605.2 ns)
  516076.0 |########################
  519002.4 |
  521928.9 |
  524855.4 |
  527781.8 |####################################
  530708.3 |########################################
  533634.7 |####
  536561.2 |
  539487.7 |
  542414.1 |
  545340.6 |
  548267.0 |
  551193.5 |
  554120.0 |
  557046.4 |
  559972.9 |
  562899.3 |
  565825.8 |
  568752.3 |
  571678.7 |################################
  (4 below, 2 above range)

bitpack-footprint-packed-naive (n=40, range 2694844.9-2775847.5 ns)
  2694844.9 |########################################
  2698895.0 |#####
  2702945.2 |########
  2706995.3 |
  2711045.4 |
  2715095.6 |
  2719145.7 |
  2723195.8 |
  2727245.9 |####################
  2731296.1 |
  2735346.2 |##
  2739396.3 |#####
  2743446.5 |
  2747496.6 |
  2751546.7 |
  2755596.8 |
  2759647.0 |##
  2763697.1 |
  2767747.2 |
  2771797.4 |
  (4 below, 6 above range)

```
