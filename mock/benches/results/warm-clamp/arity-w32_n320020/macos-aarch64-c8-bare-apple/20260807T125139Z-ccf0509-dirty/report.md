# Clamping fold at 32 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-min-lanes beats baseline by 39% (significant)

warm-clamp-min-lanes is -327 ns (39%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-accfit-dyn is an outlier: 6.1x slower than the field

warm-clamp-accfit-dyn (3.08 us) is 6.1x the fastest (505 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-clamp-min-lanes, warm-clamp-minimum) are a dead heat (<1%)

warm-clamp-min-lanes (505 ns) and warm-clamp-minimum (506 ns) differ by 0.28%, inside the noise, even though the wider field spreads 509.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-clamp-minimum shows warm-up / thermal drift (autocorr +0.88)

warm-clamp-minimum's per-pass series has lag-1 autocorrelation +0.88, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-min-lanes, warm-clamp-minimum, warm-clamp-acc64, warm-clamp-accfit, warm-clamp-head} vs {warm-clamp-accfit-dyn} (167% apart)

The field splits into a fast tier {warm-clamp-min-lanes, warm-clamp-minimum, warm-clamp-acc64, warm-clamp-accfit, warm-clamp-head} and a slow tier {warm-clamp-accfit-dyn} with a 167% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.1x the fastest

Fastest warm-clamp-min-lanes (505 ns) to slowest warm-clamp-accfit-dyn (3.08 us): 6.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-clamp-accfit's edge over baseline is significant but tiny (5 ns, 0.55%)

warm-clamp-accfit differs from baseline warm-clamp-acc64 by 5 ns (0.55%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-clamp-min-lanes** at 504.8 ns median (-39.6% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 6.09x (fastest 504.8 ns, slowest 3075.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 909ns | 897ns | 881ns | 893ns | 986ns | base |
| warm-clamp-accfit | 911ns | 899ns | 882ns | 899ns | 974ns | +0.16% |
| warm-clamp-accfit-dyn | 3252ns | 3141ns | 3109ns | 3184ns | 3598ns | +257.65% |
| warm-clamp-head | 1231ns | 1211ns | 1207ns | 1222ns | 1283ns | +35.41% |
| warm-clamp-min-lanes | 565ns | 564ns | 557ns | 564ns | 573ns | -37.90% |
| warm-clamp-minimum | 595ns | 570ns | 557ns | 580ns | 678ns | -34.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 847ns | 821ns | 921ns | base | 9.669 |
| warm-clamp-accfit | 848ns | 822ns | 906ns | +0.08% | 9.661 |
| warm-clamp-accfit-dyn | 3186ns | 3050ns | 3525ns | +276.09% | 2.571 |
| warm-clamp-head | 1169ns | 1146ns | 1218ns | +38.00% | 7.007 |
| warm-clamp-min-lanes | 503ns | 496ns | 510ns | -40.62% | 16.284 |
| warm-clamp-minimum | 529ns | 497ns | 604ns | -37.53% | 15.478 |

## Performance model

- Peak throughput: **16.503 Gops/s** (warm-clamp-min-lanes; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 9.801 | 59.4% |
| warm-clamp-accfit | 9.791 | 59.3% |
| warm-clamp-accfit-dyn | 2.664 | 16.1% |
| warm-clamp-head | 7.121 | 43.2% |
| warm-clamp-min-lanes | 16.228 | 98.3% |
| warm-clamp-minimum | 16.183 | 98.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 909ns | 909ns | base |
| warm-clamp-accfit | 911ns | 911ns | +0.16% |
| warm-clamp-accfit-dyn | 3252ns | 3252ns | +257.65% |
| warm-clamp-head | 1231ns | 1231ns | +35.41% |
| warm-clamp-min-lanes | 565ns | 565ns | -37.90% |
| warm-clamp-minimum | 595ns | 595ns | -34.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 836ns | base | --- | [825, 838] | --- | --- | --- | --- |
| warm-clamp-accfit | 837ns | no significant difference | [-1, +13]ns | [836, 838] | no | 0.4177 | 0.4177 | 2 |
| warm-clamp-accfit-dyn | 3075ns | +2238.0ns (+267.8%) | [+2228, +2349]ns | [3052, 3218] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 1150ns | +326.4ns (+39.1%) | [+323, +328]ns | [1149, 1158] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 505ns | -329.5ns (-39.4%) | [-334, -325]ns | [498, 507] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 506ns | -326.6ns (-39.1%) | [-336, -312]ns | [505, 509] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 870ns | +4.4% | +270.9% | +39.1% | -41.5% | -41.8% |
| 2 | 869ns | -3.5% | +270.1% | +39.1% | -41.6% | -41.7% |
| 3 | 883ns | -5.3% | +279.2% | +36.7% | -42.3% | -42.7% |
| 4 | 1265ns | -33.7% | +154.8% | -4.3% | -59.9% | -60.0% |
| 5 | 862ns | -2.9% | +273.6% | +40.2% | -41.4% | -41.2% |
| 6 | 838ns | +0.0% | +284.8% | +44.4% | -39.2% | -39.7% |
| 7 | 840ns | -0.4% | +283.7% | +43.9% | -39.6% | -39.7% |
| 8 | 841ns | -0.4% | +283.2% | +43.6% | -39.2% | -39.8% |
| 9 | 839ns | -0.1% | +284.0% | +44.1% | -39.3% | -39.8% |
| 10 | 839ns | +0.2% | +274.3% | +44.1% | -39.8% | -39.7% |
| 11 | 836ns | +3.9% | +351.7% | +37.6% | -40.7% | -40.5% |
| 12 | 930ns | -7.1% | +297.9% | +33.2% | -46.5% | -46.5% |
| 13 | 847ns | +2.6% | +336.9% | +35.3% | -41.3% | -41.2% |
| 14 | 836ns | +3.6% | +334.6% | +37.4% | -40.3% | -40.5% |
| 15 | 838ns | +2.9% | +264.1% | +36.8% | -39.3% | -41.0% |
| 16 | 836ns | +3.7% | +264.9% | +37.1% | -40.0% | -40.4% |
| 17 | 838ns | +36.5% | +264.4% | +37.4% | -40.8% | -40.7% |
| 18 | 841ns | -0.7% | +262.8% | +36.7% | -40.7% | -41.0% |
| 19 | 837ns | +0.9% | +266.5% | +49.4% | -40.4% | -36.7% |
| 20 | 838ns | +3.3% | +274.1% | +39.0% | -40.5% | -40.8% |
| 21 | 836ns | -0.1% | +267.1% | +37.6% | -39.4% | -39.7% |
| 22 | 825ns | +1.3% | +269.9% | +39.6% | -38.1% | -39.0% |
| 23 | 818ns | +2.3% | +272.6% | +40.1% | -38.0% | -37.6% |
| 24 | 818ns | +2.3% | +272.7% | +40.0% | -37.6% | -36.7% |
| 25 | 822ns | +1.8% | +286.1% | +40.1% | -37.9% | -38.2% |
| 26 | 822ns | +1.7% | +278.0% | +39.8% | -38.1% | -38.5% |
| 27 | 822ns | +1.6% | +325.1% | +39.8% | -38.5% | -38.6% |
| 28 | 822ns | +1.7% | +271.0% | +39.7% | -38.6% | -38.7% |
| 29 | 825ns | +1.4% | +269.7% | +39.4% | -38.5% | -38.6% |
| 30 | 823ns | +1.5% | +270.6% | +41.1% | -38.6% | -37.8% |
| 31 | 827ns | -0.4% | +301.5% | +38.4% | -40.1% | -26.7% |
| 32 | 825ns | -0.7% | +269.8% | +39.1% | -39.5% | -27.2% |
| 33 | 822ns | -0.3% | +271.0% | +40.4% | -39.5% | -26.3% |
| 34 | 820ns | +0.7% | +275.8% | +40.2% | -39.3% | -26.0% |
| 35 | 827ns | -0.6% | +268.9% | +38.8% | -40.0% | -27.3% |
| 36 | 822ns | -0.4% | +271.2% | +39.5% | -39.2% | -26.5% |
| 37 | 821ns | +0.0% | +271.6% | +39.3% | -39.5% | -26.9% |
| 38 | 823ns | +0.1% | +272.3% | +39.4% | -39.5% | -27.0% |
| 39 | 824ns | -0.1% | +270.3% | +39.4% | -39.6% | -26.8% |
| 40 | 823ns | +0.2% | +270.7% | +39.8% | -39.5% | -26.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.170 | ok |
| warm-clamp-accfit | 0.108 | ok |
| warm-clamp-accfit-dyn | 0.583 | HIGH+ (drift/warm-up) |
| warm-clamp-head | 0.477 | moderate+ |
| warm-clamp-min-lanes | 0.703 | HIGH+ (drift/warm-up) |
| warm-clamp-minimum | 0.882 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-clamp-accfit**: won 15/40, lost 22/40
- **warm-clamp-accfit-dyn**: won 0/40, lost 40/40
- **warm-clamp-head**: won 1/40, lost 39/40
- **warm-clamp-min-lanes**: won 40/40, lost 0/40
- **warm-clamp-minimum**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 2.5ns | 847.2ns | 0.3% |  |
| warm-clamp-accfit | 2.6ns | 847.9ns | 0.3% |  |
| warm-clamp-accfit-dyn | 2.9ns | 3186.3ns | 0.1% |  |
| warm-clamp-head | 2.4ns | 1169.1ns | 0.2% |  |
| warm-clamp-min-lanes | 2.3ns | 503.1ns | 0.5% |  |
| warm-clamp-minimum | 2.4ns | 529.3ns | 0.5% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 820.6-920.7 ns)
    820.6 |########################################
    825.6 |#####
    830.6 |
    835.6 |##################################
    840.6 |#####
    845.7 |##
    850.7 |
    855.7 |
    860.7 |##
    865.7 |#####
    870.7 |
    875.7 |
    880.7 |##
    885.7 |
    890.7 |
    895.7 |
    900.7 |
    905.7 |
    910.7 |
    915.7 |
  (3 below, 2 above range)

warm-clamp-accfit (n=40, range 821.5-906.4 ns)
    821.5 |############
    825.8 |
    830.0 |
    834.3 |########################################
    838.5 |##
    842.8 |##
    847.0 |
    851.2 |
    855.5 |
    859.7 |##
    864.0 |########
    868.2 |####
    872.5 |
    876.7 |
    880.9 |
    885.2 |
    889.4 |
    893.7 |
    897.9 |
    902.2 |
  (4 below, 2 above range)

warm-clamp-accfit-dyn (n=40, range 3050.0-3524.9 ns)
   3050.0 |########################################
   3073.7 |##
   3097.5 |##
   3121.2 |#####
   3145.0 |
   3168.7 |##
   3192.5 |##
   3216.2 |##################
   3240.0 |
   3263.7 |
   3287.4 |
   3311.2 |##
   3334.9 |##
   3358.7 |
   3382.4 |
   3406.2 |
   3429.9 |
   3453.7 |
   3477.4 |##
   3501.1 |
  (5 below, 4 above range)

warm-clamp-head (n=40, range 1145.7-1218.4 ns)
   1145.7 |########################################
   1149.3 |#################################
   1152.9 |###
   1156.6 |
   1160.2 |###
   1163.8 |###
   1167.5 |
   1171.1 |
   1174.7 |
   1178.4 |
   1182.0 |
   1185.6 |
   1189.3 |
   1192.9 |
   1196.5 |
   1200.2 |
   1203.8 |###
   1207.4 |##############################
   1211.1 |
   1214.7 |
  (3 below, 2 above range)

warm-clamp-min-lanes (n=40, range 496.4-510.0 ns)
    496.4 |######
    497.1 |##########################
    497.8 |########################################
    498.4 |######
    499.1 |#############
    499.8 |
    500.5 |
    501.2 |######
    501.8 |
    502.5 |
    503.2 |
    503.9 |
    504.6 |#############
    505.2 |####################
    505.9 |
    506.6 |##########################
    507.3 |#############
    508.0 |######
    508.7 |##########################
    509.3 |######
  (4 below, 4 above range)

warm-clamp-minimum (n=40, range 496.7-603.7 ns)
    496.7 |############
    502.1 |########################################
    507.4 |#######
    512.8 |
    518.1 |##
    523.5 |
    528.8 |##
    534.2 |
    539.5 |
    544.9 |
    550.2 |
    555.6 |
    560.9 |
    566.3 |
    571.6 |
    577.0 |
    582.3 |
    587.7 |
    593.0 |
    598.4 |###############
  (4 below, 4 above range)

```

## Diagnostics

- **warm-clamp-accfit-dyn**: autocorrelation=0.58 (measurement drift or warm-up artifact)
- **warm-clamp-min-lanes**: autocorrelation=0.70 (measurement drift or warm-up artifact)
- **warm-clamp-minimum**: autocorrelation=0.88 (measurement drift or warm-up artifact)
