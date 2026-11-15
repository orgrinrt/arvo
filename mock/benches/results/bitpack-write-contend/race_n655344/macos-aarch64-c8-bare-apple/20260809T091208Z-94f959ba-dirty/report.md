# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary deliberately misaligned

2 variants, 40 samples per variant.
Baseline: **bitpack-write-dense**

## Highlights

Baseline for all deltas below: **bitpack-write-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-write-dense dominates: 1655% faster than the next best (bitpack-write-guarded)

bitpack-write-dense (8.34 us) leads bitpack-write-guarded (146.41 us) by 1655%, a clear separation rather than a photo finish. CV 38.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-dense shows warm-up / thermal drift (autocorr +0.70)

bitpack-write-dense's per-pass series has lag-1 autocorrelation +0.70, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (bitpack-write-dense)

The baseline bitpack-write-dense is the fastest (8.34 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 17.6x the fastest

Fastest bitpack-write-dense (8.34 us) to slowest bitpack-write-guarded (146.41 us): 17.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-write-guarded is inconsistent: worst-20% is 2.3x its best-20%

bitpack-write-guarded's best 20% of batches run at 126.20 us but its worst 20% at 289.62 us (2.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (bitpack-write-dense) is the fastest** at 8341.0 ns median
- 1 variant significantly slower than baseline
- Spread: 17.55x (fastest 8341.0 ns, slowest 146409.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-write-dense | 10032ns | 8444ns | 7580ns | 8951ns | 15726ns | base |
| bitpack-write-guarded | 179070ns | 146657ns | 126421ns | 159523ns | 290362ns | +1685.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-dense | 9900ns | 7483ns | 15506ns | base | 6.619 |
| bitpack-write-guarded | 178706ns | 126204ns | 289617ns | +1705.06% | 0.367 |

## Performance model

- Peak throughput: **8.757 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 65534

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-dense | 7.857 | 89.7% |
| bitpack-write-guarded | 0.448 | 5.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-dense | 10032ns | 10032ns | base |
| bitpack-write-guarded | 179070ns | 179070ns | +1685.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-dense | 8341ns | base | --- | [8037, 8751] | --- | --- | --- | --- |
| bitpack-write-guarded | 146410ns | +138160.6ns (+1656.4%) | [+128452, +160501]ns | [136458, 172600] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-write-dense | bitpack-write-guarded |
|---|---|---|
| 1 | 14744ns | +763.2% |
| 2 | 9165ns | +1692.4% |
| 3 | 8463ns | +1428.3% |
| 4 | 8037ns | +1608.4% |
| 5 | 8365ns | +2542.9% |
| 6 | 8515ns | +1450.1% |
| 7 | 8935ns | +1361.7% |
| 8 | 8745ns | +1653.9% |
| 9 | 8318ns | +1438.3% |
| 10 | 8685ns | +1543.2% |
| 11 | 8206ns | +1454.8% |
| 12 | 8051ns | +1542.9% |
| 13 | 8188ns | +1528.2% |
| 14 | 8007ns | +1604.3% |
| 15 | 8006ns | +1604.5% |
| 16 | 8011ns | +1675.9% |
| 17 | 8037ns | +1986.5% |
| 18 | 8243ns | +1721.0% |
| 19 | 8758ns | +1474.3% |
| 20 | 8625ns | +1837.4% |
| 21 | 7422ns | +1500.4% |
| 22 | 7864ns | +1464.6% |
| 23 | 7422ns | +1627.4% |
| 24 | 7421ns | +1617.3% |
| 25 | 7513ns | +1636.9% |
| 26 | 7705ns | +2016.3% |
| 27 | 7533ns | +2579.9% |
| 28 | 7488ns | +1795.8% |
| 29 | 7415ns | +6058.8% |
| 30 | 7652ns | +2765.2% |
| 31 | 16162ns | +998.3% |
| 32 | 13216ns | +1697.0% |
| 33 | 13688ns | +1454.8% |
| 34 | 11932ns | +1926.5% |
| 35 | 12020ns | +1561.8% |
| 36 | 14645ns | +1341.8% |
| 37 | 13345ns | +1845.9% |
| 38 | 14295ns | +1654.2% |
| 39 | 19312ns | +1777.2% |
| 40 | 17857ns | +1507.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-dense | 0.699 | HIGH+ (drift/warm-up) |
| bitpack-write-guarded | 0.413 | moderate+ |

**Consistency summary:**

- **bitpack-write-guarded**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-dense | 5.0ns | 9900.3ns | 0.1% |  |
| bitpack-write-guarded | 41.0ns | 178706.1ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-dense (n=40, range 7483.3-15506.0 ns)
   7483.3 |##########################
   7884.4 |########################################
   8285.5 |##########################
   8686.7 |#############
   9087.8 |####
   9489.0 |
   9890.1 |
  10291.2 |
  10692.4 |
  11093.5 |
  11494.6 |
  11895.8 |########
  12296.9 |
  12698.0 |
  13099.2 |########
  13500.3 |####
  13901.5 |####
  14302.6 |####
  14703.7 |####
  15104.9 |
  (4 below, 3 above range)

bitpack-write-guarded (n=40, range 126203.6-289617.4 ns)
  126203.6 |########################################
  134374.3 |#####################
  142545.0 |#######
  150715.6 |###
  158886.3 |#######
  167057.0 |#######
  175227.7 |###
  183398.4 |
  191569.1 |
  199739.8 |#######
  207910.5 |#######
  216081.2 |#######
  224251.9 |
  232422.6 |###
  240593.2 |###
  248763.9 |###
  256934.6 |###
  265105.3 |
  273276.0 |
  281446.7 |###
  (2 below, 2 above range)

```

## Diagnostics

- **bitpack-write-dense**: CV=32.1% (high variance, measurements may be unstable)
- **bitpack-write-dense**: autocorrelation=0.70 (measurement drift or warm-up artifact)
- **bitpack-write-guarded**: CV=39.1% (high variance, measurements may be unstable)
