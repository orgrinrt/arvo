# Wide rung, payload-shape sweep, cache-resident (2048 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (wide-rung-wordround-alias, wide-rung-wordround) are a dead heat (<1%)

wide-rung-wordround-alias (3.93 us) and wide-rung-wordround (3.97 us) differ by 0.98%, inside the noise, even though the wider field spreads 5.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### wide-rung-wordround shows warm-up / thermal drift (autocorr +0.82)

wide-rung-wordround's per-pass series has lag-1 autocorrelation +0.82, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### wide-rung-ragged-overread's edge over baseline is significant but tiny (44 ns, 1.11%)

wide-rung-ragged-overread differs from baseline wide-rung-align16 by 44 ns (1.11%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-wordround-alias** at 3929.4 ns median (-1.5% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.05x (fastest 3929.4 ns, slowest 4138.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 4079ns | 4053ns | 3988ns | 4060ns | 4224ns | base |
| wide-rung-ragged | 4238ns | 4216ns | 4196ns | 4222ns | 4328ns | +3.90% |
| wide-rung-ragged-overread | 4066ns | 4039ns | 3981ns | 4050ns | 4197ns | -0.31% |
| wide-rung-wordround | 4041ns | 4036ns | 3981ns | 4025ns | 4148ns | -0.93% |
| wide-rung-wordround-alias | 4049ns | 3994ns | 3977ns | 4015ns | 4222ns | -0.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 4006ns | 3923ns | 4121ns | base | 2.045 |
| wide-rung-ragged | 4170ns | 4133ns | 4260ns | +4.11% | 1.964 |
| wide-rung-ragged-overread | 4000ns | 3916ns | 4130ns | -0.14% | 2.048 |
| wide-rung-wordround | 3973ns | 3916ns | 4080ns | -0.83% | 2.062 |
| wide-rung-wordround-alias | 3983ns | 3914ns | 4154ns | -0.56% | 2.057 |

## Performance model

- Peak throughput: **2.093 Gops/s** (wide-rung-wordround-alias; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 2.053 | 98.1% |
| wide-rung-ragged | 1.979 | 94.6% |
| wide-rung-ragged-overread | 2.063 | 98.6% |
| wide-rung-wordround | 2.064 | 98.6% |
| wide-rung-wordround-alias | 2.085 | 99.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 4079ns | 4079ns | base |
| wide-rung-ragged | 4238ns | 4238ns | +3.90% |
| wide-rung-ragged-overread | 4066ns | 4066ns | -0.31% |
| wide-rung-wordround | 4041ns | 4041ns | -0.93% |
| wide-rung-wordround-alias | 4049ns | 4049ns | -0.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 3990ns | base | --- | [3983, 4014] | --- | --- | --- | --- |
| wide-rung-ragged | 4139ns | +189.8ns (+4.8%) | [+141, +209]ns | [4136, 4171] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 3971ns | no significant difference | [-38, +9]ns | [3933, 3990] | no | 0.1076 | 0.0807 | 0 |
| wide-rung-wordround | 3968ns | no significant difference | [-86, +42]ns | [3924, 3977] | no | 0.4296 | 0.4296 | 0 |
| wide-rung-wordround-alias | 3929ns | -66.7ns (-1.7%) | [-83, -16]ns | [3920, 3978] | YES (adj: no) | 0.0770 | 0.0385 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 3985ns | +5.7% | +4.2% | +3.4% | -1.7% |
| 2 | 3986ns | +5.8% | +0.5% | +3.4% | -1.7% |
| 3 | 3983ns | +11.9% | +3.4% | +3.4% | -1.7% |
| 4 | 3985ns | +4.6% | +3.4% | +3.3% | -1.8% |
| 5 | 3988ns | +5.9% | +3.4% | +3.4% | -1.9% |
| 6 | 3991ns | +5.4% | +3.5% | +1.0% | +5.0% |
| 7 | 3994ns | +4.6% | +3.5% | -0.4% | -1.7% |
| 8 | 3992ns | +5.2% | +3.3% | -0.2% | -1.9% |
| 9 | 3985ns | +5.3% | +3.4% | +0.1% | +0.6% |
| 10 | 3982ns | +7.1% | +3.5% | -0.1% | -1.6% |
| 11 | 4014ns | +3.0% | -1.0% | -2.4% | -0.9% |
| 12 | 4147ns | +3.1% | -4.0% | -5.5% | -3.5% |
| 13 | 4004ns | +3.2% | -0.9% | -2.2% | -0.8% |
| 14 | 3983ns | +3.9% | -0.2% | -1.6% | -0.1% |
| 15 | 4004ns | +3.2% | -0.2% | -2.1% | -2.2% |
| 16 | 4171ns | -0.9% | -4.5% | -5.9% | -6.2% |
| 17 | 4035ns | +3.8% | -1.2% | -3.0% | -2.9% |
| 18 | 4013ns | +3.8% | -0.7% | -2.4% | -2.4% |
| 19 | 3979ns | +3.9% | -0.2% | -1.6% | -1.5% |
| 20 | 3927ns | +5.3% | -0.1% | -0.3% | +1.3% |
| 21 | 4092ns | +1.0% | -4.3% | -2.0% | -4.0% |
| 22 | 4022ns | +4.8% | -2.6% | -1.5% | -1.6% |
| 23 | 4095ns | +0.9% | -4.5% | -4.3% | -4.0% |
| 24 | 4055ns | +2.0% | -3.3% | -2.5% | -3.5% |
| 25 | 4123ns | +0.3% | -4.6% | -5.1% | -5.0% |
| 26 | 4075ns | +1.8% | -3.2% | -3.8% | -3.9% |
| 27 | 4051ns | +2.2% | -2.8% | -2.5% | -3.3% |
| 28 | 4117ns | +0.5% | -4.7% | -4.8% | -4.8% |
| 29 | 4052ns | +2.1% | -3.5% | -2.5% | -2.3% |
| 30 | 4145ns | +0.8% | -5.2% | -5.5% | -5.6% |
| 31 | 3927ns | +5.3% | +4.9% | +0.4% | +1.2% |
| 32 | 3920ns | +5.5% | +5.3% | +1.4% | +4.4% |
| 33 | 3918ns | +5.5% | +5.1% | +1.4% | +5.2% |
| 34 | 3922ns | +5.8% | +1.2% | +1.3% | +5.1% |
| 35 | 3924ns | +5.4% | -0.1% | +1.3% | +4.4% |
| 36 | 3925ns | +5.4% | -0.0% | +1.5% | +3.9% |
| 37 | 3934ns | +5.3% | -0.1% | +1.1% | +14.3% |
| 38 | 3926ns | +5.3% | -0.1% | +1.3% | +2.3% |
| 39 | 3937ns | +5.0% | -0.6% | +1.4% | +1.5% |
| 40 | 3924ns | +6.7% | -0.2% | +1.3% | +2.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.467 | moderate+ |
| wide-rung-ragged | 0.176 | ok |
| wide-rung-ragged-overread | 0.744 | HIGH+ (drift/warm-up) |
| wide-rung-wordround | 0.816 | HIGH+ (drift/warm-up) |
| wide-rung-wordround-alias | 0.349 | moderate+ |

**Consistency summary:**

- **wide-rung-ragged**: won 1/40, lost 39/40
- **wide-rung-ragged-overread**: won 24/40, lost 14/40
- **wide-rung-wordround**: won 22/40, lost 16/40
- **wide-rung-wordround-alias**: won 26/40, lost 13/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 2.8ns | 4005.8ns | 0.1% |  |
| wide-rung-ragged | 2.3ns | 4170.4ns | 0.1% |  |
| wide-rung-ragged-overread | 2.4ns | 4000.1ns | 0.1% |  |
| wide-rung-wordround | 2.5ns | 3972.8ns | 0.1% |  |
| wide-rung-wordround-alias | 2.4ns | 3983.4ns | 0.1% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 3923.2-4120.6 ns)
   3923.2 |##############################
   3933.1 |##########
   3943.0 |
   3952.8 |
   3962.7 |
   3972.6 |##########
   3982.5 |########################################
   3992.3 |##########
   4002.2 |##########
   4012.1 |##########
   4021.9 |#####
   4031.8 |#####
   4041.7 |#####
   4051.5 |##########
   4061.4 |
   4071.3 |#####
   4081.1 |
   4091.0 |##########
   4100.9 |
   4110.8 |#####
  (3 below, 4 above range)

wide-rung-ragged (n=40, range 4133.1-4259.6 ns)
   4133.1 |########################################
   4139.4 |#####
   4145.8 |#####
   4152.1 |
   4158.4 |
   4164.7 |#####
   4171.1 |##
   4177.4 |##
   4183.7 |#####
   4190.0 |
   4196.4 |#####
   4202.7 |##
   4209.0 |##
   4215.4 |#####
   4221.7 |##
   4228.0 |
   4234.3 |
   4240.7 |
   4247.0 |
   4253.3 |
  (4 below, 3 above range)

wide-rung-ragged-overread (n=40, range 3916.0-4129.9 ns)
   3916.0 |########################################
   3926.7 |####################
   3937.4 |#####
   3948.1 |
   3958.8 |
   3969.5 |##############################
   3980.2 |###############
   3990.9 |#####
   4001.6 |#####
   4012.3 |
   4023.0 |
   4033.7 |
   4044.4 |
   4055.0 |
   4065.7 |
   4076.4 |
   4087.1 |
   4097.8 |
   4108.5 |#####
   4119.2 |########################################
  (4 below, 3 above range)

wide-rung-wordround (n=40, range 3916.1-4079.6 ns)
   3916.1 |########################################
   3924.3 |###
   3932.4 |
   3940.6 |#######
   3948.8 |#######
   3957.0 |###
   3965.1 |###
   3973.3 |#############################
   3981.5 |##########
   3989.7 |###
   3997.9 |
   4006.0 |###
   4014.2 |
   4022.4 |
   4030.6 |###
   4038.7 |
   4046.9 |
   4055.1 |
   4063.3 |
   4071.5 |
  (3 below, 5 above range)

wide-rung-wordround-alias (n=40, range 3914.3-4153.6 ns)
   3914.3 |########################################
   3926.3 |#####
   3938.2 |
   3950.2 |#####
   3962.2 |##
   3974.1 |###########
   3986.1 |##
   3998.1 |#####
   4010.0 |##
   4022.0 |##
   4033.9 |
   4045.9 |
   4057.9 |
   4069.8 |##
   4081.8 |
   4093.8 |#####
   4105.7 |
   4117.7 |#####
   4129.6 |
   4141.6 |
  (5 below, 2 above range)

```

## Diagnostics

- **wide-rung-ragged-overread**: autocorrelation=0.74 (measurement drift or warm-up artifact)
- **wide-rung-wordround**: autocorrelation=0.82 (measurement drift or warm-up artifact)
