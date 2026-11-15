# The same arms with the column start offset by one byte from a 64-byte boundary: what the licensed vector arms cost when the load stream is not aligned

8 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (satfold-iterfold) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline satfold-iterfold has the worst median (38.60 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest satfold-neon at 284 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### satfold-neon dominates: 413% faster than the next best (satfold-lanes16)

satfold-neon (284 ns) leads satfold-lanes16 (1.46 us) by 413%, a clear separation rather than a photo finish. CV 2.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### satfold-neon beats baseline by 99% (significant)

satfold-neon is -38.31 us (99%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-iterfold is an outlier: 135.9x slower than the field

satfold-iterfold (38.60 us) is 135.9x the fastest (284 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-neon shows warm-up / thermal drift (autocorr +0.83)

satfold-neon's per-pass series has lag-1 autocorrelation +0.83, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-neon} vs {satfold-lanes16, satfold-lanes16-constl, satfold-lanes64, satfold-lanes4-idx, satfold-nolaw, satfold-seq, satfold-iterfold} (413% apart)

The field splits into a fast tier {satfold-neon} and a slow tier {satfold-lanes16, satfold-lanes16-constl, satfold-lanes64, satfold-lanes4-idx, satfold-nolaw, satfold-seq, satfold-iterfold} with a 413% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 135.9x the fastest

Fastest satfold-neon (284 ns) to slowest satfold-iterfold (38.60 us): 135.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: satfold-neon** at 283.9 ns median (-99.3% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 135.95x (fastest 283.9 ns, slowest 38602.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 38950ns | 38701ns | 38417ns | 38760ns | 40054ns | base |
| satfold-lanes16 | 1540ns | 1518ns | 1491ns | 1535ns | 1605ns | -96.05% |
| satfold-lanes16-constl | 1537ns | 1518ns | 1496ns | 1515ns | 1642ns | -96.06% |
| satfold-lanes4-idx | 7535ns | 7326ns | 7207ns | 7335ns | 8462ns | -80.66% |
| satfold-lanes64 | 2892ns | 2849ns | 2842ns | 2858ns | 3041ns | -92.58% |
| satfold-neon | 349ns | 345ns | 341ns | 347ns | 363ns | -99.10% |
| satfold-nolaw | 29597ns | 29455ns | 29319ns | 29518ns | 30111ns | -24.01% |
| satfold-seq | 38898ns | 38688ns | 38454ns | 38805ns | 39620ns | -0.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 38841ns | 38333ns | 39897ns | base | 0.844 |
| satfold-lanes16 | 1479ns | 1434ns | 1540ns | -96.19% | 22.162 |
| satfold-lanes16-constl | 1475ns | 1436ns | 1581ns | -96.20% | 22.212 |
| satfold-lanes4-idx | 7426ns | 7105ns | 8339ns | -80.88% | 4.413 |
| satfold-lanes64 | 2828ns | 2782ns | 2968ns | -92.72% | 11.588 |
| satfold-neon | 287ns | 280ns | 300ns | -99.26% | 113.994 |
| satfold-nolaw | 29519ns | 29249ns | 30027ns | -24.00% | 1.110 |
| satfold-seq | 38798ns | 38370ns | 39515ns | -0.11% | 0.845 |

## Performance model

- Peak throughput: **116.903 Gops/s** (satfold-neon; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 0.849 | 0.7% |
| satfold-lanes16 | 22.492 | 19.2% |
| satfold-lanes16-constl | 22.466 | 19.2% |
| satfold-lanes4-idx | 4.538 | 3.9% |
| satfold-lanes64 | 11.764 | 10.1% |
| satfold-neon | 115.401 | 98.7% |
| satfold-nolaw | 1.116 | 1.0% |
| satfold-seq | 0.849 | 0.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 38950ns | 38950ns | base |
| satfold-lanes16 | 1540ns | 1540ns | -96.05% |
| satfold-lanes16-constl | 1537ns | 1537ns | -96.06% |
| satfold-lanes4-idx | 7535ns | 7535ns | -80.66% |
| satfold-lanes64 | 2892ns | 2892ns | -92.58% |
| satfold-neon | 349ns | 349ns | -99.10% |
| satfold-nolaw | 29597ns | 29597ns | -24.01% |
| satfold-seq | 38898ns | 38898ns | -0.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 38602ns | base | --- | [38448, 38822] | --- | --- | --- | --- |
| satfold-lanes16 | 1457ns | -37079.9ns (-96.1%) | [-37376, -36958]ns | [1455, 1501] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 1459ns | -37144.0ns (-96.2%) | [-37376, -36972]ns | [1440, 1461] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 7221ns | -31420.0ns (-81.4%) | [-31537, -31252]ns | [7124, 7335] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 2785ns | -35781.9ns (-92.7%) | [-35999, -35608]ns | [2784, 2793] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 284ns | -38320.0ns (-99.3%) | [-38532, -38157]ns | [283, 288] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 29374ns | -9187.5ns (-23.8%) | [-9350, -9003]ns | [29302, 29529] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 38577ns | no significant difference | [-114, +135]ns | [38490, 39027] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|
| 1 | 38605ns | -96.2% | -96.3% | -81.6% | -92.8% | -99.3% | -22.4% | +2.5% |
| 2 | 38487ns | -96.2% | -96.3% | -81.5% | -92.5% | -99.3% | -22.0% | +2.1% |
| 3 | 38428ns | -96.1% | -96.0% | -81.5% | -92.4% | -99.3% | -23.3% | -0.2% |
| 4 | 38415ns | -96.1% | -96.2% | -81.3% | -92.6% | -99.3% | -23.9% | +0.3% |
| 5 | 38349ns | -96.2% | -96.3% | -81.5% | -92.7% | -99.3% | -23.7% | +2.2% |
| 6 | 38395ns | -96.2% | -96.3% | -81.5% | -92.7% | -99.3% | -22.0% | +0.4% |
| 7 | 38341ns | -96.2% | -96.2% | -81.5% | -92.7% | -99.3% | -22.4% | -0.0% |
| 8 | 38409ns | -96.1% | -96.2% | -80.7% | -92.4% | -99.3% | -23.5% | +0.4% |
| 9 | 38349ns | -96.2% | -96.2% | -81.1% | -92.4% | -99.3% | -21.2% | +0.2% |
| 10 | 38305ns | -96.1% | -96.2% | -80.9% | -92.6% | -99.3% | -23.1% | +0.5% |
| 11 | 39176ns | -96.1% | -96.3% | -81.8% | -92.9% | -99.3% | -23.5% | +2.1% |
| 12 | 38642ns | -95.9% | -96.1% | -81.6% | -92.8% | -99.3% | -23.9% | +1.1% |
| 13 | 40107ns | -96.2% | -96.4% | -82.3% | -93.1% | -99.3% | -27.1% | -2.4% |
| 14 | 38998ns | -96.1% | -96.2% | -81.8% | -92.9% | -99.3% | -24.7% | -1.0% |
| 15 | 38325ns | -96.1% | -96.2% | -81.1% | -92.7% | -99.3% | -22.0% | +0.2% |
| 16 | 38488ns | -96.1% | -96.2% | -81.5% | -92.8% | -99.3% | -22.2% | +1.5% |
| 17 | 38340ns | -96.1% | -96.2% | -81.5% | -92.7% | -99.2% | -23.0% | +0.2% |
| 18 | 38534ns | -96.1% | -96.2% | -81.6% | -92.8% | -99.3% | -23.8% | -0.5% |
| 19 | 38416ns | -96.1% | -96.1% | -81.4% | -92.8% | -99.2% | -23.8% | -0.2% |
| 20 | 38346ns | -96.1% | -96.2% | -81.5% | -92.6% | -99.3% | -23.4% | +3.5% |
| 21 | 38320ns | -96.2% | -96.2% | -80.4% | -92.7% | -99.2% | -22.7% | +0.3% |
| 22 | 38797ns | -96.3% | -96.2% | -81.0% | -92.8% | -99.2% | -24.6% | -0.3% |
| 23 | 39858ns | -96.4% | -96.3% | -81.5% | -93.0% | -99.3% | -26.6% | -1.4% |
| 24 | 39890ns | -96.3% | -96.0% | -81.6% | -93.0% | -99.2% | -26.5% | -3.6% |
| 25 | 38769ns | -96.3% | -95.8% | -64.5% | -92.8% | -99.2% | -24.4% | -1.1% |
| 26 | 38664ns | -95.9% | -96.2% | -79.6% | -92.8% | -99.2% | -24.3% | -0.5% |
| 27 | 38468ns | -96.2% | -95.8% | -80.4% | -92.8% | -99.2% | -23.9% | +2.1% |
| 28 | 39534ns | -96.3% | -96.3% | -80.9% | -93.0% | -99.2% | -25.9% | +0.3% |
| 29 | 41923ns | -96.5% | -96.0% | -82.1% | -93.4% | -99.3% | -30.3% | -7.0% |
| 30 | 39392ns | -96.3% | -95.8% | -80.9% | -92.9% | -99.2% | -25.6% | -2.3% |
| 31 | 38558ns | -96.0% | -96.3% | -81.5% | -92.7% | -99.3% | -24.0% | -0.1% |
| 32 | 38989ns | -95.9% | -96.3% | -81.5% | -91.2% | -99.3% | -24.9% | +0.4% |
| 33 | 39244ns | -96.3% | -96.3% | -81.5% | -92.5% | -99.3% | -25.4% | -0.3% |
| 34 | 38341ns | -96.3% | -96.3% | -80.8% | -92.6% | -99.3% | -22.1% | +0.5% |
| 35 | 38980ns | -96.3% | -96.3% | -81.2% | -92.5% | -99.3% | -23.7% | -1.0% |
| 36 | 38783ns | -96.3% | -96.3% | -81.3% | -92.7% | -99.3% | -23.3% | -0.2% |
| 37 | 39228ns | -96.3% | -96.3% | -81.1% | -92.9% | -99.3% | -25.3% | +0.4% |
| 38 | 38600ns | -96.3% | -96.3% | -81.5% | -92.8% | -99.3% | -24.1% | -0.4% |
| 39 | 39006ns | -96.3% | -96.3% | -81.7% | -92.9% | -99.3% | -24.3% | -1.5% |
| 40 | 38848ns | -96.3% | -96.3% | -80.9% | -92.8% | -99.3% | -22.2% | -0.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.353 | moderate+ |
| satfold-lanes16 | 0.398 | moderate+ |
| satfold-lanes16-constl | 0.328 | moderate+ |
| satfold-lanes4-idx | 0.097 | ok |
| satfold-lanes64 | 0.239 | moderate+ |
| satfold-neon | 0.830 | HIGH+ (drift/warm-up) |
| satfold-nolaw | 0.246 | moderate+ |
| satfold-seq | 0.067 | ok |

**Consistency summary:**

- **satfold-lanes16**: won 40/40, lost 0/40
- **satfold-lanes16-constl**: won 40/40, lost 0/40
- **satfold-lanes4-idx**: won 40/40, lost 0/40
- **satfold-lanes64**: won 40/40, lost 0/40
- **satfold-neon**: won 40/40, lost 0/40
- **satfold-nolaw**: won 40/40, lost 0/40
- **satfold-seq**: won 18/40, lost 20/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 3.0ns | 38841.2ns | 0.0% |  |
| satfold-lanes16 | 2.4ns | 1478.6ns | 0.2% |  |
| satfold-lanes16-constl | 2.0ns | 1475.3ns | 0.1% |  |
| satfold-lanes4-idx | 3.1ns | 7426.1ns | 0.0% |  |
| satfold-lanes64 | 2.5ns | 2827.7ns | 0.1% |  |
| satfold-neon | 2.0ns | 287.5ns | 0.7% |  |
| satfold-nolaw | 2.2ns | 29518.7ns | 0.0% |  |
| satfold-seq | 2.6ns | 38798.2ns | 0.0% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 38333.4-39897.1 ns)
  38333.4 |########################################
  38411.6 |##############################
  38489.7 |##########
  38567.9 |###############
  38646.1 |#####
  38724.3 |###############
  38802.5 |#####
  38880.7 |
  38958.9 |####################
  39037.0 |
  39115.2 |#####
  39193.4 |##########
  39271.6 |
  39349.8 |#####
  39428.0 |
  39506.2 |#####
  39584.3 |
  39662.5 |
  39740.7 |
  39818.9 |##########
  (3 below, 2 above range)

satfold-lanes16 (n=40, range 1434.1-1539.9 ns)
   1434.1 |####################
   1439.4 |
   1444.7 |
   1450.0 |##############################
   1455.3 |########################################
   1460.6 |#####
   1465.9 |
   1471.1 |
   1476.4 |
   1481.7 |#####
   1487.0 |
   1492.3 |#####
   1497.6 |#####
   1502.9 |###################################
   1508.1 |###############
   1513.4 |
   1518.7 |
   1524.0 |
   1529.3 |
   1534.6 |
  (4 below, 4 above range)

satfold-lanes16-constl (n=40, range 1435.8-1580.5 ns)
   1435.8 |########################################
   1443.1 |
   1450.3 |
   1457.5 |###########################
   1464.8 |
   1472.0 |
   1479.2 |##
   1486.5 |
   1493.7 |#######
   1500.9 |
   1508.2 |
   1515.4 |
   1522.6 |
   1529.9 |##
   1537.1 |
   1544.3 |
   1551.6 |
   1558.8 |
   1566.0 |
   1573.3 |
  (3 below, 5 above range)

satfold-lanes4-idx (n=40, range 7104.8-8338.9 ns)
   7104.8 |########################################
   7166.5 |#####
   7228.2 |###########
   7290.0 |########
   7351.7 |###########
   7413.4 |#####
   7475.1 |###########
   7536.8 |##
   7598.5 |
   7660.2 |
   7721.9 |
   7783.6 |
   7845.3 |##
   7907.0 |
   7968.7 |
   8030.4 |
   8092.1 |
   8153.8 |
   8215.5 |
   8277.2 |
  (4 below, 1 above range)

satfold-lanes64 (n=40, range 2782.1-2967.9 ns)
   2782.1 |########################################
   2791.4 |#
   2800.7 |
   2810.0 |
   2819.3 |#
   2828.5 |###
   2837.8 |#
   2847.1 |###
   2856.4 |
   2865.7 |
   2875.0 |#
   2884.3 |
   2893.6 |#
   2902.9 |###
   2912.2 |
   2921.5 |#
   2930.7 |#
   2940.0 |
   2949.3 |
   2958.6 |
  (2 below, 1 above range)

satfold-neon (n=40, range 280.3-300.2 ns)
    280.3 |#########################
    281.3 |####################
    282.3 |##########
    283.3 |########################################
    284.3 |#####
    285.3 |###############
    286.3 |
    287.3 |
    288.3 |
    289.3 |##########
    290.3 |###############
    291.3 |
    292.3 |
    293.3 |
    294.2 |
    295.2 |
    296.2 |#####
    297.2 |#####
    298.2 |###############
    299.2 |#####
  (3 below, 3 above range)

satfold-nolaw (n=40, range 29248.6-30027.1 ns)
  29248.6 |########################################
  29287.5 |############################
  29326.5 |#####
  29365.4 |######################
  29404.3 |#####
  29443.2 |###########
  29482.2 |
  29521.1 |###########
  29560.0 |
  29598.9 |#####
  29637.9 |
  29676.8 |
  29715.7 |#################
  29754.6 |
  29793.6 |
  29832.5 |#####
  29871.4 |#####
  29910.3 |
  29949.3 |######################
  29988.2 |
  (5 below, 3 above range)

satfold-seq (n=40, range 38370.1-39514.7 ns)
  38370.1 |########################################
  38427.3 |####################
  38484.5 |####################
  38541.7 |##########################
  38599.0 |#############
  38656.2 |######
  38713.4 |######
  38770.7 |
  38827.9 |
  38885.1 |
  38942.4 |
  38999.6 |#############
  39056.8 |######
  39114.1 |####################
  39171.3 |######
  39228.5 |######
  39285.8 |#############
  39343.0 |######
  39400.2 |
  39457.4 |
  (5 below, 4 above range)

```

## Diagnostics

- **satfold-neon**: autocorrelation=0.83 (measurement drift or warm-up artifact)
