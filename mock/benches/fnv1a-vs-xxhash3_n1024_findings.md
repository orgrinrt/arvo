# ContentHash algorithms: FNV1a vs xxHash3

2 variants, 40 samples per variant.
Baseline: **fnv1a**

## Highlights

Baseline for all deltas below: **fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline fnv1a has the worst median (1.63 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest xxhash3 at 531 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### xxhash3 dominates: 208% faster than the next best (fnv1a)

xxhash3 (531 ns) leads fnv1a (1.63 us) by 208%, a clear separation rather than a photo finish. CV 59.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### xxhash3 beats baseline by 62% (significant)

xxhash3 is -1.02 us (62%) faster than baseline fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### xxhash3 is fastest but the noisiest (CV 59.8%)

xxhash3 wins on median (531 ns) yet has the highest variance (CV 59.8%), while fnv1a is the steadiest (CV 5.9%, 1.63 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### xxhash3 shows warm-up / thermal drift (autocorr +0.89)

xxhash3's per-pass series has lag-1 autocorrelation +0.89, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.1x the fastest

Fastest xxhash3 (531 ns) to slowest fnv1a (1.63 us): 3.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### xxhash3 is inconsistent: worst-20% is 2.5x its best-20%

xxhash3's best 20% of batches run at 489 ns but its worst 20% at 1.24 us (2.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: xxhash3** at 530.7 ns median (-67.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 3.08x (fastest 530.7 ns, slowest 1634.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fnv1a | 1743ns | 1758ns | 1619ns | 1733ns | 1897ns | base |
| xxhash3 | 791ns | 609ns | 562ns | 661ns | 1410ns | -54.61% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fnv1a | 1619ns | 1506ns | 1759ns | base | 0.632 |
| xxhash3 | 690ns | 489ns | 1243ns | -57.36% | 1.483 |

## Performance model

- Peak throughput: **2.095 Gops/s** (xxhash3; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fnv1a | 0.626 | 29.9% |
| xxhash3 | 1.930 | 92.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fnv1a | 1743ns | 1743ns | base |
| xxhash3 | 791ns | 791ns | -54.61% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fnv1a | 1635ns | base | --- | [1631, 1637] | --- | --- | --- | --- |
| xxhash3 | 531ns | -1021.2ns (-62.5%) | [-1103, -1015]ns | [493, 533] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | fnv1a | xxhash3 |
|---|---|---|
| 1 | 1638ns | -70.1% |
| 2 | 1635ns | -70.0% |
| 3 | 1526ns | -67.7% |
| 4 | 1508ns | -67.3% |
| 5 | 1505ns | -67.4% |
| 6 | 1510ns | -67.6% |
| 7 | 1505ns | -67.5% |
| 8 | 1506ns | -67.3% |
| 9 | 1509ns | -67.2% |
| 10 | 1508ns | -67.6% |
| 11 | 1636ns | -67.4% |
| 12 | 1758ns | -69.6% |
| 13 | 1635ns | -67.4% |
| 14 | 1638ns | -67.5% |
| 15 | 1632ns | -67.4% |
| 16 | 1638ns | -67.4% |
| 17 | 1639ns | -67.5% |
| 18 | 1638ns | -67.5% |
| 19 | 1688ns | -68.3% |
| 20 | 1637ns | -67.7% |
| 21 | 1637ns | -67.5% |
| 22 | 1828ns | -71.0% |
| 23 | 1634ns | -68.5% |
| 24 | 1700ns | -71.3% |
| 25 | 1632ns | -69.8% |
| 26 | 1529ns | -68.0% |
| 27 | 1504ns | -67.4% |
| 28 | 1505ns | -67.5% |
| 29 | 1505ns | -67.7% |
| 30 | 1511ns | -67.6% |
| 31 | 1637ns | -25.5% |
| 32 | 1634ns | -25.4% |
| 33 | 1931ns | -36.7% |
| 34 | 1635ns | -23.7% |
| 35 | 1631ns | -16.0% |
| 36 | 1633ns | -25.2% |
| 37 | 1691ns | -27.8% |
| 38 | 1662ns | -26.6% |
| 39 | 1812ns | -32.5% |
| 40 | 1633ns | -25.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fnv1a | 0.455 | moderate+ |
| xxhash3 | 0.889 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **xxhash3**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fnv1a | 3.1ns | 1619.3ns | 0.2% |  |
| xxhash3 | 4.5ns | 690.5ns | 0.7% |  |

## Distribution (algo ns)

```
fnv1a (n=40, range 1505.9-1758.8 ns)
   1505.9 |############
   1518.6 |#####
   1531.2 |
   1543.9 |
   1556.5 |
   1569.1 |
   1581.8 |
   1594.4 |
   1607.1 |
   1619.7 |#######
   1632.4 |########################################
   1645.0 |
   1657.6 |##
   1670.3 |
   1682.9 |#####
   1695.6 |##
   1708.2 |
   1720.9 |
   1733.5 |
   1746.1 |##
  (6 below, 3 above range)

xxhash3 (n=40, range 488.7-1242.7 ns)
    488.7 |########################################
    526.4 |############################
    564.1 |
    601.8 |
    639.5 |
    677.2 |
    714.9 |
    752.6 |
    790.3 |
    828.0 |
    865.7 |
    903.4 |
    941.1 |
    978.8 |
   1016.5 |
   1054.2 |
   1091.9 |
   1129.6 |
   1167.3 |
   1205.0 |##################
  (1 below, 2 above range)

```

## Diagnostics

- **xxhash3**: CV=45.9% (high variance, measurements may be unstable)
- **xxhash3**: autocorrelation=0.89 (measurement drift or warm-up artifact)
