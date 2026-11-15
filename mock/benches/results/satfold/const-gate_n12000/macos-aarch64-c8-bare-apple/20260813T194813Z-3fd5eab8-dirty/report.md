# Does a const gate erase in time: the licensed arm reached directly, the same arm reached through a const verdict computed by an exhaustive sweep in a const fn, and the same gate over a law that is false so it selects the fallback

5 variants, 40 samples per variant.
Baseline: **satfold-gate-false**

## Highlights

Baseline for all deltas below: **satfold-gate-false**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-lanes16-3 beats baseline by 97% (significant)

satfold-lanes16-3 is -40.36 us (97%) faster than baseline satfold-gate-false, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-seq is an outlier: 27.4x slower than the field

satfold-seq (41.86 us) is 27.4x the fastest (1.53 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-gate-true shows warm-up / thermal drift (autocorr +0.82)

satfold-gate-true's per-pass series has lag-1 autocorrelation +0.82, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-lanes16-3, satfold-lanes16, satfold-gate-true} vs {satfold-gate-false, satfold-seq} (2597% apart)

The field splits into a fast tier {satfold-lanes16-3, satfold-lanes16, satfold-gate-true} and a slow tier {satfold-gate-false, satfold-seq} with a 2597% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 27.4x the fastest

Fastest satfold-lanes16-3 (1.53 us) to slowest satfold-seq (41.86 us): 27.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: satfold-lanes16-3** at 1529.6 ns median (-96.3% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 27.36x (fastest 1529.6 ns, slowest 41855.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-gate-false | 42011ns | 41844ns | 41590ns | 41928ns | 42682ns | base |
| satfold-gate-true | 1685ns | 1671ns | 1643ns | 1680ns | 1746ns | -95.99% |
| satfold-lanes16 | 1684ns | 1671ns | 1644ns | 1680ns | 1735ns | -95.99% |
| satfold-lanes16-3 | 1660ns | 1647ns | 1643ns | 1654ns | 1696ns | -96.05% |
| satfold-seq | 42279ns | 42007ns | 41621ns | 42088ns | 43511ns | +0.64% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-gate-false | 41894ns | 41497ns | 42521ns | base | 0.782 |
| satfold-gate-true | 1563ns | 1524ns | 1622ns | -96.27% | 20.965 |
| satfold-lanes16 | 1560ns | 1524ns | 1606ns | -96.28% | 21.002 |
| satfold-lanes16-3 | 1540ns | 1524ns | 1572ns | -96.32% | 21.283 |
| satfold-seq | 42151ns | 41519ns | 43340ns | +0.61% | 0.777 |

## Performance model

- Peak throughput: **21.508 Gops/s** (satfold-lanes16-3; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-gate-false | 0.785 | 3.6% |
| satfold-gate-true | 21.169 | 98.4% |
| satfold-lanes16 | 21.198 | 98.6% |
| satfold-lanes16-3 | 21.423 | 99.6% |
| satfold-seq | 0.783 | 3.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-gate-false | 42011ns | 42011ns | base |
| satfold-gate-true | 1685ns | 1685ns | -95.99% |
| satfold-lanes16 | 1684ns | 1684ns | -95.99% |
| satfold-lanes16-3 | 1660ns | 1660ns | -96.05% |
| satfold-seq | 42279ns | 42279ns | +0.64% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-gate-false | 41749ns | base | --- | [41607, 42121] | --- | --- | --- | --- |
| satfold-gate-true | 1548ns | -40211.5ns (-96.3%) | [-40513, -40038]ns | [1546, 1561] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16 | 1546ns | -40208.1ns (-96.3%) | [-40540, -40048]ns | [1535, 1581] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-3 | 1530ns | -40197.5ns (-96.3%) | [-40566, -40056]ns | [1526, 1544] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 41856ns | no significant difference | [-121, +458]ns | [41664, 42207] | no | 0.6358 | 0.6358 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | satfold-gate-false | satfold-gate-true | satfold-lanes16 | satfold-lanes16-3 | satfold-seq |
|---|---|---|---|---|---|
| 1 | 42333ns | -96.3% | -96.4% | -96.4% | -1.7% |
| 2 | 41726ns | -96.3% | -96.3% | -96.3% | +0.4% |
| 3 | 41752ns | -96.3% | -96.3% | -96.3% | +1.1% |
| 4 | 41865ns | -96.3% | -96.3% | -96.3% | +0.2% |
| 5 | 42129ns | -96.3% | -96.4% | -96.3% | +2.1% |
| 6 | 42458ns | -96.4% | -96.4% | -96.4% | -2.0% |
| 7 | 42182ns | -96.3% | -96.4% | -96.4% | -0.8% |
| 8 | 43248ns | -96.4% | -96.5% | -96.5% | -3.7% |
| 9 | 41607ns | -96.2% | -96.2% | -96.3% | -0.3% |
| 10 | 41651ns | -96.3% | -96.2% | -96.3% | -0.3% |
| 11 | 41492ns | -96.3% | -96.3% | -96.3% | +5.4% |
| 12 | 41508ns | -96.3% | -96.3% | -96.3% | +0.6% |
| 13 | 42440ns | -96.4% | -96.4% | -96.3% | -1.9% |
| 14 | 41900ns | -96.4% | -96.4% | -96.3% | +1.9% |
| 15 | 41530ns | -96.3% | -96.3% | -96.3% | +1.2% |
| 16 | 41747ns | -96.3% | -96.3% | -96.3% | -0.4% |
| 17 | 41562ns | -96.3% | -96.3% | -96.3% | -0.2% |
| 18 | 41606ns | -96.3% | -96.3% | -96.2% | -0.3% |
| 19 | 41652ns | -96.3% | -96.2% | -96.1% | +0.5% |
| 20 | 41520ns | -96.3% | -96.3% | -96.2% | +2.0% |
| 21 | 42910ns | -96.4% | -96.3% | -96.4% | -2.5% |
| 22 | 42176ns | -96.3% | -96.2% | -96.4% | -0.8% |
| 23 | 41839ns | -96.2% | -96.2% | -96.3% | -0.6% |
| 24 | 41804ns | -96.3% | -96.2% | -96.4% | -0.4% |
| 25 | 41477ns | -96.3% | -96.1% | -96.3% | +6.1% |
| 26 | 42284ns | -96.2% | -96.2% | -96.4% | +3.5% |
| 27 | 42112ns | -96.1% | -96.2% | -96.3% | +3.8% |
| 28 | 41475ns | -96.0% | -96.1% | -96.3% | +3.7% |
| 29 | 41651ns | -96.1% | -96.2% | -96.3% | +2.0% |
| 30 | 42308ns | -96.0% | -96.2% | -96.4% | -1.5% |
| 31 | 42190ns | -96.2% | -96.3% | -96.4% | +1.0% |
| 32 | 42152ns | -96.2% | -96.3% | -96.4% | -0.0% |
| 33 | 41513ns | -96.1% | -96.3% | -96.3% | +2.5% |
| 34 | 41516ns | -96.1% | -96.2% | -96.3% | +0.8% |
| 35 | 42143ns | -96.2% | -96.3% | -96.3% | -1.5% |
| 36 | 41600ns | -96.1% | -96.3% | -96.3% | -0.1% |
| 37 | 41510ns | -96.2% | -96.2% | -96.3% | +3.1% |
| 38 | 41483ns | -96.3% | -96.1% | -96.3% | +1.7% |
| 39 | 41560ns | -96.3% | -96.2% | -96.3% | +0.0% |
| 40 | 42138ns | -96.3% | -96.3% | -96.4% | +0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-gate-false | 0.130 | ok |
| satfold-gate-true | 0.822 | HIGH+ (drift/warm-up) |
| satfold-lanes16 | 0.596 | HIGH+ (drift/warm-up) |
| satfold-lanes16-3 | 0.728 | HIGH+ (drift/warm-up) |
| satfold-seq | 0.311 | moderate+ |

**Consistency summary:**

- **satfold-gate-true**: won 40/40, lost 0/40
- **satfold-lanes16**: won 40/40, lost 0/40
- **satfold-lanes16-3**: won 40/40, lost 0/40
- **satfold-seq**: won 17/40, lost 21/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-gate-false | 3.0ns | 41893.7ns | 0.0% |  |
| satfold-gate-true | 2.4ns | 1563.0ns | 0.2% |  |
| satfold-lanes16 | 2.7ns | 1560.3ns | 0.2% |  |
| satfold-lanes16-3 | 1.9ns | 1539.7ns | 0.1% |  |
| satfold-seq | 2.9ns | 42151.1ns | 0.0% |  |

## Distribution (algo ns)

```
satfold-gate-false (n=40, range 41496.7-42521.2 ns)
  41496.7 |########################################
  41547.9 |#############
  41599.2 |####################
  41650.4 |####################
  41701.6 |####################
  41752.8 |
  41804.1 |#############
  41855.3 |#############
  41906.5 |
  41957.7 |
  42009.0 |
  42060.2 |
  42111.4 |#################################
  42162.7 |####################
  42213.9 |
  42265.1 |#############
  42316.3 |######
  42367.6 |
  42418.8 |#############
  42470.0 |
  (4 below, 2 above range)

satfold-gate-true (n=40, range 1524.4-1622.1 ns)
   1524.4 |#####################
   1529.3 |
   1534.2 |
   1539.0 |
   1543.9 |########################################
   1548.8 |##############
   1553.7 |###
   1558.6 |
   1563.5 |###
   1568.3 |
   1573.2 |#######
   1578.1 |
   1583.0 |
   1587.9 |
   1592.8 |
   1597.6 |#######
   1602.5 |##############
   1607.4 |###
   1612.3 |###
   1617.2 |
  (4 below, 3 above range)

satfold-lanes16 (n=40, range 1524.0-1606.3 ns)
   1524.0 |########################
   1528.1 |############
   1532.2 |################
   1536.3 |####
   1540.4 |####
   1544.6 |################
   1548.7 |####
   1552.8 |
   1556.9 |####
   1561.0 |
   1565.2 |
   1569.3 |
   1573.4 |
   1577.5 |########
   1581.6 |####
   1585.8 |
   1589.9 |####
   1594.0 |
   1598.1 |####
   1602.2 |########################################
  (3 below, 1 above range)

satfold-lanes16-3 (n=40, range 1523.5-1571.8 ns)
   1523.5 |########################################
   1525.9 |############
   1528.4 |######
   1530.8 |######
   1533.2 |
   1535.6 |
   1538.0 |
   1540.4 |#########
   1542.8 |
   1545.3 |######
   1547.7 |###############
   1550.1 |
   1552.5 |
   1554.9 |
   1557.3 |###
   1559.7 |######
   1562.2 |
   1564.6 |
   1567.0 |###
   1569.4 |
  (2 below, 3 above range)

satfold-seq (n=40, range 41519.2-43340.3 ns)
  41519.2 |##########################
  41610.2 |########################################
  41701.3 |######
  41792.3 |#################################
  41883.4 |#############
  41974.4 |######
  42065.5 |######
  42156.5 |#############
  42247.6 |######
  42338.7 |######
  42429.7 |######
  42520.8 |######
  42611.8 |######
  42702.9 |#############
  42793.9 |
  42885.0 |
  42976.1 |#############
  43067.1 |
  43158.2 |
  43249.2 |
  (5 below, 4 above range)

```

## Diagnostics

- **satfold-gate-true**: autocorrelation=0.82 (measurement drift or warm-up artifact)
- **satfold-lanes16**: autocorrelation=0.60 (measurement drift or warm-up artifact)
- **satfold-lanes16-3**: autocorrelation=0.73 (measurement drift or warm-up artifact)
