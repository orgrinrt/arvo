# Clamping fold at 16 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-minimum beats baseline by 71% (significant)

warm-clamp-minimum is -615 ns (71%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-accfit-dyn is an outlier: 17.1x slower than the field

warm-clamp-accfit-dyn (4.31 us) is 17.1x the fastest (252 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-min-lanes shows warm-up / thermal drift (autocorr +0.87)

warm-clamp-min-lanes's per-pass series has lag-1 autocorrelation +0.87, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-min-lanes, warm-clamp-minimum, warm-clamp-accfit, warm-clamp-head, warm-clamp-acc64} vs {warm-clamp-accfit-dyn} (398% apart)

The field splits into a fast tier {warm-clamp-min-lanes, warm-clamp-minimum, warm-clamp-accfit, warm-clamp-head, warm-clamp-acc64} and a slow tier {warm-clamp-accfit-dyn} with a 398% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 17.1x the fastest

Fastest warm-clamp-min-lanes (252 ns) to slowest warm-clamp-accfit-dyn (4.31 us): 17.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-clamp-min-lanes** at 251.7 ns median (-70.9% vs baseline)
- 4 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 17.14x (fastest 251.7 ns, slowest 4313.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 950ns | 930ns | 884ns | 928ns | 1083ns | base |
| warm-clamp-accfit | 398ns | 395ns | 392ns | 396ns | 406ns | -58.16% |
| warm-clamp-accfit-dyn | 4621ns | 4374ns | 4315ns | 4455ns | 5423ns | +386.31% |
| warm-clamp-head | 608ns | 566ns | 556ns | 583ns | 735ns | -36.03% |
| warm-clamp-min-lanes | 325ns | 356ns | 281ns | 327ns | 363ns | -65.81% |
| warm-clamp-minimum | 329ns | 345ns | 272ns | 338ns | 361ns | -65.36% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 885ns | 824ns | 1009ns | base | 9.256 |
| warm-clamp-accfit | 337ns | 334ns | 345ns | -61.92% | 24.309 |
| warm-clamp-accfit-dyn | 4557ns | 4255ns | 5349ns | +414.84% | 1.798 |
| warm-clamp-head | 541ns | 496ns | 653ns | -38.83% | 15.131 |
| warm-clamp-min-lanes | 252ns | 219ns | 282ns | -71.57% | 32.555 |
| warm-clamp-minimum | 257ns | 212ns | 282ns | -70.99% | 31.902 |

## Performance model

- Peak throughput: **38.616 Gops/s** (warm-clamp-minimum; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 9.457 | 24.5% |
| warm-clamp-accfit | 24.468 | 63.4% |
| warm-clamp-accfit-dyn | 1.899 | 4.9% |
| warm-clamp-head | 16.228 | 42.0% |
| warm-clamp-min-lanes | 32.553 | 84.3% |
| warm-clamp-minimum | 30.505 | 79.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 950ns | 950ns | base |
| warm-clamp-accfit | 398ns | 398ns | -58.16% |
| warm-clamp-accfit-dyn | 4621ns | 4621ns | +386.31% |
| warm-clamp-head | 608ns | 608ns | -36.03% |
| warm-clamp-min-lanes | 325ns | 325ns | -65.81% |
| warm-clamp-minimum | 329ns | 329ns | -65.36% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 866ns | base | --- | [826, 871] | --- | --- | --- | --- |
| warm-clamp-accfit | 335ns | -532.5ns (-61.5%) | [-536, -491]ns | [335, 335] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 4314ns | +3439.4ns (+397.0%) | [+3391, +3464]ns | [4278, 4379] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 505ns | -320.0ns (-36.9%) | [-324, -312]ns | [504, 516] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 252ns | -599.0ns (-69.1%) | [-606, -588]ns | [225, 280] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 269ns | -612.5ns (-70.7%) | [-616, -610]ns | [258, 279] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 1000ns | -66.3% | +331.9% | -50.4% | -77.6% | -71.9% |
| 2 | 1000ns | -66.6% | +346.4% | -50.4% | -77.6% | -71.7% |
| 3 | 1000ns | -66.4% | +339.9% | -50.4% | -77.6% | -71.9% |
| 4 | 1000ns | -66.5% | +332.1% | -50.5% | -77.5% | -72.0% |
| 5 | 1066ns | -68.5% | +315.2% | -53.3% | -78.9% | -73.6% |
| 6 | 1001ns | -66.6% | +330.2% | -50.6% | -77.7% | -71.8% |
| 7 | 1000ns | -66.6% | +334.6% | -50.4% | -77.7% | -71.9% |
| 8 | 999ns | -62.8% | +334.8% | -50.0% | -77.6% | -72.0% |
| 9 | 1002ns | -66.6% | +325.1% | -50.3% | -77.6% | -72.0% |
| 10 | 999ns | -66.4% | +325.9% | -50.3% | -77.5% | -71.9% |
| 11 | 866ns | -61.4% | +391.6% | -24.9% | -67.6% | -74.4% |
| 12 | 867ns | -61.5% | +390.7% | -24.9% | -67.8% | -70.6% |
| 13 | 870ns | -61.6% | +413.9% | -25.0% | -67.6% | -70.4% |
| 14 | 868ns | -61.5% | +390.8% | -24.6% | -67.6% | -70.0% |
| 15 | 871ns | -61.6% | +391.7% | -25.0% | -67.6% | -70.4% |
| 16 | 868ns | -61.5% | +390.4% | -24.7% | -67.4% | -70.1% |
| 17 | 872ns | -61.6% | +388.1% | -25.1% | -68.0% | -70.2% |
| 18 | 865ns | -61.5% | +404.0% | -24.6% | -67.5% | -70.2% |
| 19 | 868ns | -61.6% | +394.1% | -25.1% | -67.7% | -70.3% |
| 20 | 870ns | -61.6% | +389.8% | -24.9% | -67.7% | -70.4% |
| 21 | 828ns | -59.5% | +584.1% | -37.1% | -65.8% | -66.3% |
| 22 | 824ns | -59.3% | +580.2% | -35.9% | -66.4% | -66.1% |
| 23 | 827ns | -59.4% | +574.6% | -36.8% | -65.9% | -66.5% |
| 24 | 825ns | -59.5% | +530.1% | -38.9% | -66.1% | -66.4% |
| 25 | 900ns | -62.9% | +473.8% | -43.6% | -68.7% | -69.0% |
| 26 | 823ns | -59.3% | +531.8% | -38.8% | -65.7% | -66.0% |
| 27 | 826ns | -59.5% | +525.1% | -38.9% | -66.0% | -66.2% |
| 28 | 824ns | -59.3% | +526.7% | -38.6% | -66.0% | -66.2% |
| 29 | 825ns | -59.6% | +525.6% | -39.1% | -66.2% | -65.9% |
| 30 | 825ns | -59.4% | +533.6% | -39.5% | -66.0% | -65.8% |
| 31 | 898ns | -62.0% | +375.0% | -43.7% | -76.0% | -76.4% |
| 32 | 826ns | -58.8% | +417.2% | -38.9% | -73.9% | -74.3% |
| 33 | 825ns | -58.8% | +419.2% | -38.9% | -73.6% | -74.2% |
| 34 | 822ns | -58.3% | +421.4% | -38.7% | -73.7% | -74.1% |
| 35 | 823ns | -58.4% | +418.5% | -38.8% | -72.7% | -74.2% |
| 36 | 826ns | -58.8% | +423.6% | -38.5% | -72.6% | -74.5% |
| 37 | 826ns | -59.0% | +418.9% | -38.8% | -72.8% | -74.1% |
| 38 | 827ns | -59.0% | +416.5% | -38.2% | -73.2% | -74.4% |
| 39 | 825ns | -58.8% | +416.5% | -36.8% | -73.1% | -74.1% |
| 40 | 828ns | -59.0% | +413.3% | -39.3% | -72.9% | -74.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.848 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit | 0.050 | ok |
| warm-clamp-accfit-dyn | 0.792 | HIGH+ (drift/warm-up) |
| warm-clamp-head | 0.861 | HIGH+ (drift/warm-up) |
| warm-clamp-min-lanes | 0.868 | HIGH+ (drift/warm-up) |
| warm-clamp-minimum | 0.802 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-clamp-accfit**: won 40/40, lost 0/40
- **warm-clamp-accfit-dyn**: won 0/40, lost 40/40
- **warm-clamp-head**: won 40/40, lost 0/40
- **warm-clamp-min-lanes**: won 40/40, lost 0/40
- **warm-clamp-minimum**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 2.7ns | 885.1ns | 0.3% |  |
| warm-clamp-accfit | 2.2ns | 337.0ns | 0.6% |  |
| warm-clamp-accfit-dyn | 2.9ns | 4556.6ns | 0.1% |  |
| warm-clamp-head | 2.5ns | 541.4ns | 0.5% |  |
| warm-clamp-min-lanes | 3.0ns | 251.6ns | 1.2% |  |
| warm-clamp-minimum | 2.8ns | 256.8ns | 1.1% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 823.6-1008.6 ns)
    823.6 |########################################
    832.9 |
    842.1 |
    851.4 |
    860.6 |################
    869.9 |##########
    879.1 |
    888.4 |##
    897.6 |##
    906.9 |
    916.1 |
    925.4 |
    934.6 |
    943.9 |
    953.1 |
    962.4 |
    971.6 |
    980.9 |
    990.1 |#####
    999.4 |##################
  (3 below, 1 above range)

warm-clamp-accfit (n=40, range 333.7-344.8 ns)
    333.7 |########################################
    334.3 |########################################
    334.8 |#############
    335.4 |#################
    335.9 |####
    336.5 |
    337.1 |####
    337.6 |
    338.2 |
    338.7 |#############
    339.3 |
    339.8 |########
    340.4 |########
    340.9 |####
    341.5 |
    342.1 |########
    342.6 |
    343.2 |
    343.7 |
    344.3 |
  (2 below, 1 above range)

warm-clamp-accfit-dyn (n=40, range 4254.9-5348.8 ns)
   4254.9 |########################################
   4309.6 |##############
   4364.3 |##
   4419.0 |#######
   4473.7 |
   4528.4 |
   4583.1 |
   4637.8 |
   4692.4 |
   4747.1 |
   4801.8 |
   4856.5 |
   4911.2 |
   4965.9 |
   5020.6 |
   5075.3 |
   5130.0 |#########
   5184.7 |#######
   5239.4 |
   5294.1 |
  (3 below, 3 above range)

warm-clamp-head (n=40, range 496.0-652.8 ns)
    496.0 |########################################
    503.9 |#########################
    511.7 |
    519.5 |########
    527.4 |##
    535.2 |
    543.1 |
    550.9 |
    558.7 |
    566.6 |
    574.4 |
    582.3 |
    590.1 |
    597.9 |
    605.8 |
    613.6 |
    621.5 |
    629.3 |
    637.2 |
    645.0 |##############
  (3 below, 5 above range)

warm-clamp-min-lanes (n=40, range 219.4-282.1 ns)
    219.4 |######
    222.5 |########################################
    225.7 |###
    228.8 |
    231.9 |
    235.1 |
    238.2 |
    241.3 |
    244.5 |
    247.6 |
    250.8 |
    253.9 |
    257.0 |
    260.2 |
    263.3 |
    266.4 |
    269.6 |
    272.7 |
    275.8 |############
    279.0 |#################################
  (4 below, 5 above range)

warm-clamp-minimum (n=40, range 212.1-281.7 ns)
    212.1 |#################
    215.6 |
    219.1 |##
    222.6 |
    226.0 |
    229.5 |
    233.0 |
    236.5 |
    239.9 |
    243.4 |
    246.9 |
    250.4 |
    253.8 |##
    257.3 |######################
    260.8 |
    264.3 |
    267.7 |
    271.2 |
    274.7 |#####
    278.2 |########################################
  (4 below, 4 above range)

```

## Diagnostics

- **warm-clamp-acc64**: autocorrelation=0.85 (measurement drift or warm-up artifact)
- **warm-clamp-accfit-dyn**: autocorrelation=0.79 (measurement drift or warm-up artifact)
- **warm-clamp-head**: autocorrelation=0.86 (measurement drift or warm-up artifact)
- **warm-clamp-min-lanes**: autocorrelation=0.87 (measurement drift or warm-up artifact)
- **warm-clamp-minimum**: autocorrelation=0.80 (measurement drift or warm-up artifact)
