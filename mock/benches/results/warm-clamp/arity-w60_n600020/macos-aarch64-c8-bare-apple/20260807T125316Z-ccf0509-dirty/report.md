# Clamping fold at 60 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-accfit-dyn is an outlier: 4.1x slower than the field

warm-clamp-accfit-dyn (4.73 us) is 4.1x the fastest (1.15 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-accfit shows warm-up / thermal drift (autocorr +0.79)

warm-clamp-accfit's per-pass series has lag-1 autocorrelation +0.79, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (warm-clamp-acc64)

The baseline warm-clamp-acc64 is the fastest (1.15 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {warm-clamp-acc64, warm-clamp-accfit, warm-clamp-minimum} vs {warm-clamp-min-lanes, warm-clamp-head, warm-clamp-accfit-dyn} (83% apart)

The field splits into a fast tier {warm-clamp-acc64, warm-clamp-accfit, warm-clamp-minimum} and a slow tier {warm-clamp-min-lanes, warm-clamp-head, warm-clamp-accfit-dyn} with a 83% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.1x the fastest

Fastest warm-clamp-acc64 (1.15 us) to slowest warm-clamp-accfit-dyn (4.73 us): 4.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-clamp-accfit's edge over baseline is significant but tiny (5 ns, 0.43%)

warm-clamp-accfit differs from baseline warm-clamp-acc64 by 5 ns (0.43%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (warm-clamp-acc64) is the fastest** at 1152.2 ns median
- 5 variants significantly slower than baseline
- Spread: 4.11x (fastest 1152.2 ns, slowest 4730.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 1218ns | 1213ns | 1207ns | 1216ns | 1238ns | base |
| warm-clamp-accfit | 1244ns | 1234ns | 1207ns | 1245ns | 1278ns | +2.12% |
| warm-clamp-accfit-dyn | 4774ns | 4790ns | 4630ns | 4780ns | 4903ns | +291.89% |
| warm-clamp-head | 2908ns | 2889ns | 2832ns | 2889ns | 3038ns | +138.66% |
| warm-clamp-min-lanes | 2248ns | 2223ns | 2187ns | 2224ns | 2380ns | +84.50% |
| warm-clamp-minimum | 1282ns | 1242ns | 1209ns | 1254ns | 1442ns | +5.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 1156ns | 1147ns | 1172ns | base | 7.084 |
| warm-clamp-accfit | 1181ns | 1148ns | 1210ns | +2.15% | 6.935 |
| warm-clamp-accfit-dyn | 4714ns | 4574ns | 4839ns | +307.67% | 1.738 |
| warm-clamp-head | 2811ns | 2742ns | 2932ns | +143.06% | 2.914 |
| warm-clamp-min-lanes | 2184ns | 2125ns | 2314ns | +88.86% | 3.751 |
| warm-clamp-minimum | 1218ns | 1148ns | 1372ns | +5.33% | 6.725 |

## Performance model

- Peak throughput: **7.141 Gops/s** (warm-clamp-acc64; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 7.110 | 99.6% |
| warm-clamp-accfit | 6.988 | 97.9% |
| warm-clamp-accfit-dyn | 1.732 | 24.2% |
| warm-clamp-head | 2.935 | 41.1% |
| warm-clamp-min-lanes | 3.793 | 53.1% |
| warm-clamp-minimum | 6.954 | 97.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 1218ns | 1218ns | base |
| warm-clamp-accfit | 1244ns | 1244ns | +2.12% |
| warm-clamp-accfit-dyn | 4774ns | 4774ns | +291.89% |
| warm-clamp-head | 2908ns | 2908ns | +138.66% |
| warm-clamp-min-lanes | 2248ns | 2248ns | +84.50% |
| warm-clamp-minimum | 1282ns | 1282ns | +5.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 1152ns | base | --- | [1150, 1155] | --- | --- | --- | --- |
| warm-clamp-accfit | 1172ns | +9.0ns (+0.8%) | [+3, +53]ns | [1169, 1207] | YES | 0.0001 | 0.0000 | 3 |
| warm-clamp-accfit-dyn | 4731ns | +3569.2ns (+309.8%) | [+3503, +3642]ns | [4651, 4808] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 2791ns | +1632.7ns (+141.7%) | [+1609, +1647]ns | [2771, 2812] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 2160ns | +996.2ns (+86.5%) | [+987, +1009]ns | [2143, 2165] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 1178ns | +23.1ns (+2.0%) | [+15, +49]ns | [1171, 1212] | YES | 0.0002 | 0.0002 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 1148ns | +3.5% | +298.5% | +139.8% | +85.1% | +1.6% |
| 2 | 1148ns | +1.5% | +298.2% | +139.7% | +85.2% | +2.7% |
| 3 | 1154ns | +2.9% | +297.8% | +137.5% | +84.2% | +1.9% |
| 4 | 1149ns | +5.1% | +298.3% | +139.9% | +85.1% | +1.4% |
| 5 | 1148ns | +6.5% | +298.4% | +139.0% | +85.6% | +2.2% |
| 6 | 1145ns | +5.4% | +299.5% | +139.6% | +88.5% | +5.5% |
| 7 | 1149ns | +5.1% | +298.5% | +139.4% | +85.0% | +11.2% |
| 8 | 1150ns | +5.1% | +298.0% | +138.2% | +85.1% | +16.3% |
| 9 | 1149ns | +5.1% | +298.2% | +139.4% | +85.0% | +2.1% |
| 10 | 1161ns | +4.1% | +294.3% | +137.3% | +83.3% | +1.5% |
| 11 | 1171ns | +0.0% | +317.2% | +138.5% | +84.8% | -0.1% |
| 12 | 1168ns | +0.0% | +316.3% | +139.7% | +85.4% | +0.3% |
| 13 | 1167ns | +0.2% | +313.2% | +142.9% | +85.4% | +1.7% |
| 14 | 1166ns | +0.7% | +312.1% | +140.8% | +85.3% | +3.9% |
| 15 | 1167ns | +0.0% | +305.8% | +141.6% | +85.4% | +0.2% |
| 16 | 1167ns | +0.3% | +305.3% | +140.5% | +85.7% | +1.2% |
| 17 | 1169ns | +0.3% | +303.0% | +138.3% | +84.9% | +0.6% |
| 18 | 1168ns | +0.1% | +313.0% | +138.5% | +84.9% | +4.0% |
| 19 | 1166ns | +0.2% | +313.2% | +139.4% | +85.2% | +4.5% |
| 20 | 1168ns | +0.4% | +317.0% | +140.2% | +85.3% | +4.4% |
| 21 | 1150ns | +5.1% | +312.6% | +163.8% | +87.4% | +1.3% |
| 22 | 1147ns | +5.3% | +315.1% | +156.7% | +88.3% | +2.7% |
| 23 | 1151ns | +5.0% | +317.9% | +151.4% | +89.6% | -0.3% |
| 24 | 1150ns | +5.0% | +318.8% | +160.0% | +87.5% | -0.1% |
| 25 | 1151ns | +5.1% | +310.3% | +151.9% | +89.0% | -0.4% |
| 26 | 1144ns | +5.7% | +306.2% | +153.8% | +85.8% | +0.2% |
| 27 | 1150ns | +5.0% | +304.5% | +152.7% | +84.7% | -0.3% |
| 28 | 1154ns | +4.7% | +307.0% | +148.1% | +84.2% | -0.3% |
| 29 | 1155ns | +4.5% | +302.6% | +150.1% | +84.1% | -0.5% |
| 30 | 1149ns | -0.3% | +311.8% | +145.1% | +85.2% | +0.2% |
| 31 | 1150ns | +0.0% | +316.8% | +145.7% | +94.4% | +21.5% |
| 32 | 1155ns | -0.5% | +296.1% | +141.5% | +93.6% | +20.8% |
| 33 | 1185ns | -3.1% | +286.1% | +135.0% | +88.9% | +17.6% |
| 34 | 1151ns | -0.1% | +307.3% | +142.5% | +94.5% | +21.2% |
| 35 | 1149ns | +0.0% | +319.2% | +142.5% | +94.3% | +21.4% |
| 36 | 1147ns | +0.3% | +319.8% | +140.5% | +94.9% | +16.0% |
| 37 | 1155ns | -0.6% | +316.7% | +136.2% | +93.7% | +4.9% |
| 38 | 1154ns | +0.7% | +317.3% | +137.4% | +104.0% | +15.9% |
| 39 | 1153ns | +0.8% | +317.1% | +144.8% | +121.7% | -0.5% |
| 40 | 1180ns | -2.8% | +307.9% | +133.8% | +104.5% | +5.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.456 | moderate+ |
| warm-clamp-accfit | 0.791 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit-dyn | 0.696 | HIGH+ (drift/warm-up) |
| warm-clamp-head | 0.718 | HIGH+ (drift/warm-up) |
| warm-clamp-min-lanes | 0.744 | HIGH+ (drift/warm-up) |
| warm-clamp-minimum | 0.660 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-clamp-accfit**: won 5/40, lost 29/40
- **warm-clamp-accfit-dyn**: won 0/40, lost 40/40
- **warm-clamp-head**: won 0/40, lost 40/40
- **warm-clamp-min-lanes**: won 0/40, lost 40/40
- **warm-clamp-minimum**: won 8/40, lost 32/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 2.5ns | 1156.4ns | 0.2% |  |
| warm-clamp-accfit | 2.4ns | 1181.2ns | 0.2% |  |
| warm-clamp-accfit-dyn | 2.0ns | 4714.4ns | 0.0% |  |
| warm-clamp-head | 2.8ns | 2810.8ns | 0.1% |  |
| warm-clamp-min-lanes | 2.6ns | 2184.0ns | 0.1% |  |
| warm-clamp-minimum | 2.9ns | 1218.1ns | 0.2% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 1147.1-1171.8 ns)
   1147.1 |####################
   1148.4 |########################################
   1149.6 |#################################
   1150.8 |#############
   1152.1 |######
   1153.3 |####################
   1154.5 |####################
   1155.8 |
   1157.0 |
   1158.2 |
   1159.5 |
   1160.7 |######
   1162.0 |
   1163.2 |
   1164.4 |
   1165.7 |##########################
   1166.9 |####################
   1168.1 |#############
   1169.4 |
   1170.6 |######
  (4 below, 2 above range)

warm-clamp-accfit (n=40, range 1148.4-1210.4 ns)
   1148.4 |###############
   1151.5 |
   1154.6 |
   1157.7 |
   1160.8 |######
   1163.9 |###
   1167.0 |###############
   1170.1 |############
   1173.2 |###
   1176.3 |
   1179.4 |
   1182.5 |
   1185.6 |######
   1188.7 |
   1191.8 |
   1194.9 |
   1198.0 |
   1201.1 |
   1204.2 |######
   1207.3 |########################################
  (4 below, 1 above range)

warm-clamp-accfit-dyn (n=40, range 4574.4-4839.2 ns)
   4574.4 |##############################
   4587.6 |#####
   4600.9 |
   4614.1 |
   4627.4 |
   4640.6 |###############
   4653.8 |
   4667.1 |
   4680.3 |#####
   4693.6 |#####
   4706.8 |#####
   4720.0 |###############
   4733.3 |##########
   4746.5 |
   4759.8 |#####
   4773.0 |
   4786.2 |#####
   4799.5 |####################
   4812.7 |########################################
   4826.0 |
  (5 below, 3 above range)

warm-clamp-head (n=40, range 2742.1-2932.5 ns)
   2742.1 |##########################
   2751.7 |########################################
   2761.2 |
   2770.7 |
   2780.2 |#################################
   2789.7 |####################
   2799.2 |##########################
   2808.7 |######
   2818.3 |####################
   2827.8 |######
   2837.3 |
   2846.8 |
   2856.3 |######
   2865.8 |
   2875.4 |
   2884.9 |######
   2894.4 |#############
   2903.9 |#############
   2913.4 |
   2922.9 |
  (4 below, 3 above range)

warm-clamp-min-lanes (n=40, range 2125.4-2313.8 ns)
   2125.4 |########################################
   2134.8 |
   2144.2 |
   2153.7 |##############################
   2163.1 |################
   2172.5 |###
   2181.9 |###
   2191.3 |
   2200.8 |
   2210.2 |
   2219.6 |
   2229.0 |####################
   2238.4 |###
   2247.9 |
   2257.3 |
   2266.7 |
   2276.1 |
   2285.5 |
   2295.0 |
   2304.4 |
  (2 below, 3 above range)

warm-clamp-minimum (n=40, range 1147.6-1372.2 ns)
   1147.6 |#################
   1158.8 |######################
   1170.1 |########################################
   1181.3 |####
   1192.5 |
   1203.8 |#################
   1215.0 |########
   1226.2 |
   1237.5 |####
   1248.7 |
   1259.9 |
   1271.2 |####
   1282.4 |
   1293.6 |
   1304.9 |
   1316.1 |
   1327.3 |#############
   1338.6 |
   1349.8 |
   1361.0 |
  (5 below, 5 above range)

```

## Diagnostics

- **warm-clamp-accfit**: autocorrelation=0.79 (measurement drift or warm-up artifact)
- **warm-clamp-accfit-dyn**: autocorrelation=0.70 (measurement drift or warm-up artifact)
- **warm-clamp-head**: autocorrelation=0.72 (measurement drift or warm-up artifact)
- **warm-clamp-min-lanes**: autocorrelation=0.74 (measurement drift or warm-up artifact)
- **warm-clamp-minimum**: autocorrelation=0.66 (measurement drift or warm-up artifact)
