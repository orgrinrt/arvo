# Clamping fold at 32 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-minimum is an outlier: 5.1x slower than the field

warm-clamp-minimum (2.13 us) is 5.1x the fastest (416 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-clamp-accfit, warm-clamp-acc64) are a dead heat (<1%)

warm-clamp-accfit (416 ns) and warm-clamp-acc64 (416 ns) differ by 0.05%, inside the noise, even though the wider field spreads 411.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-clamp-minimum shows warm-up / thermal drift (autocorr +0.88)

warm-clamp-minimum's per-pass series has lag-1 autocorrelation +0.88, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-accfit, warm-clamp-acc64, warm-clamp-head, warm-clamp-accfit-dyn} vs {warm-clamp-min-lanes, warm-clamp-minimum} (101% apart)

The field splits into a fast tier {warm-clamp-accfit, warm-clamp-acc64, warm-clamp-head, warm-clamp-accfit-dyn} and a slow tier {warm-clamp-min-lanes, warm-clamp-minimum} with a 101% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### warm-clamp-accfit's comparison is tie-heavy (15% tied pairs)

15% of paired samples for warm-clamp-accfit are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Wide spread: slowest is 5.1x the fastest

Fastest warm-clamp-accfit (416 ns) to slowest warm-clamp-minimum (2.13 us): 5.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-clamp-accfit's edge over baseline is significant but tiny (0 ns, 0.00%)

