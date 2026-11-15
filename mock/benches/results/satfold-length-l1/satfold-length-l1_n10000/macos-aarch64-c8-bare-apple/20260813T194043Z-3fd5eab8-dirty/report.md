# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (satfold-iterfold) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline satfold-iterfold has the worst median (38.50 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest satfold-neon8 at 274 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### satfold-neon8 beats baseline by 101% (significant)

satfold-neon8 is -38.97 us (101%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-iterfold is an outlier: 140.6x slower than the field

satfold-iterfold (38.50 us) is 140.6x the fastest (274 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-lanes16 shows warm-up / thermal drift (autocorr +0.80)

satfold-lanes16's per-pass series has lag-1 autocorrelation +0.80, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-neon8, satfold-neon} vs {satfold-lanes16, satfold-lanes16-constl, satfold-lanes64, satfold-lanes4-idx, satfold-nolaw, satfold-seq, satfold-iterfold} (406% apart)

The field splits into a fast tier {satfold-neon8, satfold-neon} and a slow tier {satfold-lanes16, satfold-lanes16-constl, satfold-lanes64, satfold-lanes4-idx, satfold-nolaw, satfold-seq, satfold-iterfold} with a 406% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 140.6x the fastest

Fastest satfold-neon8 (274 ns) to slowest satfold-iterfold (38.50 us): 140.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: satfold-neon8** at 273.8 ns median (-99.3% vs baseline)
- 7 variants significantly faster than baseline
- Spread: 140.63x (fastest 273.8 ns, slowest 38496.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 38846ns | 38624ns | 38400ns | 38703ns | 39722ns | base |
| satfold-lanes16 | 1555ns | 1496ns | 1490ns | 1526ns | 1705ns | -96.00% |
| satfold-lanes16-constl | 1508ns | 1502ns | 1495ns | 1503ns | 1535ns | -96.12% |
| satfold-lanes4-idx | 7316ns | 7218ns | 7204ns | 7246ns | 7636ns | -81.17% |
| satfold-lanes64 | 2898ns | 2842ns | 2837ns | 2867ns | 3052ns | -92.54% |
| satfold-neon | 353ns | 345ns | 339ns | 346ns | 386ns | -99.09% |
| satfold-neon8 | 344ns | 334ns | 330ns | 337ns | 382ns | -99.11% |
| satfold-nolaw | 29586ns | 29509ns | 29336ns | 29543ns | 29966ns | -23.84% |
| satfold-seq | 38614ns | 38471ns | 38362ns | 38496ns | 39219ns | -0.60% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 38739ns | 38310ns | 39595ns | base | 0.846 |
| satfold-lanes16 | 1492ns | 1431ns | 1637ns | -96.15% | 21.960 |
| satfold-lanes16-constl | 1445ns | 1435ns | 1467ns | -96.27% | 22.684 |
| satfold-lanes4-idx | 7211ns | 7103ns | 7522ns | -81.39% | 4.544 |
| satfold-lanes64 | 2835ns | 2775ns | 2984ns | -92.68% | 11.560 |
| satfold-neon | 291ns | 279ns | 320ns | -99.25% | 112.562 |
| satfold-neon8 | 283ns | 270ns | 319ns | -99.27% | 115.789 |
| satfold-nolaw | 29495ns | 29253ns | 29870ns | -23.86% | 1.111 |
| satfold-seq | 38520ns | 38282ns | 39113ns | -0.57% | 0.851 |

## Performance model

- Peak throughput: **121.273 Gops/s** (satfold-neon8; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 0.851 | 0.7% |
| satfold-lanes16 | 22.825 | 18.8% |
| satfold-lanes16-constl | 22.768 | 18.8% |
| satfold-lanes4-idx | 4.606 | 3.8% |
| satfold-lanes64 | 11.776 | 9.7% |
| satfold-neon | 115.502 | 95.2% |
| satfold-neon8 | 119.700 | 98.7% |
| satfold-nolaw | 1.114 | 0.9% |
| satfold-seq | 0.854 | 0.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 38846ns | 38846ns | base |
| satfold-lanes16 | 1555ns | 1555ns | -96.00% |
| satfold-lanes16-constl | 1508ns | 1508ns | -96.12% |
| satfold-lanes4-idx | 7316ns | 7316ns | -81.17% |
| satfold-lanes64 | 2898ns | 2898ns | -92.54% |
| satfold-neon | 353ns | 353ns | -99.09% |
| satfold-neon8 | 344ns | 344ns | -99.11% |
| satfold-nolaw | 29586ns | 29586ns | -23.84% |
| satfold-seq | 38614ns | 38614ns | -0.60% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 38496ns | base | --- | [38401, 38769] | --- | --- | --- | --- |
| satfold-lanes16 | 1436ns | -37006.1ns (-96.1%) | [-37336, -36962]ns | [1433, 1464] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 1439ns | -37056.7ns (-96.3%) | [-37331, -36960]ns | [1439, 1441] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 7115ns | -31351.5ns (-81.4%) | [-31608, -31275]ns | [7106, 7175] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 2782ns | -35708.9ns (-92.8%) | [-35958, -35614]ns | [2779, 2788] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 284ns | -38215.1ns (-99.3%) | [-38485, -38120]ns | [283, 286] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 274ns | -38224.0ns (-99.3%) | [-38496, -38129]ns | [272, 275] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 29419ns | -9125.8ns (-23.7%) | [-9342, -8983]ns | [29358, 29532] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 38368ns | no significant difference | [-319, +22]ns | [38339, 38426] | no | 0.1539 | 0.1539 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 38319ns | -95.8% | -96.2% | -81.3% | -91.6% | -99.2% | -99.3% | -22.8% | +0.8% |
| 2 | 38271ns | -95.8% | -96.3% | -81.4% | -92.0% | -99.2% | -99.3% | -23.3% | +0.3% |
| 3 | 39087ns | -95.8% | -96.3% | -81.8% | -92.5% | -99.2% | -99.3% | -25.2% | -1.8% |
| 4 | 38685ns | -95.5% | -96.3% | -81.6% | -92.4% | -99.2% | -99.3% | -23.3% | -0.8% |
| 5 | 38303ns | -95.8% | -96.2% | -81.4% | -92.4% | -99.2% | -99.3% | -23.3% | -0.1% |
| 6 | 38496ns | -95.8% | -96.3% | -81.5% | -92.4% | -99.2% | -99.3% | -24.0% | +3.6% |
| 7 | 38397ns | -95.8% | -96.3% | -81.5% | -92.2% | -99.2% | -99.3% | -23.4% | +0.6% |
| 8 | 38397ns | -95.8% | -96.3% | -81.5% | -92.4% | -99.2% | -99.3% | -23.4% | -0.1% |
| 9 | 38611ns | -95.8% | -96.3% | -81.5% | -92.4% | -99.2% | -99.3% | -24.3% | -0.3% |
| 10 | 39101ns | -95.8% | -96.3% | -81.8% | -92.5% | -99.2% | -99.3% | -24.2% | -2.0% |
| 11 | 38755ns | -96.3% | -96.3% | -81.4% | -92.8% | -99.3% | -99.3% | -24.1% | -0.3% |
| 12 | 38404ns | -96.3% | -96.3% | -81.2% | -92.8% | -99.3% | -99.3% | -22.4% | -0.3% |
| 13 | 38357ns | -96.3% | -96.3% | -81.0% | -92.8% | -99.3% | -99.3% | -21.8% | +0.3% |
| 14 | 38392ns | -96.3% | -96.3% | -81.5% | -92.8% | -99.3% | -99.3% | -22.6% | +0.1% |
| 15 | 38497ns | -96.1% | -96.3% | -81.6% | -92.8% | -99.3% | -99.3% | -23.6% | -0.6% |
| 16 | 38862ns | -96.3% | -96.3% | -81.7% | -92.5% | -99.3% | -99.3% | -24.1% | -1.4% |
| 17 | 38912ns | -96.3% | -96.3% | -81.7% | -92.9% | -99.3% | -99.3% | -24.9% | -1.4% |
| 18 | 39058ns | -96.3% | -96.3% | -81.8% | -92.9% | -99.3% | -99.0% | -23.9% | -1.8% |
| 19 | 39226ns | -96.3% | -96.3% | -81.6% | -92.7% | -99.3% | -99.3% | -23.8% | -2.0% |
| 20 | 38782ns | -96.3% | -96.3% | -81.7% | -92.8% | -99.3% | -99.3% | -23.6% | +0.8% |
| 21 | 39245ns | -96.2% | -96.3% | -81.7% | -92.9% | -99.3% | -99.3% | -25.2% | -2.1% |
| 22 | 41222ns | -96.5% | -96.4% | -82.8% | -93.3% | -99.3% | -99.3% | -28.9% | -7.1% |
| 23 | 39027ns | -96.3% | -96.3% | -81.5% | -92.9% | -99.3% | -98.9% | -24.9% | -1.8% |
| 24 | 38552ns | -96.3% | -96.2% | -81.2% | -92.8% | -99.3% | -99.3% | -21.9% | +0.1% |
| 25 | 38398ns | -96.3% | -96.3% | -81.5% | -92.8% | -99.3% | -99.3% | -23.2% | +1.9% |
| 26 | 38415ns | -96.3% | -96.2% | -81.5% | -92.7% | -99.3% | -99.3% | -22.6% | +0.6% |
| 27 | 38495ns | -96.3% | -96.2% | -81.5% | -92.8% | -99.3% | -99.3% | -22.1% | +0.4% |
| 28 | 38268ns | -96.3% | -96.1% | -81.3% | -92.7% | -99.3% | -99.3% | -23.1% | +0.3% |
| 29 | 38872ns | -96.2% | -96.2% | -81.7% | -92.8% | -99.3% | -99.3% | -24.5% | -1.5% |
| 30 | 40350ns | -96.4% | -96.3% | -82.4% | -93.1% | -99.3% | -99.3% | -26.8% | -1.4% |
| 31 | 38310ns | -96.3% | -96.2% | -79.0% | -92.8% | -99.3% | -99.3% | -22.9% | +0.1% |
| 32 | 38363ns | -96.3% | -96.3% | -80.4% | -92.8% | -99.3% | -99.3% | -23.4% | -0.2% |
| 33 | 38447ns | -96.3% | -96.2% | -80.6% | -92.8% | -99.3% | -99.3% | -24.0% | -0.4% |
| 34 | 38663ns | -96.3% | -96.3% | -80.2% | -92.8% | -99.3% | -99.3% | -24.3% | -0.8% |
| 35 | 39124ns | -96.3% | -96.3% | -80.9% | -92.9% | -99.3% | -99.3% | -24.5% | -2.1% |
| 36 | 38362ns | -96.3% | -96.2% | -80.5% | -92.8% | -99.3% | -99.3% | -23.7% | -0.1% |
| 37 | 38482ns | -96.3% | -96.3% | -81.2% | -92.8% | -99.3% | -99.3% | -23.9% | -0.5% |
| 38 | 38288ns | -96.3% | -96.2% | -81.4% | -92.7% | -99.3% | -99.3% | -23.5% | +0.1% |
| 39 | 38374ns | -96.3% | -96.2% | -81.3% | -92.8% | -99.3% | -99.3% | -23.7% | +0.1% |
| 40 | 39402ns | -96.0% | -96.3% | -82.0% | -92.8% | -98.7% | -99.3% | -24.7% | -0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.222 | moderate+ |
| satfold-lanes16 | 0.797 | HIGH+ (drift/warm-up) |
| satfold-lanes16-constl | 0.577 | HIGH+ (drift/warm-up) |
| satfold-lanes4-idx | 0.505 | HIGH+ (drift/warm-up) |
| satfold-lanes64 | 0.611 | HIGH+ (drift/warm-up) |
| satfold-neon | -0.005 | ok |
| satfold-neon8 | -0.071 | ok |
| satfold-nolaw | 0.221 | moderate+ |
| satfold-seq | -0.029 | ok |

**Consistency summary:**

- **satfold-lanes16**: won 40/40, lost 0/40
- **satfold-lanes16-constl**: won 40/40, lost 0/40
- **satfold-lanes4-idx**: won 40/40, lost 0/40
- **satfold-lanes64**: won 40/40, lost 0/40
- **satfold-neon**: won 40/40, lost 0/40
- **satfold-neon8**: won 40/40, lost 0/40
- **satfold-nolaw**: won 40/40, lost 0/40
- **satfold-seq**: won 23/40, lost 12/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 3.6ns | 38739.3ns | 0.0% |  |
| satfold-lanes16 | 3.1ns | 1492.2ns | 0.2% |  |
| satfold-lanes16-constl | 2.6ns | 1444.5ns | 0.2% |  |
| satfold-lanes4-idx | 2.8ns | 7210.5ns | 0.0% |  |
| satfold-lanes64 | 2.7ns | 2834.7ns | 0.1% |  |
| satfold-neon | 2.2ns | 291.1ns | 0.8% |  |
| satfold-neon8 | 1.6ns | 283.0ns | 0.6% |  |
| satfold-nolaw | 2.3ns | 29494.9ns | 0.0% |  |
| satfold-seq | 2.0ns | 38520.0ns | 0.0% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 38309.7-39594.6 ns)
  38309.7 |############################
  38373.9 |########################################
  38438.2 |############################
  38502.4 |#####
  38566.7 |#####
  38630.9 |###########
  38695.2 |#####
  38759.4 |#####
  38823.7 |###########
  38887.9 |#####
  38952.2 |
  39016.4 |###########
  39080.7 |#################
  39144.9 |
  39209.2 |###########
  39273.4 |
  39337.6 |#####
  39401.9 |
  39466.1 |
  39530.4 |
  (4 below, 2 above range)

satfold-lanes16 (n=40, range 1431.0-1636.9 ns)
   1431.0 |########################################
   1441.3 |####
   1451.6 |
   1461.9 |
   1472.2 |##
   1482.5 |##
   1492.8 |##
   1503.1 |
   1513.4 |
   1523.7 |
   1534.0 |
   1544.3 |
   1554.5 |
   1564.8 |
   1575.1 |##
   1585.4 |
   1595.7 |
   1606.0 |
   1616.3 |##################
   1626.6 |
  (4 below, 1 above range)

satfold-lanes16-constl (n=40, range 1435.2-1466.8 ns)
   1435.2 |#########################
   1436.8 |##############################
   1438.3 |########################################
   1439.9 |####################
   1441.5 |####################
   1443.1 |#####
   1444.7 |
   1446.2 |
   1447.8 |
   1449.4 |
   1451.0 |#####
   1452.6 |
   1454.2 |
   1455.7 |#####
   1457.3 |#####
   1458.9 |
   1460.5 |##########
   1462.1 |
   1463.6 |
   1465.2 |#####
  (3 below, 3 above range)

satfold-lanes4-idx (n=40, range 7102.5-7522.1 ns)
   7102.5 |########################################
   7123.5 |##
   7144.5 |#######
   7165.5 |####
   7186.4 |
   7207.4 |###########
   7228.4 |##
   7249.4 |
   7270.4 |##
   7291.3 |
   7312.3 |
   7333.3 |
   7354.3 |
   7375.3 |
   7396.2 |
   7417.2 |
   7438.2 |
   7459.2 |####
   7480.2 |##
   7501.1 |##
  (4 below, 2 above range)

satfold-lanes64 (n=40, range 2775.2-2984.1 ns)
   2775.2 |########################################
   2785.6 |########
   2796.1 |
   2806.5 |
   2817.0 |
   2827.4 |
   2837.9 |##
   2848.3 |##
   2858.8 |
   2869.2 |
   2879.7 |
   2890.1 |
   2900.6 |
   2911.0 |##
   2921.4 |##############
   2931.9 |
   2942.3 |
   2952.8 |
   2963.2 |
   2973.7 |##
  (4 below, 2 above range)

satfold-neon (n=40, range 279.4-320.1 ns)
    279.4 |########################################
    281.4 |##############################
    283.5 |##############################
    285.5 |####################
    287.5 |
    289.6 |#####
    291.6 |##############################
    293.6 |#########################
    295.7 |
    297.7 |
    299.7 |
    301.8 |
    303.8 |
    305.8 |
    307.9 |
    309.9 |
    311.9 |
    314.0 |
    316.0 |
    318.0 |
  (3 below, 1 above range)

satfold-neon8 (n=40, range 270.2-318.9 ns)
    270.2 |########################################
    272.6 |########################################
    275.1 |###
    277.5 |
    279.9 |
    282.4 |
    284.8 |#########################
    287.3 |##########
    289.7 |
    292.1 |
    294.6 |
    297.0 |
    299.4 |
    301.9 |
    304.3 |
    306.8 |
    309.2 |
    311.6 |
    314.1 |
    316.5 |
  (5 below, 2 above range)

satfold-nolaw (n=40, range 29253.1-29870.4 ns)
  29253.1 |########################################
  29284.0 |########################################
  29314.9 |##########
  29345.7 |##############################
  29376.6 |##############################
  29407.5 |##############################
  29438.3 |
  29469.2 |####################
  29500.0 |##########
  29530.9 |####################
  29561.8 |##########
  29592.6 |
  29623.5 |####################
  29654.4 |####################
  29685.2 |##########
  29716.1 |####################
  29746.9 |
  29777.8 |##########
  29808.7 |
  29839.5 |
  (4 below, 4 above range)

satfold-seq (n=40, range 38281.5-39112.8 ns)
  38281.5 |######################
  38323.1 |########################################
  38364.6 |#################
  38406.2 |#################
  38447.8 |####
  38489.3 |####
  38530.9 |
  38572.5 |####
  38614.0 |#############
  38655.6 |########
  38697.2 |
  38738.7 |
  38780.3 |
  38821.8 |
  38863.4 |
  38905.0 |
  38946.5 |
  38988.1 |
  39029.7 |
  39071.2 |#############
  (5 below, 2 above range)

```

## Diagnostics

- **satfold-lanes16**: autocorrelation=0.80 (measurement drift or warm-up artifact)
- **satfold-lanes16-constl**: autocorrelation=0.58 (measurement drift or warm-up artifact)
- **satfold-lanes4-idx**: autocorrelation=0.51 (measurement drift or warm-up artifact)
- **satfold-lanes64**: autocorrelation=0.61 (measurement drift or warm-up artifact)
