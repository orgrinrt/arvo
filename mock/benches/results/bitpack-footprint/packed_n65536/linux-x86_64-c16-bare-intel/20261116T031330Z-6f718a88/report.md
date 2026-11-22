# Layout::Bitpacked footprint: plan-driven sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-packed**

## Highlights

Baseline for all deltas below: **bitpack-footprint-packed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed dominates: 487% faster than the next best (bitpack-footprint-packed-naive)

bitpack-footprint-packed (8.11 us) leads bitpack-footprint-packed-naive (47.61 us) by 487%, a clear separation rather than a photo finish. CV 9.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (bitpack-footprint-packed)

The baseline bitpack-footprint-packed is the fastest (8.11 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 5.9x the fastest

Fastest bitpack-footprint-packed (8.11 us) to slowest bitpack-footprint-packed-naive (47.61 us): 5.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (bitpack-footprint-packed) is the fastest** at 8113.4 ns median
- 1 variant significantly slower than baseline
- Spread: 5.87x (fastest 8113.4 ns, slowest 47606.4 ns)

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
| bitpack-footprint-packed | 8628ns | 8181ns | 8129ns | 8333ns | 10011ns | base |
| bitpack-footprint-packed-naive | 47585ns | 47716ns | 42777ns | 47595ns | 52364ns | +451.53% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-packed | 8552ns | 8060ns | 9921ns | base | 7.664 |
| bitpack-footprint-packed-naive | 47501ns | 42709ns | 52263ns | +455.47% | 1.380 |

## Performance model

- Peak throughput: **8.131 Gops/s** (bitpack-footprint-packed; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-packed | 8.078 | 99.3% |
| bitpack-footprint-packed-naive | 1.377 | 16.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-packed | 8628ns | 8628ns | base |
| bitpack-footprint-packed-naive | 47585ns | 47585ns | +451.53% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 8113ns | base | --- | [8108, 8114] | --- | --- | --- | --- |
| bitpack-footprint-packed-naive | 47606ns | +39417.2ns (+485.8%) | [+34600, +44124]ns | [42797, 52216] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|---|
| 1 | 1 | 8114ns | +426.6% |
| 1 | 2 | 9917ns | +331.1% |
| 1 | 3 | 8113ns | +543.8% |
| 1 | 4 | 8063ns | +547.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-packed | -0.395 | moderate- |
| bitpack-footprint-packed-naive | 0.250 | moderate+ |

**Consistency summary:**

- **bitpack-footprint-packed-naive**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-packed | 20.4ns | 8551.6ns | 0.2% |  |
| bitpack-footprint-packed-naive | 22.4ns | 47501.5ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-packed (n=40, range 8060.1-9921.5 ns)
   8060.1 |########################################
   8153.2 |
   8246.3 |
   8339.3 |
   8432.4 |
   8525.5 |
   8618.5 |
   8711.6 |
   8804.7 |
   8897.7 |
   8990.8 |
   9083.9 |
   9176.9 |
   9270.0 |
   9363.1 |
   9456.1 |
   9549.2 |
   9642.3 |
   9735.3 |
   9828.4 |###########
  (2 below, 2 above range)

bitpack-footprint-packed-naive (n=40, range 42708.5-52263.2 ns)
  42708.5 |########################################
  43186.3 |
  43664.0 |
  44141.7 |
  44619.5 |
  45097.2 |
  45574.9 |
  46052.7 |
  46530.4 |
  47008.1 |
  47485.9 |
  47963.6 |
  48441.3 |
  48919.1 |
  49396.8 |
  49874.5 |
  50352.3 |
  50830.0 |
  51307.7 |
  51785.5 |########################################
  (4 below, 4 above range)

```
