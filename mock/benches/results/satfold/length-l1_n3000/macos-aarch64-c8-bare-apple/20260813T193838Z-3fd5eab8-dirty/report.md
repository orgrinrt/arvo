# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-neon8 beats baseline by 87% (significant)

satfold-neon8 is -9.16 us (87%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-lanes16 is an outlier: 6.8x slower than the field

satfold-lanes16 (10.50 us) is 6.8x the fastest (1.53 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (satfold-neon8, satfold-neon) are a dead heat (<1%)

satfold-neon8 (1.53 us) and satfold-neon (1.54 us) differ by 0.25%, inside the noise, even though the wider field spreads 584.8%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### satfold-iterfold shows warm-up / thermal drift (autocorr +0.70)

satfold-iterfold's per-pass series has lag-1 autocorrelation +0.70, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-neon8, satfold-neon} vs {satfold-lanes4-idx, satfold-seq, satfold-nolaw, satfold-lanes16-constl, satfold-lanes64, satfold-iterfold, satfold-lanes16} (412% apart)

The field splits into a fast tier {satfold-neon8, satfold-neon} and a slow tier {satfold-lanes4-idx, satfold-seq, satfold-nolaw, satfold-lanes16-constl, satfold-lanes64, satfold-iterfold, satfold-lanes16} with a 412% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.8x the fastest

Fastest satfold-neon8 (1.53 us) to slowest satfold-lanes16 (10.50 us): 6.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-lanes64's edge over baseline is significant but tiny (-24 ns, 0.23%)

satfold-lanes64 differs from baseline satfold-iterfold by -24 ns (0.23%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: satfold-neon8** at 1533.3 ns median (-85.4% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 6.85x (fastest 1533.3 ns, slowest 10499.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 10760ns | 10540ns | 10439ns | 10616ns | 11512ns | base |
| satfold-lanes16 | 10658ns | 10557ns | 10400ns | 10617ns | 11039ns | -0.95% |
| satfold-lanes16-constl | 10551ns | 10495ns | 10272ns | 10479ns | 11045ns | -1.94% |
| satfold-lanes4-idx | 8034ns | 7936ns | 7803ns | 7943ns | 8539ns | -25.33% |
| satfold-lanes64 | 10634ns | 10538ns | 10353ns | 10591ns | 11044ns | -1.17% |
| satfold-neon | 1617ns | 1598ns | 1568ns | 1601ns | 1714ns | -84.97% |
| satfold-neon8 | 1596ns | 1596ns | 1567ns | 1596ns | 1622ns | -85.17% |
| satfold-nolaw | 10679ns | 10486ns | 10440ns | 10507ns | 11433ns | -0.75% |
| satfold-seq | 10495ns | 10470ns | 10313ns | 10482ns | 10715ns | -2.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 10690ns | 10381ns | 11419ns | base | 3.065 |
| satfold-lanes16 | 10593ns | 10335ns | 10969ns | -0.90% | 3.093 |
| satfold-lanes16-constl | 10484ns | 10216ns | 10956ns | -1.93% | 3.126 |
| satfold-lanes4-idx | 7965ns | 7736ns | 8463ns | -25.49% | 4.114 |
| satfold-lanes64 | 10569ns | 10292ns | 10973ns | -1.13% | 3.100 |
| satfold-neon | 1555ns | 1507ns | 1648ns | -85.46% | 21.075 |
| satfold-neon8 | 1530ns | 1506ns | 1549ns | -85.68% | 21.412 |
| satfold-nolaw | 10609ns | 10376ns | 11349ns | -0.75% | 3.089 |
| satfold-seq | 10433ns | 10252ns | 10649ns | -2.40% | 3.141 |

## Performance model

- Peak throughput: **21.761 Gops/s** (satfold-neon8; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 3.129 | 14.4% |
| satfold-lanes16 | 3.121 | 14.3% |
| satfold-lanes16-constl | 3.140 | 14.4% |
| satfold-lanes4-idx | 4.164 | 19.1% |
| satfold-lanes64 | 3.130 | 14.4% |
| satfold-neon | 21.318 | 98.0% |
| satfold-neon8 | 21.371 | 98.2% |
| satfold-nolaw | 3.144 | 14.4% |
| satfold-seq | 3.147 | 14.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 10760ns | 10760ns | base |
| satfold-lanes16 | 10658ns | 10658ns | -0.95% |
| satfold-lanes16-constl | 10551ns | 10551ns | -1.94% |
| satfold-lanes4-idx | 8034ns | 8034ns | -25.33% |
| satfold-lanes64 | 10634ns | 10634ns | -1.17% |
| satfold-neon | 1617ns | 1617ns | -84.97% |
| satfold-neon8 | 1596ns | 1596ns | -85.17% |
| satfold-nolaw | 10679ns | 10679ns | -0.75% |
| satfold-seq | 10495ns | 10495ns | -2.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 10473ns | base | --- | [10420, 10698] | --- | --- | --- | --- |
| satfold-lanes16 | 10499ns | no significant difference | [-18, +45]ns | [10432, 10719] | no | 0.8746 | 0.8746 | 0 |
| satfold-lanes16-constl | 10435ns | no significant difference | [-374, +43]ns | [10257, 10491] | no | 0.7267 | 0.6358 | 0 |
| satfold-lanes4-idx | 7869ns | -2667.1ns (-25.5%) | [-2722, -2636]ns | [7762, 7990] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 10469ns | no significant difference | [-45, +6]ns | [10384, 10695] | no | 0.5728 | 0.4296 | 0 |
| satfold-neon | 1537ns | -8944.8ns (-85.4%) | [-9151, -8891]ns | [1534, 1546] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 1533ns | -8947.2ns (-85.4%) | [-9166, -8884]ns | [1531, 1535] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 10422ns | no significant difference | [-39, +16]ns | [10395, 10456] | no | 0.5728 | 0.4296 | 0 |
| satfold-seq | 10411ns | no significant difference | [-316, +7]ns | [10399, 10440] | no | 0.3077 | 0.1539 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 11931ns | -12.9% | -14.2% | -35.1% | -10.1% | -87.0% | -87.4% | -13.0% | -13.9% |
| 2 | 11575ns | -9.2% | -11.7% | -33.2% | -7.1% | -86.4% | -87.0% | -10.2% | -11.8% |
| 3 | 11577ns | -11.1% | -11.7% | -31.4% | -8.1% | -86.7% | -86.9% | -10.2% | -10.7% |
| 4 | 11800ns | -13.2% | -13.4% | -34.5% | -13.0% | -86.9% | -87.0% | -12.0% | -12.0% |
| 5 | 11306ns | -8.1% | -9.6% | -29.6% | -7.2% | -86.3% | -86.6% | -6.4% | -7.6% |
| 6 | 10492ns | -0.5% | -2.2% | -21.8% | -0.1% | -85.2% | -85.6% | +0.6% | +6.7% |
| 7 | 10269ns | +2.8% | -0.5% | -23.0% | -0.7% | -85.1% | -85.4% | +1.2% | +2.6% |
| 8 | 10414ns | -0.1% | +1.4% | -25.4% | -1.6% | -85.3% | -85.6% | +0.1% | -0.3% |
| 9 | 10639ns | -2.8% | +6.1% | -27.1% | -4.1% | -85.6% | -85.8% | -2.4% | -3.6% |
| 10 | 10627ns | -3.6% | -1.2% | -26.3% | +1.1% | -85.6% | -85.8% | -2.2% | -4.0% |
| 11 | 10451ns | -0.2% | +3.5% | -24.4% | +1.4% | -85.6% | -85.3% | -0.0% | -1.4% |
| 12 | 10623ns | +1.8% | +1.7% | -26.9% | +0.5% | -85.8% | -85.6% | -1.9% | -1.9% |
| 13 | 10521ns | -0.5% | +5.0% | -26.5% | +0.0% | -85.6% | -85.4% | -0.2% | -0.9% |
| 14 | 10510ns | -0.7% | +0.9% | -26.5% | -1.5% | -85.6% | -85.4% | +2.4% | +0.2% |
| 15 | 10442ns | +1.4% | +0.5% | -25.9% | -1.0% | -85.6% | -85.3% | +0.1% | +2.0% |
| 16 | 10419ns | +0.5% | +0.3% | -25.7% | -0.4% | -85.5% | -85.3% | +1.5% | -0.7% |
| 17 | 10404ns | +0.8% | +0.3% | -25.6% | -0.2% | -85.5% | -85.3% | -0.3% | +0.6% |
| 18 | 10454ns | +2.8% | +0.3% | -13.0% | -0.7% | -85.6% | -85.3% | +0.1% | -1.8% |
| 19 | 10950ns | -0.7% | -4.7% | -15.5% | -4.5% | -86.2% | -86.0% | -5.3% | -5.8% |
| 20 | 11106ns | +4.9% | -6.0% | -21.0% | -6.7% | -86.4% | -86.2% | -6.3% | -8.2% |
| 21 | 10406ns | +1.0% | +0.3% | -25.5% | +3.5% | -83.5% | -85.3% | +0.2% | +0.1% |
| 22 | 10400ns | -0.2% | -1.6% | -25.2% | +4.3% | -83.6% | -85.3% | +0.3% | +1.3% |
| 23 | 10429ns | +2.2% | +0.9% | -25.6% | +4.0% | -84.0% | -85.1% | -0.1% | -0.2% |
| 24 | 10784ns | -0.0% | -5.1% | -26.6% | -0.0% | -85.2% | -85.7% | -3.6% | -3.6% |
| 25 | 10784ns | -0.1% | -5.2% | -25.5% | +0.1% | -85.2% | -85.8% | -3.8% | -2.5% |
| 26 | 11107ns | -2.4% | -7.8% | -27.6% | +0.9% | -84.4% | -86.2% | -5.4% | -4.7% |
| 27 | 10785ns | +3.0% | -5.2% | -24.4% | +3.8% | -85.3% | -85.8% | +29.2% | -3.3% |
| 28 | 10800ns | -0.1% | -5.1% | -24.9% | +3.4% | -85.3% | -85.6% | +2.7% | -2.6% |
| 29 | 10756ns | +0.1% | -5.1% | -25.3% | +0.2% | -85.7% | -85.7% | +1.1% | -3.3% |
| 30 | 10757ns | +0.3% | -5.0% | -25.3% | -0.4% | -85.4% | -85.5% | +9.8% | -3.3% |
| 31 | 10411ns | -0.3% | +3.7% | -22.8% | -0.4% | -85.2% | -85.3% | -0.3% | +1.7% |
| 32 | 10402ns | -0.1% | +5.1% | -22.3% | -0.2% | -85.2% | -85.2% | -0.2% | +0.1% |
| 33 | 10394ns | +0.1% | +7.0% | -22.7% | +0.0% | -85.2% | -85.2% | -0.1% | +0.7% |
| 34 | 10398ns | +0.0% | +4.3% | -22.8% | -0.2% | -85.3% | -85.2% | -0.2% | +0.1% |
| 35 | 10415ns | -0.1% | +2.7% | -23.6% | -0.4% | -85.2% | -85.3% | -0.4% | -0.1% |
| 36 | 10420ns | +0.2% | +0.1% | -25.6% | +5.0% | -85.3% | -85.2% | -0.1% | +0.1% |
| 37 | 10380ns | +3.7% | +1.1% | -25.2% | +0.1% | -85.2% | -85.2% | +0.5% | +0.2% |
| 38 | 10452ns | +3.1% | +0.4% | -25.6% | -0.2% | -85.3% | -85.3% | +3.1% | +0.3% |
| 39 | 10405ns | +4.7% | +0.0% | -25.5% | +0.0% | -85.2% | -85.3% | +3.5% | +1.4% |
| 40 | 10401ns | +2.8% | +0.2% | -25.4% | +0.1% | -85.2% | -84.9% | +3.5% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.701 | HIGH+ (drift/warm-up) |
| satfold-lanes16 | 0.450 | moderate+ |
| satfold-lanes16-constl | 0.598 | HIGH+ (drift/warm-up) |
| satfold-lanes4-idx | 0.605 | HIGH+ (drift/warm-up) |
| satfold-lanes64 | 0.645 | HIGH+ (drift/warm-up) |
| satfold-neon | 0.623 | HIGH+ (drift/warm-up) |
| satfold-neon8 | 0.567 | HIGH+ (drift/warm-up) |
| satfold-nolaw | 0.173 | ok |
| satfold-seq | 0.257 | moderate+ |

**Consistency summary:**

- **satfold-lanes16**: won 17/40, lost 16/40
- **satfold-lanes16-constl**: won 18/40, lost 21/40
- **satfold-lanes4-idx**: won 40/40, lost 0/40
- **satfold-lanes64**: won 21/40, lost 12/40
- **satfold-neon**: won 40/40, lost 0/40
- **satfold-neon8**: won 40/40, lost 0/40
- **satfold-nolaw**: won 22/40, lost 14/40
- **satfold-seq**: won 23/40, lost 12/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 2.8ns | 10689.9ns | 0.0% |  |
| satfold-lanes16 | 3.1ns | 10593.4ns | 0.0% |  |
| satfold-lanes16-constl | 2.7ns | 10483.9ns | 0.0% |  |
| satfold-lanes4-idx | 2.8ns | 7965.0ns | 0.0% |  |
| satfold-lanes64 | 3.2ns | 10569.4ns | 0.0% |  |
| satfold-neon | 2.2ns | 1554.8ns | 0.1% |  |
| satfold-neon8 | 2.2ns | 1530.3ns | 0.1% |  |
| satfold-nolaw | 2.7ns | 10609.4ns | 0.0% |  |
| satfold-seq | 2.0ns | 10432.9ns | 0.0% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 10381.0-11418.9 ns)
  10381.0 |########################################
  10432.9 |###########
  10484.8 |########
  10536.7 |
  10588.6 |########
  10640.5 |
  10692.4 |
  10744.3 |##############
  10796.2 |##
  10848.1 |
  10900.0 |##
  10951.9 |
  11003.8 |
  11055.7 |#####
  11107.6 |
  11159.5 |
  11211.4 |
  11263.3 |##
  11315.2 |
  11367.1 |
  (2 below, 4 above range)

satfold-lanes16 (n=40, range 10334.8-10969.2 ns)
  10334.8 |########
  10366.5 |########################################
  10398.3 |################################
  10430.0 |################################
  10461.7 |########################
  10493.4 |################
  10525.1 |########
  10556.9 |
  10588.6 |########
  10620.3 |
  10652.0 |########
  10683.7 |########
  10715.4 |########
  10747.2 |########################
  10778.9 |################################
  10810.6 |################
  10842.3 |
  10874.0 |################
  10905.8 |
  10937.5 |
  (3 below, 2 above range)

satfold-lanes16-constl (n=40, range 10215.9-10956.0 ns)
  10215.9 |########################################
  10252.9 |####
  10289.9 |
  10326.9 |
  10363.9 |
  10400.9 |########################
  10437.9 |########
  10474.9 |####################
  10511.9 |####
  10548.9 |####
  10585.9 |####
  10622.9 |
  10660.0 |####
  10697.0 |
  10734.0 |
  10771.0 |####
  10808.0 |########
  10845.0 |####
  10882.0 |
  10919.0 |####
  (4 below, 3 above range)

satfold-lanes4-idx (n=40, range 7736.2-8463.1 ns)
   7736.2 |########################################
   7772.6 |#####
   7808.9 |##
   7845.3 |
   7881.6 |#####
   7918.0 |########
   7954.3 |##
   7990.6 |##
   8027.0 |#################
   8063.3 |##
   8099.7 |##
   8136.0 |##
   8172.3 |##
   8208.7 |
   8245.0 |
   8281.4 |
   8317.7 |
   8354.0 |
   8390.4 |
   8426.7 |
  (3 below, 3 above range)

satfold-lanes64 (n=40, range 10292.1-10973.2 ns)
  10292.1 |
  10326.1 |########
  10360.2 |########################################
  10394.2 |#############
  10428.3 |########
  10462.4 |########
  10496.4 |####
  10530.5 |
  10564.5 |
  10598.6 |####
  10632.6 |####
  10666.7 |####
  10700.8 |########
  10734.8 |########
  10768.9 |#################
  10802.9 |
  10837.0 |########
  10871.1 |
  10905.1 |
  10939.2 |####
  (4 below, 3 above range)

satfold-neon (n=40, range 1507.4-1648.3 ns)
   1507.4 |###############################
   1514.4 |
   1521.5 |####
   1528.5 |##########################
   1535.6 |########################################
   1542.6 |
   1549.7 |#################
   1556.7 |
   1563.8 |####
   1570.8 |####
   1577.9 |
   1584.9 |########
   1592.0 |########
   1599.0 |
   1606.0 |
   1613.1 |
   1620.1 |
   1627.2 |
   1634.2 |
   1641.3 |
  (3 below, 4 above range)

satfold-neon8 (n=40, range 1505.8-1548.9 ns)
   1505.8 |#############
   1508.0 |
   1510.1 |####
   1512.3 |
   1514.4 |####
   1516.6 |
   1518.7 |
   1520.9 |
   1523.0 |
   1525.2 |
   1527.4 |####
   1529.5 |#################
   1531.7 |########################################
   1533.8 |###################################
   1536.0 |#################
   1538.1 |
   1540.3 |####
   1542.4 |
   1544.6 |
   1546.8 |
  (4 below, 4 above range)

satfold-nolaw (n=40, range 10375.6-11348.6 ns)
  10375.6 |########################################
  10424.2 |############
  10472.9 |####
  10521.5 |##
  10570.2 |####
  10618.8 |
  10667.5 |
  10716.1 |##
  10764.8 |######
  10813.5 |
  10862.1 |##
  10910.8 |
  10959.4 |
  11008.1 |
  11056.7 |##
  11105.4 |
  11154.0 |
  11202.7 |
  11251.3 |
  11300.0 |
  (2 below, 2 above range)

satfold-seq (n=40, range 10251.5-10649.5 ns)
  10251.5 |##########
  10271.4 |#####
  10291.3 |##########
  10311.2 |
  10331.1 |##########
  10351.0 |
  10370.9 |##########
  10390.8 |########################################
  10410.7 |#########################
  10430.6 |##########
  10450.5 |##########
  10470.4 |#####
  10490.3 |
  10510.2 |##########
  10530.1 |###############
  10550.0 |#####
  10569.9 |
  10589.8 |##########
  10609.7 |
  10629.6 |
  (3 below, 2 above range)

```

## Diagnostics

- **satfold-iterfold**: autocorrelation=0.70 (measurement drift or warm-up artifact)
- **satfold-lanes16-constl**: autocorrelation=0.60 (measurement drift or warm-up artifact)
- **satfold-lanes4-idx**: autocorrelation=0.61 (measurement drift or warm-up artifact)
- **satfold-lanes64**: autocorrelation=0.64 (measurement drift or warm-up artifact)
- **satfold-neon**: autocorrelation=0.62 (measurement drift or warm-up artifact)
- **satfold-neon8**: autocorrelation=0.57 (measurement drift or warm-up artifact)
