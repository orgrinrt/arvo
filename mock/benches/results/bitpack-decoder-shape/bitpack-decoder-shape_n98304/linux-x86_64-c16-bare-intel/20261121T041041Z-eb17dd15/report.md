# Layout::Bitpacked decoder shape: index-driven vs plan-driven, across the L1 boundary

4 variants, 40 samples per variant.
Baseline: **bitpack-plan-naive**

## Highlights

Baseline for all deltas below: **bitpack-plan-naive**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-plan-naive) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-plan-naive has the worst median (51.97 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-plan-simd at 12.07 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-plan-simd beats baseline by 77% (significant)

bitpack-plan-simd is -39.95 us (77%) faster than baseline bitpack-plan-naive, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-plan-naive is an outlier: 4.3x slower than the field

bitpack-plan-naive (51.97 us) is 4.3x the fastest (12.07 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (bitpack-plan-simd, bitpack-plan-windowed) are a dead heat (<1%)

bitpack-plan-simd (12.07 us) and bitpack-plan-windowed (12.07 us) differ by 0.00%, inside the noise, even though the wider field spreads 330.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {bitpack-plan-simd, bitpack-plan-windowed, bitpack-plan-native} vs {bitpack-plan-naive} (248% apart)

The field splits into a fast tier {bitpack-plan-simd, bitpack-plan-windowed, bitpack-plan-native} and a slow tier {bitpack-plan-naive} with a 248% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.3x the fastest

Fastest bitpack-plan-simd (12.07 us) to slowest bitpack-plan-naive (51.97 us): 4.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-plan-simd** at 12072.2 ns median (-76.8% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 4.31x (fastest 12072.2 ns, slowest 51971.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Layout::Bitpacked decoder shape: index-driven vs plan-driven, across the L1 boundary |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112800587 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-plan-naive | 52031ns | 52041ns | 51788ns | 52055ns | 52203ns | base |
| bitpack-plan-native | 15027ns | 15022ns | 15014ns | 15024ns | 15049ns | -71.12% |
| bitpack-plan-simd | 12119ns | 12139ns | 12036ns | 12133ns | 12160ns | -76.71% |
| bitpack-plan-windowed | 12144ns | 12140ns | 12040ns | 12139ns | 12265ns | -76.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-plan-naive | 51958ns | 51714ns | 52129ns | base | 1.892 |
| bitpack-plan-native | 14956ns | 14946ns | 14965ns | -71.21% | 6.573 |
| bitpack-plan-simd | 12048ns | 11967ns | 12081ns | -76.81% | 8.159 |
| bitpack-plan-windowed | 12073ns | 11973ns | 12187ns | -76.76% | 8.142 |

## Performance model

- Peak throughput: **8.215 Gops/s** (bitpack-plan-simd; best 20% batches)
- Ops per call: 98304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-plan-naive | 1.891 | 23.0% |
| bitpack-plan-native | 6.574 | 80.0% |
| bitpack-plan-simd | 8.143 | 99.1% |
| bitpack-plan-windowed | 8.143 | 99.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-plan-naive | 52031ns | 52031ns | base |
| bitpack-plan-native | 15027ns | 15027ns | -71.12% |
| bitpack-plan-simd | 12119ns | 12119ns | -76.71% |
| bitpack-plan-windowed | 12144ns | 12144ns | -76.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-plan-naive | 51972ns | base | --- | [51924, 52048] | --- | --- | --- | --- |
| bitpack-plan-native | 14954ns | -37023.5ns (-71.2%) | [-37096, -36962]ns | [14953, 14962] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-plan-simd | 12072ns | -39948.2ns (-76.9%) | [-39998, -39848]ns | [12070, 12073] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-plan-windowed | 12072ns | -39913.6ns (-76.8%) | [-39976, -39862]ns | [12071, 12074] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-plan-naive | bitpack-plan-native | bitpack-plan-simd | bitpack-plan-windowed |
|---|---|---|---|---|---|
| 1 | 1 | 52008ns | -71.3% | -76.8% | -76.8% |
| 1 | 2 | 51878ns | -71.2% | -76.7% | -76.9% |
| 1 | 3 | 51985ns | -71.2% | -76.8% | -76.8% |
| 1 | 4 | 52039ns | -71.3% | -77.0% | -76.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-plan-naive | -0.225 | moderate- |
| bitpack-plan-native | -0.298 | moderate- |
| bitpack-plan-simd | -0.078 | ok |
| bitpack-plan-windowed | -0.421 | moderate- |

**Consistency summary:**

- **bitpack-plan-native**: won 4/4, lost 0/4
- **bitpack-plan-simd**: won 4/4, lost 0/4
- **bitpack-plan-windowed**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-plan-naive | 20.8ns | 51957.5ns | 0.0% |  |
| bitpack-plan-native | 20.4ns | 14956.2ns | 0.1% |  |
| bitpack-plan-simd | 20.5ns | 12048.1ns | 0.2% |  |
| bitpack-plan-windowed | 20.8ns | 12073.1ns | 0.2% |  |

## Distribution (algo ns)

```
bitpack-plan-naive (n=40, range 51714.4-52128.8 ns)
  51714.4 |
  51735.1 |
  51755.8 |#############
  51776.5 |######
  51797.2 |######
  51818.0 |
  51838.7 |######
  51859.4 |
  51880.1 |
  51900.9 |##########################
  51921.6 |#############
  51942.3 |#############
  51963.0 |####################
  51983.7 |
  52004.5 |#############
  52025.2 |####################
  52045.9 |######
  52066.6 |########################################
  52087.4 |####################
  52108.1 |######
  (5 below, 3 above range)

bitpack-plan-native (n=40, range 14946.1-14965.0 ns)
  14946.1 |
  14947.0 |
  14948.0 |
  14948.9 |
  14949.9 |
  14950.8 |
  14951.8 |#########################
  14952.7 |########################################
  14953.7 |###############
  14954.6 |
  14955.6 |##########
  14956.5 |
  14957.5 |
  14958.4 |
  14959.4 |
  14960.3 |
  14961.2 |
  14962.2 |########################################
  14963.1 |###############
  14964.1 |##########
  (6 below, 3 above range)

bitpack-plan-simd (n=40, range 11967.0-12080.6 ns)
  11967.0 |##
  11972.7 |######
  11978.4 |##
  11984.1 |####
  11989.7 |
  11995.4 |
  12001.1 |
  12006.8 |
  12012.5 |
  12018.1 |
  12023.8 |
  12029.5 |
  12035.2 |
  12040.8 |
  12046.5 |
  12052.2 |##
  12057.9 |##
  12063.5 |##
  12069.2 |########################################
  12074.9 |########
  (3 below, 5 above range)

bitpack-plan-windowed (n=40, range 11973.5-12186.5 ns)
  11973.5 |###########
  11984.1 |#
  11994.8 |
  12005.4 |
  12016.1 |
  12026.7 |
  12037.4 |
  12048.0 |
  12058.7 |#####
  12069.3 |########################################
  12080.0 |#########
  12090.7 |#
  12101.3 |
  12112.0 |
  12122.6 |
  12133.3 |
  12143.9 |
  12154.6 |
  12165.2 |
  12175.9 |
  (2 below, 1 above range)

```
