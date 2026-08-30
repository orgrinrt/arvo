# Clamping fold at 64 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-accfit-dyn is an outlier: 3.2x slower than the field

warm-clamp-accfit-dyn (3.19 us) is 3.2x the fastest (1.00 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-clamp-acc64, warm-clamp-minimum) are a dead heat (<1%)

warm-clamp-acc64 (1.00 us) and warm-clamp-minimum (1.01 us) differ by 0.25%, inside the noise, even though the wider field spreads 217.7%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-clamp-acc64 shows warm-up / thermal drift (autocorr +0.88)

warm-clamp-acc64's per-pass series has lag-1 autocorrelation +0.88, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (warm-clamp-acc64)

The baseline warm-clamp-acc64 is the fastest (1.00 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {warm-clamp-acc64, warm-clamp-minimum, warm-clamp-min-lanes} vs {warm-clamp-head, warm-clamp-accfit, warm-clamp-accfit-dyn} (99% apart)

The field splits into a fast tier {warm-clamp-acc64, warm-clamp-minimum, warm-clamp-min-lanes} and a slow tier {warm-clamp-head, warm-clamp-accfit, warm-clamp-accfit-dyn} with a 99% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.2x the fastest

Fastest warm-clamp-acc64 (1.00 us) to slowest warm-clamp-accfit-dyn (3.19 us): 3.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-clamp-min-lanes's edge over baseline is significant but tiny (3 ns, 0.29%)

warm-clamp-min-lanes differs from baseline warm-clamp-acc64 by 3 ns (0.29%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (warm-clamp-acc64) is the fastest** at 1004.8 ns median
- 3 variants significantly slower than baseline
- Spread: 3.18x (fastest 1004.8 ns, slowest 3191.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 1114ns | 1066ns | 1049ns | 1084ns | 1269ns | base |
| warm-clamp-accfit | 2201ns | 2191ns | 2104ns | 2175ns | 2377ns | +97.56% |
| warm-clamp-accfit-dyn | 3355ns | 3250ns | 3174ns | 3283ns | 3752ns | +201.13% |
| warm-clamp-head | 2204ns | 2143ns | 2071ns | 2156ns | 2481ns | +97.85% |
| warm-clamp-min-lanes | 1109ns | 1092ns | 1046ns | 1101ns | 1198ns | -0.44% |
| warm-clamp-minimum | 1121ns | 1084ns | 1044ns | 1097ns | 1271ns | +0.64% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 1050ns | 987ns | 1195ns | base | 7.805 |
| warm-clamp-accfit | 2133ns | 2040ns | 2304ns | +103.25% | 3.840 |
| warm-clamp-accfit-dyn | 3293ns | 3115ns | 3682ns | +213.77% | 2.487 |
| warm-clamp-head | 2104ns | 1979ns | 2372ns | +100.50% | 3.893 |
| warm-clamp-min-lanes | 1043ns | 985ns | 1126ns | -0.64% | 7.855 |
| warm-clamp-minimum | 1053ns | 984ns | 1197ns | +0.30% | 7.782 |

## Performance model

- Peak throughput: **8.322 Gops/s** (warm-clamp-minimum; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 8.153 | 98.0% |
| warm-clamp-accfit | 3.857 | 46.3% |
| warm-clamp-accfit-dyn | 2.566 | 30.8% |
| warm-clamp-head | 4.007 | 48.1% |
| warm-clamp-min-lanes | 7.971 | 95.8% |
| warm-clamp-minimum | 8.133 | 97.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 1114ns | 1114ns | base |
| warm-clamp-accfit | 2201ns | 2201ns | +97.56% |
| warm-clamp-accfit-dyn | 3355ns | 3355ns | +201.13% |
| warm-clamp-head | 2204ns | 2204ns | +97.85% |
| warm-clamp-min-lanes | 1109ns | 1109ns | -0.44% |
| warm-clamp-minimum | 1121ns | 1121ns | +0.64% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 1005ns | base | --- | [1000, 1027] | --- | --- | --- | --- |
| warm-clamp-accfit | 2124ns | +1104.4ns (+109.9%) | [+1073, +1131]ns | [2078, 2132] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 3192ns | +2174.4ns (+216.4%) | [+2135, +2277]ns | [3173, 3291] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 2045ns | +1021.0ns (+101.6%) | [+998, +1051]ns | [2018, 2050] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 1028ns | no significant difference | [-14, +9]ns | [994, 1056] | no | 0.6530 | 0.5224 | 1 |
| warm-clamp-minimum | 1007ns | no significant difference | [-13, +17]ns | [989, 1039] | no | 0.8746 | 0.8746 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 1080ns | +89.2% | +187.7% | +119.4% | -3.9% | -8.5% |
| 2 | 1195ns | +72.4% | +160.0% | +98.4% | -13.1% | -17.5% |
| 3 | 1198ns | +71.9% | +161.2% | +97.8% | -16.0% | -17.8% |
| 4 | 1192ns | +70.7% | +163.0% | +99.1% | -14.5% | -17.3% |
| 5 | 1196ns | +73.3% | +163.5% | +98.4% | -13.4% | -17.7% |
| 6 | 1193ns | +71.0% | +166.3% | +98.8% | -11.5% | -17.5% |
| 7 | 1194ns | +70.8% | +166.5% | +98.5% | -3.7% | -17.3% |
| 8 | 1192ns | +71.0% | +166.3% | +98.9% | -4.1% | -16.7% |
| 9 | 1197ns | +70.5% | +166.6% | +98.0% | -9.0% | -17.1% |
| 10 | 1196ns | +70.5% | +163.0% | +98.4% | -13.3% | -17.5% |
| 11 | 1001ns | +112.9% | +213.0% | +94.3% | -1.4% | -1.2% |
| 12 | 1000ns | +113.4% | +214.0% | +101.2% | -1.2% | -1.3% |
| 13 | 1000ns | +113.1% | +245.6% | +105.1% | -0.2% | -1.6% |
| 14 | 1000ns | +124.3% | +273.6% | +105.0% | -1.3% | -1.2% |
| 15 | 1008ns | +111.4% | +267.2% | +103.8% | -2.6% | +4.3% |
| 16 | 1006ns | +111.9% | +216.1% | +103.5% | -2.0% | -0.7% |
| 17 | 1049ns | +103.5% | +196.9% | +95.5% | +0.8% | -5.9% |
| 18 | 993ns | +114.9% | +211.8% | +106.0% | +1.0% | -0.6% |
| 19 | 990ns | +115.4% | +213.4% | +106.9% | +1.1% | -0.8% |
| 20 | 981ns | +117.3% | +225.9% | +108.7% | +2.0% | +0.2% |
| 21 | 985ns | +118.9% | +219.5% | +101.3% | +0.4% | +21.3% |
| 22 | 985ns | +138.5% | +275.2% | +101.3% | +0.1% | +20.8% |
| 23 | 986ns | +109.1% | +278.2% | +101.2% | -0.2% | +21.2% |
| 24 | 984ns | +110.3% | +280.2% | +101.6% | +0.0% | +22.0% |
| 25 | 987ns | +108.4% | +261.3% | +101.2% | -0.2% | +21.3% |
| 26 | 1001ns | +108.2% | +213.6% | +98.2% | -1.5% | +19.3% |
| 27 | 1000ns | +110.2% | +231.6% | +98.5% | -1.3% | +19.8% |
| 28 | 1001ns | +111.7% | +218.2% | +98.2% | -1.4% | +19.8% |
| 29 | 1000ns | +113.1% | +229.2% | +98.2% | -1.4% | +19.5% |
| 30 | 1000ns | +104.7% | +232.4% | +98.3% | +5.1% | +19.2% |
| 31 | 1022ns | +104.4% | +212.6% | +97.3% | +9.2% | +1.2% |
| 32 | 998ns | +111.0% | +221.2% | +105.4% | +12.2% | +7.1% |
| 33 | 1035ns | +105.6% | +208.5% | +94.9% | +7.8% | +0.4% |
| 34 | 1029ns | +102.9% | +208.0% | +96.3% | +8.8% | +2.1% |
| 35 | 1022ns | +122.2% | +222.4% | +100.1% | +9.4% | -2.0% |
| 36 | 1000ns | +129.7% | +228.2% | +102.3% | +12.0% | +3.4% |
| 37 | 1025ns | +131.3% | +222.8% | +99.4% | +9.2% | +1.1% |
| 38 | 1020ns | +125.4% | +222.4% | +97.8% | +9.6% | +0.3% |
| 39 | 1003ns | +129.0% | +259.4% | +100.4% | +11.3% | +1.0% |
| 40 | 1035ns | +122.2% | +255.5% | +95.8% | +8.2% | +0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.879 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit | 0.643 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit-dyn | 0.610 | HIGH+ (drift/warm-up) |
| warm-clamp-head | 0.859 | HIGH+ (drift/warm-up) |
| warm-clamp-min-lanes | 0.846 | HIGH+ (drift/warm-up) |
| warm-clamp-minimum | 0.850 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-clamp-accfit**: won 0/40, lost 40/40
- **warm-clamp-accfit-dyn**: won 0/40, lost 40/40
- **warm-clamp-head**: won 0/40, lost 40/40
- **warm-clamp-min-lanes**: won 22/40, lost 17/40
- **warm-clamp-minimum**: won 19/40, lost 21/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 2.5ns | 1049.6ns | 0.2% |  |
| warm-clamp-accfit | 2.6ns | 2133.3ns | 0.1% |  |
| warm-clamp-accfit-dyn | 2.9ns | 3293.3ns | 0.1% |  |
| warm-clamp-head | 2.8ns | 2104.5ns | 0.1% |  |
| warm-clamp-min-lanes | 2.5ns | 1042.9ns | 0.2% |  |
| warm-clamp-minimum | 2.6ns | 1052.7ns | 0.2% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 986.6-1195.2 ns)
    986.6 |#########
    997.0 |########################################
   1007.4 |###
   1017.8 |############
   1028.3 |#########
   1038.7 |###
   1049.1 |
   1059.6 |
   1070.0 |###
   1080.4 |
   1090.9 |
   1101.3 |
   1111.7 |
   1122.2 |
   1132.6 |
   1143.0 |
   1153.5 |
   1163.9 |
   1174.3 |
   1184.8 |###############
  (5 below, 4 above range)

warm-clamp-accfit (n=40, range 2040.4-2304.1 ns)
   2040.4 |###############
   2053.5 |####################
   2066.7 |##########
   2079.9 |###############
   2093.1 |##########
   2106.3 |#####
   2119.5 |########################################
   2132.7 |###############
   2145.8 |#####
   2159.0 |
   2172.2 |
   2185.4 |
   2198.6 |
   2211.8 |
   2225.0 |
   2238.1 |#####
   2251.3 |
   2264.5 |#####
   2277.7 |
   2290.9 |####################
  (5 below, 2 above range)

warm-clamp-accfit-dyn (n=40, range 3115.5-3682.5 ns)
   3115.5 |######################
   3143.8 |#################
   3172.2 |########################################
   3200.5 |####
   3228.9 |
   3257.2 |####
   3285.6 |#################
   3313.9 |########
   3342.3 |
   3370.6 |
   3399.0 |
   3427.3 |
   3455.7 |####
   3484.0 |
   3512.4 |
   3540.7 |####
   3569.1 |
   3597.4 |####
   3625.8 |
   3654.1 |####
  (5 below, 5 above range)

warm-clamp-head (n=40, range 1978.7-2371.7 ns)
   1978.7 |####################################
   1998.3 |##############
   2018.0 |##############
   2037.6 |########################################
   2057.3 |
   2076.9 |
   2096.6 |
   2116.2 |
   2135.9 |
   2155.5 |
   2175.2 |
   2194.8 |
   2214.5 |
   2234.1 |
   2253.8 |
   2273.5 |
   2293.1 |
   2312.8 |
   2332.4 |
   2352.1 |#####################
  (1 below, 4 above range)

warm-clamp-min-lanes (n=40, range 985.1-1126.2 ns)
    985.1 |########################################
    992.2 |####
    999.3 |############
   1006.3 |####
   1013.4 |####
   1020.4 |
   1027.5 |
   1034.5 |################
   1041.6 |
   1048.6 |####
   1055.7 |########
   1062.7 |
   1069.8 |
   1076.8 |
   1083.9 |####
   1090.9 |
   1098.0 |
   1105.1 |
   1112.1 |########################
   1119.2 |################
  (4 below, 2 above range)

warm-clamp-minimum (n=40, range 984.4-1196.8 ns)
    984.4 |########################################
    995.0 |#####
   1005.6 |##
   1016.2 |##
   1026.9 |########
   1037.5 |#####
   1048.1 |#####
   1058.7 |##
   1069.3 |
   1080.0 |
   1090.6 |
   1101.2 |
   1111.8 |
   1122.4 |
   1133.1 |
   1143.7 |
   1154.3 |
   1164.9 |
   1175.5 |
   1186.2 |##################
  (3 below, 3 above range)

```

## Diagnostics

- **warm-clamp-acc64**: autocorrelation=0.88 (measurement drift or warm-up artifact)
- **warm-clamp-accfit**: autocorrelation=0.64 (measurement drift or warm-up artifact)
- **warm-clamp-accfit-dyn**: autocorrelation=0.61 (measurement drift or warm-up artifact)
- **warm-clamp-head**: autocorrelation=0.86 (measurement drift or warm-up artifact)
- **warm-clamp-min-lanes**: autocorrelation=0.85 (measurement drift or warm-up artifact)
- **warm-clamp-minimum**: autocorrelation=0.85 (measurement drift or warm-up artifact)
