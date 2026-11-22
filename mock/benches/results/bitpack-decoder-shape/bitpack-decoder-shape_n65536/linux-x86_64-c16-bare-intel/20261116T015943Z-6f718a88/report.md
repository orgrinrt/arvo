# Layout::Bitpacked decoder shape: index-driven vs plan-driven, across the L1 boundary

4 variants, 40 samples per variant.
Baseline: **bitpack-plan-naive**

## Highlights

Baseline for all deltas below: **bitpack-plan-naive**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-plan-naive) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-plan-naive has the worst median (35.90 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-plan-windowed at 8.11 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-plan-windowed beats baseline by 77% (significant)

bitpack-plan-windowed is -27.79 us (77%) faster than baseline bitpack-plan-naive, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-plan-naive is an outlier: 4.4x slower than the field

bitpack-plan-naive (35.90 us) is 4.4x the fastest (8.11 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (bitpack-plan-windowed, bitpack-plan-simd) are a dead heat (<1%)

bitpack-plan-windowed (8.11 us) and bitpack-plan-simd (8.11 us) differ by 0.07%, inside the noise, even though the wider field spreads 342.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {bitpack-plan-windowed, bitpack-plan-simd, bitpack-plan-native} vs {bitpack-plan-naive} (260% apart)

The field splits into a fast tier {bitpack-plan-windowed, bitpack-plan-simd, bitpack-plan-native} and a slow tier {bitpack-plan-naive} with a 260% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.4x the fastest

Fastest bitpack-plan-windowed (8.11 us) to slowest bitpack-plan-naive (35.90 us): 4.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-plan-windowed** at 8108.8 ns median (-77.4% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 4.43x (fastest 8108.8 ns, slowest 35899.2 ns)

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
| bitpack-plan-naive | 35432ns | 35972ns | 34308ns | 35615ns | 36004ns | base |
| bitpack-plan-native | 10065ns | 10049ns | 10046ns | 10050ns | 10131ns | -71.59% |
| bitpack-plan-simd | 8962ns | 8181ns | 8173ns | 8445ns | 11304ns | -74.71% |
| bitpack-plan-windowed | 8164ns | 8177ns | 8114ns | 8171ns | 8193ns | -76.96% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-plan-naive | 35357ns | 34234ns | 35918ns | base | 1.854 |
| bitpack-plan-native | 9993ns | 9978ns | 10043ns | -71.74% | 6.558 |
| bitpack-plan-simd | 8871ns | 8106ns | 11151ns | -74.91% | 7.387 |
| bitpack-plan-windowed | 8093ns | 8046ns | 8115ns | -77.11% | 8.098 |

## Performance model

- Peak throughput: **8.145 Gops/s** (bitpack-plan-windowed; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-plan-naive | 1.826 | 22.4% |
| bitpack-plan-native | 6.566 | 80.6% |
| bitpack-plan-simd | 8.077 | 99.2% |
| bitpack-plan-windowed | 8.082 | 99.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-plan-naive | 35432ns | 35432ns | base |
| bitpack-plan-native | 10065ns | 10065ns | -71.59% |
| bitpack-plan-simd | 8962ns | 8962ns | -74.71% |
| bitpack-plan-windowed | 8164ns | 8164ns | -76.96% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-plan-naive | 35899ns | base | --- | [35612, 35906] | --- | --- | --- | --- |
| bitpack-plan-native | 9981ns | -25919.3ns (-72.2%) | [-25926, -25631]ns | [9980, 9981] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-plan-simd | 8114ns | -27783.2ns (-77.4%) | [-27791, -26144]ns | [8114, 8116] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-plan-windowed | 8109ns | -27791.4ns (-77.4%) | [-27798, -27500]ns | [8103, 8112] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-plan-naive | bitpack-plan-native | bitpack-plan-simd | bitpack-plan-windowed |
|---|---|---|---|---|---|
| 1 | 1 | 35909ns | -72.2% | -77.4% | -77.4% |
| 1 | 2 | 34261ns | -70.9% | -76.3% | -76.3% |
| 1 | 3 | 34266ns | -70.9% | -67.5% | -76.5% |
| 1 | 4 | 35906ns | -72.2% | -77.4% | -77.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-plan-naive | -0.250 | moderate- |
| bitpack-plan-native | 0.021 | ok |
| bitpack-plan-simd | -0.417 | moderate- |
| bitpack-plan-windowed | -0.364 | moderate- |

**Consistency summary:**

- **bitpack-plan-native**: won 4/4, lost 0/4
- **bitpack-plan-simd**: won 4/4, lost 0/4
- **bitpack-plan-windowed**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-plan-naive | 19.6ns | 35356.6ns | 0.1% |  |
| bitpack-plan-native | 19.3ns | 9992.6ns | 0.2% |  |
| bitpack-plan-simd | 24.1ns | 8871.3ns | 0.3% |  |
| bitpack-plan-windowed | 19.3ns | 8093.0ns | 0.2% |  |

## Distribution (algo ns)

```
bitpack-plan-naive (n=40, range 34234.5-35917.6 ns)
  34234.5 |###############
  34318.7 |
  34402.8 |#
  34487.0 |
  34571.1 |
  34655.3 |
  34739.4 |
  34823.6 |
  34907.7 |
  34991.9 |
  35076.0 |
  35160.2 |
  35244.3 |
  35328.5 |#
  35412.6 |
  35496.8 |
  35580.9 |
  35665.1 |
  35749.2 |
  35833.4 |########################################
  (3 below, 2 above range)

bitpack-plan-native (n=40, range 9977.7-10043.0 ns)
   9977.7 |########################################
   9981.0 |#################################
   9984.2 |
   9987.5 |
   9990.8 |
   9994.0 |
   9997.3 |
  10000.6 |##
  10003.8 |
  10007.1 |
  10010.3 |
  10013.6 |
  10016.9 |
  10020.1 |
  10023.4 |
  10026.7 |
  10029.9 |
  10033.2 |
  10036.5 |
  10039.7 |
  (4 below, 2 above range)

bitpack-plan-simd (n=40, range 8106.0-11150.6 ns)
   8106.0 |########################################
   8258.2 |
   8410.4 |
   8562.7 |
   8714.9 |
   8867.1 |
   9019.4 |
   9171.6 |
   9323.8 |
   9476.0 |
   9628.3 |
   9780.5 |
   9932.7 |
  10085.0 |
  10237.2 |
  10389.4 |
  10541.7 |
  10693.9 |
  10846.1 |
  10998.4 |###########
  (6 below, 3 above range)

bitpack-plan-windowed (n=40, range 8046.2-8114.8 ns)
   8046.2 |
   8049.6 |####
   8053.0 |####
   8056.5 |####
   8059.9 |########################
   8063.3 |####
   8066.8 |
   8070.2 |
   8073.6 |
   8077.0 |
   8080.5 |
   8083.9 |
   8087.3 |
   8090.8 |
   8094.2 |
   8097.6 |
   8101.0 |################
   8104.5 |########
   8107.9 |################################
   8111.3 |########################################
  (2 below, 4 above range)

```
