# Clamping fold at arity 16, 1048576 elements: the same fork with both containers crossing this host's L2

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-accfit beats baseline by 64% (significant)

warm-clamp-accfit is -51.02 us (64%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-minimum is an outlier: 4.3x slower than the field

warm-clamp-minimum (126.41 us) is 4.3x the fastest (29.40 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-minimum shows warm-up / thermal drift (autocorr +0.57)

warm-clamp-minimum's per-pass series has lag-1 autocorrelation +0.57, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-accfit, warm-clamp-head} vs {warm-clamp-acc64, warm-clamp-accfit-dyn, warm-clamp-min-lanes, warm-clamp-minimum} (147% apart)

The field splits into a fast tier {warm-clamp-accfit, warm-clamp-head} and a slow tier {warm-clamp-acc64, warm-clamp-accfit-dyn, warm-clamp-min-lanes, warm-clamp-minimum} with a 147% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.3x the fastest

Fastest warm-clamp-accfit (29.40 us) to slowest warm-clamp-minimum (126.41 us): 4.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-clamp-accfit** at 29401.2 ns median (-63.1% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 4.30x (fastest 29401.2 ns, slowest 126413.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 80086ns | 79903ns | 78788ns | 79897ns | 81952ns | base |
| warm-clamp-accfit | 29632ns | 29526ns | 29015ns | 29523ns | 30575ns | -63.00% |
| warm-clamp-accfit-dyn | 87217ns | 86903ns | 86363ns | 86994ns | 88741ns | +8.90% |
| warm-clamp-head | 32617ns | 32402ns | 32170ns | 32467ns | 33515ns | -59.27% |
| warm-clamp-min-lanes | 126347ns | 125631ns | 125112ns | 125804ns | 129213ns | +57.76% |
| warm-clamp-minimum | 127010ns | 126669ns | 125066ns | 126929ns | 129198ns | +58.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 79932ns | 78613ns | 81794ns | base | 13.118 |
| warm-clamp-accfit | 29506ns | 28912ns | 30435ns | -63.09% | 35.538 |
| warm-clamp-accfit-dyn | 87007ns | 86172ns | 88573ns | +8.85% | 12.052 |
| warm-clamp-head | 32496ns | 32061ns | 33383ns | -59.34% | 32.267 |
| warm-clamp-min-lanes | 126080ns | 124863ns | 128914ns | +57.73% | 8.317 |
| warm-clamp-minimum | 126777ns | 124888ns | 128981ns | +58.61% | 8.271 |

## Performance model

- Peak throughput: **36.268 Gops/s** (warm-clamp-accfit; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 13.145 | 36.2% |
| warm-clamp-accfit | 35.664 | 98.3% |
| warm-clamp-accfit-dyn | 12.095 | 33.3% |
| warm-clamp-head | 32.480 | 89.6% |
| warm-clamp-min-lanes | 8.364 | 23.1% |
| warm-clamp-minimum | 8.295 | 22.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 80086ns | 80086ns | base |
| warm-clamp-accfit | 29632ns | 29632ns | -63.00% |
| warm-clamp-accfit-dyn | 87217ns | 87217ns | +8.90% |
| warm-clamp-head | 32617ns | 32617ns | -59.27% |
| warm-clamp-min-lanes | 126347ns | 126347ns | +57.76% |
| warm-clamp-minimum | 127010ns | 127010ns | +58.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 79768ns | base | --- | [79490, 79921] | --- | --- | --- | --- |
| warm-clamp-accfit | 29401ns | -50239.8ns (-63.0%) | [-50784, -49784]ns | [29206, 29556] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 86694ns | +6847.9ns (+8.6%) | [+6446, +7553]ns | [86335, 86969] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 32284ns | -47422.8ns (-59.5%) | [-47585, -46996]ns | [32218, 32399] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 125370ns | +45524.8ns (+57.1%) | [+45363, +46206]ns | [125039, 125982] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 126413ns | +46664.6ns (+58.5%) | [+46024, +47718]ns | [126147, 127420] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 79378ns | -62.8% | +8.7% | -59.4% | +69.7% | +61.8% |
| 2 | 79104ns | -61.6% | +10.9% | -59.1% | +66.8% | +60.9% |
| 3 | 79326ns | -62.2% | +9.6% | -59.3% | +60.3% | +62.1% |
| 4 | 79705ns | -62.5% | +9.6% | -59.7% | +59.6% | +61.1% |
| 5 | 79507ns | -63.5% | +9.4% | -59.5% | +60.0% | +61.6% |
| 6 | 80228ns | -63.8% | +8.0% | -59.9% | +55.9% | +61.0% |
| 7 | 79936ns | -63.8% | +8.5% | -59.4% | +58.4% | +58.2% |
| 8 | 82518ns | -64.1% | +6.0% | -60.8% | +55.0% | +52.6% |
| 9 | 79786ns | -62.4% | +8.2% | -59.6% | +56.7% | +59.8% |
| 10 | 79907ns | -63.5% | +11.7% | -59.6% | +56.4% | +59.5% |
| 11 | 78778ns | -62.9% | +10.3% | -56.9% | +60.7% | +61.1% |
| 12 | 78895ns | -63.4% | +9.9% | -58.2% | +59.0% | +59.4% |
| 13 | 78953ns | -63.3% | +9.1% | -59.3% | +59.8% | +59.8% |
| 14 | 79472ns | -63.3% | +9.4% | -59.8% | +57.1% | +57.1% |
| 15 | 78921ns | -62.4% | +11.9% | -59.4% | +58.3% | +58.0% |
| 16 | 78442ns | -62.6% | +13.8% | -59.2% | +59.1% | +58.9% |
| 17 | 78551ns | -63.3% | +13.9% | -56.8% | +59.7% | +61.5% |
| 18 | 78473ns | -63.0% | +11.8% | -58.1% | +59.2% | +60.9% |
| 19 | 78409ns | -62.6% | +11.8% | -59.2% | +60.6% | +59.7% |
| 20 | 78432ns | -62.5% | +13.7% | -59.2% | +59.1% | +59.3% |
| 21 | 79070ns | -62.6% | +9.1% | -59.2% | +58.0% | +57.9% |
| 22 | 82970ns | -64.5% | +4.0% | -61.0% | +51.0% | +52.0% |
| 23 | 80265ns | -63.3% | +7.4% | -58.8% | +55.7% | +55.9% |
| 24 | 79596ns | -63.0% | +8.2% | -58.0% | +56.8% | +57.2% |
| 25 | 79899ns | -62.0% | +7.8% | -59.7% | +56.5% | +56.2% |
| 26 | 79696ns | -61.4% | +8.1% | -59.6% | +56.7% | +57.5% |
| 27 | 79749ns | -61.2% | +8.1% | -59.4% | +57.0% | +60.4% |
| 28 | 81038ns | -63.5% | +6.4% | -60.0% | +54.1% | +57.2% |
| 29 | 79832ns | -63.2% | +8.1% | -59.6% | +57.2% | +58.1% |
| 30 | 82064ns | -63.9% | +5.5% | -60.8% | +53.0% | +55.4% |
| 31 | 81059ns | -63.8% | +6.7% | -60.3% | +55.2% | +56.6% |
| 32 | 79817ns | -63.9% | +9.7% | -58.8% | +56.9% | +61.3% |
| 33 | 80025ns | -64.0% | +8.7% | -58.5% | +56.5% | +61.3% |
| 34 | 79627ns | -63.7% | +9.7% | -59.2% | +57.0% | +61.0% |
| 35 | 80521ns | -63.7% | +7.5% | -60.0% | +56.6% | +58.9% |
| 36 | 81277ns | -63.1% | +7.5% | -59.9% | +55.8% | +61.1% |
| 37 | 81142ns | -61.9% | +6.3% | -58.6% | +56.2% | +55.5% |
| 38 | 80736ns | -63.9% | +7.6% | -59.5% | +56.2% | +56.4% |
| 39 | 79879ns | -63.5% | +8.0% | -59.6% | +56.4% | +58.2% |
| 40 | 82283ns | -64.1% | +4.6% | -60.6% | +56.0% | +53.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.359 | moderate+ |
| warm-clamp-accfit | 0.499 | moderate+ |
| warm-clamp-accfit-dyn | 0.393 | moderate+ |
| warm-clamp-head | 0.232 | moderate+ |
| warm-clamp-min-lanes | 0.485 | moderate+ |
| warm-clamp-minimum | 0.566 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-clamp-accfit**: won 40/40, lost 0/40
- **warm-clamp-accfit-dyn**: won 0/40, lost 40/40
- **warm-clamp-head**: won 40/40, lost 0/40
- **warm-clamp-min-lanes**: won 0/40, lost 40/40
- **warm-clamp-minimum**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 3.3ns | 79931.7ns | 0.0% |  |
| warm-clamp-accfit | 3.0ns | 29506.1ns | 0.0% |  |
| warm-clamp-accfit-dyn | 3.3ns | 87006.7ns | 0.0% |  |
| warm-clamp-head | 3.1ns | 32496.5ns | 0.0% |  |
| warm-clamp-min-lanes | 3.7ns | 126080.0ns | 0.0% |  |
| warm-clamp-minimum | 4.8ns | 126777.5ns | 0.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 78612.7-81793.6 ns)
  78612.7 |
  78771.7 |########################
  78930.7 |################
  79089.8 |########
  79248.8 |################
  79407.9 |################
  79566.9 |################################
  79726.0 |########################################
  79885.0 |################################
  80044.1 |
  80203.1 |################
  80362.2 |########
  80521.2 |
  80680.3 |########
  80839.3 |
  80998.4 |########################
  81157.4 |########
  81316.5 |
  81475.5 |
  81634.6 |
  (5 below, 4 above range)

warm-clamp-accfit (n=40, range 28911.8-30434.5 ns)
  28911.8 |##############################
  28987.9 |####################
  29064.1 |####################
  29140.2 |##############################
  29216.4 |####################
  29292.5 |##############################
  29368.6 |##############################
  29444.8 |####################
  29520.9 |########################################
  29597.0 |####################
  29673.2 |##########
  29749.3 |
  29825.4 |
  29901.6 |##########
  29977.7 |##############################
  30053.8 |
  30130.0 |
  30206.1 |
  30282.3 |##########
  30358.4 |##########
  (4 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 86172.2-88573.0 ns)
  86172.2 |########################################
  86292.2 |#############
  86412.3 |########
  86532.3 |########
  86652.4 |########
  86772.4 |########
  86892.4 |#################
  87012.5 |
  87132.5 |
  87252.6 |####
  87372.6 |#############
  87492.6 |####
  87612.7 |########
  87732.7 |####
  87852.8 |
  87972.8 |
  88092.8 |
  88212.9 |####
  88332.9 |
  88453.0 |
  (3 below, 4 above range)

warm-clamp-head (n=40, range 32061.4-33383.4 ns)
  32061.4 |
  32127.5 |######################
  32193.6 |########################################
  32259.7 |#################
  32325.8 |#############
  32391.9 |########
  32458.0 |
  32524.1 |####
  32590.2 |####
  32656.3 |
  32722.4 |####
  32788.5 |
  32854.6 |########
  32920.7 |####
  32986.8 |
  33052.9 |####
  33119.0 |
  33185.1 |####
  33251.2 |
  33317.3 |
  (5 below, 4 above range)

warm-clamp-min-lanes (n=40, range 124863.0-128913.6 ns)
  124863.0 |########################################
  125065.6 |#############
  125268.1 |######
  125470.6 |##########
  125673.1 |###
  125875.7 |######
  126078.2 |######
  126280.7 |
  126483.3 |##########
  126685.8 |###
  126888.3 |
  127090.8 |##########
  127293.4 |
  127495.9 |
  127698.4 |
  127901.0 |###
  128103.5 |
  128306.0 |###
  128508.5 |
  128711.1 |
  (3 below, 2 above range)

warm-clamp-minimum (n=40, range 124887.7-128980.6 ns)
  124887.7 |#############
  125092.4 |####################
  125297.0 |
  125501.6 |######
  125706.3 |#############
  125910.9 |######
  126115.6 |########################################
  126320.2 |#############
  126524.9 |
  126729.5 |####################
  126934.1 |
  127138.8 |######
  127343.4 |##########################
  127548.1 |
  127752.7 |#############
  127957.4 |
  128162.0 |######
  128366.6 |####################
  128571.3 |#############
  128775.9 |
  (4 below, 3 above range)

```

## Diagnostics

- **warm-clamp-minimum**: autocorrelation=0.57 (measurement drift or warm-up artifact)
