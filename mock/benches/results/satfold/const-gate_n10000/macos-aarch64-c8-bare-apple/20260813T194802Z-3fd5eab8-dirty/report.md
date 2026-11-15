# Does a const gate erase in time: the licensed arm reached directly, the same arm reached through a const verdict computed by an exhaustive sweep in a const fn, and the same gate over a law that is false so it selects the fallback

5 variants, 40 samples per variant.
Baseline: **satfold-gate-false**

## Highlights

Baseline for all deltas below: **satfold-gate-false**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-lanes16 beats baseline by 96% (significant)

satfold-lanes16 is -36.90 us (96%) faster than baseline satfold-gate-false, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-seq is an outlier: 26.9x slower than the field

satfold-seq (38.74 us) is 26.9x the fastest (1.44 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-lanes16-3 shows warm-up / thermal drift (autocorr +0.85)

satfold-lanes16-3's per-pass series has lag-1 autocorrelation +0.85, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-gate-true, satfold-lanes16, satfold-lanes16-3} vs {satfold-gate-false, satfold-seq} (2444% apart)

The field splits into a fast tier {satfold-gate-true, satfold-lanes16, satfold-lanes16-3} and a slow tier {satfold-gate-false, satfold-seq} with a 2444% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 26.9x the fastest

Fastest satfold-gate-true (1.44 us) to slowest satfold-seq (38.74 us): 26.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: satfold-gate-true** at 1438.1 ns median (-96.3% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 26.94x (fastest 1438.1 ns, slowest 38740.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-gate-false | 38621ns | 38478ns | 38403ns | 38496ns | 39217ns | base |
| satfold-gate-true | 1528ns | 1505ns | 1494ns | 1515ns | 1600ns | -96.04% |
| satfold-lanes16 | 1528ns | 1518ns | 1493ns | 1521ns | 1580ns | -96.04% |
| satfold-lanes16-3 | 1583ns | 1571ns | 1496ns | 1574ns | 1694ns | -95.90% |
| satfold-seq | 38976ns | 38842ns | 38356ns | 38867ns | 39926ns | +0.92% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-gate-false | 38526ns | 38311ns | 39116ns | base | 0.851 |
| satfold-gate-true | 1460ns | 1433ns | 1520ns | -96.21% | 22.442 |
| satfold-lanes16 | 1465ns | 1434ns | 1513ns | -96.20% | 22.366 |
| satfold-lanes16-3 | 1519ns | 1436ns | 1627ns | -96.06% | 21.578 |
| satfold-seq | 38884ns | 38290ns | 39814ns | +0.93% | 0.843 |

## Performance model

- Peak throughput: **22.864 Gops/s** (satfold-gate-true; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-gate-false | 0.854 | 3.7% |
| satfold-gate-true | 22.786 | 99.7% |
| satfold-lanes16 | 22.509 | 98.4% |
| satfold-lanes16-3 | 21.718 | 95.0% |
| satfold-seq | 0.846 | 3.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-gate-false | 38621ns | 38621ns | base |
| satfold-gate-true | 1528ns | 1528ns | -96.04% |
| satfold-lanes16 | 1528ns | 1528ns | -96.04% |
| satfold-lanes16-3 | 1583ns | 1583ns | -95.90% |
| satfold-seq | 38976ns | 38976ns | +0.92% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-gate-false | 38391ns | base | --- | [38374, 38405] | --- | --- | --- | --- |
| satfold-gate-true | 1438ns | -36940.4ns (-96.2%) | [-36958, -36900]ns | [1435, 1460] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16 | 1456ns | -36931.9ns (-96.2%) | [-36964, -36901]ns | [1454, 1460] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-3 | 1509ns | -36902.1ns (-96.1%) | [-36949, -36861]ns | [1471, 1534] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 38741ns | no significant difference | [-11, +617]ns | [38375, 39082] | no | 0.1539 | 0.1539 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | satfold-gate-false | satfold-gate-true | satfold-lanes16 | satfold-lanes16-3 | satfold-seq |
|---|---|---|---|---|---|
| 1 | 38375ns | -96.1% | -96.1% | -96.2% | +2.9% |
| 2 | 38404ns | -96.0% | -96.0% | -96.2% | +2.6% |
| 3 | 38343ns | -95.9% | -96.1% | -96.1% | +2.6% |
| 4 | 39757ns | -96.2% | -96.2% | -96.2% | -1.3% |
| 5 | 39173ns | -96.2% | -96.1% | -96.1% | +0.5% |
| 6 | 38978ns | -96.3% | -96.1% | -96.1% | +0.9% |
| 7 | 39217ns | -96.3% | -96.2% | -96.1% | +1.5% |
| 8 | 38402ns | -96.2% | -96.1% | -96.0% | +4.5% |
| 9 | 38400ns | -96.2% | -96.1% | -96.0% | +3.1% |
| 10 | 38498ns | -96.2% | -96.1% | -96.0% | +5.1% |
| 11 | 38388ns | -96.3% | -96.3% | -95.8% | +4.4% |
| 12 | 38330ns | -96.3% | -96.2% | -95.8% | +0.3% |
| 13 | 38398ns | -96.3% | -96.3% | -95.8% | -0.2% |
| 14 | 38311ns | -96.3% | -96.3% | -95.8% | +1.7% |
| 15 | 38391ns | -96.3% | -96.3% | -95.8% | +0.7% |
| 16 | 38386ns | -96.3% | -96.3% | -95.8% | -0.3% |
| 17 | 38340ns | -96.3% | -96.3% | -95.8% | -0.1% |
| 18 | 38405ns | -96.3% | -96.3% | -95.8% | -0.2% |
| 19 | 38323ns | -96.3% | -96.3% | -95.8% | +1.9% |
| 20 | 38318ns | -96.2% | -96.3% | -95.8% | +2.1% |
| 21 | 38464ns | -96.3% | -96.2% | -96.3% | +1.7% |
| 22 | 38300ns | -96.3% | -96.2% | -96.2% | +1.9% |
| 23 | 38375ns | -96.3% | -96.2% | -96.3% | +0.8% |
| 24 | 38386ns | -96.3% | -96.2% | -96.3% | +1.8% |
| 25 | 38301ns | -96.3% | -96.2% | -96.2% | +0.8% |
| 26 | 38391ns | -96.3% | -96.2% | -96.2% | +0.0% |
| 27 | 38359ns | -96.3% | -96.2% | -96.2% | +0.3% |
| 28 | 38605ns | -96.3% | -96.2% | -96.3% | +2.4% |
| 29 | 38323ns | -96.3% | -96.2% | -96.3% | +1.3% |
| 30 | 38313ns | -96.3% | -96.2% | -96.3% | +1.4% |
| 31 | 38515ns | -96.3% | -96.1% | -96.1% | -0.5% |
| 32 | 39321ns | -96.4% | -96.3% | -96.2% | -2.7% |
| 33 | 38527ns | -96.3% | -96.3% | -96.1% | -0.6% |
| 34 | 38355ns | -96.2% | -96.2% | -96.1% | -0.0% |
| 35 | 38298ns | -96.1% | -96.2% | -96.1% | -0.1% |
| 36 | 38373ns | -96.1% | -96.2% | -96.1% | -0.2% |
| 37 | 38785ns | -96.1% | -96.2% | -96.1% | -1.3% |
| 38 | 39094ns | -96.1% | -96.3% | -96.2% | -2.0% |
| 39 | 38416ns | -96.1% | -96.2% | -96.2% | -0.2% |
| 40 | 38395ns | -96.1% | -96.2% | -96.2% | -0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-gate-false | 0.383 | moderate+ |
| satfold-gate-true | 0.840 | HIGH+ (drift/warm-up) |
| satfold-lanes16 | 0.799 | HIGH+ (drift/warm-up) |
| satfold-lanes16-3 | 0.851 | HIGH+ (drift/warm-up) |
| satfold-seq | 0.698 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **satfold-gate-true**: won 40/40, lost 0/40
- **satfold-lanes16**: won 40/40, lost 0/40
- **satfold-lanes16-3**: won 40/40, lost 0/40
- **satfold-seq**: won 12/40, lost 24/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-gate-false | 2.0ns | 38525.8ns | 0.0% |  |
| satfold-gate-true | 1.5ns | 1460.1ns | 0.1% |  |
| satfold-lanes16 | 2.6ns | 1465.1ns | 0.2% |  |
| satfold-lanes16-3 | 1.7ns | 1518.6ns | 0.1% |  |
| satfold-seq | 2.0ns | 38884.2ns | 0.0% |  |

## Distribution (algo ns)

```
satfold-gate-false (n=40, range 38310.9-39116.3 ns)
  38310.9 |################################
  38351.1 |########################################
  38391.4 |############################
  38431.7 |####
  38472.0 |####
  38512.2 |########
  38552.5 |
  38592.8 |####
  38633.0 |
  38673.3 |
  38713.6 |
  38753.9 |####
  38794.1 |
  38834.4 |
  38874.7 |
  38914.9 |
  38955.2 |####
  38995.5 |
  39035.8 |
  39076.0 |####
  (3 below, 4 above range)

satfold-gate-true (n=40, range 1433.2-1520.1 ns)
   1433.2 |########################################
   1437.5 |#######
   1441.9 |##
   1446.2 |##
   1450.5 |
   1454.9 |#####
   1459.2 |##
   1463.6 |##
   1467.9 |##
   1472.3 |##
   1476.6 |
   1481.0 |
   1485.3 |
   1489.7 |
   1494.0 |##
   1498.3 |
   1502.7 |##
   1507.0 |###############
   1511.4 |
   1515.7 |
  (3 below, 2 above range)

satfold-lanes16 (n=40, range 1433.7-1512.6 ns)
   1433.7 |#########################
   1437.6 |#######
   1441.6 |
   1445.5 |
   1449.5 |##########
   1453.4 |########################################
   1457.4 |#######
   1461.3 |###
   1465.3 |
   1469.2 |
   1473.2 |
   1477.1 |###
   1481.0 |###
   1485.0 |
   1488.9 |
   1492.9 |
   1496.8 |###
   1500.8 |
   1504.7 |##########
   1508.7 |##############
  (2 below, 2 above range)

satfold-lanes16-3 (n=40, range 1436.2-1627.0 ns)
   1436.2 |########################################
   1445.8 |#####
   1455.3 |#################
   1464.9 |
   1474.4 |#####
   1483.9 |#####
   1493.5 |
   1503.0 |########################################
   1512.5 |#####
   1522.1 |#####
   1531.6 |#################
   1541.1 |###########
   1550.7 |
   1560.2 |
   1569.7 |
   1579.3 |
   1588.8 |
   1598.4 |
   1607.9 |
   1617.4 |############################
  (3 below, 5 above range)

satfold-seq (n=40, range 38290.2-39813.8 ns)
  38290.2 |########################################
  38366.4 |###
  38442.6 |#######
  38518.7 |
  38594.9 |##########
  38671.1 |
  38747.3 |###
  38823.5 |###
  38899.6 |###
  38975.8 |###
  39052.0 |##############
  39128.2 |
  39204.4 |###
  39280.5 |#######
  39356.7 |#######
  39432.9 |###
  39509.1 |###
  39585.3 |###
  39661.4 |
  39737.6 |
  (3 below, 4 above range)

```

## Diagnostics

- **satfold-gate-true**: autocorrelation=0.84 (measurement drift or warm-up artifact)
- **satfold-lanes16**: autocorrelation=0.80 (measurement drift or warm-up artifact)
- **satfold-lanes16-3**: autocorrelation=0.85 (measurement drift or warm-up artifact)
- **satfold-seq**: autocorrelation=0.70 (measurement drift or warm-up artifact)
