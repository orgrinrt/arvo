# The same arms with the column start offset by one byte from a 64-byte boundary: what the licensed vector arms cost when the load stream is not aligned

8 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (satfold-iterfold) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline satfold-iterfold has the worst median (10.36 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest satfold-neon at 1.52 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### satfold-neon dominates: 409% faster than the next best (satfold-lanes4-idx)

satfold-neon (1.52 us) leads satfold-lanes4-idx (7.74 us) by 409%, a clear separation rather than a photo finish. CV 2.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### satfold-neon beats baseline by 84% (significant)

satfold-neon is -8.69 us (84%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-iterfold is an outlier: 6.8x slower than the field

satfold-iterfold (10.36 us) is 6.8x the fastest (1.52 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-neon shows warm-up / thermal drift (autocorr +0.80)

satfold-neon's per-pass series has lag-1 autocorrelation +0.80, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-neon} vs {satfold-lanes4-idx, satfold-lanes64, satfold-seq, satfold-lanes16-constl, satfold-nolaw, satfold-lanes16, satfold-iterfold} (409% apart)

The field splits into a fast tier {satfold-neon} and a slow tier {satfold-lanes4-idx, satfold-lanes64, satfold-seq, satfold-lanes16-constl, satfold-nolaw, satfold-lanes16, satfold-iterfold} with a 409% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.8x the fastest

Fastest satfold-neon (1.52 us) to slowest satfold-iterfold (10.36 us): 6.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-lanes16-constl's edge over baseline is significant but tiny (18 ns, 0.17%)

satfold-lanes16-constl differs from baseline satfold-iterfold by 18 ns (0.17%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: satfold-neon** at 1520.8 ns median (-85.3% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 6.81x (fastest 1520.8 ns, slowest 10361.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 10448ns | 10421ns | 10249ns | 10408ns | 10766ns | base |
| satfold-lanes16 | 10457ns | 10408ns | 10268ns | 10406ns | 10798ns | +0.08% |
| satfold-lanes16-constl | 10343ns | 10297ns | 10269ns | 10304ns | 10533ns | -1.01% |
| satfold-lanes4-idx | 8022ns | 7810ns | 7673ns | 7816ns | 8989ns | -23.22% |
| satfold-lanes64 | 10335ns | 10278ns | 10244ns | 10283ns | 10583ns | -1.08% |
| satfold-neon | 1598ns | 1584ns | 1567ns | 1589ns | 1659ns | -84.70% |
| satfold-nolaw | 10502ns | 10306ns | 10254ns | 10353ns | 11194ns | +0.51% |
| satfold-seq | 10341ns | 10289ns | 10252ns | 10318ns | 10497ns | -1.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 10387ns | 10193ns | 10700ns | base | 3.155 |
| satfold-lanes16 | 10394ns | 10209ns | 10731ns | +0.07% | 3.153 |
| satfold-lanes16-constl | 10280ns | 10209ns | 10467ns | -1.03% | 3.188 |
| satfold-lanes4-idx | 7946ns | 7606ns | 8882ns | -23.50% | 4.124 |
| satfold-lanes64 | 10274ns | 10186ns | 10518ns | -1.09% | 3.190 |
| satfold-neon | 1536ns | 1507ns | 1595ns | -85.21% | 21.332 |
| satfold-nolaw | 10433ns | 10194ns | 11099ns | +0.45% | 3.141 |
| satfold-seq | 10278ns | 10196ns | 10427ns | -1.05% | 3.188 |

## Performance model

- Peak throughput: **21.744 Gops/s** (satfold-neon; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 3.163 | 14.5% |
| satfold-lanes16 | 3.166 | 14.6% |
| satfold-lanes16-constl | 3.202 | 14.7% |
| satfold-lanes4-idx | 4.234 | 19.5% |
| satfold-lanes64 | 3.209 | 14.8% |
| satfold-neon | 21.546 | 99.1% |
| satfold-nolaw | 3.198 | 14.7% |
| satfold-seq | 3.203 | 14.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 10448ns | 10448ns | base |
| satfold-lanes16 | 10457ns | 10457ns | +0.08% |
| satfold-lanes16-constl | 10343ns | 10343ns | -1.01% |
| satfold-lanes4-idx | 8022ns | 8022ns | -23.22% |
| satfold-lanes64 | 10335ns | 10335ns | -1.08% |
| satfold-neon | 1598ns | 1598ns | -84.70% |
| satfold-nolaw | 10502ns | 10502ns | +0.51% |
| satfold-seq | 10341ns | 10341ns | -1.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 10361ns | base | --- | [10226, 10418] | --- | --- | --- | --- |
| satfold-lanes16 | 10349ns | no significant difference | [-5, +24]ns | [10235, 10426] | no | 0.1130 | 0.0807 | 0 |
| satfold-lanes16-constl | 10232ns | no significant difference | [-178, +10]ns | [10214, 10246] | no | 0.3129 | 0.2682 | 0 |
| satfold-lanes4-idx | 7739ns | -2588.5ns (-25.0%) | [-2655, -2506]ns | [7680, 7770] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 10210ns | -36.6ns (-0.4%) | [-174, -5]ns | [10198, 10241] | YES | 0.0387 | 0.0166 | 0 |
| satfold-neon | 1521ns | -8797.6ns (-84.9%) | [-8882, -8704]ns | [1511, 1541] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 10248ns | no significant difference | [-72, +12]ns | [10215, 10375] | no | 0.6358 | 0.6358 | 0 |
| satfold-seq | 10229ns | -54.4ns (-0.5%) | [-181, -1]ns | [10215, 10306] | YES (adj: no) | 0.0673 | 0.0385 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|
| 1 | 10724ns | -2.9% | -3.3% | -28.0% | +0.0% | -85.9% | -3.9% | -2.0% |
| 2 | 10733ns | -4.6% | -3.4% | -27.7% | -0.1% | -85.9% | -5.0% | -3.5% |
| 3 | 10719ns | -4.8% | -3.7% | -27.6% | -2.1% | -85.9% | -5.0% | -3.5% |
| 4 | 10422ns | -2.0% | -2.1% | -23.7% | -2.2% | -85.5% | -1.7% | +0.4% |
| 5 | 10208ns | +2.2% | +0.0% | -23.4% | -0.2% | -85.2% | +0.7% | -0.1% |
| 6 | 10284ns | -0.2% | -0.5% | -26.0% | -0.8% | -85.3% | -0.3% | -0.5% |
| 7 | 10202ns | +0.1% | +0.1% | -25.4% | -0.0% | -85.2% | +1.8% | -0.0% |
| 8 | 10202ns | +0.7% | +0.1% | -24.8% | +0.0% | -85.2% | +0.8% | +0.1% |
| 9 | 10196ns | +0.1% | +0.4% | -24.9% | -0.1% | -85.2% | +0.0% | -0.0% |
| 10 | 10195ns | +0.1% | +0.2% | -25.4% | -0.1% | -85.2% | -0.0% | +0.3% |
| 11 | 10723ns | +0.2% | -3.3% | -27.5% | -4.1% | -85.7% | +0.1% | -3.3% |
| 12 | 10726ns | +0.2% | -3.3% | -28.0% | -5.0% | -85.7% | +25.9% | -4.4% |
| 13 | 10728ns | +0.7% | -3.7% | -27.8% | -5.0% | -85.5% | -3.4% | -4.6% |
| 14 | 10541ns | -0.3% | -3.1% | -26.7% | -2.7% | -85.5% | -1.1% | -3.3% |
| 15 | 10409ns | -0.3% | -1.9% | -26.5% | -2.2% | -85.1% | -1.6% | -2.1% |
| 16 | 10446ns | +2.1% | -1.1% | -27.2% | -1.9% | -85.2% | -2.2% | -1.2% |
| 17 | 10490ns | +2.1% | -2.6% | -26.7% | -2.1% | -85.2% | -2.6% | -2.7% |
| 18 | 10395ns | +0.7% | -1.8% | -25.6% | -1.5% | -85.3% | -1.5% | -1.6% |
| 19 | 10388ns | -1.2% | -1.5% | -26.4% | -1.4% | -85.3% | -1.8% | -1.8% |
| 20 | 10236ns | +0.1% | -0.3% | -25.5% | -0.5% | -85.1% | -0.3% | -0.4% |
| 21 | 10620ns | -2.2% | +1.8% | -25.6% | -1.8% | -85.3% | +3.0% | -3.7% |
| 22 | 10371ns | +0.2% | +4.2% | -24.1% | +2.0% | -84.7% | +5.4% | -1.4% |
| 23 | 10534ns | +1.2% | -2.1% | -24.5% | +0.0% | -84.6% | -0.6% | -3.2% |
| 24 | 10415ns | +3.4% | -1.4% | -21.7% | -2.0% | -84.7% | +0.3% | -1.5% |
| 25 | 10609ns | -1.8% | -3.7% | +33.4% | -3.1% | -85.0% | -0.8% | -3.7% |
| 26 | 10400ns | +0.4% | -1.6% | -20.6% | -1.9% | -84.7% | +4.8% | -1.7% |
| 27 | 10628ns | -1.2% | -3.9% | -23.6% | -4.1% | -85.1% | -1.6% | -4.0% |
| 28 | 10295ns | +4.4% | -0.5% | -18.9% | -0.7% | -84.5% | +1.4% | +0.2% |
| 29 | 10196ns | +3.6% | +0.2% | -21.2% | -0.1% | -84.8% | +3.2% | +2.2% |
| 30 | 10193ns | +1.9% | +0.2% | -21.2% | -0.0% | -84.4% | +5.4% | +2.2% |
| 31 | 10250ns | -0.3% | -0.4% | -25.8% | +0.6% | -85.3% | -0.5% | -0.5% |
| 32 | 10197ns | +0.1% | +0.2% | -25.4% | +0.1% | -85.2% | +0.1% | +0.3% |
| 33 | 10188ns | +0.2% | +0.6% | -25.4% | +0.2% | -85.2% | +0.4% | +1.5% |
| 34 | 10189ns | +0.2% | +0.3% | -25.0% | +0.1% | -85.2% | +0.0% | +1.8% |
| 35 | 10194ns | +0.2% | +0.3% | -24.2% | +0.3% | -85.2% | -0.0% | +2.1% |
| 36 | 10217ns | +0.1% | +0.3% | -24.1% | +0.0% | -85.2% | +0.4% | +2.3% |
| 37 | 10198ns | +0.1% | +0.5% | -23.8% | -0.1% | -85.2% | +0.0% | +1.5% |
| 38 | 10351ns | -1.4% | -1.0% | -24.2% | -1.4% | -85.4% | -1.3% | -0.5% |
| 39 | 10211ns | +1.0% | +1.1% | -23.9% | +0.2% | -85.2% | -0.2% | +0.0% |
| 40 | 10242ns | -0.3% | -0.0% | -25.7% | +1.1% | -85.3% | -0.5% | -0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.596 | HIGH+ (drift/warm-up) |
| satfold-lanes16 | 0.634 | HIGH+ (drift/warm-up) |
| satfold-lanes16-constl | 0.480 | moderate+ |
| satfold-lanes4-idx | 0.126 | ok |
| satfold-lanes64 | 0.594 | HIGH+ (drift/warm-up) |
| satfold-neon | 0.802 | HIGH+ (drift/warm-up) |
| satfold-nolaw | 0.153 | ok |
| satfold-seq | 0.449 | moderate+ |

**Consistency summary:**

- **satfold-lanes16**: won 14/40, lost 25/40
- **satfold-lanes16-constl**: won 23/40, lost 14/40
- **satfold-lanes4-idx**: won 39/40, lost 1/40
- **satfold-lanes64**: won 24/40, lost 7/40
- **satfold-neon**: won 40/40, lost 0/40
- **satfold-nolaw**: won 20/40, lost 14/40
- **satfold-seq**: won 25/40, lost 12/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 2.6ns | 10386.6ns | 0.0% |  |
| satfold-lanes16 | 2.7ns | 10393.5ns | 0.0% |  |
| satfold-lanes16-constl | 2.8ns | 10279.5ns | 0.0% |  |
| satfold-lanes4-idx | 3.6ns | 7945.6ns | 0.0% |  |
| satfold-lanes64 | 3.0ns | 10273.7ns | 0.0% |  |
| satfold-neon | 2.2ns | 1536.1ns | 0.1% |  |
| satfold-nolaw | 2.4ns | 10433.3ns | 0.0% |  |
| satfold-seq | 2.3ns | 10277.5ns | 0.0% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 10193.4-10700.0 ns)
  10193.4 |########################################
  10218.8 |#######
  10244.1 |###
  10269.4 |###
  10294.8 |###
  10320.1 |
  10345.4 |###
  10370.8 |##########
  10396.1 |##########
  10421.4 |#######
  10446.7 |
  10472.1 |###
  10497.4 |
  10522.7 |#######
  10548.1 |
  10573.4 |
  10598.7 |#######
  10624.1 |###
  10649.4 |
  10674.7 |
  (3 below, 6 above range)

satfold-lanes16 (n=40, range 10209.1-10731.1 ns)
  10209.1 |########################################
  10235.2 |##########
  10261.3 |#######
  10287.4 |
  10313.5 |###
  10339.6 |
  10365.7 |##########
  10391.8 |#######
  10417.9 |#######
  10444.0 |###
  10470.1 |###
  10496.2 |#######
  10522.3 |
  10548.4 |###
  10574.5 |
  10600.6 |
  10626.7 |
  10652.8 |#######
  10678.9 |
  10705.0 |###
  (3 below, 5 above range)

satfold-lanes16-constl (n=40, range 10209.5-10466.8 ns)
  10209.5 |########################################
  10222.3 |###########
  10235.2 |#################
  10248.1 |##
  10260.9 |##
  10273.8 |
  10286.7 |
  10299.5 |##
  10312.4 |##
  10325.3 |########
  10338.1 |
  10351.0 |
  10363.9 |###########
  10376.7 |
  10389.6 |
  10402.5 |
  10415.3 |
  10428.2 |
  10441.1 |
  10453.9 |
  (3 below, 2 above range)

satfold-lanes4-idx (n=40, range 7605.7-8881.9 ns)
   7605.7 |########################################
   7669.5 |###################################
   7733.3 |###################################
   7797.1 |##########
   7861.0 |##########
   7924.8 |##########
   7988.6 |##########
   8052.4 |
   8116.2 |##########
   8180.0 |
   8243.8 |#####
   8307.6 |#####
   8371.4 |
   8435.3 |
   8499.1 |
   8562.9 |
   8626.7 |
   8690.5 |
   8754.3 |
   8818.1 |
  (5 below, 1 above range)

satfold-lanes64 (n=40, range 10185.6-10517.9 ns)
  10185.6 |########################################
  10202.2 |##################
  10218.9 |##############
  10235.5 |##########
  10252.1 |#######
  10268.7 |###
  10285.3 |###
  10301.9 |###
  10318.5 |
  10335.1 |###
  10351.7 |
  10368.4 |
  10385.0 |
  10401.6 |
  10418.2 |###
  10434.8 |
  10451.4 |
  10468.0 |
  10484.6 |###
  10501.2 |
  (5 below, 4 above range)

satfold-neon (n=40, range 1507.0-1594.9 ns)
   1507.0 |########################################
   1511.4 |#############
   1515.8 |
   1520.2 |
   1524.6 |###
   1529.0 |#############
   1533.4 |###
   1537.8 |
   1542.2 |
   1546.6 |##########
   1551.0 |######
   1555.4 |###
   1559.8 |
   1564.2 |
   1568.6 |
   1573.0 |
   1577.4 |
   1581.8 |
   1586.2 |#############
   1590.6 |
  (4 below, 4 above range)

satfold-nolaw (n=40, range 10193.7-11099.5 ns)
  10193.7 |########################################
  10239.0 |############
  10284.3 |#####
  10329.6 |##
  10374.9 |##
  10420.2 |##########
  10465.4 |##
  10510.7 |#####
  10556.0 |
  10601.3 |
  10646.6 |
  10691.9 |##
  10737.2 |##
  10782.5 |
  10827.8 |
  10873.0 |##
  10918.3 |#####
  10963.6 |
  11008.9 |
  11054.2 |
  (2 below, 1 above range)

satfold-seq (n=40, range 10195.9-10427.5 ns)
  10195.9 |########################################
  10207.5 |#################
  10219.0 |##################################
  10230.6 |###########
  10242.2 |#####
  10253.8 |#####
  10265.4 |
  10276.9 |
  10288.5 |#####
  10300.1 |
  10311.7 |#####
  10323.3 |#####
  10334.8 |#####
  10346.4 |#################
  10358.0 |
  10369.6 |###########
  10381.2 |
  10392.7 |
  10404.3 |###########
  10415.9 |#####
  (5 below, 3 above range)

```

## Diagnostics

- **satfold-iterfold**: autocorrelation=0.60 (measurement drift or warm-up artifact)
- **satfold-lanes16**: autocorrelation=0.63 (measurement drift or warm-up artifact)
- **satfold-lanes64**: autocorrelation=0.59 (measurement drift or warm-up artifact)
- **satfold-neon**: autocorrelation=0.80 (measurement drift or warm-up artifact)
