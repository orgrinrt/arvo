# Container fork under saturating semantics, declared-width sweep (8192 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-kernel dominates: 275% faster than the next best (warm-container-headroom)

warm-container-kernel (1.07 us) leads warm-container-headroom (4.02 us) by 275%, a clear separation rather than a photo finish. CV 14.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-container-kernel beats baseline by 73% (significant)

warm-container-kernel is -2.94 us (73%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-native is an outlier: 7.4x slower than the field

warm-container-native (7.89 us) is 7.4x the fastest (1.07 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-minimum shows alternating (throttle bounce) (autocorr -0.53)

warm-container-minimum's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-kernel} vs {warm-container-headroom, warm-container-minimum, warm-container-plusone, warm-container-native} (275% apart)

The field splits into a fast tier {warm-container-kernel} and a slow tier {warm-container-headroom, warm-container-minimum, warm-container-plusone, warm-container-native} with a 275% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 7.4x the fastest

Fastest warm-container-kernel (1.07 us) to slowest warm-container-native (7.89 us): 7.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-container-headroom is inconsistent: worst-20% is 2.7x its best-20%

warm-container-headroom's best 20% of batches run at 3.38 us but its worst 20% at 9.28 us (2.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-kernel** at 1072.1 ns median (-73.4% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 7.36x (fastest 1072.1 ns, slowest 7893.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 5748ns | 4088ns | 3430ns | 5278ns | 9477ns | base |
| warm-container-kernel | 1342ns | 1311ns | 1157ns | 1312ns | 1617ns | -76.65% |
| warm-container-minimum | 7495ns | 5955ns | 4937ns | 6612ns | 12702ns | +30.39% |
| warm-container-native | 8592ns | 7983ns | 7095ns | 8175ns | 11340ns | +49.47% |
| warm-container-plusone | 6559ns | 6086ns | 5901ns | 6083ns | 8646ns | +14.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 5643ns | 3376ns | 9279ns | base | 5.806 |
| warm-container-kernel | 1085ns | 906ns | 1325ns | -80.77% | 30.201 |
| warm-container-minimum | 7404ns | 4879ns | 12536ns | +31.20% | 4.425 |
| warm-container-native | 8508ns | 7015ns | 11223ns | +50.76% | 3.851 |
| warm-container-plusone | 6471ns | 5829ns | 8489ns | +14.66% | 5.064 |

## Performance model

- Peak throughput: **36.183 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 8.142 | 22.5% |
| warm-container-kernel | 30.564 | 84.5% |
| warm-container-minimum | 5.570 | 15.4% |
| warm-container-native | 4.151 | 11.5% |
| warm-container-plusone | 5.449 | 15.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 5748ns | 5748ns | base |
| warm-container-kernel | 1342ns | 1342ns | -76.65% |
| warm-container-minimum | 7495ns | 7495ns | +30.39% |
| warm-container-native | 8592ns | 8592ns | +49.47% |
| warm-container-plusone | 6559ns | 6559ns | +14.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 4024ns | base | --- | [4012, 7272] | --- | --- | --- | --- |
| warm-container-kernel | 1072ns | -2944.5ns (-73.2%) | [-6328, -2794]ns | [1007, 1107] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 5883ns | +1603.1ns (+39.8%) | [+1208, +1878]ns | [4896, 5913] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 7893ns | +3857.8ns (+95.9%) | [+3279, +3954]ns | [7811, 7988] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 6014ns | +1821.8ns (+45.3%) | [+640, +1998]ns | [6003, 6030] | YES (adj: no) | 0.0807 | 0.0807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-kernel | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 1 | 3287ns | -69.4% | +48.6% | +140.1% | +84.4% |
| 1 | 2 | 7288ns | -87.5% | +65.1% | +53.8% | -17.5% |
| 1 | 3 | 4018ns | -68.7% | +21.8% | +96.4% | +49.6% |
| 1 | 4 | 4024ns | -72.5% | +46.4% | +74.5% | +45.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | -0.507 | HIGH- (thermal bounce) |
| warm-container-kernel | -0.197 | ok |
| warm-container-minimum | -0.527 | HIGH- (thermal bounce) |
| warm-container-native | -0.234 | moderate- |
| warm-container-plusone | -0.031 | ok |

**Consistency summary:**

- **warm-container-kernel**: won 4/4, lost 0/4
- **warm-container-minimum**: won 0/4, lost 4/4
- **warm-container-native**: won 0/4, lost 4/4
- **warm-container-plusone**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 34.6ns | 5643.4ns | 0.6% |  |
| warm-container-kernel | 92.7ns | 1085.0ns | 8.5% | HIGH |
| warm-container-minimum | 28.3ns | 7404.4ns | 0.4% |  |
| warm-container-native | 26.9ns | 8508.2ns | 0.3% |  |
| warm-container-plusone | 27.2ns | 6470.9ns | 0.4% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 3376.1-9279.4 ns)
   3376.1 |
   3671.3 |
   3966.4 |########################################
   4261.6 |
   4556.8 |
   4851.9 |
   5147.1 |
   5442.3 |
   5737.4 |##
   6032.6 |
   6327.7 |
   6622.9 |
   6918.1 |
   7213.2 |#########################
   7508.4 |
   7803.5 |
   8098.7 |#####
   8393.9 |
   8689.0 |
   8984.2 |
  (7 below, 4 above range)

warm-container-kernel (n=40, range 905.6-1324.8 ns)
    905.6 |
    926.6 |###########
    947.5 |############################
    968.5 |
    989.5 |##################################
   1010.4 |
   1031.4 |#####
   1052.3 |#####
   1073.3 |#####
   1094.3 |########################################
   1115.2 |#####
   1136.2 |#####
   1157.1 |
   1178.1 |
   1199.1 |
   1220.0 |######################
   1241.0 |#####
   1261.9 |
   1282.9 |#####
   1303.9 |
  (5 below, 4 above range)

warm-container-minimum (n=40, range 4878.7-12536.0 ns)
   4878.7 |########################################
   5261.6 |
   5644.5 |#########################
   6027.3 |
   6410.2 |##
   6793.1 |
   7175.9 |
   7558.8 |
   7941.7 |
   8324.5 |
   8707.4 |
   9090.3 |
   9473.1 |
   9856.0 |
  10238.9 |
  10621.7 |
  11004.6 |
  11387.5 |
  11770.3 |############################
  12153.2 |##
  (4 below, 1 above range)

warm-container-native (n=40, range 7015.5-11223.5 ns)
   7015.5 |####################
   7225.9 |
   7436.3 |###
   7646.7 |####################
   7857.1 |########################################
   8067.5 |###
   8277.9 |
   8488.3 |
   8698.7 |
   8909.1 |
   9119.5 |
   9329.9 |
   9540.3 |
   9750.7 |
   9961.1 |
  10171.5 |
  10381.9 |
  10592.3 |
  10802.7 |
  11013.1 |####################
  (4 below, 4 above range)

warm-container-plusone (n=40, range 5829.2-8489.4 ns)
   5829.2 |########
   5962.2 |########################################
   6095.2 |
   6228.2 |
   6361.3 |
   6494.3 |
   6627.3 |
   6760.3 |
   6893.3 |
   7026.3 |
   7159.3 |
   7292.3 |#
   7425.3 |
   7558.3 |
   7691.3 |
   7824.3 |
   7957.4 |
   8090.4 |
   8223.4 |
   8356.4 |
  (3 below, 3 above range)

```

## Diagnostics

- **warm-container-headroom**: CV=42.4% (high variance, measurements may be unstable)
- **warm-container-minimum**: CV=45.0% (high variance, measurements may be unstable)
- **warm-container-plusone**: CV=27.4% (high variance, measurements may be unstable)
