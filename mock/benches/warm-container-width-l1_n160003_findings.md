# Container fork, declared-width sweep, cache-resident (8192 elements, 3 ops/element, wrapping)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-native beats baseline by 96% (significant)

warm-container-native is -8.02 us (96%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 22.0x slower than the field

warm-container-plusone (8.71 us) is 22.0x the fastest (396 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-native is fastest but the noisiest (CV 11.0%)

warm-container-native wins on median (396 ns) yet has the highest variance (CV 11.0%), while warm-container-lanes-deferred is the steadiest (CV 2.0%, 807 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (warm-container-native, warm-container-minimum) are a dead heat (<1%)

warm-container-native (396 ns) and warm-container-minimum (397 ns) differ by 0.43%, inside the noise, even though the wider field spreads 2102.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-lanes-deferred shows warm-up / thermal drift (autocorr +0.78)

warm-container-lanes-deferred's per-pass series has lag-1 autocorrelation +0.78, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-native, warm-container-minimum, warm-container-kernel, warm-container-lanes-deferred} vs {warm-container-headroom, warm-container-plusone} (935% apart)

The field splits into a fast tier {warm-container-native, warm-container-minimum, warm-container-kernel, warm-container-lanes-deferred} and a slow tier {warm-container-headroom, warm-container-plusone} with a 935% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 22.0x the fastest

Fastest warm-container-native (396 ns) to slowest warm-container-plusone (8.71 us): 22.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-container-native** at 395.6 ns median (-95.3% vs baseline)
- 4 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 22.02x (fastest 395.6 ns, slowest 8711.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 8701ns | 8414ns | 8345ns | 8447ns | 9819ns | base |
| warm-container-kernel | 909ns | 864ns | 845ns | 883ns | 1049ns | -89.56% |
| warm-container-lanes-deferred | 869ns | 869ns | 844ns | 869ns | 893ns | -90.01% |
| warm-container-minimum | 471ns | 460ns | 451ns | 458ns | 528ns | -94.59% |
| warm-container-native | 470ns | 460ns | 452ns | 460ns | 517ns | -94.60% |
| warm-container-plusone | 8770ns | 8775ns | 8515ns | 8751ns | 9082ns | +0.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 8638ns | 8287ns | 9748ns | base | 3.794 |
| warm-container-kernel | 846ns | 789ns | 978ns | -90.20% | 38.711 |
| warm-container-lanes-deferred | 809ns | 788ns | 833ns | -90.63% | 40.506 |
| warm-container-minimum | 408ns | 389ns | 461ns | -95.28% | 80.334 |
| warm-container-native | 406ns | 391ns | 450ns | -95.30% | 80.746 |
| warm-container-plusone | 8703ns | 8448ns | 9003ns | +0.75% | 3.765 |

## Performance model

- Peak throughput: **84.131 Gops/s** (warm-container-minimum; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 3.924 | 4.7% |
| warm-container-kernel | 40.863 | 48.6% |
| warm-container-lanes-deferred | 40.610 | 48.3% |
| warm-container-minimum | 82.477 | 98.0% |
| warm-container-native | 82.831 | 98.5% |
| warm-container-plusone | 3.761 | 4.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 8701ns | 8701ns | base |
| warm-container-kernel | 909ns | 909ns | -89.56% |
| warm-container-lanes-deferred | 869ns | 869ns | -90.01% |
| warm-container-minimum | 471ns | 471ns | -94.59% |
| warm-container-native | 470ns | 470ns | -94.60% |
| warm-container-plusone | 8770ns | 8770ns | +0.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 8351ns | base | --- | [8317, 8469] | --- | --- | --- | --- |
| warm-container-kernel | 802ns | -7558.5ns (-90.5%) | [-7667, -7500]ns | [792, 810] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 807ns | -7556.3ns (-90.5%) | [-7638, -7517]ns | [805, 808] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 397ns | -7954.8ns (-95.3%) | [-8050, -7913]ns | [393, 399] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 396ns | -7957.2ns (-95.3%) | [-8068, -7906]ns | [395, 398] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 8712ns | +324.6ns (+3.9%) | [+75, +412]ns | [8663, 8725] | YES | 0.0022 | 0.0022 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 8291ns | -87.0% | -90.3% | -95.0% | -95.1% | +5.2% |
| 2 | 8327ns | -90.0% | -90.3% | -95.1% | -95.3% | +5.2% |
| 3 | 8351ns | -90.5% | -90.3% | -95.0% | -95.3% | +4.5% |
| 4 | 8286ns | -90.4% | -90.3% | -95.1% | -95.3% | +5.4% |
| 5 | 9396ns | -91.1% | -91.4% | -95.8% | -95.9% | -7.2% |
| 6 | 10055ns | -92.1% | -92.0% | -96.0% | -96.1% | -13.2% |
| 7 | 10172ns | -92.2% | -92.1% | -96.1% | -96.2% | -14.2% |
| 8 | 10058ns | -92.1% | -91.9% | -96.1% | -96.1% | -12.5% |
| 9 | 9395ns | -91.6% | -91.3% | -95.8% | -95.9% | -7.2% |
| 10 | 8350ns | -90.5% | -90.3% | -95.2% | -95.3% | +4.5% |
| 11 | 8325ns | -88.4% | -90.3% | -95.3% | -95.3% | +1.4% |
| 12 | 8287ns | -88.4% | -90.3% | -95.2% | -95.2% | +5.0% |
| 13 | 8288ns | -88.4% | -90.3% | -95.3% | -95.2% | +5.0% |
| 14 | 8286ns | -88.4% | -90.3% | -95.3% | -95.0% | +5.0% |
| 15 | 9712ns | -90.1% | -91.7% | -96.0% | -95.9% | -10.4% |
| 16 | 8351ns | -88.5% | -90.4% | -95.3% | -95.3% | +5.5% |
| 17 | 8297ns | -88.4% | -90.3% | -95.3% | -95.3% | +5.6% |
| 18 | 8310ns | -88.4% | -90.1% | -95.3% | -95.2% | +4.8% |
| 19 | 9042ns | -89.4% | -91.1% | -95.7% | -95.6% | -5.5% |
| 20 | 10153ns | -90.5% | -92.1% | -96.2% | -96.1% | -16.4% |
| 21 | 8466ns | -90.5% | -90.2% | -95.3% | -95.3% | +0.2% |
| 22 | 8500ns | -90.5% | -90.2% | -94.3% | -95.3% | -1.0% |
| 23 | 8435ns | -90.5% | -90.1% | -95.3% | -95.2% | +0.5% |
| 24 | 8484ns | -90.5% | -90.2% | -95.3% | -95.3% | -0.8% |
| 25 | 8415ns | -90.5% | -90.1% | -95.3% | -95.3% | +0.7% |
| 26 | 8475ns | -90.6% | -90.2% | -95.3% | -95.3% | +1.4% |
| 27 | 8500ns | -90.6% | -90.2% | -95.3% | -94.9% | +5.5% |
| 28 | 8418ns | -90.5% | -90.1% | -95.3% | -95.3% | +5.2% |
| 29 | 8473ns | -90.5% | -90.2% | -95.3% | -95.3% | +3.7% |
| 30 | 8518ns | -90.5% | -90.2% | -95.3% | -95.3% | +1.1% |
| 31 | 8288ns | -90.5% | -90.5% | -93.2% | -95.2% | +4.1% |
| 32 | 8285ns | -90.5% | -90.5% | -95.3% | -95.2% | +3.8% |
| 33 | 8346ns | -90.5% | -90.6% | -95.3% | -95.3% | +4.5% |
| 34 | 8304ns | -90.5% | -90.5% | -94.1% | -95.2% | +5.0% |
| 35 | 8290ns | -90.2% | -90.5% | -94.8% | -95.3% | +5.0% |
| 36 | 8285ns | -90.4% | -90.5% | -95.2% | -95.2% | +8.2% |
| 37 | 8292ns | -90.5% | -90.5% | -95.3% | -95.2% | +14.0% |
| 38 | 8344ns | -90.5% | -90.5% | -95.4% | -92.0% | +12.6% |
| 39 | 8302ns | -90.5% | -90.5% | -95.3% | -95.0% | +2.9% |
| 40 | 8349ns | -90.5% | -90.5% | -94.3% | -94.5% | +0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.564 | HIGH+ (drift/warm-up) |
| warm-container-kernel | 0.655 | HIGH+ (drift/warm-up) |
| warm-container-lanes-deferred | 0.781 | HIGH+ (drift/warm-up) |
| warm-container-minimum | -0.015 | ok |
| warm-container-native | 0.051 | ok |
| warm-container-plusone | 0.568 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-lanes-deferred**: won 40/40, lost 0/40
- **warm-container-minimum**: won 40/40, lost 0/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 10/40, lost 30/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 2.7ns | 8637.5ns | 0.0% |  |
| warm-container-kernel | 2.5ns | 846.5ns | 0.3% |  |
| warm-container-lanes-deferred | 2.6ns | 809.0ns | 0.3% |  |
| warm-container-minimum | 2.3ns | 407.9ns | 0.6% |  |
| warm-container-native | 4.0ns | 405.8ns | 1.0% |  |
| warm-container-plusone | 3.0ns | 8702.6ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 8287.0-9747.8 ns)
   8287.0 |########################################
   8360.0 |####
   8433.0 |###############
   8506.1 |##
   8579.1 |
   8652.2 |
   8725.2 |
   8798.3 |
   8871.3 |
   8944.3 |
   9017.4 |##
   9090.4 |
   9163.5 |
   9236.5 |
   9309.6 |
   9382.6 |####
   9455.6 |
   9528.7 |
   9601.7 |
   9674.8 |##
  (4 below, 4 above range)

warm-container-kernel (n=40, range 789.0-977.6 ns)
    789.0 |########################################
    798.4 |#################################
    807.8 |###
    817.3 |
    826.7 |######
    836.1 |
    845.6 |
    855.0 |
    864.4 |
    873.9 |
    883.3 |
    892.7 |
    902.1 |
    911.6 |
    921.0 |
    930.4 |
    939.9 |
    949.3 |
    958.7 |#################################
    968.2 |
  (4 below, 1 above range)

warm-container-lanes-deferred (n=40, range 787.8-833.1 ns)
    787.8 |##########################
    790.1 |####
    792.3 |
    794.6 |
    796.9 |
    799.1 |########
    801.4 |####
    803.7 |#############
    805.9 |########################################
    808.2 |########
    810.5 |
    812.7 |####
    815.0 |
    817.3 |####
    819.5 |####
    821.8 |
    824.1 |
    826.3 |
    828.6 |########
    830.9 |##########################
  (3 below, 2 above range)

warm-container-minimum (n=40, range 389.5-460.9 ns)
    389.5 |#################################
    393.1 |#############
    396.6 |########################################
    400.2 |###
    403.8 |###
    407.3 |
    410.9 |##########
    414.5 |
    418.0 |
    421.6 |
    425.2 |
    428.7 |###
    432.3 |
    435.9 |
    439.4 |
    443.0 |
    446.6 |
    450.1 |
    453.7 |
    457.3 |
  (4 below, 4 above range)

warm-container-native (n=40, range 390.6-450.3 ns)
    390.6 |#######################
    393.6 |########################################
    396.5 |##############################
    399.5 |##########
    402.5 |
    405.5 |
    408.5 |###
    411.5 |###
    414.4 |###
    417.4 |
    420.4 |
    423.4 |
    426.4 |
    429.4 |
    432.4 |###
    435.3 |
    438.3 |
    441.3 |
    444.3 |
    447.3 |
  (3 below, 2 above range)

warm-container-plusone (n=40, range 8448.4-9002.9 ns)
   8448.4 |###
   8476.1 |#########
   8503.9 |
   8531.6 |######
   8559.3 |
   8587.0 |#########
   8614.8 |###
   8642.5 |
   8670.2 |
   8697.9 |########################################
   8725.6 |#########
   8753.4 |######
   8781.1 |#########
   8808.8 |
   8836.5 |###
   8864.3 |
   8892.0 |
   8919.7 |
   8947.4 |######
   8975.2 |
  (4 below, 2 above range)

```

## Diagnostics

- **warm-container-headroom**: autocorrelation=0.56 (measurement drift or warm-up artifact)
- **warm-container-kernel**: autocorrelation=0.65 (measurement drift or warm-up artifact)
- **warm-container-lanes-deferred**: autocorrelation=0.78 (measurement drift or warm-up artifact)
- **warm-container-plusone**: autocorrelation=0.57 (measurement drift or warm-up artifact)
