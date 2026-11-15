# Container fork, declared-width sweep, 1048576 elements (3 ops/element, wrapping)

4 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-minimum beats baseline by 86% (significant)

warm-container-minimum is -160.76 us (86%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 7.3x slower than the field

warm-container-plusone (187.14 us) is 7.3x the fastest (25.67 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-container-minimum, warm-container-native) are a dead heat (<1%)

warm-container-minimum (25.67 us) and warm-container-native (25.73 us) differ by 0.21%, inside the noise, even though the wider field spreads 628.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {warm-container-minimum, warm-container-native} vs {warm-container-headroom, warm-container-plusone} (625% apart)

The field splits into a fast tier {warm-container-minimum, warm-container-native} and a slow tier {warm-container-headroom, warm-container-plusone} with a 625% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 7.3x the fastest

Fastest warm-container-minimum (25.67 us) to slowest warm-container-plusone (187.14 us): 7.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-container-minimum** at 25672.8 ns median (-86.2% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 7.29x (fastest 25672.8 ns, slowest 187136.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 186295ns | 186587ns | 185110ns | 186489ns | 186899ns | base |
| warm-container-minimum | 25776ns | 25746ns | 25522ns | 25743ns | 26130ns | -86.16% |
| warm-container-native | 25867ns | 25804ns | 25761ns | 25805ns | 26160ns | -86.12% |
| warm-container-plusone | 186976ns | 187283ns | 185515ns | 187109ns | 188039ns | +0.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 186143ns | 184952ns | 186744ns | base | 22.533 |
| warm-container-minimum | 25699ns | 25445ns | 26044ns | -86.19% | 163.210 |
| warm-container-native | 25789ns | 25686ns | 26084ns | -86.15% | 162.638 |
| warm-container-plusone | 186818ns | 185348ns | 187881ns | +0.36% | 22.451 |

## Performance model

- Peak throughput: **164.839 Gops/s** (warm-container-minimum; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 22.496 | 13.6% |
| warm-container-minimum | 163.375 | 99.1% |
| warm-container-native | 163.034 | 98.9% |
| warm-container-plusone | 22.413 | 13.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 186295ns | 186295ns | base |
| warm-container-minimum | 25776ns | 25776ns | -86.16% |
| warm-container-native | 25867ns | 25867ns | -86.12% |
| warm-container-plusone | 186976ns | 186976ns | +0.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 186445ns | base | --- | [186432, 186456] | --- | --- | --- | --- |
| warm-container-minimum | 25673ns | -160757.4ns (-86.2%) | [-160878, -160553]ns | [25620, 25706] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 25726ns | -160735.8ns (-86.2%) | [-160765, -160691]ns | [25712, 25735] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 187136ns | +677.6ns (+0.4%) | [+231, +1279]ns | [186843, 187355] | YES | 0.0166 | 0.0166 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|
| 1 | 1 | 186445ns | -86.3% | -86.2% | +0.3% |
| 1 | 2 | 184966ns | -86.1% | -86.1% | +1.3% |
| 1 | 3 | 186591ns | -86.2% | -86.2% | +0.5% |
| 1 | 4 | 186447ns | -86.2% | -86.2% | -0.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.436 | moderate- |
| warm-container-minimum | 0.056 | ok |
| warm-container-native | -0.215 | moderate- |
| warm-container-plusone | -0.209 | moderate- |

**Consistency summary:**

- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 19.8ns | 186143.2ns | 0.0% |  |
| warm-container-minimum | 20.3ns | 25698.8ns | 0.1% |  |
| warm-container-native | 20.3ns | 25789.3ns | 0.1% |  |
| warm-container-plusone | 20.3ns | 186817.5ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 184952.1-186743.9 ns)
  184952.1 |###########
  185041.7 |##
  185131.3 |
  185220.9 |
  185310.4 |
  185400.0 |
  185489.6 |
  185579.2 |
  185668.8 |
  185758.4 |
  185848.0 |
  185937.6 |
  186027.2 |
  186116.7 |
  186206.3 |
  186295.9 |
  186385.5 |########################################
  186475.1 |###########
  186564.7 |#########
  186654.3 |####
  (4 below, 2 above range)

warm-container-minimum (n=40, range 25444.9-26043.8 ns)
  25444.9 |################
  25474.8 |########################
  25504.8 |################
  25534.7 |
  25564.7 |################
  25594.6 |################
  25624.6 |########################
  25654.5 |################################
  25684.5 |########################################
  25714.4 |########
  25744.3 |########
  25774.3 |
  25804.2 |########
  25834.2 |################
  25864.1 |
  25894.1 |################
  25924.0 |
  25954.0 |################
  25983.9 |
  26013.9 |
  (4 below, 4 above range)

warm-container-native (n=40, range 25686.2-26084.0 ns)
  25686.2 |#############################
  25706.1 |################################
  25726.0 |########################################
  25745.9 |##########
  25765.8 |
  25785.7 |
  25805.6 |
  25825.4 |#######
  25845.3 |
  25865.2 |
  25885.1 |
  25905.0 |
  25924.9 |
  25944.8 |
  25964.6 |
  25984.5 |
  26004.4 |
  26024.3 |
  26044.2 |
  26064.1 |
  (2 below, 5 above range)

warm-container-plusone (n=40, range 185347.5-187881.0 ns)
  185347.5 |################
  185474.2 |################################
  185600.9 |
  185727.5 |################
  185854.2 |
  185980.9 |
  186107.6 |
  186234.2 |
  186360.9 |
  186487.6 |########
  186614.3 |
  186740.9 |########
  186867.6 |################
  186994.3 |################################
  187121.0 |########################
  187247.6 |########################################
  187374.3 |########
  187501.0 |########
  187627.7 |########################
  187754.3 |################
  (4 below, 5 above range)

```
