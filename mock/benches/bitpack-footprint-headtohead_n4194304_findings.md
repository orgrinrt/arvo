# Bitpacked against Dense over one column, swept from L1 to past a 12 MB L2

4 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed-naive is an outlier: 6.5x slower than the field

bitpack-footprint-packed-naive (2.39 ms) is 6.5x the fastest (365.70 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (bitpack-footprint-dense-alt, bitpack-footprint-dense) are a dead heat (<1%)

bitpack-footprint-dense-alt (365.70 us) and bitpack-footprint-dense (366.40 us) differ by 0.19%, inside the noise, even though the wider field spreads 554.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### bitpack-footprint-dense shows warm-up / thermal drift (autocorr +0.54)

bitpack-footprint-dense's per-pass series has lag-1 autocorrelation +0.54, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-footprint-dense-alt, bitpack-footprint-dense, bitpack-footprint-packed} vs {bitpack-footprint-packed-naive} (346% apart)

The field splits into a fast tier {bitpack-footprint-dense-alt, bitpack-footprint-dense, bitpack-footprint-packed} and a slow tier {bitpack-footprint-packed-naive} with a 346% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 6.5x the fastest

Fastest bitpack-footprint-dense-alt (365.70 us) to slowest bitpack-footprint-packed-naive (2.39 ms): 6.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-footprint-dense-alt** at 365704.8 ns median (-0.2% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 6.55x (fastest 365704.8 ns, slowest 2393924.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 366834ns | 366895ns | 362340ns | 366429ns | 372542ns | base |
| bitpack-footprint-dense-alt | 367791ns | 366458ns | 363725ns | 367099ns | 373930ns | +0.26% |
| bitpack-footprint-packed | 539063ns | 537301ns | 534205ns | 537350ns | 549060ns | +46.95% |
| bitpack-footprint-packed-naive | 2401058ns | 2395272ns | 2387437ns | 2395943ns | 2430027ns | +554.54% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 366307ns | 361786ns | 372102ns | base | 11.450 |
| bitpack-footprint-dense-alt | 367203ns | 363134ns | 373385ns | +0.24% | 11.422 |
| bitpack-footprint-packed | 538446ns | 533635ns | 548545ns | +46.99% | 7.790 |
| bitpack-footprint-packed-naive | 2399640ns | 2385972ns | 2428855ns | +555.09% | 1.748 |

## Performance model

- Peak throughput: **11.593 Gops/s** (bitpack-footprint-dense; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 11.447 | 98.7% |
| bitpack-footprint-dense-alt | 11.469 | 98.9% |
| bitpack-footprint-packed | 7.814 | 67.4% |
| bitpack-footprint-packed-naive | 1.752 | 15.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 366834ns | 366834ns | base |
| bitpack-footprint-dense-alt | 367791ns | 367791ns | +0.26% |
| bitpack-footprint-packed | 539063ns | 539063ns | +46.95% |
| bitpack-footprint-packed-naive | 2401058ns | 2401058ns | +554.54% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 366400ns | base | --- | [363788, 367540] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 365705ns | no significant difference | [-1941, +2090]ns | [365107, 368057] | no | 0.8746 | 0.8746 | 0 |
| bitpack-footprint-packed | 536767ns | +170825.4ns (+46.6%) | [+167640, +175491]ns | [535570, 537604] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-footprint-packed-naive | 2393924ns | +2030610.7ns (+554.2%) | [+2026376, +2032884]ns | [2391235, 2397148] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|---|---|
| 1 | 366837ns | -0.8% | +45.3% | +553.0% |
| 2 | 369282ns | -0.1% | +44.9% | +556.2% |
| 3 | 369533ns | -0.5% | +44.3% | +566.9% |
| 4 | 365594ns | -0.0% | +45.8% | +558.2% |
| 5 | 368005ns | -1.2% | +45.3% | +552.4% |
| 6 | 368318ns | -0.9% | +44.8% | +550.5% |
| 7 | 367412ns | +3.8% | +45.0% | +561.6% |
| 8 | 367141ns | -1.0% | +45.7% | +550.6% |
| 9 | 366258ns | -0.8% | +47.9% | +553.2% |
| 10 | 368790ns | -1.4% | +45.2% | +547.0% |
| 11 | 364230ns | +1.5% | +47.4% | +559.3% |
| 12 | 365543ns | +1.5% | +46.6% | +553.3% |
| 13 | 362648ns | +1.5% | +48.6% | +558.8% |
| 14 | 362874ns | +1.5% | +48.2% | +560.6% |
| 15 | 362516ns | +2.4% | +52.5% | +558.7% |
| 16 | 364928ns | +3.2% | +46.8% | +556.9% |
| 17 | 363159ns | +1.4% | +47.4% | +560.4% |
| 18 | 363346ns | +2.5% | +47.8% | +559.0% |
| 19 | 364594ns | +1.5% | +46.9% | +562.7% |
| 20 | 368335ns | +0.4% | +45.4% | +562.5% |
| 21 | 361672ns | +0.2% | +48.8% | +571.1% |
| 22 | 361805ns | +0.3% | +48.6% | +561.9% |
| 23 | 361437ns | +0.3% | +48.8% | +562.1% |
| 24 | 361589ns | +0.4% | +48.6% | +562.3% |
| 25 | 372349ns | -0.4% | +47.8% | +541.9% |
| 26 | 363210ns | +0.4% | +53.2% | +558.4% |
| 27 | 362282ns | +0.9% | +51.2% | +560.6% |
| 28 | 361782ns | +2.1% | +49.2% | +560.6% |
| 29 | 362101ns | +1.4% | +49.1% | +567.0% |
| 30 | 361619ns | +0.8% | +48.6% | +561.8% |
| 31 | 367127ns | -0.6% | +45.7% | +550.6% |
| 32 | 366835ns | -0.5% | +48.0% | +551.8% |
| 33 | 366542ns | -0.5% | +45.8% | +549.7% |
| 34 | 374296ns | -2.0% | +43.0% | +539.2% |
| 35 | 367668ns | -0.7% | +46.1% | +552.2% |
| 36 | 370006ns | -1.2% | +50.6% | +544.2% |
| 37 | 372369ns | +0.4% | +44.2% | +540.6% |
| 38 | 372989ns | -1.7% | +43.8% | +547.2% |
| 39 | 373537ns | -2.1% | +43.4% | +539.4% |
| 40 | 371737ns | -1.6% | +44.4% | +542.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | 0.540 | HIGH+ (drift/warm-up) |
| bitpack-footprint-dense-alt | 0.117 | ok |
| bitpack-footprint-packed | 0.316 | moderate+ |
| bitpack-footprint-packed-naive | 0.339 | moderate+ |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 18/40, lost 21/40
- **bitpack-footprint-packed**: won 0/40, lost 40/40
- **bitpack-footprint-packed-naive**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 4.9ns | 366307.3ns | 0.0% |  |
| bitpack-footprint-dense-alt | 5.2ns | 367203.0ns | 0.0% |  |
| bitpack-footprint-packed | 4.7ns | 538446.4ns | 0.0% |  |
| bitpack-footprint-packed-naive | 20.9ns | 2399640.0ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 361786.0-372101.9 ns)
  361786.0 |########################################
  362301.8 |##########################
  362817.6 |########################################
  363333.4 |#############
  363849.2 |#############
  364365.0 |#############
  364880.8 |#############
  365396.6 |##########################
  365912.4 |#############
  366428.2 |########################################
  366944.0 |########################################
  367459.8 |#############
  367975.6 |########################################
  368491.3 |#############
  369007.1 |#############
  369522.9 |##########################
  370038.7 |
  370554.5 |
  371070.3 |
  371586.1 |#############
  (5 below, 5 above range)

bitpack-footprint-dense-alt (n=40, range 363133.5-373385.4 ns)
  363133.5 |################################
  363646.1 |########
  364158.7 |########################
  364671.3 |########################
  365183.9 |########################################
  365696.5 |########
  366209.1 |################
  366721.7 |
  367234.3 |################
  367746.9 |################
  368259.5 |########
  368772.1 |################
  369284.6 |################
  369797.2 |########
  370309.8 |########
  370822.4 |################
  371335.0 |
  371847.6 |########
  372360.2 |
  372872.8 |
  (4 below, 3 above range)

bitpack-footprint-packed (n=40, range 533635.0-548544.9 ns)
  533635.0 |
  534380.5 |####################
  535126.0 |########################################
  535871.5 |##########
  536617.0 |##############################
  537362.5 |#########################
  538108.0 |
  538853.5 |#####
  539599.0 |##########
  540344.4 |
  541089.9 |#####
  541835.4 |
  542580.9 |#####
  543326.4 |
  544071.9 |
  544817.4 |
  545562.9 |
  546308.4 |
  547053.9 |
  547799.4 |#####
  (5 below, 4 above range)

bitpack-footprint-packed-naive (n=40, range 2385972.3-2428854.6 ns)
  2385972.3 |##########################
  2388116.4 |########################################
  2390260.5 |####################
  2392404.6 |##########################
  2394548.8 |#################################
  2396692.9 |##########################
  2398837.0 |######
  2400981.1 |######
  2403125.2 |
  2405269.3 |######
  2407413.5 |
  2409557.6 |
  2411701.7 |
  2413845.8 |#############
  2415989.9 |######
  2418134.0 |
  2420278.2 |
  2422422.3 |######
  2424566.4 |
  2426710.5 |######
  (3 below, 3 above range)

```

## Diagnostics

- **bitpack-footprint-dense**: autocorrelation=0.54 (measurement drift or warm-up artifact)
