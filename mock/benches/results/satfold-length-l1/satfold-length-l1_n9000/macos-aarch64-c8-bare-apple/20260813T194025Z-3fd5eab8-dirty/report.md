# Saturating fold reassociation, reduction length swept, 32 KiB column: the fold as written against the idiomatic iterator form, against the licensed arm whose bounds are unprovable, against the licensed arm with the bounds proof, against the 64-element unroll with a tree combine, against the bounds proof with no law, against hand-written NEON, against the licensed arm with the length known at compile time

9 variants, 40 samples per variant.
Baseline: **satfold-iterfold**

## Highlights

Baseline for all deltas below: **satfold-iterfold**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### satfold-neon beats baseline by 99% (significant)

satfold-neon is -34.77 us (99%) faster than baseline satfold-iterfold, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-seq is an outlier: 104.0x slower than the field

satfold-seq (35.22 us) is 104.0x the fastest (339 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-neon8 shows warm-up / thermal drift (autocorr +0.83)

satfold-neon8's per-pass series has lag-1 autocorrelation +0.83, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-neon8, satfold-neon} vs {satfold-lanes16, satfold-lanes16-constl, satfold-lanes4-idx, satfold-lanes64, satfold-nolaw, satfold-iterfold, satfold-seq} (555% apart)

The field splits into a fast tier {satfold-neon8, satfold-neon} and a slow tier {satfold-lanes16, satfold-lanes16-constl, satfold-lanes4-idx, satfold-lanes64, satfold-nolaw, satfold-iterfold, satfold-seq} with a 555% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 104.0x the fastest

Fastest satfold-neon8 (339 ns) to slowest satfold-seq (35.22 us): 104.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: satfold-neon8** at 338.5 ns median (-99.0% vs baseline)
- 7 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 104.02x (fastest 338.5 ns, slowest 35216.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-iterfold | 35308ns | 35145ns | 35000ns | 35187ns | 35977ns | base |
| satfold-lanes16 | 2314ns | 2305ns | 2291ns | 2308ns | 2355ns | -93.45% |
| satfold-lanes16-constl | 2350ns | 2322ns | 2311ns | 2335ns | 2434ns | -93.35% |
| satfold-lanes4-idx | 2645ns | 2609ns | 2598ns | 2622ns | 2761ns | -92.51% |
| satfold-lanes64 | 5205ns | 5184ns | 5176ns | 5185ns | 5292ns | -85.26% |
| satfold-neon | 407ns | 403ns | 396ns | 403ns | 431ns | -98.85% |
| satfold-neon8 | 403ns | 399ns | 394ns | 401ns | 419ns | -98.86% |
| satfold-nolaw | 27097ns | 27011ns | 26735ns | 27036ns | 27640ns | -23.26% |
| satfold-seq | 35536ns | 35298ns | 35051ns | 35404ns | 36418ns | +0.65% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-iterfold | 35213ns | 34922ns | 35858ns | base | 0.931 |
| satfold-lanes16 | 2251ns | 2232ns | 2293ns | -93.61% | 14.557 |
| satfold-lanes16-constl | 2287ns | 2251ns | 2368ns | -93.51% | 14.330 |
| satfold-lanes4-idx | 2574ns | 2529ns | 2689ns | -92.69% | 12.731 |
| satfold-lanes64 | 5142ns | 5118ns | 5226ns | -85.40% | 6.372 |
| satfold-neon | 346ns | 337ns | 366ns | -99.02% | 94.798 |
| satfold-neon8 | 341ns | 334ns | 354ns | -99.03% | 96.028 |
| satfold-nolaw | 27016ns | 26668ns | 27539ns | -23.28% | 1.213 |
| satfold-seq | 35437ns | 34966ns | 36308ns | +0.63% | 0.925 |

## Performance model

- Peak throughput: **98.012 Gops/s** (satfold-neon8; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-iterfold | 0.935 | 1.0% |
| satfold-lanes16 | 14.634 | 14.9% |
| satfold-lanes16-constl | 14.530 | 14.8% |
| satfold-lanes4-idx | 12.929 | 13.2% |
| satfold-lanes64 | 6.398 | 6.5% |
| satfold-neon | 95.785 | 97.7% |
| satfold-neon8 | 96.789 | 98.8% |
| satfold-nolaw | 1.217 | 1.2% |
| satfold-seq | 0.930 | 0.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-iterfold | 35308ns | 35308ns | base |
| satfold-lanes16 | 2314ns | 2314ns | -93.45% |
| satfold-lanes16-constl | 2350ns | 2350ns | -93.35% |
| satfold-lanes4-idx | 2645ns | 2645ns | -92.51% |
| satfold-lanes64 | 5205ns | 5205ns | -85.26% |
| satfold-neon | 407ns | 407ns | -98.85% |
| satfold-neon8 | 403ns | 403ns | -98.86% |
| satfold-nolaw | 27097ns | 27097ns | -23.26% |
| satfold-seq | 35536ns | 35536ns | +0.65% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-iterfold | 35064ns | base | --- | [35014, 35107] | --- | --- | --- | --- |
| satfold-lanes16 | 2239ns | -32794.1ns (-93.5%) | [-32871, -32752]ns | [2235, 2244] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-constl | 2255ns | -32781.2ns (-93.5%) | [-32815, -32722]ns | [2252, 2268] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes4-idx | 2534ns | -32499.4ns (-92.7%) | [-32556, -32418]ns | [2532, 2568] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes64 | 5121ns | -29918.5ns (-85.3%) | [-29987, -29868]ns | [5120, 5124] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon | 342ns | -34706.0ns (-99.0%) | [-34764, -34664]ns | [339, 344] | YES | 0.0000 | 0.0000 | 0 |
| satfold-neon8 | 339ns | -34713.1ns (-99.0%) | [-34772, -34670]ns | [336, 340] | YES | 0.0000 | 0.0000 | 0 |
| satfold-nolaw | 26921ns | -8229.8ns (-23.5%) | [-8316, -8010]ns | [26783, 27124] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 35216ns | +79.8ns (+0.2%) | [+20, +205]ns | [35090, 35552] | YES | 0.0064 | 0.0064 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | satfold-iterfold | satfold-lanes16 | satfold-lanes16-constl | satfold-lanes4-idx | satfold-lanes64 | satfold-neon | satfold-neon8 | satfold-nolaw | satfold-seq |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 36213ns | -93.8% | -93.8% | -93.0% | -85.8% | -99.0% | -99.0% | -24.9% | -2.7% |
| 2 | 36040ns | -93.8% | -93.7% | -93.0% | -85.5% | -99.1% | -99.0% | -25.8% | -0.1% |
| 3 | 34987ns | -93.6% | -93.5% | -92.8% | -85.4% | -99.0% | -99.0% | -22.9% | +0.2% |
| 4 | 35305ns | -93.7% | -93.6% | -92.8% | -85.5% | -99.0% | -99.0% | -22.1% | +1.3% |
| 5 | 34902ns | -93.6% | -93.4% | -92.1% | -85.1% | -99.0% | -99.0% | -22.5% | +0.2% |
| 6 | 35016ns | -93.6% | -93.6% | -92.2% | -85.4% | -99.0% | -99.0% | -23.5% | +0.0% |
| 7 | 35066ns | -93.6% | -93.6% | -92.8% | -85.4% | -98.8% | -99.0% | -22.7% | +0.2% |
| 8 | 35066ns | -93.5% | -93.5% | -92.8% | -84.9% | -99.0% | -99.0% | -23.2% | +1.2% |
| 9 | 35419ns | -93.7% | -93.6% | -92.8% | -85.0% | -99.0% | -99.0% | -23.2% | +4.6% |
| 10 | 35066ns | -93.6% | -93.6% | -92.8% | -85.3% | -99.0% | -99.0% | -23.7% | +3.5% |
| 11 | 34984ns | -93.6% | -93.2% | -92.4% | -85.4% | -99.0% | -99.0% | -21.6% | +5.4% |
| 12 | 35573ns | -93.7% | -93.4% | -92.5% | -85.6% | -99.0% | -99.1% | -21.5% | +1.1% |
| 13 | 34968ns | -93.4% | -93.2% | -92.4% | -85.4% | -99.0% | -99.0% | -22.3% | +2.9% |
| 14 | 35084ns | -93.6% | -93.3% | -92.4% | -85.4% | -99.0% | -99.0% | -23.5% | +3.1% |
| 15 | 35515ns | -93.6% | -93.4% | -92.8% | -85.2% | -99.0% | -99.1% | -23.3% | +0.1% |
| 16 | 34944ns | -93.6% | -93.2% | -92.8% | -85.3% | -99.0% | -99.0% | -22.3% | +2.0% |
| 17 | 35425ns | -93.7% | -93.3% | -92.9% | -85.5% | -99.0% | -99.1% | -24.4% | +0.7% |
| 18 | 35754ns | -93.8% | -93.4% | -92.9% | -85.7% | -99.0% | -99.1% | -24.8% | +1.1% |
| 19 | 35130ns | -93.6% | -93.3% | -92.8% | -85.4% | -99.0% | -99.0% | -24.0% | -0.1% |
| 20 | 35340ns | -93.7% | -93.3% | -92.8% | -85.5% | -99.0% | -99.1% | -23.9% | -0.4% |
| 21 | 35035ns | -93.5% | -93.6% | -92.8% | -85.4% | -99.0% | -99.0% | -21.6% | +0.3% |
| 22 | 34921ns | -93.4% | -93.6% | -92.7% | -85.3% | -99.0% | -99.0% | -22.0% | +1.1% |
| 23 | 34970ns | -93.4% | -93.6% | -92.8% | -85.4% | -99.0% | -99.0% | -23.2% | +0.0% |
| 24 | 34937ns | -93.4% | -93.6% | -92.8% | -85.3% | -99.0% | -99.0% | -21.9% | +1.1% |
| 25 | 35062ns | -93.5% | -93.6% | -92.8% | -85.4% | -99.0% | -99.0% | -23.9% | -0.1% |
| 26 | 35028ns | -93.4% | -93.6% | -92.8% | -85.4% | -99.0% | -99.0% | -24.0% | +0.2% |
| 27 | 35030ns | -93.5% | -93.6% | -92.8% | -85.1% | -99.0% | -99.0% | -23.7% | -0.1% |
| 28 | 36359ns | -93.7% | -93.8% | -93.0% | -85.9% | -99.0% | -99.1% | -26.5% | -3.7% |
| 29 | 35070ns | -93.6% | -93.6% | -92.8% | -85.4% | -99.0% | -99.0% | -22.0% | -0.3% |
| 30 | 34921ns | -93.6% | -93.6% | -92.8% | -85.3% | -99.0% | -99.0% | -21.3% | +0.2% |
| 31 | 35012ns | -93.6% | -93.6% | -92.4% | -85.4% | -99.0% | -99.0% | -23.5% | +0.5% |
| 32 | 34899ns | -93.6% | -93.6% | -92.6% | -85.3% | -99.0% | -99.0% | -23.0% | +0.1% |
| 33 | 35065ns | -93.6% | -93.3% | -92.7% | -85.4% | -99.0% | -99.0% | -23.7% | -0.3% |
| 34 | 34927ns | -93.6% | -93.6% | -92.6% | -85.4% | -99.0% | -99.0% | -23.8% | +0.4% |
| 35 | 34974ns | -93.6% | -93.6% | -92.3% | -85.4% | -99.0% | -99.0% | -23.8% | -0.1% |
| 36 | 35044ns | -93.6% | -93.6% | -92.4% | -85.4% | -99.0% | -99.0% | -23.8% | +2.4% |
| 37 | 34927ns | -93.6% | -93.6% | -92.6% | -85.3% | -99.0% | -99.0% | -23.7% | +0.2% |
| 38 | 35839ns | -93.8% | -93.7% | -92.8% | -85.7% | -99.1% | -99.1% | -24.7% | -0.8% |
| 39 | 35573ns | -93.7% | -93.7% | -92.8% | -85.6% | -99.0% | -99.1% | -21.7% | +0.2% |
| 40 | 35145ns | -93.6% | -93.6% | -92.7% | -85.4% | -99.0% | -99.0% | -23.1% | +0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-iterfold | 0.137 | ok |
| satfold-lanes16 | 0.384 | moderate+ |
| satfold-lanes16-constl | 0.742 | HIGH+ (drift/warm-up) |
| satfold-lanes4-idx | 0.471 | moderate+ |
| satfold-lanes64 | 0.248 | moderate+ |
| satfold-neon | 0.006 | ok |
| satfold-neon8 | 0.834 | HIGH+ (drift/warm-up) |
| satfold-nolaw | 0.232 | moderate+ |
| satfold-seq | 0.499 | moderate+ |

**Consistency summary:**

- **satfold-lanes16**: won 40/40, lost 0/40
- **satfold-lanes16-constl**: won 40/40, lost 0/40
- **satfold-lanes4-idx**: won 40/40, lost 0/40
- **satfold-lanes64**: won 40/40, lost 0/40
- **satfold-neon**: won 40/40, lost 0/40
- **satfold-neon8**: won 40/40, lost 0/40
- **satfold-nolaw**: won 40/40, lost 0/40
- **satfold-seq**: won 8/40, lost 25/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-iterfold | 3.1ns | 35213.5ns | 0.0% |  |
| satfold-lanes16 | 2.8ns | 2250.9ns | 0.1% |  |
| satfold-lanes16-constl | 2.5ns | 2286.6ns | 0.1% |  |
| satfold-lanes4-idx | 2.9ns | 2573.9ns | 0.1% |  |
| satfold-lanes64 | 2.7ns | 5142.1ns | 0.1% |  |
| satfold-neon | 2.1ns | 345.7ns | 0.6% |  |
| satfold-neon8 | 2.0ns | 341.2ns | 0.6% |  |
| satfold-nolaw | 2.8ns | 27016.2ns | 0.0% |  |
| satfold-seq | 2.7ns | 35436.6ns | 0.0% |  |

## Distribution (algo ns)

```
satfold-iterfold (n=40, range 34922.3-35858.4 ns)
  34922.3 |#################################
  34969.1 |########################################
  35015.9 |#################################
  35062.7 |########################################
  35109.5 |#############
  35156.3 |
  35203.1 |
  35249.9 |
  35296.8 |#############
  35343.6 |
  35390.4 |#############
  35437.2 |
  35484.0 |######
  35530.8 |#############
  35577.6 |
  35624.4 |
  35671.2 |
  35718.0 |######
  35764.8 |
  35811.6 |######
  (4 below, 3 above range)

satfold-lanes16 (n=40, range 2231.6-2293.4 ns)
   2231.6 |########################################
   2234.7 |##########
   2237.8 |#########################
   2240.9 |##########
   2244.0 |#######
   2247.1 |
   2250.2 |
   2253.3 |###
   2256.3 |
   2259.4 |
   2262.5 |
   2265.6 |
   2268.7 |#######
   2271.8 |###
   2274.9 |###
   2278.0 |###
   2281.1 |
   2284.2 |
   2287.3 |###
   2290.3 |
  (2 below, 5 above range)

satfold-lanes16-constl (n=40, range 2250.5-2367.6 ns)
   2250.5 |########################################
   2256.4 |#######
   2262.3 |#######
   2268.1 |
   2274.0 |##
   2279.8 |
   2285.7 |
   2291.5 |
   2297.4 |
   2303.2 |
   2309.1 |##
   2315.0 |
   2320.8 |
   2326.7 |
   2332.5 |
   2338.4 |##
   2344.2 |
   2350.1 |
   2355.9 |##
   2361.8 |###########
  (4 below, 4 above range)

satfold-lanes4-idx (n=40, range 2529.1-2688.7 ns)
   2529.1 |########################################
   2537.0 |##
   2545.0 |
   2553.0 |####
   2561.0 |########
   2569.0 |######
   2577.0 |
   2584.9 |
   2592.9 |
   2600.9 |
   2608.9 |
   2616.9 |
   2624.8 |
   2632.8 |
   2640.8 |
   2648.8 |##
   2656.8 |####
   2664.8 |####
   2672.7 |##
   2680.7 |
  (3 below, 3 above range)

satfold-lanes64 (n=40, range 5118.0-5226.1 ns)
   5118.0 |########################################
   5123.4 |############
   5128.8 |
   5134.2 |#
   5139.6 |
   5145.0 |
   5150.4 |
   5155.8 |
   5161.2 |
   5166.6 |#
   5172.0 |
   5177.4 |
   5182.9 |#
   5188.3 |
   5193.7 |
   5199.1 |
   5204.5 |
   5209.9 |
   5215.3 |#
   5220.7 |
  (3 below, 4 above range)

satfold-neon (n=40, range 336.5-365.7 ns)
    336.5 |############################
    338.0 |########################################
    339.4 |#################
    340.9 |###########
    342.4 |############################
    343.8 |#################
    345.3 |#################
    346.7 |#####
    348.2 |
    349.7 |
    351.1 |###########
    352.6 |
    354.0 |
    355.5 |###########
    357.0 |#####
    358.4 |
    359.9 |
    361.4 |#####
    362.8 |
    364.3 |
  (3 below, 2 above range)

satfold-neon8 (n=40, range 334.3-353.6 ns)
    334.3 |#################
    335.3 |########################################
    336.3 |########
    337.2 |########
    338.2 |
    339.1 |##########################
    340.1 |########
    341.1 |####
    342.0 |
    343.0 |
    344.0 |
    344.9 |
    345.9 |
    346.8 |
    347.8 |
    348.8 |####
    349.7 |####
    350.7 |
    351.7 |#################
    352.6 |########
  (3 below, 3 above range)

satfold-nolaw (n=40, range 26668.1-27539.2 ns)
  26668.1 |########################
  26711.7 |########################################
  26755.2 |########################
  26798.8 |
  26842.3 |################################
  26885.9 |########
  26929.4 |################
  26973.0 |########
  27016.5 |################
  27060.1 |
  27103.7 |################
  27147.2 |########
  27190.8 |################
  27234.3 |########################
  27277.9 |
  27321.4 |########
  27365.0 |
  27408.5 |########
  27452.1 |################
  27495.7 |########
  (4 below, 2 above range)

satfold-seq (n=40, range 34966.2-36307.9 ns)
  34966.2 |########################################
  35033.2 |##########
  35100.3 |####################
  35167.4 |##########
  35234.5 |###############
  35301.6 |#####
  35368.7 |
  35435.7 |#####
  35502.8 |##########
  35569.9 |
  35637.0 |###############
  35704.1 |#####
  35771.2 |
  35838.3 |#####
  35905.3 |##########
  35972.4 |#####
  36039.5 |
  36106.6 |#####
  36173.7 |#####
  36240.8 |#####
  (4 below, 2 above range)

```

## Diagnostics

- **satfold-lanes16-constl**: autocorrelation=0.74 (measurement drift or warm-up artifact)
- **satfold-neon8**: autocorrelation=0.83 (measurement drift or warm-up artifact)
