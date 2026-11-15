# Clamping fold at arity 16, 1048576 elements: the same fork with both containers crossing this host's L2

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-minimum is an outlier: 4.6x slower than the field

warm-clamp-minimum (269.14 us) is 4.6x the fastest (58.65 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-head shows warm-up / thermal drift (autocorr +0.75)

warm-clamp-head's per-pass series has lag-1 autocorrelation +0.75, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-accfit, warm-clamp-acc64, warm-clamp-accfit-dyn, warm-clamp-head} vs {warm-clamp-min-lanes, warm-clamp-minimum} (102% apart)

The field splits into a fast tier {warm-clamp-accfit, warm-clamp-acc64, warm-clamp-accfit-dyn, warm-clamp-head} and a slow tier {warm-clamp-min-lanes, warm-clamp-minimum} with a 102% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.6x the fastest

Fastest warm-clamp-accfit (58.65 us) to slowest warm-clamp-minimum (269.14 us): 4.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-clamp-accfit** at 58646.0 ns median (-1.2% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 4.59x (fastest 58646.0 ns, slowest 269137.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 58536ns | 59553ns | 53694ns | 58562ns | 63302ns | base |
| warm-clamp-accfit | 58440ns | 58849ns | 53055ns | 58508ns | 63620ns | -0.16% |
| warm-clamp-accfit-dyn | 115003ns | 113116ns | 110652ns | 113642ns | 123438ns | +96.47% |
| warm-clamp-head | 122421ns | 119535ns | 113677ns | 119682ns | 139384ns | +109.14% |
| warm-clamp-min-lanes | 242501ns | 241252ns | 239029ns | 241725ns | 248300ns | +314.27% |
| warm-clamp-minimum | 271019ns | 269458ns | 267658ns | 269666ns | 278440ns | +362.99% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 58354ns | 53558ns | 63060ns | base | 17.969 |
| warm-clamp-accfit | 58255ns | 52902ns | 63436ns | -0.17% | 18.000 |
| warm-clamp-accfit-dyn | 114779ns | 110413ns | 123176ns | +96.70% | 9.136 |
| warm-clamp-head | 121908ns | 112733ns | 139072ns | +108.91% | 8.601 |
| warm-clamp-min-lanes | 242130ns | 238632ns | 248043ns | +314.93% | 4.331 |
| warm-clamp-minimum | 270618ns | 267224ns | 278122ns | +363.75% | 3.875 |

## Performance model

- Peak throughput: **19.821 Gops/s** (warm-clamp-accfit; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 17.666 | 89.1% |
| warm-clamp-accfit | 17.880 | 90.2% |
| warm-clamp-accfit-dyn | 9.286 | 46.9% |
| warm-clamp-head | 8.805 | 44.4% |
| warm-clamp-min-lanes | 4.354 | 22.0% |
| warm-clamp-minimum | 3.896 | 19.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 58536ns | 58536ns | base |
| warm-clamp-accfit | 58440ns | 58440ns | -0.16% |
| warm-clamp-accfit-dyn | 115003ns | 115003ns | +96.47% |
| warm-clamp-head | 122421ns | 122421ns | +109.14% |
| warm-clamp-min-lanes | 242501ns | 242501ns | +314.27% |
| warm-clamp-minimum | 271019ns | 271019ns | +362.99% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 59354ns | base | --- | [56253, 60066] | --- | --- | --- | --- |
| warm-clamp-accfit | 58646ns | no significant difference | [-1851, +2234]ns | [56399, 59850] | no | 0.8746 | 0.8746 | 0 |
| warm-clamp-accfit-dyn | 112915ns | +56653.8ns (+95.5%) | [+54291, +59106]ns | [111959, 114660] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 119086ns | +61113.1ns (+103.0%) | [+58035, +65381]ns | [116370, 121963] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 240844ns | +182954.2ns (+308.2%) | [+180836, +185550]ns | [240041, 242642] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 269137ns | +211219.6ns (+355.9%) | [+208477, +213683]ns | [268175, 270041] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 56916ns | -3.0% | +101.7% | +101.7% | +326.1% | +391.0% |
| 2 | 52419ns | +6.3% | +110.0% | +127.7% | +370.0% | +438.0% |
| 3 | 54800ns | +7.6% | +109.6% | +125.6% | +338.5% | +403.8% |
| 4 | 58128ns | -3.4% | +93.9% | +125.4% | +312.9% | +365.6% |
| 5 | 55982ns | +7.5% | +98.7% | +152.0% | +333.4% | +382.3% |
| 6 | 60093ns | -7.5% | +92.5% | +153.4% | +297.3% | +348.0% |
| 7 | 69801ns | -24.3% | +63.4% | +97.9% | +242.2% | +291.9% |
| 8 | 55210ns | -4.0% | +112.0% | +166.0% | +339.6% | +424.1% |
| 9 | 53967ns | -2.9% | +107.6% | +133.7% | +349.1% | +396.8% |
| 10 | 63422ns | -13.2% | +74.9% | +112.4% | +279.5% | +324.6% |
| 11 | 58951ns | +5.4% | +106.9% | +110.8% | +307.2% | +357.3% |
| 12 | 60672ns | +2.9% | +106.6% | +93.4% | +293.5% | +342.9% |
| 13 | 60448ns | +0.2% | +106.8% | +101.5% | +303.6% | +345.1% |
| 14 | 60040ns | +6.4% | +112.2% | +100.3% | +302.0% | +345.6% |
| 15 | 62524ns | -0.8% | +98.0% | +97.4% | +290.8% | +327.3% |
| 16 | 61231ns | +1.8% | +83.1% | +103.5% | +292.9% | +337.0% |
| 17 | 59937ns | +6.9% | +88.0% | +121.7% | +298.1% | +346.4% |
| 18 | 63133ns | +0.7% | +80.8% | +114.7% | +281.7% | +326.7% |
| 19 | 56238ns | +13.1% | +96.0% | +114.3% | +324.4% | +391.3% |
| 20 | 52955ns | +22.3% | +114.3% | +127.2% | +360.8% | +415.8% |
| 21 | 56230ns | -4.5% | +98.9% | +97.4% | +331.6% | +377.4% |
| 22 | 52862ns | +7.4% | +112.0% | +110.1% | +366.8% | +405.8% |
| 23 | 52762ns | +13.3% | +114.4% | +121.6% | +373.5% | +406.2% |
| 24 | 59020ns | +1.5% | +103.2% | +96.6% | +304.7% | +353.1% |
| 25 | 61592ns | -3.3% | +100.1% | +87.5% | +287.3% | +333.7% |
| 26 | 59716ns | -0.4% | +98.8% | +94.1% | +299.4% | +347.1% |
| 27 | 59688ns | -0.1% | +90.2% | +94.2% | +299.4% | +348.0% |
| 28 | 62103ns | -8.8% | +80.0% | +87.9% | +286.0% | +334.9% |
| 29 | 53810ns | +8.4% | +112.8% | +121.7% | +343.7% | +397.9% |
| 30 | 55404ns | +7.1% | +101.6% | +120.5% | +333.2% | +382.3% |
| 31 | 55358ns | +4.2% | +108.1% | +110.9% | +351.2% | +387.7% |
| 32 | 57264ns | -7.5% | +95.7% | +103.8% | +330.6% | +372.5% |
| 33 | 54886ns | -3.7% | +103.9% | +107.0% | +345.6% | +388.7% |
| 34 | 59842ns | -12.0% | +85.4% | +89.7% | +312.9% | +347.2% |
| 35 | 59785ns | -4.7% | +87.2% | +98.9% | +323.8% | +349.8% |
| 36 | 60113ns | -12.2% | +83.2% | +90.8% | +305.2% | +349.3% |
| 37 | 60223ns | -8.8% | +82.4% | +87.9% | +301.0% | +346.4% |
| 38 | 56269ns | +0.7% | +95.2% | +101.6% | +326.6% | +383.5% |
| 39 | 60219ns | +4.2% | +85.5% | +88.0% | +297.6% | +352.5% |
| 40 | 60138ns | +3.5% | +95.5% | +87.8% | +300.3% | +357.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.239 | moderate+ |
| warm-clamp-accfit | 0.625 | HIGH+ (drift/warm-up) |
| warm-clamp-accfit-dyn | 0.648 | HIGH+ (drift/warm-up) |
| warm-clamp-head | 0.747 | HIGH+ (drift/warm-up) |
| warm-clamp-min-lanes | 0.405 | moderate+ |
| warm-clamp-minimum | 0.386 | moderate+ |

**Consistency summary:**

- **warm-clamp-accfit**: won 18/40, lost 21/40
- **warm-clamp-accfit-dyn**: won 0/40, lost 40/40
- **warm-clamp-head**: won 0/40, lost 40/40
- **warm-clamp-min-lanes**: won 0/40, lost 40/40
- **warm-clamp-minimum**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 3.8ns | 58353.8ns | 0.0% |  |
| warm-clamp-accfit | 3.2ns | 58255.4ns | 0.0% |  |
| warm-clamp-accfit-dyn | 4.5ns | 114779.3ns | 0.0% |  |
| warm-clamp-head | 4.7ns | 121908.3ns | 0.0% |  |
| warm-clamp-min-lanes | 5.1ns | 242129.5ns | 0.0% |  |
| warm-clamp-minimum | 6.0ns | 270617.8ns | 0.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 53557.6-63059.9 ns)
  53557.6 |###########
  54032.7 |
  54507.8 |###########
  54982.9 |#################
  55458.0 |
  55933.1 |######################
  56408.3 |
  56883.4 |###########
  57358.5 |
  57833.6 |#####
  58308.7 |
  58783.8 |###########
  59259.0 |###########
  59734.1 |########################################
  60209.2 |######################
  60684.3 |
  61159.4 |###########
  61634.5 |#####
  62109.7 |#####
  62584.8 |
  (4 below, 3 above range)

warm-clamp-accfit (n=40, range 52901.6-63436.1 ns)
  52901.6 |####################
  53428.3 |##########
  53955.0 |
  54481.7 |##########
  55008.5 |####################
  55535.2 |####################
  56061.9 |##########
  56588.7 |########################################
  57115.4 |
  57642.1 |##########
  58168.8 |##########
  58695.6 |##########
  59222.3 |########################################
  59749.0 |##############################
  60275.7 |##########
  60802.5 |
  61329.2 |
  61855.9 |########################################
  62382.6 |####################
  62909.4 |
  (5 below, 5 above range)

warm-clamp-accfit-dyn (n=40, range 110413.1-123176.2 ns)
  110413.1 |########
  111051.2 |####
  111689.4 |########################################
  112327.5 |########
  112965.7 |############
  113603.9 |########
  114242.0 |############
  114880.2 |####
  115518.3 |####
  116156.5 |
  116794.6 |####
  117432.8 |####
  118070.9 |
  118709.1 |####
  119347.2 |####
  119985.4 |
  120623.5 |
  121261.7 |
  121899.8 |####
  122538.0 |
  (5 below, 5 above range)

warm-clamp-head (n=40, range 112732.9-139071.6 ns)
  112732.9 |########################################
  114049.8 |#############
  115366.8 |##########################
  116683.7 |#################################
  118000.7 |#############
  119317.6 |##########################
  120634.5 |######
  121951.5 |######
  123268.4 |####################
  124585.3 |######
  125902.3 |######
  127219.2 |
  128536.2 |
  129853.1 |######
  131170.0 |
  132487.0 |######
  133803.9 |######
  135120.8 |######
  136437.8 |
  137754.7 |######
  (2 below, 3 above range)

warm-clamp-min-lanes (n=40, range 238631.9-248042.8 ns)
  238631.9 |########################################
  239102.4 |######
  239573.0 |####################
  240043.5 |####################
  240514.1 |##########################
  240984.6 |######
  241455.1 |######
  241925.7 |######
  242396.2 |##########################
  242866.8 |
  243337.3 |######
  243807.9 |#############
  244278.4 |#############
  244749.0 |
  245219.5 |
  245690.1 |
  246160.6 |#############
  246631.2 |#############
  247101.7 |
  247572.3 |
  (4 below, 3 above range)

warm-clamp-minimum (n=40, range 267223.8-278122.0 ns)
  267223.8 |########################################
  267768.7 |###############
  268313.6 |###############
  268858.5 |#########################
  269403.4 |#####
  269948.3 |####################
  270493.2 |##########
  271038.2 |
  271583.1 |#####
  272128.0 |#####
  272672.9 |#####
  273217.8 |#####
  273762.7 |
  274307.6 |
  274852.6 |#####
  275397.5 |
  275942.4 |##########
  276487.3 |
  277032.2 |
  277577.1 |
  (4 below, 3 above range)

```

## Diagnostics

- **warm-clamp-accfit**: autocorrelation=0.63 (measurement drift or warm-up artifact)
- **warm-clamp-accfit-dyn**: autocorrelation=0.65 (measurement drift or warm-up artifact)
- **warm-clamp-head**: autocorrelation=0.75 (measurement drift or warm-up artifact)
