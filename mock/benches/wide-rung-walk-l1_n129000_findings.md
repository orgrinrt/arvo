# Wide rung, bare column walk, 2048 elements (1 wide op/element, cache-resident)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide-rung-align16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide-rung-align16 has the worst median (2.74 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide-rung-ragged at 2.06 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### wide-rung-ragged dominates: 27% faster than the next best (wide-rung-wordround-alias)

wide-rung-ragged (2.06 us) leads wide-rung-wordround-alias (2.60 us) by 27%, a clear separation rather than a photo finish. CV 4.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### wide-rung-ragged beats baseline by 24% (significant)

wide-rung-ragged is -670 ns (24%) faster than baseline wide-rung-align16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### wide-rung-wordround shows warm-up / thermal drift (autocorr +0.89)

wide-rung-wordround's per-pass series has lag-1 autocorrelation +0.89, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {wide-rung-ragged} vs {wide-rung-wordround-alias, wide-rung-wordround, wide-rung-ragged-overread, wide-rung-align16} (27% apart)

The field splits into a fast tier {wide-rung-ragged} and a slow tier {wide-rung-wordround-alias, wide-rung-wordround, wide-rung-ragged-overread, wide-rung-align16} with a 27% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### wide-rung-wordround-alias is inconsistent: worst-20% is 3.4x its best-20%

wide-rung-wordround-alias's best 20% of batches run at 2.55 us but its worst 20% at 8.54 us (3.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: wide-rung-ragged** at 2056.2 ns median (-25.0% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.33x (fastest 2056.2 ns, slowest 2741.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 3899ns | 2807ns | 2625ns | 3154ns | 7408ns | base |
| wide-rung-ragged | 2136ns | 2127ns | 2035ns | 2120ns | 2287ns | -45.21% |
| wide-rung-ragged-overread | 3333ns | 2802ns | 2616ns | 2917ns | 5296ns | -14.52% |
| wide-rung-wordround | 3284ns | 2705ns | 2610ns | 2875ns | 5185ns | -15.78% |
| wide-rung-wordround-alias | 4058ns | 2666ns | 2614ns | 2906ns | 8958ns | +4.07% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 3765ns | 2562ns | 7060ns | base | 0.544 |
| wide-rung-ragged | 2068ns | 1969ns | 2217ns | -45.08% | 0.990 |
| wide-rung-ragged-overread | 3234ns | 2555ns | 5083ns | -14.12% | 0.633 |
| wide-rung-wordround | 3186ns | 2549ns | 4977ns | -15.38% | 0.643 |
| wide-rung-wordround-alias | 3916ns | 2550ns | 8544ns | +4.00% | 0.523 |

## Performance model

