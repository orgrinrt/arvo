# Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-d16 shows warm-up / thermal drift (autocorr +0.90)

bitpack-contend-d16's per-pass series has lag-1 autocorrelation +0.90, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-contend-d32, bitpack-contend-d16-control, bitpack-contend-d16} vs {bitpack-contend-d64, bitpack-contend-packed-simd, bitpack-contend-packed} (29% apart)

The field splits into a fast tier {bitpack-contend-d32, bitpack-contend-d16-control, bitpack-contend-d16} and a slow tier {bitpack-contend-d64, bitpack-contend-packed-simd, bitpack-contend-packed} with a 29% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader bitpack-contend-d32 vs stability leader bitpack-contend-d16 (+5% speed for 2.5x steadier)

bitpack-contend-d32 is fastest (27.68 us, CV 17.2%); bitpack-contend-d16 gives up 5.5% median for 2.5x lower variance (CV 6.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### bitpack-contend-d64 is inconsistent: worst-20% is 1.6x its best-20%

bitpack-contend-d64's best 20% of batches run at 35.11 us but its worst 20% at 56.58 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-contend-d32** at 27682.3 ns median (-5.2% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1.59x (fastest 27682.3 ns, slowest 43975.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 27949ns | 29349ns | 24864ns | 28380ns | 29740ns | base |
| bitpack-contend-d16-control | 28250ns | 28099ns | 24104ns | 28156ns | 32678ns | +1.08% |
| bitpack-contend-d32 | 28667ns | 27837ns | 23898ns | 27811ns | 36006ns | +2.57% |
| bitpack-contend-d64 | 42027ns | 37906ns | 35281ns | 39275ns | 57030ns | +50.37% |
| bitpack-contend-packed | 42171ns | 44144ns | 35475ns | 43089ns | 46115ns | +50.89% |
| bitpack-contend-packed-simd | 38213ns | 38298ns | 33308ns | 38248ns | 43015ns | +36.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 27818ns | 24751ns | 29586ns | base | 37.694 |
| bitpack-contend-d16-control | 28122ns | 23998ns | 32528ns | +1.09% | 37.286 |
| bitpack-contend-d32 | 28511ns | 23732ns | 35784ns | +2.49% | 36.777 |
| bitpack-contend-d64 | 41757ns | 35107ns | 56580ns | +50.11% | 25.111 |
| bitpack-contend-packed | 42004ns | 35355ns | 45903ns | +50.99% | 24.964 |
| bitpack-contend-packed-simd | 38087ns | 33198ns | 42870ns | +36.91% | 27.531 |

## Performance model

- Peak throughput: **44.185 Gops/s** (bitpack-contend-d32; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 35.912 | 81.3% |
| bitpack-contend-d16-control | 37.465 | 84.8% |
| bitpack-contend-d32 | 37.879 | 85.7% |
| bitpack-contend-d64 | 27.875 | 63.1% |
| bitpack-contend-packed | 23.844 | 54.0% |
| bitpack-contend-packed-simd | 27.463 | 62.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 27949ns | 27949ns | base |
| bitpack-contend-d16-control | 28250ns | 28250ns | +1.08% |
| bitpack-contend-d32 | 28667ns | 28667ns | +2.57% |
| bitpack-contend-d64 | 42027ns | 42027ns | +50.37% |
| bitpack-contend-packed | 42171ns | 42171ns | +50.89% |
| bitpack-contend-packed-simd | 38213ns | 38213ns | +36.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 29198ns | base | --- | [27182, 29456] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 27988ns | -150.2ns (-0.5%) | [-919, -2]ns | [27196, 28885] | YES | 0.0481 | 0.0385 | 0 |
| bitpack-contend-d32 | 27682ns | -370.4ns (-1.3%) | [-426, -5]ns | [26752, 29058] | YES (adj: no) | 0.0807 | 0.0807 | 0 |
| bitpack-contend-d64 | 37618ns | +11040.6ns (+37.8%) | [+9651, +13119]ns | [36735, 40930] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed | 43976ns | +14527.2ns (+49.8%) | [+14351, +14647]ns | [41607, 44030] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed-simd | 38181ns | +9424.6ns (+32.3%) | [+8642, +11088]ns | [38137, 38803] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-d32 | bitpack-contend-d64 | bitpack-contend-packed | bitpack-contend-packed-simd |
|---|---|---|---|---|---|---|
| 1 | 23698ns | -0.7% | -0.8% | +47.6% | +48.4% | +40.2% |
| 2 | 23548ns | +0.1% | +0.4% | +51.7% | +49.2% | +42.1% |
| 3 | 23805ns | -1.0% | -2.2% | +48.0% | +47.5% | +39.2% |
| 4 | 25469ns | -7.6% | -8.6% | +38.0% | +38.0% | +31.5% |
| 5 | 25510ns | -7.7% | -7.1% | +38.5% | +37.9% | +30.7% |
| 6 | 25362ns | -7.2% | -7.4% | +38.9% | +38.9% | +30.7% |
| 7 | 25496ns | +21.2% | -6.0% | +37.7% | +38.3% | +30.1% |
| 8 | 25402ns | +0.9% | -0.9% | +41.2% | +44.2% | +30.6% |
| 9 | 25362ns | +0.1% | +0.4% | +45.2% | +47.4% | +31.3% |
| 10 | 25359ns | -0.2% | -0.9% | +45.3% | +126.9% | +30.6% |
| 11 | 25762ns | +5.8% | +3.9% | +124.6% | +58.1% | +48.1% |
| 12 | 27188ns | +0.3% | -1.4% | +114.5% | +49.4% | +44.4% |
| 13 | 27221ns | +4.8% | -1.8% | +116.0% | +52.2% | +51.6% |
| 14 | 27119ns | +8.8% | +3.7% | +114.7% | +61.8% | +41.0% |
| 15 | 27176ns | +91.6% | +6.8% | +113.3% | +49.1% | +40.7% |
| 16 | 27176ns | +9.6% | +0.3% | +113.3% | +60.5% | +47.6% |
| 17 | 27218ns | +6.0% | +66.3% | +52.3% | +53.5% | +52.6% |
| 18 | 27242ns | -0.2% | -1.9% | +85.2% | +55.5% | +51.4% |
| 19 | 29122ns | -2.9% | -1.4% | +48.3% | +51.2% | +31.2% |
| 20 | 29275ns | +0.7% | -0.3% | +38.0% | +50.3% | +39.3% |
| 21 | 28417ns | -4.3% | -12.0% | +38.6% | +55.5% | +34.3% |
| 22 | 29488ns | -4.2% | -10.9% | +48.8% | +49.3% | +29.4% |
| 23 | 29573ns | -0.3% | -9.2% | +56.1% | +49.8% | +35.5% |
| 24 | 29431ns | -5.5% | -9.1% | +23.0% | +50.6% | +40.7% |
| 25 | 29469ns | -8.0% | -9.2% | +49.1% | +49.8% | +40.5% |
| 26 | 29482ns | -0.9% | -1.9% | +79.8% | +49.1% | +40.5% |
| 27 | 29460ns | +0.0% | -1.4% | +27.5% | +50.7% | +30.2% |
| 28 | 29448ns | -0.0% | -1.4% | +27.4% | +49.8% | +36.7% |
| 29 | 29517ns | -3.4% | -1.5% | +27.5% | +49.5% | +41.0% |
| 30 | 29439ns | -7.1% | -1.3% | +27.7% | +50.0% | +40.9% |
| 31 | 29452ns | -7.8% | +9.4% | +15.8% | +39.9% | +18.9% |
| 32 | 29600ns | -8.3% | +7.6% | +48.2% | +48.6% | +23.9% |
| 33 | 29556ns | -5.7% | -1.1% | +24.1% | +49.4% | +29.0% |
| 34 | 29645ns | -0.5% | +1.2% | +24.1% | +48.5% | +28.7% |
| 35 | 29582ns | -0.5% | +15.4% | +23.2% | +49.1% | +29.1% |
| 36 | 29492ns | -0.0% | +42.0% | +23.9% | +49.2% | +29.3% |
| 37 | 29543ns | -0.1% | +36.2% | +27.5% | +48.8% | +78.8% |
| 38 | 29437ns | -1.7% | -1.2% | +32.9% | +49.7% | +29.5% |
| 39 | 29647ns | -8.4% | +3.6% | +23.0% | +48.4% | +28.9% |
| 40 | 29540ns | -4.8% | -1.4% | +31.2% | +49.5% | +29.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | 0.903 | HIGH+ (drift/warm-up) |
| bitpack-contend-d16-control | 0.243 | moderate+ |
| bitpack-contend-d32 | 0.472 | moderate+ |
| bitpack-contend-d64 | 0.690 | HIGH+ (drift/warm-up) |
| bitpack-contend-packed | 0.452 | moderate+ |
| bitpack-contend-packed-simd | 0.485 | moderate+ |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 24/40, lost 11/40
- **bitpack-contend-d32**: won 26/40, lost 14/40
- **bitpack-contend-d64**: won 0/40, lost 40/40
- **bitpack-contend-packed**: won 0/40, lost 40/40
- **bitpack-contend-packed-simd**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 3.2ns | 27818.2ns | 0.0% |  |
| bitpack-contend-d16-control | 3.1ns | 28122.5ns | 0.0% |  |
| bitpack-contend-d32 | 3.8ns | 28511.5ns | 0.0% |  |
| bitpack-contend-d64 | 6.9ns | 41757.0ns | 0.0% |  |
| bitpack-contend-packed | 3.0ns | 42003.6ns | 0.0% |  |
| bitpack-contend-packed-simd | 3.1ns | 38087.1ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 24750.5-29585.7 ns)
  24750.5 |
  24992.3 |
  25234.1 |############
  25475.8 |#####
  25717.6 |##
  25959.3 |
  26201.1 |
  26442.9 |
  26684.6 |
  26926.4 |##
  27168.1 |###############
  27409.9 |
  27651.7 |
  27893.4 |
  28135.2 |
  28376.9 |##
  28618.7 |
  28860.4 |
  29102.2 |#####
  29344.0 |########################################
  (3 below, 3 above range)

bitpack-contend-d16-control (n=40, range 23997.7-32527.6 ns)
  23997.7 |
  24424.2 |
  24850.7 |
  25277.2 |############
  25703.7 |
  26130.2 |
  26556.7 |
  26983.2 |####################################
  27409.7 |####
  27836.2 |############
  28262.7 |############
  28689.1 |########
  29115.6 |########################################
  29542.1 |####
  29968.6 |
  30395.1 |
  30821.6 |####
  31248.1 |
  31674.6 |
  32101.1 |
  (6 below, 1 above range)

bitpack-contend-d32 (n=40, range 23731.6-35783.6 ns)
  23731.6 |####
  24334.2 |
  24936.8 |#################
  25539.4 |
  26142.0 |#############
  26744.6 |##########################
  27347.2 |
  27949.8 |####
  28552.4 |########################################
  29155.0 |########
  29757.6 |####
  30360.2 |####
  30962.8 |
  31565.4 |####
  32168.0 |####
  32770.6 |
  33373.2 |
  33975.8 |####
  34578.4 |
  35181.0 |
  (6 below, 3 above range)

bitpack-contend-d64 (n=40, range 35107.4-56580.2 ns)
  35107.4 |###################################
  36181.0 |########################################
  37254.7 |#########################
  38328.3 |###############
  39402.0 |#####
  40475.6 |#####
  41549.2 |
  42622.9 |#####
  43696.5 |###############
  44770.1 |
  45843.8 |#####
  46917.4 |
  47991.1 |
  49064.7 |
  50138.3 |#####
  51212.0 |
  52285.6 |#####
  53359.2 |
  54432.9 |
  55506.5 |
  (2 below, 6 above range)

bitpack-contend-packed (n=40, range 35354.9-45903.0 ns)
  35354.9 |
  35882.3 |
  36409.7 |##
  36937.1 |##
  37464.5 |
  37991.9 |
  38519.3 |
  39046.7 |
  39574.1 |
  40101.5 |####
  40628.9 |##
  41156.4 |####
  41683.8 |##
  42211.2 |##
  42738.6 |
  43266.0 |##
  43793.4 |########################################
  44320.8 |####
  44848.2 |
  45375.6 |
  (7 below, 1 above range)

bitpack-contend-packed-simd (n=40, range 33197.7-42870.1 ns)
  33197.7 |##############
  33681.3 |
  34164.9 |
  34648.5 |##
  35132.1 |
  35615.8 |
  36099.4 |
  36583.0 |##
  37066.6 |
  37550.2 |
  38033.9 |########################################
  38517.5 |
  39001.1 |##
  39484.7 |
  39968.3 |########
  40452.0 |##
  40935.6 |###########
  41419.2 |###########
  41902.8 |
  42386.4 |
  (5 below, 1 above range)

```

## Diagnostics

- **bitpack-contend-d16**: autocorrelation=0.90 (measurement drift or warm-up artifact)
- **bitpack-contend-d64**: autocorrelation=0.69 (measurement drift or warm-up artifact)
