# Layout::Bitpacked footprint: plan-driven sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-packed**

## Highlights

Baseline for all deltas below: **bitpack-footprint-packed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed dominates: 403% faster than the next best (bitpack-footprint-packed-naive)

bitpack-footprint-packed (132.65 us) leads bitpack-footprint-packed-naive (667.26 us) by 403%, a clear separation rather than a photo finish. CV 1.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (bitpack-footprint-packed)

The baseline bitpack-footprint-packed is the fastest (132.65 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 5.0x the fastest

Fastest bitpack-footprint-packed (132.65 us) to slowest bitpack-footprint-packed-naive (667.26 us): 5.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (bitpack-footprint-packed) is the fastest** at 132654.0 ns median
- 1 variant significantly slower than baseline
- Spread: 5.03x (fastest 132654.0 ns, slowest 667257.4 ns)

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
| bitpack-footprint-packed | 131431ns | 132833ns | 129114ns | 131266ns | 134243ns | base |
| bitpack-footprint-packed-naive | 673238ns | 667427ns | 667129ns | 670491ns | 687587ns | +412.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-packed | 131261ns | 128946ns | 134075ns | base | 7.988 |
| bitpack-footprint-packed-naive | 673060ns | 666971ns | 687417ns | +412.76% | 1.558 |

## Performance model

- Peak throughput: **8.132 Gops/s** (bitpack-footprint-packed; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-packed | 7.905 | 97.2% |
| bitpack-footprint-packed-naive | 1.571 | 19.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-packed | 131431ns | 131431ns | base |
| bitpack-footprint-packed-naive | 673238ns | 673238ns | +412.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 132654ns | base | --- | [129060, 132704] | --- | --- | --- | --- |
| bitpack-footprint-packed-naive | 667257ns | +538120.9ns (+405.7%) | [+534587, +540893]ns | [667165, 670538] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|---|
| 1 | 1 | 129053ns | +417.0% |
| 1 | 2 | 128949ns | +432.8% |
| 1 | 3 | 132694ns | +402.8% |
| 1 | 4 | 132760ns | +402.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-packed | 0.247 | moderate+ |
| bitpack-footprint-packed-naive | -0.406 | moderate- |

**Consistency summary:**

- **bitpack-footprint-packed-naive**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-packed | 27.4ns | 131261.3ns | 0.0% |  |
| bitpack-footprint-packed-naive | 31.6ns | 673059.5ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-packed (n=40, range 128945.9-134074.6 ns)
  128945.9 |##################################
  129202.3 |########
  129458.8 |
  129715.2 |##
  129971.6 |
  130228.1 |
  130484.5 |
  130740.9 |
  130997.4 |
  131253.8 |
  131510.2 |
  131766.7 |
  132023.1 |
  132279.5 |
  132536.0 |########################################
  132792.4 |###########
  133048.8 |
  133305.3 |##
  133561.7 |
  133818.1 |##
  (3 below, 1 above range)

bitpack-footprint-packed-naive (n=40, range 666971.3-687417.3 ns)
  666971.3 |########################################
  667993.6 |##
  669015.9 |
  670038.2 |
  671060.5 |
  672082.8 |####
  673105.1 |
  674127.4 |##
  675149.7 |
  676172.0 |
  677194.3 |
  678216.6 |
  679238.9 |
  680261.2 |##
  681283.5 |
  682305.8 |
  683328.1 |
  684350.4 |
  685372.7 |
  686395.0 |############
  (5 below, 4 above range)

```
