# Wide rung, payload-shape sweep, 458752 elements (3 ops/element, past L2 for the wide strides)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide-rung-align16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide-rung-align16 has the worst median (1.06 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide-rung-ragged-overread at 889.91 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### wide-rung-ragged-overread dominates: 11% faster than the next best (wide-rung-wordround-alias)

wide-rung-ragged-overread (889.91 us) leads wide-rung-wordround-alias (984.13 us) by 11%, a clear separation rather than a photo finish. CV 6.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### wide-rung-ragged-overread is fastest but the noisiest (CV 6.1%)

wide-rung-ragged-overread wins on median (889.91 us) yet has the highest variance (CV 6.1%), while wide-rung-ragged is the steadiest (CV 4.0%, 1.02 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### wide-rung-ragged shows alternating (throttle bounce) (autocorr -0.79)

wide-rung-ragged's per-pass series has lag-1 autocorrelation -0.79, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: wide-rung-ragged-overread** at 889910.2 ns median (-16.2% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.19x (fastest 889910.2 ns, slowest 1061491.2 ns)

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
| wide-rung-align16 | 1060392ns | 1062052ns | 981782ns | 1053648ns | 1159236ns | base |
| wide-rung-ragged | 1025804ns | 1023323ns | 976055ns | 1028681ns | 1066924ns | -3.26% |
| wide-rung-ragged-overread | 930152ns | 890218ns | 883762ns | 915826ns | 1019523ns | -12.28% |
| wide-rung-wordround | 1039605ns | 1030526ns | 993141ns | 1025195ns | 1129300ns | -1.96% |
| wide-rung-wordround-alias | 980399ns | 984416ns | 891441ns | 985614ns | 1053714ns | -7.54% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 1059888ns | 981371ns | 1158702ns | base | 1.731 |
| wide-rung-ragged | 1025468ns | 975712ns | 1066494ns | -3.25% | 1.789 |
| wide-rung-ragged-overread | 929931ns | 883608ns | 1019272ns | -12.26% | 1.973 |
| wide-rung-wordround | 1039200ns | 992709ns | 1128993ns | -1.95% | 1.766 |
| wide-rung-wordround-alias | 980039ns | 891144ns | 1053343ns | -7.53% | 1.872 |

## Performance model

- Peak throughput: **2.077 Gops/s** (wide-rung-ragged-overread; best 20% batches)
- Ops per call: 1835008

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.729 | 83.2% |
| wide-rung-ragged | 1.794 | 86.4% |
| wide-rung-ragged-overread | 2.062 | 99.3% |
| wide-rung-wordround | 1.781 | 85.8% |
| wide-rung-wordround-alias | 1.865 | 89.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 1060392ns | 1060392ns | base |
| wide-rung-ragged | 1025804ns | 1025804ns | -3.26% |
| wide-rung-ragged-overread | 930152ns | 930152ns | -12.28% |
| wide-rung-wordround | 1039605ns | 1039605ns | -1.96% |
| wide-rung-wordround-alias | 980399ns | 980399ns | -7.54% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 1061491ns | base | --- | [1060368, 1063120] | --- | --- | --- | --- |
| wide-rung-ragged | 1023121ns | -39101.9ns (-3.7%) | [-41707, -15809]ns | [1020993, 1034555] | YES | 0.0166 | 0.0166 | 0 |
| wide-rung-ragged-overread | 889910ns | -130668.2ns (-12.3%) | [-177325, -101005]ns | [884725, 945531] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround | 1030087ns | -51762.1ns (-4.9%) | [-68075, -23397]ns | [999812, 1040722] | YES | 0.0166 | 0.0166 | 0 |
| wide-rung-wordround-alias | 984132ns | -85664.1ns (-8.1%) | [-101179, -72434]ns | [967588, 998151] | YES | 0.0166 | 0.0166 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 1061254ns | -3.7% | -10.9% | -1.3% | -16.8% |
| 1 | 2 | 989790ns | +5.9% | +2.0% | +4.1% | +3.9% |
| 1 | 3 | 1063120ns | -4.0% | -16.9% | -2.9% | -9.0% |
| 1 | 4 | 1061760ns | -1.5% | -16.7% | -6.5% | -7.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.417 | moderate- |
| wide-rung-ragged | -0.789 | HIGH- (thermal bounce) |
| wide-rung-ragged-overread | -0.033 | ok |
| wide-rung-wordround | -0.051 | ok |
| wide-rung-wordround-alias | -0.454 | moderate- |

**Consistency summary:**

- **wide-rung-ragged**: won 3/4, lost 1/4
- **wide-rung-ragged-overread**: won 3/4, lost 1/4
- **wide-rung-wordround**: won 3/4, lost 1/4
- **wide-rung-wordround-alias**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 109.2ns | 1059888.3ns | 0.0% |  |
| wide-rung-ragged | 110.4ns | 1025468.2ns | 0.0% |  |
| wide-rung-ragged-overread | 36.5ns | 929931.3ns | 0.0% |  |
| wide-rung-wordround | 143.3ns | 1039200.2ns | 0.0% |  |
| wide-rung-wordround-alias | 72.0ns | 980038.8ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 981371.1-1158702.2 ns)
  981371.1 |################
  990237.6 |#####
  999104.2 |
  1007970.7 |
  1016837.3 |
  1025703.8 |
  1034570.4 |
  1043437.0 |
  1052303.5 |################
  1061170.1 |########################################
  1070036.6 |
  1078903.2 |#####
  1087769.8 |
  1096636.3 |
  1105502.9 |
  1114369.4 |
  1123236.0 |
  1132102.5 |#####
  1140969.1 |
  1149835.7 |
  (3 below, 4 above range)

