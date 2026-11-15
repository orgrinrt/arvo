# Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-packed shows warm-up / thermal drift (autocorr +0.89)

bitpack-contend-packed's per-pass series has lag-1 autocorrelation +0.89, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-contend-d64, bitpack-contend-d32, bitpack-contend-d16-control, bitpack-contend-d16} vs {bitpack-contend-packed-simd, bitpack-contend-packed} (29% apart)

The field splits into a fast tier {bitpack-contend-d64, bitpack-contend-d32, bitpack-contend-d16-control, bitpack-contend-d16} and a slow tier {bitpack-contend-packed-simd, bitpack-contend-packed} with a 29% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader bitpack-contend-d64 vs stability leader bitpack-contend-d32 (+5% speed for 1.0x steadier)

bitpack-contend-d64 is fastest (519 ns, CV 5.2%); bitpack-contend-d32 gives up 4.8% median for 1.0x lower variance (CV 5.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: bitpack-contend-d64** at 519.2 ns median (-8.2% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.44x (fastest 519.2 ns, slowest 748.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 632ns | 644ns | 577ns | 639ns | 666ns | base |
| bitpack-contend-d16-control | 638ns | 622ns | 577ns | 629ns | 723ns | +0.94% |
| bitpack-contend-d32 | 623ns | 613ns | 588ns | 618ns | 672ns | -1.43% |
| bitpack-contend-d64 | 600ns | 593ns | 563ns | 596ns | 647ns | -5.08% |
| bitpack-contend-packed | 826ns | 822ns | 764ns | 824ns | 891ns | +30.68% |
| bitpack-contend-packed-simd | 801ns | 800ns | 740ns | 801ns | 861ns | +26.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 557ns | 509ns | 588ns | base | 29.418 |
| bitpack-contend-d16-control | 561ns | 508ns | 637ns | +0.81% | 29.182 |
| bitpack-contend-d32 | 549ns | 520ns | 589ns | -1.44% | 29.849 |
| bitpack-contend-d64 | 523ns | 488ns | 558ns | -6.13% | 31.338 |
| bitpack-contend-packed | 751ns | 693ns | 810ns | +34.83% | 21.818 |
| bitpack-contend-packed-simd | 728ns | 672ns | 783ns | +30.79% | 22.492 |

## Performance model

- Peak throughput: **33.575 Gops/s** (bitpack-contend-d64; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 28.967 | 86.3% |
| bitpack-contend-d16-control | 29.843 | 88.9% |
| bitpack-contend-d32 | 30.120 | 89.7% |
| bitpack-contend-d64 | 31.556 | 94.0% |
| bitpack-contend-packed | 21.901 | 65.2% |
| bitpack-contend-packed-simd | 22.444 | 66.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 632ns | 632ns | base |
| bitpack-contend-d16-control | 638ns | 638ns | +0.94% |
| bitpack-contend-d32 | 623ns | 623ns | -1.43% |
| bitpack-contend-d64 | 600ns | 600ns | -5.08% |
| bitpack-contend-packed | 826ns | 826ns | +30.68% |
| bitpack-contend-packed-simd | 801ns | 801ns | +26.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 566ns | base | --- | [546, 584] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 549ns | no significant difference | [-33, +36]ns | [545, 555] | no | 1.0000 | 0.8746 | 0 |
| bitpack-contend-d32 | 544ns | no significant difference | [-60, +29]ns | [524, 565] | no | 1.0000 | 1.0000 | 0 |
| bitpack-contend-d64 | 519ns | -36.2ns (-6.4%) | [-44, -29]ns | [502, 543] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed | 748ns | +200.0ns (+35.4%) | [+164, +239]ns | [746, 752] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed-simd | 730ns | +162.9ns (+28.8%) | [+94, +234]ns | [679, 779] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-d32 | bitpack-contend-d64 | bitpack-contend-packed | bitpack-contend-packed-simd |
|---|---|---|---|---|---|---|
| 1 | 592ns | +9.3% | -10.8% | -7.9% | +28.4% | +13.9% |
| 2 | 584ns | +8.3% | -10.4% | -9.6% | +28.5% | +17.4% |
| 3 | 585ns | +8.7% | -10.7% | -8.2% | +27.5% | +14.4% |
| 4 | 582ns | +8.5% | -9.4% | -6.7% | +28.4% | +15.7% |
| 5 | 583ns | +9.1% | -9.7% | -6.5% | +28.1% | +16.7% |
| 6 | 585ns | +8.5% | -12.3% | -7.3% | +28.6% | +16.0% |
| 7 | 586ns | +8.5% | -11.3% | -9.0% | +27.2% | +14.8% |
| 8 | 585ns | +8.5% | -10.9% | -7.3% | +27.4% | +15.9% |
| 9 | 588ns | +7.9% | -11.1% | -10.9% | +28.0% | +14.7% |
| 10 | 587ns | +8.2% | -11.4% | -8.2% | +28.0% | +15.5% |
| 11 | 554ns | -6.0% | +5.5% | -10.0% | +46.7% | +40.5% |
| 12 | 550ns | +1.7% | +6.4% | -8.9% | +47.1% | +41.1% |
| 13 | 548ns | -7.2% | +3.3% | -8.3% | +48.0% | +42.7% |
| 14 | 547ns | -6.4% | +3.3% | -8.2% | +48.7% | +42.4% |
| 15 | 549ns | -7.6% | +4.4% | -8.7% | +46.4% | +43.3% |
| 16 | 544ns | -6.3% | +3.2% | -7.7% | +48.6% | +43.8% |
| 17 | 545ns | -6.9% | +5.3% | -9.1% | +48.4% | +42.5% |
| 18 | 546ns | -6.8% | +5.3% | -9.0% | +47.9% | +43.1% |
| 19 | 545ns | -6.9% | +4.5% | -7.9% | +48.2% | +43.0% |
| 20 | 550ns | -7.8% | +5.7% | -8.1% | +47.4% | +42.0% |
| 21 | 516ns | +6.5% | +17.4% | -1.0% | +45.0% | +50.2% |
| 22 | 509ns | +7.0% | +18.0% | +0.2% | +46.8% | +53.3% |
| 23 | 507ns | +7.6% | +19.1% | +1.5% | +47.7% | +54.8% |
| 24 | 508ns | +7.8% | +10.8% | +0.3% | +46.6% | +53.3% |
| 25 | 510ns | +7.8% | +9.9% | -2.5% | +47.0% | +53.1% |
| 26 | 507ns | +6.5% | +11.7% | -4.1% | +47.0% | +53.9% |
| 27 | 513ns | +6.5% | +10.1% | -6.1% | +46.4% | +52.5% |
| 28 | 510ns | +3.0% | +11.0% | -5.8% | +48.9% | +52.8% |
| 29 | 508ns | +7.1% | +11.3% | -5.6% | +47.6% | +54.7% |
| 30 | 511ns | +4.5% | +10.4% | -4.7% | +45.8% | +52.3% |
| 31 | 578ns | -4.1% | -9.2% | -3.1% | +22.2% | +15.9% |
| 32 | 584ns | -6.4% | -10.5% | -4.6% | +20.0% | +16.1% |
| 33 | 583ns | -6.6% | -10.1% | -4.1% | +19.1% | +16.9% |
| 34 | 585ns | -5.5% | -10.2% | -4.6% | +17.2% | +16.0% |
| 35 | 588ns | -6.1% | -11.1% | -5.9% | +19.9% | +15.5% |
| 36 | 582ns | -5.7% | -10.2% | -4.9% | +19.2% | +15.8% |
| 37 | 585ns | -4.0% | -10.8% | -4.6% | +18.1% | +16.5% |
| 38 | 589ns | -5.3% | -11.4% | -5.2% | +17.9% | +15.0% |
| 39 | 586ns | -5.0% | -10.5% | -5.3% | +18.4% | +14.4% |
| 40 | 584ns | -5.8% | -9.7% | -4.6% | +18.7% | +15.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | 0.885 | HIGH+ (drift/warm-up) |
| bitpack-contend-d16-control | 0.831 | HIGH+ (drift/warm-up) |
| bitpack-contend-d32 | 0.833 | HIGH+ (drift/warm-up) |
| bitpack-contend-d64 | 0.820 | HIGH+ (drift/warm-up) |
| bitpack-contend-packed | 0.894 | HIGH+ (drift/warm-up) |
| bitpack-contend-packed-simd | 0.866 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 19/40, lost 21/40
- **bitpack-contend-d32**: won 20/40, lost 20/40
- **bitpack-contend-d64**: won 37/40, lost 3/40
- **bitpack-contend-packed**: won 0/40, lost 40/40
- **bitpack-contend-packed-simd**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 2.5ns | 556.9ns | 0.5% |  |
| bitpack-contend-d16-control | 2.6ns | 561.4ns | 0.5% |  |
| bitpack-contend-d32 | 2.1ns | 548.9ns | 0.4% |  |
| bitpack-contend-d64 | 2.7ns | 522.8ns | 0.5% |  |
| bitpack-contend-packed | 2.3ns | 750.9ns | 0.3% |  |
| bitpack-contend-packed-simd | 2.7ns | 728.4ns | 0.4% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 508.5-587.7 ns)
    508.5 |##############
    512.5 |#######
    516.5 |
    520.4 |
    524.4 |
    528.3 |
    532.3 |
    536.3 |
    540.2 |
    544.2 |#####################
    548.1 |##########
    552.1 |###
    556.0 |
    560.0 |
    564.0 |
    567.9 |
    571.9 |
    575.8 |###
    579.8 |##############
    583.7 |########################################
  (4 below, 4 above range)

bitpack-contend-d16-control (n=40, range 508.5-637.0 ns)
    508.5 |#############
    514.9 |####
    521.3 |####
    527.8 |
    534.2 |########
    540.6 |##########################
    547.0 |###############################
    553.5 |#################
    559.9 |####
    566.3 |
    572.7 |
    579.2 |
    585.6 |
    592.0 |
    598.4 |
    604.8 |
    611.3 |
    617.7 |
    624.1 |
    630.5 |########################################
  (5 below, 1 above range)

bitpack-contend-d32 (n=40, range 520.4-588.7 ns)
    520.4 |########################################
    523.8 |##########################
    527.3 |########
    530.7 |
    534.1 |
    537.5 |
    540.9 |
    544.3 |
    547.7 |
    551.1 |
    554.6 |
    558.0 |####
    561.4 |######################
    564.8 |#################
    568.2 |####
    571.6 |########
    575.0 |####
    578.4 |####
    581.9 |####
    585.3 |####
  (3 below, 3 above range)

bitpack-contend-d64 (n=40, range 488.0-557.8 ns)
    488.0 |
    491.5 |
    495.0 |########################################
    498.5 |##############################
    501.9 |########################################
    505.4 |##########
    508.9 |####################
    512.4 |##########
    515.9 |
    519.4 |
    522.9 |##########
    526.4 |##########
    529.9 |##########
    533.4 |
    536.9 |####################
    540.4 |##############################
    543.9 |####################
    547.3 |
    550.8 |####################
    554.3 |########################################
  (5 below, 4 above range)

bitpack-contend-packed (n=40, range 693.3-810.3 ns)
    693.3 |##############
    699.2 |#######
    705.0 |###
    710.9 |
    716.7 |
    722.6 |
    728.4 |
    734.3 |
    740.1 |##################
    746.0 |########################################
    751.8 |#######
    757.7 |#######
    763.5 |
    769.4 |
    775.2 |
    781.1 |
    786.9 |
    792.8 |
    798.6 |###
    804.5 |#####################
  (3 below, 3 above range)

bitpack-contend-packed-simd (n=40, range 672.3-783.1 ns)
    672.3 |######################
    677.9 |######################
    683.4 |##
    688.9 |
    694.5 |
    700.0 |
    705.6 |
    711.1 |
    716.6 |
    722.2 |
    727.7 |
    733.3 |
    738.8 |
    744.3 |
    749.9 |
    755.4 |
    761.0 |
    766.5 |
    772.1 |########
    777.6 |########################################
  (3 below, 3 above range)

```

## Diagnostics

- **bitpack-contend-d16**: autocorrelation=0.89 (measurement drift or warm-up artifact)
- **bitpack-contend-d16-control**: autocorrelation=0.83 (measurement drift or warm-up artifact)
- **bitpack-contend-d32**: autocorrelation=0.83 (measurement drift or warm-up artifact)
- **bitpack-contend-d64**: autocorrelation=0.82 (measurement drift or warm-up artifact)
- **bitpack-contend-packed**: autocorrelation=0.89 (measurement drift or warm-up artifact)
- **bitpack-contend-packed-simd**: autocorrelation=0.87 (measurement drift or warm-up artifact)
