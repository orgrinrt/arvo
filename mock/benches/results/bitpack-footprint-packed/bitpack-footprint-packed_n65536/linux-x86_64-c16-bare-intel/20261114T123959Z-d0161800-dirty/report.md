# Layout::Bitpacked footprint: plan-driven sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-packed**

## Highlights

Baseline for all deltas below: **bitpack-footprint-packed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed dominates: 427% faster than the next best (bitpack-footprint-packed-naive)

bitpack-footprint-packed (9.02 us) leads bitpack-footprint-packed-naive (47.51 us) by 427%, a clear separation rather than a photo finish. CV 10.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-footprint-packed is fastest but the noisiest (CV 10.0%)

bitpack-footprint-packed wins on median (9.02 us) yet has the highest variance (CV 10.0%), while bitpack-footprint-packed-naive is the steadiest (CV 10.0%, 47.51 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (bitpack-footprint-packed)

The baseline bitpack-footprint-packed is the fastest (9.02 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 5.3x the fastest

Fastest bitpack-footprint-packed (9.02 us) to slowest bitpack-footprint-packed-naive (47.51 us): 5.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (bitpack-footprint-packed) is the fastest** at 9015.2 ns median
- 1 variant significantly slower than baseline
- Spread: 5.27x (fastest 9015.2 ns, slowest 47505.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 9094ns | 9114ns | 8178ns | 9094ns | 10011ns | base |
| bitpack-footprint-packed-naive | 47581ns | 47584ns | 42777ns | 47583ns | 52380ns | +423.19% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-packed | 9014ns | 8109ns | 9917ns | base | 7.270 |
| bitpack-footprint-packed-naive | 47498ns | 42707ns | 52290ns | +426.93% | 1.380 |

## Performance model

- Peak throughput: **8.082 Gops/s** (bitpack-footprint-packed; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-packed | 7.269 | 89.9% |
| bitpack-footprint-packed-naive | 1.380 | 17.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-packed | 9094ns | 9094ns | base |
| bitpack-footprint-packed-naive | 47581ns | 47581ns | +423.19% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 9015ns | base | --- | [8114, 9916] | --- | --- | --- | --- |
| bitpack-footprint-packed-naive | 47506ns | +38490.7ns (+427.0%) | [+32891, +44101]ns | [42808, 52212] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|---|
| 1 | 1 | 8113ns | +543.6% |
| 1 | 2 | 9917ns | +331.9% |
| 1 | 3 | 9916ns | +330.7% |
| 1 | 4 | 8113ns | +544.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-packed | -0.250 | moderate- |
| bitpack-footprint-packed-naive | -0.250 | moderate- |

**Consistency summary:**

- **bitpack-footprint-packed-naive**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-packed | 21.5ns | 9014.1ns | 0.2% |  |
| bitpack-footprint-packed-naive | 22.2ns | 47498.1ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-packed (n=40, range 8109.1-9917.0 ns)
   8109.1 |########################################
   8199.5 |
   8289.9 |
   8380.3 |
   8470.6 |
   8561.0 |
   8651.4 |
   8741.8 |
   8832.2 |
   8922.6 |
   9013.0 |
   9103.4 |
   9193.8 |
   9284.2 |
   9374.6 |
   9465.0 |
   9555.4 |
   9645.8 |
   9736.2 |
   9826.6 |#####################################
  (3 below, 4 above range)

bitpack-footprint-packed-naive (n=40, range 42706.7-52289.8 ns)
  42706.7 |########################################
  43185.9 |
  43665.0 |
  44144.2 |
  44623.3 |
  45102.5 |
  45581.6 |
  46060.8 |
  46539.9 |
  47019.1 |
  47498.3 |
  47977.4 |
  48456.6 |
  48935.7 |
  49414.9 |
  49894.0 |
  50373.2 |
  50852.3 |
  51331.5 |
  51810.6 |########################################
  (3 below, 3 above range)

```
