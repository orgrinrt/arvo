# Clamping fold at 32 bits, arity 2 / 4 / 8 / 16 / 64 / 256: the shipped doubled container against minimum storage, against minimum storage with the fold lane-split, and against minimum storage with the accumulator sized by the design's own interior-safety rule

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-minimum beats baseline by 52% (significant)

warm-clamp-minimum is -448 ns (52%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-accfit-dyn is an outlier: 13.4x slower than the field

warm-clamp-accfit-dyn (5.66 us) is 13.4x the fastest (422 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-head shows warm-up / thermal drift (autocorr +0.76)

warm-clamp-head's per-pass series has lag-1 autocorrelation +0.76, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-minimum, warm-clamp-min-lanes, warm-clamp-acc64, warm-clamp-accfit, warm-clamp-head} vs {warm-clamp-accfit-dyn} (461% apart)

The field splits into a fast tier {warm-clamp-minimum, warm-clamp-min-lanes, warm-clamp-acc64, warm-clamp-accfit, warm-clamp-head} and a slow tier {warm-clamp-accfit-dyn} with a 461% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 13.4x the fastest

Fastest warm-clamp-minimum (422 ns) to slowest warm-clamp-accfit-dyn (5.66 us): 13.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### warm-clamp-accfit's edge over baseline is significant but tiny (4 ns, 0.51%)

warm-clamp-accfit differs from baseline warm-clamp-acc64 by 4 ns (0.51%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: warm-clamp-minimum** at 422.1 ns median (-50.7% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 13.40x (fastest 422.1 ns, slowest 5655.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 931ns | 919ns | 899ns | 919ns | 1000ns | base |
| warm-clamp-accfit | 1004ns | 932ns | 901ns | 957ns | 1251ns | +7.90% |
| warm-clamp-accfit-dyn | 6138ns | 5720ns | 5551ns | 5727ns | 7956ns | +559.37% |
| warm-clamp-head | 1116ns | 1069ns | 1045ns | 1076ns | 1304ns | +19.84% |
| warm-clamp-min-lanes | 496ns | 501ns | 475ns | 495ns | 520ns | -46.76% |
| warm-clamp-minimum | 486ns | 484ns | 477ns | 484ns | 500ns | -47.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 867ns | 838ns | 931ns | base | 9.453 |
| warm-clamp-accfit | 935ns | 838ns | 1164ns | +7.86% | 8.764 |
| warm-clamp-accfit-dyn | 6065ns | 5487ns | 7861ns | +599.90% | 1.351 |
| warm-clamp-head | 1050ns | 986ns | 1229ns | +21.21% | 7.799 |
| warm-clamp-min-lanes | 432ns | 415ns | 449ns | -50.17% | 18.971 |
| warm-clamp-minimum | 424ns | 416ns | 436ns | -51.13% | 19.341 |

## Performance model

- Peak throughput: **19.716 Gops/s** (warm-clamp-min-lanes; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 9.570 | 48.5% |
| warm-clamp-accfit | 9.435 | 47.9% |
| warm-clamp-accfit-dyn | 1.449 | 7.3% |
| warm-clamp-head | 8.133 | 41.2% |
| warm-clamp-min-lanes | 18.733 | 95.0% |
| warm-clamp-minimum | 19.408 | 98.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 931ns | 931ns | base |
| warm-clamp-accfit | 1004ns | 1004ns | +7.90% |
| warm-clamp-accfit-dyn | 6138ns | 6138ns | +559.37% |
| warm-clamp-head | 1116ns | 1116ns | +19.84% |
| warm-clamp-min-lanes | 496ns | 496ns | -46.76% |
| warm-clamp-minimum | 486ns | 486ns | -47.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 856ns | base | --- | [843, 865] | --- | --- | --- | --- |
| warm-clamp-accfit | 868ns | +5.2ns (+0.6%) | [+0, +32]ns | [864, 873] | YES (adj: no) | 0.0807 | 0.0807 | 0 |
| warm-clamp-accfit-dyn | 5655ns | +4810.5ns (+562.0%) | [+4711, +4896]ns | [5556, 5764] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 1007ns | +165.9ns (+19.4%) | [+157, +170]ns | [1003, 1033] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 437ns | -425.2ns (-49.7%) | [-428, -423]ns | [417, 439] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 422ns | -429.6ns (-50.2%) | [-436, -422]ns | [421, 423] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 842ns | +78.0% | +551.4% | +68.9% | -50.4% | -50.0% |
| 2 | 840ns | +78.6% | +557.8% | +69.0% | -50.5% | -49.6% |
| 3 | 842ns | +47.7% | +567.0% | +68.8% | -50.8% | -49.8% |
| 4 | 841ns | +2.7% | +558.0% | +56.1% | -50.4% | -50.0% |
| 5 | 843ns | +3.2% | +1125.4% | +18.9% | -50.6% | -49.9% |
| 6 | 841ns | +3.9% | +1274.3% | +19.1% | -50.5% | -49.8% |
| 7 | 841ns | +3.7% | +806.7% | +19.3% | -50.5% | -49.7% |
| 8 | 843ns | +2.5% | +554.8% | +18.9% | -50.6% | -50.1% |
| 9 | 891ns | -3.5% | +515.4% | +12.3% | -53.5% | -52.4% |
| 10 | 1330ns | -37.2% | +658.7% | -24.7% | -68.6% | -68.4% |
| 11 | 856ns | -1.6% | +554.1% | +20.6% | -48.9% | -50.6% |
| 12 | 842ns | -0.2% | +564.6% | +19.5% | -48.2% | -49.8% |
| 13 | 838ns | +0.5% | +568.1% | +20.5% | -47.8% | -49.7% |
| 14 | 836ns | +0.4% | +575.8% | +20.4% | -47.3% | -49.4% |
| 15 | 853ns | -1.6% | +563.3% | +18.1% | -48.6% | -50.7% |
| 16 | 843ns | -0.2% | +589.1% | +19.5% | -47.7% | -50.0% |
| 17 | 862ns | -2.7% | +574.0% | +16.9% | -49.3% | -50.8% |
| 18 | 835ns | +0.5% | +595.8% | +20.6% | -47.1% | -49.1% |
| 19 | 839ns | -0.2% | +592.5% | +20.5% | -47.7% | -49.7% |
| 20 | 835ns | +0.5% | +594.1% | +20.3% | -47.8% | -49.7% |
| 21 | 869ns | -0.0% | +571.9% | +19.4% | -49.5% | -49.4% |
| 22 | 868ns | +0.1% | +563.2% | +19.6% | -49.4% | -50.9% |
| 23 | 865ns | +0.7% | +566.0% | +19.6% | -49.2% | -50.6% |
| 24 | 868ns | -0.4% | +564.5% | +19.6% | -49.5% | -51.5% |
| 25 | 868ns | +0.4% | +564.8% | +19.3% | -49.2% | -50.1% |
| 26 | 868ns | -0.3% | +565.2% | +19.6% | -49.4% | -49.5% |
| 27 | 862ns | +0.7% | +561.8% | +20.0% | -49.0% | -49.0% |
| 28 | 869ns | -0.6% | +561.8% | +28.1% | -49.6% | -49.4% |
| 29 | 870ns | -0.2% | +562.2% | +19.3% | -41.8% | -50.1% |
| 30 | 865ns | -3.8% | +566.8% | +23.1% | -49.2% | -49.7% |
| 31 | 850ns | +17.5% | +556.2% | +16.0% | -51.2% | -50.9% |
| 32 | 838ns | +19.5% | +554.9% | +17.8% | -50.4% | -50.3% |
| 33 | 857ns | +16.7% | +541.8% | +14.9% | -51.4% | -51.5% |
| 34 | 877ns | +13.8% | +525.6% | +12.3% | -52.5% | -52.6% |
| 35 | 863ns | +15.5% | +535.5% | +14.1% | -51.7% | -51.8% |
| 36 | 856ns | +16.9% | +545.0% | +15.2% | -49.3% | -51.3% |
| 37 | 852ns | +26.0% | +546.9% | +15.7% | -48.5% | -51.2% |
| 38 | 867ns | +15.3% | +533.1% | +13.6% | -49.5% | -52.0% |
| 39 | 872ns | +14.6% | +529.3% | +13.1% | -49.6% | -52.3% |
| 40 | 869ns | +15.0% | +535.4% | +13.7% | -49.6% | -52.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.067 | ok |
| warm-clamp-accfit | 0.703 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit-dyn | 0.416 | moderate+ |
| warm-clamp-head | 0.757 | HIGH+ (drift/warm-up) |
| warm-clamp-min-lanes | 0.482 | moderate+ |
| warm-clamp-minimum | 0.673 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-clamp-accfit**: won 13/40, lost 25/40
- **warm-clamp-accfit-dyn**: won 0/40, lost 40/40
- **warm-clamp-head**: won 1/40, lost 39/40
- **warm-clamp-min-lanes**: won 40/40, lost 0/40
- **warm-clamp-minimum**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 2.2ns | 866.6ns | 0.3% |  |
| warm-clamp-accfit | 2.8ns | 934.7ns | 0.3% |  |
| warm-clamp-accfit-dyn | 2.6ns | 6065.5ns | 0.0% |  |
| warm-clamp-head | 2.5ns | 1050.4ns | 0.2% |  |
| warm-clamp-min-lanes | 2.2ns | 431.8ns | 0.5% |  |
| warm-clamp-minimum | 2.1ns | 423.5ns | 0.5% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 837.7-930.9 ns)
    837.7 |########################################
    842.4 |#############
    847.0 |####
    851.7 |#################
    856.3 |####
    861.0 |######################
    865.6 |########################################
    870.3 |####
    875.0 |####
    879.6 |
    884.3 |
    888.9 |####
    893.6 |
    898.2 |
    902.9 |
    907.6 |
    912.2 |
    916.9 |
    921.5 |
    926.2 |
  (4 below, 1 above range)

warm-clamp-accfit (n=40, range 837.6-1164.4 ns)
    837.6 |####################################
    854.0 |########################################
    870.3 |####################
    886.6 |
    903.0 |
    919.3 |
    935.7 |
    952.0 |
    968.3 |
    984.7 |####################################
   1001.0 |
   1017.4 |
   1033.7 |
   1050.0 |
   1066.4 |####
   1082.7 |
   1099.1 |
   1115.4 |
   1131.8 |
   1148.1 |
  (3 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 5486.7-7861.1 ns)
   5486.7 |#####################################
   5605.4 |###########
   5724.2 |########################################
   5842.9 |
   5961.6 |
   6080.3 |
   6199.0 |
   6317.7 |
   6436.5 |
   6555.2 |
   6673.9 |
   6792.6 |
   6911.3 |
   7030.1 |
   7148.8 |
   7267.5 |
   7386.2 |
   7504.9 |
   7623.7 |##
   7742.4 |
  (5 below, 3 above range)

warm-clamp-head (n=40, range 985.5-1228.6 ns)
    985.5 |##################
    997.7 |########################################
   1009.8 |######
   1022.0 |######
   1034.1 |#####################
   1046.3 |
   1058.4 |###
   1070.6 |
   1082.7 |
   1094.9 |
   1107.0 |###
   1119.2 |
   1131.4 |
   1143.5 |
   1155.7 |
   1167.8 |
   1180.0 |
   1192.1 |
   1204.3 |
   1216.4 |
  (4 below, 4 above range)

warm-clamp-min-lanes (n=40, range 415.5-448.6 ns)
    415.5 |####################################
    417.2 |###
    418.8 |
    420.5 |
    422.1 |
    423.8 |
    425.4 |
    427.1 |
    428.8 |
    430.4 |
    432.1 |
    433.7 |###
    435.4 |##########
    437.0 |##################
    438.7 |########################################
    440.4 |##########
    442.0 |###
    443.7 |
    445.3 |
    447.0 |
  (4 below, 1 above range)

warm-clamp-minimum (n=40, range 415.9-436.0 ns)
    415.9 |####################
    417.0 |#############
    418.0 |
    419.0 |
    420.0 |########################################
    421.0 |####################
    422.0 |########################################
    423.0 |####################
    424.0 |#############
    425.0 |#############
    426.0 |
    427.0 |######
    428.0 |
    429.0 |
    430.0 |
    431.0 |
    432.0 |
    433.0 |######
    434.0 |######
    435.0 |######
  (5 below, 4 above range)

```

## Diagnostics

- **warm-clamp-accfit**: autocorrelation=0.70 (measurement drift or warm-up artifact)
- **warm-clamp-accfit-dyn**: CV=22.5% (high variance, measurements may be unstable)
- **warm-clamp-head**: autocorrelation=0.76 (measurement drift or warm-up artifact)
- **warm-clamp-minimum**: autocorrelation=0.67 (measurement drift or warm-up artifact)
