# Wide rung, payload-shape sweep, cache-resident (2048 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-align16 dominates: 19% faster than the next best (wide-rung-ragged-overread)

wide-rung-align16 (5.35 us) leads wide-rung-ragged-overread (6.37 us) by 19%, a clear separation rather than a photo finish. CV 41.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### wide-rung-align16 is fastest but the noisiest (CV 41.5%)

wide-rung-align16 wins on median (5.35 us) yet has the highest variance (CV 41.5%), while wide-rung-ragged is the steadiest (CV 9.4%, 6.78 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### wide-rung-align16 shows alternating (throttle bounce) (autocorr -0.57)

wide-rung-align16's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.43 us) is smaller than the fastest variant's own run-to-run std-dev (2.22 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (wide-rung-align16)

The baseline wide-rung-align16 is the fastest (5.35 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### wide-rung-align16 is inconsistent: worst-20% is 2.1x its best-20%

wide-rung-align16's best 20% of batches run at 5.28 us but its worst 20% at 10.98 us (2.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (wide-rung-align16) is the fastest** at 5353.2 ns median
- 1 variant significantly slower than baseline
- Spread: 1.27x (fastest 5353.2 ns, slowest 6783.8 ns)

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
| wide-rung-align16 | 6889ns | 5449ns | 5346ns | 5987ns | 11141ns | base |
| wide-rung-ragged | 6413ns | 6910ns | 5696ns | 6438ns | 7056ns | -6.91% |
| wide-rung-ragged-overread | 6477ns | 6490ns | 5808ns | 6463ns | 7189ns | -5.98% |
| wide-rung-wordround | 6347ns | 6551ns | 5356ns | 6500ns | 6879ns | -7.87% |
| wide-rung-wordround-alias | 5992ns | 6541ns | 5314ns | 6003ns | 6639ns | -13.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 6796ns | 5279ns | 10982ns | base | 1.205 |
| wide-rung-ragged | 6332ns | 5630ns | 6963ns | -6.83% | 1.294 |
| wide-rung-ragged-overread | 6396ns | 5739ns | 7094ns | -5.89% | 1.281 |
| wide-rung-wordround | 6261ns | 5289ns | 6764ns | -7.88% | 1.308 |
| wide-rung-wordround-alias | 5912ns | 5247ns | 6535ns | -13.01% | 1.386 |

## Performance model

- Peak throughput: **1.561 Gops/s** (wide-rung-wordround-alias; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.530 | 98.0% |
| wide-rung-ragged | 1.208 | 77.3% |
| wide-rung-ragged-overread | 1.286 | 82.4% |
| wide-rung-wordround | 1.268 | 81.2% |
| wide-rung-wordround-alias | 1.268 | 81.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 6889ns | 6889ns | base |
| wide-rung-ragged | 6413ns | 6413ns | -6.91% |
| wide-rung-ragged-overread | 6477ns | 6477ns | -5.98% |
| wide-rung-wordround | 6347ns | 6347ns | -7.87% |
| wide-rung-wordround-alias | 5992ns | 5992ns | -13.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 5353ns | base | --- | [5295, 6472] | --- | --- | --- | --- |
| wide-rung-ragged | 6784ns | no significant difference | [-838, +409]ns | [5689, 6952] | no | 0.8746 | 0.8746 | 0 |
| wide-rung-ragged-overread | 6371ns | +521.0ns (+9.7%) | [+451, +558]ns | [5748, 7018] | YES | 0.0027 | 0.0007 | 0 |
| wide-rung-wordround | 6463ns | no significant difference | [-1174, +1174]ns | [6459, 6471] | no | 0.8478 | 0.6358 | 0 |
| wide-rung-wordround-alias | 6461ns | no significant difference | [-10, +14]ns | [5291, 6472] | no | 0.8478 | 0.6358 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 5294ns | +31.4% | +8.5% | +22.2% | +0.0% |
| 1 | 2 | 11326ns | -38.6% | -49.3% | -42.9% | -53.6% |
| 1 | 3 | 5293ns | +7.5% | +32.7% | +22.2% | +22.3% |
| 1 | 4 | 6472ns | -12.9% | +8.6% | -18.2% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.570 | HIGH- (thermal bounce) |
| wide-rung-ragged | 0.260 | moderate+ |
| wide-rung-ragged-overread | 0.252 | moderate+ |
| wide-rung-wordround | -0.083 | ok |
| wide-rung-wordround-alias | 0.240 | moderate+ |

**Consistency summary:**

- **wide-rung-ragged**: won 2/4, lost 2/4
- **wide-rung-ragged-overread**: won 1/4, lost 3/4
- **wide-rung-wordround**: won 2/4, lost 2/4
- **wide-rung-wordround-alias**: won 1/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 25.1ns | 6796.4ns | 0.4% |  |
| wide-rung-ragged | 21.3ns | 6332.1ns | 0.3% |  |
| wide-rung-ragged-overread | 21.4ns | 6396.0ns | 0.3% |  |
| wide-rung-wordround | 22.8ns | 6261.0ns | 0.4% |  |
| wide-rung-wordround-alias | 21.1ns | 5912.0ns | 0.4% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 5278.9-10982.3 ns)
   5278.9 |########################################
   5564.1 |
   5849.3 |
   6134.5 |
   6419.6 |####################
   6704.8 |
   6990.0 |
   7275.1 |
   7560.3 |
   7845.5 |
   8130.6 |##
   8415.8 |##
   8701.0 |
   8986.1 |
   9271.3 |
   9556.5 |
   9841.7 |
  10126.8 |
  10412.0 |
  10697.2 |
  (1 below, 7 above range)

wide-rung-ragged (n=40, range 5629.5-6963.4 ns)
   5629.5 |###########################
   5696.2 |####
   5762.9 |
   5829.6 |
   5896.3 |
   5963.0 |
   6029.7 |
   6096.4 |
   6163.1 |
   6229.8 |
   6296.5 |
   6363.2 |
   6429.8 |
   6496.5 |
   6563.2 |##
   6629.9 |
   6696.6 |
   6763.3 |
   6830.0 |
   6896.7 |########################################
  (4 below, 1 above range)

wide-rung-ragged-overread (n=40, range 5739.4-7094.2 ns)
   5739.4 |########################################
   5807.2 |
   5874.9 |
   5942.6 |
   6010.4 |
   6078.1 |
   6145.9 |
   6213.6 |
   6281.3 |
   6349.1 |
   6416.8 |
   6484.6 |
   6552.3 |
   6620.0 |
   6687.8 |
   6755.5 |
   6823.3 |
   6891.0 |
   6958.7 |##############################
   7026.5 |#################
  (4 below, 1 above range)

wide-rung-wordround (n=40, range 5289.0-6764.2 ns)
   5289.0 |###########
   5362.8 |
   5436.5 |
   5510.3 |
   5584.0 |
   5657.8 |
   5731.6 |
   5805.3 |
   5879.1 |
   5952.8 |
   6026.6 |
   6100.4 |
   6174.1 |
   6247.9 |
   6321.6 |
   6395.4 |##############################
   6469.1 |########################################
   6542.9 |
   6616.7 |
   6690.4 |
  (4 below, 1 above range)

wide-rung-wordround-alias (n=40, range 5246.8-6534.6 ns)
   5246.8 |########################################
   5311.2 |
   5375.6 |
   5440.0 |
   5504.4 |
   5568.8 |
   5633.2 |
   5697.6 |
   5761.9 |
   5826.3 |
   5890.7 |
   5955.1 |
   6019.5 |
   6083.9 |
   6148.3 |
   6212.7 |
   6277.1 |
   6341.4 |
   6405.8 |################
   6470.2 |#####################################
  (4 below, 1 above range)

```

## Diagnostics

- **wide-rung-align16**: CV=32.7% (high variance, measurements may be unstable)
