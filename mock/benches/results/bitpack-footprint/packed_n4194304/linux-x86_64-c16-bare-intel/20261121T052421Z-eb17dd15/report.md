# Layout::Bitpacked footprint: plan-driven sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-packed**

## Highlights

Baseline for all deltas below: **bitpack-footprint-packed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed dominates: 399% faster than the next best (bitpack-footprint-packed-naive)

bitpack-footprint-packed (541.76 us) leads bitpack-footprint-packed-naive (2.70 ms) by 399%, a clear separation rather than a photo finish. CV 4.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (bitpack-footprint-packed)

The baseline bitpack-footprint-packed is the fastest (541.76 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 5.0x the fastest

Fastest bitpack-footprint-packed (541.76 us) to slowest bitpack-footprint-packed-naive (2.70 ms): 5.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (bitpack-footprint-packed) is the fastest** at 541760.9 ns median
- 1 variant significantly slower than baseline
- Spread: 4.99x (fastest 541760.9 ns, slowest 2702369.1 ns)

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
| bitpack-footprint-packed | 550796ns | 541985ns | 527930ns | 545147ns | 590612ns | base |
| bitpack-footprint-packed-naive | 2715050ns | 2702669ns | 2699594ns | 2708542ns | 2750029ns | +392.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-packed | 550543ns | 527770ns | 590213ns | base | 7.618 |
| bitpack-footprint-packed-naive | 2714733ns | 2699370ns | 2749607ns | +393.10% | 1.545 |

## Performance model

- Peak throughput: **7.947 Gops/s** (bitpack-footprint-packed; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-packed | 7.742 | 97.4% |
| bitpack-footprint-packed-naive | 1.552 | 19.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-packed | 550796ns | 550796ns | base |
| bitpack-footprint-packed-naive | 2715050ns | 2715050ns | +392.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 541761ns | base | --- | [528979, 555760] | --- | --- | --- | --- |
| bitpack-footprint-packed-naive | 2702369ns | +2171351.6ns (+400.8%) | [+2145444, +2174533]ns | [2701294, 2713388] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|---|
| 1 | 1 | 587688ns | +359.6% |
| 1 | 2 | 527799ns | +412.0% |
| 1 | 3 | 555460ns | +386.5% |
| 1 | 4 | 528691ns | +416.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-packed | -0.449 | moderate- |
| bitpack-footprint-packed-naive | -0.077 | ok |

**Consistency summary:**

- **bitpack-footprint-packed-naive**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-packed | 32.3ns | 550542.7ns | 0.0% |  |
| bitpack-footprint-packed-naive | 64.9ns | 2714733.5ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-packed (n=40, range 527770.5-590213.1 ns)
  527770.5 |########################################
  530892.6 |
  534014.7 |
  537136.9 |
  540259.0 |
  543381.1 |
  546503.3 |
  549625.4 |
  552747.5 |##############
  555869.6 |#######
  558991.8 |##
  562113.9 |
  565236.0 |
  568358.2 |
  571480.3 |
  574602.4 |
  577724.5 |
  580846.7 |
  583968.8 |####
  587090.9 |################
  (3 below, 1 above range)

bitpack-footprint-packed-naive (n=40, range 2699370.0-2749607.5 ns)
  2699370.0 |########################################
  2701881.8 |###############
  2704393.7 |#########
  2706905.6 |###
  2709417.5 |
  2711929.3 |######
  2714441.2 |###
  2716953.1 |
  2719465.0 |
  2721976.8 |
  2724488.7 |###
  2727000.6 |######
  2729512.5 |#########
  2732024.3 |#########
  2734536.2 |
  2737048.1 |###
  2739560.0 |
  2742071.8 |
  2744583.7 |
  2747095.6 |
  (3 below, 2 above range)

```
