# Layout::Bitpacked footprint: plan-driven sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-packed**

## Highlights

Baseline for all deltas below: **bitpack-footprint-packed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed dominates: 380% faster than the next best (bitpack-footprint-packed-naive)

bitpack-footprint-packed (8.91 us) leads bitpack-footprint-packed-naive (42.74 us) by 380%, a clear separation rather than a photo finish. CV 10.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (bitpack-footprint-packed)

The baseline bitpack-footprint-packed is the fastest (8.91 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.8x the fastest

Fastest bitpack-footprint-packed (8.91 us) to slowest bitpack-footprint-packed-naive (42.74 us): 4.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (bitpack-footprint-packed) is the fastest** at 8910.1 ns median
- 1 variant significantly slower than baseline
- Spread: 4.80x (fastest 8910.1 ns, slowest 42739.6 ns)

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
| Counter frequency | 2112746100 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 8995ns | 9008ns | 8054ns | 8991ns | 9951ns | base |
| bitpack-footprint-packed-naive | 44805ns | 42809ns | 41684ns | 43313ns | 52403ns | +398.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-packed | 8917ns | 7988ns | 9863ns | base | 7.350 |
| bitpack-footprint-packed-naive | 44727ns | 41615ns | 52311ns | +401.62% | 1.465 |

## Performance model

- Peak throughput: **8.205 Gops/s** (bitpack-footprint-packed; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-packed | 7.355 | 89.6% |
| bitpack-footprint-packed-naive | 1.533 | 18.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-packed | 8995ns | 8995ns | base |
| bitpack-footprint-packed-naive | 44805ns | 44805ns | +398.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 8910ns | base | --- | [8053, 9765] | --- | --- | --- | --- |
| bitpack-footprint-packed-naive | 42740ns | +33277.4ns (+373.5%) | [+32914, +33573]ns | [41914, 42752] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|---|
| 1 | 1 | 9765ns | +334.5% |
| 1 | 2 | 9846ns | +334.1% |
| 1 | 3 | 8054ns | +416.9% |
| 1 | 4 | 7991ns | +554.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-packed | 0.247 | moderate+ |
| bitpack-footprint-packed-naive | -0.165 | ok |

**Consistency summary:**

- **bitpack-footprint-packed-naive**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-packed | 21.2ns | 8916.6ns | 0.2% |  |
| bitpack-footprint-packed-naive | 20.3ns | 44727.2ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-packed (n=40, range 7987.5-9862.8 ns)
   7987.5 |########################################
   8081.3 |
   8175.0 |
   8268.8 |
   8362.6 |
   8456.3 |
   8550.1 |
   8643.9 |
   8737.6 |
   8831.4 |
   8925.2 |
   9018.9 |
   9112.7 |
   9206.4 |
   9300.2 |
   9394.0 |
   9487.7 |
   9581.5 |
   9675.3 |#######################
   9769.0 |##################
  (3 below, 2 above range)

bitpack-footprint-packed-naive (n=40, range 41615.5-52311.4 ns)
  41615.5 |#############################
  42150.3 |
  42685.1 |########################################
  43219.9 |
  43754.7 |
  44289.5 |
  44824.3 |
  45359.1 |
  45893.9 |
  46428.7 |
  46963.4 |
  47498.2 |
  48033.0 |
  48567.8 |
  49102.6 |
  49637.4 |
  50172.2 |
  50707.0 |
  51241.8 |
  51776.6 |########################
  (4 below, 1 above range)

```
