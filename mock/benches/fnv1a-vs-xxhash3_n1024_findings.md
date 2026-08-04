# ContentHash algorithms: FNV1a vs xxHash3

2 variants, 40 samples per variant.
Baseline: **fnv1a**

## Highlights

Baseline for all deltas below: **fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline fnv1a has the worst median (1.50 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest xxhash3 at 488 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### xxhash3 dominates: 208% faster than the next best (fnv1a)

xxhash3 (488 ns) leads fnv1a (1.50 us) by 208%, a clear separation rather than a photo finish. CV 6.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### xxhash3 beats baseline by 67% (significant)

xxhash3 is -1.01 us (67%) faster than baseline fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### xxhash3 is fastest but the noisiest (CV 6.7%)

xxhash3 wins on median (488 ns) yet has the highest variance (CV 6.7%), while fnv1a is the steadiest (CV 5.7%, 1.50 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### fnv1a shows warm-up / thermal drift (autocorr +0.87)

fnv1a's per-pass series has lag-1 autocorrelation +0.87, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.1x the fastest

Fastest xxhash3 (488 ns) to slowest fnv1a (1.50 us): 3.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: xxhash3** at 487.7 ns median (-67.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 3.08x (fastest 487.7 ns, slowest 1504.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fnv1a | 1553ns | 1618ns | 1404ns | 1579ns | 1626ns | base |
| xxhash3 | 552ns | 561ns | 523ns | 550ns | 588ns | -64.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fnv1a | 1443ns | 1305ns | 1510ns | base | 0.709 |
| xxhash3 | 481ns | 456ns | 515ns | -66.65% | 2.127 |

## Performance model

- Peak throughput: **2.245 Gops/s** (xxhash3; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fnv1a | 0.681 | 30.3% |
| xxhash3 | 2.100 | 93.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fnv1a | 1553ns | 1553ns | base |
| xxhash3 | 552ns | 552ns | -64.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fnv1a | 1504ns | base | --- | [1409, 1506] | --- | --- | --- | --- |
| xxhash3 | 488ns | -1014.2ns (-67.4%) | [-1018, -918]ns | [458, 490] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | fnv1a | xxhash3 |
|---|---|---|
| 1 | 1505ns | -67.4% |
| 2 | 1507ns | -67.5% |
| 3 | 1505ns | -67.6% |
| 4 | 1507ns | -67.4% |
| 5 | 1508ns | -67.5% |
| 6 | 1505ns | -67.6% |
| 7 | 1503ns | -67.4% |
| 8 | 1503ns | -67.3% |
| 9 | 1505ns | -67.4% |
| 10 | 1504ns | -67.3% |
| 11 | 1506ns | -69.7% |
| 12 | 1504ns | -69.6% |
| 13 | 1507ns | -69.6% |
| 14 | 1507ns | -69.8% |
| 15 | 1509ns | -69.7% |
| 16 | 1507ns | -69.6% |
| 17 | 1509ns | -69.9% |
| 18 | 1512ns | -69.7% |
| 19 | 1506ns | -69.6% |
| 20 | 1506ns | -69.7% |
| 21 | 1407ns | -65.3% |
| 22 | 1408ns | -64.0% |
| 23 | 1409ns | -65.1% |
| 24 | 1409ns | -65.1% |
| 25 | 1407ns | -65.0% |
| 26 | 1475ns | -66.9% |
| 27 | 1506ns | -67.6% |
| 28 | 1519ns | -67.5% |
| 29 | 1506ns | -67.5% |
| 30 | 1507ns | -67.6% |
| 31 | 1308ns | -65.1% |
| 32 | 1305ns | -64.9% |
| 33 | 1309ns | -65.0% |
| 34 | 1303ns | -65.0% |
| 35 | 1305ns | -64.8% |
| 36 | 1305ns | -64.8% |
| 37 | 1308ns | -49.7% |
| 38 | 1306ns | -62.8% |
| 39 | 1309ns | -62.4% |
| 40 | 1304ns | -62.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fnv1a | 0.865 | HIGH+ (drift/warm-up) |
| xxhash3 | 0.141 | ok |

**Consistency summary:**

- **xxhash3**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fnv1a | 2.5ns | 1443.5ns | 0.2% |  |
| xxhash3 | 2.3ns | 481.4ns | 0.5% |  |

## Distribution (algo ns)

```
fnv1a (n=40, range 1305.3-1509.8 ns)
   1305.3 |#########
   1315.5 |
   1325.7 |
   1335.9 |
   1346.2 |
   1356.4 |
   1366.6 |
   1376.9 |
   1387.1 |
   1397.3 |#####
   1407.5 |###
   1417.8 |
   1428.0 |
   1438.2 |
   1448.4 |
   1458.7 |
   1468.9 |#
   1479.1 |
   1489.4 |
   1499.6 |########################################
  (5 below, 2 above range)

xxhash3 (n=40, range 456.1-514.9 ns)
    456.1 |########################################
    459.1 |###
    462.0 |
    464.9 |
    467.9 |
    470.8 |
    473.7 |
    476.7 |
    479.6 |
    482.6 |
    485.5 |##################
    488.4 |########################################
    491.4 |#####################
    494.3 |
    497.2 |
    500.2 |
    503.1 |
    506.0 |###
    509.0 |
    511.9 |
  (4 below, 1 above range)

```

## Diagnostics

- **fnv1a**: autocorrelation=0.87 (measurement drift or warm-up artifact)
