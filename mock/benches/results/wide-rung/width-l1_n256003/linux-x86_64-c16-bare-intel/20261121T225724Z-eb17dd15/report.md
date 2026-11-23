# Wide rung, payload-shape sweep, cache-resident (2048 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (wide-rung-ragged, wide-rung-ragged-overread) are a dead heat (<1%)

wide-rung-ragged (5.64 us) and wide-rung-ragged-overread (5.64 us) differ by 0.01%, inside the noise, even though the wider field spreads 22.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### wide-rung-ragged shows alternating (throttle bounce) (autocorr -0.79)

wide-rung-ragged's per-pass series has lag-1 autocorrelation -0.79, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### wide-rung-wordround-alias's edge over baseline is significant but tiny (2 ns, 0.03%)

wide-rung-wordround-alias differs from baseline wide-rung-align16 by 2 ns (0.03%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-ragged** at 5644.1 ns median (-14.6% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.22x (fastest 5644.1 ns, slowest 6883.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Wide rung, payload-shape sweep, cache-resident (2048 elements, 3 ops/element) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112636733 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 6370ns | 6685ns | 5692ns | 6382ns | 7011ns | base |
| wide-rung-ragged | 5733ns | 5710ns | 5701ns | 5708ns | 5842ns | -9.99% |
| wide-rung-ragged-overread | 6273ns | 5713ns | 5697ns | 5870ns | 8059ns | -1.51% |
| wide-rung-wordround | 6366ns | 6770ns | 5691ns | 6377ns | 7006ns | -0.06% |
| wide-rung-wordround-alias | 6652ns | 6964ns | 5696ns | 6859ns | 6988ns | +4.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 6290ns | 5625ns | 6916ns | base | 1.302 |
| wide-rung-ragged | 5661ns | 5634ns | 5746ns | -10.01% | 1.447 |
| wide-rung-ragged-overread | 6194ns | 5630ns | 7949ns | -1.53% | 1.322 |
| wide-rung-wordround | 6286ns | 5623ns | 6912ns | -0.08% | 1.303 |
| wide-rung-wordround-alias | 6569ns | 5629ns | 6888ns | +4.43% | 1.247 |

## Performance model

- Peak throughput: **1.457 Gops/s** (wide-rung-wordround; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.239 | 85.0% |
| wide-rung-ragged | 1.451 | 99.6% |
| wide-rung-ragged-overread | 1.451 | 99.6% |
| wide-rung-wordround | 1.232 | 84.6% |
| wide-rung-wordround-alias | 1.190 | 81.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 6370ns | 6370ns | base |
| wide-rung-ragged | 5733ns | 5733ns | -9.99% |
| wide-rung-ragged-overread | 6273ns | 6273ns | -1.51% |
| wide-rung-wordround | 6366ns | 6366ns | -0.06% |
| wide-rung-wordround-alias | 6652ns | 6652ns | +4.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 6612ns | base | --- | [5642, 6883] | --- | --- | --- | --- |
| wide-rung-ragged | 5644ns | -698.8ns (-10.6%) | [-1237, -7]ns | [5635, 5645] | YES (adj: no) | 0.1614 | 0.0807 | 0 |
| wide-rung-ragged-overread | 5645ns | no significant difference | [-25, +6]ns | [5636, 5648] | no | 0.8746 | 0.8746 | 0 |
| wide-rung-wordround | 6650ns | no significant difference | [-6, +6]ns | [5635, 6877] | no | 0.2051 | 0.1539 | 0 |
| wide-rung-wordround-alias | 6883ns | +2.1ns (+0.0%) | [+0, +13]ns | [6872, 6884] | YES (adj: no) | 0.1539 | 0.0385 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 5632ns | +0.2% | +0.2% | +0.0% | +22.2% |
| 1 | 2 | 5633ns | +0.1% | +0.0% | +22.2% | +0.0% |
| 1 | 3 | 6884ns | -18.0% | -18.0% | -18.2% | -0.0% |
| 1 | 4 | 6883ns | -18.1% | +0.1% | +0.0% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.250 | moderate+ |
| wide-rung-ragged | -0.790 | HIGH- (thermal bounce) |
| wide-rung-ragged-overread | -0.078 | ok |
| wide-rung-wordround | -0.750 | HIGH- (thermal bounce) |
| wide-rung-wordround-alias | -0.415 | moderate- |

**Consistency summary:**

- **wide-rung-ragged**: won 2/4, lost 2/4
- **wide-rung-ragged-overread**: won 1/4, lost 2/4
- **wide-rung-wordround**: won 1/4, lost 1/4
- **wide-rung-wordround-alias**: won 0/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 21.3ns | 6290.4ns | 0.3% |  |
| wide-rung-ragged | 19.0ns | 5660.6ns | 0.3% |  |
| wide-rung-ragged-overread | 21.1ns | 6194.4ns | 0.3% |  |
| wide-rung-wordround | 21.4ns | 6285.6ns | 0.3% |  |
| wide-rung-wordround-alias | 22.0ns | 6569.2ns | 0.3% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 5624.6-6915.7 ns)
   5624.6 |#######################
   5689.1 |##
   5753.7 |
   5818.2 |##
   5882.8 |
   5947.3 |
   6011.9 |
   6076.5 |
   6141.0 |
   6205.6 |
   6270.1 |
   6334.7 |##
   6399.2 |
   6463.8 |
   6528.3 |
   6592.9 |
   6657.5 |
   6722.0 |
   6786.6 |
   6851.1 |########################################
  (6 below, 1 above range)

wide-rung-ragged (n=40, range 5634.3-5746.3 ns)
   5634.3 |########################################
   5639.9 |########################################
   5645.5 |#################
   5651.1 |##
   5656.7 |
   5662.3 |
   5667.9 |
   5673.5 |
   5679.1 |
   5684.7 |
   5690.3 |
   5695.9 |
   5701.5 |
   5707.1 |
   5712.7 |
   5718.3 |
   5723.9 |
   5729.5 |
   5735.1 |
   5740.7 |
  (4 below, 1 above range)

wide-rung-ragged-overread (n=40, range 5629.8-7948.6 ns)
   5629.8 |########################################
   5745.7 |
   5861.6 |
   5977.6 |
   6093.5 |
   6209.5 |
   6325.4 |
   6441.3 |
   6557.3 |
   6673.2 |
   6789.2 |#########
   6905.1 |###
   7021.0 |###
   7137.0 |
   7252.9 |
   7368.9 |
   7484.8 |
   7600.7 |
   7716.7 |
   7832.6 |
  (4 below, 1 above range)

wide-rung-wordround (n=40, range 5623.5-6912.1 ns)
   5623.5 |#############################
   5687.9 |
   5752.3 |##
   5816.8 |
   5881.2 |
   5945.6 |
   6010.1 |
   6074.5 |
   6138.9 |
   6203.4 |
   6267.8 |
   6332.2 |
   6396.7 |##
   6461.1 |
   6525.5 |
   6590.0 |
   6654.4 |
   6718.8 |
   6783.3 |
   6847.7 |########################################
  (4 below, 1 above range)

wide-rung-wordround-alias (n=40, range 5629.4-6888.4 ns)
   5629.4 |#########
   5692.4 |
   5755.3 |
   5818.3 |
   5881.2 |
   5944.2 |
   6007.1 |
   6070.1 |
   6133.0 |
   6196.0 |
   6258.9 |
   6321.9 |
   6384.8 |
   6447.8 |
   6510.7 |
   6573.7 |
   6636.6 |
   6699.6 |
   6762.5 |
   6825.5 |########################################
  (4 below, 4 above range)

```

## Diagnostics

- **wide-rung-ragged-overread**: CV=24.4% (high variance, measurements may be unstable)
