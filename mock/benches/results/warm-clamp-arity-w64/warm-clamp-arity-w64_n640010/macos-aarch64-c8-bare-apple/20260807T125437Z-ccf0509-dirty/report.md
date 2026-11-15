# Clamping fold at 64 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-accfit-dyn is an outlier: 5.2x slower than the field

warm-clamp-accfit-dyn (4.31 us) is 5.2x the fastest (824 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-minimum is fastest but the noisiest (CV 9.3%)

warm-clamp-minimum wins on median (824 ns) yet has the highest variance (CV 9.3%), while warm-clamp-head is the steadiest (CV 1.0%, 2.06 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (warm-clamp-minimum, warm-clamp-min-lanes) are a dead heat (<1%)

warm-clamp-minimum (824 ns) and warm-clamp-min-lanes (826 ns) differ by 0.17%, inside the noise, even though the wider field spreads 423.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-clamp-minimum shows warm-up / thermal drift (autocorr +0.81)

warm-clamp-minimum's per-pass series has lag-1 autocorrelation +0.81, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-minimum, warm-clamp-min-lanes, warm-clamp-acc64, warm-clamp-accfit, warm-clamp-head} vs {warm-clamp-accfit-dyn} (109% apart)

The field splits into a fast tier {warm-clamp-minimum, warm-clamp-min-lanes, warm-clamp-acc64, warm-clamp-accfit, warm-clamp-head} and a slow tier {warm-clamp-accfit-dyn} with a 109% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.2x the fastest

Fastest warm-clamp-minimum (824 ns) to slowest warm-clamp-accfit-dyn (4.31 us): 5.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-clamp-minimum** at 824.2 ns median (-1.6% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 5.23x (fastest 824.2 ns, slowest 4312.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 966ns | 903ns | 893ns | 955ns | 1074ns | base |
| warm-clamp-accfit | 1098ns | 1083ns | 1069ns | 1081ns | 1177ns | +13.61% |
| warm-clamp-accfit-dyn | 4543ns | 4376ns | 4122ns | 4496ns | 5103ns | +370.15% |
| warm-clamp-head | 2162ns | 2155ns | 2147ns | 2156ns | 2198ns | +123.79% |
| warm-clamp-min-lanes | 912ns | 889ns | 883ns | 891ns | 1004ns | -5.61% |
| warm-clamp-minimum | 937ns | 886ns | 882ns | 910ns | 1073ns | -3.00% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 899ns | 832ns | 998ns | base | 9.110 |
| warm-clamp-accfit | 1035ns | 1007ns | 1113ns | +15.07% | 7.917 |
| warm-clamp-accfit-dyn | 4476ns | 4066ns | 5026ns | +397.79% | 1.830 |
| warm-clamp-head | 2072ns | 2060ns | 2103ns | +130.38% | 3.955 |
| warm-clamp-min-lanes | 848ns | 822ns | 934ns | -5.66% | 9.657 |
| warm-clamp-minimum | 870ns | 821ns | 1000ns | -3.26% | 9.417 |

## Performance model

- Peak throughput: **9.982 Gops/s** (warm-clamp-minimum; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 9.777 | 97.9% |
| warm-clamp-accfit | 8.027 | 80.4% |
| warm-clamp-accfit-dyn | 1.900 | 19.0% |
| warm-clamp-head | 3.969 | 39.8% |
| warm-clamp-min-lanes | 9.922 | 99.4% |
| warm-clamp-minimum | 9.939 | 99.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 966ns | 966ns | base |
| warm-clamp-accfit | 1098ns | 1098ns | +13.61% |
| warm-clamp-accfit-dyn | 4543ns | 4543ns | +370.15% |
| warm-clamp-head | 2162ns | 2162ns | +123.79% |
| warm-clamp-min-lanes | 912ns | 912ns | -5.61% |
| warm-clamp-minimum | 937ns | 937ns | -3.00% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 838ns | base | --- | [836, 968] | --- | --- | --- | --- |
| warm-clamp-accfit | 1021ns | +173.1ns (+20.7%) | [+51, +182]ns | [1010, 1024] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 4312ns | +3475.0ns (+414.7%) | [+3280, +3728]ns | [4116, 4726] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 2064ns | +1226.0ns (+146.3%) | [+1104, +1229]ns | [2063, 2066] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 826ns | -14.8ns (-1.8%) | [-140, -12]ns | [823, 835] | YES | 0.0028 | 0.0022 | 0 |
| warm-clamp-minimum | 824ns | -13.9ns (-1.7%) | [-96, -10]ns | [824, 826] | YES | 0.0064 | 0.0064 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 998ns | +2.2% | +372.7% | +106.4% | -16.2% | -17.5% |
| 2 | 999ns | +0.9% | +375.2% | +107.9% | -16.4% | -7.6% |
| 3 | 997ns | +2.9% | +372.3% | +106.9% | -16.5% | -17.5% |
| 4 | 998ns | +1.0% | +378.0% | +106.3% | -17.2% | -17.4% |
| 5 | 998ns | +0.9% | +373.4% | +106.6% | -17.7% | -17.6% |
| 6 | 998ns | +1.3% | +373.6% | +106.5% | -17.5% | -17.5% |
| 7 | 998ns | +1.0% | +374.7% | +106.7% | -17.5% | -17.5% |
| 8 | 999ns | +1.1% | +384.1% | +105.8% | -17.8% | -17.5% |
| 9 | 998ns | +1.1% | +373.5% | +106.5% | -17.3% | -17.6% |
| 10 | 999ns | +0.8% | +370.8% | +106.5% | -17.6% | -18.1% |
| 11 | 826ns | +24.3% | +400.9% | +150.0% | +21.0% | +0.2% |
| 12 | 835ns | +22.6% | +396.4% | +147.5% | +19.5% | -1.1% |
| 13 | 839ns | +22.8% | +412.5% | +150.8% | +19.2% | -1.7% |
| 14 | 835ns | +61.0% | +502.2% | +152.5% | +19.5% | -1.1% |
| 15 | 836ns | +37.0% | +501.3% | +146.9% | +13.1% | -1.3% |
| 16 | 836ns | +23.2% | +500.5% | +146.8% | -1.7% | -1.7% |
| 17 | 834ns | +23.1% | +501.6% | +147.7% | -1.4% | -1.3% |
| 18 | 837ns | +22.0% | +499.3% | +146.9% | -1.5% | -1.4% |
| 19 | 837ns | +20.4% | +405.8% | +146.6% | -1.8% | -1.1% |
| 20 | 836ns | +28.7% | +394.7% | +147.1% | -1.5% | -1.6% |
| 21 | 936ns | +7.7% | +339.2% | +132.2% | -8.8% | +6.7% |
| 22 | 823ns | +22.8% | +399.2% | +154.6% | +1.6% | +21.5% |
| 23 | 834ns | +22.9% | +389.8% | +148.7% | +0.3% | +19.8% |
| 24 | 835ns | +20.9% | +391.9% | +148.5% | +0.4% | +19.8% |
| 25 | 860ns | +20.9% | +379.5% | +140.2% | -2.8% | +16.2% |
| 26 | 835ns | +21.0% | +417.8% | +150.5% | -0.0% | +19.7% |
| 27 | 834ns | +21.0% | +495.3% | +147.1% | +0.2% | +20.0% |
| 28 | 836ns | +20.8% | +505.3% | +146.4% | +0.1% | +19.5% |
| 29 | 866ns | +16.3% | +486.0% | +138.5% | -3.5% | +15.3% |
| 30 | 865ns | +30.9% | +470.7% | +142.8% | -3.4% | +15.8% |
| 31 | 837ns | +20.2% | +386.0% | +146.3% | -1.7% | -1.6% |
| 32 | 836ns | +20.6% | +390.0% | +146.8% | -1.2% | -1.9% |
| 33 | 837ns | +21.2% | +386.2% | +146.8% | -1.6% | -2.1% |
| 34 | 836ns | +27.7% | +386.9% | +146.7% | -1.4% | -2.0% |
| 35 | 836ns | +28.3% | +385.0% | +147.5% | -1.2% | -1.5% |
| 36 | 940ns | +8.7% | +332.9% | +120.1% | -12.5% | -12.3% |
| 37 | 998ns | +2.6% | +306.8% | +106.8% | -17.8% | -17.5% |
| 38 | 996ns | +2.7% | +308.8% | +107.1% | -17.7% | -17.5% |
| 39 | 997ns | +2.6% | +307.4% | +107.0% | -17.4% | -17.6% |
| 40 | 998ns | +2.5% | +307.7% | +106.8% | -17.6% | -17.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.806 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit | 0.259 | moderate+ |
| warm-clamp-accfit-dyn | 0.757 | HIGH+ (drift/warm-up) |
| warm-clamp-head | 0.212 | moderate+ |
| warm-clamp-min-lanes | 0.778 | HIGH+ (drift/warm-up) |
| warm-clamp-minimum | 0.814 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-clamp-accfit**: won 0/40, lost 40/40
- **warm-clamp-accfit-dyn**: won 0/40, lost 40/40
- **warm-clamp-head**: won 0/40, lost 40/40
- **warm-clamp-min-lanes**: won 29/40, lost 9/40
- **warm-clamp-minimum**: won 29/40, lost 11/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 2.9ns | 899.2ns | 0.3% |  |
| warm-clamp-accfit | 2.7ns | 1034.7ns | 0.3% |  |
| warm-clamp-accfit-dyn | 3.3ns | 4476.0ns | 0.1% |  |
| warm-clamp-head | 2.5ns | 2071.5ns | 0.1% |  |
| warm-clamp-min-lanes | 2.8ns | 848.3ns | 0.3% |  |
| warm-clamp-minimum | 2.8ns | 869.9ns | 0.3% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 832.1-998.4 ns)
    832.1 |########################################
    840.4 |
    848.7 |
    857.0 |####
    865.4 |##
    873.7 |
    882.0 |
    890.3 |
    898.6 |
    906.9 |
    915.2 |
    923.6 |
    931.9 |##
    940.2 |##
    948.5 |
    956.8 |
    965.1 |
    973.5 |
    981.8 |
    990.1 |#######################
  (2 below, 3 above range)

warm-clamp-accfit (n=40, range 1007.2-1113.4 ns)
   1007.2 |########################################
   1012.5 |##
   1017.8 |##########
   1023.1 |#####################
   1028.4 |#####
   1033.7 |
   1039.0 |##
   1044.4 |
   1049.7 |
   1055.0 |
   1060.3 |
   1065.6 |##
   1070.9 |#####
   1076.2 |
   1081.5 |
   1086.8 |
   1092.1 |
   1097.5 |
   1102.8 |
   1108.1 |
  (3 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 4065.5-5026.1 ns)
   4065.5 |########################################
   4113.6 |##############
   4161.6 |
   4209.6 |###
   4257.7 |###
   4305.7 |###
   4353.7 |
   4401.8 |
   4449.8 |
   4497.8 |
   4545.8 |
   4593.9 |
   4641.9 |
   4689.9 |#########################
   4738.0 |#######
   4786.0 |
   4834.0 |###
   4882.0 |
   4930.1 |#######
   4978.1 |##########
  (3 below, 4 above range)

warm-clamp-head (n=40, range 2059.7-2103.0 ns)
   2059.7 |########################
   2061.9 |################################
   2064.0 |########################################
   2066.2 |########
   2068.4 |########
   2070.5 |
   2072.7 |########
   2074.9 |####
   2077.0 |
   2079.2 |
   2081.3 |
   2083.5 |
   2085.7 |
   2087.8 |
   2090.0 |
   2092.2 |####
   2094.3 |####
   2096.5 |
   2098.7 |####
   2100.8 |
  (3 below, 3 above range)

warm-clamp-min-lanes (n=40, range 821.6-933.9 ns)
    821.6 |########################################
    827.2 |##
    832.8 |#######################
    838.5 |
    844.1 |
    849.7 |##
    855.3 |
    860.9 |
    866.5 |
    872.1 |
    877.8 |
    883.4 |
    889.0 |
    894.6 |
    900.2 |
    905.8 |
    911.4 |
    917.0 |
    922.7 |
    928.3 |
  (3 below, 5 above range)

warm-clamp-minimum (n=40, range 820.6-999.8 ns)
    820.6 |########################################
    829.6 |
    838.6 |
    847.5 |
    856.5 |
    865.4 |
    874.4 |
    883.4 |
    892.3 |
    901.3 |
    910.2 |
    919.2 |#
    928.2 |
    937.1 |
    946.1 |
    955.0 |
    964.0 |
    973.0 |
    981.9 |
    990.9 |########
  (4 below, 5 above range)

```

## Diagnostics

- **warm-clamp-acc64**: autocorrelation=0.81 (measurement drift or warm-up artifact)
- **warm-clamp-accfit-dyn**: autocorrelation=0.76 (measurement drift or warm-up artifact)
- **warm-clamp-min-lanes**: autocorrelation=0.78 (measurement drift or warm-up artifact)
- **warm-clamp-minimum**: autocorrelation=0.81 (measurement drift or warm-up artifact)
