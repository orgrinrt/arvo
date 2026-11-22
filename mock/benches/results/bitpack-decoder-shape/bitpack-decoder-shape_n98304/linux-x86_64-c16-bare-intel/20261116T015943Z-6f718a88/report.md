# Layout::Bitpacked decoder shape: index-driven vs plan-driven, across the L1 boundary

4 variants, 40 samples per variant.
Baseline: **bitpack-plan-naive**

## Highlights

Baseline for all deltas below: **bitpack-plan-naive**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-plan-naive) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-plan-naive has the worst median (52.30 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-plan-simd at 12.07 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-plan-simd beats baseline by 77% (significant)

bitpack-plan-simd is -40.20 us (77%) faster than baseline bitpack-plan-naive, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-plan-naive is an outlier: 4.3x slower than the field

bitpack-plan-naive (52.30 us) is 4.3x the fastest (12.07 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (bitpack-plan-simd, bitpack-plan-windowed) are a dead heat (<1%)

bitpack-plan-simd (12.07 us) and bitpack-plan-windowed (12.11 us) differ by 0.37%, inside the noise, even though the wider field spreads 333.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {bitpack-plan-simd, bitpack-plan-windowed, bitpack-plan-native} vs {bitpack-plan-naive} (250% apart)

The field splits into a fast tier {bitpack-plan-simd, bitpack-plan-windowed, bitpack-plan-native} and a slow tier {bitpack-plan-naive} with a 250% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.3x the fastest

Fastest bitpack-plan-simd (12.07 us) to slowest bitpack-plan-naive (52.30 us): 4.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-plan-simd** at 12066.5 ns median (-76.9% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 4.33x (fastest 12066.5 ns, slowest 52303.0 ns)

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
| Counter frequency | 2112822771 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-plan-naive | 51992ns | 52375ns | 50378ns | 52370ns | 52475ns | base |
| bitpack-plan-native | 15042ns | 15039ns | 15022ns | 15039ns | 15069ns | -71.07% |
| bitpack-plan-simd | 12134ns | 12148ns | 12025ns | 12140ns | 12228ns | -76.66% |
| bitpack-plan-windowed | 12160ns | 12190ns | 12065ns | 12169ns | 12230ns | -76.61% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-plan-naive | 51920ns | 50309ns | 52404ns | base | 1.893 |
| bitpack-plan-native | 14961ns | 14946ns | 14975ns | -71.18% | 6.571 |
| bitpack-plan-simd | 12055ns | 11942ns | 12150ns | -76.78% | 8.155 |
| bitpack-plan-windowed | 12084ns | 11984ns | 12155ns | -76.73% | 8.135 |

## Performance model

- Peak throughput: **8.232 Gops/s** (bitpack-plan-simd; best 20% batches)
- Ops per call: 98304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-plan-naive | 1.880 | 22.8% |
| bitpack-plan-native | 6.571 | 79.8% |
| bitpack-plan-simd | 8.147 | 99.0% |
| bitpack-plan-windowed | 8.117 | 98.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-plan-naive | 51992ns | 51992ns | base |
| bitpack-plan-native | 15042ns | 15042ns | -71.07% |
| bitpack-plan-simd | 12134ns | 12134ns | -76.66% |
| bitpack-plan-windowed | 12160ns | 12160ns | -76.61% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-plan-naive | 52303ns | base | --- | [52269, 52317] | --- | --- | --- | --- |
| bitpack-plan-native | 14961ns | -37342.8ns (-71.4%) | [-37355, -37294]ns | [14957, 14967] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-plan-simd | 12066ns | -40203.9ns (-76.9%) | [-40233, -40178]ns | [12020, 12096] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-plan-windowed | 12111ns | -40189.7ns (-76.8%) | [-40249, -40177]ns | [12059, 12130] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-plan-naive | bitpack-plan-native | bitpack-plan-simd | bitpack-plan-windowed |
|---|---|---|---|---|---|
| 1 | 1 | 51555ns | -71.0% | -76.5% | -76.6% |
| 1 | 2 | 52313ns | -71.4% | -76.8% | -76.8% |
| 1 | 3 | 52272ns | -71.4% | -76.9% | -76.8% |
| 1 | 4 | 52342ns | -71.4% | -77.1% | -77.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-plan-naive | -0.107 | ok |
| bitpack-plan-native | -0.253 | moderate- |
| bitpack-plan-simd | 0.119 | ok |
| bitpack-plan-windowed | -0.232 | moderate- |

**Consistency summary:**

- **bitpack-plan-native**: won 4/4, lost 0/4
- **bitpack-plan-simd**: won 4/4, lost 0/4
- **bitpack-plan-windowed**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-plan-naive | 19.4ns | 51919.8ns | 0.0% |  |
| bitpack-plan-native | 19.3ns | 14960.9ns | 0.1% |  |
| bitpack-plan-simd | 19.3ns | 12054.7ns | 0.2% |  |
| bitpack-plan-windowed | 19.3ns | 12084.0ns | 0.2% |  |

## Distribution (algo ns)

```
bitpack-plan-naive (n=40, range 50308.9-52404.0 ns)
  50308.9 |
  50413.7 |##
  50518.4 |
  50623.2 |
  50727.9 |####
  50832.7 |
  50937.4 |
  51042.2 |
  51147.0 |
  51251.7 |
  51356.5 |
  51461.2 |
  51566.0 |
  51670.7 |
  51775.5 |
  51880.2 |
  51985.0 |
  52089.8 |
  52194.5 |########################
  52299.3 |########################################
  (4 below, 1 above range)

bitpack-plan-native (n=40, range 14946.0-14974.6 ns)
  14946.0 |
  14947.4 |
  14948.8 |################
  14950.3 |
  14951.7 |################
  14953.1 |########################
  14954.6 |
  14956.0 |################
  14957.4 |########################
  14958.9 |########
  14960.3 |########################################
  14961.7 |########
  14963.2 |
  14964.6 |################
  14966.0 |
  14967.5 |########################
  14968.9 |########################
  14970.3 |########################################
  14971.8 |########
  14973.2 |
  (5 below, 2 above range)

bitpack-plan-simd (n=40, range 11941.7-12149.8 ns)
  11941.7 |
  11952.1 |
  11962.5 |##############################
  11972.9 |
  11983.3 |
  11993.7 |########################################
  12004.1 |##########
  12014.5 |########################################
  12024.9 |##########
  12035.3 |
  12045.8 |##########
  12056.2 |####################
  12066.6 |##############################
  12077.0 |####################
  12087.4 |##########
  12097.8 |####################
  12108.2 |##########
  12118.6 |####################
  12129.0 |##########
  12139.4 |##############################
  (4 below, 5 above range)

bitpack-plan-windowed (n=40, range 11984.5-12155.0 ns)
  11984.5 |######
  11993.0 |#############
  12001.5 |#############
  12010.1 |#############
  12018.6 |######
  12027.1 |
  12035.6 |######
  12044.1 |
  12052.7 |######
  12061.2 |######
  12069.7 |######
  12078.2 |######
  12086.8 |######
  12095.3 |
  12103.8 |#############
  12112.3 |######
  12120.9 |#################################
  12129.4 |#############
  12137.9 |####################
  12146.4 |########################################
  (4 below, 3 above range)

```
