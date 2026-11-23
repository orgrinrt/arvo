# Packed 13-bit against the u16 carrier with both columns several times past a 12 MB L2, at one and four threads

4 variants, 40 samples per variant.
Baseline: **bitpack-wide-d16**

## Highlights

Baseline for all deltas below: **bitpack-wide-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-wide-pipe4 dominates: 26% faster than the next best (bitpack-wide-d16)

bitpack-wide-pipe4 (2.33 ms) leads bitpack-wide-d16 (2.92 ms) by 26%, a clear separation rather than a photo finish. CV 1.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-wide-pipe4 beats baseline by 20% (significant)

bitpack-wide-pipe4 is -591.23 us (20%) faster than baseline bitpack-wide-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-wide-d16-control shows alternating (throttle bounce) (autocorr -0.51)

bitpack-wide-d16-control's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-wide-pipe4} vs {bitpack-wide-d16, bitpack-wide-d16-control, bitpack-wide-d16-padal} (26% apart)

The field splits into a fast tier {bitpack-wide-pipe4} and a slow tier {bitpack-wide-d16, bitpack-wide-d16-control, bitpack-wide-d16-padal} with a 26% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-wide-pipe4** at 2327694.1 ns median (-20.4% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.26x (fastest 2327694.1 ns, slowest 2932100.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Packed 13-bit against the u16 carrier with both columns several times past a 12 MB L2, at one and four threads |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112815569 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-wide-d16 | 2925767ns | 2923454ns | 2916138ns | 2924409ns | 2939469ns | base |
| bitpack-wide-d16-control | 2938072ns | 2924098ns | 2915167ns | 2924331ns | 3002201ns | +0.42% |
| bitpack-wide-d16-padal | 3099154ns | 2933187ns | 2916858ns | 3052877ns | 3420281ns | +5.93% |
| bitpack-wide-pipe4 | 2337311ns | 2328774ns | 2322359ns | 2329898ns | 2374502ns | -20.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-wide-d16 | 2924737ns | 2915097ns | 2938421ns | base | 5.736 |
| bitpack-wide-d16-control | 2937066ns | 2914139ns | 3001199ns | +0.42% | 5.712 |
| bitpack-wide-d16-padal | 3098173ns | 2915840ns | 3419356ns | +5.93% | 5.415 |
| bitpack-wide-pipe4 | 2336291ns | 2321321ns | 2373439ns | -20.12% | 7.181 |

## Performance model

- Peak throughput: **7.227 Gops/s** (bitpack-wide-pipe4; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-wide-d16 | 5.741 | 79.4% |
| bitpack-wide-d16-control | 5.740 | 79.4% |
| bitpack-wide-d16-padal | 5.722 | 79.2% |
| bitpack-wide-pipe4 | 7.208 | 99.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-wide-d16 | 2925767ns | 2925767ns | base |
| bitpack-wide-d16-control | 2938072ns | 2938072ns | +0.42% |
| bitpack-wide-d16-padal | 3099154ns | 3099154ns | +5.93% |
| bitpack-wide-pipe4 | 2337311ns | 2337311ns | -20.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-wide-d16 | 2922479ns | base | --- | [2920640, 2927127] | --- | --- | --- | --- |
| bitpack-wide-d16-control | 2923099ns | no significant difference | [-2903, +2998]ns | [2919752, 2927018] | no | 0.2682 | 0.2682 | 0 |
| bitpack-wide-d16-padal | 2932101ns | +7698.6ns (+0.3%) | [+3441, +328801]ns | [2924746, 3258254] | YES | 0.0010 | 0.0007 | 0 |
| bitpack-wide-pipe4 | 2327694ns | -591229.8ns (-20.2%) | [-594096, -590179]ns | [2326621, 2330781] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-wide-d16 | bitpack-wide-d16-control | bitpack-wide-d16-padal | bitpack-wide-pipe4 |
|---|---|---|---|---|---|
| 1 | 1 | 2919716ns | +0.1% | +0.3% | -20.2% |
| 1 | 2 | 2924898ns | +0.2% | -0.1% | -20.5% |
| 1 | 3 | 2924303ns | -0.3% | +15.6% | -20.4% |
| 1 | 4 | 2922449ns | +0.0% | +0.2% | -20.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-wide-d16 | -0.245 | moderate- |
| bitpack-wide-d16-control | -0.509 | HIGH- (thermal bounce) |
| bitpack-wide-d16-padal | -0.423 | moderate- |
| bitpack-wide-pipe4 | -0.067 | ok |

**Consistency summary:**

- **bitpack-wide-d16-control**: won 1/4, lost 2/4
- **bitpack-wide-d16-padal**: won 1/4, lost 3/4
- **bitpack-wide-pipe4**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-wide-d16 | 123.3ns | 2924736.7ns | 0.0% |  |
| bitpack-wide-d16-control | 112.4ns | 2937066.2ns | 0.0% |  |
| bitpack-wide-d16-padal | 111.0ns | 3098172.8ns | 0.0% |  |
| bitpack-wide-pipe4 | 103.7ns | 2336290.7ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-wide-d16 (n=40, range 2915097.4-2938420.9 ns)
  2915097.4 |##########################
  2916263.6 |#############
  2917429.8 |#############
  2918595.9 |####################
  2919762.1 |######
  2920928.3 |########################################
  2922094.5 |######
  2923260.7 |##########################
  2924426.8 |
  2925593.0 |######
  2926759.2 |######
  2927925.4 |####################
  2929091.5 |##########################
  2930257.7 |
  2931423.9 |######
  2932590.1 |
  2933756.2 |######
  2934922.4 |
  2936088.6 |######
  2937254.8 |
  (2 below, 3 above range)

bitpack-wide-d16-control (n=40, range 2914139.0-3001199.2 ns)
  2914139.0 |########################################
  2918492.0 |###############################
  2922845.0 |##########################
  2927198.0 |##########################
  2931551.0 |########
  2935904.0 |#################
  2940257.0 |
  2944610.0 |
  2948963.1 |
  2953316.1 |
  2957669.1 |
  2962022.1 |
  2966375.1 |
  2970728.1 |
  2975081.1 |
  2979434.1 |####
  2983787.1 |
  2988140.2 |
  2992493.2 |
  2996846.2 |
  (4 below, 1 above range)

bitpack-wide-d16-padal (n=40, range 2915839.7-3419355.6 ns)
  2915839.7 |########################################
  2941015.5 |##
  2966191.3 |
  2991367.1 |##
  3016542.9 |
  3041718.7 |
  3066894.5 |
  3092070.3 |
  3117246.1 |
  3142421.9 |
  3167597.7 |
  3192773.5 |##
  3217949.3 |
  3243125.0 |
  3268300.8 |
  3293476.6 |##
  3318652.4 |
  3343828.2 |
  3369004.0 |############
  3394179.8 |############
  (4 below, 1 above range)

bitpack-wide-pipe4 (n=40, range 2321320.7-2373438.7 ns)
  2321320.7 |####
  2323926.6 |########################################
  2326532.5 |####################################
  2329138.4 |####################
  2331744.3 |############
  2334350.2 |
  2336956.1 |############
  2339562.0 |
  2342167.9 |
  2344773.8 |########
  2347379.7 |####
  2349985.6 |####
  2352591.5 |####
  2355197.4 |
  2357803.3 |
  2360409.2 |
  2363015.1 |
  2365621.0 |
  2368226.9 |
  2370832.8 |
  (3 below, 1 above range)

```
