# Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-d32 beats baseline by 22% (significant)

bitpack-contend-d32 is -20.37 us (22%) faster than baseline bitpack-contend-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-contend-d64 is an outlier: 2.0x slower than the field

bitpack-contend-d64 (133.83 us) is 2.0x the fastest (65.47 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (bitpack-contend-packed, bitpack-contend-d32) are a dead heat (<1%)

bitpack-contend-packed (65.47 us) and bitpack-contend-d32 (65.67 us) differ by 0.30%, inside the noise, even though the wider field spreads 104.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### bitpack-contend-d16 shows alternating (throttle bounce) (autocorr -0.78)

bitpack-contend-d16's per-pass series has lag-1 autocorrelation -0.78, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader bitpack-contend-packed vs stability leader bitpack-contend-packed-simd (+10% speed for 1.4x steadier)

bitpack-contend-packed is fastest (65.47 us, CV 14.1%); bitpack-contend-packed-simd gives up 9.8% median for 1.4x lower variance (CV 10.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### bitpack-contend-d64 is inconsistent: worst-20% is 1.9x its best-20%

bitpack-contend-d64's best 20% of batches run at 81.95 us but its worst 20% at 156.27 us (1.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-contend-packed** at 65474.6 ns median (-28.2% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.04x (fastest 65474.6 ns, slowest 133828.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112749291 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 93182ns | 91214ns | 80687ns | 91439ns | 110904ns | base |
| bitpack-contend-d16-control | 101839ns | 102952ns | 85102ns | 100441ns | 122774ns | +9.29% |
| bitpack-contend-d32 | 67930ns | 65869ns | 60559ns | 66584ns | 79340ns | -27.10% |
| bitpack-contend-d64 | 124203ns | 134368ns | 82256ns | 127303ns | 156849ns | +33.29% |
| bitpack-contend-packed | 70616ns | 65548ns | 64973ns | 66766ns | 87808ns | -24.22% |
| bitpack-contend-packed-simd | 72370ns | 71954ns | 64968ns | 72169ns | 80376ns | -22.33% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 93106ns | 80614ns | 110828ns | base | 11.262 |
| bitpack-contend-d16-control | 101764ns | 85032ns | 122693ns | +9.30% | 10.304 |
| bitpack-contend-d32 | 67751ns | 60392ns | 79156ns | -27.23% | 15.477 |
| bitpack-contend-d64 | 123772ns | 81954ns | 156273ns | +32.94% | 8.472 |
| bitpack-contend-packed | 70538ns | 64894ns | 87730ns | -24.24% | 14.865 |
| bitpack-contend-packed-simd | 72296ns | 64892ns | 80300ns | -22.35% | 14.504 |

## Performance model

- Peak throughput: **17.363 Gops/s** (bitpack-contend-d32; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 11.507 | 66.3% |
| bitpack-contend-d16-control | 10.192 | 58.7% |
| bitpack-contend-d32 | 15.968 | 92.0% |
| bitpack-contend-d64 | 7.835 | 45.1% |
| bitpack-contend-packed | 16.015 | 92.2% |
| bitpack-contend-packed-simd | 14.587 | 84.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 93182ns | 93182ns | base |
| bitpack-contend-d16-control | 101839ns | 101839ns | +9.29% |
| bitpack-contend-d32 | 67930ns | 67930ns | -27.10% |
| bitpack-contend-d64 | 124203ns | 124203ns | +33.29% |
| bitpack-contend-packed | 70616ns | 70616ns | -24.22% |
| bitpack-contend-packed-simd | 72370ns | 72370ns | -22.33% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 91127ns | base | --- | [81223, 99533] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 102880ns | no significant difference | [-3965, +21901]ns | [100270, 103987] | no | 0.1539 | 0.1539 | 0 |
| bitpack-contend-d32 | 65668ns | -20367.7ns (-22.4%) | [-20667, -20155]ns | [65474, 66140] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-d64 | 133829ns | +29015.6ns (+31.8%) | [+1083, +43625]ns | [94643, 154211] | YES | 0.0001 | 0.0000 | 0 |
| bitpack-contend-packed | 65475ns | -15618.1ns (-17.1%) | [-16933, -15459]ns | [65361, 65731] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed-simd | 71884ns | -19104.3ns (-21.0%) | [-20487, -16234]ns | [65106, 79110] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-d32 | bitpack-contend-d64 | bitpack-contend-packed | bitpack-contend-packed-simd |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 80695ns | +8.3% | -25.1% | +93.3% | -19.0% | -1.2% |
| 1 | 2 | 110687ns | -7.2% | -40.8% | +39.2% | -41.3% | -41.2% |
| 1 | 3 | 81273ns | +27.9% | -18.6% | +1.0% | -19.2% | -19.9% |
| 1 | 4 | 99296ns | +7.8% | -20.5% | -3.7% | -10.2% | -20.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | -0.778 | HIGH- (thermal bounce) |
| bitpack-contend-d16-control | 0.013 | ok |
| bitpack-contend-d32 | 0.011 | ok |
| bitpack-contend-d64 | 0.193 | ok |
| bitpack-contend-packed | -0.066 | ok |
| bitpack-contend-packed-simd | -0.250 | moderate- |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 1/4, lost 3/4
- **bitpack-contend-d32**: won 4/4, lost 0/4
- **bitpack-contend-d64**: won 1/4, lost 3/4
- **bitpack-contend-packed**: won 4/4, lost 0/4
- **bitpack-contend-packed-simd**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 20.7ns | 93105.9ns | 0.0% |  |
| bitpack-contend-d16-control | 20.5ns | 101764.1ns | 0.0% |  |
| bitpack-contend-d32 | 29.5ns | 67751.4ns | 0.0% |  |
| bitpack-contend-d64 | 39.0ns | 123771.7ns | 0.0% |  |
| bitpack-contend-packed | 20.9ns | 70538.2ns | 0.0% |  |
| bitpack-contend-packed-simd | 20.6ns | 72295.6ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 80614.3-110828.4 ns)
  80614.3 |########################################
  82125.0 |##
  83635.7 |
  85146.4 |
  86657.1 |
  88167.8 |
  89678.5 |
  91189.2 |
  92699.9 |
  94210.6 |
  95721.4 |
  97232.1 |
  98742.8 |#########################
  100253.5 |
  101764.2 |
  103274.9 |
  104785.6 |
  106296.3 |
  107807.0 |
  109317.7 |#################
  (3 below, 3 above range)

bitpack-contend-d16-control (n=40, range 85032.0-122693.1 ns)
  85032.0 |################
  86915.1 |############
  88798.1 |########
  90681.2 |
  92564.2 |
  94447.3 |
  96330.3 |
  98213.4 |####
  100096.4 |########
  101979.5 |########################################
  103862.5 |############
  105745.6 |############
  107628.7 |####
  109511.7 |
  111394.8 |
  113277.8 |
  115160.9 |
  117043.9 |
  118927.0 |
  120810.0 |
  (4 below, 7 above range)

bitpack-contend-d32 (n=40, range 60391.6-79156.1 ns)
  60391.6 |####################
  61329.8 |
  62268.0 |
  63206.2 |
  64144.5 |
  65082.7 |########################################
  66020.9 |#################
  66959.1 |
  67897.4 |
  68835.6 |
  69773.8 |
  70712.0 |
  71650.3 |
  72588.5 |
  73526.7 |
  74464.9 |
  75403.2 |
  76341.4 |
  77279.6 |
  78217.8 |#########################
  (3 below, 1 above range)

bitpack-contend-d64 (n=40, range 81954.3-156272.7 ns)
  81954.3 |################
  85670.2 |
  89386.1 |
  93102.0 |##############
  96818.0 |
  100533.9 |
  104249.8 |
  107965.7 |
  111681.6 |
  115397.6 |#########
  119113.5 |
  122829.4 |
  126545.3 |
  130261.2 |
  133977.2 |
  137693.1 |
  141409.0 |
  145124.9 |
  148840.8 |##
  152556.8 |########################################
  (3 below, 2 above range)

bitpack-contend-packed (n=40, range 64894.3-87730.1 ns)
  64894.3 |########################################
  66036.1 |#
  67177.9 |
  68319.7 |
  69461.4 |
  70603.2 |
  71745.0 |
  72886.8 |
  74028.6 |
  75170.4 |
  76312.2 |
  77454.0 |
  78595.8 |#
  79737.5 |###
  80879.3 |
  82021.1 |
  83162.9 |
  84304.7 |
  85446.5 |
  86588.3 |#
  (4 below, 6 above range)

bitpack-contend-packed-simd (n=40, range 64892.3-80299.7 ns)
  64892.3 |########################################
  65662.6 |
  66433.0 |
  67203.4 |
  67973.7 |
  68744.1 |
  69514.5 |
  70284.9 |
  71055.2 |
  71825.6 |
  72596.0 |
  73366.3 |
  74136.7 |
  74907.1 |
  75677.5 |
  76447.8 |
  77218.2 |
  77988.6 |#####
  78758.9 |#########################
  79529.3 |#################
  (4 below, 1 above range)

```

## Diagnostics

- **bitpack-contend-d64**: CV=26.3% (high variance, measurements may be unstable)
