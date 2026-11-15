# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-neon beats baseline by 90% (significant)

satfold-neon is -9.67 us (90%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-lanes16 is an outlier: 6.5x slower than the field

satfold-lanes16 (10.79 us) is 6.5x the fastest (1.65 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (satfold-neon, satfold-neon8) are a dead heat (<1%)

satfold-neon (1.65 us) and satfold-neon8 (1.67 us) differ by 0.95%, inside the noise, even though the wider field spreads 553.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### satfold-neon shows warm-up / thermal drift (autocorr +0.83)

satfold-neon's per-pass series has lag-1 autocorrelation +0.83, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-neon, satfold-neon8} vs {satfold-lanes4-idx, satfold-lanes16-constl, satfold-seq, satfold-nolaw, satfold-iterfold, satfold-lanes64, satfold-lanes16} (370% apart)

The field splits into a fast tier {satfold-neon, satfold-neon8} and a slow tier {satfold-lanes4-idx, satfold-lanes16-constl, satfold-seq, satfold-nolaw, satfold-iterfold, satfold-lanes64, satfold-lanes16} with a 370% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.5x the fastest

Fastest satfold-neon (1.65 us) to slowest satfold-lanes16 (10.79 us): 6.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: satfold-neon** at 1651.2 ns median (-84.6% vs baseline)
- 5 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 6.53x (fastest 1651.2 ns, slowest 10787.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 11191ns | 10806ns | 10601ns | 11044ns | 12223ns | base |
| satfold-lanes16 | 11007ns | 10849ns | 10660ns | 10965ns | 11478ns | -1.65% |
| satfold-lanes16-constl | 10843ns | 10725ns | 10640ns | 10754ns | 11313ns | -3.11% |
| satfold-lanes4-idx | 8052ns | 7908ns | 7798ns | 7953ns | 8600ns | -28.05% |
| satfold-lanes64 | 10966ns | 10824ns | 10585ns | 10897ns | 11555ns | -2.01% |
| satfold-neon | 1725ns | 1712ns | 1703ns | 1717ns | 1771ns | -84.59% |
| satfold-neon8 | 1810ns | 1729ns | 1718ns | 1755ns | 2065ns | -83.83% |
| satfold-nolaw | 11224ns | 10802ns | 10762ns | 10990ns | 12387ns | +0.29% |
| satfold-seq | 10728ns | 10760ns | 10573ns | 10733ns | 10866ns | -4.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 11123ns | 10541ns | 12150ns | base | 2.946 |
| satfold-lanes16 | 10943ns | 10598ns | 11407ns | -1.62% | 2.994 |
| satfold-lanes16-constl | 10779ns | 10577ns | 11245ns | -3.10% | 3.040 |
| satfold-lanes4-idx | 7984ns | 7736ns | 8527ns | -28.22% | 4.104 |
| satfold-lanes64 | 10903ns | 10525ns | 11483ns | -1.98% | 3.005 |
| satfold-neon | 1662ns | 1641ns | 1704ns | -85.06% | 19.721 |
| satfold-neon8 | 1744ns | 1657ns | 1990ns | -84.32% | 18.789 |
| satfold-nolaw | 11159ns | 10697ns | 12316ns | +0.32% | 2.936 |
| satfold-seq | 10668ns | 10516ns | 10806ns | -4.09% | 3.072 |

## Performance model

- Peak throughput: **19.974 Gops/s** (satfold-neon; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 3.051 | 15.3% |
| satfold-lanes16 | 3.038 | 15.2% |
| satfold-lanes16-constl | 3.073 | 15.4% |
| satfold-lanes4-idx | 4.179 | 20.9% |
| satfold-lanes64 | 3.044 | 15.2% |
| satfold-neon | 19.845 | 99.4% |
| satfold-neon8 | 19.659 | 98.4% |
| satfold-nolaw | 3.053 | 15.3% |
| satfold-seq | 3.063 | 15.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 11191ns | 11191ns | base |
| satfold-lanes16 | 11007ns | 11007ns | -1.65% |
| satfold-lanes16-constl | 10843ns | 10843ns | -3.11% |
| satfold-lanes4-idx | 8052ns | 8052ns | -28.05% |
| satfold-lanes64 | 10966ns | 10966ns | -2.01% |
| satfold-neon | 1725ns | 1725ns | -84.59% |
| satfold-neon8 | 1810ns | 1810ns | -83.83% |
| satfold-nolaw | 11224ns | 11224ns | +0.29% |
| satfold-seq | 10728ns | 10728ns | -4.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 10740ns | base | --- | [10722, 11081] | --- | --- | --- | --- |
| satfold-lanes16 | 10788ns | +175.1ns (+1.6%) | [+45, +245]ns | [10762, 11133] | YES | 0.0221 | 0.0166 | 0 |
| satfold-lanes16-constl | 10662ns | -90.0ns (-0.8%) | [-282, -64]ns | [10657, 10709] | YES | 0.0221 | 0.0166 | 0 |
| satfold-lanes4-idx | 7842ns | -2897.3ns (-27.0%) | [-3044, -2728]ns | [7824, 7943] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 10766ns | no significant difference | [-57, +117]ns | [10713, 10962] | no | 0.4910 | 0.4296 | 0 |
| satfold-neon | 1651ns | -9091.6ns (-84.7%) | [-9440, -9072]ns | [1649, 1655] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 1667ns | -9067.5ns (-84.4%) | [-9423, -9042]ns | [1661, 1704] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 10732ns | no significant difference | [-7, +156]ns | [10716, 10926] | no | 0.6358 | 0.6358 | 0 |
| satfold-seq | 10699ns | -30.0ns (-0.3%) | [-360, -12]ns | [10654, 10718] | YES | 0.0021 | 0.0011 | 1 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 10516ns | +2.3% | +1.3% | -25.5% | -0.0% | -84.3% | -84.2% | +1.9% | +0.0% |
| 2 | 10548ns | +2.0% | +1.0% | -25.8% | -0.3% | -84.4% | -84.3% | +1.6% | -0.3% |
| 3 | 10528ns | +2.2% | +1.3% | -25.7% | -0.1% | -84.3% | -84.2% | +1.8% | -0.1% |
| 4 | 10517ns | +2.3% | +1.2% | -25.6% | +1.6% | -84.3% | -84.1% | +1.9% | +0.0% |
| 5 | 10525ns | +2.2% | +1.0% | -25.6% | +1.5% | -84.3% | -84.2% | +1.8% | +1.5% |
| 6 | 10569ns | +2.3% | +0.9% | -26.0% | +2.6% | -84.4% | -84.2% | +1.9% | -0.0% |
| 7 | 10545ns | +2.0% | +1.1% | -25.8% | +3.5% | -84.4% | -84.2% | +1.5% | -0.3% |
| 8 | 10577ns | +2.2% | +1.2% | -25.7% | +1.0% | -84.5% | -84.2% | +1.9% | +0.9% |
| 9 | 12052ns | -10.7% | -11.5% | -35.2% | -1.3% | -86.3% | -86.2% | -11.1% | -12.7% |
| 10 | 11551ns | -6.8% | -7.7% | -32.4% | -6.5% | -85.7% | -85.6% | -7.3% | -9.0% |
| 11 | 12668ns | -16.1% | -5.9% | -37.7% | -5.4% | -86.5% | -83.6% | +1.2% | -17.0% |
| 12 | 11936ns | -11.5% | -0.6% | -35.1% | -0.1% | -85.7% | -83.4% | +6.9% | -8.8% |
| 13 | 11934ns | -8.0% | -6.7% | -35.7% | -9.6% | -85.6% | -83.4% | +4.3% | -10.5% |
| 14 | 11975ns | -11.6% | -8.4% | -35.9% | -11.2% | -85.8% | -83.5% | +0.3% | -11.3% |
| 15 | 12018ns | -11.3% | -12.9% | -36.1% | -12.0% | -85.9% | -83.6% | -0.0% | -12.1% |
| 16 | 11967ns | -11.4% | -12.4% | -35.9% | -10.5% | -85.8% | -83.4% | +0.6% | -12.0% |
| 17 | 11925ns | -11.4% | -11.9% | -29.5% | -11.6% | -85.8% | -83.4% | +0.0% | -11.8% |
| 18 | 12371ns | -14.2% | -10.9% | -24.2% | -15.0% | -86.3% | -84.1% | -3.4% | -14.0% |
| 19 | 12118ns | -12.9% | -9.2% | -24.8% | -13.2% | -85.9% | -84.7% | -1.6% | -11.1% |
| 20 | 12035ns | +0.5% | -8.5% | -25.6% | -12.6% | -85.9% | -84.7% | +3.9% | -8.1% |
| 21 | 10684ns | +4.4% | +3.2% | -24.6% | +2.2% | -84.5% | -84.5% | +0.3% | +0.2% |
| 22 | 10722ns | +4.1% | +1.1% | -24.8% | +0.5% | -84.6% | -84.2% | -0.1% | +0.0% |
| 23 | 10748ns | +4.0% | -1.1% | -25.9% | +3.4% | -84.6% | -84.1% | -0.1% | -0.3% |
| 24 | 10720ns | +4.1% | -0.8% | -24.5% | +6.0% | -84.5% | -84.1% | -0.1% | +0.0% |
| 25 | 10726ns | +4.3% | -0.6% | -25.3% | +3.7% | -84.6% | -84.5% | -0.2% | -0.3% |
| 26 | 10722ns | +4.2% | -0.7% | -24.6% | +4.5% | -84.6% | -84.1% | +0.3% | -0.1% |
| 27 | 10725ns | +3.0% | -0.8% | -24.4% | +4.0% | -84.7% | -84.4% | -0.3% | +0.2% |
| 28 | 10757ns | +3.8% | -0.9% | -24.9% | +3.4% | -84.7% | -84.1% | -0.6% | -0.2% |
| 29 | 10733ns | +3.8% | -0.8% | -26.0% | +3.6% | -84.6% | -84.5% | -0.4% | -0.2% |
| 30 | 10710ns | +3.8% | -0.5% | -25.9% | +3.9% | -84.6% | -84.5% | -0.3% | -0.1% |
| 31 | 10717ns | +0.3% | +0.4% | -27.0% | +0.0% | -84.6% | -84.5% | -0.0% | -0.2% |
| 32 | 10720ns | +0.5% | -0.8% | -25.5% | -0.0% | -84.7% | -84.5% | -0.0% | +0.0% |
| 33 | 10724ns | +0.4% | -0.6% | -27.1% | -0.1% | -84.6% | -84.6% | -0.1% | -0.0% |
| 34 | 10816ns | -0.5% | -1.4% | -27.7% | -0.8% | -84.8% | -84.7% | -0.9% | -0.9% |
| 35 | 10728ns | +1.1% | +0.5% | -27.1% | -0.2% | -84.7% | -83.7% | +0.3% | +0.0% |
| 36 | 10791ns | +6.2% | -1.2% | -27.5% | -0.3% | -84.8% | -84.6% | +1.9% | -0.3% |
| 37 | 11085ns | +6.3% | -2.5% | -29.4% | -3.1% | -85.3% | -85.0% | +0.6% | -3.5% |
| 38 | 11084ns | +0.6% | -3.3% | -28.9% | -0.7% | -85.3% | -85.0% | +4.8% | -3.0% |
| 39 | 11078ns | +1.3% | -2.6% | -28.3% | +1.2% | -85.1% | -85.1% | +2.7% | -3.4% |
| 40 | 11072ns | +0.7% | -3.8% | -29.1% | -0.1% | -85.1% | -85.0% | -2.0% | -3.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.790 | HIGH+ (drift/warm-up) |
| satfold-lanes16 | 0.449 | moderate+ |
| satfold-lanes16-constl | 0.588 | HIGH+ (drift/warm-up) |
| satfold-lanes4-idx | 0.754 | HIGH+ (drift/warm-up) |
| satfold-lanes64 | 0.420 | moderate+ |
| satfold-neon | 0.830 | HIGH+ (drift/warm-up) |
| satfold-neon8 | 0.796 | HIGH+ (drift/warm-up) |
| satfold-nolaw | 0.744 | HIGH+ (drift/warm-up) |
| satfold-seq | 0.459 | moderate+ |

**Consistency summary:**

- **satfold-lanes16**: won 12/40, lost 28/40
- **satfold-lanes16-constl**: won 28/40, lost 12/40
- **satfold-lanes4-idx**: won 40/40, lost 0/40
- **satfold-lanes64**: won 19/40, lost 16/40
- **satfold-neon**: won 40/40, lost 0/40
- **satfold-neon8**: won 40/40, lost 0/40
- **satfold-nolaw**: won 12/40, lost 21/40
- **satfold-seq**: won 27/40, lost 4/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 2.5ns | 11123.4ns | 0.0% |  |
| satfold-lanes16 | 3.1ns | 10943.1ns | 0.0% |  |
| satfold-lanes16-constl | 2.3ns | 10778.8ns | 0.0% |  |
| satfold-lanes4-idx | 2.9ns | 7984.4ns | 0.0% |  |
| satfold-lanes64 | 3.1ns | 10903.2ns | 0.0% |  |
| satfold-neon | 2.0ns | 1661.6ns | 0.1% |  |
| satfold-neon8 | 2.2ns | 1744.0ns | 0.1% |  |
| satfold-nolaw | 2.3ns | 11159.0ns | 0.0% |  |
| satfold-seq | 2.2ns | 10668.0ns | 0.0% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 10540.6-12150.4 ns)
  10540.6 |############
  10621.1 |###
  10701.6 |########################################
  10782.0 |######
  10862.5 |
  10943.0 |
  11023.5 |############
  11104.0 |
  11184.5 |
  11265.0 |
  11345.5 |
  11426.0 |
  11506.4 |###
  11586.9 |
  11667.4 |
  11747.9 |
  11828.4 |
  11908.9 |###############
  11989.4 |#########
  12069.9 |###
  (4 below, 2 above range)

satfold-lanes16 (n=40, range 10597.7-11407.3 ns)
  10597.7 |#############
  10638.2 |####
  10678.6 |
  10719.1 |#############
  10759.6 |########################################
  10800.1 |########
  10840.6 |####
  10881.1 |
  10921.5 |
  10962.0 |####
  11002.5 |
  11043.0 |####
  11083.5 |####
  11123.9 |######################
  11164.4 |######################
  11204.9 |####
  11245.4 |
  11285.9 |
  11326.4 |
  11366.8 |
  (4 below, 3 above range)

satfold-lanes16-constl (n=40, range 10577.0-11245.1 ns)
  10577.0 |
  10610.4 |###############
  10643.8 |########################################
  10677.2 |##
  10710.6 |##
  10744.0 |##
  10777.4 |#####
  10810.8 |##
  10844.2 |##
  10877.6 |
  10911.0 |
  10944.4 |##
  10977.8 |##
  11011.2 |#######
  11044.6 |
  11078.0 |
  11111.4 |##
  11144.8 |
  11178.2 |
  11211.6 |
  (3 below, 2 above range)

satfold-lanes4-idx (n=40, range 7736.4-8527.2 ns)
   7736.4 |##
   7775.9 |#####
   7815.5 |########################################
   7855.0 |########
   7894.6 |##
   7934.1 |########
   7973.6 |#####
   8013.2 |
   8052.7 |###########
   8092.3 |#####
   8131.8 |
   8171.3 |
   8210.9 |
   8250.4 |
   8290.0 |
   8329.5 |
   8369.1 |
   8408.6 |##
   8448.1 |
   8487.7 |
  (4 below, 3 above range)

satfold-lanes64 (n=40, range 10524.8-11483.1 ns)
  10524.8 |#############
  10572.8 |
  10620.7 |######
  10668.6 |########################################
  10716.5 |#################################
  10764.4 |####################
  10812.3 |######
  10860.2 |
  10908.1 |#############
  10956.1 |
  11004.0 |######
  11051.9 |######
  11099.8 |#################################
  11147.7 |######
  11195.6 |#############
  11243.5 |
  11291.5 |
  11339.4 |######
  11387.3 |
  11435.2 |
  (6 below, 3 above range)

satfold-neon (n=40, range 1640.5-1703.8 ns)
   1640.5 |###########
   1643.7 |##################################
   1646.8 |########################################
   1650.0 |##################################
   1653.2 |############################
   1656.3 |
   1659.5 |
   1662.7 |#####
   1665.8 |
   1669.0 |
   1672.1 |
   1675.3 |
   1678.5 |
   1681.6 |
   1684.8 |
   1688.0 |#####
   1691.1 |#####
   1694.3 |###########
   1697.4 |###########
   1700.6 |
  (3 below, 4 above range)

satfold-neon8 (n=40, range 1656.7-1989.8 ns)
   1656.7 |########################################
   1673.4 |
   1690.0 |######
   1706.7 |####
   1723.3 |
   1740.0 |##
   1756.6 |
   1773.3 |
   1790.0 |
   1806.6 |
   1823.3 |
   1839.9 |####
   1856.6 |
   1873.2 |
   1889.9 |
   1906.5 |
   1923.2 |
   1939.9 |
   1956.5 |####
   1973.2 |##########
  (4 below, 1 above range)

satfold-nolaw (n=40, range 10696.8-12316.1 ns)
  10696.8 |########################################
  10777.7 |###
  10858.7 |
  10939.7 |#
  11020.6 |
  11101.6 |#
  11182.6 |
  11263.6 |
  11344.5 |#
  11425.5 |
  11506.5 |
  11587.4 |#
  11668.4 |
  11749.4 |
  11830.3 |
  11911.3 |#####
  11992.3 |#####
  12073.2 |
  12154.2 |
  12235.2 |
  (3 below, 4 above range)

satfold-seq (n=40, range 10516.1-10805.8 ns)
  10516.1 |#########################
  10530.6 |#####
  10545.1 |#####
  10559.6 |#####
  10574.1 |
  10588.5 |
  10603.0 |
  10617.5 |#####
  10632.0 |#####
  10646.5 |
  10660.9 |#####
  10675.4 |##########
  10689.9 |####################
  10704.4 |#########################
  10718.9 |########################################
  10733.4 |
  10747.8 |##########
  10762.3 |##########
  10776.8 |
  10791.3 |
  (4 below, 2 above range)

```

## Diagnostics

- **satfold-iterfold**: autocorrelation=0.79 (measurement drift or warm-up artifact)
- **satfold-lanes16-constl**: autocorrelation=0.59 (measurement drift or warm-up artifact)
- **satfold-lanes4-idx**: autocorrelation=0.75 (measurement drift or warm-up artifact)
- **satfold-neon**: autocorrelation=0.83 (measurement drift or warm-up artifact)
- **satfold-neon8**: autocorrelation=0.80 (measurement drift or warm-up artifact)
- **satfold-nolaw**: autocorrelation=0.74 (measurement drift or warm-up artifact)
