# Clamping fold at 32 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-accfit-dyn is an outlier: 5.0x slower than the field

warm-clamp-accfit-dyn (2.76 us) is 5.0x the fastest (550 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-accfit shows warm-up / thermal drift (autocorr +0.89)

warm-clamp-accfit's per-pass series has lag-1 autocorrelation +0.89, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (warm-clamp-acc64)

The baseline warm-clamp-acc64 is the fastest (550 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {warm-clamp-acc64, warm-clamp-accfit, warm-clamp-head} vs {warm-clamp-min-lanes, warm-clamp-minimum, warm-clamp-accfit-dyn} (195% apart)

The field splits into a fast tier {warm-clamp-acc64, warm-clamp-accfit, warm-clamp-head} and a slow tier {warm-clamp-min-lanes, warm-clamp-minimum, warm-clamp-accfit-dyn} with a 195% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.0x the fastest

Fastest warm-clamp-acc64 (550 ns) to slowest warm-clamp-accfit-dyn (2.76 us): 5.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-clamp-accfit's edge over baseline is significant but tiny (7 ns, 1.32%)

warm-clamp-accfit differs from baseline warm-clamp-acc64 by 7 ns (1.32%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (warm-clamp-acc64) is the fastest** at 549.8 ns median
- 4 variants significantly slower than baseline
- Spread: 5.02x (fastest 549.8 ns, slowest 2762.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 625ns | 612ns | 605ns | 618ns | 666ns | base |
| warm-clamp-accfit | 634ns | 630ns | 607ns | 628ns | 681ns | +1.48% |
| warm-clamp-accfit-dyn | 2884ns | 2827ns | 2676ns | 2824ns | 3271ns | +361.45% |
| warm-clamp-head | 704ns | 690ns | 680ns | 694ns | 758ns | +12.62% |
| warm-clamp-min-lanes | 1928ns | 1916ns | 1870ns | 1928ns | 1989ns | +208.52% |
| warm-clamp-minimum | 2031ns | 1974ns | 1875ns | 2003ns | 2271ns | +224.92% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 560ns | 541ns | 597ns | base | 14.623 |
| warm-clamp-accfit | 571ns | 548ns | 615ns | +1.88% | 14.352 |
| warm-clamp-accfit-dyn | 2815ns | 2614ns | 3189ns | +402.49% | 2.910 |
| warm-clamp-head | 640ns | 618ns | 690ns | +14.28% | 12.796 |
| warm-clamp-min-lanes | 1863ns | 1807ns | 1921ns | +232.61% | 4.396 |
| warm-clamp-minimum | 1962ns | 1813ns | 2195ns | +250.13% | 4.176 |

## Performance model

- Peak throughput: **15.148 Gops/s** (warm-clamp-acc64; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 14.900 | 98.4% |
| warm-clamp-accfit | 14.489 | 95.6% |
| warm-clamp-accfit-dyn | 2.966 | 19.6% |
| warm-clamp-head | 13.033 | 86.0% |
| warm-clamp-min-lanes | 4.424 | 29.2% |
| warm-clamp-minimum | 4.298 | 28.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 625ns | 625ns | base |
| warm-clamp-accfit | 634ns | 634ns | +1.48% |
| warm-clamp-accfit-dyn | 2884ns | 2884ns | +361.45% |
| warm-clamp-head | 704ns | 704ns | +12.62% |
| warm-clamp-min-lanes | 1928ns | 1928ns | +208.52% |
| warm-clamp-minimum | 2031ns | 2031ns | +224.92% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 550ns | base | --- | [547, 566] | --- | --- | --- | --- |
| warm-clamp-accfit | 565ns | no significant difference | [-1, +7]ns | [549, 568] | no | 0.3368 | 0.3368 | 1 |
| warm-clamp-accfit-dyn | 2762ns | +2190.2ns (+398.4%) | [+2144, +2205]ns | [2710, 2775] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 629ns | +80.4ns (+14.6%) | [+78, +82]ns | [622, 632] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 1852ns | +1289.2ns (+234.5%) | [+1278, +1333]ns | [1835, 1900] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 1906ns | +1354.3ns (+246.3%) | [+1336, +1358]ns | [1903, 1921] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 551ns | +2.2% | +374.0% | +12.3% | +228.1% | +297.8% |
| 2 | 565ns | -2.1% | +363.0% | +9.9% | +221.1% | +289.3% |
| 3 | 548ns | +0.2% | +389.2% | +13.5% | +230.0% | +299.6% |
| 4 | 550ns | -0.4% | +391.7% | +13.4% | +228.8% | +299.0% |
| 5 | 541ns | +1.4% | +383.7% | +15.1% | +233.8% | +304.9% |
| 6 | 540ns | +1.4% | +384.8% | +15.3% | +234.9% | +306.6% |
| 7 | 541ns | +1.4% | +382.8% | +14.9% | +233.9% | +306.7% |
| 8 | 547ns | +0.1% | +377.6% | +28.2% | +233.4% | +300.8% |
| 9 | 539ns | +1.5% | +384.6% | +39.4% | +242.2% | +306.6% |
| 10 | 540ns | +1.7% | +383.8% | +38.9% | +234.1% | +297.2% |
| 11 | 551ns | +3.0% | +476.4% | +14.2% | +234.1% | +246.5% |
| 12 | 549ns | +0.9% | +480.7% | +14.6% | +234.6% | +246.7% |
| 13 | 545ns | +0.5% | +481.4% | +15.7% | +236.4% | +249.1% |
| 14 | 548ns | +0.2% | +479.4% | +15.1% | +235.2% | +248.0% |
| 15 | 547ns | +1.3% | +480.0% | +14.8% | +235.0% | +247.9% |
| 16 | 549ns | +0.0% | +481.7% | +14.7% | +251.6% | +246.8% |
| 17 | 550ns | -0.1% | +433.4% | +15.2% | +245.0% | +245.8% |
| 18 | 551ns | -0.8% | +400.3% | +14.2% | +245.9% | +250.9% |
| 19 | 548ns | -0.1% | +532.9% | +14.5% | +246.1% | +247.7% |
| 20 | 547ns | +0.4% | +443.7% | +14.8% | +248.3% | +248.2% |
| 21 | 570ns | +0.1% | +385.0% | +14.9% | +233.3% | +234.7% |
| 22 | 575ns | -1.4% | +380.3% | +12.8% | +231.5% | +231.2% |
| 23 | 571ns | -0.9% | +386.2% | +21.6% | +233.1% | +233.4% |
| 24 | 574ns | -0.9% | +380.4% | +12.5% | +232.3% | +232.6% |
| 25 | 569ns | -0.4% | +385.4% | +15.0% | +234.7% | +234.0% |
| 26 | 571ns | -0.9% | +383.6% | +14.4% | +233.9% | +233.5% |
| 27 | 573ns | -0.6% | +383.6% | +14.0% | +232.8% | +244.0% |
| 28 | 568ns | -0.1% | +386.7% | +15.3% | +235.3% | +243.7% |
| 29 | 573ns | -0.1% | +382.1% | +14.2% | +232.8% | +233.5% |
| 30 | 566ns | -0.1% | +411.6% | +14.9% | +239.0% | +236.8% |
| 31 | 542ns | +13.0% | +413.7% | +14.5% | +250.4% | +233.6% |
| 32 | 542ns | +13.0% | +391.9% | +14.1% | +242.8% | +233.7% |
| 33 | 642ns | -4.8% | +332.6% | -3.7% | +185.2% | +194.3% |
| 34 | 656ns | -6.0% | +309.9% | -5.7% | +185.4% | +182.6% |
| 35 | 583ns | +4.8% | +355.5% | +6.3% | +215.2% | +232.3% |
| 36 | 555ns | +11.0% | +378.5% | +11.6% | +230.8% | +225.4% |
| 37 | 547ns | +13.2% | +421.6% | +12.6% | +262.4% | +230.9% |
| 38 | 602ns | +0.7% | +357.7% | +3.2% | +212.2% | +199.5% |
| 39 | 542ns | +13.5% | +401.3% | +14.1% | +234.1% | +233.2% |
| 40 | 542ns | +14.1% | +412.2% | +14.8% | +235.7% | +233.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.485 | moderate+ |
| warm-clamp-accfit | 0.887 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit-dyn | 0.604 | HIGH+ (drift/warm-up) |
| warm-clamp-head | 0.623 | HIGH+ (drift/warm-up) |
| warm-clamp-min-lanes | 0.621 | HIGH+ (drift/warm-up) |
| warm-clamp-minimum | 0.878 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-clamp-accfit**: won 14/40, lost 22/40
- **warm-clamp-accfit-dyn**: won 0/40, lost 40/40
- **warm-clamp-head**: won 2/40, lost 38/40
- **warm-clamp-min-lanes**: won 0/40, lost 40/40
- **warm-clamp-minimum**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 2.7ns | 560.2ns | 0.5% |  |
| warm-clamp-accfit | 2.7ns | 570.8ns | 0.5% |  |
| warm-clamp-accfit-dyn | 2.8ns | 2815.1ns | 0.1% |  |
| warm-clamp-head | 2.3ns | 640.2ns | 0.4% |  |
| warm-clamp-min-lanes | 2.5ns | 1863.3ns | 0.1% |  |
| warm-clamp-minimum | 2.8ns | 1961.5ns | 0.1% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 540.8-597.3 ns)
    540.8 |##########################
    543.6 |####
    546.4 |########################################
    549.3 |######################
    552.1 |
    554.9 |####
    557.7 |
    560.6 |
    563.4 |########
    566.2 |####
    569.0 |#################
    571.9 |#############
    574.7 |####
    577.5 |
    580.3 |####
    583.2 |
    586.0 |
    588.8 |
    591.6 |
    594.5 |
  (3 below, 3 above range)

warm-clamp-accfit (n=40, range 547.6-615.0 ns)
    547.6 |########################################
    551.0 |##########
    554.3 |
    557.7 |
    561.1 |###
    564.4 |#######################
    567.8 |##########
    571.2 |###
    574.6 |
    577.9 |
    581.3 |
    584.7 |
    588.0 |
    591.4 |
    594.8 |
    598.1 |
    601.5 |
    604.9 |###
    608.3 |######
    611.6 |##########
  (3 below, 4 above range)

warm-clamp-accfit-dyn (n=40, range 2614.2-3189.1 ns)
   2614.2 |##########
   2643.0 |##########
   2671.7 |######
   2700.5 |######
   2729.2 |######
   2758.0 |########################################
   2786.7 |
   2815.4 |
   2844.2 |###
   2872.9 |###
   2901.7 |
   2930.4 |###
   2959.1 |###
   2987.9 |
   3016.6 |
   3045.4 |
   3074.1 |
   3102.8 |
   3131.6 |
   3160.3 |################
  (5 below, 2 above range)

warm-clamp-head (n=40, range 618.4-689.6 ns)
    618.4 |########################################
    621.9 |#########################
    625.5 |####################
    629.1 |#########################
    632.6 |#####
    636.2 |
    639.7 |
    643.3 |#####
    646.9 |#####
    650.4 |###############
    654.0 |####################
    657.5 |
    661.1 |
    664.7 |
    668.2 |
    671.8 |
    675.3 |
    678.9 |
    682.5 |
    686.0 |
  (4 below, 4 above range)

warm-clamp-min-lanes (n=40, range 1807.4-1921.0 ns)
   1807.4 |####################
   1813.1 |##########
   1818.7 |#####
   1824.4 |#####
   1830.1 |###############
   1835.8 |####################
   1841.5 |#####
   1847.1 |
   1852.8 |#####
   1858.5 |
   1864.2 |
   1869.8 |#####
   1875.5 |#####
   1881.2 |
   1886.9 |
   1892.6 |##########
   1898.2 |####################
   1903.9 |########################################
   1909.6 |
   1915.3 |#####
  (4 below, 2 above range)

warm-clamp-minimum (n=40, range 1812.8-2194.5 ns)
   1812.8 |
   1831.9 |
   1850.9 |###
   1870.0 |###
   1889.1 |########################################
   1908.2 |############
   1927.3 |######
   1946.4 |###
   1965.5 |###
   1984.6 |
   2003.7 |
   2022.7 |
   2041.8 |
   2060.9 |
   2080.0 |
   2099.1 |
   2118.2 |
   2137.3 |###
   2156.4 |
   2175.4 |##################
  (7 below, 3 above range)

```

## Diagnostics

- **warm-clamp-accfit**: autocorrelation=0.89 (measurement drift or warm-up artifact)
- **warm-clamp-accfit-dyn**: autocorrelation=0.60 (measurement drift or warm-up artifact)
- **warm-clamp-head**: autocorrelation=0.62 (measurement drift or warm-up artifact)
- **warm-clamp-min-lanes**: autocorrelation=0.62 (measurement drift or warm-up artifact)
- **warm-clamp-minimum**: autocorrelation=0.88 (measurement drift or warm-up artifact)
