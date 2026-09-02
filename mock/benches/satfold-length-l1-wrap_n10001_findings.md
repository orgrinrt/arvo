# The identical arms over wrapping addition, which the backend may reassociate with no help from any typestate: the ceiling every saturating arm is measured against

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-lanes4-idx is an outlier: 2.4x slower than the field

satfold-lanes4-idx (558 ns) is 2.4x the fastest (229 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (satfold-lanes64, satfold-lanes16) are a dead heat (<1%)

satfold-lanes64 (229 ns) and satfold-lanes16 (229 ns) differ by 0.09%, inside the noise, even though the wider field spreads 143.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### satfold-lanes16 shows warm-up / thermal drift (autocorr +0.90)

satfold-lanes16's per-pass series has lag-1 autocorrelation +0.90, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-lanes64, satfold-lanes16, satfold-iterfold, satfold-lanes16-constl, satfold-nolaw, satfold-seq, satfold-neon, satfold-neon8} vs {satfold-lanes4-idx} (77% apart)

The field splits into a fast tier {satfold-lanes64, satfold-lanes16, satfold-iterfold, satfold-lanes16-constl, satfold-nolaw, satfold-seq, satfold-neon, satfold-neon8} and a slow tier {satfold-lanes4-idx} with a 77% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### satfold-nolaw is inconsistent: worst-20% is 1.6x its best-20%

satfold-nolaw's best 20% of batches run at 227 ns but its worst 20% at 360 ns (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### satfold-lanes16's edge over baseline is significant but tiny (-3 ns, 1.15%)

satfold-lanes16 differs from baseline satfold-iterfold by -3 ns (1.15%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: satfold-lanes64** at 229.2 ns median (-2.3% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 2.43x (fastest 229.2 ns, slowest 557.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 313ns | 299ns | 284ns | 304ns | 369ns | base |
| satfold-lanes16 | 302ns | 294ns | 281ns | 295ns | 342ns | -3.58% |
| satfold-lanes16-constl | 318ns | 299ns | 284ns | 301ns | 404ns | +1.66% |
| satfold-lanes4-idx | 626ns | 622ns | 563ns | 621ns | 702ns | +99.96% |
| satfold-lanes64 | 294ns | 290ns | 281ns | 292ns | 315ns | -6.03% |
| satfold-neon | 367ns | 367ns | 345ns | 366ns | 392ns | +17.38% |
| satfold-neon8 | 382ns | 381ns | 345ns | 382ns | 419ns | +22.14% |
| satfold-nolaw | 341ns | 308ns | 288ns | 320ns | 459ns | +9.04% |
| satfold-seq | 312ns | 312ns | 284ns | 311ns | 343ns | -0.34% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 247ns | 224ns | 297ns | base | 132.456 |
| satfold-lanes16 | 237ns | 221ns | 271ns | -4.08% | 138.085 |
| satfold-lanes16-constl | 247ns | 222ns | 303ns | -0.11% | 132.600 |
| satfold-lanes4-idx | 560ns | 503ns | 631ns | +126.38% | 58.511 |
| satfold-lanes64 | 231ns | 221ns | 247ns | -6.46% | 141.609 |
| satfold-neon | 303ns | 285ns | 324ns | +22.52% | 108.111 |
| satfold-neon8 | 315ns | 286ns | 347ns | +27.51% | 103.879 |
| satfold-nolaw | 268ns | 227ns | 360ns | +8.46% | 122.125 |
| satfold-seq | 245ns | 222ns | 271ns | -0.80% | 133.523 |

## Performance model

- Peak throughput: **148.566 Gops/s** (satfold-lanes16; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 139.676 | 94.0% |
| satfold-lanes16 | 142.873 | 96.2% |
| satfold-lanes16-constl | 138.847 | 93.5% |
| satfold-lanes4-idx | 58.735 | 39.5% |
| satfold-lanes64 | 142.998 | 96.3% |
| satfold-neon | 108.467 | 73.0% |
| satfold-neon8 | 104.224 | 70.2% |
| satfold-nolaw | 134.543 | 90.6% |
| satfold-seq | 133.312 | 89.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 313ns | 313ns | base |
| satfold-lanes16 | 302ns | 302ns | -3.58% |
| satfold-lanes16-constl | 318ns | 318ns | +1.66% |
| satfold-lanes4-idx | 626ns | 626ns | +99.96% |
| satfold-lanes64 | 294ns | 294ns | -6.03% |
| satfold-neon | 367ns | 367ns | +17.38% |
| satfold-neon8 | 382ns | 382ns | +22.14% |
| satfold-nolaw | 341ns | 341ns | +9.04% |
| satfold-seq | 312ns | 312ns | -0.34% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 235ns | base | --- | [234, 237] | --- | --- | --- | --- |
| satfold-lanes16 | 229ns | no significant difference | [-5, +2]ns | [224, 234] | no | 0.5364 | 0.2682 | 0 |
| satfold-lanes16-constl | 236ns | no significant difference | [-4, +2]ns | [235, 238] | no | 0.8563 | 0.7493 | 1 |
| satfold-lanes4-idx | 558ns | +309.6ns (+131.9%) | [+296, +339]ns | [533, 573] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 229ns | no significant difference | [-3, +1]ns | [224, 235] | no | 0.8361 | 0.6271 | 2 |
| satfold-neon | 302ns | +66.7ns (+28.4%) | [+65, +68]ns | [300, 304] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 314ns | +65.2ns (+27.8%) | [+61, +71]ns | [304, 324] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 244ns | no significant difference | [-4, +18]ns | [234, 252] | no | 0.8746 | 0.8746 | 0 |
| satfold-seq | 246ns | no significant difference | [-1, +4]ns | [235, 252] | no | 0.6683 | 0.4177 | 2 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 456ns | -51.8% | -48.1% | +10.4% | -52.0% | -37.6% | -30.1% | -49.8% | -41.5% |
| 2 | 271ns | -18.3% | -14.3% | +85.9% | -17.8% | +5.4% | +19.9% | -15.8% | +0.0% |
| 3 | 273ns | -18.6% | -13.4% | +84.6% | -17.1% | +4.1% | +18.6% | -18.0% | -0.6% |
| 4 | 269ns | -17.6% | -12.4% | +87.0% | -17.1% | +5.4% | +20.2% | -16.4% | +0.3% |
| 5 | 272ns | -19.4% | -13.6% | +84.7% | -18.5% | +5.7% | +17.6% | -17.1% | +0.5% |
| 6 | 271ns | -18.3% | -14.3% | +85.6% | -17.8% | +5.5% | +19.7% | -13.8% | -0.3% |
| 7 | 269ns | -17.5% | -11.9% | +87.1% | -16.7% | +6.8% | +21.1% | -13.0% | +0.9% |
| 8 | 273ns | -19.7% | -13.9% | +100.9% | -18.5% | +4.9% | +19.5% | -15.7% | +0.0% |
| 9 | 263ns | -16.1% | -11.1% | +94.0% | -13.6% | +7.9% | +22.9% | -12.0% | +1.3% |
| 10 | 271ns | -18.5% | -13.4% | +86.2% | -18.5% | +4.2% | +19.2% | -16.8% | +0.3% |
| 11 | 229ns | -2.4% | -2.6% | +167.0% | -3.8% | +39.1% | +25.5% | +58.1% | -3.8% |
| 12 | 228ns | -1.4% | -3.5% | +169.2% | -3.1% | +41.1% | +25.8% | +57.3% | -4.2% |
| 13 | 218ns | +2.3% | +1.7% | +178.3% | +1.3% | +49.2% | +30.7% | +64.7% | +1.3% |
| 14 | 222ns | +0.8% | +0.4% | +176.7% | +0.5% | +46.0% | +29.4% | +61.9% | +0.4% |
| 15 | 224ns | +1.3% | -0.6% | +173.9% | +0.0% | +45.2% | +29.0% | +60.1% | -0.2% |
| 16 | 222ns | +2.3% | +0.4% | +176.4% | -1.1% | +44.6% | +29.3% | +62.4% | +1.8% |
| 17 | 222ns | +2.4% | +0.8% | +175.4% | +0.5% | +46.5% | +28.0% | +61.0% | +1.3% |
| 18 | 224ns | +0.5% | -1.3% | +173.6% | +0.4% | +42.9% | +27.3% | +60.2% | -0.4% |
| 19 | 291ns | -22.5% | -24.2% | +109.0% | -22.7% | +10.6% | -2.3% | +23.0% | -22.5% |
| 20 | 228ns | -1.6% | -1.8% | +230.8% | -1.6% | +43.0% | +25.1% | +57.3% | -3.3% |
| 21 | 234ns | -0.9% | +1.9% | +128.8% | +0.7% | +29.2% | +30.1% | +1.4% | +2.8% |
| 22 | 235ns | -1.6% | -0.2% | +126.8% | +0.0% | +28.3% | +30.1% | -0.2% | -0.7% |
| 23 | 235ns | -0.2% | +0.0% | +128.6% | +1.2% | +29.7% | +31.2% | -0.3% | +1.6% |
| 24 | 238ns | -1.4% | +180.7% | +122.6% | -0.3% | +27.9% | +28.0% | -1.7% | -1.2% |
| 25 | 240ns | -3.7% | +0.3% | +123.1% | +0.5% | +26.4% | +28.5% | -1.0% | -1.2% |
| 26 | 234ns | -0.2% | +0.7% | +127.9% | +3.4% | +29.9% | +30.3% | -1.5% | +3.0% |
| 27 | 236ns | -1.0% | +0.7% | +124.6% | +0.4% | +29.0% | +27.5% | -1.6% | -0.2% |
| 28 | 237ns | -1.9% | +0.2% | +126.0% | -0.9% | +28.5% | +28.1% | -1.8% | -1.1% |
| 29 | 240ns | -2.6% | -0.2% | +120.5% | -0.7% | +27.0% | +26.4% | -1.0% | -1.8% |
| 30 | 236ns | -0.3% | +0.6% | +125.4% | -1.0% | +28.6% | +31.2% | -2.8% | -1.4% |
| 31 | 233ns | +15.9% | +7.0% | +144.1% | -0.5% | +28.6% | +47.7% | +8.4% | +7.9% |
| 32 | 232ns | +16.5% | +8.0% | +146.4% | +1.8% | +29.2% | +49.3% | +9.1% | +8.6% |
| 33 | 232ns | +17.6% | +8.4% | +148.0% | -0.4% | +30.6% | +50.0% | +8.8% | +8.2% |
| 34 | 233ns | +17.0% | +8.1% | +145.5% | +0.9% | +29.2% | +49.2% | +7.7% | +8.9% |
| 35 | 232ns | +17.3% | +8.1% | +146.0% | +1.4% | +30.4% | +49.4% | +8.1% | +8.8% |
| 36 | 236ns | +13.4% | +6.4% | +141.9% | +6.9% | +27.6% | +46.6% | +5.9% | +7.1% |
| 37 | 234ns | +15.3% | +7.5% | +148.0% | +7.5% | +27.9% | +48.5% | +7.3% | +7.8% |
| 38 | 235ns | +13.0% | +7.5% | +144.6% | +6.7% | +27.5% | +47.8% | +6.9% | +7.6% |
| 39 | 234ns | +14.2% | +7.1% | +145.8% | +7.8% | +28.3% | +48.1% | +7.8% | +7.3% |
| 40 | 234ns | +16.7% | +6.9% | +142.3% | +7.5% | +28.4% | +49.3% | +7.5% | +7.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.239 | moderate+ |
| satfold-lanes16 | 0.900 | HIGH+ (drift/warm-up) |
| satfold-lanes16-constl | 0.002 | ok |
| satfold-lanes4-idx | 0.554 | HIGH+ (drift/warm-up) |
| satfold-lanes64 | 0.840 | HIGH+ (drift/warm-up) |
| satfold-neon | 0.839 | HIGH+ (drift/warm-up) |
| satfold-neon8 | 0.895 | HIGH+ (drift/warm-up) |
| satfold-nolaw | 0.842 | HIGH+ (drift/warm-up) |
| satfold-seq | 0.835 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **satfold-lanes16**: won 24/40, lost 16/40
- **satfold-lanes16-constl**: won 18/40, lost 21/40
- **satfold-lanes4-idx**: won 0/40, lost 40/40
- **satfold-lanes64**: won 21/40, lost 17/40
- **satfold-neon**: won 1/40, lost 39/40
- **satfold-neon8**: won 2/40, lost 38/40
- **satfold-nolaw**: won 19/40, lost 21/40
- **satfold-seq**: won 16/40, lost 22/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 2.5ns | 247.4ns | 1.0% |  |
| satfold-lanes16 | 3.3ns | 237.3ns | 1.4% |  |
| satfold-lanes16-constl | 2.7ns | 247.1ns | 1.1% |  |
| satfold-lanes4-idx | 3.3ns | 560.0ns | 0.6% |  |
| satfold-lanes64 | 3.0ns | 231.4ns | 1.3% |  |
| satfold-neon | 2.3ns | 303.1ns | 0.7% |  |
| satfold-neon8 | 2.3ns | 315.4ns | 0.7% |  |
| satfold-nolaw | 3.1ns | 268.3ns | 1.2% |  |
| satfold-seq | 2.7ns | 245.4ns | 1.1% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 223.6-297.3 ns)
    223.6 |######
    227.2 |##########
    230.9 |########################################
    234.6 |####################
    238.3 |######
    242.0 |
    245.7 |
    249.3 |
    253.0 |
    256.7 |
    260.4 |###
    264.1 |
    267.8 |################
    271.5 |##########
    275.1 |
    278.8 |
    282.5 |
    286.2 |
    289.9 |###
    293.6 |
  (4 below, 1 above range)

satfold-lanes16 (n=40, range 220.6-271.1 ns)
    220.6 |########################################
    223.1 |##################################
    225.6 |######################
    228.1 |
    230.7 |######################
    233.2 |##################################
    235.7 |
    238.2 |
    240.8 |
    243.3 |
    245.8 |
    248.4 |
    250.9 |
    253.4 |
    255.9 |
    258.5 |
    261.0 |
    263.5 |#####
    266.0 |###########
    268.6 |#################
  (3 below, 4 above range)

satfold-lanes16-constl (n=40, range 222.0-303.4 ns)
    222.0 |####################
    226.0 |
    230.1 |#####
    234.2 |########################################
    238.2 |########
    242.3 |
    246.4 |###########
    250.5 |#################
    254.5 |
    258.6 |
    262.7 |
    266.7 |
    270.8 |
    274.9 |
    279.0 |
    283.0 |
    287.1 |
    291.2 |
    295.2 |
    299.3 |
  (3 below, 1 above range)

satfold-lanes4-idx (n=40, range 503.4-630.5 ns)
    503.4 |#################
    509.7 |#####
    516.1 |
    522.4 |#####
    528.8 |########################################
    535.2 |###########
    541.5 |
    547.9 |#####
    554.2 |
    560.6 |
    566.9 |##################################
    573.3 |#################
    579.7 |#####
    586.0 |
    592.4 |
    598.7 |
    605.1 |#################
    611.5 |##################################
    617.8 |
    624.2 |
  (5 below, 1 above range)

satfold-lanes64 (n=40, range 220.8-247.3 ns)
    220.8 |####################
    222.2 |########################################
    223.5 |####################
    224.8 |#############
    226.1 |######
    227.5 |######
    228.8 |
    230.1 |######
    231.4 |######
    232.8 |######
    234.1 |#################################
    235.4 |######
    236.7 |##########################
    238.1 |
    239.4 |
    240.7 |#############
    242.0 |
    243.4 |
    244.7 |
    246.0 |
  (4 below, 5 above range)

satfold-neon (n=40, range 284.5-323.9 ns)
    284.5 |####################
    286.5 |##########
    288.5 |
    290.4 |
    292.4 |
    294.4 |
    296.3 |
    298.3 |####################
    300.3 |###################################
    302.2 |########################################
    304.2 |#####
    306.2 |
    308.2 |
    310.1 |
    312.1 |
    314.1 |
    316.0 |
    318.0 |#####
    320.0 |##########
    321.9 |##########
  (4 below, 5 above range)

satfold-neon8 (n=40, range 285.7-347.3 ns)
    285.7 |######################
    288.8 |#####
    291.9 |
    294.9 |
    298.0 |
    301.1 |#################
    304.2 |######################
    307.3 |#################
    310.3 |
    313.4 |
    316.5 |#####
    319.6 |#####
    322.7 |########################################
    325.8 |#####
    328.8 |
    331.9 |
    335.0 |
    338.1 |
    341.2 |
    344.3 |##################################
  (5 below, 4 above range)

satfold-nolaw (n=40, range 227.1-359.6 ns)
    227.1 |########################################
    233.7 |###############################
    240.3 |
    247.0 |########################################
    253.6 |####
    260.2 |
    266.8 |
    273.5 |
    280.1 |
    286.7 |
    293.3 |
    300.0 |
    306.6 |
    313.2 |
    319.8 |
    326.5 |
    333.1 |
    339.7 |
    346.3 |
    353.0 |##########################
  (4 below, 4 above range)

satfold-seq (n=40, range 221.8-271.5 ns)
    221.8 |###############
    224.3 |###############
    226.8 |
    229.3 |
    231.7 |###############
    234.2 |###############
    236.7 |##########
    239.2 |##########
    241.7 |
    244.1 |
    246.6 |
    249.1 |##########
    251.6 |########################################
    254.1 |
    256.6 |
    259.0 |
    261.5 |
    264.0 |
    266.5 |##########
    269.0 |#########################
  (4 below, 3 above range)

```

## Diagnostics

- **satfold-lanes16**: autocorrelation=0.90 (measurement drift or warm-up artifact)
- **satfold-lanes16-constl**: CV=27.6% (high variance, measurements may be unstable)
- **satfold-lanes4-idx**: autocorrelation=0.55 (measurement drift or warm-up artifact)
- **satfold-lanes64**: autocorrelation=0.84 (measurement drift or warm-up artifact)
- **satfold-neon**: autocorrelation=0.84 (measurement drift or warm-up artifact)
- **satfold-neon8**: autocorrelation=0.89 (measurement drift or warm-up artifact)
- **satfold-nolaw**: autocorrelation=0.84 (measurement drift or warm-up artifact)
- **satfold-seq**: autocorrelation=0.83 (measurement drift or warm-up artifact)
