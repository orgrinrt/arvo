# Layout::Bitpacked decoder shape: index-driven vs plan-driven, across the L1 boundary

4 variants, 40 samples per variant.
Baseline: **bitpack-plan-naive**

## Highlights

Baseline for all deltas below: **bitpack-plan-naive**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-plan-naive) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-plan-naive has the worst median (143.71 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-plan-windowed at 32.44 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-plan-windowed beats baseline by 77% (significant)

bitpack-plan-windowed is -111.27 us (77%) faster than baseline bitpack-plan-naive, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-plan-naive is an outlier: 4.4x slower than the field

bitpack-plan-naive (143.71 us) is 4.4x the fastest (32.44 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (bitpack-plan-windowed, bitpack-plan-simd) are a dead heat (<1%)

bitpack-plan-windowed (32.44 us) and bitpack-plan-simd (32.45 us) differ by 0.02%, inside the noise, even though the wider field spreads 343.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### bitpack-plan-windowed shows alternating (throttle bounce) (autocorr -0.59)

bitpack-plan-windowed's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-plan-windowed, bitpack-plan-simd, bitpack-plan-native} vs {bitpack-plan-naive} (260% apart)

The field splits into a fast tier {bitpack-plan-windowed, bitpack-plan-simd, bitpack-plan-native} and a slow tier {bitpack-plan-naive} with a 260% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.4x the fastest

Fastest bitpack-plan-windowed (32.44 us) to slowest bitpack-plan-naive (143.71 us): 4.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-plan-windowed** at 32440.5 ns median (-77.4% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 4.43x (fastest 32440.5 ns, slowest 143709.2 ns)

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
| bitpack-plan-naive | 141726ns | 143787ns | 137211ns | 142431ns | 144128ns | base |
| bitpack-plan-native | 40010ns | 39994ns | 39969ns | 39994ns | 40101ns | -71.77% |
| bitpack-plan-simd | 32531ns | 32525ns | 32519ns | 32526ns | 32558ns | -77.05% |
| bitpack-plan-windowed | 32523ns | 32521ns | 32484ns | 32521ns | 32569ns | -77.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-plan-naive | 141643ns | 137129ns | 144034ns | base | 1.851 |
| bitpack-plan-native | 39926ns | 39887ns | 40007ns | -71.81% | 6.566 |
| bitpack-plan-simd | 32449ns | 32442ns | 32464ns | -77.09% | 8.079 |
| bitpack-plan-windowed | 32439ns | 32402ns | 32477ns | -77.10% | 8.081 |

## Performance model

- Peak throughput: **8.090 Gops/s** (bitpack-plan-windowed; best 20% batches)
- Ops per call: 262144

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-plan-naive | 1.824 | 22.5% |
| bitpack-plan-native | 6.568 | 81.2% |
| bitpack-plan-simd | 8.079 | 99.9% |
| bitpack-plan-windowed | 8.081 | 99.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-plan-naive | 141726ns | 141726ns | base |
| bitpack-plan-native | 40010ns | 40010ns | -71.77% |
| bitpack-plan-simd | 32531ns | 32531ns | -77.05% |
| bitpack-plan-windowed | 32523ns | 32523ns | -77.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-plan-naive | 143709ns | base | --- | [143700, 143713] | --- | --- | --- | --- |
| bitpack-plan-native | 39914ns | -103792.8ns (-72.2%) | [-103801, -103551]ns | [39909, 39916] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-plan-simd | 32446ns | -111261.9ns (-77.4%) | [-111266, -111222]ns | [32445, 32448] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-plan-windowed | 32440ns | -111267.7ns (-77.4%) | [-111278, -111153]ns | [32435, 32443] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-plan-naive | bitpack-plan-native | bitpack-plan-simd | bitpack-plan-windowed |
|---|---|---|---|---|---|
| 1 | 1 | 137143ns | -70.9% | -76.3% | -76.3% |
| 1 | 2 | 143716ns | -72.2% | -77.4% | -77.4% |
| 1 | 3 | 143712ns | -72.2% | -77.4% | -77.4% |
| 1 | 4 | 143710ns | -72.2% | -77.4% | -77.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-plan-naive | -0.084 | ok |
| bitpack-plan-native | -0.524 | HIGH- (thermal bounce) |
| bitpack-plan-simd | -0.243 | moderate- |
| bitpack-plan-windowed | -0.589 | HIGH- (thermal bounce) |

**Consistency summary:**

- **bitpack-plan-native**: won 4/4, lost 0/4
- **bitpack-plan-simd**: won 4/4, lost 0/4
- **bitpack-plan-windowed**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-plan-naive | 20.1ns | 141642.9ns | 0.0% |  |
| bitpack-plan-native | 19.3ns | 39926.3ns | 0.0% |  |
| bitpack-plan-simd | 19.2ns | 32449.4ns | 0.1% |  |
| bitpack-plan-windowed | 19.2ns | 32439.2ns | 0.1% |  |

## Distribution (algo ns)

```
bitpack-plan-naive (n=40, range 137128.9-144033.6 ns)
  137128.9 |###############
  137474.1 |
  137819.4 |
  138164.6 |
  138509.8 |
  138855.1 |
  139200.3 |
  139545.5 |
  139890.8 |
  140236.0 |
  140581.2 |
  140926.5 |
  141271.7 |
  141616.9 |
  141962.2 |
  142307.4 |
  142652.6 |
  142997.9 |
  143343.1 |
  143688.3 |########################################
  (3 below, 1 above range)

bitpack-plan-native (n=40, range 39887.4-40007.4 ns)
  39887.4 |##########
  39893.4 |#######
  39899.4 |##############
  39905.4 |##########
  39911.4 |########################################
  39917.4 |####################################
  39923.4 |
  39929.4 |
  39935.4 |
  39941.4 |
  39947.4 |
  39953.4 |
  39959.4 |
  39965.4 |
  39971.4 |
  39977.4 |
  39983.4 |
  39989.4 |
  39995.4 |
  40001.4 |
  (4 below, 3 above range)

bitpack-plan-simd (n=40, range 32442.0-32464.2 ns)
  32442.0 |#########################
  32443.1 |####################
  32444.2 |########################################
  32445.3 |##########
  32446.4 |##########
  32447.6 |##########
  32448.7 |##########
  32449.8 |
  32450.9 |
  32452.0 |##########
  32453.1 |###############
  32454.2 |#####
  32455.3 |###############
  32456.4 |#####
  32457.6 |
  32458.7 |
  32459.8 |
  32460.9 |
  32462.0 |
  32463.1 |
  (3 below, 2 above range)

bitpack-plan-windowed (n=40, range 32401.7-32477.3 ns)
  32401.7 |
  32405.5 |
  32409.2 |#####
  32413.0 |#####
  32416.8 |#####
  32420.6 |#####
  32424.4 |###########
  32428.1 |######################
  32431.9 |#################
  32435.7 |######################
  32439.5 |########################################
  32443.3 |############################
  32447.0 |######################
  32450.8 |###########
  32454.6 |###########
  32458.4 |
  32462.1 |
  32465.9 |
  32469.7 |
  32473.5 |
  (2 below, 1 above range)

```