wide-rung-ragged (n=40, range 975711.7-1066494.1 ns)
  975711.7 |
  980250.8 |
  984790.0 |
  989329.1 |
  993868.2 |
  998407.3 |
  1002946.4 |
  1007485.6 |
  1012024.7 |
  1016563.8 |########################################
  1021102.9 |#############################
  1025642.0 |#######
  1030181.2 |###
  1034720.3 |###
  1039259.4 |
  1043798.5 |##################
  1048337.6 |#########################
  1052876.8 |
  1057415.9 |
  1061955.0 |
  (4 below, 1 above range)

wide-rung-ragged-overread (n=40, range 883607.8-1019271.8 ns)
  883607.8 |########################################
  890391.0 |
  897174.2 |
  903957.4 |
  910740.6 |
  917523.8 |
  924307.0 |
  931090.2 |
  937873.4 |
  944656.6 |####################
  951439.8 |
  958223.0 |
  965006.2 |##
  971789.4 |##
  978572.6 |
  985355.8 |
  992139.0 |
  998922.2 |##
  1005705.4 |#############
  1012488.6 |
  (3 below, 1 above range)

wide-rung-wordround (n=40, range 992708.9-1128992.6 ns)
  992708.9 |########################################
  999523.1 |###
  1006337.3 |
  1013151.5 |######
  1019965.6 |###
  1026779.8 |##########
  1033594.0 |#############
  1040408.2 |################
  1047222.4 |###
  1054036.6 |
  1060850.7 |
  1067664.9 |###
  1074479.1 |
  1081293.3 |
  1088107.5 |##########
  1094921.6 |###
  1101735.8 |
  1108550.0 |
  1115364.2 |
  1122178.4 |
  (2 below, 4 above range)

wide-rung-wordround-alias (n=40, range 891144.2-1053342.9 ns)
  891144.2 |
  899254.1 |####
  907364.1 |
  915474.0 |
  923584.0 |
  931693.9 |
  939803.8 |
  947913.8 |####
  956023.7 |########
  964133.6 |########################################
  972243.6 |
  980353.5 |########################
  988463.4 |####
  996573.4 |####
  1004683.3 |####
  1012793.2 |
  1020903.2 |########################
  1029013.1 |############
  1037123.0 |
  1045233.0 |
  (5 below, 3 above range)

```
