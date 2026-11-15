# Clamping fold at 13 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-accfit dominates: 13% faster than the next best (warm-clamp-head)

warm-clamp-accfit (208 ns) leads warm-clamp-head (235 ns) by 13%, a clear separation rather than a photo finish. CV 10.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### warm-clamp-accfit beats baseline by 59% (significant)

warm-clamp-accfit is -317 ns (59%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-minimum is an outlier: 34.3x slower than the field

warm-clamp-minimum (7.13 us) is 34.3x the fastest (208 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-accfit shows warm-up / thermal drift (autocorr +0.83)

warm-clamp-accfit's per-pass series has lag-1 autocorrelation +0.83, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-accfit, warm-clamp-head, warm-clamp-accfit-dyn, warm-clamp-acc64, warm-clamp-min-lanes} vs {warm-clamp-minimum} (1092% apart)

The field splits into a fast tier {warm-clamp-accfit, warm-clamp-head, warm-clamp-accfit-dyn, warm-clamp-acc64, warm-clamp-min-lanes} and a slow tier {warm-clamp-minimum} with a 1092% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 34.3x the fastest

Fastest warm-clamp-accfit (208 ns) to slowest warm-clamp-minimum (7.13 us): 34.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-clamp-accfit** at 207.9 ns median (-61.3% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 34.31x (fastest 207.9 ns, slowest 7133.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 594ns | 604ns | 571ns | 592ns | 622ns | base |
| warm-clamp-accfit | 281ns | 277ns | 247ns | 278ns | 323ns | -52.70% |
| warm-clamp-accfit-dyn | 340ns | 340ns | 337ns | 340ns | 342ns | -42.80% |
| warm-clamp-head | 305ns | 295ns | 288ns | 301ns | 336ns | -48.61% |
| warm-clamp-min-lanes | 679ns | 658ns | 649ns | 668ns | 741ns | +14.36% |
| warm-clamp-minimum | 7609ns | 7207ns | 6958ns | 7333ns | 9087ns | +1181.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 531ns | 512ns | 557ns | base | 15.426 |
| warm-clamp-accfit | 211ns | 186ns | 243ns | -60.23% | 38.791 |
| warm-clamp-accfit-dyn | 279ns | 276ns | 281ns | -47.54% | 29.403 |
| warm-clamp-head | 243ns | 228ns | 268ns | -54.33% | 33.778 |
| warm-clamp-min-lanes | 616ns | 590ns | 672ns | +15.99% | 13.300 |
| warm-clamp-minimum | 7527ns | 6887ns | 8977ns | +1317.34% | 1.088 |

## Performance model

- Peak throughput: **44.022 Gops/s** (warm-clamp-accfit; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 15.248 | 34.6% |
| warm-clamp-accfit | 39.394 | 89.5% |
| warm-clamp-accfit-dyn | 29.409 | 66.8% |
| warm-clamp-head | 34.860 | 79.2% |
| warm-clamp-min-lanes | 13.691 | 31.1% |
| warm-clamp-minimum | 1.148 | 2.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 594ns | 594ns | base |
| warm-clamp-accfit | 281ns | 281ns | -52.70% |
| warm-clamp-accfit-dyn | 340ns | 340ns | -42.80% |
| warm-clamp-head | 305ns | 305ns | -48.61% |
| warm-clamp-min-lanes | 679ns | 679ns | +14.36% |
| warm-clamp-minimum | 7609ns | 7609ns | +1181.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 537ns | base | --- | [515, 541] | --- | --- | --- | --- |
| warm-clamp-accfit | 208ns | -320.4ns (-59.6%) | [-323, -317]ns | [192, 223] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 279ns | -257.7ns (-48.0%) | [-262, -237]ns | [278, 279] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 235ns | -282.5ns (-52.6%) | [-289, -279]ns | [232, 242] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 598ns | +77.3ns (+14.4%) | [+76, +79]ns | [592, 608] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 7134ns | +6592.3ns (+1227.0%) | [+6516, +6759]ns | [7037, 7303] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 514ns | -62.9% | -45.1% | -55.3% | +15.5% | +1495.1% |
| 2 | 515ns | -62.3% | -45.4% | -55.1% | +15.0% | +1492.7% |
| 3 | 518ns | -62.8% | -46.5% | -56.6% | +14.2% | +1541.9% |
| 4 | 515ns | -61.8% | -45.3% | -54.7% | +15.4% | +1553.1% |
| 5 | 517ns | -62.4% | -46.7% | -54.6% | +14.3% | +1545.2% |
| 6 | 515ns | -62.8% | -45.8% | -55.3% | +14.6% | +1305.9% |
| 7 | 511ns | -62.2% | -45.7% | -53.1% | +15.3% | +1272.9% |
| 8 | 516ns | -62.7% | -45.6% | -54.4% | +14.7% | +1259.7% |
| 9 | 515ns | -62.2% | -46.0% | -43.0% | +15.0% | +1243.9% |
| 10 | 513ns | -62.7% | -45.6% | -54.5% | +24.6% | +1242.2% |
| 11 | 539ns | -58.9% | -48.1% | -51.8% | +11.5% | +1201.8% |
| 12 | 584ns | -61.1% | -52.1% | -55.1% | +3.0% | +1104.5% |
| 13 | 540ns | -58.5% | -47.7% | -52.4% | +11.3% | +1211.4% |
| 14 | 538ns | -58.8% | -48.1% | -51.5% | +11.0% | +1204.5% |
| 15 | 539ns | -58.9% | -48.6% | -51.0% | +11.4% | +1202.3% |
| 16 | 542ns | -58.9% | -48.8% | -51.1% | +14.3% | +1195.5% |
| 17 | 540ns | -59.1% | -48.6% | -50.1% | +10.9% | +1222.6% |
| 18 | 542ns | -59.5% | -48.8% | -50.7% | +14.0% | +1246.5% |
| 19 | 573ns | -60.5% | -51.3% | -54.6% | +5.6% | +1239.4% |
| 20 | 536ns | -58.4% | -48.1% | -51.2% | +13.8% | +1188.7% |
| 21 | 515ns | -63.8% | -46.4% | -55.1% | +15.1% | +1213.3% |
| 22 | 511ns | -63.3% | -46.3% | -54.2% | +16.0% | +1226.7% |
| 23 | 514ns | -63.4% | -45.9% | -55.3% | +15.1% | +1271.1% |
| 24 | 572ns | -67.1% | -51.3% | -59.6% | +3.1% | +1162.4% |
| 25 | 514ns | -64.0% | -46.3% | -54.5% | +14.9% | +1302.2% |
| 26 | 512ns | -63.9% | -45.5% | -53.3% | +15.9% | +1310.6% |
| 27 | 511ns | -63.7% | -45.4% | -52.2% | +15.4% | +1258.6% |
| 28 | 513ns | -63.6% | -45.6% | -51.7% | +14.8% | +1273.5% |
| 29 | 513ns | -63.8% | -45.5% | -52.9% | +14.9% | +1271.0% |
| 30 | 514ns | -63.6% | -46.0% | -54.1% | +14.7% | +1241.2% |
| 31 | 542ns | -55.2% | -48.5% | -57.6% | +23.4% | +1212.5% |
| 32 | 544ns | -55.4% | -48.2% | -57.5% | +23.3% | +1205.7% |
| 33 | 546ns | -55.6% | -49.4% | -58.9% | +23.0% | +1238.6% |
| 34 | 542ns | -55.3% | -48.3% | -57.8% | +23.8% | +1489.4% |
| 35 | 544ns | -55.4% | -48.9% | -57.4% | +23.3% | +1279.2% |
| 36 | 543ns | -55.5% | -49.0% | -56.6% | +24.0% | +1420.0% |
| 37 | 542ns | -55.2% | -48.7% | -55.3% | +23.9% | +1661.4% |
| 38 | 545ns | -55.6% | -49.0% | -57.1% | +23.7% | +1985.9% |
| 39 | 540ns | -55.2% | -48.1% | -57.3% | +24.3% | +1477.0% |
| 40 | 543ns | -55.6% | -48.7% | -57.2% | +23.2% | +1258.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.440 | moderate+ |
| warm-clamp-accfit | 0.831 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit-dyn | 0.004 | ok |
| warm-clamp-head | 0.508 | HIGH+ (drift/warm-up) |
| warm-clamp-min-lanes | 0.830 | HIGH+ (drift/warm-up) |
| warm-clamp-minimum | 0.669 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-clamp-accfit**: won 40/40, lost 0/40
- **warm-clamp-accfit-dyn**: won 40/40, lost 0/40
- **warm-clamp-head**: won 40/40, lost 0/40
- **warm-clamp-min-lanes**: won 0/40, lost 40/40
- **warm-clamp-minimum**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 2.8ns | 531.0ns | 0.5% |  |
| warm-clamp-accfit | 2.8ns | 211.2ns | 1.3% |  |
| warm-clamp-accfit-dyn | 2.8ns | 278.6ns | 1.0% |  |
| warm-clamp-head | 2.4ns | 242.5ns | 1.0% |  |
| warm-clamp-min-lanes | 2.3ns | 615.9ns | 0.4% |  |
| warm-clamp-minimum | 2.9ns | 7526.6ns | 0.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 512.2-556.5 ns)
    512.2 |########################################
    514.4 |##################################
    516.7 |###########
    518.9 |
    521.1 |
    523.3 |
    525.5 |
    527.7 |
    529.9 |
    532.2 |
    534.4 |#####
    536.6 |#####
    538.8 |############################
    541.0 |##################################
    543.2 |######################
    545.4 |#####
    547.7 |
    549.9 |
    552.1 |
    554.3 |
  (4 below, 3 above range)

warm-clamp-accfit (n=40, range 186.1-242.5 ns)
    186.1 |##################################
    188.9 |#################
    191.7 |############################
    194.5 |###########
    197.4 |
    200.2 |
    203.0 |
    205.8 |
    208.7 |
    211.5 |
    214.3 |
    217.1 |#####
    219.9 |######################
    222.8 |#################
    225.6 |###########
    228.4 |
    231.2 |
    234.0 |
    236.9 |
    239.7 |########################################
  (4 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 276.1-281.2 ns)
    276.1 |#############
    276.4 |
    276.6 |
    276.9 |##########################
    277.2 |
    277.4 |####################
    277.7 |##########################
    277.9 |
    278.2 |##########################
    278.4 |
    278.7 |####################
    278.9 |########################################
    279.2 |
    279.5 |#############
    279.7 |
    280.0 |#############
    280.2 |#############
    280.5 |
    280.7 |
    281.0 |######
  (3 below, 4 above range)

warm-clamp-head (n=40, range 228.5-268.0 ns)
    228.5 |########################################
    230.4 |########################################
    232.4 |#################################
    234.4 |##########################
    236.4 |
    238.3 |#############
    240.3 |######
    242.3 |######
    244.3 |######
    246.2 |######
    248.2 |
    250.2 |
    252.2 |
    254.1 |
    256.1 |######
    258.1 |#############
    260.1 |#############
    262.0 |#############
    264.0 |######
    266.0 |######
  (2 below, 2 above range)

warm-clamp-min-lanes (n=40, range 589.9-671.9 ns)
    589.9 |########################################
    594.0 |##
    598.1 |############
    602.2 |##
    606.3 |
    610.4 |##
    614.5 |##
    618.6 |##
    622.7 |
    626.8 |
    630.9 |
    635.0 |
    639.1 |##
    643.2 |
    647.3 |
    651.4 |
    655.5 |
    659.6 |
    663.7 |
    667.8 |#################
  (3 below, 3 above range)

warm-clamp-minimum (n=40, range 6886.8-8977.0 ns)
   6886.8 |##############
   6991.3 |########################################
   7095.8 |##########
   7200.3 |##################
   7304.8 |#######
   7409.4 |###
   7513.9 |
   7618.4 |###
   7722.9 |
   7827.4 |
   7931.9 |
   8036.4 |
   8140.9 |#######
   8245.4 |###
   8349.9 |
   8454.4 |##############
   8558.9 |###
   8663.5 |
   8768.0 |
   8872.5 |
  (3 below, 2 above range)

```

## Diagnostics

- **warm-clamp-accfit**: autocorrelation=0.83 (measurement drift or warm-up artifact)
- **warm-clamp-head**: autocorrelation=0.51 (measurement drift or warm-up artifact)
- **warm-clamp-min-lanes**: autocorrelation=0.83 (measurement drift or warm-up artifact)
- **warm-clamp-minimum**: autocorrelation=0.67 (measurement drift or warm-up artifact)
