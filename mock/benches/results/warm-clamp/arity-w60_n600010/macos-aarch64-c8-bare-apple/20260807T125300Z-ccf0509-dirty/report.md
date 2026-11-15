# Clamping fold at 60 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-accfit-dyn is an outlier: 4.3x slower than the field

warm-clamp-accfit-dyn (4.30 us) is 4.3x the fastest (1.00 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-clamp-minimum, warm-clamp-accfit) are a dead heat (<1%)

warm-clamp-minimum (1.00 us) and warm-clamp-accfit (1.00 us) differ by 0.17%, inside the noise, even though the wider field spreads 328.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-clamp-head shows warm-up / thermal drift (autocorr +0.87)

warm-clamp-head's per-pass series has lag-1 autocorrelation +0.87, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-minimum, warm-clamp-accfit, warm-clamp-acc64} vs {warm-clamp-min-lanes, warm-clamp-head, warm-clamp-accfit-dyn} (77% apart)

The field splits into a fast tier {warm-clamp-minimum, warm-clamp-accfit, warm-clamp-acc64} and a slow tier {warm-clamp-min-lanes, warm-clamp-head, warm-clamp-accfit-dyn} with a 77% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.3x the fastest

Fastest warm-clamp-minimum (1.00 us) to slowest warm-clamp-accfit-dyn (4.30 us): 4.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-clamp-minimum's edge over baseline is significant but tiny (-17 ns, 1.60%)

