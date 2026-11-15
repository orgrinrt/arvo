# The identical arms over wrapping addition, which the backend may reassociate with no help from any typestate: the ceiling every saturating arm is measured against

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-lanes16 shows warm-up / thermal drift (autocorr +0.89)

satfold-lanes16's per-pass series has lag-1 autocorrelation +0.89, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### satfold-neon is inconsistent: worst-20% is 1.6x its best-20%

satfold-neon's best 20% of batches run at 502 ns but its worst 20% at 807 ns (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### satfold-lanes64's edge over baseline is significant but tiny (-1 ns, 0.27%)

satfold-lanes64 differs from baseline satfold-iterfold by -1 ns (0.27%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: satfold-nolaw** at 362.1 ns median (-10.8% vs baseline)
- 4 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.46x (fastest 362.1 ns, slowest 528.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 510ns | 472ns | 422ns | 485ns | 673ns | base |
| satfold-lanes16 | 476ns | 439ns | 422ns | 452ns | 600ns | -6.67% |
| satfold-lanes16-constl | 443ns | 439ns | 417ns | 441ns | 476ns | -13.08% |
| satfold-lanes4-idx | 454ns | 432ns | 416ns | 442ns | 527ns | -11.03% |
| satfold-lanes64 | 485ns | 490ns | 421ns | 485ns | 549ns | -4.81% |
| satfold-neon | 665ns | 592ns | 561ns | 620ns | 905ns | +30.48% |
| satfold-neon8 | 572ns | 569ns | 562ns | 570ns | 588ns | +12.16% |
| satfold-nolaw | 428ns | 426ns | 416ns | 428ns | 441ns | -16.01% |
| satfold-seq | 486ns | 456ns | 417ns | 461ns | 629ns | -4.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 438ns | 363ns | 577ns | base | 74.796 |
| satfold-lanes16 | 409ns | 363ns | 515ns | -6.74% | 80.197 |
| satfold-lanes16-constl | 380ns | 359ns | 407ns | -13.22% | 86.191 |
| satfold-lanes4-idx | 390ns | 357ns | 456ns | -10.96% | 84.004 |
| satfold-lanes64 | 417ns | 361ns | 472ns | -4.87% | 78.627 |
| satfold-neon | 594ns | 502ns | 807ns | +35.53% | 55.189 |
| satfold-neon8 | 511ns | 503ns | 524ns | +16.57% | 64.165 |
| satfold-nolaw | 367ns | 355ns | 378ns | -16.28% | 89.335 |
| satfold-seq | 416ns | 359ns | 535ns | -5.14% | 78.845 |

## Performance model

- Peak throughput: **92.268 Gops/s** (satfold-nolaw; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 80.749 | 87.5% |
| satfold-lanes16 | 86.987 | 94.3% |
| satfold-lanes16-constl | 86.895 | 94.2% |
| satfold-lanes4-idx | 88.514 | 95.9% |
| satfold-lanes64 | 77.714 | 84.2% |
| satfold-neon | 62.049 | 67.2% |
| satfold-neon8 | 64.460 | 69.9% |
| satfold-nolaw | 90.507 | 98.1% |
| satfold-seq | 83.667 | 90.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 510ns | 510ns | base |
| satfold-lanes16 | 476ns | 476ns | -6.67% |
| satfold-lanes16-constl | 443ns | 443ns | -13.08% |
| satfold-lanes4-idx | 454ns | 454ns | -11.03% |
| satfold-lanes64 | 485ns | 485ns | -4.81% |
| satfold-neon | 665ns | 665ns | +30.48% |
| satfold-neon8 | 572ns | 572ns | +12.16% |
| satfold-nolaw | 428ns | 428ns | -16.01% |
| satfold-seq | 486ns | 486ns | -4.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 406ns | base | --- | [377, 436] | --- | --- | --- | --- |
| satfold-lanes16 | 377ns | no significant difference | [-62, +0]ns | [376, 378] | no | 0.1236 | 0.1081 | 1 |
| satfold-lanes16-constl | 377ns | -31.5ns (-7.8%) | [-59, -4]ns | [376, 378] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 370ns | -21.1ns (-5.2%) | [-78, -10]ns | [362, 381] | YES | 0.0030 | 0.0022 | 0 |
| satfold-lanes64 | 422ns | no significant difference | [-16, +2]ns | [365, 468] | no | 0.6358 | 0.6358 | 0 |
| satfold-neon | 528ns | +160.8ns (+39.6%) | [+125, +164]ns | [527, 529] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 508ns | +97.9ns (+24.1%) | [+69, +128]ns | [505, 513] | YES | 0.0030 | 0.0022 | 0 |
| satfold-nolaw | 362ns | -43.8ns (-10.8%) | [-79, -5]ns | [359, 375] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 392ns | -12.6ns (-3.1%) | [-25, -8]ns | [365, 409] | YES | 0.0000 | 0.0000 | 1 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 434ns | -16.2% | -13.0% | -17.3% | +7.9% | +21.6% | +16.5% | -17.3% | -5.8% |
| 2 | 438ns | -16.7% | -13.5% | -19.0% | +7.3% | +20.6% | +14.8% | -18.1% | -6.5% |
| 3 | 437ns | -17.5% | -13.9% | -18.1% | +7.0% | +21.1% | +14.8% | -19.0% | -6.4% |
| 4 | 435ns | -16.9% | -13.1% | -17.1% | +7.8% | +21.9% | +16.1% | -19.0% | -6.5% |
| 5 | 438ns | -17.5% | -14.0% | -18.3% | +7.5% | +20.6% | +15.6% | -18.0% | -7.4% |
| 6 | 434ns | -16.5% | -13.2% | -16.9% | +7.9% | +21.8% | +16.4% | -18.6% | -5.6% |
| 7 | 433ns | -16.2% | -13.1% | -4.9% | +8.0% | +21.6% | +16.2% | -17.8% | -6.5% |
| 8 | 436ns | -16.6% | -13.8% | -18.0% | +8.1% | +20.9% | +15.6% | -17.3% | +49.2% |
| 9 | 435ns | -11.1% | -13.6% | -18.0% | +7.7% | +21.1% | +15.9% | -17.3% | -5.8% |
| 10 | 435ns | -12.8% | -13.1% | -17.7% | +7.8% | +21.9% | +15.9% | -17.2% | -6.7% |
| 11 | 376ns | -0.2% | +0.2% | -1.3% | -3.6% | +33.9% | +34.3% | -3.4% | -3.3% |
| 12 | 375ns | +0.1% | -0.1% | +1.7% | -2.7% | +60.9% | +34.5% | +0.3% | -2.9% |
| 13 | 378ns | +0.1% | +1.0% | -2.3% | -5.3% | +36.3% | +33.9% | +0.1% | -4.4% |
| 14 | 376ns | -0.2% | +0.3% | -3.5% | +0.6% | +33.7% | +34.2% | -0.2% | -3.4% |
| 15 | 378ns | +0.0% | +0.1% | -2.3% | -4.5% | +32.2% | +33.5% | -0.7% | -2.8% |
| 16 | 378ns | -0.1% | -0.4% | -2.7% | -3.5% | +32.5% | +32.8% | -0.4% | +0.0% |
| 17 | 377ns | +0.1% | -1.0% | -5.3% | -4.2% | +32.9% | +33.7% | -0.6% | -2.1% |
| 18 | 375ns | +0.9% | +1.4% | -4.1% | -3.1% | +33.7% | +34.5% | +1.2% | -2.6% |
| 19 | 377ns | -0.1% | +0.2% | -5.5% | -4.1% | +32.8% | +33.0% | -0.3% | -3.9% |
| 20 | 377ns | -0.5% | -0.1% | -4.9% | -4.3% | +33.5% | +33.5% | +1.2% | -4.0% |
| 21 | 364ns | +3.2% | -1.7% | +0.7% | -0.2% | +45.3% | +40.3% | -1.4% | -2.2% |
| 22 | 365ns | +2.8% | -1.0% | -1.4% | -0.8% | +44.4% | +39.6% | -1.7% | -1.5% |
| 23 | 365ns | +3.0% | -1.8% | +1.0% | +0.1% | +44.8% | +40.4% | -1.7% | -2.1% |
| 24 | 362ns | +3.9% | -1.0% | +2.8% | +0.3% | +46.5% | +42.0% | -0.8% | -0.9% |
| 25 | 362ns | +4.0% | -0.4% | +63.7% | +1.3% | +45.2% | +40.8% | -0.5% | -0.2% |
| 26 | 361ns | +4.5% | -0.2% | +5.2% | +0.5% | +46.3% | +42.7% | -1.4% | -0.4% |
| 27 | 363ns | +4.0% | -0.8% | +5.4% | +0.6% | +44.1% | +40.9% | -1.5% | -1.0% |
| 28 | 363ns | +3.8% | -1.4% | +4.9% | +1.5% | +44.8% | +41.4% | -1.2% | -1.6% |
| 29 | 366ns | +2.9% | -0.9% | +3.5% | -0.7% | +44.2% | +40.1% | -1.8% | +1.7% |
| 30 | 364ns | +3.3% | -1.0% | +4.7% | +0.7% | +44.6% | +45.9% | -2.9% | +2.4% |
| 31 | 577ns | -10.8% | -29.8% | -25.4% | -18.0% | +39.6% | -10.5% | -35.2% | -18.4% |
| 32 | 578ns | -10.7% | -29.7% | -23.7% | -19.3% | +39.6% | -11.4% | -34.9% | +0.7% |
| 33 | 577ns | -10.8% | -29.3% | -24.9% | -18.3% | +40.4% | -10.8% | -34.4% | -18.9% |
| 34 | 577ns | -10.6% | -29.2% | -24.6% | -18.6% | +40.1% | -10.8% | -34.8% | +0.6% |
| 35 | 576ns | -10.6% | -29.3% | -24.1% | -18.4% | +40.3% | -9.8% | -34.9% | -17.9% |
| 36 | 578ns | -10.7% | -29.6% | -24.6% | -18.4% | +39.2% | -6.3% | -35.3% | -18.8% |
| 37 | 575ns | -10.6% | -30.1% | -24.7% | -19.1% | +39.8% | -7.5% | -34.8% | +0.7% |
| 38 | 575ns | -10.3% | -29.2% | -24.7% | -18.3% | +40.0% | -9.6% | -34.2% | -18.1% |
| 39 | 575ns | -11.1% | -29.1% | -23.9% | -17.9% | +39.6% | -10.1% | -34.4% | -18.5% |
| 40 | 578ns | -11.2% | -29.8% | -24.7% | -19.1% | +40.0% | -10.7% | -35.0% | -18.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.879 | HIGH+ (drift/warm-up) |
| satfold-lanes16 | 0.891 | HIGH+ (drift/warm-up) |
| satfold-lanes16-constl | 0.852 | HIGH+ (drift/warm-up) |
| satfold-lanes4-idx | 0.341 | moderate+ |
| satfold-lanes64 | 0.869 | HIGH+ (drift/warm-up) |
| satfold-neon | 0.879 | HIGH+ (drift/warm-up) |
| satfold-neon8 | 0.744 | HIGH+ (drift/warm-up) |
| satfold-nolaw | 0.784 | HIGH+ (drift/warm-up) |
| satfold-seq | 0.515 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **satfold-lanes16**: won 25/40, lost 14/40
- **satfold-lanes16-constl**: won 34/40, lost 6/40
- **satfold-lanes4-idx**: won 30/40, lost 10/40
- **satfold-lanes64**: won 22/40, lost 18/40
- **satfold-neon**: won 0/40, lost 40/40
- **satfold-neon8**: won 10/40, lost 30/40
- **satfold-nolaw**: won 36/40, lost 4/40
- **satfold-seq**: won 33/40, lost 6/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 3.2ns | 438.1ns | 0.7% |  |
| satfold-lanes16 | 3.1ns | 408.6ns | 0.8% |  |
| satfold-lanes16-constl | 2.6ns | 380.2ns | 0.7% |  |
| satfold-lanes4-idx | 3.1ns | 390.1ns | 0.8% |  |
| satfold-lanes64 | 3.3ns | 416.8ns | 0.8% |  |
| satfold-neon | 2.9ns | 593.7ns | 0.5% |  |
| satfold-neon8 | 2.3ns | 510.7ns | 0.4% |  |
| satfold-nolaw | 2.4ns | 366.8ns | 0.7% |  |
| satfold-seq | 2.4ns | 415.6ns | 0.6% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 363.1-577.0 ns)
    363.1 |####################
    373.8 |########################################
    384.5 |
    395.2 |
    405.9 |
    416.6 |
    427.3 |####################################
    438.0 |####
    448.7 |
    459.4 |
    470.1 |
    480.7 |
    491.4 |
    502.1 |
    512.8 |
    523.5 |
    534.2 |
    544.9 |
    555.6 |
    566.3 |####################
  (5 below, 5 above range)

satfold-lanes16 (n=40, range 362.6-515.1 ns)
    362.6 |##########
    370.2 |########################################
    377.9 |############
    385.5 |##
    393.1 |
    400.7 |
    408.3 |
    416.0 |
    423.6 |
    431.2 |
    438.8 |
    446.5 |
    454.1 |
    461.7 |
    469.3 |
    476.9 |
    484.6 |
    492.2 |
    499.8 |
    507.4 |###############
  (4 below, 4 above range)

satfold-lanes16-constl (n=40, range 359.3-407.1 ns)
    359.3 |###########
    361.6 |#####
    364.0 |
    366.4 |
    368.8 |
    371.2 |##
    373.6 |#####
    376.0 |########################################
    378.4 |#####
    380.8 |##
    383.2 |
    385.6 |
    388.0 |
    390.4 |
    392.8 |
    395.2 |
    397.6 |
    399.9 |##
    402.3 |
    404.7 |#################
  (4 below, 3 above range)

satfold-lanes4-idx (n=40, range 357.0-456.0 ns)
    357.0 |########################################
    361.9 |#######
    366.9 |##################
    371.8 |###
    376.8 |##################
    381.7 |###
    386.7 |
    391.6 |
    396.6 |
    401.5 |
    406.5 |
    411.4 |###
    416.4 |
    421.3 |
    426.3 |###
    431.2 |#####################
    436.2 |#######
    441.1 |###
    446.1 |
    451.0 |
  (3 below, 1 above range)

satfold-lanes64 (n=40, range 361.2-471.5 ns)
    361.2 |########################################
    366.8 |##
    372.3 |
    377.8 |##
    383.3 |
    388.8 |
    394.3 |
    399.8 |
    405.3 |
    410.9 |
    416.4 |
    421.9 |
    427.4 |
    432.9 |
    438.4 |
    443.9 |
    449.5 |
    455.0 |
    460.5 |##
    466.0 |########################################
  (4 below, 5 above range)

satfold-neon (n=40, range 501.7-807.0 ns)
    501.7 |########
    517.0 |########################################
    532.2 |
    547.5 |
    562.8 |
    578.0 |
    593.3 |##
    608.6 |
    623.8 |
    639.1 |
    654.4 |
    669.6 |
    684.9 |
    700.2 |
    715.4 |
    730.7 |
    746.0 |
    761.2 |
    776.5 |
    791.8 |############
  (5 below, 4 above range)

satfold-neon8 (n=40, range 502.8-524.1 ns)
    502.8 |#################################
    503.9 |########################################
    505.0 |#################################
    506.0 |######
    507.1 |
    508.2 |
    509.2 |#############
    510.3 |#############
    511.4 |######
    512.4 |####################
    513.5 |######
    514.5 |####################
    515.6 |######
    516.7 |#############
    517.7 |
    518.8 |#############
    519.9 |
    520.9 |
    522.0 |
    523.1 |
  (3 below, 3 above range)

satfold-nolaw (n=40, range 355.1-378.2 ns)
    355.1 |########
    356.3 |
    357.4 |####
    358.6 |########################################
    359.8 |############
    360.9 |
    362.1 |
    363.2 |####
    364.4 |
    365.5 |
    366.7 |
    367.8 |
    369.0 |
    370.1 |
    371.3 |
    372.5 |
    373.6 |########
    374.8 |############################
    375.9 |################
    377.1 |########
  (4 below, 4 above range)

satfold-seq (n=40, range 358.7-534.5 ns)
    358.7 |########################################
    367.5 |#############
    376.3 |###
    385.1 |
    393.9 |
    402.6 |##############################
    411.4 |
    420.2 |
    429.0 |
    437.8 |
    446.6 |
    455.4 |
    464.2 |####################
    473.0 |###
    481.8 |
    490.6 |
    499.4 |
    508.1 |
    516.9 |
    525.7 |
  (3 below, 4 above range)

```

## Diagnostics

- **satfold-iterfold**: autocorrelation=0.88 (measurement drift or warm-up artifact)
- **satfold-lanes16**: autocorrelation=0.89 (measurement drift or warm-up artifact)
- **satfold-lanes16-constl**: autocorrelation=0.85 (measurement drift or warm-up artifact)
- **satfold-lanes64**: autocorrelation=0.87 (measurement drift or warm-up artifact)
- **satfold-neon**: CV=20.9% (high variance, measurements may be unstable)
- **satfold-neon**: autocorrelation=0.88 (measurement drift or warm-up artifact)
- **satfold-neon8**: autocorrelation=0.74 (measurement drift or warm-up artifact)
- **satfold-nolaw**: autocorrelation=0.78 (measurement drift or warm-up artifact)
- **satfold-seq**: autocorrelation=0.52 (measurement drift or warm-up artifact)
