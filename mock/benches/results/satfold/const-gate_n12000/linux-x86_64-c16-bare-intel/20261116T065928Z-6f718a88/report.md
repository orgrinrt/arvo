# Does a const gate erase in time: the licensed arm reached directly, the same arm reached through a const verdict computed by an exhaustive sweep in a const fn, and the same gate over a law that is false so it selects the fallback

5 variants, 40 samples per variant.
Baseline: **satfold-gate-false**

## Highlights

Baseline for all deltas below: **satfold-gate-false**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (satfold-gate-false) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline satfold-gate-false has the worst median (27.28 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest satfold-lanes16-3 at 2.09 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### satfold-lanes16-3 beats baseline by 92% (significant)

satfold-lanes16-3 is -25.19 us (92%) faster than baseline satfold-gate-false, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### satfold-gate-false is an outlier: 13.1x slower than the field

satfold-gate-false (27.28 us) is 13.1x the fastest (2.09 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### satfold-lanes16 shows alternating (throttle bounce) (autocorr -0.58)

satfold-lanes16's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {satfold-lanes16-3, satfold-lanes16, satfold-gate-true} vs {satfold-seq, satfold-gate-false} (994% apart)

The field splits into a fast tier {satfold-lanes16-3, satfold-lanes16, satfold-gate-true} and a slow tier {satfold-seq, satfold-gate-false} with a 994% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 13.1x the fastest

Fastest satfold-lanes16-3 (2.09 us) to slowest satfold-gate-false (27.28 us): 13.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### satfold-gate-true is inconsistent: worst-20% is 1.6x its best-20%

satfold-gate-true's best 20% of batches run at 1.89 us but its worst 20% at 2.92 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: satfold-lanes16-3** at 2088.9 ns median (-92.3% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 13.06x (fastest 2088.9 ns, slowest 27280.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Does a const gate erase in time: the licensed arm reached directly, the same arm reached through a const verdict computed by an exhaustive sweep in a const fn, and the same gate over a law that is false so it selects the fallback |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112736210 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| satfold-gate-false | 26113ns | 27350ns | 22374ns | 26936ns | 27384ns | base |
| satfold-gate-true | 2556ns | 2532ns | 2082ns | 2472ns | 3280ns | -90.21% |
| satfold-lanes16 | 2538ns | 2470ns | 2141ns | 2503ns | 3042ns | -90.28% |
| satfold-lanes16-3 | 2350ns | 2307ns | 2047ns | 2328ns | 2719ns | -91.00% |
| satfold-seq | 24875ns | 24893ns | 22375ns | 24874ns | 27378ns | -4.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| satfold-gate-false | 26044ns | 22318ns | 27299ns | base | 1.258 |
| satfold-gate-true | 2301ns | 1886ns | 2924ns | -91.16% | 14.240 |
| satfold-lanes16 | 2276ns | 1930ns | 2712ns | -91.26% | 14.400 |
| satfold-lanes16-3 | 2118ns | 1853ns | 2422ns | -91.87% | 15.473 |
| satfold-seq | 24805ns | 22316ns | 27294ns | -4.76% | 1.321 |

## Performance model

- Peak throughput: **17.684 Gops/s** (satfold-lanes16-3; best 20% batches)
- Ops per call: 32768

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| satfold-gate-false | 1.201 | 6.8% |
| satfold-gate-true | 14.446 | 81.7% |
| satfold-lanes16 | 14.877 | 84.1% |
| satfold-lanes16-3 | 15.687 | 88.7% |
| satfold-seq | 1.321 | 7.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| satfold-gate-false | 26113ns | 26113ns | base |
| satfold-gate-true | 2556ns | 2556ns | -90.21% |
| satfold-lanes16 | 2538ns | 2538ns | -90.28% |
| satfold-lanes16-3 | 2350ns | 2350ns | -91.00% |
| satfold-seq | 24875ns | 24875ns | -4.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| satfold-gate-false | 27281ns | base | --- | [27274, 27284] | --- | --- | --- | --- |
| satfold-gate-true | 2268ns | -24796.7ns (-90.9%) | [-24985, -24371]ns | [2090, 2326] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16 | 2203ns | -25072.8ns (-91.9%) | [-25084, -24840]ns | [2200, 2303] | YES | 0.0000 | 0.0000 | 0 |
| satfold-lanes16-3 | 2089ns | -25186.7ns (-92.3%) | [-25199, -24990]ns | [2083, 2121] | YES | 0.0000 | 0.0000 | 0 |
| satfold-seq | 24808ns | no significant difference | [-5, +4]ns | [22321, 27287] | no | 0.3368 | 0.3368 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | satfold-gate-false | satfold-gate-true | satfold-lanes16 | satfold-lanes16-3 | satfold-seq |
|---|---|---|---|---|---|---|
| 1 | 1 | 27284ns | -89.8% | -91.2% | -92.6% | +0.0% |
| 1 | 2 | 27283ns | -91.5% | -91.9% | -92.7% | -18.2% |
| 1 | 3 | 22317ns | -90.7% | -88.9% | -89.7% | +0.0% |
| 1 | 4 | 27286ns | -92.8% | -92.7% | -92.0% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| satfold-gate-false | -0.417 | moderate- |
| satfold-gate-true | 0.201 | moderate+ |
| satfold-lanes16 | -0.583 | HIGH- (thermal bounce) |
| satfold-lanes16-3 | 0.036 | ok |
| satfold-seq | -0.250 | moderate- |

**Consistency summary:**

- **satfold-gate-true**: won 4/4, lost 0/4
- **satfold-lanes16**: won 4/4, lost 0/4
- **satfold-lanes16-3**: won 4/4, lost 0/4
- **satfold-seq**: won 1/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| satfold-gate-false | 22.2ns | 26043.7ns | 0.1% |  |
| satfold-gate-true | 83.9ns | 2301.2ns | 3.6% |  |
| satfold-lanes16 | 87.9ns | 2275.6ns | 3.9% |  |
| satfold-lanes16-3 | 78.7ns | 2117.8ns | 3.7% |  |
| satfold-seq | 22.2ns | 24804.9ns | 0.1% |  |

## Distribution (algo ns)

```
satfold-gate-false (n=40, range 22318.3-27299.3 ns)
  22318.3 |#####
  22567.3 |
  22816.4 |
  23065.4 |
  23314.5 |
  23563.5 |
  23812.6 |
  24061.6 |
  24310.7 |
  24559.7 |
  24808.8 |
  25057.8 |
  25306.9 |
  25556.0 |
  25805.0 |
  26054.1 |
  26303.1 |
  26552.2 |
  26801.2 |
  27050.3 |########################################
  (6 below, 1 above range)

satfold-gate-true (n=40, range 1886.0-2923.6 ns)
   1886.0 |#################################
   1937.9 |####################
   1989.7 |
   2041.6 |#################################
   2093.5 |####################
   2145.4 |######
   2197.3 |######
   2249.2 |########################################
   2301.0 |######
   2352.9 |######
   2404.8 |#############
   2456.7 |
   2508.6 |####################
   2560.5 |######
   2612.3 |
   2664.2 |######
   2716.1 |
   2768.0 |
   2819.9 |
   2871.8 |######
  (2 below, 4 above range)

satfold-lanes16 (n=40, range 1930.3-2711.7 ns)
   1930.3 |####
   1969.4 |################
   2008.5 |####
   2047.5 |####
   2086.6 |
   2125.7 |####
   2164.8 |########################################
   2203.8 |########
   2242.9 |########
   2282.0 |####
   2321.0 |########
   2360.1 |
   2399.2 |
   2438.2 |####################
   2477.3 |####
   2516.4 |
   2555.4 |
   2594.5 |####
   2633.6 |
   2672.6 |
  (3 below, 5 above range)

satfold-lanes16-3 (n=40, range 1853.0-2422.5 ns)
   1853.0 |###
   1881.4 |###
   1909.9 |################
   1938.4 |
   1966.9 |###
   1995.3 |
   2023.8 |######
   2052.3 |###
   2080.8 |########################################
   2109.2 |
   2137.7 |######
   2166.2 |
   2194.7 |
   2223.1 |
   2251.6 |###
   2280.1 |################
   2308.6 |
   2337.0 |##########
   2365.5 |
   2394.0 |
  (3 below, 3 above range)

satfold-seq (n=40, range 22316.3-27293.9 ns)
  22316.3 |#####################################
  22565.2 |
  22814.1 |
  23063.0 |
  23311.9 |
  23560.7 |
  23809.6 |
  24058.5 |
  24307.4 |
  24556.2 |
  24805.1 |
  25054.0 |
  25302.9 |
  25551.7 |
  25800.6 |
  26049.5 |
  26298.4 |
  26547.2 |
  26796.1 |
  27045.0 |########################################
  (3 below, 2 above range)

```
