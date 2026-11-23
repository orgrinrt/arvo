# Wide rung, payload-shape sweep, 458752 elements (3 ops/element, past L2 for the wide strides)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide-rung-align16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide-rung-align16 has the worst median (1.00 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide-rung-ragged at 886.54 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### wide-rung-ragged shows alternating (throttle bounce) (autocorr -0.55)

wide-rung-ragged's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: wide-rung-ragged** at 886542.7 ns median (-11.6% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.13x (fastest 886542.7 ns, slowest 1002947.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Wide rung, payload-shape sweep, 458752 elements (3 ops/element, past L2 for the wide strides) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112691900 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 996926ns | 1003430ns | 873197ns | 1001132ns | 1108035ns | base |
| wide-rung-ragged | 888595ns | 886987ns | 860352ns | 884007ns | 930603ns | -10.87% |
| wide-rung-ragged-overread | 960652ns | 942241ns | 938728ns | 951353ns | 1010471ns | -3.64% |
| wide-rung-wordround | 900639ns | 921780ns | 802437ns | 916299ns | 951862ns | -9.66% |
| wide-rung-wordround-alias | 902128ns | 901317ns | 859174ns | 902338ns | 944451ns | -9.51% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 996391ns | 872696ns | 1107428ns | base | 1.842 |
| wide-rung-ragged | 888270ns | 860117ns | 930246ns | -10.85% | 2.066 |
| wide-rung-ragged-overread | 960364ns | 938484ns | 1010116ns | -3.62% | 1.911 |
| wide-rung-wordround | 900268ns | 802003ns | 951559ns | -9.65% | 2.038 |
| wide-rung-wordround-alias | 901705ns | 858750ns | 944063ns | -9.50% | 2.035 |

## Performance model

- Peak throughput: **2.288 Gops/s** (wide-rung-wordround; best 20% batches)
- Ops per call: 1835008

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.830 | 80.0% |
| wide-rung-ragged | 2.070 | 90.5% |
| wide-rung-ragged-overread | 1.948 | 85.1% |
| wide-rung-wordround | 1.992 | 87.0% |
| wide-rung-wordround-alias | 2.037 | 89.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 996926ns | 996926ns | base |
| wide-rung-ragged | 888595ns | 888595ns | -10.87% |
| wide-rung-ragged-overread | 960652ns | 960652ns | -3.64% |
| wide-rung-wordround | 900639ns | 900639ns | -9.66% |
| wide-rung-wordround-alias | 902128ns | 902128ns | -9.51% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 1002948ns | base | --- | [980874, 1021471] | --- | --- | --- | --- |
| wide-rung-ragged | 886543ns | -99919.6ns (-10.0%) | [-117686, -66616]ns | [877159, 887947] | YES | 0.0030 | 0.0022 | 0 |
| wide-rung-ragged-overread | 941995ns | -34132.0ns (-3.4%) | [-54929, -897]ns | [941066, 945857] | YES (adj: no) | 0.0807 | 0.0807 | 0 |
| wide-rung-wordround | 921378ns | -121107.6ns (-12.1%) | [-156790, -61111]ns | [897190, 931943] | YES | 0.0004 | 0.0002 | 0 |
| wide-rung-wordround-alias | 900754ns | -103089.4ns (-10.3%) | [-122766, -72040]ns | [873715, 925282] | YES | 0.0002 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 998679ns | -8.7% | -0.9% | -14.6% | -10.7% |
| 1 | 2 | 1096573ns | -21.6% | -14.2% | -18.2% | -17.9% |
| 1 | 3 | 873894ns | +1.6% | +7.5% | +7.8% | +7.9% |
| 1 | 4 | 981853ns | -10.7% | -4.1% | -5.3% | -11.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.422 | moderate- |
| wide-rung-ragged | -0.553 | HIGH- (thermal bounce) |
| wide-rung-ragged-overread | -0.071 | ok |
| wide-rung-wordround | 0.215 | moderate+ |
| wide-rung-wordround-alias | -0.472 | moderate- |

**Consistency summary:**

- **wide-rung-ragged**: won 3/4, lost 1/4
- **wide-rung-ragged-overread**: won 3/4, lost 1/4
- **wide-rung-wordround**: won 3/4, lost 1/4
- **wide-rung-wordround-alias**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 52.5ns | 996390.8ns | 0.0% |  |
| wide-rung-ragged | 94.8ns | 888269.7ns | 0.0% |  |
| wide-rung-ragged-overread | 63.6ns | 960363.9ns | 0.0% |  |
| wide-rung-wordround | 87.0ns | 900268.5ns | 0.0% |  |
| wide-rung-wordround-alias | 61.2ns | 901704.7ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 872696.0-1107427.8 ns)
  872696.0 |############################
  884432.6 |
  896169.2 |#####
  907905.7 |
  919642.3 |
  931378.9 |
  943115.5 |
  954852.1 |
  966588.7 |###########
  978325.3 |##################################
  990061.9 |#####
  1001798.5 |#################
  1013535.1 |######################
  1025271.7 |
  1037008.2 |#####
  1048744.8 |
  1060481.4 |
  1072218.0 |
  1083954.6 |#################
  1095691.2 |########################################
  (5 below, 2 above range)

wide-rung-ragged (n=40, range 860117.1-930245.8 ns)
  860117.1 |########
  863623.6 |########
  867130.0 |####
  870636.4 |
  874142.9 |#################
  877649.3 |#################
  881155.7 |
  884662.2 |########################################
  888168.6 |####
  891675.1 |########
  895181.5 |#############
  898687.9 |####
  902194.4 |
  905700.8 |
  909207.2 |####
  912713.7 |
  916220.1 |
  919726.5 |
  923233.0 |####
  926739.4 |
  (5 below, 4 above range)

wide-rung-ragged-overread (n=40, range 938483.9-1010116.2 ns)
  938483.9 |########################################
  942065.5 |###############
  945647.1 |##
  949228.7 |
  952810.3 |
  956391.9 |
  959973.6 |
  963555.2 |
  967136.8 |
  970718.4 |
  974300.0 |
  977881.6 |
  981463.2 |#####
  985044.9 |#######
  988626.5 |#####
  992208.1 |#####
  995789.7 |##
  999371.3 |
  1002952.9 |##
  1006534.6 |
  (4 below, 2 above range)

wide-rung-wordround (n=40, range 802002.8-951558.7 ns)
  802002.8 |
  809480.5 |
  816958.3 |
  824436.1 |
  831913.9 |
  839391.7 |
  846869.5 |#####
  854347.3 |###############
  861825.1 |
  869302.9 |
  876780.7 |
  884258.5 |#####
  891736.3 |########################################
  899214.1 |##########
  906691.9 |
  914169.7 |#####
  921647.5 |###############
  929125.3 |#########################
  936603.1 |##############################
  944080.9 |#########################
  (4 below, 1 above range)

wide-rung-wordround-alias (n=40, range 858750.1-944063.4 ns)
  858750.1 |
  863015.7 |#########################
  867281.4 |###############
  871547.1 |##########
  875812.7 |
  880078.4 |
  884344.1 |
  888609.7 |
  892875.4 |
  897141.1 |##############################
  901406.7 |##########
  905672.4 |#####
  909938.1 |
  914203.7 |#####
  918469.4 |
  922735.1 |###############
  927000.7 |#####
  931266.4 |
  935532.1 |#####
  939797.7 |########################################
  (5 below, 2 above range)

```
