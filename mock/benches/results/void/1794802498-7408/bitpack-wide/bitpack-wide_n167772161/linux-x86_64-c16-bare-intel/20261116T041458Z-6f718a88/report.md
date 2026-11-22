# Packed 13-bit against the u16 carrier with both columns several times past a 12 MB L2, at one and four threads

4 variants, 40 samples per variant.
Baseline: **bitpack-wide-d16**

## Highlights

Baseline for all deltas below: **bitpack-wide-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-wide-d16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-wide-d16 has the worst median (2.93 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-wide-pipe4 at 2.34 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-wide-pipe4 dominates: 25% faster than the next best (bitpack-wide-d16-padal)

bitpack-wide-pipe4 (2.34 ms) leads bitpack-wide-d16-padal (2.92 ms) by 25%, a clear separation rather than a photo finish. CV 5.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-wide-pipe4 beats baseline by 20% (significant)

bitpack-wide-pipe4 is -596.32 us (20%) faster than baseline bitpack-wide-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-wide-d16-padal shows alternating (throttle bounce) (autocorr -0.68)

bitpack-wide-d16-padal's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-wide-pipe4} vs {bitpack-wide-d16-padal, bitpack-wide-d16-control, bitpack-wide-d16} (25% apart)

The field splits into a fast tier {bitpack-wide-pipe4} and a slow tier {bitpack-wide-d16-padal, bitpack-wide-d16-control, bitpack-wide-d16} with a 25% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-wide-pipe4** at 2338172.2 ns median (-20.2% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.25x (fastest 2338172.2 ns, slowest 2929808.2 ns)

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
| Counter frequency | 2112726093 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-wide-d16 | 3029605ns | 2930913ns | 2919867ns | 2935095ns | 3422873ns | base |
| bitpack-wide-d16-control | 2982153ns | 2927906ns | 2911727ns | 2927667ns | 3216037ns | -1.57% |
| bitpack-wide-d16-padal | 2932352ns | 2924822ns | 2918163ns | 2925183ns | 2968045ns | -3.21% |
| bitpack-wide-pipe4 | 2380577ns | 2339095ns | 2328386ns | 2339780ns | 2555161ns | -21.42% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-wide-d16 | 3028552ns | 2918743ns | 3421947ns | base | 5.540 |
| bitpack-wide-d16-control | 2980938ns | 2910471ns | 3214885ns | -1.57% | 5.628 |
| bitpack-wide-d16-padal | 2931445ns | 2917296ns | 2967077ns | -3.21% | 5.723 |
| bitpack-wide-pipe4 | 2379589ns | 2327379ns | 2554289ns | -21.43% | 7.050 |

## Performance model

- Peak throughput: **7.209 Gops/s** (bitpack-wide-pipe4; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-wide-d16 | 5.726 | 79.4% |
| bitpack-wide-d16-control | 5.733 | 79.5% |
| bitpack-wide-d16-padal | 5.738 | 79.6% |
| bitpack-wide-pipe4 | 7.175 | 99.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-wide-d16 | 3029605ns | 3029605ns | base |
| bitpack-wide-d16-control | 2982153ns | 2982153ns | -1.57% |
| bitpack-wide-d16-padal | 2932352ns | 2932352ns | -3.21% |
| bitpack-wide-pipe4 | 2380577ns | 2380577ns | -21.42% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-wide-d16 | 2929808ns | base | --- | [2924096, 2939672] | --- | --- | --- | --- |
| bitpack-wide-d16-control | 2926598ns | -6852.4ns (-0.2%) | [-9326, -1348]ns | [2923355, 2929581] | YES | 0.0096 | 0.0064 | 0 |
| bitpack-wide-d16-padal | 2923925ns | -6789.6ns (-0.2%) | [-13803, -1135]ns | [2921493, 2926529] | YES (adj: no) | 0.0807 | 0.0807 | 0 |
| bitpack-wide-pipe4 | 2338172ns | -596322.3ns (-20.4%) | [-604819, -588785]ns | [2336590, 2340372] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-wide-d16 | bitpack-wide-d16-control | bitpack-wide-d16-padal | bitpack-wide-pipe4 |
|---|---|---|---|---|---|
| 1 | 1 | 2927023ns | +0.1% | -0.2% | -20.1% |
| 1 | 2 | 2921129ns | -0.2% | +0.2% | -19.7% |
| 1 | 3 | 2935438ns | -0.4% | -0.6% | -20.6% |
| 1 | 4 | 3414452ns | -14.2% | -14.2% | -31.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-wide-d16 | -0.063 | ok |
| bitpack-wide-d16-control | -0.386 | moderate- |
| bitpack-wide-d16-padal | -0.682 | HIGH- (thermal bounce) |
| bitpack-wide-pipe4 | -0.397 | moderate- |

**Consistency summary:**

- **bitpack-wide-d16-control**: won 3/4, lost 0/4
- **bitpack-wide-d16-padal**: won 3/4, lost 1/4
- **bitpack-wide-pipe4**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-wide-d16 | 54.7ns | 3028551.9ns | 0.0% |  |
| bitpack-wide-d16-control | 66.6ns | 2980938.5ns | 0.0% |  |
| bitpack-wide-d16-padal | 40.9ns | 2931444.8ns | 0.0% |  |
| bitpack-wide-pipe4 | 26.2ns | 2379588.6ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-wide-d16 (n=40, range 2918742.8-3421946.6 ns)
  2918742.8 |########################################
  2943903.0 |######
  2969063.2 |
  2994223.4 |
  3019383.6 |
  3044543.8 |
  3069704.0 |
  3094864.2 |
  3120024.3 |
  3145184.5 |
  3170344.7 |
  3195504.9 |
  3220665.1 |
  3245825.3 |
  3270985.5 |
  3296145.6 |
  3321305.8 |
  3346466.0 |
  3371626.2 |
  3396786.4 |######
  (3 below, 4 above range)

bitpack-wide-d16-control (n=40, range 2910471.3-3214884.7 ns)
  2910471.3 |########################################
  2925691.9 |########################################
  2940912.6 |#####
  2956133.3 |
  2971353.9 |
  2986574.6 |
  3001795.3 |
  3017015.9 |
  3032236.6 |
  3047457.3 |##
  3062678.0 |
  3077898.6 |
  3093119.3 |
  3108340.0 |
  3123560.6 |
  3138781.3 |
  3154002.0 |
  3169222.6 |
  3184443.3 |
  3199664.0 |
  (3 below, 4 above range)

bitpack-wide-d16-padal (n=40, range 2917296.4-2967076.7 ns)
  2917296.4 |###################################
  2919785.4 |##############################
  2922274.4 |########################################
  2924763.4 |####################
  2927252.4 |##########
  2929741.5 |###############
  2932230.5 |
  2934719.5 |##########
  2937208.5 |
  2939697.5 |
  2942186.5 |##########
  2944675.6 |
  2947164.6 |
  2949653.6 |
  2952142.6 |#####
  2954631.6 |
  2957120.6 |#####
  2959609.7 |
  2962098.7 |
  2964587.7 |
  (3 below, 1 above range)

bitpack-wide-pipe4 (n=40, range 2327378.6-2554289.5 ns)
  2327378.6 |########################################
  2338724.1 |############
  2350069.7 |######
  2361415.2 |
  2372760.7 |
  2384106.3 |
  2395451.8 |
  2406797.4 |
  2418142.9 |
  2429488.5 |
  2440834.0 |
  2452179.6 |
  2463525.1 |
  2474870.6 |
  2486216.2 |
  2497561.7 |
  2508907.3 |
  2520252.8 |
  2531598.4 |
  2542943.9 |
  (2 below, 4 above range)

```
