# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-neon8 dominates: 42% faster than the next best (satfold-neon)

satfold-neon8 (255 ns) leads satfold-neon (362 ns) by 42%, a clear separation rather than a photo finish. CV 9.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### satfold-neon8 beats baseline by 100% (significant)

satfold-neon8 is -41.66 us (100%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-seq is an outlier: 163.6x slower than the field

satfold-seq (41.71 us) is 163.6x the fastest (255 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-neon8 is fastest but the noisiest (CV 9.9%)

satfold-neon8 wins on median (255 ns) yet has the highest variance (CV 9.9%), while satfold-iterfold is the steadiest (CV 0.9%, 41.60 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### satfold-lanes64 shows warm-up / thermal drift (autocorr +0.88)

satfold-lanes64's per-pass series has lag-1 autocorrelation +0.88, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-neon8, satfold-neon, satfold-lanes64, satfold-lanes16-constl, satfold-lanes16} vs {satfold-lanes4-idx, satfold-nolaw, satfold-iterfold, satfold-seq} (881% apart)

The field splits into a fast tier {satfold-neon8, satfold-neon, satfold-lanes64, satfold-lanes16-constl, satfold-lanes16} and a slow tier {satfold-lanes4-idx, satfold-nolaw, satfold-iterfold, satfold-seq} with a 881% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 163.6x the fastest

Fastest satfold-neon8 (255 ns) to slowest satfold-seq (41.71 us): 163.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-seq's edge over baseline is significant but tiny (-35 ns, 0.08%)

satfold-seq differs from baseline satfold-iterfold by -35 ns (0.08%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: satfold-neon8** at 255.0 ns median (-99.4% vs baseline)
- 7 variants significantly faster than baseline
- Spread: 163.58x (fastest 255.0 ns, slowest 41712.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 41897ns | 41708ns | 41572ns | 41789ns | 42544ns | base |
| satfold-lanes16 | 1662ns | 1649ns | 1642ns | 1653ns | 1706ns | -96.03% |
| satfold-lanes16-constl | 1668ns | 1645ns | 1641ns | 1649ns | 1750ns | -96.02% |
| satfold-lanes4-idx | 15347ns | 15119ns | 15085ns | 15174ns | 16127ns | -63.37% |
| satfold-lanes64 | 728ns | 731ns | 706ns | 730ns | 746ns | -98.26% |
| satfold-neon | 423ns | 423ns | 416ns | 423ns | 431ns | -98.99% |
| satfold-neon8 | 319ns | 317ns | 304ns | 314ns | 349ns | -99.24% |
| satfold-nolaw | 32307ns | 32101ns | 31864ns | 32169ns | 33165ns | -22.89% |
| satfold-seq | 42084ns | 41806ns | 41579ns | 41905ns | 43125ns | +0.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 41791ns | 41493ns | 42425ns | base | 0.784 |
| satfold-lanes16 | 1540ns | 1522ns | 1583ns | -96.31% | 21.274 |
| satfold-lanes16-constl | 1547ns | 1522ns | 1627ns | -96.30% | 21.182 |
| satfold-lanes4-idx | 15213ns | 14956ns | 15979ns | -63.60% | 2.154 |
| satfold-lanes64 | 667ns | 647ns | 683ns | -98.40% | 49.152 |
| satfold-neon | 363ns | 357ns | 370ns | -99.13% | 90.253 |
| satfold-neon8 | 257ns | 244ns | 285ns | -99.38% | 127.258 |
| satfold-nolaw | 32211ns | 31804ns | 33031ns | -22.93% | 1.017 |
| satfold-seq | 41979ns | 41496ns | 43006ns | +0.45% | 0.781 |

## Performance model

- Peak throughput: **134.295 Gops/s** (satfold-neon8; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 0.788 | 0.6% |
| satfold-lanes16 | 21.452 | 16.0% |
| satfold-lanes16-constl | 21.493 | 16.0% |
| satfold-lanes4-idx | 2.186 | 1.6% |
| satfold-lanes64 | 48.940 | 36.4% |
| satfold-neon | 90.444 | 67.3% |
| satfold-neon8 | 128.502 | 95.7% |
| satfold-nolaw | 1.024 | 0.8% |
| satfold-seq | 0.786 | 0.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 41897ns | 41897ns | base |
| satfold-lanes16 | 1662ns | 1662ns | -96.03% |
| satfold-lanes16-constl | 1668ns | 1668ns | -96.02% |
| satfold-lanes4-idx | 15347ns | 15347ns | -63.37% |
| satfold-lanes64 | 728ns | 728ns | -98.26% |
| satfold-neon | 423ns | 423ns | -98.99% |
| satfold-neon8 | 319ns | 319ns | -99.24% |
| satfold-nolaw | 32307ns | 32307ns | -22.89% |
| satfold-seq | 42084ns | 42084ns | +0.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 41596ns | base | --- | [41536, 41858] | --- | --- | --- | --- |
| satfold-lanes16 | 1528ns | -40058.3ns (-96.3%) | [-40321, -40013]ns | [1525, 1540] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 1525ns | -40045.0ns (-96.3%) | [-40275, -40003]ns | [1524, 1528] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 14989ns | -26563.6ns (-63.9%) | [-26643, -26512]ns | [14975, 15036] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 670ns | -40931.0ns (-98.4%) | [-41176, -40864]ns | [657, 680] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 362ns | -41236.7ns (-99.1%) | [-41490, -41171]ns | [361, 365] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 255ns | -41351.4ns (-99.4%) | [-41560, -41289]ns | [247, 257] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 31997ns | -9577.5ns (-23.0%) | [-9726, -9467]ns | [31903, 32241] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 41713ns | no significant difference | [-39, +279]ns | [41536, 42056] | no | 0.6358 | 0.6358 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 41511ns | -96.3% | -96.3% | -61.3% | -98.4% | -99.1% | -99.1% | -22.9% | +2.1% |
| 2 | 41529ns | -96.3% | -96.3% | -63.1% | -98.4% | -99.1% | -99.4% | -20.9% | +5.8% |
| 3 | 41517ns | -96.3% | -96.3% | -61.2% | -98.4% | -99.1% | -99.4% | -21.4% | +5.1% |
| 4 | 41568ns | -96.3% | -96.3% | -62.3% | -98.4% | -99.1% | -99.4% | -22.8% | +1.8% |
| 5 | 41844ns | -96.4% | -96.4% | -64.2% | -98.4% | -99.1% | -99.4% | -23.4% | +0.8% |
| 6 | 41987ns | -96.4% | -96.4% | -64.3% | -98.4% | -99.1% | -99.4% | -24.1% | +0.4% |
| 7 | 41526ns | -96.3% | -96.3% | -64.0% | -98.4% | -99.1% | -99.4% | -22.8% | +2.6% |
| 8 | 41512ns | -96.3% | -96.3% | -64.0% | -98.4% | -99.1% | -99.4% | -23.1% | +4.9% |
| 9 | 41571ns | -96.3% | -96.3% | -64.0% | -98.4% | -99.1% | -99.4% | -20.9% | +2.1% |
| 10 | 42743ns | -96.4% | -96.4% | -65.0% | -98.4% | -99.1% | -99.4% | -20.3% | -1.3% |
| 11 | 41508ns | -96.3% | -96.3% | -64.0% | -98.4% | -99.1% | -99.4% | -23.0% | +0.0% |
| 12 | 41470ns | -96.1% | -96.3% | -63.9% | -98.4% | -99.1% | -99.4% | -23.2% | +0.0% |
| 13 | 41524ns | -96.2% | -96.3% | -64.0% | -98.4% | -99.1% | -99.4% | -23.4% | -0.1% |
| 14 | 42350ns | -96.4% | -96.4% | -64.6% | -98.5% | -99.2% | -99.4% | -24.9% | -1.9% |
| 15 | 41905ns | -96.4% | -96.4% | -64.2% | -98.4% | -99.1% | -99.4% | -24.1% | -0.4% |
| 16 | 41495ns | -96.3% | -96.3% | -64.0% | -98.4% | -99.1% | -99.4% | -20.3% | +1.4% |
| 17 | 41493ns | -96.3% | -96.3% | -63.8% | -98.4% | -99.1% | -99.4% | -22.2% | +0.1% |
| 18 | 41485ns | -96.3% | -96.3% | -63.9% | -98.4% | -99.1% | -99.4% | -23.4% | +0.1% |
| 19 | 41892ns | -96.3% | -96.4% | -64.3% | -98.4% | -99.2% | -99.4% | -24.0% | -0.9% |
| 20 | 42275ns | -96.4% | -96.4% | -64.6% | -98.4% | -99.2% | -99.4% | -24.8% | -1.8% |
| 21 | 42035ns | -96.4% | -96.2% | -64.4% | -98.5% | -99.1% | -99.4% | -23.1% | +2.0% |
| 22 | 41580ns | -96.3% | -96.0% | -64.0% | -98.4% | -99.1% | -99.4% | -23.4% | +0.7% |
| 23 | 41971ns | -96.4% | -96.0% | -64.3% | -98.5% | -99.1% | -99.4% | -24.0% | +1.5% |
| 24 | 41709ns | -96.3% | -95.9% | -64.1% | -98.4% | -99.1% | -99.4% | -19.5% | +0.7% |
| 25 | 41628ns | -96.3% | -96.3% | -64.0% | -98.5% | -99.1% | -99.4% | -22.6% | +0.2% |
| 26 | 42498ns | -96.3% | -96.4% | -63.8% | -98.5% | -99.1% | -99.4% | -23.6% | -1.9% |
| 27 | 42291ns | -96.4% | -96.3% | -63.0% | -98.5% | -99.1% | -99.4% | -24.7% | -1.6% |
| 28 | 42667ns | -96.4% | -96.4% | -60.5% | -98.5% | -99.2% | -99.4% | -25.0% | -1.5% |
| 29 | 42010ns | -96.4% | -96.3% | -61.2% | -98.5% | -99.1% | -99.4% | -22.7% | -1.2% |
| 30 | 41550ns | -96.3% | -96.3% | -62.3% | -98.4% | -99.1% | -99.4% | -23.4% | +1.1% |
| 31 | 41613ns | -96.3% | -96.3% | -63.9% | -98.4% | -99.1% | -99.4% | -23.3% | -0.1% |
| 32 | 41472ns | -96.3% | -96.1% | -63.6% | -98.4% | -99.1% | -99.4% | -23.0% | -0.0% |
| 33 | 42540ns | -96.4% | -96.4% | -64.8% | -98.4% | -99.1% | -99.4% | -25.3% | -2.4% |
| 34 | 41872ns | -96.3% | -96.0% | -62.8% | -98.4% | -99.1% | -99.1% | -22.6% | -0.8% |
| 35 | 41526ns | -95.9% | -96.3% | -63.9% | -98.4% | -99.1% | -99.4% | -21.7% | -0.0% |
| 36 | 41775ns | -96.3% | -96.4% | -64.2% | -98.4% | -99.1% | -99.4% | -23.7% | -0.7% |
| 37 | 41511ns | -96.3% | -96.3% | -63.6% | -98.4% | -99.1% | -99.4% | -22.9% | +0.0% |
| 38 | 41613ns | -96.3% | -96.3% | -64.0% | -98.4% | -99.1% | -99.4% | -22.3% | -0.2% |
| 39 | 41549ns | -96.3% | -96.3% | -63.0% | -98.4% | -99.1% | -99.4% | -21.4% | +1.6% |
| 40 | 41543ns | -96.3% | -96.3% | -63.8% | -98.4% | -99.1% | -99.4% | -22.5% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.182 | ok |
| satfold-lanes16 | 0.236 | moderate+ |
| satfold-lanes16-constl | 0.473 | moderate+ |
| satfold-lanes4-idx | 0.585 | HIGH+ (drift/warm-up) |
| satfold-lanes64 | 0.882 | HIGH+ (drift/warm-up) |
| satfold-neon | 0.518 | HIGH+ (drift/warm-up) |
| satfold-neon8 | -0.046 | ok |
| satfold-nolaw | 0.116 | ok |
| satfold-seq | 0.563 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **satfold-lanes16**: won 40/40, lost 0/40
- **satfold-lanes16-constl**: won 40/40, lost 0/40
- **satfold-lanes4-idx**: won 40/40, lost 0/40
- **satfold-lanes64**: won 40/40, lost 0/40
- **satfold-neon**: won 40/40, lost 0/40
- **satfold-neon8**: won 40/40, lost 0/40
- **satfold-nolaw**: won 40/40, lost 0/40
- **satfold-seq**: won 14/40, lost 18/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 2.8ns | 41791.4ns | 0.0% |  |
| satfold-lanes16 | 2.8ns | 1540.3ns | 0.2% |  |
| satfold-lanes16-constl | 2.4ns | 1547.0ns | 0.2% |  |
| satfold-lanes4-idx | 3.4ns | 15213.1ns | 0.0% |  |
| satfold-lanes64 | 3.0ns | 666.7ns | 0.5% |  |
| satfold-neon | 2.3ns | 363.1ns | 0.6% |  |
| satfold-neon8 | 2.2ns | 257.5ns | 0.9% |  |
| satfold-nolaw | 2.3ns | 32210.7ns | 0.0% |  |
| satfold-seq | 2.8ns | 41978.9ns | 0.0% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 41493.0-42424.9 ns)
  41493.0 |########################################
  41539.6 |########################
  41586.2 |############
  41632.8 |
  41679.4 |####
  41726.0 |
  41772.6 |####
  41819.2 |####
  41865.8 |############
  41912.4 |
  41958.9 |########
  42005.5 |########
  42052.1 |
  42098.7 |
  42145.3 |
  42191.9 |
  42238.5 |####
  42285.1 |####
  42331.7 |####
  42378.3 |
  (4 below, 4 above range)

satfold-lanes16 (n=40, range 1521.9-1582.5 ns)
   1521.9 |########################################
   1525.0 |###############################
   1528.0 |####
   1531.0 |########
   1534.1 |
   1537.1 |########
   1540.1 |####
   1543.2 |####
   1546.2 |######################
   1549.2 |
   1552.2 |########
   1555.3 |####
   1558.3 |
   1561.3 |
   1564.4 |
   1567.4 |####
   1570.4 |
   1573.5 |
   1576.5 |
   1579.5 |####
  (5 below, 2 above range)

satfold-lanes16-constl (n=40, range 1522.0-1626.8 ns)
   1522.0 |########################################
   1527.2 |###
   1532.5 |#
   1537.7 |
   1542.9 |#######
   1548.2 |###
   1553.4 |
   1558.7 |
   1563.9 |
   1569.2 |
   1574.4 |#
   1579.6 |
   1584.9 |
   1590.1 |
   1595.4 |
   1600.6 |
   1605.9 |
   1611.1 |
   1616.3 |
   1621.6 |
  (3 below, 5 above range)

satfold-lanes4-idx (n=40, range 14956.2-15979.5 ns)
  14956.2 |########################################
  15007.4 |#####
  15058.5 |#
  15109.7 |#
  15160.9 |
  15212.0 |
  15263.2 |
  15314.3 |###
  15365.5 |#
  15416.7 |
  15467.8 |
  15519.0 |
  15570.2 |#
  15621.3 |#####
  15672.5 |
  15723.6 |
  15774.8 |
  15826.0 |
  15877.1 |
  15928.3 |
  (2 below, 4 above range)

satfold-lanes64 (n=40, range 646.8-682.5 ns)
    646.8 |###############
    648.6 |###############
    650.4 |
    652.2 |
    654.0 |#####
    655.7 |#########################
    657.5 |##########
    659.3 |##########
    661.1 |
    662.9 |
    664.7 |
    666.4 |
    668.2 |
    670.0 |
    671.8 |
    673.6 |
    675.4 |
    677.2 |###############
    678.9 |###################################
    680.7 |########################################
  (4 below, 2 above range)

satfold-neon (n=40, range 356.6-370.5 ns)
    356.6 |########
    357.3 |########
    358.0 |########################
    358.7 |################
    359.4 |########
    360.1 |
    360.8 |########################################
    361.5 |################################
    362.2 |########
    362.8 |########
    363.5 |########################
    364.2 |########
    364.9 |################
    365.6 |################################
    366.3 |########
    367.0 |################
    367.7 |########
    368.4 |
    369.1 |########
    369.8 |########
  (3 below, 2 above range)

satfold-neon8 (n=40, range 244.0-285.2 ns)
    244.0 |#############################
    246.1 |##########
    248.1 |#######
    250.2 |
    252.2 |#######
    254.3 |##################
    256.4 |########################################
    258.4 |##########
    260.5 |
    262.5 |
    264.6 |
    266.7 |
    268.7 |
    270.8 |
    272.8 |
    274.9 |
    277.0 |
    279.0 |
    281.1 |
    283.2 |
  (4 below, 2 above range)

satfold-nolaw (n=40, range 31804.0-33031.2 ns)
  31804.0 |########################################
  31865.4 |########################################
  31926.7 |#############
  31988.1 |####################
  32049.5 |#############
  32110.8 |
  32172.2 |#############
  32233.5 |######
  32294.9 |#############
  32356.3 |######
  32417.6 |#############
  32479.0 |######
  32540.3 |
  32601.7 |#############
  32663.1 |
  32724.4 |
  32785.8 |
  32847.2 |#############
  32908.5 |
  32969.9 |
  (5 below, 3 above range)

satfold-seq (n=40, range 41496.1-43006.5 ns)
  41496.1 |########################################
  41571.6 |##
  41647.1 |##
  41722.7 |#####
  41798.2 |##
  41873.7 |
  41949.2 |#####
  42024.7 |#####
  42100.3 |##
  42175.8 |########
  42251.3 |##
  42326.8 |##
  42402.3 |##
  42477.8 |
  42553.4 |#####
  42628.9 |
  42704.4 |
  42779.9 |
  42855.4 |##
  42930.9 |
  (4 below, 3 above range)

```

## Diagnostics

- **satfold-lanes4-idx**: autocorrelation=0.58 (measurement drift or warm-up artifact)
- **satfold-lanes64**: autocorrelation=0.88 (measurement drift or warm-up artifact)
- **satfold-neon**: autocorrelation=0.52 (measurement drift or warm-up artifact)
- **satfold-seq**: autocorrelation=0.56 (measurement drift or warm-up artifact)
