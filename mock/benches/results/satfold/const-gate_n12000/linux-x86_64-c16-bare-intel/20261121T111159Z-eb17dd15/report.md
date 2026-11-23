# Does a const gate erase in time: the licensed arm reached directly, the same arm reached through a const verdict computed by an exhaustive sweep in a const fn, and the same gate over a law that is false so it selects the fallback

5 variants, 40 samples per variant.
Baseline: **satfold-gate-false**

## Highlights

Baseline for all deltas below: **satfold-gate-false**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (satfold-gate-false) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline satfold-gate-false has the worst median (27.29 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest satfold-lanes16-3 at 1.78 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### satfold-lanes16-3 beats baseline by 93% (significant)

satfold-lanes16-3 is -25.45 us (93%) faster than baseline satfold-gate-false, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-gate-false is an outlier: 15.4x slower than the field

satfold-gate-false (27.29 us) is 15.4x the fastest (1.78 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {satfold-lanes16-3, satfold-lanes16, satfold-gate-true} vs {satfold-seq, satfold-gate-false} (1028% apart)

The field splits into a fast tier {satfold-lanes16-3, satfold-lanes16, satfold-gate-true} and a slow tier {satfold-seq, satfold-gate-false} with a 1028% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 15.4x the fastest

Fastest satfold-lanes16-3 (1.78 us) to slowest satfold-gate-false (27.29 us): 15.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-gate-true is inconsistent: worst-20% is 1.8x its best-20%

satfold-gate-true's best 20% of batches run at 1.88 us but its worst 20% at 3.34 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-lanes16-3** at 1775.8 ns median (-93.5% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 15.36x (fastest 1775.8 ns, slowest 27285.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Does a const gate erase in time: the licensed arm reached directly, the same arm reached through a const verdict computed by an exhaustive sweep in a const fn, and the same gate over a law that is false so it selects the fallback |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112526248 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-gate-false | 26133ns | 27356ns | 22379ns | 26943ns | 27458ns | base |
| satfold-gate-true | 2610ns | 2434ns | 2078ns | 2448ns | 3627ns | -90.01% |
| satfold-lanes16 | 2182ns | 2113ns | 1849ns | 2168ns | 2557ns | -91.65% |
| satfold-lanes16-3 | 2000ns | 2002ns | 1732ns | 1978ns | 2330ns | -92.35% |
| satfold-seq | 24902ns | 24886ns | 22373ns | 24870ns | 27527ns | -4.71% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-gate-false | 26059ns | 22319ns | 27365ns | base | 1.257 |
| satfold-gate-true | 2366ns | 1879ns | 3335ns | -90.92% | 13.850 |
| satfold-lanes16 | 1972ns | 1677ns | 2298ns | -92.43% | 16.614 |
| satfold-lanes16-3 | 1799ns | 1565ns | 2080ns | -93.10% | 18.214 |
| satfold-seq | 24837ns | 22319ns | 27446ns | -4.69% | 1.319 |

## Performance model

- Peak throughput: **20.933 Gops/s** (satfold-lanes16-3; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-gate-false | 1.201 | 5.7% |
| satfold-gate-true | 14.900 | 71.2% |
| satfold-lanes16 | 17.077 | 81.6% |
| satfold-lanes16-3 | 18.452 | 88.1% |
| satfold-seq | 1.321 | 6.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-gate-false | 26133ns | 26133ns | base |
| satfold-gate-true | 2610ns | 2610ns | -90.01% |
| satfold-lanes16 | 2182ns | 2182ns | -91.65% |
| satfold-lanes16-3 | 2000ns | 2000ns | -92.35% |
| satfold-seq | 24902ns | 24902ns | -4.71% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-gate-false | 27285ns | base | --- | [27276, 27287] | --- | --- | --- | --- |
| satfold-gate-true | 2199ns | -25022.7ns (-91.7%) | [-25082, -24745]ns | [2196, 2233] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16 | 1919ns | -25195.6ns (-92.3%) | [-25370, -24976]ns | [1889, 2090] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-3 | 1776ns | -25448.8ns (-93.3%) | [-25588, -25280]ns | [1703, 1847] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 24805ns | no significant difference | [-9, +6]ns | [22329, 27286] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-gate-false | satfold-gate-true | satfold-lanes16 | satfold-lanes16-3 | satfold-seq |
|---|---|---|---|---|---|---|
| 1 | 1 | 27288ns | -91.2% | -91.7% | -94.2% | -18.2% |
| 1 | 2 | 22319ns | -90.2% | -90.8% | -91.7% | +0.0% |
| 1 | 3 | 27285ns | -91.9% | -93.0% | -93.8% | +0.0% |
| 1 | 4 | 27288ns | -92.8% | -93.8% | -92.6% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-gate-false | -0.417 | moderate- |
| satfold-gate-true | -0.021 | ok |
| satfold-lanes16 | 0.191 | ok |
| satfold-lanes16-3 | -0.392 | moderate- |
| satfold-seq | 0.250 | moderate+ |

**Consistency summary:**

- **satfold-gate-true**: won 4/4, lost 0/4
- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-3**: won 4/4, lost 0/4
- **satfold-seq**: won 1/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-gate-false | 22.1ns | 26059.2ns | 0.1% |  |
| satfold-gate-true | 80.9ns | 2365.9ns | 3.4% |  |
| satfold-lanes16 | 71.0ns | 1972.3ns | 3.6% |  |
| satfold-lanes16-3 | 66.1ns | 1799.0ns | 3.7% |  |
| satfold-seq | 21.1ns | 24836.7ns | 0.1% |  |

## Distribution (algo ns)

```
satfold-gate-false (n=40, range 22318.9-27364.6 ns)
  22318.9 |#########
  22571.1 |
  22823.4 |
  23075.7 |
  23328.0 |
  23580.3 |
  23832.6 |
  24084.8 |
  24337.1 |
  24589.4 |
  24841.7 |
  25094.0 |
  25346.3 |
  25598.6 |
  25850.8 |
  26103.1 |
  26355.4 |
  26607.7 |
  26860.0 |
  27112.3 |########################################
  (3 below, 1 above range)

satfold-gate-true (n=40, range 1878.7-3335.2 ns)
   1878.7 |##
   1951.5 |########
   2024.4 |#####
   2097.2 |##
   2170.0 |########################################
   2242.8 |#############
   2315.7 |##
   2388.5 |##
   2461.3 |########
   2534.1 |##
   2607.0 |
   2679.8 |
   2752.6 |##
   2825.4 |##
   2898.3 |
   2971.1 |
   3043.9 |
   3116.7 |
   3189.6 |
   3262.4 |
  (4 below, 1 above range)

satfold-lanes16 (n=40, range 1677.4-2297.7 ns)
   1677.4 |#####
   1708.4 |#####
   1739.4 |###############
   1770.4 |##########
   1801.5 |##########
   1832.5 |#####
   1863.5 |
   1894.5 |###################################
   1925.5 |
   1956.5 |#####
   1987.5 |#####
   2018.5 |
   2049.6 |
   2080.6 |########################################
   2111.6 |
   2142.6 |#####
   2173.6 |#####
   2204.6 |#####
   2235.6 |
   2266.6 |
  (4 below, 6 above range)

satfold-lanes16-3 (n=40, range 1565.4-2079.8 ns)
   1565.4 |########################################
   1591.1 |######
   1616.8 |######
   1642.5 |######
   1668.2 |####################
   1694.0 |##########################
   1719.7 |######
   1745.4 |#############
   1771.1 |######
   1796.8 |
   1822.6 |########################################
   1848.3 |######
   1874.0 |######
   1899.7 |######
   1925.4 |######
   1951.2 |
   1976.9 |######
   2002.6 |####################
   2028.3 |######
   2054.0 |#############
  (1 below, 2 above range)

satfold-seq (n=40, range 22318.8-27445.5 ns)
  22318.8 |#################################
  22575.2 |
  22831.5 |
  23087.8 |
  23344.2 |
  23600.5 |
  23856.8 |
  24113.2 |
  24369.5 |
  24625.8 |
  24882.2 |
  25138.5 |
  25394.8 |
  25651.2 |
  25907.5 |
  26163.8 |
  26420.2 |
  26676.5 |
  26932.8 |
  27189.2 |########################################
  (5 below, 2 above range)

```

## Diagnostics

- **satfold-gate-true**: CV=42.8% (high variance, measurements may be unstable)
