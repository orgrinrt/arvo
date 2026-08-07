# Clamping fold at 32 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-minimum is an outlier: 9.6x slower than the field

warm-clamp-minimum (3.74 us) is 9.6x the fastest (390 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-acc64 shows warm-up / thermal drift (autocorr +0.90)

warm-clamp-acc64's per-pass series has lag-1 autocorrelation +0.90, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-accfit, warm-clamp-acc64, warm-clamp-accfit-dyn, warm-clamp-head} vs {warm-clamp-min-lanes, warm-clamp-minimum} (265% apart)

The field splits into a fast tier {warm-clamp-accfit, warm-clamp-acc64, warm-clamp-accfit-dyn, warm-clamp-head} and a slow tier {warm-clamp-min-lanes, warm-clamp-minimum} with a 265% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 9.6x the fastest

Fastest warm-clamp-accfit (390 ns) to slowest warm-clamp-minimum (3.74 us): 9.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-clamp-accfit's edge over baseline is significant but tiny (-1 ns, 0.20%)

warm-clamp-accfit differs from baseline warm-clamp-acc64 by -1 ns (0.20%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-clamp-accfit** at 390.2 ns median (-1.9% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 9.57x (fastest 390.2 ns, slowest 3735.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 476ns | 461ns | 443ns | 467ns | 535ns | base |
| warm-clamp-accfit | 454ns | 454ns | 442ns | 452ns | 471ns | -4.59% |
| warm-clamp-accfit-dyn | 526ns | 525ns | 517ns | 524ns | 540ns | +10.57% |
| warm-clamp-head | 601ns | 552ns | 534ns | 591ns | 697ns | +26.27% |
| warm-clamp-min-lanes | 1865ns | 1852ns | 1808ns | 1863ns | 1926ns | +291.91% |
| warm-clamp-minimum | 3812ns | 3798ns | 3734ns | 3782ns | 3984ns | +701.25% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 410ns | 380ns | 462ns | base | 19.974 |
| warm-clamp-accfit | 393ns | 382ns | 408ns | -4.29% | 20.870 |
| warm-clamp-accfit-dyn | 464ns | 457ns | 476ns | +13.14% | 17.655 |
| warm-clamp-head | 532ns | 472ns | 617ns | +29.63% | 15.408 |
| warm-clamp-min-lanes | 1799ns | 1745ns | 1856ns | +338.62% | 4.554 |
| warm-clamp-minimum | 3747ns | 3670ns | 3917ns | +813.62% | 2.186 |

## Performance model

- Peak throughput: **21.549 Gops/s** (warm-clamp-acc64; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 20.586 | 95.5% |
| warm-clamp-accfit | 20.994 | 97.4% |
| warm-clamp-accfit-dyn | 17.705 | 82.2% |
| warm-clamp-head | 16.732 | 77.6% |
| warm-clamp-min-lanes | 4.578 | 21.2% |
| warm-clamp-minimum | 2.193 | 10.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 476ns | 476ns | base |
| warm-clamp-accfit | 454ns | 454ns | -4.59% |
| warm-clamp-accfit-dyn | 526ns | 526ns | +10.57% |
| warm-clamp-head | 601ns | 601ns | +26.27% |
| warm-clamp-min-lanes | 1865ns | 1865ns | +291.91% |
| warm-clamp-minimum | 3812ns | 3812ns | +701.25% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 398ns | base | --- | [391, 407] | --- | --- | --- | --- |
| warm-clamp-accfit | 390ns | no significant difference | [-12, +1]ns | [388, 394] | no | 0.1081 | 0.1081 | 1 |
| warm-clamp-accfit-dyn | 463ns | +72.9ns (+18.3%) | [+60, +76]ns | [459, 465] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 490ns | +89.6ns (+22.5%) | [+85, +156]ns | [479, 598] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 1789ns | +1402.9ns (+352.5%) | [+1345, +1422]ns | [1778, 1818] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 3735ns | +3342.5ns (+839.9%) | [+3270, +3354]ns | [3674, 3739] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 377ns | +1.9% | +21.8% | +24.2% | +374.3% | +892.8% |
| 2 | 380ns | +1.6% | +20.8% | +54.8% | +370.4% | +885.8% |
| 3 | 383ns | -0.3% | +19.5% | +59.9% | +366.9% | +885.1% |
| 4 | 380ns | +0.9% | +20.7% | +60.3% | +370.8% | +902.9% |
| 5 | 383ns | +1.1% | +20.9% | +60.5% | +367.4% | +878.1% |
| 6 | 386ns | +0.5% | +19.8% | +58.8% | +362.4% | +870.1% |
| 7 | 385ns | +20.9% | +19.0% | +59.7% | +371.3% | +870.7% |
| 8 | 383ns | +2.5% | +19.3% | +60.3% | +369.2% | +877.1% |
| 9 | 375ns | +1.9% | +21.7% | +63.5% | +388.4% | +897.1% |
| 10 | 380ns | +2.0% | +20.1% | +62.2% | +398.5% | +884.3% |
| 11 | 386ns | +2.5% | +21.2% | +23.7% | +381.4% | +867.7% |
| 12 | 394ns | -0.8% | +19.3% | +22.4% | +371.5% | +849.2% |
| 13 | 389ns | -0.2% | +19.7% | +22.6% | +379.7% | +860.3% |
| 14 | 395ns | -0.6% | +18.5% | +22.1% | +367.5% | +847.2% |
| 15 | 397ns | -0.9% | +17.1% | +20.9% | +364.0% | +841.3% |
| 16 | 395ns | +0.0% | +17.6% | +19.6% | +364.6% | +843.1% |
| 17 | 394ns | +2.3% | +18.3% | +20.9% | +367.3% | +848.3% |
| 18 | 394ns | +1.4% | +18.5% | +21.7% | +367.5% | +848.9% |
| 19 | 383ns | +1.4% | +22.0% | +25.0% | +379.2% | +874.7% |
| 20 | 388ns | +1.4% | +19.8% | +22.3% | +374.2% | +861.9% |
| 21 | 399ns | -1.0% | +21.6% | +19.8% | +336.4% | +842.2% |
| 22 | 403ns | -2.0% | +19.6% | +17.7% | +338.9% | +811.1% |
| 23 | 400ns | -3.2% | +20.1% | +21.2% | +341.8% | +817.9% |
| 24 | 404ns | -2.2% | +18.7% | +18.1% | +355.7% | +866.2% |
| 25 | 419ns | -4.3% | +14.5% | +13.7% | +316.5% | +776.0% |
| 26 | 412ns | -3.8% | +12.5% | +16.2% | +322.8% | +790.7% |
| 27 | 421ns | -3.9% | +9.8% | +17.5% | +314.0% | +772.6% |
| 28 | 408ns | -2.8% | +13.5% | +22.3% | +326.7% | +799.7% |
| 29 | 400ns | -2.2% | +15.8% | +24.6% | +334.5% | +818.1% |
| 30 | 406ns | -2.2% | +14.5% | +21.8% | +329.6% | +804.4% |
| 31 | 459ns | -17.1% | +0.1% | +34.8% | +285.5% | +700.1% |
| 32 | 461ns | -16.3% | -0.7% | +34.1% | +294.6% | +697.0% |
| 33 | 456ns | -17.3% | +0.3% | +35.1% | +290.1% | +704.4% |
| 34 | 458ns | -15.6% | -0.1% | +35.1% | +285.7% | +702.4% |
| 35 | 462ns | -16.2% | -0.8% | +33.7% | +283.8% | +694.4% |
| 36 | 460ns | -15.6% | -0.5% | +33.9% | +292.8% | +697.0% |
| 37 | 470ns | -15.6% | -2.2% | +16.4% | +278.6% | +681.8% |
| 38 | 464ns | -16.2% | -1.4% | +3.6% | +281.3% | +692.0% |
| 39 | 455ns | -16.0% | +0.5% | +2.2% | +298.4% | +808.6% |
| 40 | 461ns | -15.4% | +0.1% | +1.3% | +286.0% | +866.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.896 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit | 0.077 | ok |
| warm-clamp-accfit-dyn | 0.793 | HIGH+ (drift/warm-up) |
| warm-clamp-head | 0.796 | HIGH+ (drift/warm-up) |
| warm-clamp-min-lanes | 0.655 | HIGH+ (drift/warm-up) |
| warm-clamp-minimum | 0.393 | moderate+ |

**Consistency summary:**

- **warm-clamp-accfit**: won 25/40, lost 14/40
- **warm-clamp-accfit-dyn**: won 5/40, lost 32/40
- **warm-clamp-head**: won 0/40, lost 40/40
- **warm-clamp-min-lanes**: won 0/40, lost 40/40
- **warm-clamp-minimum**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 3.0ns | 410.1ns | 0.7% |  |
| warm-clamp-accfit | 2.7ns | 392.5ns | 0.7% |  |
| warm-clamp-accfit-dyn | 3.1ns | 464.0ns | 0.7% |  |
| warm-clamp-head | 3.0ns | 531.7ns | 0.6% |  |
| warm-clamp-min-lanes | 2.8ns | 1798.9ns | 0.2% |  |
| warm-clamp-minimum | 2.6ns | 3747.1ns | 0.1% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 380.2-461.8 ns)
    380.2 |########################################
    384.2 |################################
    388.3 |########
    392.4 |########################################
    396.5 |################################
    400.6 |################
    404.6 |################
    408.7 |########
    412.8 |
    416.9 |################
    421.0 |
    425.0 |
    429.1 |
    433.2 |
    437.3 |
    441.4 |
    445.4 |
    449.5 |
    453.6 |########################
    457.7 |########################################
  (4 below, 2 above range)

warm-clamp-accfit (n=40, range 382.3-407.9 ns)
    382.3 |##############################
    383.6 |##########
    384.8 |####################
    386.1 |########################################
    387.4 |##############################
    388.7 |##############################
    390.0 |##############################
    391.2 |####################
    392.5 |####################
    393.8 |####################
    395.1 |########################################
    396.4 |##############################
    397.6 |
    398.9 |##########
    400.2 |##########
    401.5 |
    402.8 |##########
    404.0 |##########
    405.3 |
    406.6 |
  (3 below, 1 above range)

warm-clamp-accfit-dyn (n=40, range 457.1-476.4 ns)
    457.1 |########################################
    458.1 |########################################
    459.1 |################
    460.0 |
    461.0 |########
    462.0 |########################
    462.9 |################
    463.9 |################
    464.8 |########################
    465.8 |########################
    466.8 |########################
    467.7 |
    468.7 |########
    469.6 |
    470.6 |
    471.6 |
    472.5 |
    473.5 |
    474.4 |
    475.4 |
  (5 below, 5 above range)

warm-clamp-head (n=40, range 471.8-617.3 ns)
    471.8 |########################################
    479.0 |###################################
    486.3 |
    493.6 |#################
    500.9 |
    508.2 |
    515.4 |
    522.7 |
    530.0 |
    537.3 |
    544.5 |####
    551.8 |
    559.1 |
    566.4 |
    573.7 |
    580.9 |####
    588.2 |
    595.5 |
    602.8 |####
    610.1 |########################################
  (3 below, 4 above range)

warm-clamp-min-lanes (n=40, range 1744.9-1855.9 ns)
   1744.9 |
   1750.4 |
   1756.0 |
   1761.5 |######
   1767.1 |#################################
   1772.6 |######
   1778.2 |#############
   1783.7 |##########################
   1789.3 |#############
   1794.8 |######
   1800.4 |
   1805.9 |######
   1811.5 |#############
   1817.0 |######
   1822.6 |
   1828.1 |######
   1833.7 |#############
   1839.2 |########################################
   1844.8 |
   1850.3 |
  (7 below, 4 above range)

warm-clamp-minimum (n=40, range 3669.9-3916.8 ns)
   3669.9 |##################################
   3682.2 |
   3694.6 |
   3706.9 |
   3719.3 |##
   3731.6 |########################################
   3743.9 |########
   3756.3 |##
   3768.6 |##
   3781.0 |
   3793.3 |
   3805.7 |##
   3818.0 |
   3830.4 |
   3842.7 |
   3855.0 |
   3867.4 |
   3879.7 |
   3892.1 |
   3904.4 |##
  (4 below, 2 above range)

```

## Diagnostics

- **warm-clamp-acc64**: autocorrelation=0.90 (measurement drift or warm-up artifact)
- **warm-clamp-accfit-dyn**: autocorrelation=0.79 (measurement drift or warm-up artifact)
- **warm-clamp-head**: autocorrelation=0.80 (measurement drift or warm-up artifact)
- **warm-clamp-min-lanes**: autocorrelation=0.65 (measurement drift or warm-up artifact)
