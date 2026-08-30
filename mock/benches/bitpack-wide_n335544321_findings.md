# Packed 13-bit against the u16 carrier with both columns several times past a 12 MB L2, at one and four threads

4 variants, 40 samples per variant.
Baseline: **bitpack-wide-d16**

## Highlights

Baseline for all deltas below: **bitpack-wide-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-wide-d16-padal dominates: 96% faster than the next best (bitpack-wide-pipe4)

bitpack-wide-d16-padal (1.34 ms) leads bitpack-wide-pipe4 (2.64 ms) by 96%, a clear separation rather than a photo finish. CV 9.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-wide-d16-padal beats baseline by 58% (significant)

bitpack-wide-d16-padal is -1.73 ms (58%) faster than baseline bitpack-wide-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-wide-d16-control is an outlier: 2.3x slower than the field

bitpack-wide-d16-control (3.04 ms) is 2.3x the fastest (1.34 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-wide-d16 shows warm-up / thermal drift (autocorr +0.70)

bitpack-wide-d16's per-pass series has lag-1 autocorrelation +0.70, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-wide-d16-padal} vs {bitpack-wide-pipe4, bitpack-wide-d16, bitpack-wide-d16-control} (96% apart)

The field splits into a fast tier {bitpack-wide-d16-padal} and a slow tier {bitpack-wide-pipe4, bitpack-wide-d16, bitpack-wide-d16-control} with a 96% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-wide-d16-padal** at 1344542.9 ns median (-55.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.26x (fastest 1344542.9 ns, slowest 3041899.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-wide-d16 | 3080463ns | 2999388ns | 2879123ns | 3017464ns | 3470798ns | base |
| bitpack-wide-d16-control | 3147273ns | 3044228ns | 2887910ns | 3042366ns | 3721355ns | +2.17% |
| bitpack-wide-d16-padal | 1322928ns | 1346486ns | 1129529ns | 1328055ns | 1500948ns | -57.05% |
| bitpack-wide-pipe4 | 2729441ns | 2641081ns | 2401745ns | 2664143ns | 3253032ns | -11.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-wide-d16 | 3078232ns | 2876775ns | 3468497ns | base | 10.901 |
| bitpack-wide-d16-control | 3144882ns | 2885664ns | 3718847ns | +2.17% | 10.670 |
| bitpack-wide-d16-padal | 1320778ns | 1127174ns | 1498932ns | -57.09% | 25.405 |
| bitpack-wide-pipe4 | 2727142ns | 2399582ns | 3250592ns | -11.41% | 12.304 |

## Performance model

- Peak throughput: **29.769 Gops/s** (bitpack-wide-d16-padal; best 20% batches)
- Ops per call: 33554432

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-wide-d16 | 11.195 | 37.6% |
| bitpack-wide-d16-control | 11.031 | 37.1% |
| bitpack-wide-d16-padal | 24.956 | 83.8% |
| bitpack-wide-pipe4 | 12.715 | 42.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-wide-d16 | 3080463ns | 3080463ns | base |
| bitpack-wide-d16-control | 3147273ns | 3147273ns | +2.17% |
| bitpack-wide-d16-padal | 1322928ns | 1322928ns | -57.05% |
| bitpack-wide-pipe4 | 2729441ns | 2729441ns | -11.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-wide-d16 | 2997297ns | base | --- | [2958074, 3071324] | --- | --- | --- | --- |
| bitpack-wide-d16-control | 3041900ns | no significant difference | [-19907, +46752]ns | [2979595, 3077020] | no | 0.6358 | 0.6358 | 0 |
| bitpack-wide-d16-padal | 1344543ns | -1721113.1ns (-57.4%) | [-1790425, -1677160]ns | [1275092, 1366776] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-wide-pipe4 | 2638883ns | -395543.6ns (-13.2%) | [-440962, -340408]ns | [2538832, 2772699] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-wide-d16 | bitpack-wide-d16-control | bitpack-wide-d16-padal | bitpack-wide-pipe4 |
|---|---|---|---|---|
| 1 | 2996625ns | +0.2% | -46.3% | -6.7% |
| 2 | 2933133ns | +3.3% | -50.2% | -3.0% |
| 3 | 3083166ns | +2.7% | -53.0% | -14.3% |
| 4 | 3145155ns | -2.9% | -56.7% | -12.3% |
| 5 | 3016317ns | +5.9% | -52.3% | -11.7% |
| 6 | 3019432ns | -0.4% | -51.1% | -13.4% |
| 7 | 3023028ns | +1.7% | -54.7% | -11.5% |
| 8 | 3016393ns | +0.0% | -54.5% | -10.8% |
| 9 | 3173798ns | -3.2% | -56.9% | -16.9% |
| 10 | 3105750ns | +0.5% | -56.2% | -12.7% |
| 11 | 3229290ns | +5.2% | -56.1% | -8.5% |
| 12 | 4090019ns | -16.8% | -65.2% | -31.9% |
| 13 | 3697670ns | -14.1% | -63.3% | +5.0% |
| 14 | 3452753ns | -10.8% | -54.6% | -2.4% |
| 15 | 3503615ns | -7.6% | -57.1% | -17.1% |
| 16 | 3313552ns | -7.3% | -61.5% | -9.0% |
| 17 | 3150415ns | +4.8% | -57.3% | -5.2% |
| 18 | 3074403ns | +73.7% | -57.3% | -4.4% |
| 19 | 3165657ns | +7.0% | -56.8% | -9.8% |
| 20 | 3287275ns | -6.6% | -56.8% | -12.1% |
| 21 | 2862749ns | +2.3% | -58.3% | -16.8% |
| 22 | 2969937ns | -1.9% | -62.0% | -19.9% |
| 23 | 2881378ns | -0.8% | -62.5% | -15.4% |
| 24 | 2866853ns | +1.5% | -61.3% | -15.2% |
| 25 | 2926055ns | -1.7% | -62.1% | -17.2% |
| 26 | 2875241ns | -0.6% | -61.7% | -17.4% |
| 27 | 2868222ns | +2.6% | -56.8% | +13.1% |
| 28 | 2858807ns | +0.3% | -57.8% | -12.2% |
| 29 | 2941820ns | -1.5% | -62.1% | -18.8% |
| 30 | 2874892ns | +0.9% | -58.9% | -17.2% |
| 31 | 2980481ns | -0.0% | -50.1% | -15.9% |
| 32 | 2966600ns | +51.5% | -55.1% | -14.6% |
| 33 | 2949462ns | +7.6% | -58.3% | -14.7% |
| 34 | 2930405ns | +4.8% | -56.5% | -15.2% |
| 35 | 2997969ns | +3.5% | -57.5% | -17.4% |
| 36 | 2983243ns | -0.1% | -56.5% | -14.8% |
| 37 | 3068244ns | -3.2% | -55.8% | +17.5% |
| 38 | 2933333ns | +0.6% | -54.2% | -11.8% |
| 39 | 2959030ns | +1.0% | -57.9% | -11.0% |
| 40 | 2957118ns | -1.0% | -57.1% | -11.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-wide-d16 | 0.704 | HIGH+ (drift/warm-up) |
| bitpack-wide-d16-control | 0.192 | ok |
| bitpack-wide-d16-padal | 0.640 | HIGH+ (drift/warm-up) |
| bitpack-wide-pipe4 | 0.358 | moderate+ |

**Consistency summary:**

- **bitpack-wide-d16-control**: won 17/40, lost 21/40
- **bitpack-wide-d16-padal**: won 40/40, lost 0/40
- **bitpack-wide-pipe4**: won 37/40, lost 3/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-wide-d16 | 41.2ns | 3078232.2ns | 0.0% |  |
| bitpack-wide-d16-control | 39.3ns | 3144882.4ns | 0.0% |  |
| bitpack-wide-d16-padal | 19.5ns | 1320778.4ns | 0.0% |  |
| bitpack-wide-pipe4 | 40.5ns | 2727141.9ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-wide-d16 (n=40, range 2876774.6-3468496.6 ns)
  2876774.6 |######
  2906360.7 |##########################
  2935946.8 |##########################
  2965532.9 |##########################
  2995119.0 |########################################
  3024705.1 |
  3054291.2 |####################
  3083877.3 |######
  3113463.4 |
  3143049.5 |####################
  3172635.6 |######
  3202221.7 |######
  3231807.8 |
  3261393.9 |######
  3290980.0 |######
  3320566.1 |
  3350152.2 |
  3379738.3 |
  3409324.4 |
  3438910.5 |######
  (6 below, 3 above range)

bitpack-wide-d16-control (n=40, range 2885663.6-3718847.4 ns)
  2885663.6 |######################
  2927322.8 |############################
  2968982.0 |############################
  3010641.2 |###########
  3052300.4 |########################################
  3093959.5 |###########
  3135618.7 |#################
  3177277.9 |#####
  3218937.1 |#####
  3260596.3 |#####
  3302255.5 |
  3343914.7 |
  3385573.9 |#################
  3427233.1 |
  3468892.3 |
  3510551.4 |
  3552210.6 |
  3593869.8 |
  3635529.0 |
  3677188.2 |
  (4 below, 2 above range)

bitpack-wide-d16-padal (n=40, range 1127174.3-1498931.9 ns)
  1127174.3 |######
  1145762.2 |
  1164350.0 |######
  1182937.9 |######
  1201525.8 |######
  1220113.7 |######
  1238701.6 |#############
  1257289.5 |####################
  1275877.3 |######
  1294465.2 |######
  1313053.1 |#############
  1331641.0 |#############
  1350228.9 |########################################
  1368816.7 |#############
  1387404.6 |
  1405992.5 |####################
  1424580.4 |######
  1443168.3 |#############
  1461756.2 |######
  1480344.0 |######
  (5 below, 3 above range)

bitpack-wide-pipe4 (n=40, range 2399582.3-3250592.4 ns)
  2399582.3 |########################
  2442132.8 |########
  2484683.3 |################################
  2527233.8 |################
  2569784.3 |########
  2612334.8 |########################################
  2654885.3 |########################
  2697435.8 |########
  2739986.3 |########
  2782536.9 |################
  2825087.4 |################
  2867637.9 |################
  2910188.4 |########
  2952738.9 |################
  2995289.4 |########
  3037839.9 |
  3080390.4 |
  3122940.9 |
  3165491.4 |
  3208041.9 |########
  (5 below, 3 above range)

```

## Diagnostics

- **bitpack-wide-d16**: autocorrelation=0.70 (measurement drift or warm-up artifact)
- **bitpack-wide-d16-padal**: autocorrelation=0.64 (measurement drift or warm-up artifact)