warm-clamp-minimum differs from baseline warm-clamp-acc64 by -17 ns (1.60%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-clamp-minimum** at 1003.3 ns median (-3.5% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 4.28x (fastest 1003.3 ns, slowest 4297.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 1130ns | 1105ns | 1047ns | 1111ns | 1270ns | base |
| warm-clamp-accfit | 1073ns | 1067ns | 1045ns | 1068ns | 1113ns | -5.09% |
| warm-clamp-accfit-dyn | 4378ns | 4360ns | 4314ns | 4359ns | 4499ns | +287.35% |
| warm-clamp-head | 3296ns | 3141ns | 3088ns | 3207ns | 3772ns | +191.60% |
| warm-clamp-min-lanes | 1927ns | 1909ns | 1862ns | 1919ns | 2016ns | +70.47% |
| warm-clamp-minimum | 1068ns | 1066ns | 1061ns | 1066ns | 1084ns | -5.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 1063ns | 986ns | 1195ns | base | 7.704 |
| warm-clamp-accfit | 1010ns | 985ns | 1048ns | -5.01% | 8.111 |
| warm-clamp-accfit-dyn | 4319ns | 4257ns | 4438ns | +306.17% | 1.897 |
| warm-clamp-head | 3197ns | 2998ns | 3655ns | +200.68% | 2.562 |
| warm-clamp-min-lanes | 1862ns | 1800ns | 1947ns | +75.11% | 4.400 |
| warm-clamp-minimum | 1006ns | 1001ns | 1020ns | -5.39% | 8.143 |

## Performance model

- Peak throughput: **8.320 Gops/s** (warm-clamp-accfit; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 7.880 | 94.7% |
| warm-clamp-accfit | 8.151 | 98.0% |
| warm-clamp-accfit-dyn | 1.906 | 22.9% |
| warm-clamp-head | 2.687 | 32.3% |
| warm-clamp-min-lanes | 4.441 | 53.4% |
| warm-clamp-minimum | 8.165 | 98.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 1130ns | 1130ns | base |
| warm-clamp-accfit | 1073ns | 1073ns | -5.09% |
| warm-clamp-accfit-dyn | 4378ns | 4378ns | +287.35% |
| warm-clamp-head | 3296ns | 3296ns | +191.60% |
| warm-clamp-min-lanes | 1927ns | 1927ns | +70.47% |
| warm-clamp-minimum | 1068ns | 1068ns | -5.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 1040ns | base | --- | [1009, 1042] | --- | --- | --- | --- |
| warm-clamp-accfit | 1005ns | -31.2ns (-3.0%) | [-38, -15]ns | [1000, 1009] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 4298ns | +3275.8ns (+315.1%) | [+3272, +3286]ns | [4261, 4341] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 3048ns | +2014.8ns (+193.8%) | [+2010, +2069]ns | [3006, 3059] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 1845ns | +840.0ns (+80.8%) | [+809, +854]ns | [1831, 1881] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 1003ns | -35.0ns (-3.4%) | [-38, -9]ns | [1001, 1005] | YES | 0.0166 | 0.0166 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 1197ns | -13.5% | +255.5% | +204.8% | +50.3% | -16.4% |
| 2 | 1193ns | -12.9% | +256.7% | +205.7% | +50.7% | -16.2% |
| 3 | 1193ns | -13.3% | +256.8% | +205.5% | +50.6% | -14.1% |
| 4 | 1196ns | -13.1% | +256.0% | +210.3% | +50.7% | -16.3% |
| 5 | 1194ns | -13.3% | +256.5% | +205.4% | +56.4% | -16.2% |
| 6 | 1195ns | -13.2% | +259.7% | +205.1% | +52.8% | -16.0% |
| 7 | 1194ns | -13.1% | +267.3% | +205.2% | +52.9% | -15.1% |
| 8 | 1196ns | -13.3% | +274.4% | +204.9% | +50.4% | -15.8% |
| 9 | 1194ns | -13.3% | +275.1% | +205.6% | +50.7% | -13.4% |
| 10 | 1197ns | -13.3% | +278.6% | +204.7% | +52.1% | -13.3% |
| 11 | 1040ns | -3.3% | +317.3% | +204.4% | +76.4% | -3.1% |
| 12 | 1040ns | -3.2% | +317.3% | +198.9% | +77.2% | -3.6% |
| 13 | 1052ns | -4.6% | +312.9% | +189.6% | +75.0% | -4.4% |
| 14 | 1038ns | -2.2% | +318.3% | +193.6% | +79.1% | -3.4% |
| 15 | 1040ns | -3.0% | +317.3% | +193.2% | +78.9% | -3.4% |
| 16 | 1041ns | -3.5% | +316.9% | +192.6% | +81.8% | -3.6% |
| 17 | 1046ns | -3.6% | +316.5% | +191.7% | +81.5% | -4.1% |
| 18 | 1042ns | -3.8% | +317.5% | +192.4% | +81.8% | -3.6% |
| 19 | 1042ns | -3.4% | +320.4% | +192.4% | +82.5% | -3.2% |
| 20 | 1041ns | -3.0% | +315.2% | +193.4% | +82.8% | -3.7% |
| 21 | 988ns | -0.2% | +331.6% | +210.1% | +97.6% | +1.6% |
| 22 | 986ns | +0.0% | +332.6% | +209.2% | +84.7% | +2.0% |
| 23 | 985ns | -0.1% | +332.4% | +210.0% | +89.7% | +1.6% |
| 24 | 985ns | +0.2% | +332.4% | +204.7% | +84.9% | +2.2% |
| 25 | 987ns | +0.1% | +331.7% | +204.5% | +85.7% | +1.5% |
| 26 | 986ns | -0.3% | +331.5% | +203.9% | +87.9% | +2.0% |
| 27 | 985ns | +0.2% | +332.5% | +204.5% | +85.8% | +1.9% |
| 28 | 987ns | -0.1% | +331.6% | +204.2% | +86.8% | +1.6% |
| 29 | 987ns | -0.3% | +331.4% | +203.7% | +82.5% | +3.7% |
| 30 | 987ns | -0.2% | +332.0% | +203.8% | +82.9% | +1.8% |
| 31 | 1000ns | -1.3% | +347.7% | +199.6% | +99.1% | +0.1% |
| 32 | 1002ns | -2.0% | +340.9% | +199.1% | +84.3% | -0.0% |
| 33 | 1154ns | -13.9% | +274.4% | +160.4% | +58.9% | -13.3% |
| 34 | 1038ns | -2.9% | +316.5% | +189.1% | +87.4% | -3.4% |
| 35 | 1021ns | +10.1% | +323.4% | +194.5% | +85.6% | -2.0% |
| 36 | 1022ns | -2.0% | +320.2% | +198.3% | +85.4% | -2.2% |
| 37 | 1039ns | -3.7% | +313.4% | +188.7% | +82.3% | -3.7% |
| 38 | 1017ns | -1.6% | +318.8% | +194.9% | +100.5% | -1.7% |
| 39 | 1002ns | -0.2% | +324.9% | +199.2% | +94.2% | -0.1% |
| 40 | 1000ns | +0.5% | +325.6% | +246.7% | +89.5% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.836 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit | 0.447 | moderate+ |
| warm-clamp-accfit-dyn | 0.686 | HIGH+ (drift/warm-up) |
| warm-clamp-head | 0.875 | HIGH+ (drift/warm-up) |
| warm-clamp-min-lanes | 0.419 | moderate+ |
| warm-clamp-minimum | 0.326 | moderate+ |

**Consistency summary:**

- **warm-clamp-accfit**: won 32/40, lost 5/40
- **warm-clamp-accfit-dyn**: won 0/40, lost 40/40
- **warm-clamp-head**: won 0/40, lost 40/40
- **warm-clamp-min-lanes**: won 0/40, lost 40/40
- **warm-clamp-minimum**: won 26/40, lost 11/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 2.8ns | 1063.3ns | 0.3% |  |
| warm-clamp-accfit | 2.5ns | 1010.0ns | 0.2% |  |
| warm-clamp-accfit-dyn | 2.6ns | 4318.7ns | 0.1% |  |
| warm-clamp-head | 2.2ns | 3197.1ns | 0.1% |  |
| warm-clamp-min-lanes | 2.7ns | 1861.9ns | 0.1% |  |
| warm-clamp-minimum | 2.5ns | 1006.0ns | 0.3% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 986.0-1195.4 ns)
    986.0 |##########################
    996.5 |#################
   1006.9 |####
   1017.4 |########
   1027.9 |########
   1038.3 |########################################
   1048.8 |####
   1059.3 |
   1069.8 |
   1080.2 |
   1090.7 |
   1101.2 |
   1111.7 |
   1122.1 |
   1132.6 |
   1143.1 |
   1153.5 |####
   1164.0 |
   1174.5 |
   1185.0 |##########################
  (4 below, 4 above range)

warm-clamp-accfit (n=40, range 984.6-1048.5 ns)
    984.6 |###################################
    987.8 |#####
    991.0 |
    994.2 |#####
    997.4 |#####
   1000.6 |#########################
   1003.8 |####################
   1007.0 |#########################
   1010.2 |
   1013.3 |#####
   1016.5 |
   1019.7 |
   1022.9 |
   1026.1 |
   1029.3 |
   1032.5 |#####
   1035.7 |########################################
   1038.9 |#####
   1042.1 |
   1045.3 |
  (4 below, 1 above range)

warm-clamp-accfit-dyn (n=40, range 4256.8-4438.0 ns)
   4256.8 |########################################
   4265.9 |
   4274.9 |
   4284.0 |
   4293.1 |########
   4302.1 |
   4311.2 |
   4320.2 |###########
   4329.3 |
   4338.3 |#################
   4347.4 |#####
   4356.5 |
   4365.5 |
   4374.6 |##
   4383.6 |##
   4392.7 |
   4401.8 |
   4410.8 |##
   4419.9 |
   4428.9 |
  (4 below, 4 above range)

warm-clamp-head (n=40, range 2997.9-3655.1 ns)
   2997.9 |####################################
   3030.7 |########################################
   3063.6 |
   3096.4 |###
   3129.3 |
   3162.2 |###
   3195.0 |
   3227.9 |
   3260.7 |
   3293.6 |
   3326.5 |
   3359.3 |
   3392.2 |
   3425.0 |
   3457.9 |###
   3490.8 |
   3523.6 |
   3556.5 |
   3589.3 |
   3622.2 |##############################
  (4 below, 1 above range)

warm-clamp-min-lanes (n=40, range 1800.4-1946.7 ns)
   1800.4 |########################
   1807.7 |
   1815.0 |########################
   1822.3 |################
   1829.6 |################################
   1836.9 |########################
   1844.3 |########
   1851.6 |########
   1858.9 |################
   1866.2 |################
   1873.5 |
   1880.8 |
   1888.2 |########################################
   1895.5 |################################
   1902.8 |
   1910.1 |
   1917.4 |
   1924.7 |
   1932.1 |
   1939.4 |################
  (5 below, 3 above range)

warm-clamp-minimum (n=40, range 1000.5-1019.6 ns)
   1000.5 |########################################
   1001.5 |#######
   1002.4 |#########################
   1003.4 |#######
   1004.3 |#######
   1005.3 |##############
   1006.2 |#######
   1007.2 |#######
   1008.1 |
   1009.1 |
   1010.0 |
   1011.0 |
   1011.9 |
   1012.9 |###
   1013.9 |
   1014.8 |
   1015.8 |
   1016.7 |
   1017.7 |
   1018.6 |
  (3 below, 4 above range)

```

## Diagnostics

- **warm-clamp-acc64**: autocorrelation=0.84 (measurement drift or warm-up artifact)
- **warm-clamp-accfit-dyn**: autocorrelation=0.69 (measurement drift or warm-up artifact)
- **warm-clamp-head**: autocorrelation=0.87 (measurement drift or warm-up artifact)
