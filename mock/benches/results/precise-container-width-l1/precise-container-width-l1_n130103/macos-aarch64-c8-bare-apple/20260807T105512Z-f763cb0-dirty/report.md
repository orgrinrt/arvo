# Container fork under saturating semantics, declared-width sweep (8192 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-kernel dominates: 190167% faster than the next best (warm-container-headroom)

warm-container-kernel (4 ns) leads warm-container-headroom (7.99 us) by 190167%, a clear separation rather than a photo finish. CV 23.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-kernel beats baseline by 98% (significant)

warm-container-kernel is -7.85 us (98%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-minimum is an outlier: 2534.9x slower than the field

warm-container-minimum (10.65 us) is 2534.9x the fastest (4 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-kernel is fastest but the noisiest (CV 23.2%)

warm-container-kernel wins on median (4 ns) yet has the highest variance (CV 23.2%), while warm-container-minimum is the steadiest (CV 1.5%, 10.65 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### warm-container-native shows warm-up / thermal drift (autocorr +0.75)

warm-container-native's per-pass series has lag-1 autocorrelation +0.75, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-kernel} vs {warm-container-headroom, warm-container-plusone, warm-container-native, warm-container-minimum} (190167% apart)

The field splits into a fast tier {warm-container-kernel} and a slow tier {warm-container-headroom, warm-container-plusone, warm-container-native, warm-container-minimum} with a 190167% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 2534.9x the fastest

Fastest warm-container-kernel (4 ns) to slowest warm-container-minimum (10.65 us): 2534.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-kernel is inconsistent: worst-20% is 2.0x its best-20%

warm-container-kernel's best 20% of batches run at 3 ns but its worst 20% at 5 ns (2.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 4.2 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 2534.87x (fastest 4.2 ns, slowest 10646.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 8096ns | 8048ns | 7920ns | 8059ns | 8384ns | base |
| warm-container-kernel | 64ns | 63ns | 61ns | 64ns | 68ns | -99.21% |
| warm-container-minimum | 10717ns | 10717ns | 10504ns | 10708ns | 10955ns | +32.37% |
| warm-container-native | 10792ns | 10713ns | 10510ns | 10748ns | 11209ns | +33.31% |
| warm-container-plusone | 10737ns | 10702ns | 10518ns | 10691ns | 11094ns | +32.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 8031ns | 7859ns | 8311ns | base | 4.080 |
| warm-container-kernel | 4ns | 3ns | 5ns | -99.95% | 8248.710 |
| warm-container-minimum | 10645ns | 10439ns | 10872ns | +32.54% | 3.078 |
| warm-container-native | 10719ns | 10444ns | 11122ns | +33.46% | 3.057 |
| warm-container-plusone | 10665ns | 10450ns | 11022ns | +32.79% | 3.072 |

## Performance model

- Peak throughput: **12365.283 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 4.101 | 0.0% |
| warm-container-kernel | 7801.905 | 63.1% |
| warm-container-minimum | 3.078 | 0.0% |
| warm-container-native | 3.079 | 0.0% |
| warm-container-plusone | 3.082 | 0.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 8096ns | 8096ns | base |
| warm-container-kernel | 64ns | 64ns | -99.21% |
| warm-container-minimum | 10717ns | 10717ns | +32.37% |
| warm-container-native | 10792ns | 10792ns | +33.31% |
| warm-container-plusone | 10737ns | 10737ns | +32.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 7991ns | base | --- | [7932, 8049] | --- | --- | --- | --- |
| warm-container-kernel | 4ns | -7986.8ns (-99.9%) | [-8046, -7928]ns | [4, 4] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 10646ns | +2611.0ns (+32.7%) | [+2555, +2662]ns | [10611, 10686] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 10642ns | +2634.8ns (+33.0%) | [+2538, +2765]ns | [10556, 10817] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 10632ns | +2588.6ns (+32.4%) | [+2559, +2678]ns | [10537, 10668] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|
| 1 | 8036ns | -100.0% | +29.9% | +30.0% | +31.6% |
| 2 | 8038ns | -100.0% | +29.8% | +30.0% | +30.7% |
| 3 | 8164ns | -99.9% | +27.8% | +28.0% | +31.3% |
| 4 | 8092ns | -99.9% | +29.0% | +29.0% | +30.5% |
| 5 | 7860ns | -100.0% | +32.8% | +32.9% | +32.9% |
| 6 | 8175ns | -100.0% | +28.8% | +28.1% | +30.5% |
| 7 | 7928ns | -100.0% | +31.9% | +31.8% | +31.7% |
| 8 | 7916ns | -99.9% | +32.3% | +31.9% | +32.0% |
| 9 | 7855ns | -100.0% | +32.8% | +33.3% | +34.0% |
| 10 | 7856ns | -99.9% | +32.9% | +33.0% | +32.9% |
| 11 | 7986ns | -99.9% | +32.8% | +33.2% | +33.6% |
| 12 | 8017ns | -99.9% | +32.2% | +33.2% | +32.6% |
| 13 | 7980ns | -100.0% | +33.7% | +37.7% | +37.3% |
| 14 | 7981ns | -99.9% | +34.3% | +37.9% | +40.3% |
| 15 | 7996ns | -99.9% | +33.1% | +42.0% | +41.2% |
| 16 | 8111ns | -99.9% | +31.2% | +33.6% | +33.1% |
| 17 | 8963ns | -100.0% | +18.8% | +19.0% | +20.8% |
| 18 | 8303ns | -100.0% | +28.9% | +29.0% | +33.8% |
| 19 | 8276ns | -100.0% | +28.2% | +28.4% | +31.2% |
| 20 | 8185ns | -100.0% | +30.4% | +29.6% | +30.3% |
| 21 | 8079ns | -100.0% | +32.8% | +34.2% | +34.2% |
| 22 | 8051ns | -100.0% | +32.8% | +37.3% | +32.1% |
| 23 | 8030ns | -99.9% | +32.7% | +31.8% | +31.1% |
| 24 | 7955ns | -99.9% | +34.4% | +31.7% | +32.4% |
| 25 | 7901ns | -99.9% | +37.1% | +32.1% | +34.7% |
| 26 | 7868ns | -100.0% | +36.7% | +33.8% | +36.4% |
| 27 | 7935ns | -100.0% | +33.7% | +32.3% | +35.6% |
| 28 | 8047ns | -100.0% | +39.4% | +31.5% | +37.8% |
| 29 | 8089ns | -99.9% | +32.2% | +31.4% | +32.0% |
| 30 | 7856ns | -99.9% | +35.3% | +35.8% | +34.7% |
| 31 | 8096ns | -99.9% | +31.2% | +34.2% | +29.0% |
| 32 | 7915ns | -99.9% | +35.0% | +39.4% | +33.0% |
| 33 | 8130ns | -99.9% | +31.4% | +34.7% | +29.6% |
| 34 | 7958ns | -100.0% | +33.7% | +41.7% | +31.4% |
| 35 | 8291ns | -99.9% | +30.8% | +32.9% | +26.3% |
| 36 | 7882ns | -100.0% | +36.2% | +41.8% | +32.5% |
| 37 | 7874ns | -99.9% | +33.2% | +40.6% | +35.1% |
| 38 | 7882ns | -99.9% | +38.5% | +39.4% | +33.9% |
| 39 | 7850ns | -100.0% | +36.1% | +37.6% | +33.9% |
| 40 | 7850ns | -100.0% | +39.5% | +35.6% | +35.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.340 | moderate+ |
| warm-container-kernel | 0.060 | ok |
| warm-container-minimum | 0.352 | moderate+ |
| warm-container-native | 0.750 | HIGH+ (drift/warm-up) |
| warm-container-plusone | 0.628 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-minimum**: won 0/40, lost 40/40
- **warm-container-native**: won 0/40, lost 40/40
- **warm-container-plusone**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 2.5ns | 8031.5ns | 0.0% |  |
| warm-container-kernel | 2.0ns | 4.0ns | 49.5% | HIGH |
| warm-container-minimum | 2.4ns | 10644.5ns | 0.0% |  |
| warm-container-native | 5.3ns | 10718.5ns | 0.0% |  |
| warm-container-plusone | 2.6ns | 10665.2ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 7858.7-8310.9 ns)
   7858.7 |##############################
   7881.3 |##############################
   7903.9 |####################
   7926.5 |####################
   7949.1 |####################
   7971.7 |##############################
   7994.4 |##########
   8017.0 |########################################
   8039.6 |####################
   8062.2 |##########
   8084.8 |##############################
   8107.4 |####################
   8130.0 |
   8152.6 |##########
   8175.3 |####################
   8197.9 |
   8220.5 |
   8243.1 |
   8265.7 |##########
   8288.3 |####################
  (5 below, 1 above range)

warm-container-kernel (n=40, range 2.7-5.3 ns)
      2.7 |
      2.8 |####
      2.9 |
      3.0 |
      3.2 |########################################
      3.3 |
      3.4 |
      3.6 |######################
      3.7 |
      3.8 |
      4.0 |
      4.1 |###############################
      4.2 |
      4.4 |
      4.5 |##########################
      4.6 |
      4.8 |
      4.9 |#################
      5.0 |
      5.2 |
  (4 below, 4 above range)

warm-container-minimum (n=40, range 10439.1-10872.4 ns)
  10439.1 |#################
  10460.7 |#####
  10482.4 |#####
  10504.1 |
  10525.7 |#####
  10547.4 |
  10569.1 |
  10590.7 |######################
  10612.4 |###########
  10634.1 |############################
  10655.7 |###########
  10677.4 |########################################
  10699.1 |#####
  10720.7 |###########
  10742.4 |#####
  10764.1 |
  10785.7 |
  10807.4 |
  10829.1 |###########
  10850.7 |
  (5 below, 3 above range)

warm-container-native (n=40, range 10444.0-11121.9 ns)
  10444.0 |########################################
  10477.8 |##########
  10511.7 |#####
  10545.6 |
  10579.5 |###############
  10613.4 |####################
  10647.3 |###############
  10681.2 |#####
  10715.1 |
  10749.0 |
  10782.9 |#####
  10816.8 |##########
  10850.7 |#####
  10884.6 |
  10918.5 |#####
  10952.4 |#####
  10986.3 |##########
  11020.2 |##########
  11054.1 |##########
  11088.0 |
  (3 below, 3 above range)

warm-container-plusone (n=40, range 10449.9-11022.3 ns)
  10449.9 |################
  10478.5 |########
  10507.1 |########################################
  10535.8 |########################
  10564.4 |################
  10593.0 |
  10621.6 |########################################
  10650.2 |########################
  10678.9 |########
  10707.5 |########
  10736.1 |################
  10764.7 |
  10793.3 |########
  10822.0 |################
  10850.6 |########
  10879.2 |
  10907.8 |
  10936.4 |########
  10965.0 |
  10993.7 |
  (6 below, 4 above range)

```

## Diagnostics

- **warm-container-kernel**: CV=24.5% (high variance, measurements may be unstable)
- **warm-container-kernel**: bridge=50.0% of algo (FFI overhead may distort results)
- **warm-container-native**: autocorrelation=0.75 (measurement drift or warm-up artifact)
- **warm-container-plusone**: autocorrelation=0.63 (measurement drift or warm-up artifact)
