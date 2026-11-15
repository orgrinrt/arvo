# Affine-only wrapping reduction at 13 bits, operation-density swept: how much of the deferred form's advantage is the optimiser collapsing the chain rather than the mask being gone

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-kernel beats baseline by 95% (significant)

warm-container-kernel is -8.07 us (95%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-plusone is an outlier: 21.8x slower than the field

warm-container-plusone (8.54 us) is 21.8x the fastest (392 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-container-kernel, warm-container-lanes-deferred) are a dead heat (<1%)

warm-container-kernel (392 ns) and warm-container-lanes-deferred (396 ns) differ by 0.79%, inside the noise, even though the wider field spreads 2077.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-native shows warm-up / thermal drift (autocorr +0.87)

warm-container-native's per-pass series has lag-1 autocorrelation +0.87, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-kernel, warm-container-lanes-deferred, warm-container-native} vs {warm-container-minimum, warm-container-headroom, warm-container-plusone} (1940% apart)

The field splits into a fast tier {warm-container-kernel, warm-container-lanes-deferred, warm-container-native} and a slow tier {warm-container-minimum, warm-container-headroom, warm-container-plusone} with a 1940% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 21.8x the fastest

Fastest warm-container-kernel (392 ns) to slowest warm-container-plusone (8.54 us): 21.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader warm-container-kernel vs stability leader warm-container-lanes-deferred (+1% speed for 1.6x steadier)

warm-container-kernel is fastest (392 ns, CV 5.0%); warm-container-lanes-deferred gives up 0.8% median for 1.6x lower variance (CV 3.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: warm-container-kernel** at 392.5 ns median (-95.4% vs baseline)
- 4 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 21.77x (fastest 392.5 ns, slowest 8544.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 8804ns | 8595ns | 8344ns | 8631ns | 9785ns | base |
| warm-container-kernel | 459ns | 454ns | 451ns | 455ns | 478ns | -94.79% |
| warm-container-lanes-deferred | 466ns | 458ns | 452ns | 461ns | 493ns | -94.71% |
| warm-container-minimum | 8514ns | 8430ns | 8346ns | 8442ns | 8899ns | -3.30% |
| warm-container-native | 484ns | 475ns | 451ns | 474ns | 550ns | -94.50% |
| warm-container-plusone | 8896ns | 8617ns | 8366ns | 8747ns | 9871ns | +1.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 8733ns | 8282ns | 9715ns | base | 4.690 |
| warm-container-kernel | 397ns | 389ns | 417ns | -95.45% | 103.135 |
| warm-container-lanes-deferred | 401ns | 390ns | 419ns | -95.41% | 102.222 |
| warm-container-minimum | 8447ns | 8287ns | 8825ns | -3.27% | 4.849 |
| warm-container-native | 418ns | 389ns | 474ns | -95.22% | 98.106 |
| warm-container-plusone | 8826ns | 8305ns | 9795ns | +1.07% | 4.641 |

## Performance model

- Peak throughput: **105.418 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 4.811 | 4.6% |
| warm-container-kernel | 104.357 | 99.0% |
| warm-container-lanes-deferred | 103.539 | 98.2% |
| warm-container-minimum | 4.897 | 4.6% |
| warm-container-native | 99.902 | 94.8% |
| warm-container-plusone | 4.794 | 4.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 8804ns | 8804ns | base |
| warm-container-kernel | 459ns | 459ns | -94.79% |
| warm-container-lanes-deferred | 466ns | 466ns | -94.71% |
| warm-container-minimum | 8514ns | 8514ns | -3.30% |
| warm-container-native | 484ns | 484ns | -94.50% |
| warm-container-plusone | 8896ns | 8896ns | +1.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 8514ns | base | --- | [8403, 8579] | --- | --- | --- | --- |
| warm-container-kernel | 392ns | -8113.4ns (-95.3%) | [-8182, -8013]ns | [391, 396] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 396ns | -8100.9ns (-95.1%) | [-8176, -8010]ns | [393, 400] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 8365ns | -78.4ns (-0.9%) | [-183, -30]ns | [8319, 8431] | YES | 0.0008 | 0.0007 | 0 |
| warm-container-native | 410ns | -8099.0ns (-95.1%) | [-8142, -8015]ns | [392, 413] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 8545ns | +221.1ns (+2.6%) | [+4, +395]ns | [8413, 8881] | YES | 0.0385 | 0.0385 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 8279ns | -95.3% | -95.3% | +20.2% | -94.3% | +0.1% |
| 2 | 9527ns | -95.8% | -95.9% | -8.5% | -95.0% | -12.4% |
| 3 | 10047ns | -96.1% | -96.1% | -13.2% | -95.3% | -17.2% |
| 4 | 10045ns | -96.1% | -96.1% | -10.7% | -95.3% | -17.5% |
| 5 | 9526ns | -95.9% | -95.3% | -9.8% | -95.0% | -12.0% |
| 6 | 9512ns | -95.9% | -95.9% | -9.7% | -95.0% | -12.9% |
| 7 | 10075ns | -96.1% | -96.1% | -17.3% | -95.3% | -16.9% |
| 8 | 9558ns | -95.9% | -95.9% | -13.2% | -95.1% | -10.8% |
| 9 | 8748ns | -95.6% | -95.5% | -2.1% | -94.6% | -3.3% |
| 10 | 8599ns | -95.4% | -95.5% | -3.2% | -94.5% | -0.7% |
| 11 | 8366ns | -95.3% | -95.3% | -0.4% | -95.3% | +12.7% |
| 12 | 9390ns | -95.9% | -95.8% | -11.7% | -95.9% | +7.0% |
| 13 | 8280ns | -95.1% | -95.3% | +0.1% | -95.3% | +9.0% |
| 14 | 8282ns | -95.3% | -95.3% | +0.0% | -95.3% | +1.6% |
| 15 | 8282ns | -95.3% | -95.2% | +0.1% | -95.3% | +10.5% |
| 16 | 8402ns | -95.3% | -95.4% | -1.4% | -95.4% | +19.7% |
| 17 | 9428ns | -95.8% | -95.8% | -12.1% | -95.9% | +7.7% |
| 18 | 8821ns | -95.6% | -95.6% | -5.8% | -95.6% | +14.6% |
| 19 | 8284ns | -95.2% | -95.3% | +0.4% | -95.2% | +19.4% |
| 20 | 8405ns | -95.4% | -95.3% | -0.9% | -95.3% | +12.8% |
| 21 | 8617ns | -95.4% | -95.4% | -2.8% | -95.2% | -3.1% |
| 22 | 9032ns | -95.7% | -95.6% | -7.3% | -95.7% | -7.1% |
| 23 | 8490ns | -95.4% | -95.3% | -2.2% | -95.4% | -0.4% |
| 24 | 8335ns | -95.3% | -95.2% | -0.2% | -95.3% | +2.1% |
| 25 | 8282ns | -95.3% | -95.2% | +0.0% | -95.3% | +1.5% |
| 26 | 8287ns | -95.3% | -95.2% | +1.0% | -95.3% | +1.3% |
| 27 | 8283ns | -95.3% | -95.2% | +0.3% | -95.3% | +3.2% |
| 28 | 8306ns | -95.3% | -95.0% | -0.2% | -95.3% | +0.4% |
| 29 | 8282ns | -93.8% | -95.2% | +0.8% | -95.2% | +0.0% |
| 30 | 8365ns | -95.2% | -95.3% | -0.1% | -95.3% | -0.8% |
| 31 | 8528ns | -95.3% | -95.1% | -1.2% | -95.2% | +3.4% |
| 32 | 8548ns | -95.3% | -95.2% | -1.2% | -95.2% | +5.9% |
| 33 | 8469ns | -95.3% | -95.1% | -0.1% | -95.1% | +5.7% |
| 34 | 8502ns | -95.3% | -95.2% | -0.8% | -95.1% | +8.1% |
| 35 | 8558ns | -95.4% | -95.2% | -1.5% | -95.2% | +6.9% |
| 36 | 8514ns | -95.3% | -95.1% | -1.0% | -95.1% | +4.1% |
| 37 | 8550ns | -95.3% | -95.2% | -0.7% | -95.2% | +3.6% |
| 38 | 8514ns | -95.3% | -95.1% | -0.7% | -95.1% | +4.0% |
| 39 | 8457ns | -95.2% | -95.1% | -0.3% | -95.2% | +5.2% |
| 40 | 8536ns | -95.3% | -95.1% | -0.7% | -95.2% | +3.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.656 | HIGH+ (drift/warm-up) |
| warm-container-kernel | 0.006 | ok |
| warm-container-lanes-deferred | 0.312 | moderate+ |
| warm-container-minimum | 0.327 | moderate+ |
| warm-container-native | 0.869 | HIGH+ (drift/warm-up) |
| warm-container-plusone | 0.753 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-lanes-deferred**: won 40/40, lost 0/40
- **warm-container-minimum**: won 29/40, lost 6/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 13/40, lost 25/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 2.8ns | 8732.7ns | 0.0% |  |
| warm-container-kernel | 2.3ns | 397.1ns | 0.6% |  |
| warm-container-lanes-deferred | 2.4ns | 400.7ns | 0.6% |  |
| warm-container-minimum | 3.0ns | 8447.0ns | 0.0% |  |
| warm-container-native | 2.8ns | 417.5ns | 0.7% |  |
| warm-container-plusone | 2.9ns | 8826.2ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 8281.8-9714.7 ns)
   8281.8 |########################################
   8353.4 |####################
   8425.1 |###############
   8496.7 |########################################
   8568.4 |##########
   8640.0 |
   8711.7 |#####
   8783.3 |#####
   8854.9 |
   8926.6 |
   8998.2 |#####
   9069.9 |
   9141.5 |
   9213.2 |
   9284.8 |
   9356.5 |#####
   9428.1 |#####
   9499.8 |####################
   9571.4 |
   9643.0 |
  (3 below, 3 above range)

warm-container-kernel (n=40, range 388.5-416.7 ns)
    388.5 |########
    390.0 |########################################
    391.4 |######################
    392.8 |#################
    394.2 |####
    395.6 |#############
    397.0 |#############
    398.4 |####
    399.8 |########
    401.2 |########
    402.6 |########
    404.0 |####
    405.4 |
    406.8 |
    408.2 |
    409.6 |
    411.0 |
    412.4 |
    413.9 |
    415.3 |
  (4 below, 1 above range)

warm-container-lanes-deferred (n=40, range 390.0-418.6 ns)
    390.0 |#################################
    391.4 |########################################
    392.8 |#############
    394.3 |##########################
    395.7 |#############
    397.1 |######
    398.6 |####################
    400.0 |######
    401.4 |######
    402.9 |
    404.3 |
    405.7 |
    407.2 |
    408.6 |
    410.0 |
    411.5 |#################################
    412.9 |##########################
    414.3 |######
    415.8 |######
    417.2 |
  (3 below, 1 above range)

warm-container-minimum (n=40, range 8287.0-8824.9 ns)
   8287.0 |########################################
   8313.9 |##############################
   8340.8 |##########
   8367.7 |###############
   8394.5 |
   8421.4 |##############################
   8448.3 |##########
   8475.2 |##########
   8502.1 |
   8529.0 |
   8555.9 |#####
   8582.8 |##########
   8609.7 |
   8636.6 |
   8663.5 |
   8690.4 |#####
   8717.3 |#####
   8744.2 |
   8771.1 |
   8798.0 |
  (4 below, 2 above range)

warm-container-native (n=40, range 388.8-474.4 ns)
    388.8 |########################################
    393.1 |#########
    397.4 |
    401.6 |
    405.9 |###
    410.2 |########################
    414.5 |######
    418.7 |
    423.0 |
    427.3 |
    431.6 |
    435.9 |
    440.1 |
    444.4 |
    448.7 |
    453.0 |
    457.3 |
    461.5 |
    465.8 |
    470.1 |##################
  (3 below, 4 above range)

warm-container-plusone (n=40, range 8305.2-9794.6 ns)
   8305.2 |########################################
   8379.7 |########################################
   8454.2 |########################
   8528.6 |########################
   8603.1 |
   8677.6 |
   8752.0 |########
   8826.5 |########################################
   8901.0 |########
   8975.4 |########
   9049.9 |########
   9124.4 |########################
   9198.8 |
   9273.3 |
   9347.8 |
   9422.2 |################
   9496.7 |
   9571.2 |
   9645.6 |
   9720.1 |
  (5 below, 5 above range)

```

## Diagnostics

- **warm-container-headroom**: autocorrelation=0.66 (measurement drift or warm-up artifact)
- **warm-container-native**: autocorrelation=0.87 (measurement drift or warm-up artifact)
- **warm-container-plusone**: autocorrelation=0.75 (measurement drift or warm-up artifact)
