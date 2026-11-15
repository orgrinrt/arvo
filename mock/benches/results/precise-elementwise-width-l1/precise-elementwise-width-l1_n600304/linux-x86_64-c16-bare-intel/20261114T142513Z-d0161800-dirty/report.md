# Container fork under saturating semantics, elementwise, declared-width sweep (8192 elements, 4 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-native beats baseline by 55% (significant)

warm-container-native is -4.58 us (55%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-kernel is an outlier: 2.8x slower than the field

warm-container-kernel (10.72 us) is 2.8x the fastest (3.77 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-container-native, warm-container-plusone) are a dead heat (<1%)

warm-container-native (3.77 us) and warm-container-plusone (3.78 us) differ by 0.39%, inside the noise, even though the wider field spreads 184.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-kernel shows alternating (throttle bounce) (autocorr -0.80)

warm-container-kernel's per-pass series has lag-1 autocorrelation -0.80, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-native, warm-container-plusone, warm-container-minimum} vs {warm-container-headroom, warm-container-kernel} (120% apart)

The field splits into a fast tier {warm-container-native, warm-container-plusone, warm-container-minimum} and a slow tier {warm-container-headroom, warm-container-kernel} with a 120% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-container-minimum is inconsistent: worst-20% is 2.8x its best-20%

warm-container-minimum's best 20% of batches run at 3.77 us but its worst 20% at 10.51 us (2.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-native** at 3769.9 ns median (-54.8% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.84x (fastest 3769.9 ns, slowest 10716.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 8436ns | 8432ns | 8420ns | 8432ns | 8466ns | base |
| warm-container-kernel | 10804ns | 10800ns | 10793ns | 10800ns | 10829ns | +28.07% |
| warm-container-minimum | 5252ns | 3868ns | 3849ns | 3872ns | 10795ns | -37.75% |
| warm-container-native | 4444ns | 3851ns | 3159ns | 3856ns | 7493ns | -47.33% |
| warm-container-plusone | 5030ns | 3868ns | 3853ns | 3870ns | 9687ns | -40.38% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 8345ns | 8335ns | 8353ns | base | 4.909 |
| warm-container-kernel | 10717ns | 10707ns | 10726ns | +28.43% | 3.822 |
| warm-container-minimum | 5129ns | 3767ns | 10506ns | -38.54% | 7.986 |
| warm-container-native | 4337ns | 3092ns | 7272ns | -48.02% | 9.444 |
| warm-container-plusone | 4913ns | 3771ns | 9431ns | -41.13% | 8.338 |

## Performance model

- Peak throughput: **13.248 Gops/s** (warm-container-native; best 20% batches)
- Ops per call: 40960

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 4.908 | 37.0% |
| warm-container-kernel | 3.822 | 28.9% |
| warm-container-minimum | 10.818 | 81.7% |
| warm-container-native | 10.865 | 82.0% |
| warm-container-plusone | 10.823 | 81.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 8436ns | 8436ns | base |
| warm-container-kernel | 10804ns | 10804ns | +28.07% |
| warm-container-minimum | 5252ns | 5252ns | -37.75% |
| warm-container-native | 4444ns | 4444ns | -47.33% |
| warm-container-plusone | 5030ns | 5030ns | -40.38% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 8345ns | base | --- | [8343, 8348] | --- | --- | --- | --- |
| warm-container-kernel | 10716ns | +2372.9ns (+28.4%) | [+2368, +2377]ns | [10715, 10719] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 3786ns | -4554.4ns (-54.6%) | [-4567, -4539]ns | [3780, 3804] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 3770ns | -4575.3ns (-54.8%) | [-4582, -4557]ns | [3758, 3790] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 3785ns | -4558.9ns (-54.6%) | [-4569, -4543]ns | [3778, 3795] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 8346ns | +28.4% | -54.8% | -62.9% | -54.7% |
| 1 | 2 | 8348ns | +28.3% | -54.7% | -54.9% | -54.7% |
| 1 | 3 | 8342ns | +28.5% | -54.4% | -54.8% | -54.5% |
| 1 | 4 | 8346ns | +28.4% | -54.4% | -54.5% | -54.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.482 | moderate- |
| warm-container-kernel | -0.802 | HIGH- (thermal bounce) |
| warm-container-minimum | 0.276 | moderate+ |
| warm-container-native | -0.067 | ok |
| warm-container-plusone | -0.419 | moderate- |

**Consistency summary:**

- **warm-container-kernel**: won 0/4, lost 4/4
- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 25.1ns | 8344.6ns | 0.3% |  |
| warm-container-kernel | 24.7ns | 10716.6ns | 0.2% |  |
| warm-container-minimum | 33.6ns | 5129.0ns | 0.7% |  |
| warm-container-native | 28.5ns | 4337.2ns | 0.7% |  |
| warm-container-plusone | 32.0ns | 4912.6ns | 0.7% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 8334.5-8352.7 ns)
   8334.5 |################
   8335.4 |
   8336.3 |########
   8337.2 |
   8338.2 |########
   8339.1 |########
   8340.0 |########
   8340.9 |########################
   8341.8 |################
   8342.7 |########################
   8343.6 |################
   8344.5 |################
   8345.4 |################
   8346.4 |################
   8347.3 |
   8348.2 |########################################
   8349.1 |
   8350.0 |########################
   8350.9 |################
   8351.8 |########
  (3 below, 4 above range)

