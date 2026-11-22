# Layout::Bitpacked footprint: plan-driven sum swept past L1 and L2

2 variants, 40 samples per variant.
Baseline: **bitpack-footprint-packed**

## Highlights

Baseline for all deltas below: **bitpack-footprint-packed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed dominates: 376% faster than the next best (bitpack-footprint-packed-naive)

bitpack-footprint-packed (955.20 us) leads bitpack-footprint-packed-naive (4.55 ms) by 376%, a clear separation rather than a photo finish. CV 4.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (bitpack-footprint-packed)

The baseline bitpack-footprint-packed is the fastest (955.20 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.8x the fastest

Fastest bitpack-footprint-packed (955.20 us) to slowest bitpack-footprint-packed-naive (4.55 ms): 4.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (bitpack-footprint-packed) is the fastest** at 955195.8 ns median
- 1 variant significantly slower than baseline
- Spread: 4.76x (fastest 955195.8 ns, slowest 4547846.2 ns)

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
| bitpack-footprint-packed | 977232ns | 955573ns | 930788ns | 973155ns | 1035909ns | base |
| bitpack-footprint-packed-naive | 4541156ns | 4548388ns | 4506842ns | 4542068ns | 4572732ns | +364.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-packed | 976887ns | 930406ns | 1035598ns | base | 7.166 |
| bitpack-footprint-packed-naive | 4540622ns | 4506319ns | 4572208ns | +364.81% | 1.542 |

## Performance model

- Peak throughput: **7.524 Gops/s** (bitpack-footprint-packed; best 20% batches)
- Ops per call: 7000000

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-packed | 7.328 | 97.4% |
| bitpack-footprint-packed-naive | 1.539 | 20.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-packed | 977232ns | 977232ns | base |
| bitpack-footprint-packed-naive | 4541156ns | 4541156ns | +364.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-packed | 955196ns | base | --- | [952434, 997134] | --- | --- | --- | --- |
| bitpack-footprint-packed-naive | 4547846ns | +3570567.2ns (+373.8%) | [+3555675, +3579639]ns | [4523248, 4558415] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|---|
| 1 | 1 | 996837ns | +357.1% |
| 1 | 2 | 1034214ns | +335.8% |
| 1 | 3 | 953109ns | +374.6% |
| 1 | 4 | 948088ns | +382.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-packed | 0.045 | ok |
| bitpack-footprint-packed-naive | -0.212 | moderate- |

**Consistency summary:**

- **bitpack-footprint-packed-naive**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-packed | 61.9ns | 976887.3ns | 0.0% |  |
| bitpack-footprint-packed-naive | 198.2ns | 4540621.5ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-packed (n=40, range 930406.1-1035597.8 ns)
  930406.1 |####
  935665.7 |
  940925.3 |
  946184.9 |############################
  951444.4 |########################################
  956704.0 |####
  961963.6 |
  967223.2 |
  972482.8 |########
  977742.4 |
  983001.9 |
  988261.5 |
  993521.1 |####################
  998780.7 |####
  1004040.3 |
  1009299.9 |
  1014559.5 |
  1019819.0 |
  1025078.6 |
  1030338.2 |################################
  (3 below, 2 above range)

bitpack-footprint-packed-naive (n=40, range 4506318.8-4572208.2 ns)
  4506318.8 |#############
  4509613.3 |#############
  4512907.8 |
  4516202.2 |
  4519496.7 |##########################
  4522791.2 |##########################
  4526085.6 |#############
  4529380.1 |
  4532674.6 |
  4535969.1 |
  4539263.5 |
  4542558.0 |
  4545852.5 |######
  4549146.9 |##########################
  4552441.4 |######
  4555735.9 |#############
  4559030.3 |#############
  4562324.8 |
  4565619.3 |########################################
  4568913.7 |######
  (5 below, 4 above range)

```
