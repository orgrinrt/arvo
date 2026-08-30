# Container fork, declared-width sweep, cache-resident (8192 elements, 3 ops/element, wrapping)

6 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### warm-container-kernel beats baseline by 73% (significant)

warm-container-kernel is -6.11 us (73%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-minimum is an outlier: 3.8x slower than the field

warm-container-minimum (8.45 us) is 3.8x the fastest (2.23 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### warm-container-kernel is fastest but the noisiest (CV 8.8%)

warm-container-kernel wins on median (2.23 us) yet has the highest variance (CV 8.8%), while warm-container-headroom is the steadiest (CV 1.2%, 8.38 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (warm-container-kernel, warm-container-lanes-deferred) are a dead heat (<1%)

warm-container-kernel (2.23 us) and warm-container-lanes-deferred (2.23 us) differ by 0.28%, inside the noise, even though the wider field spreads 279.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-kernel shows warm-up / thermal drift (autocorr +0.91)

warm-container-kernel's per-pass series has lag-1 autocorrelation +0.91, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-kernel, warm-container-lanes-deferred, warm-container-native} vs {warm-container-headroom, warm-container-plusone, warm-container-minimum} (246% apart)

The field splits into a fast tier {warm-container-kernel, warm-container-lanes-deferred, warm-container-native} and a slow tier {warm-container-headroom, warm-container-plusone, warm-container-minimum} with a 246% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.8x the fastest

Fastest warm-container-kernel (2.23 us) to slowest warm-container-minimum (8.45 us): 3.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: warm-container-kernel** at 2226.2 ns median (-73.5% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 3.79x (fastest 2226.2 ns, slowest 8447.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 8509ns | 8479ns | 8406ns | 8490ns | 8667ns | base |
| warm-container-kernel | 2407ns | 2288ns | 2284ns | 2325ns | 2776ns | -71.71% |
| warm-container-lanes-deferred | 2401ns | 2293ns | 2283ns | 2359ns | 2647ns | -71.78% |
| warm-container-minimum | 8715ns | 8511ns | 8347ns | 8531ns | 9633ns | +2.42% |
| warm-container-native | 2582ns | 2488ns | 2474ns | 2527ns | 2855ns | -69.66% |
| warm-container-plusone | 8917ns | 8507ns | 8347ns | 8710ns | 10108ns | +4.80% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 8410ns | 8310ns | 8562ns | base | 3.896 |
| warm-container-kernel | 2341ns | 2221ns | 2700ns | -72.16% | 13.996 |
| warm-container-lanes-deferred | 2335ns | 2221ns | 2570ns | -72.23% | 14.032 |
| warm-container-minimum | 8647ns | 8283ns | 9561ns | +2.81% | 3.790 |
| warm-container-native | 2521ns | 2419ns | 2787ns | -70.03% | 12.999 |
| warm-container-plusone | 8846ns | 8287ns | 10030ns | +5.19% | 3.704 |

## Performance model

- Peak throughput: **14.756 Gops/s** (warm-container-kernel; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 3.908 | 26.5% |
| warm-container-kernel | 14.719 | 99.8% |
| warm-container-lanes-deferred | 14.678 | 99.5% |
| warm-container-minimum | 3.879 | 26.3% |
| warm-container-native | 13.523 | 91.6% |
| warm-container-plusone | 3.881 | 26.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 8509ns | 8509ns | base |
| warm-container-kernel | 2407ns | 2407ns | -71.71% |
| warm-container-lanes-deferred | 2401ns | 2401ns | -71.78% |
| warm-container-minimum | 8715ns | 8715ns | +2.42% |
| warm-container-native | 2582ns | 2582ns | -69.66% |
| warm-container-plusone | 8917ns | 8917ns | +4.80% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 8385ns | base | --- | [8346, 8434] | --- | --- | --- | --- |
| warm-container-kernel | 2226ns | -6133.9ns (-73.2%) | [-6213, -6100]ns | [2225, 2228] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-lanes-deferred | 2232ns | -6096.8ns (-72.7%) | [-6122, -6081]ns | [2226, 2314] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-minimum | 8447ns | no significant difference | [-40, +148]ns | [8385, 8543] | no | 0.3352 | 0.2682 | 0 |
| warm-container-native | 2423ns | -5903.3ns (-70.4%) | [-5937, -5887]ns | [2422, 2489] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 8443ns | no significant difference | [-69, +130]ns | [8336, 8556] | no | 0.8746 | 0.8746 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | warm-container-headroom | warm-container-kernel | warm-container-lanes-deferred | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|---|
| 1 | 8642ns | -74.2% | -70.8% | -4.0% | -66.0% | -0.7% |
| 2 | 8492ns | -73.8% | -69.4% | -2.5% | -66.1% | +0.4% |
| 3 | 8698ns | -74.4% | -71.0% | -4.8% | -68.5% | -2.1% |
| 4 | 8579ns | -74.1% | -70.6% | -3.3% | -68.0% | -1.7% |
| 5 | 8504ns | -73.8% | -70.3% | -1.4% | -67.7% | -0.5% |
| 6 | 8358ns | -73.4% | -69.8% | -0.9% | -67.2% | +0.1% |
| 7 | 8512ns | -73.9% | -70.4% | -2.4% | -67.7% | -2.6% |
| 8 | 8375ns | -73.5% | -69.8% | -1.1% | -69.8% | -0.5% |
| 9 | 8428ns | -73.6% | -70.1% | -1.7% | -69.7% | -0.9% |
| 10 | 8336ns | -73.3% | -69.0% | +1.1% | -66.9% | -0.6% |
| 11 | 8363ns | -73.4% | -73.4% | +4.2% | -71.1% | -1.0% |
| 12 | 8395ns | -73.5% | -73.5% | -0.6% | -70.2% | -1.3% |
| 13 | 8454ns | -73.7% | -73.6% | +1.1% | -71.3% | -2.0% |
| 14 | 8490ns | -73.8% | -73.7% | -0.8% | -71.5% | -2.4% |
| 15 | 8530ns | -73.0% | -73.3% | -0.7% | -71.6% | -2.3% |
| 16 | 8424ns | -71.9% | -72.2% | -0.5% | -71.3% | -1.5% |
| 17 | 8410ns | -73.5% | -72.6% | +1.8% | -71.2% | -1.4% |
| 18 | 8313ns | -73.0% | -72.2% | +0.5% | -70.9% | +0.2% |
| 19 | 8441ns | -73.7% | -73.2% | -1.9% | -71.3% | -1.8% |
| 20 | 8348ns | -73.4% | -67.0% | -0.5% | -71.1% | +1.2% |
| 21 | 8314ns | -73.3% | -73.3% | +8.9% | -70.9% | -0.1% |
| 22 | 8515ns | -73.9% | -73.9% | +18.0% | -71.5% | +16.3% |
| 23 | 8330ns | -73.3% | -73.2% | +20.6% | -70.9% | +20.8% |
| 24 | 8464ns | -73.8% | -73.6% | +18.7% | -71.4% | +11.9% |
| 25 | 8519ns | -73.9% | -73.8% | +14.2% | -71.6% | -1.2% |
| 26 | 8464ns | -73.7% | -73.7% | +11.0% | -71.4% | -0.2% |
| 27 | 8334ns | -73.3% | -73.3% | +2.6% | -71.0% | +1.9% |
| 28 | 8307ns | -73.2% | -73.2% | -0.4% | -69.9% | +0.7% |
| 29 | 8405ns | -73.6% | -73.6% | +1.7% | -71.2% | -1.3% |
| 30 | 8344ns | -73.3% | -73.4% | +12.6% | -71.0% | -0.4% |
| 31 | 8354ns | -71.4% | -69.9% | +0.7% | -70.9% | +18.2% |
| 32 | 8312ns | -67.5% | -72.1% | +1.4% | -70.9% | +20.9% |
| 33 | 8404ns | -67.9% | -73.6% | +1.2% | -71.2% | +19.6% |
| 34 | 8323ns | -67.6% | -73.3% | +5.4% | -70.9% | +14.3% |
| 35 | 8311ns | -67.5% | -73.3% | +4.8% | -70.8% | +19.5% |
| 36 | 8308ns | -67.5% | -73.3% | +2.6% | -70.2% | +21.0% |
| 37 | 8308ns | -67.3% | -73.2% | +1.6% | -70.9% | +21.6% |
| 38 | 8345ns | -67.7% | -73.4% | +2.4% | -71.0% | +21.0% |
| 39 | 8310ns | -67.5% | -73.3% | +1.7% | -67.6% | +13.4% |
| 40 | 8347ns | -67.7% | -73.3% | +1.4% | -67.1% | +12.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.435 | moderate+ |
| warm-container-kernel | 0.905 | HIGH+ (drift/warm-up) |
| warm-container-lanes-deferred | 0.517 | HIGH+ (drift/warm-up) |
| warm-container-minimum | 0.762 | HIGH+ (drift/warm-up) |
| warm-container-native | 0.702 | HIGH+ (drift/warm-up) |
| warm-container-plusone | 0.803 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **warm-container-kernel**: won 40/40, lost 0/40
- **warm-container-lanes-deferred**: won 40/40, lost 0/40
- **warm-container-minimum**: won 16/40, lost 24/40
- **warm-container-native**: won 40/40, lost 0/40
- **warm-container-plusone**: won 20/40, lost 19/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 3.1ns | 8410.2ns | 0.0% |  |
| warm-container-kernel | 2.1ns | 2341.2ns | 0.1% |  |
| warm-container-lanes-deferred | 2.4ns | 2335.3ns | 0.1% |  |
| warm-container-minimum | 2.9ns | 8646.8ns | 0.0% |  |
| warm-container-native | 2.5ns | 2520.8ns | 0.1% |  |
| warm-container-plusone | 2.8ns | 8846.3ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 8310.3-8562.3 ns)
   8310.3 |################################
   8322.9 |########################
   8335.5 |########################################
   8348.1 |################
   8360.7 |########
   8373.3 |########
   8385.9 |########
   8398.5 |########################
   8411.1 |
   8423.7 |################
   8436.3 |########
   8448.9 |########
   8461.5 |################
   8474.1 |
   8486.7 |################
   8499.3 |################
   8511.9 |################
   8524.5 |########
   8537.1 |
   8549.7 |
  (4 below, 3 above range)

warm-container-kernel (n=40, range 2220.6-2700.4 ns)
   2220.6 |########################################
   2244.6 |
   2268.6 |
   2292.6 |#
   2316.6 |
   2340.6 |
   2364.6 |#
   2388.6 |#
   2412.5 |
   2436.5 |
   2460.5 |
   2484.5 |
   2508.5 |
   2532.5 |
   2556.5 |
   2580.5 |
   2604.4 |
   2628.4 |
   2652.4 |
   2676.4 |############
  (5 below, 2 above range)

warm-container-lanes-deferred (n=40, range 2221.1-2570.0 ns)
   2221.1 |########################################
   2238.5 |
   2256.0 |##
   2273.4 |##
   2290.9 |##
   2308.3 |####
   2325.8 |
   2343.2 |##
   2360.7 |
   2378.1 |
   2395.5 |
   2413.0 |
   2430.4 |
   2447.9 |
   2465.3 |
   2482.8 |
   2500.2 |
   2517.7 |##################
   2535.1 |
   2552.5 |
  (2 below, 3 above range)

warm-container-minimum (n=40, range 8283.5-9561.5 ns)
   8283.5 |########################################
   8347.4 |###############
   8411.3 |########################################
   8475.2 |##########
   8539.1 |#########################
   8603.0 |
   8666.9 |##########
   8730.8 |#####
   8794.7 |
   8858.6 |
   8922.5 |
   8986.4 |
   9050.3 |#####
   9114.2 |
   9178.1 |
   9242.0 |
   9305.9 |
   9369.8 |##########
   9433.7 |
   9497.6 |
  (4 below, 4 above range)

warm-container-native (n=40, range 2418.8-2787.0 ns)
   2418.8 |########################################
   2437.2 |
   2455.6 |
   2474.0 |#
   2492.4 |###
   2510.8 |#
   2529.2 |
   2547.6 |#
   2566.1 |
   2584.5 |
   2602.9 |
   2621.3 |
   2639.7 |
   2658.1 |
   2676.5 |#
   2694.9 |
   2713.3 |
   2731.7 |###########
   2750.1 |#
   2768.6 |
  (4 below, 2 above range)

warm-container-plusone (n=40, range 8287.5-10029.9 ns)
   8287.5 |########################################
   8374.6 |#############
   8461.7 |########
   8548.8 |##
   8635.9 |
   8723.1 |
   8810.2 |
   8897.3 |
   8984.4 |
   9071.6 |
   9158.7 |
   9245.8 |
   9332.9 |##
   9420.0 |#####
   9507.2 |##
   9594.3 |
   9681.4 |
   9768.5 |
   9855.6 |########
   9942.8 |
  (3 below, 6 above range)

```

## Diagnostics

- **warm-container-kernel**: autocorrelation=0.91 (measurement drift or warm-up artifact)
- **warm-container-lanes-deferred**: autocorrelation=0.52 (measurement drift or warm-up artifact)
- **warm-container-minimum**: autocorrelation=0.76 (measurement drift or warm-up artifact)
- **warm-container-native**: autocorrelation=0.70 (measurement drift or warm-up artifact)
- **warm-container-plusone**: autocorrelation=0.80 (measurement drift or warm-up artifact)
