# Layout::Bitpacked footprint: plan-driven sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-packed**

## Highlights

Baseline for all deltas below: **bitpack-footprint-packed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed dominates: 357% faster than the next best (bitpack-footprint-packed-naive)

bitpack-footprint-packed (4.76 ms) leads bitpack-footprint-packed-naive (21.80 ms) by 357%, a clear separation rather than a photo finish. CV 4.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-footprint-packed-naive shows alternating (throttle bounce) (autocorr -0.52)

bitpack-footprint-packed-naive's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (bitpack-footprint-packed)

The baseline bitpack-footprint-packed is the fastest (4.76 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.6x the fastest

Fastest bitpack-footprint-packed (4.76 ms) to slowest bitpack-footprint-packed-naive (21.80 ms): 4.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (bitpack-footprint-packed) is the fastest** at 4764385.8 ns median
- 1 variant significantly slower than baseline
- Spread: 4.57x (fastest 4764385.8 ns, slowest 21797055.7 ns)

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
| bitpack-footprint-packed | 4825298ns | 4765386ns | 4755699ns | 4766746ns | 5070550ns | base |
| bitpack-footprint-packed-naive | 21828815ns | 21798153ns | 21787829ns | 21806189ns | 21937680ns | +352.38% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-packed | 4824345ns | 4754751ns | 5069612ns | base | 6.955 |
| bitpack-footprint-packed-naive | 21827732ns | 21786720ns | 21936628ns | +352.45% | 1.537 |

## Performance model

- Peak throughput: **7.057 Gops/s** (bitpack-footprint-packed; best 20% batches)
- Ops per call: 33554432

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-packed | 7.043 | 99.8% |
| bitpack-footprint-packed-naive | 1.539 | 21.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-packed | 4825298ns | 4825298ns | base |
| bitpack-footprint-packed-naive | 21828815ns | 21828815ns | +352.38% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 4764386ns | base | --- | [4762152, 4769586] | --- | --- | --- | --- |
| bitpack-footprint-packed-naive | 21797056ns | +17032181.1ns (+357.5%) | [+17028228, +17051801]ns | [21792958, 21805069] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|---|
| 1 | 1 | 4766537ns | +357.2% |
| 1 | 2 | 4768876ns | +358.1% |
| 1 | 3 | 4761198ns | +357.7% |
| 1 | 4 | 4763625ns | +357.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-packed | -0.106 | ok |
| bitpack-footprint-packed-naive | -0.519 | HIGH- (thermal bounce) |

**Consistency summary:**

- **bitpack-footprint-packed-naive**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-packed | 111.9ns | 4824345.3ns | 0.0% |  |
| bitpack-footprint-packed-naive | 178.3ns | 21827731.7ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-packed (n=40, range 4754750.5-5069611.7 ns)
  4754750.5 |########################################
  4770493.6 |############
  4786236.6 |
  4801979.7 |
  4817722.8 |
  4833465.8 |
  4849208.9 |
  4864951.9 |
  4880695.0 |
  4896438.1 |
  4912181.1 |
  4927924.2 |
  4943667.2 |#
  4959410.3 |
  4975153.4 |
  4990896.4 |
  5006639.5 |
  5022382.5 |
  5038125.6 |
  5053868.7 |
  (2 below, 3 above range)

bitpack-footprint-packed-naive (n=40, range 21786719.7-21936627.9 ns)
  21786719.7 |########################################
  21794215.1 |###########################
  21801710.5 |######
  21809205.9 |
  21816701.3 |
  21824196.7 |
  21831692.1 |######
  21839187.6 |##################
  21846683.0 |#########
  21854178.4 |
  21861673.8 |
  21869169.2 |
  21876664.6 |
  21884160.0 |
  21891655.4 |
  21899150.9 |
  21906646.3 |
  21914141.7 |
  21921637.1 |
  21929132.5 |
  (3 below, 2 above range)

```