warm-clamp-accfit differs from baseline warm-clamp-acc64 by 0 ns (0.00%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-clamp-accfit** at 416.0 ns median (-0.0% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 5.12x (fastest 416.0 ns, slowest 2128.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 482ns | 478ns | 474ns | 478ns | 504ns | base |
| warm-clamp-accfit | 477ns | 476ns | 474ns | 476ns | 479ns | -1.21% |
| warm-clamp-accfit-dyn | 936ns | 928ns | 922ns | 931ns | 965ns | +94.08% |
| warm-clamp-head | 609ns | 596ns | 555ns | 601ns | 690ns | +26.35% |
| warm-clamp-min-lanes | 1839ns | 1805ns | 1786ns | 1808ns | 1985ns | +281.28% |
| warm-clamp-minimum | 2355ns | 2196ns | 2163ns | 2261ns | 2833ns | +388.33% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 421ns | 415ns | 439ns | base | 19.475 |
| warm-clamp-accfit | 416ns | 415ns | 418ns | -1.06% | 19.684 |
| warm-clamp-accfit-dyn | 873ns | 862ns | 891ns | +107.44% | 9.388 |
| warm-clamp-head | 544ns | 495ns | 617ns | +29.33% | 15.058 |
| warm-clamp-min-lanes | 1775ns | 1724ns | 1914ns | +321.91% | 4.616 |
| warm-clamp-minimum | 2284ns | 2097ns | 2751ns | +442.88% | 3.587 |

## Performance model

- Peak throughput: **19.759 Gops/s** (warm-clamp-accfit; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 19.683 | 99.6% |
| warm-clamp-accfit | 19.692 | 99.7% |
| warm-clamp-accfit-dyn | 9.441 | 47.8% |
| warm-clamp-head | 15.426 | 78.1% |
| warm-clamp-min-lanes | 4.699 | 23.8% |
| warm-clamp-minimum | 3.849 | 19.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 482ns | 482ns | base |
| warm-clamp-accfit | 477ns | 477ns | -1.21% |
| warm-clamp-accfit-dyn | 936ns | 936ns | +94.08% |
| warm-clamp-head | 609ns | 609ns | +26.35% |
| warm-clamp-min-lanes | 1839ns | 1839ns | +281.28% |
| warm-clamp-minimum | 2355ns | 2355ns | +388.33% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 416ns | base | --- | [416, 417] | --- | --- | --- | --- |
| warm-clamp-accfit | 416ns | no significant difference | [-1, +0]ns | [415, 416] | no | 1.0000 | 1.0000 | 3 |
| warm-clamp-accfit-dyn | 868ns | +449.8ns (+108.1%) | [+449, +452]ns | [866, 873] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 531ns | +99.4ns (+23.9%) | [+82, +149]ns | [497, 566] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 1743ns | +1325.7ns (+318.5%) | [+1323, +1329]ns | [1741, 1746] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 2128ns | +1705.8ns (+409.9%) | [+1698, +1750]ns | [2114, 2166] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 417ns | +0.1% | +107.8% | +19.3% | +312.3% | +410.8% |
| 2 | 416ns | -0.1% | +107.5% | +19.2% | +313.1% | +402.6% |
| 3 | 417ns | +0.3% | +106.5% | +19.2% | +312.1% | +398.7% |
| 4 | 416ns | +0.1% | +108.1% | +19.7% | +312.7% | +403.4% |
| 5 | 415ns | +0.3% | +108.7% | +19.7% | +313.9% | +400.6% |
| 6 | 416ns | +0.1% | +111.4% | +18.7% | +315.0% | +405.8% |
| 7 | 415ns | +0.2% | +108.9% | +19.9% | +324.9% | +409.7% |
| 8 | 415ns | +0.4% | +107.7% | +19.4% | +320.4% | +408.8% |
| 9 | 416ns | -0.4% | +108.1% | +19.0% | +319.1% | +407.5% |
| 10 | 416ns | +0.0% | +107.4% | +19.4% | +328.2% | +408.0% |
| 11 | 415ns | +0.0% | +109.1% | +45.6% | +332.0% | +408.8% |
| 12 | 416ns | +0.1% | +108.7% | +46.2% | +318.8% | +408.1% |
| 13 | 418ns | -0.7% | +107.5% | +44.4% | +317.6% | +409.8% |
| 14 | 416ns | -0.2% | +108.4% | +44.6% | +317.8% | +463.5% |
| 15 | 416ns | -0.1% | +107.9% | +45.3% | +318.7% | +409.1% |
| 16 | 415ns | +0.1% | +109.7% | +45.5% | +320.4% | +410.1% |
| 17 | 424ns | -2.3% | +104.9% | +43.2% | +310.7% | +416.2% |
| 18 | 422ns | -1.6% | +104.9% | +41.9% | +390.8% | +405.2% |
| 19 | 417ns | -0.6% | +108.4% | +45.4% | +406.6% | +406.5% |
| 20 | 415ns | -0.1% | +109.9% | +45.5% | +401.7% | +470.6% |
| 21 | 457ns | -8.9% | +93.2% | +25.1% | +280.8% | +372.2% |
| 22 | 416ns | +0.2% | +112.1% | +36.3% | +317.9% | +407.4% |
| 23 | 416ns | +0.4% | +111.7% | +67.7% | +318.8% | +412.8% |
| 24 | 416ns | +0.0% | +111.5% | +35.8% | +318.5% | +422.0% |
| 25 | 417ns | -0.3% | +110.6% | +35.7% | +317.9% | +407.7% |
| 26 | 415ns | +0.1% | +116.4% | +35.8% | +319.7% | +419.7% |
| 27 | 420ns | -0.9% | +112.4% | +35.0% | +314.4% | +402.9% |
| 28 | 431ns | -3.6% | +104.2% | +30.6% | +303.5% | +391.2% |
| 29 | 430ns | -3.5% | +104.1% | +31.0% | +305.2% | +398.3% |
| 30 | 422ns | -1.5% | +109.3% | +33.9% | +313.4% | +400.9% |
| 31 | 507ns | -17.7% | +82.9% | -2.5% | +244.5% | +393.1% |
| 32 | 422ns | -2.0% | +105.1% | +17.6% | +313.7% | +548.1% |
| 33 | 415ns | +0.4% | +107.2% | +20.0% | +320.6% | +560.0% |
| 34 | 417ns | +0.1% | +107.6% | +19.1% | +343.8% | +556.9% |
| 35 | 415ns | +0.1% | +108.2% | +20.1% | +327.3% | +561.0% |
| 36 | 416ns | +0.1% | +106.8% | +20.0% | +338.8% | +581.0% |
| 37 | 415ns | +1.1% | +108.3% | +19.8% | +319.8% | +559.1% |
| 38 | 415ns | +0.9% | +108.0% | +19.5% | +321.0% | +559.5% |
| 39 | 416ns | -0.3% | +111.7% | +18.2% | +319.3% | +558.2% |
| 40 | 414ns | +1.7% | +111.2% | +20.4% | +333.0% | +562.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.061 | ok |
| warm-clamp-accfit | 0.063 | ok |
| warm-clamp-accfit-dyn | 0.400 | moderate+ |
| warm-clamp-head | 0.740 | HIGH+ (drift/warm-up) |
| warm-clamp-min-lanes | 0.614 | HIGH+ (drift/warm-up) |
| warm-clamp-minimum | 0.875 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-clamp-accfit**: won 15/40, lost 10/40
- **warm-clamp-accfit-dyn**: won 0/40, lost 40/40
- **warm-clamp-head**: won 1/40, lost 39/40
- **warm-clamp-min-lanes**: won 0/40, lost 40/40
- **warm-clamp-minimum**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 2.4ns | 420.6ns | 0.6% |  |
| warm-clamp-accfit | 2.4ns | 416.2ns | 0.6% |  |
| warm-clamp-accfit-dyn | 2.6ns | 872.6ns | 0.3% |  |
| warm-clamp-head | 2.9ns | 544.0ns | 0.5% |  |
| warm-clamp-min-lanes | 2.5ns | 1774.8ns | 0.1% |  |
| warm-clamp-minimum | 2.8ns | 2283.6ns | 0.1% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 414.8-439.4 ns)
    414.8 |########################################
    416.0 |##################################
    417.3 |##
    418.5 |
    419.7 |##
    420.9 |##
    422.2 |#####
    423.4 |##
    424.6 |
    425.9 |
    427.1 |
    428.3 |
    429.6 |##
    430.8 |##
    432.0 |
    433.3 |
    434.5 |
    435.7 |
    437.0 |
    438.2 |
  (4 below, 2 above range)

warm-clamp-accfit (n=40, range 414.6-418.3 ns)
    414.6 |###########
    414.8 |
    415.0 |############################
    415.2 |
    415.3 |############################
    415.5 |
    415.7 |############################
    415.9 |
    416.1 |########################################
    416.3 |
    416.5 |
    416.7 |#################
    416.8 |
    417.0 |######################
    417.2 |
    417.4 |###########
    417.6 |
    417.8 |#####
    418.0 |
    418.2 |
  (3 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 862.1-891.1 ns)
    862.1 |####################
    863.6 |##########
    865.0 |########################################
    866.5 |####################
    867.9 |###############
    869.4 |#####
    870.8 |#####
    872.3 |#####
    873.7 |
    875.2 |
    876.6 |#####
    878.1 |##########
    879.5 |###############
    881.0 |#####
    882.4 |###############
    883.8 |
    885.3 |
    886.7 |
    888.2 |
    889.6 |
  (3 below, 3 above range)

warm-clamp-head (n=40, range 495.0-616.9 ns)
    495.0 |########################################
    501.1 |
    507.2 |
    513.3 |
    519.4 |
    525.5 |
    531.6 |
    537.6 |
    543.7 |
    549.8 |
    555.9 |
    562.0 |##################
    568.1 |##
    574.2 |
    580.3 |
    586.4 |
    592.5 |
    598.6 |##############
    604.7 |#########
    610.8 |
  (3 below, 1 above range)

warm-clamp-min-lanes (n=40, range 1724.1-1914.1 ns)
   1724.1 |###
   1733.6 |########################################
   1743.1 |#################################
   1752.6 |###
   1762.1 |
   1771.6 |###
   1781.1 |###
   1790.6 |######
   1800.1 |
   1809.6 |
   1819.1 |###
   1828.6 |
   1838.1 |
   1847.6 |###
   1857.1 |
   1866.6 |
   1876.1 |
   1885.6 |
   1895.1 |
   1904.6 |
  (5 below, 3 above range)

warm-clamp-minimum (n=40, range 2097.5-2750.9 ns)
   2097.5 |########################################
   2130.2 |###########
   2162.8 |####
   2195.5 |
   2228.2 |
   2260.9 |
   2293.5 |
   2326.2 |##
   2358.9 |##
   2391.5 |
   2424.2 |
   2456.9 |
   2489.6 |##
   2522.2 |
   2554.9 |
   2587.6 |
   2620.2 |
   2652.9 |
   2685.6 |
   2718.3 |##################
  (4 below, 1 above range)

```

## Diagnostics

- **warm-clamp-head**: autocorrelation=0.74 (measurement drift or warm-up artifact)
- **warm-clamp-min-lanes**: autocorrelation=0.61 (measurement drift or warm-up artifact)
- **warm-clamp-minimum**: autocorrelation=0.88 (measurement drift or warm-up artifact)
