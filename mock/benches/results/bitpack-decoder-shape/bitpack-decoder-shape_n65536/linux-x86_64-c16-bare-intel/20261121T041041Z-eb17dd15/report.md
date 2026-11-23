# Layout::Bitpacked decoder shape: index-driven vs plan-driven, across the L1 boundary

4 variants, 40 samples per variant.
Baseline: **bitpack-plan-naive**

## Highlights

Baseline for all deltas below: **bitpack-plan-naive**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-plan-naive) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-plan-naive has the worst median (35.57 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-plan-simd at 8.05 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-plan-simd beats baseline by 77% (significant)

bitpack-plan-simd is -27.52 us (77%) faster than baseline bitpack-plan-naive, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-plan-naive is an outlier: 4.4x slower than the field

bitpack-plan-naive (35.57 us) is 4.4x the fastest (8.05 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (bitpack-plan-simd, bitpack-plan-windowed) are a dead heat (<1%)

bitpack-plan-simd (8.05 us) and bitpack-plan-windowed (8.05 us) differ by 0.01%, inside the noise, even though the wider field spreads 341.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {bitpack-plan-simd, bitpack-plan-windowed, bitpack-plan-native} vs {bitpack-plan-naive} (257% apart)

The field splits into a fast tier {bitpack-plan-simd, bitpack-plan-windowed, bitpack-plan-native} and a slow tier {bitpack-plan-naive} with a 257% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.4x the fastest

Fastest bitpack-plan-simd (8.05 us) to slowest bitpack-plan-naive (35.57 us): 4.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-plan-simd** at 8054.3 ns median (-77.4% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 4.42x (fastest 8054.3 ns, slowest 35567.5 ns)

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
| bitpack-plan-naive | 35321ns | 35637ns | 34290ns | 35529ns | 35728ns | base |
| bitpack-plan-native | 10048ns | 10042ns | 10038ns | 10042ns | 10074ns | -71.55% |
| bitpack-plan-simd | 8113ns | 8121ns | 8050ns | 8116ns | 8167ns | -77.03% |
| bitpack-plan-windowed | 8124ns | 8122ns | 8115ns | 8122ns | 8142ns | -77.00% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-plan-naive | 35247ns | 34217ns | 35644ns | base | 1.859 |
| bitpack-plan-native | 9977ns | 9969ns | 9988ns | -71.70% | 6.569 |
| bitpack-plan-simd | 8042ns | 7982ns | 8088ns | -77.18% | 8.149 |
| bitpack-plan-windowed | 8054ns | 8046ns | 8059ns | -77.15% | 8.137 |

## Performance model

- Peak throughput: **8.211 Gops/s** (bitpack-plan-simd; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-plan-naive | 1.843 | 22.4% |
| bitpack-plan-native | 6.570 | 80.0% |
| bitpack-plan-simd | 8.137 | 99.1% |
| bitpack-plan-windowed | 8.136 | 99.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-plan-naive | 35321ns | 35321ns | base |
| bitpack-plan-native | 10048ns | 10048ns | -71.55% |
| bitpack-plan-simd | 8113ns | 8113ns | -77.03% |
| bitpack-plan-windowed | 8124ns | 8124ns | -77.00% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-plan-naive | 35568ns | base | --- | [35554, 35586] | --- | --- | --- | --- |
| bitpack-plan-native | 9975ns | -25589.5ns (-71.9%) | [-25609, -25569]ns | [9975, 9976] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-plan-simd | 8054ns | -27519.9ns (-77.4%) | [-27535, -27506]ns | [8045, 8055] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-plan-windowed | 8055ns | -27514.1ns (-77.4%) | [-27532, -27495]ns | [8055, 8055] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-plan-naive | bitpack-plan-native | bitpack-plan-simd | bitpack-plan-windowed |
|---|---|---|---|---|---|
| 1 | 1 | 35576ns | -72.0% | -77.4% | -77.4% |
| 1 | 2 | 35575ns | -72.0% | -77.4% | -77.4% |
| 1 | 3 | 34221ns | -70.8% | -76.5% | -76.5% |
| 1 | 4 | 35595ns | -72.0% | -77.6% | -77.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-plan-naive | -0.424 | moderate- |
| bitpack-plan-native | -0.103 | ok |
| bitpack-plan-simd | -0.091 | ok |
| bitpack-plan-windowed | -0.362 | moderate- |

**Consistency summary:**

- **bitpack-plan-native**: won 4/4, lost 0/4
- **bitpack-plan-simd**: won 4/4, lost 0/4
- **bitpack-plan-windowed**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-plan-naive | 21.0ns | 35247.5ns | 0.1% |  |
| bitpack-plan-native | 20.6ns | 9976.7ns | 0.2% |  |
| bitpack-plan-simd | 20.9ns | 8042.0ns | 0.3% |  |
| bitpack-plan-windowed | 20.8ns | 8054.0ns | 0.3% |  |

## Distribution (algo ns)

```
bitpack-plan-naive (n=40, range 34217.0-35644.2 ns)
  34217.0 |#############
  34288.4 |
  34359.7 |
  34431.1 |
  34502.4 |
  34573.8 |
  34645.2 |
  34716.5 |
  34787.9 |
  34859.2 |
  34930.6 |
  35001.9 |
  35073.3 |
  35144.7 |
  35216.0 |
  35287.4 |
  35358.7 |
  35430.1 |
  35501.4 |########################
  35572.8 |########################################
  (4 below, 1 above range)

bitpack-plan-native (n=40, range 9968.6-9988.4 ns)
   9968.6 |#######
   9969.6 |
   9970.6 |
   9971.6 |
   9972.6 |
   9973.6 |
   9974.5 |########################################
   9975.5 |######################
   9976.5 |#####
   9977.5 |##
   9978.5 |
   9979.5 |
   9980.5 |##
   9981.5 |
   9982.5 |
   9983.4 |
   9984.4 |
   9985.4 |
   9986.4 |
   9987.4 |
  (5 below, 3 above range)

bitpack-plan-simd (n=40, range 7981.9-8087.5 ns)
   7981.9 |##
   7987.1 |#############
   7992.4 |
   7997.7 |
   8003.0 |
   8008.3 |
   8013.6 |
   8018.8 |
   8024.1 |
   8029.4 |
   8034.7 |
   8040.0 |###########
   8045.3 |##
   8050.5 |########################################
   8055.8 |########
   8061.1 |
   8066.4 |
   8071.7 |
   8076.9 |
   8082.2 |
  (3 below, 2 above range)

bitpack-plan-windowed (n=40, range 8046.1-8059.2 ns)
   8046.1 |
   8046.7 |
   8047.4 |
   8048.1 |
   8048.7 |
   8049.4 |
   8050.0 |
   8050.7 |
   8051.3 |
   8052.0 |
   8052.7 |
   8053.3 |####
   8054.0 |########################################
   8054.6 |###################################
   8055.3 |########################################
   8055.9 |#############
   8056.6 |
   8057.3 |
   8057.9 |
   8058.6 |########
  (7 below, 1 above range)

```
