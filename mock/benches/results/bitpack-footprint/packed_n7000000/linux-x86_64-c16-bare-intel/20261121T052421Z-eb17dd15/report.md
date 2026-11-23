# Layout::Bitpacked footprint: plan-driven sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-packed**

## Highlights

Baseline for all deltas below: **bitpack-footprint-packed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed dominates: 385% faster than the next best (bitpack-footprint-packed-naive)

bitpack-footprint-packed (934.99 us) leads bitpack-footprint-packed-naive (4.54 ms) by 385%, a clear separation rather than a photo finish. CV 1.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (bitpack-footprint-packed)

The baseline bitpack-footprint-packed is the fastest (934.99 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.9x the fastest

Fastest bitpack-footprint-packed (934.99 us) to slowest bitpack-footprint-packed-naive (4.54 ms): 4.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (bitpack-footprint-packed) is the fastest** at 934985.1 ns median
- 1 variant significantly slower than baseline
- Spread: 4.85x (fastest 934985.1 ns, slowest 4538003.5 ns)

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
| bitpack-footprint-packed | 930267ns | 935488ns | 904830ns | 933151ns | 947055ns | base |
| bitpack-footprint-packed-naive | 4555351ns | 4538464ns | 4525824ns | 4544713ns | 4616791ns | +389.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-packed | 929738ns | 904366ns | 946334ns | base | 7.529 |
| bitpack-footprint-packed-naive | 4554851ns | 4525391ns | 4616287ns | +389.91% | 1.537 |

## Performance model

- Peak throughput: **7.740 Gops/s** (bitpack-footprint-packed; best 20% batches)
- Ops per call: 7000000

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-packed | 7.487 | 96.7% |
| bitpack-footprint-packed-naive | 1.543 | 19.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-packed | 930267ns | 930267ns | base |
| bitpack-footprint-packed-naive | 4555351ns | 4555351ns | +389.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 934985ns | base | --- | [928003, 937909] | --- | --- | --- | --- |
| bitpack-footprint-packed-naive | 4538004ns | +3614052.3ns (+386.5%) | [+3598317, +3638132]ns | [4534387, 4543454] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|---|
| 1 | 1 | 904286ns | +402.3% |
| 1 | 2 | 945171ns | +379.9% |
| 1 | 3 | 928268ns | +387.7% |
| 1 | 4 | 936694ns | +392.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-packed | -0.441 | moderate- |
| bitpack-footprint-packed-naive | -0.187 | ok |

**Consistency summary:**

- **bitpack-footprint-packed-naive**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-packed | 93.2ns | 929738.1ns | 0.0% |  |
| bitpack-footprint-packed-naive | 150.1ns | 4554851.1ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-packed (n=40, range 904365.7-946334.5 ns)
  904365.7 |#############
  906464.1 |######
  908562.6 |
  910661.0 |######
  912759.5 |######
  914857.9 |
  916956.3 |
  919054.8 |
  921153.2 |
  923251.6 |
  925350.1 |####################
  927448.5 |##########################
  929547.0 |#############
  931645.4 |
  933743.8 |##########################
  935842.3 |####################
  937940.7 |#############
  940039.1 |######
  942137.6 |######
  944236.0 |########################################
  (5 below, 4 above range)

bitpack-footprint-packed-naive (n=40, range 4525390.5-4616286.9 ns)
  4525390.5 |########################################
  4529935.3 |############################
  4534480.1 |########################################
  4539025.0 |############################
  4543569.8 |###########
  4548114.6 |#####
  4552659.4 |
  4557204.2 |
  4561749.1 |
  4566293.9 |
  4570838.7 |#####
  4575383.5 |
  4579928.3 |
  4584473.1 |
  4589018.0 |
  4593562.8 |#####
  4598107.6 |#####
  4602652.4 |###########
  4607197.2 |#####
  4611742.0 |#####
  (2 below, 4 above range)

```
