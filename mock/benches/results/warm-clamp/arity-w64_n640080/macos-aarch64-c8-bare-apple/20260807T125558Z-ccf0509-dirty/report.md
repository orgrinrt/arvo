# Clamping fold at 64 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-min-lanes beats baseline by 60% (significant)

warm-clamp-min-lanes is -3.04 us (60%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-minimum is an outlier: 2.5x slower than the field

warm-clamp-minimum (5.08 us) is 2.5x the fastest (2.04 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-minimum shows warm-up / thermal drift (autocorr +0.68)

warm-clamp-minimum's per-pass series has lag-1 autocorrelation +0.68, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-min-lanes, warm-clamp-accfit, warm-clamp-head, warm-clamp-accfit-dyn} vs {warm-clamp-acc64, warm-clamp-minimum} (79% apart)

The field splits into a fast tier {warm-clamp-min-lanes, warm-clamp-accfit, warm-clamp-head, warm-clamp-accfit-dyn} and a slow tier {warm-clamp-acc64, warm-clamp-minimum} with a 79% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader warm-clamp-min-lanes vs stability leader warm-clamp-head (+6% speed for 3.2x steadier)

warm-clamp-min-lanes is fastest (2.04 us, CV 4.3%); warm-clamp-head gives up 6.1% median for 3.2x lower variance (CV 1.3%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-clamp-minimum's edge over baseline is significant but tiny (10 ns, 0.20%)

warm-clamp-minimum differs from baseline warm-clamp-acc64 by 10 ns (0.20%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-clamp-min-lanes** at 2035.2 ns median (-59.7% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 2.50x (fastest 2035.2 ns, slowest 5082.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 5163ns | 5121ns | 5041ns | 5127ns | 5393ns | base |
| warm-clamp-accfit | 2225ns | 2201ns | 2174ns | 2200ns | 2351ns | -56.91% |
| warm-clamp-accfit-dyn | 2944ns | 2888ns | 2841ns | 2895ns | 3196ns | -42.97% |
| warm-clamp-head | 2268ns | 2251ns | 2242ns | 2260ns | 2316ns | -56.08% |
| warm-clamp-min-lanes | 2109ns | 2102ns | 2018ns | 2094ns | 2244ns | -59.15% |
| warm-clamp-minimum | 5292ns | 5149ns | 5041ns | 5191ns | 5847ns | +2.50% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 5095ns | 4978ns | 5315ns | base | 1.608 |
| warm-clamp-accfit | 2160ns | 2111ns | 2284ns | -57.60% | 3.792 |
| warm-clamp-accfit-dyn | 2885ns | 2783ns | 3132ns | -43.37% | 2.839 |
| warm-clamp-head | 2176ns | 2152ns | 2222ns | -57.30% | 3.765 |
| warm-clamp-min-lanes | 2044ns | 1955ns | 2178ns | -59.88% | 4.008 |
| warm-clamp-minimum | 5223ns | 4977ns | 5772ns | +2.52% | 1.568 |

## Performance model

- Peak throughput: **4.191 Gops/s** (warm-clamp-min-lanes; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 1.620 | 38.7% |
| warm-clamp-accfit | 3.837 | 91.6% |
| warm-clamp-accfit-dyn | 2.895 | 69.1% |
| warm-clamp-head | 3.795 | 90.6% |
| warm-clamp-min-lanes | 4.025 | 96.0% |
| warm-clamp-minimum | 1.612 | 38.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 5163ns | 5163ns | base |
| warm-clamp-accfit | 2225ns | 2225ns | -56.91% |
| warm-clamp-accfit-dyn | 2944ns | 2944ns | -42.97% |
| warm-clamp-head | 2268ns | 2268ns | -56.08% |
| warm-clamp-min-lanes | 2109ns | 2109ns | -59.15% |
| warm-clamp-minimum | 5292ns | 5292ns | +2.50% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 5056ns | base | --- | [5050, 5074] | --- | --- | --- | --- |
| warm-clamp-accfit | 2135ns | -2924.3ns (-57.8%) | [-2950, -2906]ns | [2116, 2149] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 2829ns | -2216.6ns (-43.8%) | [-2248, -2189]ns | [2792, 2885] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 2159ns | -2873.9ns (-56.8%) | [-2901, -2856]ns | [2157, 2184] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 2035ns | -3015.9ns (-59.6%) | [-3046, -3005]ns | [1990, 2047] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 5082ns | no significant difference | [-1, +93]ns | [5064, 5200] | no | 0.1081 | 0.1081 | 1 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 5050ns | -58.2% | -43.1% | -57.3% | -57.3% | -1.5% |
| 2 | 5089ns | -58.2% | -44.2% | -55.8% | -58.2% | -2.2% |
| 3 | 5139ns | -58.9% | -34.7% | -58.0% | -56.9% | -3.3% |
| 4 | 5058ns | -58.3% | -43.9% | -57.4% | -57.3% | +0.1% |
| 5 | 4975ns | -57.5% | -44.1% | -56.6% | -56.7% | -0.0% |
| 6 | 4972ns | -57.5% | -43.9% | -55.4% | -57.2% | +0.0% |
| 7 | 5028ns | -58.0% | -44.5% | -57.1% | -57.6% | -1.1% |
| 8 | 5047ns | -58.1% | -44.4% | -57.2% | -57.8% | -0.7% |
| 9 | 4995ns | -57.5% | -44.3% | -56.8% | -57.4% | +4.2% |
| 10 | 5052ns | -58.1% | -44.2% | -57.3% | -57.9% | +1.7% |
| 11 | 5093ns | -58.0% | -38.9% | -57.7% | -60.6% | -0.2% |
| 12 | 5050ns | -56.7% | -38.3% | -57.3% | -59.3% | +0.5% |
| 13 | 5299ns | -58.4% | -43.3% | -59.2% | -62.3% | -3.3% |
| 14 | 5226ns | -57.8% | -44.8% | -58.5% | -62.4% | -3.1% |
| 15 | 5374ns | -60.4% | -46.3% | -59.9% | -63.8% | -3.3% |
| 16 | 5735ns | -63.2% | -49.7% | -62.4% | -66.2% | -12.3% |
| 17 | 5016ns | -57.8% | -42.4% | -57.1% | -61.2% | +1.2% |
| 18 | 4997ns | -57.4% | -42.2% | -56.5% | -60.3% | -0.2% |
| 19 | 5050ns | -58.2% | -42.9% | -56.5% | -59.7% | +0.1% |
| 20 | 5217ns | -59.6% | -44.7% | -58.7% | -54.9% | -4.5% |
| 21 | 4970ns | -53.7% | -44.0% | -56.6% | -60.6% | +21.4% |
| 22 | 4986ns | -55.5% | -44.2% | -56.9% | -60.7% | +21.1% |
| 23 | 4976ns | -55.3% | -44.0% | -56.8% | -60.6% | +21.6% |
| 24 | 4973ns | -53.0% | -44.1% | -55.6% | -60.4% | +17.3% |
| 25 | 5038ns | -49.2% | -44.2% | -57.1% | -59.4% | +0.9% |
| 26 | 4973ns | -55.4% | -44.0% | -56.7% | -60.4% | +2.9% |
| 27 | 5052ns | -58.2% | -44.7% | -57.4% | -61.0% | +12.0% |
| 28 | 5092ns | -58.5% | -45.2% | -57.7% | -61.4% | +11.0% |
| 29 | 5103ns | -58.6% | -45.5% | -57.8% | -61.5% | +10.8% |
| 30 | 5098ns | -58.5% | -44.0% | -57.8% | -61.5% | +3.1% |
| 31 | 5057ns | -57.5% | -38.5% | -56.6% | -60.3% | +0.0% |
| 32 | 5059ns | -57.6% | -38.3% | -56.7% | -59.8% | +0.1% |
| 33 | 5058ns | -57.6% | -38.4% | -56.2% | -60.1% | +0.5% |
| 34 | 5318ns | -59.6% | -41.4% | -58.7% | -61.7% | -4.5% |
| 35 | 5056ns | -57.4% | -42.3% | -56.5% | -59.7% | +1.7% |
| 36 | 5057ns | -57.0% | -44.8% | -56.6% | -59.5% | +3.6% |
| 37 | 5208ns | -58.7% | -46.3% | -56.7% | -60.9% | +0.6% |
| 38 | 5145ns | -58.2% | -45.7% | -57.3% | -59.9% | +1.9% |
| 39 | 5056ns | -57.5% | -44.9% | -56.5% | -59.7% | +3.6% |
| 40 | 5055ns | -57.5% | -44.8% | -55.9% | -59.7% | +3.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.331 | moderate+ |
| warm-clamp-accfit | 0.538 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit-dyn | 0.456 | moderate+ |
| warm-clamp-head | 0.289 | moderate+ |
| warm-clamp-min-lanes | 0.480 | moderate+ |
| warm-clamp-minimum | 0.682 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-clamp-accfit**: won 40/40, lost 0/40
- **warm-clamp-accfit-dyn**: won 40/40, lost 0/40
- **warm-clamp-head**: won 40/40, lost 0/40
- **warm-clamp-min-lanes**: won 40/40, lost 0/40
- **warm-clamp-minimum**: won 13/40, lost 22/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 2.9ns | 5094.8ns | 0.1% |  |
| warm-clamp-accfit | 2.6ns | 2160.4ns | 0.1% |  |
| warm-clamp-accfit-dyn | 3.0ns | 2885.1ns | 0.1% |  |
| warm-clamp-head | 2.8ns | 2175.7ns | 0.1% |  |
| warm-clamp-min-lanes | 2.4ns | 2043.9ns | 0.1% |  |
| warm-clamp-minimum | 2.9ns | 5223.3ns | 0.1% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 4977.6-5315.3 ns)
   4977.6 |##
   4994.5 |#####
   5011.4 |#####
   5028.3 |##
   5045.2 |########################################
   5062.0 |
   5078.9 |########
   5095.8 |#####
   5112.7 |
   5129.6 |#####
   5146.4 |
   5163.3 |
   5180.2 |
   5197.1 |##
   5214.0 |#####
   5230.9 |
   5247.7 |
   5264.6 |
   5281.5 |
   5298.4 |##
  (6 below, 3 above range)

warm-clamp-accfit (n=40, range 2110.9-2283.7 ns)
   2110.9 |########################################
   2119.6 |##########
   2128.2 |###
   2136.9 |###
   2145.5 |##############################
   2154.1 |
   2162.8 |
   2171.4 |###
   2180.1 |###
   2188.7 |
   2197.3 |###
   2206.0 |###
   2214.6 |######
   2223.3 |###
   2231.9 |
   2240.5 |
   2249.2 |
   2257.8 |
   2266.5 |
   2275.1 |
  (4 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 2783.5-3132.4 ns)
   2783.5 |########################################
   2800.9 |######
   2818.4 |###
   2835.8 |######
   2853.3 |###
   2870.7 |#######################
   2888.2 |###
   2905.6 |###
   2923.1 |
   2940.5 |
   2957.9 |
   2975.4 |
   2992.8 |###
   3010.3 |
   3027.7 |
   3045.2 |
   3062.6 |
   3080.1 |
   3097.5 |##########
   3115.0 |##########
  (5 below, 1 above range)

warm-clamp-head (n=40, range 2152.1-2222.0 ns)
   2152.1 |########################################
   2155.6 |########################################
   2159.1 |###############
   2162.6 |
   2166.1 |
   2169.5 |#####
   2173.0 |#####
   2176.5 |
   2180.0 |
   2183.5 |
   2187.0 |
   2190.5 |#####
   2194.0 |####################
   2197.5 |###############
   2201.0 |
   2204.5 |
   2208.0 |#####
   2211.5 |
   2215.0 |##########
   2218.5 |
  (5 below, 3 above range)

warm-clamp-min-lanes (n=40, range 1954.8-2178.1 ns)
   1954.8 |########################################
   1966.0 |#################
   1977.1 |#####
   1988.3 |#####
   1999.5 |###########
   2010.6 |#####
   2021.8 |#####
   2033.0 |##################################
   2044.1 |###########
   2055.3 |###########
   2066.5 |
   2077.6 |
   2088.8 |
   2100.0 |
   2111.1 |
   2122.3 |##################################
   2133.5 |
   2144.6 |###########
   2155.8 |#####
   2167.0 |
  (3 below, 2 above range)

warm-clamp-minimum (n=40, range 4976.7-5772.1 ns)
   4976.7 |################
   5016.5 |########
   5056.3 |########################################
   5096.0 |########
   5135.8 |########
   5175.6 |########
   5215.3 |####################
   5255.1 |####
   5294.9 |
   5334.6 |
   5374.4 |
   5414.2 |
   5453.9 |
   5493.7 |
   5533.5 |
   5573.2 |
   5613.0 |
   5652.8 |############
   5692.6 |
   5732.3 |
  (5 below, 4 above range)

```

## Diagnostics

- **warm-clamp-accfit**: autocorrelation=0.54 (measurement drift or warm-up artifact)
- **warm-clamp-minimum**: autocorrelation=0.68 (measurement drift or warm-up artifact)
