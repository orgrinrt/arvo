# Wide rung, payload-shape sweep, cache-resident (2048 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (wide-rung-wordround, wide-rung-ragged-overread) are a dead heat (<1%)

wide-rung-wordround (3.65 us) and wide-rung-ragged-overread (3.65 us) differ by 0.05%, inside the noise, even though the wider field spreads 7.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### wide-rung-ragged shows warm-up / thermal drift (autocorr +0.88)

wide-rung-ragged's per-pass series has lag-1 autocorrelation +0.88, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### wide-rung-ragged-overread's edge over baseline is significant but tiny (-10 ns, 0.27%)

wide-rung-ragged-overread differs from baseline wide-rung-align16 by -10 ns (0.27%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-wordround** at 3651.0 ns median (-2.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.07x (fastest 3651.0 ns, slowest 3910.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 3866ns | 3790ns | 3721ns | 3807ns | 4189ns | base |
| wide-rung-ragged | 3959ns | 3977ns | 3798ns | 3935ns | 4194ns | +2.40% |
| wide-rung-ragged-overread | 3797ns | 3721ns | 3707ns | 3763ns | 3992ns | -1.78% |
| wide-rung-wordround | 3739ns | 3713ns | 3708ns | 3715ns | 3841ns | -3.29% |
| wide-rung-wordround-alias | 3806ns | 3735ns | 3709ns | 3775ns | 3996ns | -1.56% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 3798ns | 3659ns | 4100ns | base | 2.157 |
| wide-rung-ragged | 3892ns | 3732ns | 4123ns | +2.46% | 2.105 |
| wide-rung-ragged-overread | 3733ns | 3644ns | 3926ns | -1.72% | 2.194 |
| wide-rung-wordround | 3675ns | 3648ns | 3773ns | -3.24% | 2.229 |
| wide-rung-wordround-alias | 3741ns | 3647ns | 3928ns | -1.53% | 2.190 |

## Performance model

- Peak throughput: **2.248 Gops/s** (wide-rung-ragged-overread; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 2.198 | 97.8% |
| wide-rung-ragged | 2.095 | 93.2% |
| wide-rung-ragged-overread | 2.243 | 99.8% |
| wide-rung-wordround | 2.244 | 99.8% |
| wide-rung-wordround-alias | 2.232 | 99.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 3866ns | 3866ns | base |
| wide-rung-ragged | 3959ns | 3959ns | +2.40% |
| wide-rung-ragged-overread | 3797ns | 3797ns | -1.78% |
| wide-rung-wordround | 3739ns | 3739ns | -3.29% |
| wide-rung-wordround-alias | 3806ns | 3806ns | -1.56% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 3728ns | base | --- | [3666, 3789] | --- | --- | --- | --- |
| wide-rung-ragged | 3911ns | no significant difference | [-11, +121]ns | [3792, 3917] | no | 0.2682 | 0.2682 | 0 |
| wide-rung-ragged-overread | 3653ns | no significant difference | [-92, +23]ns | [3648, 3742] | no | 0.2051 | 0.1539 | 0 |
| wide-rung-wordround | 3651ns | -72.5ns (-1.9%) | [-139, -14]ns | [3650, 3652] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround-alias | 3670ns | -13.3ns (-0.4%) | [-48, -3]ns | [3652, 3764] | YES (adj: no) | 0.1614 | 0.0807 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 3708ns | +13.7% | -1.7% | +5.4% | -1.5% |
| 2 | 3660ns | +15.1% | +3.8% | +7.1% | -0.3% |
| 3 | 3659ns | +15.2% | +6.9% | +4.5% | -0.3% |
| 4 | 3662ns | +15.0% | +6.7% | +2.4% | +1.5% |
| 5 | 3664ns | +15.1% | +6.7% | -0.4% | -0.4% |
| 6 | 3662ns | +10.6% | +6.8% | -0.2% | -0.4% |
| 7 | 3667ns | +6.7% | +9.8% | -0.6% | +0.1% |
| 8 | 3663ns | +6.9% | +6.8% | -0.3% | -0.4% |
| 9 | 3658ns | +6.9% | +0.8% | -0.3% | +0.8% |
| 10 | 3662ns | +6.8% | +2.3% | -0.4% | -0.2% |
| 11 | 3790ns | +3.2% | +3.1% | -3.7% | +3.2% |
| 12 | 3788ns | +3.2% | +3.2% | -3.7% | +5.3% |
| 13 | 3786ns | +1.7% | +3.4% | -3.7% | +3.4% |
| 14 | 3788ns | -1.9% | +1.0% | -3.6% | +3.3% |
| 15 | 3787ns | -0.2% | -3.8% | -3.6% | +3.3% |
| 16 | 3789ns | -1.5% | -3.7% | -3.7% | +3.4% |
| 17 | 3793ns | -2.0% | -3.9% | -1.9% | +1.2% |
| 18 | 3773ns | -1.5% | -3.2% | -3.3% | +0.0% |
| 19 | 3655ns | +1.7% | -0.3% | +0.3% | +3.2% |
| 20 | 3663ns | +1.4% | -0.5% | -0.4% | +3.2% |
| 21 | 3804ns | -0.3% | -0.8% | -4.0% | -1.3% |
| 22 | 3729ns | +1.5% | +1.3% | -2.1% | +0.4% |
| 23 | 3722ns | +1.4% | +0.4% | -2.0% | -1.9% |
| 24 | 3726ns | +1.3% | -1.3% | -0.7% | -2.1% |
| 25 | 3696ns | +2.0% | -1.4% | -1.3% | -1.2% |
| 26 | 3660ns | +3.2% | -0.3% | -0.3% | -0.4% |
| 27 | 3668ns | +4.6% | -0.6% | -0.5% | -0.6% |
| 28 | 3664ns | +3.5% | -0.4% | -0.4% | -0.3% |
| 29 | 3660ns | +3.1% | -0.2% | +0.6% | -0.3% |
| 30 | 3660ns | +5.0% | -0.2% | -0.2% | -0.4% |
| 31 | 4312ns | -8.8% | -15.5% | -15.1% | -8.9% |
| 32 | 4006ns | -2.1% | -9.0% | -8.8% | -1.9% |
| 33 | 4350ns | -10.0% | -14.4% | -16.0% | -9.8% |
| 34 | 4380ns | -10.6% | -16.8% | -16.6% | -13.0% |
| 35 | 3935ns | -0.4% | -6.7% | -7.3% | -6.9% |
| 36 | 3935ns | -0.3% | -7.3% | -6.5% | -7.0% |
| 37 | 3944ns | -0.5% | -7.5% | -7.4% | -7.0% |
| 38 | 3935ns | -0.3% | -7.3% | -7.2% | -7.0% |
| 39 | 3935ns | -0.5% | -7.5% | -7.3% | -7.2% |
| 40 | 3939ns | -0.5% | -7.4% | -7.3% | -6.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.669 | HIGH+ (drift/warm-up) |
| wide-rung-ragged | 0.880 | HIGH+ (drift/warm-up) |
| wide-rung-ragged-overread | 0.773 | HIGH+ (drift/warm-up) |
| wide-rung-wordround | 0.695 | HIGH+ (drift/warm-up) |
| wide-rung-wordround-alias | 0.763 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **wide-rung-ragged**: won 16/40, lost 24/40
- **wide-rung-ragged-overread**: won 25/40, lost 15/40
- **wide-rung-wordround**: won 34/40, lost 6/40
- **wide-rung-wordround-alias**: won 26/40, lost 13/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 2.4ns | 3798.5ns | 0.1% |  |
| wide-rung-ragged | 2.2ns | 3891.8ns | 0.1% |  |
| wide-rung-ragged-overread | 2.5ns | 3733.1ns | 0.1% |  |
| wide-rung-wordround | 2.3ns | 3675.4ns | 0.1% |  |
| wide-rung-wordround-alias | 2.2ns | 3740.5ns | 0.1% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 3659.2-4100.3 ns)
   3659.2 |########################################
   3681.3 |###
   3703.3 |######
   3725.4 |######
   3747.4 |
   3769.5 |#####################
   3791.5 |######
   3813.6 |
   3835.6 |
   3857.7 |
   3879.7 |
   3901.8 |
   3923.8 |##################
   3945.9 |
   3968.0 |
   3990.0 |###
   4012.1 |
   4034.1 |
   4056.2 |
   4078.2 |
  (3 below, 3 above range)

wide-rung-ragged (n=40, range 3731.7-4123.1 ns)
   3731.7 |##
   3751.2 |##
   3770.8 |################
   3790.4 |#####
   3809.9 |
   3829.5 |#####
   3849.1 |##
   3868.7 |
   3888.2 |
   3907.8 |########################################
   3927.4 |##
   3946.9 |
   3966.5 |
   3986.1 |
   4005.6 |
   4025.2 |
   4044.8 |##
   4064.4 |
   4083.9 |
   4103.5 |
  (5 below, 5 above range)

wide-rung-ragged-overread (n=40, range 3643.9-3925.6 ns)
   3643.9 |########################################
   3658.0 |##
   3672.1 |##
   3686.2 |##
   3700.3 |
   3714.3 |##
   3728.4 |##
   3742.5 |##
   3756.6 |
   3770.7 |####
   3784.8 |
   3798.8 |##
   3812.9 |
   3827.0 |##
   3841.1 |
   3855.2 |
   3869.3 |
   3883.4 |
   3897.4 |############
   3911.5 |####
  (2 below, 1 above range)

wide-rung-wordround (n=40, range 3647.7-3773.1 ns)
   3647.7 |########################################
   3654.0 |#
   3660.2 |###
   3666.5 |
   3672.8 |
   3679.0 |###
   3685.3 |
   3691.6 |
   3697.8 |#
   3704.1 |
   3710.4 |
   3716.7 |#
   3722.9 |
   3729.2 |
   3735.5 |
   3741.7 |
   3748.0 |#
   3754.3 |
   3760.5 |
   3766.8 |
  (3 below, 3 above range)

wide-rung-wordround-alias (n=40, range 3647.1-3928.1 ns)
   3647.1 |########################################
   3661.2 |############
   3675.2 |###
   3689.3 |###
   3703.3 |###
   3717.4 |
   3731.4 |###
   3745.5 |###
   3759.5 |
   3773.6 |#########
   3787.6 |
   3801.7 |###
   3815.7 |
   3829.8 |###
   3843.8 |
   3857.9 |
   3871.9 |
   3886.0 |
   3900.0 |#########
   3914.1 |#########
  (4 below, 3 above range)

```

## Diagnostics

- **wide-rung-align16**: autocorrelation=0.67 (measurement drift or warm-up artifact)
- **wide-rung-ragged**: autocorrelation=0.88 (measurement drift or warm-up artifact)
- **wide-rung-ragged-overread**: autocorrelation=0.77 (measurement drift or warm-up artifact)
- **wide-rung-wordround**: autocorrelation=0.69 (measurement drift or warm-up artifact)
- **wide-rung-wordround-alias**: autocorrelation=0.76 (measurement drift or warm-up artifact)