warm-container-kernel (n=40, range 10707.3-10725.5 ns)
  10707.3 |########
  10708.2 |################
  10709.1 |
  10710.0 |########
  10710.9 |################
  10711.8 |########
  10712.7 |################
  10713.7 |########################
  10714.6 |########################################
  10715.5 |########################
  10716.4 |################
  10717.3 |########
  10718.2 |################
  10719.1 |########################
  10720.0 |################
  10721.0 |################################
  10721.9 |
  10722.8 |
  10723.7 |
  10724.6 |########
  (1 below, 4 above range)

warm-container-minimum (n=40, range 3767.0-10506.0 ns)
   3767.0 |########################################
   4104.0 |
   4440.9 |
   4777.9 |
   5114.8 |#
   5451.8 |
   5788.7 |
   6125.7 |
   6462.6 |
   6799.5 |
   7136.5 |
   7473.4 |
   7810.4 |
   8147.3 |
   8484.3 |
   8821.2 |
   9158.2 |
   9495.1 |
   9832.1 |#
  10169.0 |#
  (5 below, 5 above range)

warm-container-native (n=40, range 3091.7-7272.5 ns)
   3091.7 |#######
   3300.8 |
   3509.8 |
   3718.8 |########################################
   3927.9 |
   4136.9 |
   4346.0 |
   4555.0 |
   4764.0 |
   4973.1 |
   5182.1 |#
   5391.1 |
   5600.2 |
   5809.2 |
   6018.3 |
   6227.3 |#
   6436.3 |
   6645.4 |
   6854.4 |
   7063.4 |
  (3 below, 4 above range)

warm-container-plusone (n=40, range 3770.5-9431.0 ns)
   3770.5 |########################################
   4053.5 |#
   4336.6 |
   4619.6 |
   4902.6 |
   5185.6 |
   5468.7 |
   5751.7 |
   6034.7 |
   6317.7 |
   6600.8 |
   6883.8 |
   7166.8 |
   7449.8 |
   7732.9 |
   8015.9 |
   8298.9 |
   8581.9 |
   8865.0 |
   9148.0 |#
  (4 below, 5 above range)

```

## Diagnostics

- **warm-container-minimum**: CV=55.8% (high variance, measurements may be unstable)
- **warm-container-native**: CV=44.3% (high variance, measurements may be unstable)
- **warm-container-plusone**: CV=54.5% (high variance, measurements may be unstable)
