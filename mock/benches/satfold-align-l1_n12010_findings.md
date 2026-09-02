# The same arms with the column start offset by one byte from a 64-byte boundary: what the licensed vector arms cost when the load stream is not aligned

8 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-neon dominates: 79% faster than the next best (satfold-lanes64)

satfold-neon (364 ns) leads satfold-lanes64 (653 ns) by 79%, a clear separation rather than a photo finish. CV 2.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### satfold-neon beats baseline by 99% (significant)

satfold-neon is -41.20 us (99%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-seq is an outlier: 115.2x slower than the field

satfold-seq (41.98 us) is 115.2x the fastest (364 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-lanes16 shows warm-up / thermal drift (autocorr +0.90)

satfold-lanes16's per-pass series has lag-1 autocorrelation +0.90, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-neon, satfold-lanes64, satfold-lanes16-constl, satfold-lanes16} vs {satfold-lanes4-idx, satfold-nolaw, satfold-iterfold, satfold-seq} (875% apart)

The field splits into a fast tier {satfold-neon, satfold-lanes64, satfold-lanes16-constl, satfold-lanes16} and a slow tier {satfold-lanes4-idx, satfold-nolaw, satfold-iterfold, satfold-seq} with a 875% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 115.2x the fastest

Fastest satfold-neon (364 ns) to slowest satfold-seq (41.98 us): 115.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: satfold-neon** at 364.4 ns median (-99.1% vs baseline)
- 6 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 115.21x (fastest 364.4 ns, slowest 41983.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 43263ns | 41690ns | 41587ns | 41720ns | 49569ns | base |
| satfold-lanes16 | 1691ns | 1671ns | 1645ns | 1680ns | 1772ns | -96.09% |
| satfold-lanes16-constl | 1674ns | 1670ns | 1644ns | 1670ns | 1714ns | -96.13% |
| satfold-lanes4-idx | 15369ns | 15242ns | 15087ns | 15253ns | 15999ns | -64.48% |
| satfold-lanes64 | 719ns | 714ns | 705ns | 714ns | 751ns | -98.34% |
| satfold-neon | 428ns | 425ns | 418ns | 427ns | 442ns | -99.01% |
| satfold-nolaw | 32253ns | 32136ns | 31881ns | 32202ns | 32778ns | -25.45% |
| satfold-seq | 42406ns | 42083ns | 41626ns | 42212ns | 43769ns | -1.98% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 43139ns | 41486ns | 49390ns | base | 0.760 |
| satfold-lanes16 | 1568ns | 1525ns | 1642ns | -96.37% | 20.904 |
| satfold-lanes16-constl | 1552ns | 1524ns | 1591ns | -96.40% | 21.111 |
| satfold-lanes4-idx | 15236ns | 14957ns | 15860ns | -64.68% | 2.151 |
| satfold-lanes64 | 658ns | 646ns | 685ns | -98.48% | 49.826 |
| satfold-neon | 367ns | 359ns | 378ns | -99.15% | 89.383 |
| satfold-nolaw | 32161ns | 31811ns | 32675ns | -25.45% | 1.019 |
| satfold-seq | 42300ns | 41529ns | 43634ns | -1.94% | 0.775 |

## Performance model

- Peak throughput: **91.339 Gops/s** (satfold-neon; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 0.788 | 0.9% |
| satfold-lanes16 | 21.154 | 23.2% |
| satfold-lanes16-constl | 21.164 | 23.2% |
| satfold-lanes4-idx | 2.169 | 2.4% |
| satfold-lanes64 | 50.169 | 54.9% |
| satfold-neon | 89.923 | 98.4% |
| satfold-nolaw | 1.022 | 1.1% |
| satfold-seq | 0.780 | 0.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 43263ns | 43263ns | base |
| satfold-lanes16 | 1691ns | 1691ns | -96.09% |
| satfold-lanes16-constl | 1674ns | 1674ns | -96.13% |
| satfold-lanes4-idx | 15369ns | 15369ns | -64.48% |
| satfold-lanes64 | 719ns | 719ns | -98.34% |
| satfold-neon | 428ns | 428ns | -99.01% |
| satfold-nolaw | 32253ns | 32253ns | -25.45% |
| satfold-seq | 42406ns | 42406ns | -1.98% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 41584ns | base | --- | [41561, 41618] | --- | --- | --- | --- |
| satfold-lanes16 | 1549ns | -40014.0ns (-96.2%) | [-40068, -39956]ns | [1547, 1550] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 1548ns | -40034.2ns (-96.3%) | [-40082, -40010]ns | [1547, 1550] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 15110ns | -26482.3ns (-63.7%) | [-26587, -26393]ns | [15018, 15208] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 653ns | -40935.0ns (-98.4%) | [-40971, -40899]ns | [649, 657] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 364ns | -41221.9ns (-99.1%) | [-41260, -41195]ns | [361, 365] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 32072ns | -9512.9ns (-22.9%) | [-9736, -9323]ns | [31944, 32300] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 41984ns | +174.6ns (+0.4%) | [+27, +461]ns | [41688, 42487] | YES | 0.0022 | 0.0022 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|
| 1 | 41565ns | -96.1% | -96.3% | -64.0% | -98.1% | -99.1% | -23.5% | +4.0% |
| 2 | 41535ns | -96.0% | -96.3% | -64.0% | -98.4% | -99.1% | -22.2% | +4.1% |
| 3 | 41558ns | -96.0% | -96.3% | -63.6% | -98.4% | -99.1% | -22.0% | +2.8% |
| 4 | 41663ns | -96.0% | -96.3% | -64.0% | -98.4% | -99.1% | -22.3% | +3.5% |
| 5 | 41802ns | -96.1% | -96.3% | -63.9% | -98.4% | -99.1% | -24.0% | +5.2% |
| 6 | 41470ns | -96.1% | -96.2% | -63.9% | -98.4% | -99.1% | -22.5% | +4.3% |
| 7 | 41565ns | -96.1% | -96.3% | -63.6% | -98.4% | -99.1% | -20.9% | +2.8% |
| 8 | 41556ns | -96.1% | -96.3% | -63.7% | -98.4% | -99.1% | -21.9% | +3.3% |
| 9 | 41605ns | -96.0% | -96.2% | -63.7% | -98.4% | -99.1% | -22.9% | +3.5% |
| 10 | 41594ns | -96.0% | -96.2% | -63.5% | -98.4% | -99.1% | -20.8% | +7.9% |
| 11 | 41688ns | -96.2% | -96.3% | -63.8% | -98.4% | -99.1% | -22.3% | -0.1% |
| 12 | 41594ns | -96.2% | -96.3% | -63.4% | -98.4% | -99.1% | -21.4% | +2.2% |
| 13 | 41613ns | -96.3% | -96.3% | -61.7% | -98.4% | -99.1% | -23.6% | +0.1% |
| 14 | 41480ns | -96.3% | -96.3% | -62.0% | -98.4% | -99.1% | -22.9% | +0.2% |
| 15 | 41486ns | -96.3% | -96.1% | -63.3% | -98.4% | -99.1% | -21.4% | +0.0% |
| 16 | 41568ns | -96.3% | -96.3% | -63.5% | -98.4% | -99.1% | -22.3% | +0.4% |
| 17 | 41576ns | -96.3% | -96.3% | -64.0% | -98.4% | -99.1% | -23.4% | -0.1% |
| 18 | 41488ns | -96.3% | -96.3% | -63.9% | -98.4% | -99.1% | -22.9% | +0.3% |
| 19 | 41511ns | -96.3% | -96.3% | -64.0% | -98.4% | -99.1% | -22.0% | +2.0% |
| 20 | 41537ns | -96.3% | -96.3% | -63.6% | -98.4% | -99.1% | -23.5% | +0.1% |
| 21 | 41563ns | -96.3% | -96.3% | -59.2% | -98.4% | -99.1% | -22.8% | +0.0% |
| 22 | 41496ns | -96.3% | -96.3% | -61.6% | -98.4% | -99.1% | -22.8% | +1.3% |
| 23 | 41515ns | -96.3% | -96.3% | -62.0% | -98.4% | -99.1% | -23.1% | +2.3% |
| 24 | 41579ns | -96.3% | -96.3% | -62.4% | -98.5% | -99.1% | -23.4% | +0.7% |
| 25 | 41623ns | -96.3% | -96.3% | -63.3% | -98.4% | -99.1% | -23.3% | +0.8% |
| 26 | 41726ns | -96.3% | -96.3% | -63.0% | -98.5% | -99.1% | -23.8% | -0.4% |
| 27 | 41589ns | -96.3% | -96.3% | -63.4% | -98.4% | -99.1% | -23.4% | -0.2% |
| 28 | 41485ns | -96.3% | -96.2% | -63.5% | -98.4% | -99.1% | -22.0% | +0.3% |
| 29 | 41469ns | -96.3% | -96.2% | -63.9% | -98.4% | -99.1% | -22.9% | +0.0% |
| 30 | 41520ns | -96.3% | -96.2% | -63.8% | -98.4% | -99.1% | -23.3% | +0.0% |
| 31 | 42133ns | -96.4% | -96.3% | -64.5% | -98.4% | -99.1% | -22.0% | +1.0% |
| 32 | 41825ns | -96.4% | -96.3% | -64.1% | -98.4% | -99.1% | -22.1% | +1.0% |
| 33 | 51556ns | -97.0% | -97.0% | -70.9% | -98.7% | -99.3% | -37.5% | -18.3% |
| 34 | 63796ns | -97.6% | -97.6% | -76.5% | -99.0% | -99.4% | -49.8% | -33.9% |
| 35 | 47574ns | -96.8% | -96.4% | -68.6% | -98.6% | -99.2% | -33.1% | -8.7% |
| 36 | 43670ns | -96.5% | -96.5% | -65.7% | -98.5% | -99.1% | -26.1% | +0.6% |
| 37 | 51605ns | -97.0% | -97.0% | -71.0% | -98.7% | -99.3% | -37.3% | -18.7% |
| 38 | 52056ns | -97.0% | -97.0% | -70.5% | -98.7% | -99.3% | -38.5% | -20.2% |
| 39 | 41600ns | -96.3% | -96.3% | -63.0% | -98.4% | -99.1% | -23.4% | +0.5% |
| 40 | 42730ns | -96.4% | -96.4% | -64.1% | -98.5% | -99.1% | -25.4% | -2.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.532 | HIGH+ (drift/warm-up) |
| satfold-lanes16 | 0.899 | HIGH+ (drift/warm-up) |
| satfold-lanes16-constl | 0.022 | ok |
| satfold-lanes4-idx | 0.498 | moderate+ |
| satfold-lanes64 | 0.080 | ok |
| satfold-neon | 0.753 | HIGH+ (drift/warm-up) |
| satfold-nolaw | 0.167 | ok |
| satfold-seq | 0.496 | moderate+ |

**Consistency summary:**

- **satfold-lanes16**: won 40/40, lost 0/40
- **satfold-lanes16-constl**: won 40/40, lost 0/40
- **satfold-lanes4-idx**: won 40/40, lost 0/40
- **satfold-lanes64**: won 40/40, lost 0/40
- **satfold-neon**: won 40/40, lost 0/40
- **satfold-nolaw**: won 40/40, lost 0/40
- **satfold-seq**: won 8/40, lost 25/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 3.2ns | 43139.1ns | 0.0% |  |
| satfold-lanes16 | 2.8ns | 1567.5ns | 0.2% |  |
| satfold-lanes16-constl | 2.7ns | 1552.1ns | 0.2% |  |
| satfold-lanes4-idx | 2.9ns | 15236.2ns | 0.0% |  |
| satfold-lanes64 | 2.6ns | 657.6ns | 0.4% |  |
| satfold-neon | 2.2ns | 366.6ns | 0.6% |  |
| satfold-nolaw | 2.2ns | 32161.2ns | 0.0% |  |
| satfold-seq | 3.2ns | 42300.1ns | 0.0% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 41485.7-49390.1 ns)
  41485.7 |########################################
  41880.9 |#
  42276.2 |
  42671.4 |#
  43066.6 |
  43461.8 |#
  43857.0 |
  44252.3 |
  44647.5 |
  45042.7 |
  45437.9 |
  45833.1 |
  46228.4 |
  46623.6 |
  47018.8 |
  47414.0 |#
  47809.3 |
  48204.5 |
  48599.7 |
  48994.9 |
  (4 below, 4 above range)

satfold-lanes16 (n=40, range 1525.2-1642.0 ns)
   1525.2 |########
   1531.0 |
   1536.8 |########
   1542.7 |########################################
   1548.5 |###################################
   1554.4 |####
   1560.2 |
   1566.0 |
   1571.9 |
   1577.7 |####
   1583.6 |####
   1589.4 |
   1595.2 |
   1601.1 |
   1606.9 |
   1612.8 |####
   1618.6 |
   1624.5 |#############
   1630.3 |
   1636.1 |####
  (6 below, 5 above range)

satfold-lanes16-constl (n=40, range 1524.3-1591.4 ns)
   1524.3 |#########
   1527.6 |
   1531.0 |
   1534.3 |
   1537.7 |
   1541.0 |###
   1544.4 |########################
   1547.8 |########################################
   1551.1 |###
   1554.5 |#########
   1557.8 |
   1561.2 |
   1564.5 |###
   1567.9 |###
   1571.2 |
   1574.6 |###
   1577.9 |
   1581.3 |
   1584.7 |
   1588.0 |
  (5 below, 3 above range)

satfold-lanes4-idx (n=40, range 14956.8-15859.9 ns)
  14956.8 |########################################
  15002.0 |####################
  15047.1 |##########
  15092.3 |#########################
  15137.4 |#####
  15182.6 |###############
  15227.8 |##########
  15272.9 |#####
  15318.1 |##########
  15363.2 |#####
  15408.4 |
  15453.5 |#####
  15498.7 |
  15543.8 |
  15589.0 |#####
  15634.2 |
  15679.3 |
  15724.5 |#####
  15769.6 |#####
  15814.8 |
  (4 below, 3 above range)

satfold-lanes64 (n=40, range 645.7-684.6 ns)
    645.7 |########################################
    647.6 |########
    649.6 |################
    651.5 |####
    653.5 |################
    655.4 |################
    657.4 |####################
    659.3 |############
    661.3 |
    663.2 |
    665.2 |
    667.1 |
    669.1 |
    671.0 |
    673.0 |
    674.9 |####
    676.9 |
    678.8 |
    680.8 |
    682.7 |####
  (3 below, 2 above range)

satfold-neon (n=40, range 358.8-378.3 ns)
    358.8 |####################
    359.7 |######
    360.7 |########################################
    361.7 |######
    362.7 |#############
    363.6 |#################################
    364.6 |#################################
    365.6 |
    366.6 |######
    367.5 |
    368.5 |
    369.5 |
    370.5 |
    371.4 |######
    372.4 |
    373.4 |
    374.4 |
    375.4 |#############
    376.3 |######
    377.3 |##########################
  (4 below, 4 above range)

satfold-nolaw (n=40, range 31811.1-32675.2 ns)
  31811.1 |########################################
  31854.3 |########################
  31897.5 |################
  31940.7 |################
  31983.9 |########################
  32027.1 |########
  32070.3 |################
  32113.5 |########
  32156.7 |
  32199.9 |########
  32243.2 |################
  32286.4 |########
  32329.6 |################
  32372.8 |########################
  32416.0 |################
  32459.2 |
  32502.4 |
  32545.6 |########
  32588.8 |########
  32632.0 |
  (4 below, 4 above range)

satfold-seq (n=40, range 41528.9-43633.7 ns)
  41528.9 |########################################
  41634.1 |#############
  41739.3 |####
  41844.6 |#############
  41949.8 |########
  42055.1 |####
  42160.3 |########
  42265.6 |####
  42370.8 |####
  42476.1 |########
  42581.3 |
  42686.5 |########
  42791.8 |
  42897.0 |####
  43002.3 |########
  43107.5 |
  43212.8 |#############
  43318.0 |
  43423.2 |####
  43528.5 |
  (3 below, 3 above range)

```

## Diagnostics

- **satfold-iterfold**: autocorrelation=0.53 (measurement drift or warm-up artifact)
- **satfold-lanes16**: autocorrelation=0.90 (measurement drift or warm-up artifact)
- **satfold-neon**: autocorrelation=0.75 (measurement drift or warm-up artifact)
