# Container fork under saturating semantics, declared-width sweep (8192 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-kernel dominates: 205053% faster than the next best (warm-container-plusone)

warm-container-kernel (4 ns) leads warm-container-plusone (8.10 us) by 205053%, a clear separation rather than a photo finish. CV 27.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-kernel beats baseline by 97% (significant)

warm-container-kernel is -8.03 us (97%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-minimum is an outlier: 2786.7x slower than the field

warm-container-minimum (11.01 us) is 2786.7x the fastest (4 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-kernel is fastest but the noisiest (CV 27.3%)

warm-container-kernel wins on median (4 ns) yet has the highest variance (CV 27.3%), while warm-container-plusone is the steadiest (CV 2.8%, 8.10 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-container-plusone shows warm-up / thermal drift (autocorr +0.69)

warm-container-plusone's per-pass series has lag-1 autocorrelation +0.69, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-kernel} vs {warm-container-plusone, warm-container-headroom, warm-container-native, warm-container-minimum} (205053% apart)

The field splits into a fast tier {warm-container-kernel} and a slow tier {warm-container-plusone, warm-container-headroom, warm-container-native, warm-container-minimum} with a 205053% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 2786.7x the fastest

Fastest warm-container-kernel (4 ns) to slowest warm-container-minimum (11.01 us): 2786.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-kernel is inconsistent: worst-20% is 2.1x its best-20%

warm-container-kernel's best 20% of batches run at 3 ns but its worst 20% at 6 ns (2.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 4.0 ns median (-100.0% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2786.66x (fastest 4.0 ns, slowest 11007.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 8638ns | 8377ns | 8005ns | 8508ns | 9663ns | base |
| warm-container-kernel | 65ns | 65ns | 61ns | 65ns | 68ns | -99.25% |
| warm-container-minimum | 11286ns | 11077ns | 10652ns | 11041ns | 12654ns | +30.65% |
| warm-container-native | 11697ns | 11057ns | 10536ns | 11028ns | 14868ns | +35.41% |
| warm-container-plusone | 8215ns | 8168ns | 7946ns | 8197ns | 8538ns | -4.90% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 8557ns | 7941ns | 9554ns | base | 3.829 |
| warm-container-kernel | 4ns | 3ns | 6ns | -99.95% | 8212.531 |
| warm-container-minimum | 11195ns | 10576ns | 12505ns | +30.82% | 2.927 |
| warm-container-native | 11592ns | 10470ns | 14677ns | +35.46% | 2.827 |
| warm-container-plusone | 8149ns | 7884ns | 8461ns | -4.77% | 4.021 |

## Performance model

- Peak throughput: **12603.077 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 3.945 | 0.0% |
| warm-container-kernel | 8295.696 | 65.8% |
| warm-container-minimum | 2.977 | 0.0% |
| warm-container-native | 2.986 | 0.0% |
| warm-container-plusone | 4.044 | 0.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 8638ns | 8638ns | base |
| warm-container-kernel | 65ns | 65ns | -99.25% |
| warm-container-minimum | 11286ns | 11286ns | +30.65% |
| warm-container-native | 11697ns | 11697ns | +35.41% |
| warm-container-plusone | 8215ns | 8215ns | -4.90% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 8307ns | base | --- | [8272, 8573] | --- | --- | --- | --- |
| warm-container-kernel | 4ns | -8302.3ns (-99.9%) | [-8569, -8269]ns | [3, 4] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 11007ns | +2694.6ns (+32.4%) | [+2564, +2793]ns | [10748, 11035] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 10973ns | +2572.7ns (+31.0%) | [+2400, +2881]ns | [10639, 11093] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 8104ns | no significant difference | [-486, +34]ns | [8019, 8308] | no | 0.4296 | 0.4296 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|
| 1 | 8280ns | -99.9% | +28.1% | +159.6% | +0.4% |
| 2 | 8293ns | -99.9% | +28.2% | +32.8% | +0.1% |
| 3 | 8285ns | -99.9% | +30.2% | +34.2% | +1.2% |
| 4 | 8617ns | -99.9% | +40.6% | +27.7% | -3.6% |
| 5 | 8475ns | -99.9% | +30.2% | +33.7% | +0.6% |
| 6 | 8192ns | -100.0% | +34.4% | +61.7% | +4.0% |
| 7 | 9126ns | -100.0% | +18.4% | +71.1% | -5.0% |
| 8 | 8245ns | -100.0% | +33.5% | +50.4% | -1.2% |
| 9 | 8093ns | -99.9% | +35.8% | +35.9% | +0.1% |
| 10 | 8045ns | -100.0% | +36.8% | +38.1% | -0.1% |
| 11 | 7954ns | -99.9% | +34.1% | +32.2% | +1.1% |
| 12 | 8600ns | -100.0% | +23.5% | +21.5% | -5.6% |
| 13 | 9607ns | -100.0% | +10.2% | +8.7% | -16.6% |
| 14 | 9540ns | -99.9% | +10.8% | +10.3% | -16.9% |
| 15 | 9513ns | -100.0% | +11.5% | +11.4% | -16.1% |
| 16 | 7955ns | -100.0% | +32.3% | +37.9% | -1.0% |
| 17 | 9336ns | -100.0% | +13.8% | +15.1% | -14.0% |
| 18 | 8320ns | -100.0% | +27.6% | +26.4% | -4.9% |
| 19 | 7912ns | -100.0% | +33.6% | +32.7% | +0.7% |
| 20 | 7947ns | -100.0% | +32.3% | +31.5% | +1.2% |
| 21 | 10228ns | -99.9% | +22.3% | +7.3% | -22.8% |
| 22 | 9329ns | -100.0% | +39.6% | +17.8% | -15.6% |
| 23 | 8942ns | -100.0% | +31.5% | +23.8% | -9.3% |
| 24 | 8288ns | -100.0% | +40.5% | +35.1% | -3.2% |
| 25 | 8380ns | -100.0% | +48.0% | +44.7% | -5.9% |
| 26 | 8501ns | -99.9% | +36.9% | +36.8% | -7.1% |
| 27 | 8545ns | -100.0% | +31.2% | +43.8% | -7.8% |
| 28 | 9580ns | -100.0% | +20.5% | +62.4% | -17.6% |
| 29 | 9178ns | -99.9% | +40.9% | +47.5% | -13.3% |
| 30 | 8722ns | -100.0% | +56.8% | +52.3% | -9.7% |
| 31 | 7870ns | -100.0% | +39.8% | +36.0% | +9.4% |
| 32 | 7895ns | -99.9% | +39.5% | +35.5% | +5.2% |
| 33 | 8266ns | -100.0% | +33.1% | +29.2% | +0.6% |
| 34 | 8330ns | -99.9% | +32.2% | +27.3% | -0.1% |
| 35 | 8279ns | -99.9% | +33.5% | +27.8% | +0.4% |
| 36 | 9302ns | -99.9% | +18.5% | +15.6% | -10.6% |
| 37 | 8288ns | -100.0% | +33.5% | +26.2% | +0.6% |
| 38 | 8038ns | -99.9% | +37.3% | +31.5% | +3.3% |
| 39 | 7982ns | -99.9% | +34.2% | +30.7% | +4.1% |
| 40 | 8013ns | -100.0% | +33.0% | +31.2% | +3.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.284 | moderate+ |
| warm-container-kernel | -0.048 | ok |
| warm-container-minimum | 0.545 | HIGH+ (drift/warm-up) |
| warm-container-native | 0.223 | moderate+ |
| warm-container-plusone | 0.686 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-minimum**: won 0/40, lost 40/40
- **warm-container-native**: won 0/40, lost 40/40
- **warm-container-plusone**: won 22/40, lost 15/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 3.9ns | 8557.3ns | 0.0% |  |
| warm-container-kernel | 2.2ns | 4.0ns | 56.4% | HIGH |
| warm-container-minimum | 3.8ns | 11194.6ns | 0.0% |  |
| warm-container-native | 4.8ns | 11592.1ns | 0.0% |  |
| warm-container-plusone | 2.9ns | 8149.1ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 7941.2-9554.3 ns)
   7941.2 |######################
   8021.8 |#############
   8102.5 |
   8183.1 |########
   8263.8 |########################################
   8344.4 |####
   8425.1 |########
   8505.8 |####
   8586.4 |########
   8667.1 |####
   8747.7 |
   8828.4 |
   8909.0 |####
   8989.7 |
   9070.4 |####
   9151.0 |####
   9231.7 |####
   9312.3 |########
   9393.0 |
   9473.7 |########
  (3 below, 3 above range)

warm-container-kernel (n=40, range 2.6-5.6 ns)
      2.6 |
      2.7 |
      2.9 |#############
      3.0 |
      3.2 |########################################
      3.3 |
      3.5 |
      3.6 |######################
      3.8 |
      3.9 |
      4.1 |###################################
      4.2 |
      4.4 |
      4.5 |########
      4.7 |
      4.8 |
      5.0 |#################
      5.1 |
      5.3 |#############
      5.4 |
  (3 below, 3 above range)

warm-container-minimum (n=40, range 10575.6-12504.9 ns)
  10575.6 |##############################
  10672.1 |###
  10768.5 |######
  10865.0 |
  10961.5 |########################################
  11057.9 |###
  11154.4 |###
  11250.9 |
  11347.3 |
  11443.8 |###
  11540.3 |
  11636.7 |######
  11733.2 |###
  11829.7 |
  11926.1 |
  12022.6 |###
  12119.1 |
  12215.5 |
  12312.0 |###
  12408.5 |
  (4 below, 4 above range)

warm-container-native (n=40, range 10470.2-14677.1 ns)
  10470.2 |########################################
  10680.6 |################
  10890.9 |############################
  11101.2 |############
  11311.6 |####
  11521.9 |####
  11732.3 |
  11942.6 |####
  12153.0 |####
  12363.3 |####
  12573.7 |
  12784.0 |
  12994.3 |
  13204.7 |########
  13415.0 |####
  13625.4 |
  13835.7 |
  14046.1 |
  14256.4 |
  14466.8 |
  (5 below, 3 above range)

warm-container-plusone (n=40, range 7884.1-8461.4 ns)
   7884.1 |#################
   7912.9 |########
   7941.8 |########
   7970.7 |####
   7999.5 |#############
   8028.4 |#############
   8057.3 |
   8086.1 |########
   8115.0 |####
   8143.8 |####
   8172.7 |
   8201.6 |
   8230.4 |
   8259.3 |
   8288.2 |########################################
   8317.0 |#############
   8345.9 |
   8374.8 |####
   8403.6 |
   8432.5 |
  (4 below, 4 above range)

```

## Diagnostics

- **warm-container-kernel**: CV=27.0% (high variance, measurements may be unstable)
- **warm-container-kernel**: bridge=58.2% of algo (FFI overhead may distort results)
- **warm-container-minimum**: autocorrelation=0.55 (measurement drift or warm-up artifact)
- **warm-container-plusone**: autocorrelation=0.69 (measurement drift or warm-up artifact)
