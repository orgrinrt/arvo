# Wide rung at the ratified numeral (W=200), operation-count sweep, cache-resident

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

No notable statistical pattern fired: the variants do not separate meaningfully on this run.

## Key findings

- **Fastest: wide-rung-wordround** at 7270.1 ns median (-18.2% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.28x (fastest 7270.1 ns, slowest 9324.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Wide rung at the ratified numeral (W=200), operation-count sweep, cache-resident |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112722093 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 8664ns | 8960ns | 7283ns | 8493ns | 10558ns | base |
| wide-rung-ragged | 7841ns | 7458ns | 7293ns | 7588ns | 9147ns | -9.50% |
| wide-rung-ragged-overread | 9393ns | 9405ns | 9309ns | 9405ns | 9441ns | +8.41% |
| wide-rung-wordround | 7853ns | 7338ns | 7326ns | 7628ns | 9056ns | -9.36% |
| wide-rung-wordround-alias | 8954ns | 8960ns | 8877ns | 8956ns | 9023ns | +3.34% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 8581ns | 7211ns | 10458ns | base | 1.193 |
| wide-rung-ragged | 7764ns | 7226ns | 9058ns | -9.52% | 1.319 |
| wide-rung-ragged-overread | 9307ns | 9227ns | 9345ns | +8.45% | 1.100 |
| wide-rung-wordround | 7776ns | 7259ns | 8966ns | -9.38% | 1.317 |
| wide-rung-wordround-alias | 8867ns | 8796ns | 8922ns | +3.33% | 1.155 |

## Performance model

- Peak throughput: **1.420 Gops/s** (wide-rung-align16; best 20% batches)
- Ops per call: 10240

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.153 | 81.2% |
| wide-rung-ragged | 1.386 | 97.6% |
| wide-rung-ragged-overread | 1.098 | 77.3% |
| wide-rung-wordround | 1.409 | 99.2% |
| wide-rung-wordround-alias | 1.153 | 81.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 8664ns | 8664ns | base |
| wide-rung-ragged | 7841ns | 7841ns | -9.50% |
| wide-rung-ragged-overread | 9393ns | 9393ns | +8.41% |
| wide-rung-wordround | 7853ns | 7853ns | -9.36% |
| wide-rung-wordround-alias | 8954ns | 8954ns | +3.34% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 8882ns | base | --- | [7269, 8887] | --- | --- | --- | --- |
| wide-rung-ragged | 7390ns | no significant difference | [-1066, +161]ns | [7368, 7396] | no | 0.3576 | 0.2682 | 0 |
| wide-rung-ragged-overread | 9324ns | +441.8ns (+5.0%) | [+436, +2074]ns | [9323, 9326] | YES | 0.0089 | 0.0022 | 0 |
| wide-rung-wordround | 7270ns | no significant difference | [-1611, +3]ns | [7267, 7651] | no | 0.3576 | 0.2682 | 0 |
| wide-rung-wordround-alias | 8880ns | no significant difference | [-13, +1592]ns | [8877, 8881] | no | 0.8746 | 0.8746 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 8886ns | +1.9% | +4.9% | -18.2% | -0.1% |
| 1 | 2 | 10176ns | -28.9% | -9.3% | -12.7% | -12.8% |
| 1 | 3 | 7267ns | +1.6% | +28.6% | +0.0% | +22.2% |
| 1 | 4 | 7199ns | +2.7% | +29.5% | +0.9% | +22.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.036 | ok |
| wide-rung-ragged | -0.153 | ok |
| wide-rung-ragged-overread | -0.458 | moderate- |
| wide-rung-wordround | -0.416 | moderate- |
| wide-rung-wordround-alias | -0.109 | ok |

**Consistency summary:**

- **wide-rung-ragged**: won 1/4, lost 3/4
- **wide-rung-ragged-overread**: won 1/4, lost 3/4
- **wide-rung-wordround**: won 2/4, lost 1/4
- **wide-rung-wordround-alias**: won 1/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 22.4ns | 8581.1ns | 0.3% |  |
| wide-rung-ragged | 21.3ns | 7764.2ns | 0.3% |  |
| wide-rung-ragged-overread | 23.9ns | 9306.6ns | 0.3% |  |
| wide-rung-wordround | 20.9ns | 7776.1ns | 0.3% |  |
| wide-rung-wordround-alias | 24.1ns | 8867.0ns | 0.3% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 7210.6-10457.6 ns)
   7210.6 |##############################
   7372.9 |
   7535.3 |
   7697.6 |
   7860.0 |
   8022.3 |###
   8184.7 |
   8347.0 |
   8509.4 |
   8671.7 |######
   8834.1 |########################################
   8996.5 |
   9158.8 |###
   9321.2 |
   9483.5 |###
   9645.9 |
   9808.2 |##########
   9970.6 |
  10132.9 |
  10295.3 |###
  (6 below, 4 above range)

wide-rung-ragged (n=40, range 7225.5-9058.5 ns)
   7225.5 |##############
   7317.1 |########################################
   7408.8 |##
   7500.4 |
   7592.1 |
   7683.7 |
   7775.4 |
   7867.0 |
   7958.7 |
   8050.3 |
   8142.0 |
   8233.6 |
   8325.3 |
   8416.9 |
   8508.6 |
   8600.2 |
   8691.9 |
   8783.5 |
   8875.2 |
   8966.8 |############
  (3 below, 4 above range)

wide-rung-ragged-overread (n=40, range 9227.2-9345.3 ns)
   9227.2 |################
   9233.1 |
   9239.0 |
   9244.9 |
   9250.8 |
   9256.7 |##
   9262.6 |
   9268.5 |
   9274.4 |
   9280.3 |
   9286.2 |
   9292.2 |
   9298.1 |
   9304.0 |
   9309.9 |##
   9315.8 |##
   9321.7 |########################################
   9327.6 |##
   9333.5 |
   9339.4 |########################
  (3 below, 3 above range)

wide-rung-wordround (n=40, range 7259.1-8965.7 ns)
   7259.1 |########################################
   7344.4 |
   7429.7 |
   7515.1 |#
   7600.4 |
   7685.7 |#
   7771.1 |#
   7856.4 |
   7941.7 |#
   8027.1 |
   8112.4 |
   8197.7 |
   8283.1 |
   8368.4 |
   8453.7 |
   8539.1 |
   8624.4 |
   8709.7 |
   8795.1 |###
   8880.4 |###############
  (4 below, 1 above range)

wide-rung-wordround-alias (n=40, range 8796.2-8921.7 ns)
   8796.2 |##############
   8802.5 |
   8808.8 |
   8815.1 |##
   8821.3 |
   8827.6 |
   8833.9 |
   8840.2 |
   8846.4 |
   8852.7 |
   8859.0 |
   8865.3 |######
   8871.5 |######
   8877.8 |########################################
   8884.1 |####
   8890.3 |
   8896.6 |
   8902.9 |
   8909.2 |
   8915.4 |##
  (2 below, 1 above range)

```
