# Wide rung, payload-shape sweep, 458752 elements (3 ops/element, past L2 for the wide strides)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide-rung-align16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide-rung-align16 has the worst median (1.00 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide-rung-wordround at 838.42 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### wide-rung-wordround is fastest but the noisiest (CV 12.4%)

wide-rung-wordround wins on median (838.42 us) yet has the highest variance (CV 12.4%), while wide-rung-wordround-alias is the steadiest (CV 3.1%, 958.07 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### wide-rung-align16 shows alternating (throttle bounce) (autocorr -0.52)

wide-rung-align16's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: wide-rung-wordround** at 838415.6 ns median (-16.2% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.19x (fastest 838415.6 ns, slowest 1000456.1 ns)

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
| wide-rung-align16 | 1000979ns | 1001102ns | 884782ns | 995279ns | 1134275ns | base |
| wide-rung-ragged | 892011ns | 922377ns | 784280ns | 911757ns | 940501ns | -10.89% |
| wide-rung-ragged-overread | 894548ns | 898594ns | 809746ns | 899192ns | 965420ns | -10.63% |
| wide-rung-wordround | 879638ns | 838793ns | 789388ns | 850574ns | 1057080ns | -12.12% |
| wide-rung-wordround-alias | 939406ns | 958430ns | 900119ns | 941604ns | 972100ns | -6.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 1000542ns | 884514ns | 1133843ns | base | 1.834 |
| wide-rung-ragged | 891545ns | 783967ns | 939728ns | -10.89% | 2.058 |
| wide-rung-ragged-overread | 894093ns | 809375ns | 964976ns | -10.64% | 2.052 |
| wide-rung-wordround | 879315ns | 789099ns | 1056821ns | -12.12% | 2.087 |
| wide-rung-wordround-alias | 938966ns | 899750ns | 971477ns | -6.15% | 1.954 |

## Performance model

- Peak throughput: **2.341 Gops/s** (wide-rung-ragged; best 20% batches)
- Ops per call: 1835008

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.834 | 78.4% |
| wide-rung-ragged | 1.990 | 85.0% |
| wide-rung-ragged-overread | 2.043 | 87.3% |
| wide-rung-wordround | 2.189 | 93.5% |
| wide-rung-wordround-alias | 1.915 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 1000979ns | 1000979ns | base |
| wide-rung-ragged | 892011ns | 892011ns | -10.89% |
| wide-rung-ragged-overread | 894548ns | 894548ns | -10.63% |
| wide-rung-wordround | 879638ns | 879638ns | -12.12% |
| wide-rung-wordround-alias | 939406ns | 939406ns | -6.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 1000456ns | base | --- | [919858, 1007816] | --- | --- | --- | --- |
| wide-rung-ragged | 922031ns | -121268.1ns (-12.1%) | [-175894, -67992]ns | [909896, 930154] | YES | 0.0014 | 0.0007 | 0 |
| wide-rung-ragged-overread | 898225ns | -110231.5ns (-11.0%) | [-156441, -94475]ns | [849959, 941658] | YES | 0.0030 | 0.0022 | 0 |
| wide-rung-wordround | 838416ns | -95653.1ns (-9.6%) | [-169004, -60235]ns | [821385, 844896] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround-alias | 958071ns | no significant difference | [-46199, +973]ns | [910246, 962565] | no | 0.1539 | 0.1539 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 962955ns | -18.6% | -16.8% | -18.0% | -0.1% |
| 1 | 2 | 1000456ns | -7.1% | -10.0% | -17.4% | -2.9% |
| 1 | 3 | 884859ns | +2.9% | +6.7% | -4.5% | +2.8% |
| 1 | 4 | 1115978ns | -15.9% | -23.4% | -5.4% | -19.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | -0.523 | HIGH- (thermal bounce) |
| wide-rung-ragged | -0.158 | ok |
| wide-rung-ragged-overread | -0.135 | ok |
| wide-rung-wordround | 0.012 | ok |
| wide-rung-wordround-alias | 0.238 | moderate+ |

**Consistency summary:**

- **wide-rung-ragged**: won 3/4, lost 1/4
- **wide-rung-ragged-overread**: won 3/4, lost 1/4
- **wide-rung-wordround**: won 4/4, lost 0/4
- **wide-rung-wordround-alias**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 60.1ns | 1000541.8ns | 0.0% |  |
| wide-rung-ragged | 102.5ns | 891544.6ns | 0.0% |  |
| wide-rung-ragged-overread | 74.9ns | 894093.4ns | 0.0% |  |
| wide-rung-wordround | 97.6ns | 879314.5ns | 0.0% |  |
| wide-rung-wordround-alias | 135.9ns | 938965.8ns | 0.0% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 884513.8-1133843.0 ns)
  884513.8 |####
  896980.3 |############
  909446.7 |########################
  921913.2 |
  934379.6 |
  946846.1 |
  959312.6 |
  971779.0 |
  984245.5 |############
  996711.9 |####################################
  1009178.4 |
  1021644.8 |
  1034111.3 |####
  1046577.8 |
  1059044.2 |####
  1071510.7 |
  1083977.1 |
  1096443.6 |
  1108910.0 |########################################
  1121376.5 |
  (5 below, 1 above range)

wide-rung-ragged (n=40, range 783966.5-939727.8 ns)
  783966.5 |#################
  791754.6 |####
  799542.6 |
  807330.7 |
  815118.8 |
  822906.8 |
  830694.9 |
  838483.0 |
  846271.0 |
  854059.1 |
  861847.2 |
  869635.2 |
  877423.3 |
  885211.3 |
  892999.4 |
  900787.5 |########
  908575.5 |###################################
  916363.6 |
  924151.7 |########################################
  931939.7 |########################################
  (5 below, 2 above range)

wide-rung-ragged-overread (n=40, range 809375.2-964976.5 ns)
  809375.2 |
  817155.3 |
  824935.3 |
  832715.4 |####
  840495.4 |######################
  848275.5 |#############
  856055.6 |
  863835.6 |####
  871615.7 |
  879395.8 |
  887175.8 |
  894955.9 |##########################
  902736.0 |####
  910516.0 |
  918296.1 |
  926076.1 |
  933856.2 |#############
  941636.3 |########################################
  949416.3 |#################
  957196.4 |
  (6 below, 1 above range)

wide-rung-wordround (n=40, range 789098.6-1056820.8 ns)
  789098.6 |####################
  802484.7 |
  815870.8 |####################
  829256.9 |####################
  842643.0 |########################################
  856029.1 |
  869415.2 |
  882801.3 |
  896187.4 |
  909573.6 |
  922959.7 |
  936345.8 |
  949731.9 |
  963118.0 |
  976504.1 |
  989890.2 |
  1003276.3 |
  1016662.4 |
  1030048.6 |
  1043434.7 |############################
  (5 below, 3 above range)

wide-rung-wordround-alias (n=40, range 899750.1-971476.7 ns)
  899750.1 |#################
  903336.5 |
  906922.8 |########################################
  910509.1 |###########
  914095.5 |
  917681.8 |
  921268.1 |
  924854.4 |
  928440.8 |#####
  932027.1 |
  935613.4 |
  939199.7 |
  942786.1 |
  946372.4 |
  949958.7 |
  953545.0 |###########
  957131.4 |###########
  960717.7 |########################################
  964304.0 |###########
  967890.3 |######################
  (5 below, 5 above range)

```
