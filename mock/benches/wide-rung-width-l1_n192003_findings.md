# Wide rung, payload-shape sweep, cache-resident (2048 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (wide-rung-wordround, wide-rung-align16) are a dead heat (<1%)

wide-rung-wordround (3.72 us) and wide-rung-align16 (3.74 us) differ by 0.61%, inside the noise, even though the wider field spreads 3.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### wide-rung-ragged shows warm-up / thermal drift (autocorr +0.77)

wide-rung-ragged's per-pass series has lag-1 autocorrelation +0.77, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole field within 3.9% of the fastest

All 5 variants sit between 3.72 us and 3.87 us - a 3.9% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### wide-rung-ragged-overread's edge over baseline is significant but tiny (-16 ns, 0.44%)

wide-rung-ragged-overread differs from baseline wide-rung-align16 by -16 ns (0.44%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: wide-rung-wordround** at 3720.4 ns median (-0.6% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.04x (fastest 3720.4 ns, slowest 3866.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 3855ns | 3810ns | 3795ns | 3826ns | 4004ns | base |
| wide-rung-ragged | 3920ns | 3932ns | 3791ns | 3924ns | 4039ns | +1.69% |
| wide-rung-ragged-overread | 3829ns | 3839ns | 3783ns | 3826ns | 3882ns | -0.68% |
| wide-rung-wordround | 3840ns | 3788ns | 3776ns | 3816ns | 3978ns | -0.39% |
| wide-rung-wordround-alias | 3888ns | 3854ns | 3827ns | 3876ns | 3982ns | +0.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 3789ns | 3731ns | 3932ns | base | 2.162 |
| wide-rung-ragged | 3854ns | 3728ns | 3970ns | +1.72% | 2.126 |
| wide-rung-ragged-overread | 3764ns | 3720ns | 3815ns | -0.65% | 2.176 |
| wide-rung-wordround | 3776ns | 3713ns | 3913ns | -0.35% | 2.170 |
| wide-rung-wordround-alias | 3821ns | 3763ns | 3913ns | +0.86% | 2.144 |

## Performance model

- Peak throughput: **2.206 Gops/s** (wide-rung-wordround; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 2.189 | 99.2% |
| wide-rung-ragged | 2.119 | 96.0% |
| wide-rung-ragged-overread | 2.170 | 98.4% |
| wide-rung-wordround | 2.202 | 99.8% |
| wide-rung-wordround-alias | 2.162 | 98.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 3855ns | 3855ns | base |
| wide-rung-ragged | 3920ns | 3920ns | +1.69% |
| wide-rung-ragged-overread | 3829ns | 3829ns | -0.68% |
| wide-rung-wordround | 3840ns | 3840ns | -0.39% |
| wide-rung-wordround-alias | 3888ns | 3888ns | +0.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 3743ns | base | --- | [3736, 3792] | --- | --- | --- | --- |
| wide-rung-ragged | 3867ns | +59.3ns (+1.6%) | [+28, +121]ns | [3794, 3916] | YES (adj: no) | 0.0513 | 0.0166 | 0 |
| wide-rung-ragged-overread | 3775ns | no significant difference | [-18, +7]ns | [3746, 3778] | no | 0.1539 | 0.1539 | 0 |
| wide-rung-wordround | 3720ns | -20.8ns (-0.6%) | [-35, -16]ns | [3717, 3781] | YES (adj: no) | 0.0513 | 0.0385 | 0 |
| wide-rung-wordround-alias | 3790ns | +37.9ns (+1.0%) | [+11, +40]ns | [3775, 3854] | YES (adj: no) | 0.0513 | 0.0385 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 3728ns | +1.6% | +0.2% | +0.0% | +4.0% |
| 2 | 3737ns | +2.2% | +0.2% | -0.6% | +1.0% |
| 3 | 3731ns | +1.5% | -0.2% | -0.4% | +4.6% |
| 4 | 3735ns | +2.3% | -0.2% | -0.5% | +3.0% |
| 5 | 3735ns | +1.9% | -0.3% | -0.6% | +1.0% |
| 6 | 3794ns | -0.2% | -2.0% | -2.0% | -0.1% |
| 7 | 3736ns | +1.7% | -0.5% | -0.6% | +1.9% |
| 8 | 3737ns | +1.6% | -0.2% | -0.7% | +3.3% |
| 9 | 3750ns | +1.1% | -0.9% | -1.0% | +0.5% |
| 10 | 3735ns | +1.2% | -0.5% | -0.5% | +1.0% |
| 11 | 3789ns | +3.4% | -0.3% | -1.9% | -0.4% |
| 12 | 3793ns | +3.2% | -0.5% | -2.1% | -0.6% |
| 13 | 3791ns | +3.4% | -0.4% | -2.0% | -0.4% |
| 14 | 3792ns | +3.2% | -0.4% | -2.0% | -0.5% |
| 15 | 3736ns | +4.8% | +1.2% | -0.6% | +1.0% |
| 16 | 3736ns | +4.8% | +1.1% | -0.5% | +2.3% |
| 17 | 3736ns | +4.8% | +1.1% | +0.8% | +1.0% |
| 18 | 3732ns | +6.7% | +1.2% | -0.4% | +1.1% |
| 19 | 3736ns | +4.8% | +3.1% | -0.6% | +1.0% |
| 20 | 3732ns | +4.8% | +1.9% | -0.4% | +1.1% |
| 21 | 4032ns | -2.5% | -5.7% | -6.0% | -3.1% |
| 22 | 4045ns | -3.0% | -6.2% | -5.8% | -3.2% |
| 23 | 3988ns | +0.6% | -5.0% | -3.0% | -1.8% |
| 24 | 3946ns | -0.5% | -3.1% | -3.7% | -0.9% |
| 25 | 4002ns | -1.9% | -5.0% | -2.4% | -2.3% |
| 26 | 3793ns | +5.0% | -1.2% | +0.2% | +3.3% |
| 27 | 3792ns | +5.8% | -1.1% | -1.8% | +3.3% |
| 28 | 3798ns | +3.4% | -1.8% | -2.1% | +3.0% |
| 29 | 3793ns | +3.6% | -1.8% | +0.7% | +3.1% |
| 30 | 3839ns | +3.8% | -2.4% | -3.2% | -3.0% |
| 31 | 3735ns | -0.5% | -0.4% | +4.7% | +0.9% |
| 32 | 3734ns | -0.3% | +0.6% | +4.7% | +1.0% |
| 33 | 3730ns | -0.3% | +1.4% | +4.8% | +1.1% |
| 34 | 3795ns | -1.5% | +1.6% | +2.9% | -0.5% |
| 35 | 3792ns | -1.9% | -0.4% | +3.0% | +0.1% |
| 36 | 3750ns | +0.9% | +0.7% | +4.2% | +1.1% |
| 37 | 3736ns | -0.4% | +1.2% | +5.7% | +3.5% |
| 38 | 3731ns | -0.3% | +1.1% | +1.0% | +1.7% |
| 39 | 3731ns | +1.0% | +1.2% | +1.0% | +3.6% |
| 40 | 3808ns | +0.2% | -1.7% | -0.9% | -1.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.693 | HIGH+ (drift/warm-up) |
| wide-rung-ragged | 0.768 | HIGH+ (drift/warm-up) |
| wide-rung-ragged-overread | 0.644 | HIGH+ (drift/warm-up) |
| wide-rung-wordround | 0.708 | HIGH+ (drift/warm-up) |
| wide-rung-wordround-alias | 0.517 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **wide-rung-ragged**: won 12/40, lost 28/40
- **wide-rung-ragged-overread**: won 25/40, lost 15/40
- **wide-rung-wordround**: won 27/40, lost 12/40
- **wide-rung-wordround-alias**: won 13/40, lost 26/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 2.8ns | 3789.0ns | 0.1% |  |
| wide-rung-ragged | 2.5ns | 3854.1ns | 0.1% |  |
| wide-rung-ragged-overread | 2.3ns | 3764.5ns | 0.1% |  |
| wide-rung-wordround | 2.5ns | 3775.6ns | 0.1% |  |
| wide-rung-wordround-alias | 2.8ns | 3821.4ns | 0.1% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 3731.1-3932.0 ns)
   3731.1 |########################################
   3741.2 |#####
   3751.2 |
   3761.3 |
   3771.3 |
   3781.4 |#####
   3791.4 |######################
   3801.4 |##
   3811.5 |
   3821.5 |
   3831.6 |##
   3841.6 |
   3851.7 |
   3861.7 |
   3871.8 |
   3881.8 |
   3891.8 |
   3901.9 |
   3911.9 |
   3922.0 |
  (4 below, 5 above range)

wide-rung-ragged (n=40, range 3727.7-3969.8 ns)
   3727.7 |####
   3739.8 |
   3751.9 |
   3764.0 |####
   3776.1 |######################
   3788.2 |########
   3800.3 |########
   3812.4 |#############
   3824.5 |
   3836.6 |
   3848.7 |
   3860.8 |
   3872.9 |
   3885.0 |
   3897.1 |
   3909.2 |########################################
   3921.4 |##########################
   3933.5 |
   3945.6 |
   3957.7 |
  (6 below, 5 above range)

wide-rung-ragged-overread (n=40, range 3720.4-3814.7 ns)
   3720.4 |########
   3725.1 |############
   3729.8 |####
   3734.5 |####
   3739.2 |
   3744.0 |############
   3748.7 |########
   3753.4 |####
   3758.1 |
   3762.8 |
   3767.6 |
   3772.3 |################
   3777.0 |########################################
   3781.7 |
   3786.4 |####
   3791.1 |####
   3795.9 |####
   3800.6 |########
   3805.3 |
   3810.0 |
  (5 below, 3 above range)

wide-rung-wordround (n=40, range 3713.4-3912.6 ns)
   3713.4 |########################################
   3723.4 |##
   3733.4 |
   3743.3 |
   3753.3 |
   3763.2 |#######
   3773.2 |##
   3783.2 |##
   3793.1 |####
   3803.1 |##
   3813.0 |##
   3823.0 |
   3833.0 |
   3842.9 |
   3852.9 |
   3862.8 |##
   3872.8 |
   3882.8 |
   3892.7 |
   3902.7 |################
  (4 below, 1 above range)

wide-rung-wordround-alias (n=40, range 3762.6-3913.4 ns)
   3762.6 |############
   3770.1 |########################################
   3777.6 |
   3785.2 |######
   3792.7 |######
   3800.3 |###
   3807.8 |
   3815.4 |
   3822.9 |###
   3830.5 |
   3838.0 |
   3845.5 |###
   3853.1 |###
   3860.6 |###
   3868.2 |###
   3875.7 |###
   3883.3 |
   3890.8 |
   3898.3 |###
   3905.9 |###############
  (2 below, 4 above range)

```

## Diagnostics

- **wide-rung-align16**: autocorrelation=0.69 (measurement drift or warm-up artifact)
- **wide-rung-ragged**: autocorrelation=0.77 (measurement drift or warm-up artifact)
- **wide-rung-ragged-overread**: autocorrelation=0.64 (measurement drift or warm-up artifact)
- **wide-rung-wordround**: autocorrelation=0.71 (measurement drift or warm-up artifact)
- **wide-rung-wordround-alias**: autocorrelation=0.52 (measurement drift or warm-up artifact)