- Peak throughput: **1.040 Gops/s** (wide-rung-ragged; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 0.747 | 71.8% |
| wide-rung-ragged | 0.996 | 95.7% |
| wide-rung-ragged-overread | 0.748 | 72.0% |
| wide-rung-wordround | 0.778 | 74.8% |
| wide-rung-wordround-alias | 0.787 | 75.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 3899ns | 3899ns | base |
| wide-rung-ragged | 2136ns | 2136ns | -45.21% |
| wide-rung-ragged-overread | 3333ns | 3333ns | -14.52% |
| wide-rung-wordround | 3284ns | 3284ns | -15.78% |
| wide-rung-wordround-alias | 4058ns | 4058ns | +4.07% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 2741ns | base | --- | [2737, 2948] | --- | --- | --- | --- |
| wide-rung-ragged | 2056ns | -686.8ns (-25.1%) | [-731, -676]ns | [1995, 2065] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-ragged-overread | 2736ns | -94.9ns (-3.5%) | [-307, -4]ns | [2637, 2742] | YES | 0.0221 | 0.0166 | 0 |
| wide-rung-wordround | 2634ns | -112.5ns (-4.1%) | [-190, -3]ns | [2552, 2948] | YES | 0.0044 | 0.0022 | 0 |
| wide-rung-wordround-alias | 2604ns | -138.1ns (-5.0%) | [-190, -33]ns | [2597, 2733] | YES | 0.0237 | 0.0237 | 1 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|
| 1 | 7762ns | -73.4% | -41.2% | -38.4% | +30.4% |
| 2 | 8088ns | -74.1% | -35.4% | -39.7% | +25.7% |
| 3 | 7865ns | -74.2% | -3.6% | -28.2% | +29.1% |
| 4 | 7724ns | -73.6% | -41.2% | -38.0% | +31.2% |
| 5 | 5995ns | -66.8% | -24.4% | -18.7% | +84.0% |
| 6 | 6521ns | -69.5% | -29.5% | -19.3% | -9.2% |
| 7 | 6357ns | -68.7% | -25.4% | -24.6% | -20.6% |
| 8 | 6080ns | -67.4% | -20.1% | -21.3% | -14.6% |
| 9 | 6085ns | -67.4% | -25.3% | -28.2% | -7.4% |
| 10 | 6048ns | -67.0% | -24.7% | -55.9% | -21.1% |
| 11 | 2552ns | -21.9% | +9.2% | +0.0% | -0.1% |
| 12 | 2613ns | -24.0% | -2.4% | -2.5% | -2.4% |
| 13 | 2549ns | -22.1% | +0.0% | +0.1% | +0.0% |
| 14 | 2546ns | -21.1% | +1.7% | +0.2% | +0.2% |
| 15 | 2548ns | -22.1% | +0.0% | +0.1% | +0.9% |
| 16 | 2550ns | -22.1% | +1.4% | -0.0% | +10.6% |
| 17 | 2547ns | -21.8% | +0.7% | +0.2% | +7.3% |
| 18 | 2590ns | -23.2% | -1.7% | -0.8% | +5.7% |
| 19 | 2638ns | -27.3% | -3.4% | -1.5% | +4.9% |
| 20 | 2641ns | -27.4% | -3.5% | -2.3% | +3.5% |
| 21 | 2949ns | -24.8% | -10.6% | -0.0% | -11.9% |
| 22 | 2946ns | -24.8% | -10.3% | +0.1% | -11.8% |
| 23 | 2946ns | -24.7% | -10.5% | +0.0% | -11.8% |
| 24 | 2948ns | -24.8% | -10.0% | -0.1% | -11.9% |
| 25 | 2957ns | -25.1% | -10.8% | -0.3% | -11.9% |
| 26 | 2949ns | -24.9% | -10.6% | -0.1% | -11.9% |
| 27 | 2947ns | -24.8% | -10.5% | -0.1% | -11.8% |
| 28 | 2794ns | -20.6% | -5.7% | +5.5% | -7.0% |
| 29 | 2737ns | -19.0% | -3.6% | +7.7% | -5.1% |
| 30 | 2735ns | -19.0% | -3.5% | +7.8% | -5.1% |
| 31 | 2740ns | -24.2% | -0.1% | -6.9% | -3.5% |
| 32 | 2739ns | -24.7% | +0.0% | -7.0% | -3.7% |
| 33 | 2742ns | -25.0% | -0.2% | -7.1% | -3.8% |
| 34 | 2740ns | -25.1% | +0.0% | -6.9% | -4.0% |
| 35 | 2740ns | -24.7% | -0.2% | -6.9% | -5.0% |
| 36 | 2738ns | -24.4% | +0.0% | -6.9% | -6.8% |
| 37 | 2743ns | -25.1% | -0.1% | -7.2% | -7.0% |
| 38 | 2740ns | -24.9% | +0.1% | -6.9% | -6.9% |
| 39 | 2737ns | -24.7% | +0.2% | -6.9% | -6.8% |
| 40 | 2736ns | -24.7% | +0.2% | -5.9% | -6.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.878 | HIGH+ (drift/warm-up) |
| wide-rung-ragged | 0.826 | HIGH+ (drift/warm-up) |
| wide-rung-ragged-overread | 0.788 | HIGH+ (drift/warm-up) |
| wide-rung-wordround | 0.891 | HIGH+ (drift/warm-up) |
| wide-rung-wordround-alias | 0.861 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **wide-rung-ragged**: won 40/40, lost 0/40
- **wide-rung-ragged-overread**: won 27/40, lost 6/40
- **wide-rung-wordround**: won 26/40, lost 6/40
- **wide-rung-wordround-alias**: won 26/40, lost 12/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 5.2ns | 3765.1ns | 0.1% |  |
| wide-rung-ragged | 2.3ns | 2067.8ns | 0.1% |  |
| wide-rung-ragged-overread | 4.4ns | 3233.6ns | 0.1% |  |
| wide-rung-wordround | 3.9ns | 3186.2ns | 0.1% |  |
| wide-rung-wordround-alias | 4.9ns | 3915.6ns | 0.1% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 2562.0-7060.3 ns)
   2562.0 |########################################
   2786.9 |####################
   3011.9 |
   3236.8 |
   3461.7 |
   3686.6 |
   3911.5 |
   4136.4 |
   4361.3 |
   4586.3 |
   4811.2 |
   5036.1 |
   5261.0 |
   5485.9 |
   5710.8 |
   5935.7 |##########
   6160.7 |##
   6385.6 |##
   6610.5 |
   6835.4 |
  (6 below, 4 above range)

