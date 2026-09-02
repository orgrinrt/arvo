# Wrapping reduction whose steps are all affine: what the interior projection prevents the optimiser from doing (8192 elements, 3 ops/element)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-lanes-deferred beats baseline by 94% (significant)

warm-container-lanes-deferred is -8.14 us (94%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-minimum is an outlier: 23.9x slower than the field

warm-container-minimum (9.38 us) is 23.9x the fastest (392 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-container-kernel, warm-container-lanes-deferred) are a dead heat (<1%)

warm-container-kernel (392 ns) and warm-container-lanes-deferred (392 ns) differ by 0.17%, inside the noise, even though the wider field spreads 2295.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-native shows warm-up / thermal drift (autocorr +0.90)

warm-container-native's per-pass series has lag-1 autocorrelation +0.90, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-kernel, warm-container-lanes-deferred, warm-container-native} vs {warm-container-plusone, warm-container-headroom, warm-container-minimum} (1959% apart)

The field splits into a fast tier {warm-container-kernel, warm-container-lanes-deferred, warm-container-native} and a slow tier {warm-container-plusone, warm-container-headroom, warm-container-minimum} with a 1959% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 23.9x the fastest

Fastest warm-container-kernel (392 ns) to slowest warm-container-minimum (9.38 us): 23.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-plusone's edge over baseline is significant but tiny (-48 ns, 0.55%)

warm-container-plusone differs from baseline warm-container-headroom by -48 ns (0.55%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-container-kernel** at 391.6 ns median (-95.5% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 23.95x (fastest 391.6 ns, slowest 9380.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 8758ns | 8711ns | 8366ns | 8650ns | 9476ns | base |
| warm-container-kernel | 459ns | 453ns | 451ns | 454ns | 484ns | -94.76% |
| warm-container-lanes-deferred | 456ns | 456ns | 452ns | 456ns | 461ns | -94.79% |
| warm-container-minimum | 9369ns | 9447ns | 8415ns | 9422ns | 10166ns | +6.98% |
| warm-container-native | 497ns | 468ns | 451ns | 479ns | 596ns | -94.33% |
| warm-container-plusone | 8716ns | 8388ns | 8347ns | 8492ns | 9754ns | -0.49% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 8693ns | 8305ns | 9402ns | base | 3.769 |
| warm-container-kernel | 394ns | 389ns | 407ns | -95.47% | 83.147 |
| warm-container-lanes-deferred | 393ns | 390ns | 397ns | -95.48% | 83.373 |
| warm-container-minimum | 9294ns | 8351ns | 10082ns | +6.91% | 3.526 |
| warm-container-native | 429ns | 392ns | 515ns | -95.07% | 76.435 |
| warm-container-plusone | 8650ns | 8287ns | 9676ns | -0.50% | 3.788 |

## Performance model

- Peak throughput: **84.326 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 3.790 | 4.5% |
| warm-container-kernel | 83.667 | 99.2% |
| warm-container-lanes-deferred | 83.528 | 99.1% |
| warm-container-minimum | 3.493 | 4.1% |
| warm-container-native | 80.999 | 96.1% |
| warm-container-plusone | 3.933 | 4.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 8758ns | 8758ns | base |
| warm-container-kernel | 459ns | 459ns | -94.76% |
| warm-container-lanes-deferred | 456ns | 456ns | -94.79% |
| warm-container-minimum | 9369ns | 9369ns | +6.98% |
| warm-container-native | 497ns | 497ns | -94.33% |
| warm-container-plusone | 8716ns | 8716ns | -0.49% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 8646ns | base | --- | [8429, 8719] | --- | --- | --- | --- |
| warm-container-kernel | 392ns | -8252.5ns (-95.4%) | [-8327, -8025]ns | [390, 393] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 392ns | -8254.0ns (-95.5%) | [-8328, -8034]ns | [392, 394] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 9380ns | +745.0ns (+8.6%) | [+175, +1123]ns | [9045, 9535] | YES | 0.0080 | 0.0064 | 0 |
| warm-container-native | 405ns | -8184.0ns (-94.7%) | [-8205, -8030]ns | [397, 410] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 8332ns | no significant difference | [-302, +8]ns | [8302, 8462] | no | 0.1539 | 0.1539 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 8718ns | -95.5% | -95.5% | +15.1% | -94.1% | -4.9% |
| 2 | 8718ns | -95.4% | -95.5% | +15.1% | -94.1% | -4.9% |
| 3 | 8718ns | -95.5% | -95.5% | +15.1% | -94.1% | -5.0% |
| 4 | 8719ns | -95.5% | -95.5% | +15.1% | -94.1% | -4.1% |
| 5 | 8785ns | -95.6% | -95.5% | +15.6% | -94.2% | -5.6% |
| 6 | 8718ns | -95.5% | -95.5% | +18.1% | -94.1% | -4.5% |
| 7 | 8766ns | -95.5% | -95.5% | +9.3% | -94.1% | -5.4% |
| 8 | 8718ns | -95.5% | -95.5% | +8.7% | -94.1% | -4.4% |
| 9 | 8719ns | -95.4% | -95.5% | +7.6% | -94.1% | -4.8% |
| 10 | 8735ns | -95.4% | -95.5% | +8.6% | -94.1% | -4.5% |
| 11 | 8900ns | -95.6% | -95.6% | -2.0% | -95.4% | +13.1% |
| 12 | 8790ns | -95.5% | -95.5% | -2.3% | -95.3% | +14.9% |
| 13 | 8831ns | -95.6% | -95.6% | -1.1% | -95.3% | +14.6% |
| 14 | 8523ns | -95.4% | -95.4% | +4.3% | -95.2% | +1.0% |
| 15 | 8575ns | -95.4% | -95.5% | +4.2% | -95.2% | -2.8% |
| 16 | 8526ns | -95.4% | -95.4% | +3.1% | -95.2% | -2.8% |
| 17 | 8339ns | -95.3% | -95.3% | +10.5% | -95.1% | +2.0% |
| 18 | 8312ns | -95.3% | -95.2% | +20.5% | -95.1% | +7.0% |
| 19 | 8315ns | -95.1% | -95.3% | +15.4% | -95.1% | +13.0% |
| 20 | 8332ns | -95.3% | -95.3% | +8.9% | -95.1% | +12.8% |
| 21 | 8428ns | -95.3% | -95.3% | -1.0% | -95.2% | -0.8% |
| 22 | 8430ns | -95.4% | -95.3% | -1.9% | -95.3% | -1.2% |
| 23 | 8323ns | -95.3% | -95.3% | +0.4% | -95.2% | +1.1% |
| 24 | 8355ns | -95.3% | -95.3% | +19.5% | -95.3% | -0.8% |
| 25 | 8305ns | -95.3% | -95.3% | +20.6% | -95.2% | -0.2% |
| 26 | 8281ns | -95.3% | -95.2% | +13.3% | -95.2% | +0.1% |
| 27 | 8282ns | -95.2% | -95.2% | +13.8% | -95.2% | +0.1% |
| 28 | 8335ns | -95.3% | -95.3% | +11.1% | -95.2% | -0.2% |
| 29 | 8288ns | -95.3% | -95.2% | +0.1% | -95.2% | +0.3% |
| 30 | 8333ns | -95.3% | -95.2% | +1.0% | -95.2% | -0.1% |
| 31 | 8456ns | -94.8% | -95.4% | +18.8% | -95.4% | -2.0% |
| 32 | 8791ns | -95.2% | -95.5% | +14.1% | -95.5% | -5.7% |
| 33 | 8500ns | -95.4% | -95.4% | +18.0% | -95.4% | +0.3% |
| 34 | 8380ns | -95.3% | -95.3% | +7.6% | -95.3% | +2.2% |
| 35 | 9547ns | -95.9% | -95.9% | -13.4% | -95.9% | -1.6% |
| 36 | 10090ns | -96.2% | -96.1% | -17.9% | -96.1% | -6.8% |
| 37 | 9563ns | -95.9% | -95.9% | -4.4% | -95.9% | -0.4% |
| 38 | 9394ns | -95.8% | -95.8% | -0.1% | -95.8% | +0.1% |
| 39 | 9396ns | -95.9% | -95.8% | -0.2% | -95.8% | -11.8% |
| 40 | 9495ns | -95.9% | -95.9% | -1.2% | -95.9% | -12.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.789 | HIGH+ (drift/warm-up) |
| warm-container-kernel | 0.357 | moderate+ |
| warm-container-lanes-deferred | 0.208 | moderate+ |
| warm-container-minimum | 0.602 | HIGH+ (drift/warm-up) |
| warm-container-native | 0.902 | HIGH+ (drift/warm-up) |
| warm-container-plusone | 0.645 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-lanes-deferred**: won 40/40, lost 0/40
- **warm-container-minimum**: won 11/40, lost 29/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 24/40, lost 14/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 2.6ns | 8693.2ns | 0.0% |  |
| warm-container-kernel | 2.1ns | 394.1ns | 0.5% |  |
| warm-container-lanes-deferred | 2.1ns | 393.0ns | 0.5% |  |
| warm-container-minimum | 3.2ns | 9294.2ns | 0.0% |  |
| warm-container-native | 2.8ns | 428.7ns | 0.7% |  |
| warm-container-plusone | 2.4ns | 8650.0ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 8304.9-9402.1 ns)
   8304.9 |########################################
   8359.7 |#####
   8414.6 |###############
   8469.5 |##########
   8524.3 |##########
   8579.2 |
   8634.0 |
   8688.9 |########################################
   8743.8 |####################
   8798.6 |#####
   8853.5 |#####
   8908.3 |
   8963.2 |
   9018.1 |
   9072.9 |
   9127.8 |
   9182.6 |
   9237.5 |
   9292.4 |
   9347.2 |##########
  (4 below, 4 above range)

warm-container-kernel (n=40, range 388.6-406.8 ns)
    388.6 |########
    389.5 |########################################
    390.4 |######################
    391.3 |#################
    392.2 |#################
    393.1 |#############
    394.1 |####
    395.0 |####
    395.9 |####
    396.8 |
    397.7 |####
    398.6 |####
    399.5 |####
    400.4 |
    401.4 |
    402.3 |
    403.2 |####
    404.1 |
    405.0 |
    405.9 |
  (4 below, 2 above range)

warm-container-lanes-deferred (n=40, range 390.0-397.3 ns)
    390.0 |########################
    390.3 |################################
    390.7 |################
    391.1 |########
    391.4 |########################################
    391.8 |################
    392.2 |########
    392.5 |########
    392.9 |
    393.3 |################
    393.6 |########################
    394.0 |########
    394.4 |################
    394.8 |################
    395.1 |########
    395.5 |
    395.9 |########
    396.2 |
    396.6 |
    397.0 |################
  (3 below, 4 above range)

warm-container-minimum (n=40, range 8351.4-10082.5 ns)
   8351.4 |########
   8437.9 |
   8524.5 |####
   8611.0 |
   8697.6 |########
   8784.1 |####
   8870.7 |########
   8957.3 |####
   9043.8 |####
   9130.4 |########
   9216.9 |####
   9303.5 |######################
   9390.0 |####
   9476.6 |########
   9563.2 |########
   9649.7 |
   9736.3 |
   9822.8 |
   9909.4 |####
   9995.9 |########################################
  (5 below, 2 above range)

warm-container-native (n=40, range 392.2-514.9 ns)
    392.2 |########################################
    398.3 |######
    404.4 |#####################
    410.6 |#########
    416.7 |
    422.9 |
    429.0 |
    435.1 |
    441.3 |
    447.4 |
    453.5 |
    459.7 |
    465.8 |
    471.9 |
    478.1 |
    484.2 |
    490.4 |
    496.5 |
    502.6 |
    508.8 |##################
  (5 below, 4 above range)

warm-container-plusone (n=40, range 8286.6-9676.3 ns)
   8286.6 |########################################
   8356.0 |######
   8425.5 |
   8495.0 |######
   8564.5 |##
   8634.0 |
   8703.5 |
   8773.0 |
   8842.5 |##
   8911.9 |
   8981.4 |
   9050.9 |
   9120.4 |
   9189.9 |
   9259.4 |
   9328.9 |####
   9398.4 |######
   9467.8 |##
   9537.3 |
   9606.8 |
  (4 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: autocorrelation=0.79 (measurement drift or warm-up artifact)
- **warm-container-minimum**: autocorrelation=0.60 (measurement drift or warm-up artifact)
- **warm-container-native**: autocorrelation=0.90 (measurement drift or warm-up artifact)
- **warm-container-plusone**: autocorrelation=0.64 (measurement drift or warm-up artifact)
