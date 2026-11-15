# Layout::Bitpacked footprint: plan-driven sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-packed**

## Highlights

Baseline for all deltas below: **bitpack-footprint-packed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed dominates: 359% faster than the next best (bitpack-footprint-packed-naive)

bitpack-footprint-packed (4.76 ms) leads bitpack-footprint-packed-naive (21.84 ms) by 359%, a clear separation rather than a photo finish. CV 0.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-footprint-packed-naive shows alternating (throttle bounce) (autocorr -0.79)

bitpack-footprint-packed-naive's per-pass series has lag-1 autocorrelation -0.79, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (bitpack-footprint-packed)

The baseline bitpack-footprint-packed is the fastest (4.76 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.6x the fastest

Fastest bitpack-footprint-packed (4.76 ms) to slowest bitpack-footprint-packed-naive (21.84 ms): 4.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (bitpack-footprint-packed) is the fastest** at 4761732.6 ns median
- 1 variant significantly slower than baseline
- Spread: 4.59x (fastest 4761732.6 ns, slowest 21841391.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 4764465ns | 4762573ns | 4750543ns | 4763618ns | 4780927ns | base |
| bitpack-footprint-packed-naive | 21828841ns | 21842361ns | 21777961ns | 21826159ns | 21887766ns | +358.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-packed | 4763604ns | 4749699ns | 4780026ns | base | 7.044 |
| bitpack-footprint-packed-naive | 21827842ns | 21776921ns | 21886748ns | +358.22% | 1.537 |

## Performance model

- Peak throughput: **7.065 Gops/s** (bitpack-footprint-packed; best 20% batches)
- Ops per call: 33554432

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-packed | 7.047 | 99.7% |
| bitpack-footprint-packed-naive | 1.536 | 21.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-packed | 4764465ns | 4764465ns | base |
| bitpack-footprint-packed-naive | 21828841ns | 21828841ns | +358.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 4761733ns | base | --- | [4756748, 4769396] | --- | --- | --- | --- |
| bitpack-footprint-packed-naive | 21841391ns | +17062928.1ns (+358.3%) | [+17042639, +17075974]ns | [21797535, 21846470] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|---|
| 1 | 1 | 4774718ns | +357.6% |
| 1 | 2 | 4752548ns | +358.2% |
| 1 | 3 | 4767534ns | +358.3% |
| 1 | 4 | 4756256ns | +358.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-packed | -0.646 | HIGH- (thermal bounce) |
| bitpack-footprint-packed-naive | -0.788 | HIGH- (thermal bounce) |

**Consistency summary:**

- **bitpack-footprint-packed-naive**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-packed | 106.5ns | 4763604.0ns | 0.0% |  |
| bitpack-footprint-packed-naive | 211.8ns | 21827842.1ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-packed (n=40, range 4749698.8-4780026.2 ns)
  4749698.8 |########################################
  4751215.2 |#############
  4752731.5 |##########################
  4754247.9 |##########################
  4755764.3 |########################################
  4757280.7 |#############
  4758797.0 |##########################
  4760313.4 |##########################
  4761829.8 |##########################
  4763346.1 |#############
  4764862.5 |
  4766378.9 |#############
  4767895.2 |##########################
  4769411.6 |########################################
  4770928.0 |#############
  4772444.3 |##########################
  4773960.7 |#############
  4775477.1 |#############
  4776993.4 |##########################
  4778509.8 |#############
  (4 below, 3 above range)

bitpack-footprint-packed-naive (n=40, range 21776921.4-21886748.2 ns)
  21776921.4 |######################
  21782412.7 |####
  21787904.0 |####
  21793395.4 |##########################
  21798886.7 |####
  21804378.1 |####
  21809869.4 |
  21815360.8 |
  21820852.1 |
  21826343.4 |
  21831834.8 |
  21837326.1 |#############
  21842817.5 |########################################
  21848308.8 |##########################
  21853800.1 |####
  21859291.5 |
  21864782.8 |
  21870274.2 |
  21875765.5 |
  21881256.9 |
  (3 below, 3 above range)

```
