# Layout::Bitpacked decoder shape: index-driven vs plan-driven, across the L1 boundary

4 variants, 40 samples per variant.
Baseline: **bitpack-plan-naive**

## Highlights

Baseline for all deltas below: **bitpack-plan-naive**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-plan-naive) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-plan-naive has the worst median (142.52 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-plan-simd at 32.21 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-plan-simd beats baseline by 77% (significant)

bitpack-plan-simd is -110.38 us (77%) faster than baseline bitpack-plan-naive, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-plan-naive is an outlier: 4.4x slower than the field

bitpack-plan-naive (142.52 us) is 4.4x the fastest (32.21 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (bitpack-plan-simd, bitpack-plan-windowed) are a dead heat (<1%)

bitpack-plan-simd (32.21 us) and bitpack-plan-windowed (32.21 us) differ by 0.01%, inside the noise, even though the wider field spreads 342.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### bitpack-plan-naive shows alternating (throttle bounce) (autocorr -0.59)

bitpack-plan-naive's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-plan-simd, bitpack-plan-windowed, bitpack-plan-native} vs {bitpack-plan-naive} (257% apart)

The field splits into a fast tier {bitpack-plan-simd, bitpack-plan-windowed, bitpack-plan-native} and a slow tier {bitpack-plan-naive} with a 257% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.4x the fastest

Fastest bitpack-plan-simd (32.21 us) to slowest bitpack-plan-naive (142.52 us): 4.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-plan-simd** at 32210.5 ns median (-77.4% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 4.42x (fastest 32210.5 ns, slowest 142520.6 ns)

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
| bitpack-plan-naive | 142783ns | 142605ns | 142366ns | 142596ns | 143761ns | base |
| bitpack-plan-native | 40021ns | 40006ns | 39931ns | 40002ns | 40165ns | -71.97% |
| bitpack-plan-simd | 32243ns | 32289ns | 31983ns | 32204ns | 32621ns | -77.42% |
| bitpack-plan-windowed | 32327ns | 32295ns | 32277ns | 32299ns | 32458ns | -77.36% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-plan-naive | 142698ns | 142286ns | 143674ns | base | 1.837 |
| bitpack-plan-native | 39935ns | 39849ns | 40069ns | -72.01% | 6.564 |
| bitpack-plan-simd | 32161ns | 31907ns | 32528ns | -77.46% | 8.151 |
| bitpack-plan-windowed | 32242ns | 32196ns | 32369ns | -77.41% | 8.130 |

## Performance model

- Peak throughput: **8.216 Gops/s** (bitpack-plan-simd; best 20% batches)
- Ops per call: 262144

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-plan-naive | 1.839 | 22.4% |
| bitpack-plan-native | 6.566 | 79.9% |
| bitpack-plan-simd | 8.138 | 99.1% |
| bitpack-plan-windowed | 8.138 | 99.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-plan-naive | 142783ns | 142783ns | base |
| bitpack-plan-native | 40021ns | 40021ns | -71.97% |
| bitpack-plan-simd | 32243ns | 32243ns | -77.42% |
| bitpack-plan-windowed | 32327ns | 32327ns | -77.36% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-plan-naive | 142521ns | base | --- | [142430, 142584] | --- | --- | --- | --- |
| bitpack-plan-native | 39924ns | -102596.0ns (-72.0%) | [-102682, -102506]ns | [39918, 39928] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-plan-simd | 32210ns | -110376.5ns (-77.4%) | [-110468, -110294]ns | [31931, 32224] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-plan-windowed | 32214ns | -110290.6ns (-77.4%) | [-110338, -110197]ns | [32208, 32223] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-plan-naive | bitpack-plan-native | bitpack-plan-simd | bitpack-plan-windowed |
|---|---|---|---|---|---|
| 1 | 1 | 142368ns | -72.0% | -77.6% | -77.4% |
| 1 | 2 | 142693ns | -72.0% | -77.6% | -77.4% |
| 1 | 3 | 142483ns | -72.0% | -77.4% | -77.4% |
| 1 | 4 | 142518ns | -72.0% | -77.4% | -77.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-plan-naive | -0.589 | HIGH- (thermal bounce) |
| bitpack-plan-native | -0.011 | ok |
| bitpack-plan-simd | 0.251 | moderate+ |
| bitpack-plan-windowed | -0.328 | moderate- |

**Consistency summary:**

- **bitpack-plan-native**: won 4/4, lost 0/4
- **bitpack-plan-simd**: won 4/4, lost 0/4
- **bitpack-plan-windowed**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-plan-naive | 23.5ns | 142698.1ns | 0.0% |  |
| bitpack-plan-native | 22.3ns | 39934.8ns | 0.1% |  |
| bitpack-plan-simd | 21.8ns | 32161.3ns | 0.1% |  |
| bitpack-plan-windowed | 22.0ns | 32242.1ns | 0.1% |  |

## Distribution (algo ns)

```
bitpack-plan-naive (n=40, range 142285.7-143673.9 ns)
  142285.7 |#################################
  142355.1 |##########################
  142424.5 |#################################
  142493.9 |########################################
  142563.3 |########################################
  142632.7 |#############
  142702.1 |######
  142771.6 |#############
  142841.0 |
  142910.4 |######
  142979.8 |
  143049.2 |
  143118.6 |
  143188.0 |
  143257.4 |
  143326.9 |
  143396.3 |
  143465.7 |#############
  143535.1 |######
  143604.5 |
  (4 below, 1 above range)

bitpack-plan-native (n=40, range 39848.9-40069.1 ns)
  39848.9 |##############
  39859.9 |##
  39870.9 |
  39881.9 |
  39892.9 |
  39903.9 |########
  39915.0 |######################
  39926.0 |########################################
  39937.0 |#####
  39948.0 |#####
  39959.0 |
  39970.0 |
  39981.0 |
  39992.0 |
  40003.0 |
  40014.0 |
  40025.1 |
  40036.1 |
  40047.1 |
  40058.1 |
  (4 below, 1 above range)

bitpack-plan-simd (n=40, range 31907.5-32527.8 ns)
  31907.5 |#################################
  31938.5 |
  31969.5 |
  32000.5 |###
  32031.5 |
  32062.5 |
  32093.5 |
  32124.6 |
  32155.6 |
  32186.6 |#####################
  32217.6 |########################################
  32248.6 |
  32279.6 |
  32310.6 |
  32341.7 |
  32372.7 |###
  32403.7 |
  32434.7 |
  32465.7 |###
  32496.7 |
  (4 below, 2 above range)

bitpack-plan-windowed (n=40, range 32196.2-32368.8 ns)
  32196.2 |###############################
  32204.8 |########################################
  32213.4 |##########################
  32222.1 |######################
  32230.7 |######################
  32239.3 |####
  32248.0 |####
  32256.6 |
  32265.2 |
  32273.9 |
  32282.5 |
  32291.1 |
  32299.7 |
  32308.4 |
  32317.0 |
  32325.6 |
  32334.3 |
  32342.9 |
  32351.5 |
  32360.2 |
  (4 below, 2 above range)

```
