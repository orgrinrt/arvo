# The identical arms over wrapping addition, which the backend may reassociate with no help from any typestate: the ceiling every saturating arm is measured against

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-lanes4-idx is an outlier: 2.5x slower than the field

satfold-lanes4-idx (570 ns) is 2.5x the fastest (231 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-neon shows warm-up / thermal drift (autocorr +0.87)

satfold-neon's per-pass series has lag-1 autocorrelation +0.87, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-lanes64, satfold-neon, satfold-neon8, satfold-seq, satfold-iterfold, satfold-lanes16, satfold-lanes16-constl} vs {satfold-nolaw, satfold-lanes4-idx} (62% apart)

The field splits into a fast tier {satfold-lanes64, satfold-neon, satfold-neon8, satfold-seq, satfold-iterfold, satfold-lanes16, satfold-lanes16-constl} and a slow tier {satfold-nolaw, satfold-lanes4-idx} with a 62% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: satfold-lanes64** at 231.4 ns median (-18.8% vs baseline)
- 3 variants significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 2.46x (fastest 231.4 ns, slowest 570.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 363ns | 353ns | 326ns | 352ns | 432ns | base |
| satfold-lanes16 | 376ns | 365ns | 336ns | 370ns | 436ns | +3.76% |
| satfold-lanes16-constl | 352ns | 361ns | 326ns | 354ns | 373ns | -2.95% |
| satfold-lanes4-idx | 641ns | 639ns | 594ns | 635ns | 706ns | +76.80% |
| satfold-lanes64 | 313ns | 296ns | 292ns | 303ns | 364ns | -13.74% |
| satfold-neon | 316ns | 308ns | 297ns | 311ns | 354ns | -12.75% |
| satfold-neon8 | 323ns | 322ns | 298ns | 322ns | 351ns | -10.95% |
| satfold-nolaw | 540ns | 543ns | 507ns | 534ns | 591ns | +48.82% |
| satfold-seq | 352ns | 351ns | 323ns | 352ns | 383ns | -2.81% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 296ns | 265ns | 361ns | base | 110.848 |
| satfold-lanes16 | 306ns | 272ns | 359ns | +3.67% | 106.921 |
| satfold-lanes16-constl | 286ns | 265ns | 303ns | -3.27% | 114.600 |
| satfold-lanes4-idx | 573ns | 529ns | 635ns | +93.85% | 57.181 |
| satfold-lanes64 | 242ns | 228ns | 278ns | -18.08% | 135.310 |
| satfold-neon | 252ns | 234ns | 282ns | -14.82% | 130.135 |
| satfold-neon8 | 255ns | 234ns | 277ns | -13.86% | 128.676 |
| satfold-nolaw | 470ns | 442ns | 511ns | +59.06% | 69.689 |
| satfold-seq | 285ns | 262ns | 309ns | -3.75% | 115.172 |

## Performance model

- Peak throughput: **143.948 Gops/s** (satfold-lanes64; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 114.895 | 79.8% |
| satfold-lanes16 | 111.532 | 77.5% |
| satfold-lanes16-constl | 111.380 | 77.4% |
| satfold-lanes4-idx | 57.447 | 39.9% |
| satfold-lanes64 | 141.577 | 98.4% |
| satfold-neon | 133.529 | 92.8% |
| satfold-neon8 | 128.704 | 89.4% |
| satfold-nolaw | 68.840 | 47.8% |
| satfold-seq | 115.401 | 80.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 363ns | 363ns | base |
| satfold-lanes16 | 376ns | 376ns | +3.76% |
| satfold-lanes16-constl | 352ns | 352ns | -2.95% |
| satfold-lanes4-idx | 641ns | 641ns | +76.80% |
| satfold-lanes64 | 313ns | 313ns | -13.74% |
| satfold-neon | 316ns | 316ns | -12.75% |
| satfold-neon8 | 323ns | 323ns | -10.95% |
| satfold-nolaw | 540ns | 540ns | +48.82% |
| satfold-seq | 352ns | 352ns | -2.81% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 285ns | base | --- | [283, 286] | --- | --- | --- | --- |
| satfold-lanes16 | 294ns | +9.4ns (+3.3%) | [+6, +17]ns | [291, 308] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 294ns | +9.8ns (+3.4%) | [+8, +12]ns | [277, 297] | YES | 0.0190 | 0.0166 | 0 |
| satfold-lanes4-idx | 570ns | +281.2ns (+98.6%) | [+272, +286]ns | [568, 571] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 231ns | -45.5ns (-15.9%) | [-54, -34]ns | [230, 233] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 245ns | -31.6ns (-11.1%) | [-45, -18]ns | [244, 246] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 255ns | -31.0ns (-10.9%) | [-32, -28]ns | [252, 257] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 476ns | +190.0ns (+66.6%) | [+180, +194]ns | [449, 480] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 284ns | no significant difference | [-4, +15]ns | [265, 303] | no | 0.5224 | 0.5224 | 1 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 271ns | +4.0% | +2.2% | +96.3% | -14.6% | -9.8% | -14.1% | +69.7% | -2.1% |
| 2 | 265ns | +7.4% | +3.9% | +100.3% | -12.9% | -7.1% | -11.3% | +73.1% | -0.2% |
| 3 | 269ns | +1.5% | +3.2% | +98.3% | -14.7% | -8.5% | -12.4% | +67.5% | -2.8% |
| 4 | 266ns | +1.1% | +4.2% | +98.9% | -12.7% | -8.3% | -12.0% | +67.5% | -1.4% |
| 5 | 264ns | +1.2% | +5.3% | +100.6% | -12.5% | -6.0% | -10.4% | +68.4% | +0.0% |
| 6 | 265ns | +4.7% | +6.9% | +99.4% | -12.9% | -6.0% | -11.6% | +73.3% | -0.5% |
| 7 | 265ns | +1.9% | +4.9% | +101.1% | -13.6% | -7.5% | -11.0% | +71.4% | -1.6% |
| 8 | 264ns | +2.8% | +4.7% | +99.5% | -11.6% | -6.8% | -9.7% | +68.7% | +0.3% |
| 9 | 265ns | +2.8% | +3.0% | +98.6% | -13.2% | -7.8% | -11.9% | +153.9% | +0.2% |
| 10 | 264ns | +2.4% | +3.6% | +101.6% | -12.2% | -5.4% | -11.2% | +69.3% | +1.1% |
| 11 | 286ns | +3.4% | +4.4% | +101.5% | -6.1% | -0.7% | -11.8% | +68.1% | +11.8% |
| 12 | 283ns | +3.3% | +6.2% | +101.5% | -6.0% | -1.3% | -8.8% | +73.2% | +10.6% |
| 13 | 289ns | +1.0% | +5.5% | +99.8% | -8.2% | -2.6% | -11.0% | +67.0% | +5.6% |
| 14 | 283ns | +4.9% | +6.0% | +103.4% | -6.0% | +1.5% | -10.1% | +69.6% | +6.2% |
| 15 | 288ns | +1.5% | +3.5% | +99.6% | -8.7% | -2.6% | -10.9% | +67.7% | +7.2% |
| 16 | 287ns | +2.5% | +4.4% | +100.6% | -6.2% | -1.0% | -10.9% | +69.2% | +5.4% |
| 17 | 283ns | +3.9% | +6.3% | +101.8% | -6.6% | -1.6% | -8.4% | +71.3% | +8.2% |
| 18 | 285ns | +2.0% | +5.0% | +100.0% | -7.7% | -2.0% | -9.6% | +72.1% | +6.9% |
| 19 | 289ns | +1.1% | +2.7% | +97.4% | -8.0% | -2.5% | -11.7% | +66.8% | +5.0% |
| 20 | 285ns | +2.5% | +4.8% | +100.4% | -8.4% | -1.5% | -10.5% | +66.9% | +6.9% |
| 21 | 293ns | +17.4% | -9.9% | +94.7% | -22.3% | -16.3% | -7.1% | +52.4% | +4.8% |
| 22 | 292ns | +16.6% | -10.0% | +94.8% | -21.6% | -16.1% | -7.9% | +51.3% | +4.3% |
| 23 | 292ns | +17.4% | -8.3% | +95.4% | -21.7% | -15.7% | -8.9% | +51.6% | +4.1% |
| 24 | 288ns | +18.5% | +6.2% | +96.8% | -20.3% | -14.5% | -6.2% | +54.4% | +4.9% |
| 25 | 281ns | +21.9% | -5.9% | +103.0% | -19.2% | -14.0% | -3.2% | +58.1% | +7.7% |
| 26 | 285ns | +19.4% | -6.7% | +98.6% | -19.0% | -15.6% | -5.7% | +53.4% | +6.0% |
| 27 | 283ns | +21.2% | -5.7% | +100.2% | -18.2% | -12.9% | -6.0% | +55.3% | +6.8% |
| 28 | 349ns | -2.6% | -23.8% | +63.5% | -34.6% | -30.6% | -22.8% | +28.3% | -13.0% |
| 29 | 284ns | +19.9% | -7.6% | +98.9% | -18.1% | -13.7% | +14.1% | +55.8% | +8.2% |
| 30 | 287ns | +20.3% | -7.2% | +98.2% | -19.0% | -15.1% | -7.3% | +53.5% | +5.5% |
| 31 | 450ns | -32.2% | -30.9% | +51.0% | -20.1% | -48.0% | -43.1% | +7.7% | -40.6% |
| 32 | 285ns | +8.2% | +2.9% | +100.1% | -18.5% | -18.1% | -11.4% | +68.0% | -7.1% |
| 33 | 288ns | +7.3% | +3.6% | +132.1% | -20.1% | -16.8% | -12.2% | +69.1% | -8.5% |
| 34 | 286ns | +65.7% | +4.7% | +99.4% | -20.5% | -18.9% | -12.1% | +71.1% | -9.2% |
| 35 | 285ns | +7.9% | +4.1% | +140.5% | -18.4% | -17.1% | -11.9% | +67.5% | -7.4% |
| 36 | 458ns | -32.9% | -35.2% | +24.8% | -50.3% | -48.8% | -44.8% | +4.6% | -41.8% |
| 37 | 286ns | +3.4% | +3.2% | +100.2% | -18.5% | -17.4% | -10.9% | +66.6% | -9.0% |
| 38 | 284ns | +1.0% | +3.9% | +136.0% | -19.2% | -17.2% | -11.3% | +68.2% | -8.2% |
| 39 | 285ns | +0.4% | +3.4% | +99.7% | -20.8% | -17.7% | -11.4% | +67.8% | -7.6% |
| 40 | 466ns | -38.2% | -36.4% | +38.0% | -51.0% | -50.0% | -45.6% | +2.2% | -43.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.010 | ok |
| satfold-lanes16 | 0.437 | moderate+ |
| satfold-lanes16-constl | 0.557 | HIGH+ (drift/warm-up) |
| satfold-lanes4-idx | 0.218 | moderate+ |
| satfold-lanes64 | 0.242 | moderate+ |
| satfold-neon | 0.868 | HIGH+ (drift/warm-up) |
| satfold-neon8 | 0.633 | HIGH+ (drift/warm-up) |
| satfold-nolaw | 0.019 | ok |
| satfold-seq | 0.850 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **satfold-lanes16**: won 4/40, lost 36/40
- **satfold-lanes16-constl**: won 12/40, lost 28/40
- **satfold-lanes4-idx**: won 0/40, lost 40/40
- **satfold-lanes64**: won 40/40, lost 0/40
- **satfold-neon**: won 39/40, lost 1/40
- **satfold-neon8**: won 39/40, lost 1/40
- **satfold-nolaw**: won 0/40, lost 40/40
- **satfold-seq**: won 17/40, lost 22/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 2.1ns | 295.6ns | 0.7% |  |
| satfold-lanes16 | 3.0ns | 306.5ns | 1.0% |  |
| satfold-lanes16-constl | 2.3ns | 285.9ns | 0.8% |  |
| satfold-lanes4-idx | 2.5ns | 573.1ns | 0.4% |  |
| satfold-lanes64 | 2.8ns | 242.2ns | 1.1% |  |
| satfold-neon | 2.2ns | 251.8ns | 0.9% |  |
| satfold-neon8 | 2.8ns | 254.7ns | 1.1% |  |
| satfold-nolaw | 2.0ns | 470.2ns | 0.4% |  |
| satfold-seq | 2.0ns | 284.5ns | 0.7% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 264.8-361.2 ns)
    264.8 |###########
    269.6 |##
    274.4 |
    279.3 |###########
    284.1 |########################################
    288.9 |#########
    293.7 |
    298.5 |
    303.3 |
    308.2 |
    313.0 |
    317.8 |
    322.6 |
    327.4 |
    332.2 |
    337.1 |
    341.9 |
    346.7 |##
    351.5 |
    356.3 |
  (4 below, 3 above range)

satfold-lanes16 (n=40, range 271.5-359.3 ns)
    271.5 |#############
    275.9 |######
    280.3 |#############
    284.7 |####################
    289.1 |########################################
    293.4 |#################################
    297.8 |
    302.2 |######
    306.6 |##########################
    311.0 |
    315.4 |
    319.8 |
    324.2 |
    328.5 |
    332.9 |
    337.3 |#################################
    341.7 |#################################
    346.1 |
    350.5 |
    354.9 |
  (5 below, 1 above range)

satfold-lanes16-constl (n=40, range 264.9-302.8 ns)
    264.9 |####################
    266.8 |#############
    268.7 |
    270.6 |
    272.5 |#############
    274.4 |#############
    276.3 |####################
    278.2 |#############
    280.1 |
    282.0 |######
    283.9 |
    285.8 |
    287.7 |
    289.5 |
    291.4 |
    293.3 |####################
    295.2 |#################################
    297.1 |##########################
    299.0 |########################################
    300.9 |
  (4 below, 3 above range)

satfold-lanes4-idx (n=40, range 529.3-634.7 ns)
    529.3 |################
    534.6 |
    539.9 |
    545.1 |
    550.4 |
    555.7 |
    560.9 |##
    566.2 |########################################
    571.5 |##############
    576.8 |##
    582.0 |
    587.3 |
    592.6 |
    597.8 |
    603.1 |
    608.4 |
    613.6 |
    618.9 |
    624.2 |
    629.5 |
  (3 below, 5 above range)

satfold-lanes64 (n=40, range 227.6-278.1 ns)
    227.6 |####################################
    230.2 |########################################
    232.7 |##########
    235.2 |
    237.7 |
    240.2 |
    242.8 |
    245.3 |
    247.8 |
    250.3 |
    252.9 |
    255.4 |
    257.9 |
    260.4 |######
    262.9 |##########
    265.5 |##########
    268.0 |######
    270.5 |
    273.0 |
    275.6 |
  (3 below, 1 above range)

satfold-neon (n=40, range 234.2-282.4 ns)
    234.2 |################
    236.6 |
    239.0 |######
    241.5 |##########
    243.9 |########################################
    246.3 |######
    248.7 |######
    251.1 |
    253.5 |
    255.9 |
    258.3 |
    260.7 |
    263.1 |
    265.5 |
    267.9 |
    270.4 |
    272.8 |
    275.2 |
    277.6 |##########
    280.0 |#############
  (4 below, 3 above range)

satfold-neon8 (n=40, range 234.4-276.6 ns)
    234.4 |###############
    236.5 |##########
    238.7 |
    240.8 |
    242.9 |
    245.0 |
    247.1 |
    249.2 |#####
    251.3 |########################################
    253.4 |#########################
    255.5 |###############
    257.6 |###############
    259.7 |
    261.8 |
    263.9 |##########
    266.0 |#####
    268.1 |####################
    270.3 |#####
    272.4 |#####
    274.5 |
  (5 below, 1 above range)

satfold-nolaw (n=40, range 441.7-510.7 ns)
    441.7 |##################################
    445.1 |######################
    448.6 |#####
    452.0 |#####
    455.5 |#####
    458.9 |###########
    462.4 |
    465.8 |
    469.3 |
    472.7 |#####
    476.2 |########################################
    479.6 |############################
    483.1 |#################
    486.5 |###########
    490.0 |###########
    493.4 |
    496.9 |
    500.3 |
    503.8 |
    507.2 |
  (4 below, 1 above range)

satfold-seq (n=40, range 261.6-309.1 ns)
    261.6 |############################
    263.9 |########################
    266.3 |############
    268.7 |
    271.1 |
    273.4 |
    275.8 |
    278.2 |
    280.6 |
    282.9 |
    285.3 |
    287.7 |
    290.1 |
    292.5 |
    294.8 |
    297.2 |
    299.6 |####
    302.0 |########################################
    304.3 |################
    306.7 |############
  (4 below, 2 above range)

```

## Diagnostics

- **satfold-lanes16-constl**: autocorrelation=0.56 (measurement drift or warm-up artifact)
- **satfold-neon**: autocorrelation=0.87 (measurement drift or warm-up artifact)
- **satfold-neon8**: autocorrelation=0.63 (measurement drift or warm-up artifact)
- **satfold-seq**: autocorrelation=0.85 (measurement drift or warm-up artifact)
