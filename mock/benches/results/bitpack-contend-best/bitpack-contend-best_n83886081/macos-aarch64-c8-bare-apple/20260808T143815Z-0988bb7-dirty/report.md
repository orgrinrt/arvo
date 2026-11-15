# Both sides attacked: pairwise-accumulate dense carriers against the unrolled packed decode, at one and four threads

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-d16-padal dominates: 103% faster than the next best (bitpack-contend-pipe4)

bitpack-contend-d16-padal (315.68 us) leads bitpack-contend-pipe4 (640.00 us) by 103%, a clear separation rather than a photo finish. CV 6.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-contend-d16-padal beats baseline by 57% (significant)

bitpack-contend-d16-padal is -424.26 us (57%) faster than baseline bitpack-contend-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-contend-packed-simd is an outlier: 3.3x slower than the field

bitpack-contend-packed-simd (1.03 ms) is 3.3x the fastest (315.68 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-contend-pipe4 shows warm-up / thermal drift (autocorr +0.74)

bitpack-contend-pipe4's per-pass series has lag-1 autocorrelation +0.74, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-contend-d16-padal} vs {bitpack-contend-pipe4, bitpack-contend-d32-padal, bitpack-contend-d16, bitpack-contend-d32, bitpack-contend-packed-simd} (103% apart)

The field splits into a fast tier {bitpack-contend-d16-padal} and a slow tier {bitpack-contend-pipe4, bitpack-contend-d32-padal, bitpack-contend-d16, bitpack-contend-d32, bitpack-contend-packed-simd} with a 103% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.3x the fastest

Fastest bitpack-contend-d16-padal (315.68 us) to slowest bitpack-contend-packed-simd (1.03 ms): 3.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-contend-d16-padal** at 315681.0 ns median (-57.4% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 3.27x (fastest 315681.0 ns, slowest 1031976.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 748865ns | 741382ns | 729766ns | 742117ns | 788209ns | base |
| bitpack-contend-d16-padal | 316493ns | 317223ns | 289277ns | 314910ns | 348457ns | -57.74% |
| bitpack-contend-d32 | 860123ns | 809139ns | 743799ns | 814001ns | 1114815ns | +14.86% |
| bitpack-contend-d32-padal | 660322ns | 654524ns | 560612ns | 650017ns | 790949ns | -11.82% |
| bitpack-contend-packed-simd | 1058975ns | 1033269ns | 1026013ns | 1036571ns | 1159149ns | +41.41% |
| bitpack-contend-pipe4 | 653978ns | 640772ns | 614932ns | 642713ns | 726816ns | -12.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 747773ns | 728653ns | 787041ns | base | 11.218 |
| bitpack-contend-d16-padal | 315239ns | 288185ns | 347201ns | -57.84% | 26.610 |
| bitpack-contend-d32 | 858553ns | 742304ns | 1112110ns | +14.81% | 9.771 |
| bitpack-contend-d32-padal | 658782ns | 558496ns | 789548ns | -11.90% | 12.734 |
| bitpack-contend-packed-simd | 1057939ns | 1024891ns | 1157931ns | +41.48% | 7.929 |
| bitpack-contend-pipe4 | 653211ns | 614534ns | 725926ns | -12.65% | 12.842 |

## Performance model

- Peak throughput: **29.108 Gops/s** (bitpack-contend-d16-padal; best 20% batches)
- Ops per call: 8388608

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 11.329 | 38.9% |
| bitpack-contend-d16-padal | 26.573 | 91.3% |
| bitpack-contend-d32 | 10.379 | 35.7% |
| bitpack-contend-d32-padal | 12.844 | 44.1% |
| bitpack-contend-packed-simd | 8.129 | 27.9% |
| bitpack-contend-pipe4 | 13.107 | 45.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 748865ns | 748865ns | base |
| bitpack-contend-d16-padal | 316493ns | 316493ns | -57.74% |
| bitpack-contend-d32 | 860123ns | 860123ns | +14.86% |
| bitpack-contend-d32-padal | 660322ns | 660322ns | -11.82% |
| bitpack-contend-packed-simd | 1058975ns | 1058975ns | +41.41% |
| bitpack-contend-pipe4 | 653978ns | 653978ns | -12.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 740431ns | base | --- | [735913, 748435] | --- | --- | --- | --- |
| bitpack-contend-d16-padal | 315681ns | -425845.8ns (-57.5%) | [-436119, -421412]ns | [309031, 320095] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-d32 | 808191ns | +65326.5ns (+8.8%) | [+40671, +81718]ns | [786523, 832207] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-d32-padal | 653129ns | -101552.1ns (-13.7%) | [-142845, -73946]ns | [617579, 678447] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed-simd | 1031976ns | +296658.1ns (+40.1%) | [+287506, +301951]ns | [1029928, 1040967] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-pipe4 | 640001ns | -109440.6ns (-14.8%) | [-115551, -94660]ns | [624168, 654173] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-contend-d16 | bitpack-contend-d16-padal | bitpack-contend-d32 | bitpack-contend-d32-padal | bitpack-contend-packed-simd | bitpack-contend-pipe4 |
|---|---|---|---|---|---|---|
| 1 | 739733ns | -56.4% | +6.0% | -4.5% | +118.2% | -3.2% |
| 2 | 738990ns | -57.3% | +7.9% | +4.8% | +50.0% | -2.3% |
| 3 | 756405ns | -58.0% | +7.9% | -9.1% | +44.6% | -3.8% |
| 4 | 738881ns | -57.3% | +12.0% | +4.0% | +47.1% | -1.3% |
| 5 | 815990ns | -61.1% | +7.3% | -20.9% | +37.2% | -8.5% |
| 6 | 837607ns | -64.3% | +8.9% | -23.6% | +33.4% | -15.4% |
| 7 | 840231ns | -65.4% | +3.5% | -18.0% | +24.3% | -22.9% |
| 8 | 777385ns | -56.6% | +8.6% | -14.5% | +32.2% | -15.4% |
| 9 | 748703ns | -56.4% | +18.5% | -6.1% | +36.9% | -14.7% |
| 10 | 739942ns | -52.2% | +8.6% | -13.8% | +41.7% | -13.3% |
| 11 | 761553ns | -58.7% | -0.6% | -13.2% | +36.2% | -14.6% |
| 12 | 748715ns | -55.3% | +4.2% | -25.8% | +38.1% | -10.6% |
| 13 | 750055ns | -52.8% | +4.0% | -11.3% | +37.5% | -14.7% |
| 14 | 751285ns | -60.5% | +1.2% | -10.9% | +38.2% | -13.1% |
| 15 | 753069ns | -61.3% | +137.2% | -16.5% | +38.3% | -16.5% |
| 16 | 731273ns | -57.5% | +20.2% | +6.6% | +41.4% | -10.3% |
| 17 | 749035ns | -57.3% | +16.2% | +20.6% | +37.5% | -17.3% |
| 18 | 748167ns | -57.1% | +11.7% | +5.1% | +37.3% | -4.9% |
| 19 | 743263ns | -57.2% | +16.4% | +10.7% | +43.8% | -1.7% |
| 20 | 754086ns | -52.0% | +22.3% | +0.7% | +39.9% | -4.0% |
| 21 | 726936ns | -62.0% | +11.8% | -21.1% | +41.6% | -12.0% |
| 22 | 730475ns | -62.1% | +13.4% | -17.7% | +40.5% | -15.7% |
| 23 | 728903ns | -59.4% | +8.3% | -21.6% | +41.2% | -15.9% |
| 24 | 731191ns | -58.8% | +1.8% | -19.6% | +40.2% | -15.9% |
| 25 | 730785ns | -56.1% | +1.7% | -10.8% | +41.2% | -16.0% |
| 26 | 730910ns | -59.8% | +141.4% | -1.3% | +44.0% | -15.9% |
| 27 | 741865ns | -59.9% | +14.6% | -11.8% | +38.9% | -17.2% |
| 28 | 749242ns | -61.5% | +16.1% | -20.0% | +37.5% | -18.0% |
| 29 | 733289ns | -60.2% | +9.4% | -9.0% | +39.3% | -15.9% |
| 30 | 744490ns | -58.5% | +9.7% | -7.6% | +37.4% | -16.6% |
| 31 | 743098ns | -54.1% | -3.3% | -25.9% | +38.6% | -11.1% |
| 32 | 740921ns | -50.9% | +0.4% | -25.0% | +39.3% | -13.6% |
| 33 | 735254ns | -58.0% | +5.0% | -19.4% | +43.0% | -14.8% |
| 34 | 728302ns | -54.6% | +13.8% | -23.0% | +41.6% | -10.0% |
| 35 | 736572ns | -57.6% | +4.2% | -20.7% | +41.4% | -12.4% |
| 36 | 728532ns | -56.9% | +10.0% | -22.8% | +42.9% | -13.5% |
| 37 | 729585ns | -56.1% | +3.7% | -26.3% | +40.8% | -15.2% |
| 38 | 725709ns | -56.5% | +10.7% | -16.4% | +41.4% | -15.0% |
| 39 | 738878ns | -56.1% | +0.1% | -13.4% | +38.7% | -16.2% |
| 40 | 731618ns | -58.1% | +0.4% | -4.8% | +40.9% | -15.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | 0.718 | HIGH+ (drift/warm-up) |
| bitpack-contend-d16-padal | 0.225 | moderate+ |
| bitpack-contend-d32 | -0.027 | ok |
| bitpack-contend-d32-padal | 0.602 | HIGH+ (drift/warm-up) |
| bitpack-contend-packed-simd | 0.158 | ok |
| bitpack-contend-pipe4 | 0.742 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **bitpack-contend-d16-padal**: won 40/40, lost 0/40
- **bitpack-contend-d32**: won 2/40, lost 38/40
- **bitpack-contend-d32-padal**: won 33/40, lost 7/40
- **bitpack-contend-packed-simd**: won 0/40, lost 40/40
- **bitpack-contend-pipe4**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 8.6ns | 747773.1ns | 0.0% |  |
| bitpack-contend-d16-padal | 4.4ns | 315238.6ns | 0.0% |  |
| bitpack-contend-d32 | 20.5ns | 858552.5ns | 0.0% |  |
| bitpack-contend-d32-padal | 11.0ns | 658781.8ns | 0.0% |  |
| bitpack-contend-packed-simd | 22.1ns | 1057938.8ns | 0.0% |  |
| bitpack-contend-pipe4 | 10.7ns | 653211.4ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 728653.4-787040.5 ns)
  728653.4 |########################################
  731572.8 |###########
  734492.1 |###########
  737411.5 |############################
  740330.9 |#################
  743250.2 |###########
  746169.6 |######################
  749088.9 |#################
  752008.3 |###########
  754927.6 |#####
  757847.0 |
  760766.3 |#####
  763685.7 |
  766605.0 |
  769524.4 |
  772443.8 |
  775363.1 |#####
  778282.5 |
  781201.8 |
  784121.2 |
  (4 below, 3 above range)

bitpack-contend-d16-padal (n=40, range 288185.0-347201.1 ns)
  288185.0 |################
  291135.8 |########################
  294086.6 |########
  297037.4 |########################
  299988.2 |########
  302939.0 |
  305889.8 |################
  308840.6 |################
  311791.4 |########################
  314742.2 |########################################
  317693.0 |########################
  320643.8 |########################
  323594.6 |################
  326545.5 |
  329496.3 |########
  332447.1 |########
  335397.9 |########
  338348.7 |
  341299.5 |########
  344250.3 |
  (2 below, 4 above range)

bitpack-contend-d32 (n=40, range 742304.5-1112109.6 ns)
  742304.5 |########################################
  760794.7 |#############
  779285.0 |#################################
  797775.2 |########################################
  816265.5 |##########################
  834755.8 |####################
  853246.0 |##########################
  871736.3 |####################
  890226.5 |
  908716.8 |#############
  927207.1 |
  945697.3 |
  964187.6 |
  982677.8 |
  1001168.1 |
  1019658.3 |
  1038148.6 |
  1056638.9 |
  1075129.1 |
  1093619.4 |
  (3 below, 2 above range)

bitpack-contend-d32-padal (n=40, range 558495.8-789547.8 ns)
  558495.8 |####################
  570048.4 |####################
  581601.0 |##############################
  593153.6 |####################
  604706.2 |##########
  616258.8 |
  627811.4 |####################
  639364.0 |##############################
  650916.6 |##############################
  662469.2 |########################################
  674021.8 |
  685574.4 |########################################
  697127.0 |####################
  708679.6 |
  720232.2 |##########
  731784.8 |
  743337.4 |
  754890.0 |##########
  766442.6 |####################
  777995.2 |####################
  (4 below, 2 above range)

bitpack-contend-packed-simd (n=40, range 1024891.5-1157931.0 ns)
  1024891.5 |########################################
  1031543.4 |###############
  1038195.4 |##########
  1044847.4 |##
  1051499.4 |#######
  1058151.3 |
  1064803.3 |##
  1071455.3 |
  1078107.3 |
  1084759.3 |##
  1091411.2 |##
  1098063.2 |
  1104715.2 |##
  1111367.2 |##
  1118019.2 |##
  1124671.1 |
  1131323.1 |
  1137975.1 |
  1144627.1 |
  1151279.1 |
  (3 below, 1 above range)

bitpack-contend-pipe4 (n=40, range 614534.3-725926.4 ns)
  614534.3 |########################################
  620103.9 |##########
  625673.5 |###############
  631243.1 |
  636812.7 |#########################
  642382.3 |##########
  647951.9 |##########
  653521.6 |###############
  659091.2 |#####
  664660.8 |#####
  670230.4 |
  675800.0 |
  681369.6 |
  686939.2 |
  692508.8 |
  698078.4 |
  703648.0 |#####
  709217.6 |#####
  714787.2 |#####
  720356.8 |##########
  (4 below, 4 above range)

```

## Diagnostics

- **bitpack-contend-d16**: autocorrelation=0.72 (measurement drift or warm-up artifact)
- **bitpack-contend-d32**: CV=25.2% (high variance, measurements may be unstable)
- **bitpack-contend-d32-padal**: autocorrelation=0.60 (measurement drift or warm-up artifact)
- **bitpack-contend-pipe4**: autocorrelation=0.74 (measurement drift or warm-up artifact)
