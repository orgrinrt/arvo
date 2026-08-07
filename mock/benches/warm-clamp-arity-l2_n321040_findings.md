# Clamping fold at arity 16, 1048576 elements: the same fork with both containers crossing this host's L2

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-clamp-minimum is an outlier: 4.8x slower than the field

warm-clamp-minimum (272.47 us) is 4.8x the fastest (56.47 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-clamp-head shows warm-up / thermal drift (autocorr +0.69)

warm-clamp-head's per-pass series has lag-1 autocorrelation +0.69, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-clamp-accfit, warm-clamp-acc64, warm-clamp-accfit-dyn, warm-clamp-head} vs {warm-clamp-min-lanes, warm-clamp-minimum} (102% apart)

The field splits into a fast tier {warm-clamp-accfit, warm-clamp-acc64, warm-clamp-accfit-dyn, warm-clamp-head} and a slow tier {warm-clamp-min-lanes, warm-clamp-minimum} with a 102% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.8x the fastest

Fastest warm-clamp-accfit (56.47 us) to slowest warm-clamp-minimum (272.47 us): 4.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-clamp-accfit** at 56469.2 ns median (-5.6% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 4.83x (fastest 56469.2 ns, slowest 272465.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 59515ns | 60079ns | 53785ns | 59365ns | 65694ns | base |
| warm-clamp-accfit | 57265ns | 56646ns | 53154ns | 56946ns | 62330ns | -3.78% |
| warm-clamp-accfit-dyn | 113133ns | 112469ns | 109961ns | 112341ns | 118682ns | +90.09% |
| warm-clamp-head | 121786ns | 121003ns | 110934ns | 120792ns | 135623ns | +104.63% |
| warm-clamp-min-lanes | 244347ns | 243446ns | 239682ns | 243535ns | 251450ns | +310.57% |
| warm-clamp-minimum | 273236ns | 272991ns | 269789ns | 272865ns | 277795ns | +359.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 59336ns | 53636ns | 65491ns | base | 17.672 |
| warm-clamp-accfit | 57096ns | 53007ns | 62119ns | -3.77% | 18.365 |
| warm-clamp-accfit-dyn | 112879ns | 109737ns | 118393ns | +90.24% | 9.289 |
| warm-clamp-head | 121449ns | 110711ns | 135284ns | +104.68% | 8.634 |
| warm-clamp-min-lanes | 244032ns | 239362ns | 251167ns | +311.27% | 4.297 |
| warm-clamp-minimum | 272884ns | 269339ns | 277550ns | +359.90% | 3.843 |

## Performance model

- Peak throughput: **19.782 Gops/s** (warm-clamp-accfit; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 17.527 | 88.6% |
| warm-clamp-accfit | 18.569 | 93.9% |
| warm-clamp-accfit-dyn | 9.341 | 47.2% |
| warm-clamp-head | 8.702 | 44.0% |
| warm-clamp-min-lanes | 4.314 | 21.8% |
| warm-clamp-minimum | 3.848 | 19.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 59515ns | 59515ns | base |
| warm-clamp-accfit | 57265ns | 57265ns | -3.78% |
| warm-clamp-accfit-dyn | 113133ns | 113133ns | +90.09% |
| warm-clamp-head | 121786ns | 121786ns | +104.63% |
| warm-clamp-min-lanes | 244347ns | 244347ns | +310.57% |
| warm-clamp-minimum | 273236ns | 273236ns | +359.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 59828ns | base | --- | [56957, 60238] | --- | --- | --- | --- |
| warm-clamp-accfit | 56469ns | no significant difference | [-3758, +681]ns | [54749, 59236] | no | 0.2682 | 0.2682 | 0 |
| warm-clamp-accfit-dyn | 112252ns | +52207.3ns (+87.3%) | [+50106, +56902]ns | [110312, 113501] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-head | 120505ns | +61097.3ns (+102.1%) | [+59809, +63680]ns | [116682, 124409] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-min-lanes | 243039ns | +185644.6ns (+310.3%) | [+182291, +187531]ns | [241908, 244600] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-minimum | 272465ns | +213013.5ns (+356.0%) | [+211283, +215347]ns | [271672, 273602] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|
| 1 | 71168ns | -26.0% | +57.2% | +82.0% | +244.0% | +283.1% |
| 2 | 59960ns | -10.5% | +83.4% | +108.6% | +318.8% | +353.6% |
| 3 | 60153ns | -11.3% | +82.7% | +107.4% | +315.7% | +357.4% |
| 4 | 60117ns | +2.3% | +82.7% | +105.3% | +315.7% | +352.6% |
| 5 | 64065ns | -17.6% | +71.2% | +91.5% | +310.0% | +330.1% |
| 6 | 62658ns | +11.1% | +75.1% | +98.5% | +299.0% | +333.8% |
| 7 | 68884ns | -20.3% | +59.5% | +81.1% | +247.4% | +294.4% |
| 8 | 63785ns | -16.6% | +72.9% | +95.5% | +275.2% | +330.5% |
| 9 | 64179ns | -0.4% | +78.1% | +97.4% | +280.1% | +324.3% |
| 10 | 64166ns | -5.9% | +77.2% | +73.8% | +281.3% | +321.1% |
| 11 | 52685ns | +2.0% | +108.4% | +115.5% | +360.6% | +419.2% |
| 12 | 52451ns | +2.3% | +115.3% | +116.8% | +361.7% | +418.0% |
| 13 | 54987ns | -2.2% | +112.6% | +108.8% | +336.8% | +392.2% |
| 14 | 59504ns | -9.7% | +102.1% | +118.1% | +302.9% | +354.7% |
| 15 | 60230ns | -9.3% | +88.2% | +129.9% | +302.5% | +356.2% |
| 16 | 53900ns | +1.2% | +108.5% | +143.4% | +345.1% | +406.1% |
| 17 | 53384ns | +3.3% | +118.5% | +148.0% | +350.7% | +413.4% |
| 18 | 62022ns | -4.4% | +88.6% | +133.6% | +284.7% | +343.2% |
| 19 | 64858ns | -8.6% | +82.9% | +107.8% | +284.1% | +321.9% |
| 20 | 59750ns | -6.3% | +102.7% | +136.4% | +310.6% | +356.2% |
| 21 | 59886ns | -3.2% | +83.8% | +94.3% | +302.6% | +352.1% |
| 22 | 54920ns | +7.8% | +99.6% | +103.5% | +343.2% | +390.1% |
| 23 | 54913ns | +4.2% | +104.4% | +115.5% | +350.2% | +390.9% |
| 24 | 59401ns | +0.1% | +84.5% | +86.3% | +305.7% | +353.9% |
| 25 | 56586ns | -0.9% | +103.1% | +105.6% | +331.4% | +381.9% |
| 26 | 56766ns | +10.0% | +98.0% | +106.1% | +330.8% | +375.5% |
| 27 | 60724ns | -3.9% | +80.8% | +82.1% | +308.8% | +348.5% |
| 28 | 62381ns | -15.3% | +79.9% | +77.4% | +291.0% | +334.9% |
| 29 | 60071ns | -12.4% | +85.4% | +86.7% | +309.1% | +345.9% |
| 30 | 60245ns | -12.2% | +83.1% | +83.6% | +296.8% | +347.7% |
| 31 | 55507ns | +1.3% | +107.9% | +93.9% | +337.0% | +385.3% |
| 32 | 61402ns | -7.7% | +79.7% | +84.1% | +289.8% | +339.0% |
| 33 | 59769ns | -2.5% | +84.2% | +100.8% | +304.0% | +368.1% |
| 34 | 59720ns | -5.8% | +85.1% | +108.4% | +305.4% | +367.1% |
| 35 | 62822ns | -5.2% | +76.1% | +95.8% | +295.1% | +345.7% |
| 36 | 56502ns | +5.6% | +108.9% | +113.1% | +333.0% | +394.7% |
| 37 | 53431ns | +11.2% | +113.6% | +124.5% | +357.3% | +416.1% |
| 38 | 54068ns | +10.0% | +119.3% | +121.5% | +347.0% | +406.3% |
| 39 | 57148ns | +4.5% | +99.2% | +111.0% | +319.3% | +377.7% |
| 40 | 54255ns | +9.9% | +108.2% | +125.4% | +346.9% | +408.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | 0.399 | moderate+ |
| warm-clamp-accfit | 0.038 | ok |
| warm-clamp-accfit-dyn | 0.411 | moderate+ |
| warm-clamp-head | 0.692 | HIGH+ (drift/warm-up) |
| warm-clamp-min-lanes | 0.446 | moderate+ |
| warm-clamp-minimum | 0.586 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-clamp-accfit**: won 24/40, lost 15/40
- **warm-clamp-accfit-dyn**: won 0/40, lost 40/40
- **warm-clamp-head**: won 0/40, lost 40/40
- **warm-clamp-min-lanes**: won 0/40, lost 40/40
- **warm-clamp-minimum**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 3.2ns | 59335.6ns | 0.0% |  |
| warm-clamp-accfit | 3.7ns | 57096.5ns | 0.0% |  |
| warm-clamp-accfit-dyn | 4.5ns | 112878.9ns | 0.0% |  |
| warm-clamp-head | 4.6ns | 121448.7ns | 0.0% |  |
| warm-clamp-min-lanes | 6.3ns | 244031.9ns | 0.0% |  |
| warm-clamp-minimum | 5.7ns | 272883.6ns | 0.0% |  |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 53635.9-65490.8 ns)
  53635.9 |##########
  54228.6 |#####
  54821.4 |###############
  55414.1 |#####
  56006.9 |##########
  56599.6 |##########
  57192.4 |
  57785.1 |
  58377.9 |
  58970.6 |##########
  59563.4 |########################################
  60156.1 |###############
  60748.8 |
  61341.6 |#####
  61934.3 |##########
  62527.1 |##########
  63119.8 |
  63712.6 |####################
  64305.3 |#####
  64898.1 |
  (4 below, 2 above range)

warm-clamp-accfit (n=40, range 53006.9-62118.6 ns)
  53006.9 |###########
  53462.5 |############################
  53918.1 |
  54373.6 |###########
  54829.2 |###########
  55284.8 |
  55740.4 |###########
  56196.0 |###########
  56651.6 |#####
  57107.2 |#####
  57562.8 |#####
  58018.3 |###########
  58473.9 |
  58929.5 |#################
  59385.1 |########################################
  59840.7 |
  60296.3 |#####
  60751.9 |
  61207.4 |#####
  61663.0 |
  (5 below, 3 above range)

warm-clamp-accfit-dyn (n=40, range 109737.4-118392.6 ns)
  109737.4 |########################################
  110170.1 |####################
  110602.9 |#####
  111035.6 |#####
  111468.4 |#####
  111901.2 |##########
  112333.9 |##########
  112766.7 |##########
  113199.5 |#####
  113632.2 |##########
  114065.0 |##########
  114497.7 |
  114930.5 |#####
  115363.3 |#####
  115796.0 |
  116228.8 |#####
  116661.6 |##########
  117094.3 |
  117527.1 |
  117959.8 |#####
  (4 below, 4 above range)

warm-clamp-head (n=40, range 110711.4-135283.6 ns)
  110711.4 |#############
  111940.0 |#############
  113168.6 |#############
  114397.2 |######
  115625.8 |#############
  116854.4 |######
  118083.1 |######
  119311.7 |##########################
  120540.3 |######
  121768.9 |####################
  122997.5 |######
  124226.1 |########################################
  125454.7 |######
  126683.3 |
  127911.9 |
  129140.5 |#############
  130369.2 |######
  131597.8 |######
  132826.4 |
  134055.0 |######
  (5 below, 3 above range)

warm-clamp-min-lanes (n=40, range 239362.1-251166.8 ns)
  239362.1 |########################
  239952.3 |########
  240542.5 |########################
  241132.8 |################
  241723.0 |################
  242313.2 |################################
  242903.5 |########
  243493.7 |################
  244083.9 |########################################
  244674.2 |########
  245264.4 |################
  245854.7 |
  246444.9 |
  247035.1 |########
  247625.4 |
  248215.6 |################
  248805.8 |########
  249396.1 |################
  249986.3 |########
  250576.5 |########
  (5 below, 1 above range)

warm-clamp-minimum (n=40, range 269339.2-277549.5 ns)
  269339.2 |########################################
  269749.7 |########
  270160.2 |########
  270570.8 |########################
  270981.3 |########
  271391.8 |########################
  271802.3 |################
  272212.8 |########################
  272623.3 |################################
  273033.9 |
  273444.4 |########################
  273854.9 |########
  274265.4 |########
  274675.9 |################
  275086.4 |########
  275497.0 |########################
  275907.5 |
  276318.0 |
  276728.5 |
  277139.0 |
  (2 below, 4 above range)

```

## Diagnostics

- **warm-clamp-head**: autocorrelation=0.69 (measurement drift or warm-up artifact)
- **warm-clamp-minimum**: autocorrelation=0.59 (measurement drift or warm-up artifact)
