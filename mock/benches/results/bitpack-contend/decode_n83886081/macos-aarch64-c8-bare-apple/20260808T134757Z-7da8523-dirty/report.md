# The packed decode with one, two and four accumulators, against the u16 carrier, at one and four threads

5 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-pipe4 is fastest but the noisiest (CV 23.5%)

bitpack-contend-pipe4 wins on median (655.79 us) yet has the highest variance (CV 23.5%), while bitpack-contend-d16-control is the steadiest (CV 5.2%, 756.38 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bitpack-contend-d16-control shows warm-up / thermal drift (autocorr +0.80)

bitpack-contend-d16-control's per-pass series has lag-1 autocorrelation +0.80, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-contend-pipe4, bitpack-contend-pipe2, bitpack-contend-d16, bitpack-contend-d16-control} vs {bitpack-contend-packed-simd} (37% apart)

The field splits into a fast tier {bitpack-contend-pipe4, bitpack-contend-pipe2, bitpack-contend-d16, bitpack-contend-d16-control} and a slow tier {bitpack-contend-packed-simd} with a 37% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-contend-pipe4** at 655793.1 ns median (-13.0% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.58x (fastest 655793.1 ns, slowest 1033746.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 775025ns | 754527ns | 731878ns | 764229ns | 850561ns | base |
| bitpack-contend-d16-control | 770833ns | 757436ns | 734102ns | 760982ns | 837115ns | -0.54% |
| bitpack-contend-packed-simd | 1064937ns | 1034789ns | 1025727ns | 1043798ns | 1167565ns | +37.41% |
| bitpack-contend-pipe2 | 671602ns | 666183ns | 623088ns | 667924ns | 731149ns | -13.34% |
| bitpack-contend-pipe4 | 691939ns | 656608ns | 615494ns | 662859ns | 855624ns | -10.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 773963ns | 730725ns | 849634ns | base | 10.839 |
| bitpack-contend-d16-control | 769770ns | 732981ns | 835990ns | -0.54% | 10.898 |
| bitpack-contend-packed-simd | 1063873ns | 1024625ns | 1166458ns | +37.46% | 7.885 |
| bitpack-contend-pipe2 | 670666ns | 621902ns | 730414ns | -13.35% | 12.508 |
| bitpack-contend-pipe4 | 690990ns | 614623ns | 854335ns | -10.72% | 12.140 |

## Performance model

- Peak throughput: **13.648 Gops/s** (bitpack-contend-pipe4; best 20% batches)
- Ops per call: 8388608

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 11.135 | 81.6% |
| bitpack-contend-d16-control | 11.091 | 81.3% |
| bitpack-contend-packed-simd | 8.115 | 59.5% |
| bitpack-contend-pipe2 | 12.607 | 92.4% |
| bitpack-contend-pipe4 | 12.792 | 93.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 775025ns | 775025ns | base |
| bitpack-contend-d16-control | 770833ns | 770833ns | -0.54% |
| bitpack-contend-packed-simd | 1064937ns | 1064937ns | +37.41% |
| bitpack-contend-pipe2 | 671602ns | 671602ns | -13.34% |
| bitpack-contend-pipe4 | 691939ns | 691939ns | -10.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 753373ns | base | --- | [748266, 782696] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 756377ns | no significant difference | [-16747, +15942]ns | [743292, 767981] | no | 0.4296 | 0.4296 | 0 |
| bitpack-contend-packed-simd | 1033746ns | +293475.8ns (+39.0%) | [+274568, +297751]ns | [1030006, 1044769] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-pipe2 | 665415ns | -105115.6ns (-14.0%) | [-119178, -82946]ns | [656566, 681246] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-pipe4 | 655793ns | -113324.1ns (-15.0%) | [-120656, -100851]ns | [628949, 696983] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-packed-simd | bitpack-contend-pipe2 | bitpack-contend-pipe4 |
|---|---|---|---|---|---|
| 1 | 833975ns | -3.6% | +59.3% | -20.9% | -19.7% |
| 2 | 873233ns | -4.3% | +30.3% | -23.9% | -23.3% |
| 3 | 777496ns | +10.7% | +46.9% | -14.2% | -14.5% |
| 4 | 813964ns | +5.4% | +37.1% | -18.0% | -13.9% |
| 5 | 806920ns | +6.7% | +50.0% | -18.6% | -7.9% |
| 6 | 790125ns | +4.3% | +39.6% | -13.8% | -4.3% |
| 7 | 873314ns | -4.0% | +35.2% | -22.4% | -18.7% |
| 8 | 756643ns | +5.8% | +43.2% | -1.9% | -14.8% |
| 9 | 748073ns | -1.5% | +43.7% | +3.2% | -1.9% |
| 10 | 796781ns | -5.3% | +29.5% | -8.1% | -14.9% |
| 11 | 755582ns | -1.0% | +35.8% | -17.1% | -16.6% |
| 12 | 753836ns | -1.9% | +36.6% | -15.7% | -15.7% |
| 13 | 746488ns | +3.0% | +39.8% | -11.3% | -13.3% |
| 14 | 752910ns | -1.5% | +38.2% | -4.5% | +112.8% |
| 15 | 782193ns | -3.1% | +32.3% | -12.3% | -7.2% |
| 16 | 783198ns | +1.9% | +31.5% | -11.2% | -9.8% |
| 17 | 840362ns | -4.1% | +26.4% | -12.5% | -12.2% |
| 18 | 873799ns | -12.2% | +27.0% | -18.6% | -14.2% |
| 19 | 853175ns | -13.3% | +29.7% | -17.5% | -10.3% |
| 20 | 835252ns | -9.5% | +23.0% | -16.8% | -10.6% |
| 21 | 798950ns | -8.5% | +28.4% | -22.8% | -21.9% |
| 22 | 751173ns | -2.6% | +36.3% | -17.9% | -17.5% |
| 23 | 742866ns | -1.6% | +40.8% | -14.1% | -16.5% |
| 24 | 752470ns | -2.3% | +36.8% | -18.3% | -17.7% |
| 25 | 748460ns | -1.4% | +38.2% | -12.4% | -15.7% |
| 26 | 732468ns | +0.5% | +41.2% | -7.0% | -12.5% |
| 27 | 751536ns | -1.0% | +42.5% | -16.6% | -17.5% |
| 28 | 785607ns | -4.8% | +30.6% | -10.2% | -22.9% |
| 29 | 751382ns | +4.2% | +35.7% | -3.2% | -18.2% |
| 30 | 735670ns | +4.2% | +40.2% | -10.8% | -4.3% |
| 31 | 725569ns | +2.4% | +43.2% | -8.9% | -6.7% |
| 32 | 761662ns | -2.1% | +34.3% | -10.3% | -8.9% |
| 33 | 728700ns | +5.1% | +40.9% | -8.6% | -0.8% |
| 34 | 728844ns | +8.3% | +41.0% | -7.8% | -15.4% |
| 35 | 729218ns | +8.5% | +40.7% | -11.1% | -16.2% |
| 36 | 732225ns | +0.8% | +40.4% | -14.8% | -15.9% |
| 37 | 745016ns | -2.7% | +38.4% | -16.1% | -17.2% |
| 38 | 739914ns | -0.4% | +39.6% | -15.5% | -16.6% |
| 39 | 733102ns | +3.2% | +40.6% | -13.4% | -14.8% |
| 40 | 736373ns | +3.0% | +39.9% | -10.6% | -14.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | 0.625 | HIGH+ (drift/warm-up) |
| bitpack-contend-d16-control | 0.805 | HIGH+ (drift/warm-up) |
| bitpack-contend-packed-simd | 0.485 | moderate+ |
| bitpack-contend-pipe2 | 0.552 | HIGH+ (drift/warm-up) |
| bitpack-contend-pipe4 | 0.081 | ok |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 23/40, lost 17/40
- **bitpack-contend-packed-simd**: won 0/40, lost 40/40
- **bitpack-contend-pipe2**: won 39/40, lost 1/40
- **bitpack-contend-pipe4**: won 39/40, lost 1/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 9.7ns | 773963.1ns | 0.0% |  |
| bitpack-contend-d16-control | 8.5ns | 769770.5ns | 0.0% |  |
| bitpack-contend-packed-simd | 13.4ns | 1063872.7ns | 0.0% |  |
| bitpack-contend-pipe2 | 8.0ns | 670665.6ns | 0.0% |  |
| bitpack-contend-pipe4 | 12.3ns | 690990.2ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 730724.5-849634.2 ns)
  730724.5 |#################################
  736670.0 |######
  742615.5 |#################################
  748561.0 |########################################
  754506.5 |#############
  760451.9 |######
  766397.4 |
  772342.9 |######
  778288.4 |#############
  784233.9 |#############
  790179.4 |
  796124.8 |#############
  802070.3 |######
  808015.8 |
  813961.3 |######
  819906.8 |
  825852.2 |
  831797.7 |#############
  837743.2 |######
  843688.7 |
  (4 below, 4 above range)

bitpack-contend-d16-control (n=40, range 732980.7-835990.3 ns)
  732980.7 |##########################
  738131.2 |########################################
  743281.7 |##########################
  748432.2 |
  753582.6 |#################################
  758733.1 |
  763883.6 |##########################
  769034.1 |
  774184.6 |
  779335.0 |######
  784485.5 |######
  789636.0 |######
  794786.5 |######
  799937.0 |#############
  805087.4 |######
  810237.9 |
  815388.4 |
  820538.9 |######
  825689.4 |
  830839.8 |######
  (4 below, 4 above range)

bitpack-contend-packed-simd (n=40, range 1024624.7-1166458.0 ns)
  1024624.7 |########################################
  1031716.3 |#############
  1038808.0 |########
  1045899.7 |##
  1052991.3 |
  1060083.0 |##
  1067174.7 |##
  1074266.3 |##
  1081358.0 |##
  1088449.7 |
  1095541.4 |
  1102633.0 |#####
  1109724.7 |#####
  1116816.4 |
  1123908.0 |
  1130999.7 |##
  1138091.4 |##
  1145183.0 |
  1152274.7 |
  1159366.4 |
  (3 below, 3 above range)

bitpack-contend-pipe2 (n=40, range 621901.8-730414.0 ns)
  621901.8 |########################################
  627327.4 |
  632753.0 |########################
  638178.7 |
  643604.3 |########
  649029.9 |
  654455.5 |########################################
  659881.1 |########################
  665306.7 |########################
  670732.3 |########
  676157.9 |########################
  681583.5 |################
  687009.1 |
  692434.7 |################
  697860.4 |
  703286.0 |################
  708711.6 |########
  714137.2 |########
  719562.8 |
  724988.4 |########
  (3 below, 4 above range)

bitpack-contend-pipe4 (n=40, range 614622.8-854335.3 ns)
  614622.8 |########################################
  626608.4 |################
  638594.1 |############
  650579.7 |
  662565.3 |############
  674550.9 |########
  686536.6 |####
  698522.2 |################
  710507.8 |
  722493.4 |############
  734479.1 |########
  746464.7 |############
  758450.3 |####
  770435.9 |
  782421.6 |
  794407.2 |
  806392.8 |
  818378.4 |
  830364.1 |
  842349.7 |
  (3 below, 1 above range)

```

## Diagnostics

- **bitpack-contend-d16**: autocorrelation=0.63 (measurement drift or warm-up artifact)
- **bitpack-contend-d16-control**: autocorrelation=0.80 (measurement drift or warm-up artifact)
- **bitpack-contend-pipe2**: autocorrelation=0.55 (measurement drift or warm-up artifact)
- **bitpack-contend-pipe4**: CV=22.3% (high variance, measurements may be unstable)