wide-rung-ragged (n=40, range 1968.8-2217.1 ns)
   1968.8 |
   1981.2 |########################################
   1993.6 |###
   2006.0 |###
   2018.4 |###
   2030.8 |###
   2043.3 |###
   2055.7 |##########################
   2068.1 |######
   2080.5 |
   2092.9 |###
   2105.4 |
   2117.8 |
   2130.2 |
   2142.6 |
   2155.0 |
   2167.4 |
   2179.9 |
   2192.3 |
   2204.7 |####################
  (2 below, 4 above range)

wide-rung-ragged-overread (n=40, range 2555.5-5083.2 ns)
   2555.5 |########################################
   2681.8 |#################################
   2808.2 |
   2934.6 |
   3061.0 |
   3187.4 |
   3313.8 |
   3440.2 |
   3566.6 |
   3692.9 |
   3819.3 |
   3945.7 |
   4072.1 |
   4198.5 |
   4324.9 |
   4451.3 |###############
   4577.6 |###
   4704.0 |###
   4830.4 |###
   4956.8 |
  (6 below, 2 above range)

wide-rung-wordround (n=40, range 2548.8-4976.9 ns)
   2548.8 |########################################
   2670.2 |
   2791.6 |
   2913.0 |#########################
   3034.4 |
   3155.8 |
   3277.2 |
   3398.7 |
   3520.1 |
   3641.5 |
   3762.9 |
   3884.3 |
   4005.7 |
   4127.1 |
   4248.5 |##
   4369.9 |
   4491.3 |
   4612.7 |
   4734.1 |##########
   4855.5 |#####
  (5 below, 2 above range)

wide-rung-wordround-alias (n=40, range 2550.4-8544.0 ns)
   2550.4 |########################################
   2850.0 |
   3149.7 |
   3449.4 |
   3749.1 |
   4048.8 |
   4348.4 |
   4648.1 |#
   4947.8 |###
   5247.5 |
   5547.2 |#
   5846.8 |#
   6146.5 |
   6446.2 |
   6745.9 |
   7045.6 |
   7345.2 |
   7644.9 |
   7944.6 |
   8244.3 |
  (4 below, 5 above range)

```

## Diagnostics

- **wide-rung-align16**: CV=48.7% (high variance, measurements may be unstable)
- **wide-rung-align16**: autocorrelation=0.88 (measurement drift or warm-up artifact)
- **wide-rung-ragged**: autocorrelation=0.83 (measurement drift or warm-up artifact)
- **wide-rung-ragged-overread**: CV=34.1% (high variance, measurements may be unstable)
- **wide-rung-ragged-overread**: autocorrelation=0.79 (measurement drift or warm-up artifact)
- **wide-rung-wordround**: CV=30.0% (high variance, measurements may be unstable)
- **wide-rung-wordround**: autocorrelation=0.89 (measurement drift or warm-up artifact)
- **wide-rung-wordround-alias**: CV=66.0% (high variance, measurements may be unstable)
- **wide-rung-wordround-alias**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **wide-rung-wordround-alias**: autocorrelation=0.86 (measurement drift or warm-up artifact)
