# The same arms over a 16 MiB column, past every cache level on this host: what the reassociation is worth once the load stream is the binding constraint

8 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-neon dominates: 206% faster than the next best (satfold-lanes4-idx)

satfold-neon (359.18 us) leads satfold-lanes4-idx (1.10 ms) by 206%, a clear separation rather than a photo finish. CV 8.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### satfold-neon beats baseline by 101% (significant)

satfold-neon is -14.53 ms (101%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-seq is an outlier: 40.9x slower than the field

satfold-seq (14.69 ms) is 40.9x the fastest (359.18 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-neon shows warm-up / thermal drift (autocorr +0.76)

satfold-neon's per-pass series has lag-1 autocorrelation +0.76, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-neon} vs {satfold-lanes4-idx, satfold-lanes16, satfold-lanes16-constl, satfold-lanes64, satfold-nolaw, satfold-iterfold, satfold-seq} (206% apart)

The field splits into a fast tier {satfold-neon} and a slow tier {satfold-lanes4-idx, satfold-lanes16, satfold-lanes16-constl, satfold-lanes64, satfold-nolaw, satfold-iterfold, satfold-seq} with a 206% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 40.9x the fastest

Fastest satfold-neon (359.18 us) to slowest satfold-seq (14.69 ms): 40.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: satfold-neon** at 359180.2 ns median (-97.5% vs baseline)
- 6 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 40.90x (fastest 359180.2 ns, slowest 14690224.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 14683047ns | 14440606ns | 14384706ns | 14529720ns | 15441370ns | base |
| satfold-lanes16 | 2149058ns | 2104316ns | 2077356ns | 2111076ns | 2334707ns | -85.36% |
| satfold-lanes16-constl | 2137856ns | 2115799ns | 2098807ns | 2118659ns | 2234499ns | -85.44% |
| satfold-lanes4-idx | 1156348ns | 1098853ns | 1025935ns | 1095733ns | 1468606ns | -92.12% |
| satfold-lanes64 | 3994135ns | 3975922ns | 3957513ns | 3977158ns | 4081689ns | -72.80% |
| satfold-neon | 351867ns | 360399ns | 306535ns | 354072ns | 390587ns | -97.60% |
| satfold-nolaw | 11298615ns | 11106468ns | 11065079ns | 11120297ns | 12067104ns | -23.05% |
| satfold-seq | 14901033ns | 14693202ns | 14580879ns | 14721327ns | 15760303ns | +1.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 14680176ns | 14381756ns | 15437921ns | base | 1.143 |
| satfold-lanes16 | 2147675ns | 2076515ns | 2332717ns | -85.37% | 7.812 |
| satfold-lanes16-constl | 2136287ns | 2097185ns | 2233025ns | -85.45% | 7.853 |
| satfold-lanes4-idx | 1155036ns | 1024597ns | 1467037ns | -92.13% | 14.525 |
| satfold-lanes64 | 3992225ns | 3955449ns | 4079637ns | -72.81% | 4.202 |
| satfold-neon | 350574ns | 305115ns | 389231ns | -97.61% | 47.856 |
| satfold-nolaw | 11296366ns | 11062946ns | 12064745ns | -23.05% | 1.485 |
| satfold-seq | 14898360ns | 14578492ns | 15757195ns | +1.49% | 1.126 |

## Performance model

- Peak throughput: **54.987 Gops/s** (satfold-neon; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 1.162 | 2.1% |
| satfold-lanes16 | 7.978 | 14.5% |
| satfold-lanes16-constl | 7.936 | 14.4% |
| satfold-lanes4-idx | 15.283 | 27.8% |
| satfold-lanes64 | 4.222 | 7.7% |
| satfold-neon | 46.710 | 84.9% |
| satfold-nolaw | 1.511 | 2.7% |
| satfold-seq | 1.142 | 2.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 14683047ns | 14683047ns | base |
| satfold-lanes16 | 2149058ns | 2149058ns | -85.36% |
| satfold-lanes16-constl | 2137856ns | 2137856ns | -85.44% |
| satfold-lanes4-idx | 1156348ns | 1156348ns | -92.12% |
| satfold-lanes64 | 3994135ns | 3994135ns | -72.80% |
| satfold-neon | 351867ns | 351867ns | -97.60% |
| satfold-nolaw | 11298615ns | 11298615ns | -23.05% |
| satfold-seq | 14901033ns | 14901033ns | +1.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 14437836ns | base | --- | [14417886, 14604142] | --- | --- | --- | --- |
| satfold-lanes16 | 2103054ns | -12348568.9ns (-85.5%) | [-12497590, -12318317]ns | [2092298, 2123406] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 2114049ns | -12335437.9ns (-85.4%) | [-12477720, -12313221]ns | [2107123, 2129596] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 1097737ns | -13375636.9ns (-92.6%) | [-13422532, -13338814]ns | [1046547, 1135892] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 3974122ns | -10465771.3ns (-72.5%) | [-10555217, -10448280]ns | [3968161, 3979611] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 359180ns | -14108515.6ns (-97.7%) | [-14254327, -14084458]ns | [336940, 369095] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 11104491ns | -3353447.1ns (-23.2%) | [-3379145, -3332352]ns | [11085050, 11151536] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 14690224ns | +216591.8ns (+1.5%) | [+162025, +258056]ns | [14640774, 14772888] | YES | 0.0002 | 0.0002 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|
| 1 | 14439613ns | -85.5% | -85.5% | -92.9% | -72.6% | -97.9% | -23.2% | +1.0% |
| 2 | 14403058ns | -84.0% | -85.4% | -92.7% | -72.5% | -97.8% | -23.1% | +1.6% |
| 3 | 14408944ns | -80.2% | -85.2% | -92.8% | -72.5% | -97.8% | -23.0% | +1.1% |
| 4 | 14419924ns | -82.0% | -85.4% | -92.9% | -72.5% | -97.8% | -23.2% | +1.0% |
| 5 | 14383195ns | -84.8% | -85.1% | -92.9% | -71.9% | -97.8% | -22.9% | +2.2% |
| 6 | 14382989ns | -84.8% | -85.3% | -92.9% | -72.4% | -97.9% | -23.1% | +1.4% |
| 7 | 14393166ns | -85.3% | -85.3% | -92.9% | -72.5% | -97.9% | -22.8% | +1.1% |
| 8 | 14411860ns | -85.4% | -85.4% | -92.9% | -72.5% | -97.9% | -23.3% | +1.9% |
| 9 | 14413180ns | -84.4% | -85.4% | -92.9% | -72.4% | -97.9% | -23.1% | +1.8% |
| 10 | 14398532ns | -85.3% | -85.5% | -92.9% | -72.4% | -97.7% | -23.2% | +1.3% |
| 11 | 14424368ns | -85.5% | -85.4% | -91.7% | -72.4% | -97.3% | -23.3% | +2.5% |
| 12 | 14429889ns | -85.5% | -85.4% | -91.8% | -72.4% | -97.3% | -23.3% | +1.4% |
| 13 | 14425394ns | -85.5% | -85.4% | -92.2% | -72.5% | -97.4% | -23.3% | +14.7% |
| 14 | 14427167ns | -85.4% | -85.3% | -92.3% | -72.4% | -97.3% | -23.2% | +21.0% |
| 15 | 14436058ns | -85.5% | -85.4% | -91.7% | -72.4% | -97.4% | -23.3% | +1.8% |
| 16 | 14352119ns | -85.4% | -85.1% | -92.4% | -72.3% | -97.3% | -22.5% | +1.8% |
| 17 | 14395156ns | -85.4% | -85.2% | -84.2% | -72.3% | -97.3% | -23.1% | +1.5% |
| 18 | 14415848ns | -85.4% | -85.4% | -92.6% | -72.4% | -97.4% | -23.3% | +1.8% |
| 19 | 14490373ns | -85.6% | -85.3% | -92.7% | -72.6% | -97.5% | -11.1% | +0.8% |
| 20 | 15676008ns | -86.7% | -86.4% | -93.0% | -74.7% | -97.7% | -21.3% | +2.6% |
| 21 | 15393719ns | -86.5% | -86.2% | -93.2% | -74.2% | -97.6% | -23.3% | -4.9% |
| 22 | 14975081ns | -86.2% | -86.0% | -93.2% | -73.6% | -97.5% | -25.4% | -1.7% |
| 23 | 14509018ns | -85.7% | -85.5% | -92.8% | -72.7% | -97.4% | -23.5% | +1.0% |
| 24 | 14847910ns | -86.0% | -85.9% | -93.1% | -73.3% | -97.5% | -24.3% | -1.8% |
| 25 | 14345835ns | -85.5% | -85.4% | -92.8% | -72.5% | -97.4% | -22.0% | +1.8% |
| 26 | 14511455ns | -85.7% | -85.5% | -89.7% | -72.6% | -97.1% | -23.4% | +1.2% |
| 27 | 14485479ns | -85.5% | -85.4% | -92.4% | -72.6% | -97.4% | -23.0% | +1.5% |
| 28 | 14480761ns | -85.5% | -85.3% | -91.7% | -72.7% | -97.5% | -23.5% | +1.9% |
| 29 | 14415818ns | -85.5% | -85.4% | -92.4% | -72.4% | -97.4% | -23.0% | +1.6% |
| 30 | 14405014ns | -85.1% | -85.3% | -92.4% | -72.6% | -97.6% | -23.0% | +2.0% |
| 31 | 14953285ns | -85.7% | -84.1% | -92.6% | -73.1% | -97.8% | -25.1% | -0.8% |
| 32 | 15196867ns | -85.9% | -84.3% | -92.5% | -73.6% | -97.7% | -20.5% | +3.2% |
| 33 | 15245082ns | -85.9% | -85.1% | -92.5% | -73.8% | -97.7% | -27.0% | -2.1% |
| 34 | 16729457ns | -87.4% | -87.0% | -92.8% | -76.1% | -97.8% | -33.1% | -10.2% |
| 35 | 14866526ns | -85.7% | -85.5% | -89.5% | -71.0% | -97.7% | -25.0% | -0.1% |
| 36 | 14796812ns | -85.6% | -85.4% | -89.2% | -72.4% | -97.7% | -7.7% | +0.1% |
| 37 | 14696829ns | -85.4% | -85.4% | -91.9% | -72.0% | -97.9% | -23.1% | +2.2% |
| 38 | 14758233ns | -85.7% | -85.8% | -92.1% | -72.8% | -97.9% | -24.7% | +1.7% |
| 39 | 14733132ns | -85.8% | -85.6% | -92.5% | -73.0% | -97.5% | -24.0% | +2.3% |
| 40 | 15333873ns | -86.1% | -85.7% | -92.5% | -73.6% | -97.8% | -27.3% | -1.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.473 | moderate+ |
| satfold-lanes16 | 0.613 | HIGH+ (drift/warm-up) |
| satfold-lanes16-constl | 0.673 | HIGH+ (drift/warm-up) |
| satfold-lanes4-idx | 0.099 | ok |
| satfold-lanes64 | 0.380 | moderate+ |
| satfold-neon | 0.763 | HIGH+ (drift/warm-up) |
| satfold-nolaw | 0.202 | moderate+ |
| satfold-seq | 0.317 | moderate+ |

**Consistency summary:**

- **satfold-lanes16**: won 40/40, lost 0/40
- **satfold-lanes16-constl**: won 40/40, lost 0/40
- **satfold-lanes4-idx**: won 40/40, lost 0/40
- **satfold-lanes64**: won 40/40, lost 0/40
- **satfold-neon**: won 40/40, lost 0/40
- **satfold-nolaw**: won 40/40, lost 0/40
- **satfold-seq**: won 7/40, lost 32/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 232.6ns | 14680175.7ns | 0.0% |  |
| satfold-lanes16 | 70.6ns | 2147674.7ns | 0.0% |  |
| satfold-lanes16-constl | 58.6ns | 2136286.6ns | 0.0% |  |
| satfold-lanes4-idx | 77.3ns | 1155035.6ns | 0.0% |  |
| satfold-lanes64 | 73.4ns | 3992224.6ns | 0.0% |  |
| satfold-neon | 11.9ns | 350574.1ns | 0.0% |  |
| satfold-nolaw | 152.8ns | 11296366.2ns | 0.0% |  |
| satfold-seq | 284.4ns | 14898359.6ns | 0.0% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 14381756.3-15437921.5 ns)
  14381756.3 |########################################
  14434564.5 |#########
  14487372.8 |#######
  14540181.1 |
  14592989.3 |
  14645797.6 |##
  14698605.8 |##
  14751414.1 |####
  14804222.4 |##
  14857030.6 |##
  14909838.9 |##
  14962647.1 |##
  15015455.4 |
  15068263.7 |
  15121071.9 |
  15173880.2 |##
  15226688.4 |##
  15279496.7 |
  15332304.9 |##
  15385113.2 |##
  (2 below, 2 above range)

satfold-lanes16 (n=40, range 2076515.5-2332717.2 ns)
  2076515.5 |##########
  2089325.6 |########################################
  2102135.6 |##########
  2114945.7 |##########
  2127755.8 |#############
  2140565.9 |##########
  2153376.0 |###
  2166186.1 |
  2178996.2 |######
  2191806.3 |
  2204616.4 |
  2217426.4 |
  2230236.5 |
  2243046.6 |###
  2255856.7 |
  2268666.8 |
  2281476.9 |
  2294287.0 |
  2307097.1 |###
  2319907.1 |
  (5 below, 2 above range)

satfold-lanes16-constl (n=40, range 2097185.5-2233024.8 ns)
  2097185.5 |#########################
  2103977.4 |########################################
  2110769.4 |#########################
  2117561.4 |##########
  2124353.3 |###############
  2131145.3 |####################
  2137937.3 |
  2144729.2 |#####
  2151521.2 |#####
  2158313.2 |#####
  2165105.1 |#####
  2171897.1 |
  2178689.1 |
  2185481.0 |
  2192273.0 |#####
  2199065.0 |
  2205856.9 |
  2212648.9 |
  2219440.9 |
  2226232.8 |
  (5 below, 3 above range)

satfold-lanes4-idx (n=40, range 1024597.2-1467037.1 ns)
  1024597.2 |########################################
  1046719.2 |############
  1068841.2 |####
  1090963.2 |############################
  1113085.2 |####
  1135207.2 |############
  1157329.2 |####
  1179451.2 |################
  1201573.2 |########
  1223695.2 |
  1245817.2 |
  1267939.1 |
  1290061.1 |
  1312183.1 |
  1334305.1 |
  1356427.1 |
  1378549.1 |
  1400671.1 |
  1422793.1 |
  1444915.1 |
  (4 below, 4 above range)

satfold-lanes64 (n=40, range 3955449.2-4079637.0 ns)
  3955449.2 |########################################
  3961658.6 |######################
  3967868.0 |##################################
  3974077.3 |########################################
  3980286.7 |#####
  3986496.1 |#####
  3992705.5 |###########
  3998914.9 |#####
  4005124.3 |#####
  4011333.7 |###########
  4017543.1 |
  4023752.5 |
  4029961.9 |
  4036171.3 |#####
  4042380.6 |
  4048590.0 |#####
  4054799.4 |
  4061008.8 |
  4067218.2 |
  4073427.6 |
  (3 below, 3 above range)

satfold-neon (n=40, range 305114.6-389231.4 ns)
  305114.6 |
  309320.4 |##########
  313526.3 |##############################
  317732.1 |##########
  321937.9 |##########
  326143.8 |##########
  330349.6 |
  334555.4 |##########
  338761.3 |##########
  342967.1 |##############################
  347173.0 |##########
  351378.8 |
  355584.6 |##########
  359790.5 |####################
  363996.3 |########################################
  368202.2 |####################
  372408.0 |
  376613.8 |########################################
  380819.7 |########################################
  385025.5 |##########
  (6 below, 3 above range)

satfold-nolaw (n=40, range 11062945.9-12064745.5 ns)
  11062945.9 |########################################
  11113035.9 |############
  11163125.8 |##########
  11213215.8 |##
  11263305.8 |##
  11313395.8 |
  11363485.7 |
  11413575.7 |
  11463665.7 |
  11513755.7 |
  11563845.7 |
  11613935.6 |
  11664025.6 |
  11714115.6 |
  11764205.6 |##
  11814295.6 |
  11864385.5 |
  11914475.5 |
  11964565.5 |
  12014655.5 |
  (2 below, 4 above range)

satfold-seq (n=40, range 14578491.9-15757195.2 ns)
  14578491.9 |########################################
  14637427.1 |################################
  14696362.3 |############
  14755297.4 |########
  14814232.6 |############
  14873167.8 |####
  14932102.9 |
  14991038.1 |############
  15049973.3 |####
  15108908.4 |####
  15167843.6 |
  15226778.8 |
  15285713.9 |
  15344649.1 |
  15403584.3 |
  15462519.4 |
  15521454.6 |
  15580389.8 |
  15639324.9 |####
  15698260.1 |
  (4 below, 3 above range)

```

## Diagnostics

- **satfold-lanes16**: autocorrelation=0.61 (measurement drift or warm-up artifact)
- **satfold-lanes16-constl**: autocorrelation=0.67 (measurement drift or warm-up artifact)
- **satfold-neon**: autocorrelation=0.76 (measurement drift or warm-up artifact)
