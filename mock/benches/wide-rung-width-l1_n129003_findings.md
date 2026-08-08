# Wide rung, payload-shape sweep, cache-resident (2048 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-wordround shows warm-up / thermal drift (autocorr +0.78)

wide-rung-wordround's per-pass series has lag-1 autocorrelation +0.78, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (243 ns) is smaller than the fastest variant's own run-to-run std-dev (2.92 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader wide-rung-wordround-alias vs stability leader wide-rung-align16 (+1% speed for 29.5x steadier)

wide-rung-wordround-alias is fastest (3.89 us, CV 75.1%); wide-rung-align16 gives up 1.2% median for 29.5x lower variance (CV 2.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### wide-rung-wordround is inconsistent: worst-20% is 2.7x its best-20%

wide-rung-wordround's best 20% of batches run at 3.88 us but its worst 20% at 10.45 us (2.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### wide-rung-wordround-alias's edge over baseline is significant but tiny (-27 ns, 0.68%)

wide-rung-wordround-alias differs from baseline wide-rung-align16 by -27 ns (0.68%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-wordround-alias** at 3890.8 ns median (-1.2% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.06x (fastest 3890.8 ns, slowest 4133.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 4059ns | 4004ns | 3973ns | 4029ns | 4234ns | base |
| wide-rung-ragged | 4322ns | 4198ns | 4191ns | 4300ns | 4522ns | +6.50% |
| wide-rung-ragged-overread | 4091ns | 4099ns | 3953ns | 4086ns | 4244ns | +0.80% |
| wide-rung-wordround | 5368ns | 4100ns | 3945ns | 4089ns | 10630ns | +32.27% |
| wide-rung-wordround-alias | 4851ns | 3959ns | 3941ns | 3988ns | 8350ns | +19.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 3994ns | 3910ns | 4167ns | base | 2.051 |
| wide-rung-ragged | 4258ns | 4129ns | 4453ns | +6.61% | 1.924 |
| wide-rung-ragged-overread | 4027ns | 3891ns | 4179ns | +0.84% | 2.034 |
| wide-rung-wordround | 5279ns | 3882ns | 10453ns | +32.18% | 1.552 |
| wide-rung-wordround-alias | 4775ns | 3880ns | 8223ns | +19.57% | 1.716 |

## Performance model

- Peak throughput: **2.111 Gops/s** (wide-rung-wordround-alias; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 2.080 | 98.5% |
| wide-rung-ragged | 1.982 | 93.9% |
| wide-rung-ragged-overread | 2.029 | 96.1% |
| wide-rung-wordround | 2.035 | 96.4% |
| wide-rung-wordround-alias | 2.105 | 99.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 4059ns | 4059ns | base |
| wide-rung-ragged | 4322ns | 4322ns | +6.50% |
| wide-rung-ragged-overread | 4091ns | 4091ns | +0.80% |
| wide-rung-wordround | 5368ns | 5368ns | +32.27% |
| wide-rung-wordround-alias | 4851ns | 4851ns | +19.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 3939ns | base | --- | [3928, 4002] | --- | --- | --- | --- |
| wide-rung-ragged | 4134ns | +217.9ns (+5.5%) | [+209, +254]ns | [4132, 4426] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 4038ns | no significant difference | [-20, +91]ns | [3938, 4068] | no | 0.8746 | 0.8746 | 0 |
| wide-rung-wordround | 4026ns | no significant difference | [-33, +91]ns | [3988, 4053] | no | 0.8746 | 0.8746 | 0 |
| wide-rung-wordround-alias | 3891ns | -40.6ns (-1.0%) | [-52, -29]ns | [3885, 3952] | YES | 0.0001 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 3906ns | +6.5% | -0.1% | +337.6% | +325.2% |
| 2 | 3918ns | +5.5% | -0.7% | +279.1% | +262.3% |
| 3 | 3930ns | +5.1% | +2.3% | +256.9% | +259.3% |
| 4 | 3905ns | +5.8% | +6.0% | +262.4% | -0.4% |
| 5 | 3921ns | +5.4% | +0.6% | +180.8% | -0.8% |
| 6 | 3950ns | +4.6% | +1.8% | -1.5% | -1.5% |
| 7 | 3933ns | +5.1% | +2.0% | -1.2% | -1.2% |
| 8 | 3926ns | +5.2% | -0.5% | -1.0% | +0.2% |
| 9 | 3937ns | +5.0% | -1.1% | -1.3% | -1.3% |
| 10 | 3910ns | +5.6% | +0.6% | -0.1% | -0.1% |
| 11 | 4190ns | +6.2% | -0.1% | -4.1% | -0.6% |
| 12 | 4198ns | +6.2% | -0.5% | -4.4% | -0.6% |
| 13 | 4223ns | +5.1% | -0.9% | -4.7% | -1.1% |
| 14 | 4190ns | +6.1% | -0.1% | -4.1% | -0.2% |
| 15 | 4200ns | +5.7% | -0.5% | -2.0% | -0.9% |
| 16 | 4092ns | +8.6% | +2.1% | +1.7% | +0.7% |
| 17 | 4065ns | +9.2% | +2.3% | +2.4% | -2.1% |
| 18 | 4063ns | +9.3% | +2.4% | +2.6% | -2.2% |
| 19 | 4045ns | +9.8% | +3.1% | +3.0% | -1.2% |
| 20 | 4041ns | +10.0% | +3.3% | +2.9% | -2.1% |
| 21 | 4044ns | +5.6% | -3.7% | -0.4% | -2.6% |
| 22 | 4055ns | +1.9% | -3.8% | -0.8% | -4.3% |
| 23 | 4042ns | +2.2% | -3.3% | -0.3% | -3.8% |
| 24 | 3964ns | +4.3% | -1.0% | +1.6% | -2.0% |
| 25 | 3916ns | +5.4% | -0.3% | +2.9% | -0.9% |
| 26 | 3948ns | +4.7% | +0.3% | +2.0% | +0.0% |
| 27 | 3935ns | +5.0% | -1.4% | +2.2% | -1.2% |
| 28 | 3962ns | +6.4% | -2.0% | +3.0% | -2.0% |
| 29 | 3919ns | +5.3% | -0.6% | +2.9% | -1.0% |
| 30 | 3918ns | +5.4% | -0.7% | +4.8% | -0.9% |
| 31 | 4178ns | +7.7% | -3.3% | -7.0% | -7.1% |
| 32 | 3919ns | +12.9% | +2.9% | -0.9% | -0.3% |
| 33 | 3904ns | +13.4% | +3.6% | -0.5% | -0.4% |
| 34 | 3907ns | +13.3% | +3.5% | -0.6% | +2.6% |
| 35 | 3949ns | +12.1% | +2.5% | -0.2% | -1.7% |
| 36 | 3919ns | +8.7% | +4.3% | -1.1% | -1.0% |
| 37 | 3942ns | +4.8% | +3.2% | +3.3% | -1.5% |
| 38 | 3937ns | +4.9% | +2.8% | +0.6% | -1.4% |
| 39 | 3917ns | +5.4% | +6.1% | -0.9% | -0.9% |
| 40 | 3929ns | +5.2% | +3.6% | -1.2% | -1.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.677 | HIGH+ (drift/warm-up) |
| wide-rung-ragged | 0.781 | HIGH+ (drift/warm-up) |
| wide-rung-ragged-overread | 0.692 | HIGH+ (drift/warm-up) |
| wide-rung-wordround | 0.782 | HIGH+ (drift/warm-up) |
| wide-rung-wordround-alias | 0.632 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **wide-rung-ragged**: won 0/40, lost 40/40
- **wide-rung-ragged-overread**: won 18/40, lost 21/40
- **wide-rung-wordround**: won 20/40, lost 19/40
- **wide-rung-wordround-alias**: won 32/40, lost 6/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 2.1ns | 3993.7ns | 0.1% |  |
| wide-rung-ragged | 2.1ns | 4257.7ns | 0.0% |  |
| wide-rung-ragged-overread | 2.4ns | 4027.2ns | 0.1% |  |
| wide-rung-wordround | 3.0ns | 5278.6ns | 0.1% |  |
| wide-rung-wordround-alias | 2.6ns | 4775.2ns | 0.1% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 3910.2-4167.1 ns)
   3910.2 |########################################
   3923.0 |#########################
   3935.9 |####################
   3948.7 |##########
   3961.6 |##########
   3974.4 |
   3987.3 |
   4000.1 |
   4013.0 |
   4025.8 |
   4038.6 |####################
   4051.5 |##########
   4064.3 |#####
   4077.2 |
   4090.0 |#####
   4102.9 |
   4115.7 |
   4128.5 |
   4141.4 |
   4154.2 |
  (5 below, 6 above range)

wide-rung-ragged (n=40, range 4129.2-4453.2 ns)
   4129.2 |########################################
   4145.4 |##
   4161.6 |
   4177.8 |
   4194.0 |
   4210.2 |##
   4226.4 |
   4242.6 |
   4258.8 |####
   4275.0 |
   4291.2 |
   4307.4 |
   4323.6 |
   4339.8 |
   4356.0 |
   4372.2 |
   4388.4 |
   4404.6 |
   4420.8 |########
   4437.0 |####################
  (3 below, 2 above range)

wide-rung-ragged-overread (n=40, range 3890.9-4179.2 ns)
   3890.9 |########################################
   3905.3 |#############
   3919.7 |#############
   3934.1 |######
   3948.5 |######
   3963.0 |
   3977.4 |
   3991.8 |
   4006.2 |#############
   4020.6 |#############
   4035.0 |#################################
   4049.5 |
   4063.9 |#############
   4078.3 |######
   4092.7 |
   4107.1 |
   4121.6 |
   4136.0 |######
   4150.4 |####################
   4164.8 |##########################
  (4 below, 4 above range)

wide-rung-wordround (n=40, range 3882.0-10453.1 ns)
   3882.0 |########################################
   4210.6 |
   4539.1 |
   4867.7 |
   5196.3 |
   5524.8 |
   5853.4 |
   6181.9 |
   6510.5 |
   6839.0 |
   7167.6 |
   7496.1 |
   7824.7 |
   8153.2 |
   8481.8 |
   8810.4 |
   9138.9 |
   9467.5 |
   9796.0 |
  10124.6 |
  (3 below, 5 above range)

wide-rung-wordround-alias (n=40, range 3879.9-8222.6 ns)
   3879.9 |########################################
   4097.0 |########
   4314.2 |
   4531.3 |
   4748.4 |
   4965.6 |
   5182.7 |
   5399.8 |
   5617.0 |
   5834.1 |
   6051.3 |
   6268.4 |
   6485.5 |
   6702.7 |
   6919.8 |
   7137.0 |
   7354.1 |
   7571.2 |
   7788.4 |
   8005.5 |
  (3 below, 3 above range)

```

## Diagnostics

- **wide-rung-align16**: autocorrelation=0.68 (measurement drift or warm-up artifact)
- **wide-rung-ragged**: autocorrelation=0.78 (measurement drift or warm-up artifact)
- **wide-rung-ragged-overread**: autocorrelation=0.69 (measurement drift or warm-up artifact)
- **wide-rung-wordround**: CV=65.4% (high variance, measurements may be unstable)
- **wide-rung-wordround**: autocorrelation=0.78 (measurement drift or warm-up artifact)
- **wide-rung-wordround-alias**: CV=61.2% (high variance, measurements may be unstable)
- **wide-rung-wordround-alias**: autocorrelation=0.63 (measurement drift or warm-up artifact)
