# Packed 13-bit against u16, u32 and u64 dense carriers, swept from L1 to past a 12 MB L2

6 variants, 40 samples per variant.
Baseline: **bitpack-carrier-d16**

## Highlights

Baseline for all deltas below: **bitpack-carrier-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-carrier-d64 shows warm-up / thermal drift (autocorr +0.79)

bitpack-carrier-d64's per-pass series has lag-1 autocorrelation +0.79, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-carrier-d32, bitpack-carrier-d16, bitpack-carrier-d16-control} vs {bitpack-carrier-d64, bitpack-carrier-packed-simd, bitpack-carrier-packed} (31% apart)

The field splits into a fast tier {bitpack-carrier-d32, bitpack-carrier-d16, bitpack-carrier-d16-control} and a slow tier {bitpack-carrier-d64, bitpack-carrier-packed-simd, bitpack-carrier-packed} with a 31% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-carrier-d32** at 87922.1 ns median (-1.1% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1.51x (fastest 87922.1 ns, slowest 133074.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 89311ns | 89160ns | 88961ns | 89159ns | 90116ns | base |
| bitpack-carrier-d16-control | 89772ns | 89200ns | 88929ns | 89436ns | 91622ns | +0.52% |
| bitpack-carrier-d32 | 88490ns | 88151ns | 87883ns | 88149ns | 90118ns | -0.92% |
| bitpack-carrier-d64 | 118960ns | 116766ns | 109687ns | 117512ns | 132578ns | +33.20% |
| bitpack-carrier-packed | 133839ns | 133280ns | 133141ns | 133301ns | 136150ns | +49.86% |
| bitpack-carrier-packed-simd | 126527ns | 125825ns | 125298ns | 126211ns | 128706ns | +41.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-carrier-d16 | 89118ns | 88798ns | 89907ns | base | 11.766 |
| bitpack-carrier-d16-control | 89588ns | 88788ns | 91405ns | +0.53% | 11.704 |
| bitpack-carrier-d32 | 88290ns | 87728ns | 89922ns | -0.93% | 11.876 |
| bitpack-carrier-d64 | 118684ns | 109458ns | 132306ns | +33.18% | 8.835 |
| bitpack-carrier-packed | 133622ns | 132970ns | 135881ns | +49.94% | 7.847 |
| bitpack-carrier-packed-simd | 126334ns | 125142ns | 128564ns | +41.76% | 8.300 |

## Performance model

- Peak throughput: **11.953 Gops/s** (bitpack-carrier-d32; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-carrier-d16 | 11.790 | 98.6% |
| bitpack-carrier-d16-control | 11.782 | 98.6% |
| bitpack-carrier-d32 | 11.926 | 99.8% |
| bitpack-carrier-d64 | 9.003 | 75.3% |
| bitpack-carrier-packed | 7.880 | 65.9% |
| bitpack-carrier-packed-simd | 8.352 | 69.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-carrier-d16 | 89311ns | 89311ns | base |
| bitpack-carrier-d16-control | 89772ns | 89772ns | +0.52% |
| bitpack-carrier-d32 | 88490ns | 88490ns | -0.92% |
| bitpack-carrier-d64 | 118960ns | 118960ns | +33.20% |
| bitpack-carrier-packed | 133839ns | 133839ns | +49.86% |
| bitpack-carrier-packed-simd | 126527ns | 126527ns | +41.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 88937ns | base | --- | [88912, 89025] | --- | --- | --- | --- |
| bitpack-carrier-d16-control | 88995ns | no significant difference | [-1, +329]ns | [88899, 89645] | no | 0.0807 | 0.0807 | 0 |
| bitpack-carrier-d32 | 87922ns | -1076.0ns (-1.2%) | [-1160, -926]ns | [87834, 88002] | YES | 0.0002 | 0.0002 | 0 |
| bitpack-carrier-d64 | 116472ns | +27366.9ns (+30.8%) | [+22700, +31048]ns | [111723, 119970] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-packed | 133074ns | +44127.9ns (+49.6%) | [+44018, +44267]ns | [133033, 133131] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-packed-simd | 125551ns | +36527.7ns (+41.1%) | [+36410, +37299]ns | [125345, 126300] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-carrier-d16 | bitpack-carrier-d16-control | bitpack-carrier-d32 | bitpack-carrier-d64 | bitpack-carrier-packed | bitpack-carrier-packed-simd |
|---|---|---|---|---|---|---|
| 1 | 89640ns | -0.9% | -1.9% | +23.6% | +48.6% | +39.7% |
| 2 | 89168ns | -0.2% | -1.6% | +30.4% | +49.2% | +41.6% |
| 3 | 89270ns | -0.3% | -1.2% | +24.8% | +48.9% | +40.8% |
| 4 | 89082ns | -0.3% | -1.1% | +24.0% | +49.3% | +41.8% |
| 5 | 88787ns | -0.0% | +0.8% | +25.6% | +49.9% | +41.1% |
| 6 | 88820ns | +0.0% | -1.0% | +23.4% | +49.8% | +41.0% |
| 7 | 88789ns | +0.0% | -0.9% | +31.1% | +49.9% | +41.0% |
| 8 | 88925ns | -0.2% | -1.1% | +24.7% | +49.9% | +41.3% |
| 9 | 88958ns | -0.2% | -1.1% | +28.7% | +53.0% | +42.0% |
| 10 | 88955ns | +1.1% | -1.3% | +32.6% | +69.3% | +41.0% |
| 11 | 88817ns | -0.0% | +1.1% | +28.8% | +50.0% | +42.8% |
| 12 | 89014ns | +0.0% | -1.1% | +28.4% | +49.5% | +41.0% |
| 13 | 90196ns | +0.1% | -2.5% | +29.8% | +47.5% | +39.0% |
| 14 | 88944ns | -0.1% | +0.6% | +31.0% | +49.9% | +41.0% |
| 15 | 88943ns | +1.2% | -1.3% | +34.4% | +49.6% | +41.1% |
| 16 | 88931ns | +1.6% | -1.3% | +40.3% | +49.7% | +40.8% |
| 17 | 89037ns | +0.5% | -1.4% | +34.9% | +49.4% | +41.0% |
| 18 | 88808ns | +0.4% | -1.1% | +35.0% | +49.7% | +41.1% |
| 19 | 88769ns | +1.8% | -1.0% | +39.0% | +49.8% | +40.9% |
| 20 | 89038ns | +1.0% | -1.4% | +41.6% | +49.4% | +40.5% |
| 21 | 88813ns | +0.3% | +0.4% | +52.5% | +50.1% | +44.5% |
| 22 | 89049ns | -0.1% | +0.5% | +49.7% | +49.4% | +44.6% |
| 23 | 88905ns | -0.1% | -1.0% | +48.5% | +49.9% | +44.3% |
| 24 | 88852ns | +0.0% | -1.3% | +39.9% | +49.7% | +44.5% |
| 25 | 88931ns | -0.1% | -1.2% | +48.4% | +49.8% | +44.4% |
| 26 | 88903ns | +0.0% | -1.4% | +46.1% | +50.1% | +44.2% |
| 27 | 88876ns | +0.0% | -0.9% | +47.9% | +49.6% | +44.2% |
| 28 | 88922ns | +3.0% | +2.8% | +48.3% | +49.5% | +44.2% |
| 29 | 88965ns | +0.0% | +1.8% | +47.5% | +49.5% | +44.2% |
| 30 | 88778ns | +0.2% | +1.4% | +47.7% | +49.8% | +44.4% |
| 31 | 88920ns | +2.4% | -1.2% | +23.3% | +50.3% | +42.0% |
| 32 | 88919ns | +4.6% | -1.3% | +23.2% | +49.6% | +41.0% |
| 33 | 90818ns | +0.6% | -3.4% | +22.3% | +46.5% | +37.8% |
| 34 | 90704ns | +0.8% | -3.1% | +32.0% | +46.6% | +38.1% |
| 35 | 89260ns | +2.3% | -1.5% | +25.4% | +49.0% | +40.2% |
| 36 | 89158ns | +2.0% | -1.3% | +22.7% | +49.3% | +40.5% |
| 37 | 88849ns | +0.2% | -1.3% | +23.7% | +49.8% | +40.9% |
| 38 | 88831ns | -0.1% | -1.0% | +22.8% | +49.9% | +40.9% |
| 39 | 89708ns | +0.1% | -2.1% | +21.3% | +48.5% | +40.5% |
| 40 | 89664ns | -0.9% | -2.2% | +22.5% | +48.3% | +44.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-carrier-d16 | 0.403 | moderate+ |
| bitpack-carrier-d16-control | 0.509 | HIGH+ (drift/warm-up) |
| bitpack-carrier-d32 | 0.372 | moderate+ |
| bitpack-carrier-d64 | 0.790 | HIGH+ (drift/warm-up) |
| bitpack-carrier-packed | 0.144 | ok |
| bitpack-carrier-packed-simd | 0.663 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **bitpack-carrier-d16-control**: won 10/40, lost 18/40
- **bitpack-carrier-d32**: won 32/40, lost 8/40
- **bitpack-carrier-d64**: won 0/40, lost 40/40
- **bitpack-carrier-packed**: won 0/40, lost 40/40
- **bitpack-carrier-packed-simd**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-carrier-d16 | 2.8ns | 89117.8ns | 0.0% |  |
| bitpack-carrier-d16-control | 3.1ns | 89588.2ns | 0.0% |  |
| bitpack-carrier-d32 | 3.1ns | 88290.1ns | 0.0% |  |
| bitpack-carrier-d64 | 3.4ns | 118683.8ns | 0.0% |  |
| bitpack-carrier-packed | 2.8ns | 133622.4ns | 0.0% |  |
| bitpack-carrier-packed-simd | 2.8ns | 126333.6ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-carrier-d16 (n=40, range 88797.5-89907.3 ns)
  88797.5 |############################
  88853.0 |############
  88908.5 |########################################
  88964.0 |########
  89019.5 |############
  89075.0 |####
  89130.5 |########
  89185.9 |
  89241.4 |########
  89296.9 |
  89352.4 |
  89407.9 |
  89463.4 |
  89518.9 |
  89574.4 |
  89629.9 |########
  89685.4 |####
  89740.9 |
  89796.4 |
  89851.8 |
  (4 below, 3 above range)

bitpack-carrier-d16-control (n=40, range 88787.7-91404.7 ns)
  88787.7 |########################################
  88918.5 |#############################
  89049.4 |#######
  89180.2 |
  89311.1 |
  89441.9 |###
  89572.8 |
  89703.6 |###
  89834.5 |#######
  89965.3 |###
  90096.2 |
  90227.0 |###
  90357.9 |#######
  90488.7 |
  90619.6 |
  90750.4 |
  90881.3 |###
  91012.2 |###
  91143.0 |
  91273.9 |#######
  (4 below, 3 above range)

bitpack-carrier-d32 (n=40, range 87727.7-89922.4 ns)
  87727.7 |########################################
  87837.4 |###################################
  87947.2 |###############################
  88056.9 |#############
  88166.6 |
  88276.4 |
  88386.1 |
  88495.8 |
  88605.6 |
  88715.3 |
  88825.0 |
  88934.8 |
  89044.5 |
  89154.3 |####
  89264.0 |
  89373.7 |########
  89483.5 |####
  89593.2 |
  89702.9 |
  89812.7 |####
  (5 below, 3 above range)

bitpack-carrier-d64 (n=40, range 109458.4-132306.2 ns)
  109458.4 |########################################
  110600.8 |#################################
  111743.2 |######
  112885.6 |
  114028.0 |####################
  115170.4 |
  116312.8 |##########################
  117455.2 |######
  118597.6 |######
  119739.9 |####################
  120882.3 |
  122024.7 |
  123167.1 |#############
  124309.5 |######
  125451.9 |######
  126594.3 |
  127736.7 |
  128879.1 |######
  130021.5 |######
  131163.9 |#################################
  (3 below, 2 above range)

bitpack-carrier-packed (n=40, range 132970.3-135881.2 ns)
  132970.3 |########################################
  133115.8 |##############
  133261.4 |#######
  133406.9 |#
  133552.5 |#
  133698.0 |
  133843.6 |
  133989.1 |
  134134.6 |
  134280.2 |
  134425.7 |
  134571.3 |
  134716.8 |
  134862.4 |
  135007.9 |
  135153.5 |
  135299.0 |
  135444.6 |
  135590.1 |
  135735.7 |
  (2 below, 2 above range)

bitpack-carrier-packed-simd (n=40, range 125141.6-128563.7 ns)
  125141.6 |########################################
  125312.8 |######################
  125483.8 |#################
  125654.9 |####
  125826.1 |
  125997.1 |####
  126168.2 |#############
  126339.4 |####
  126510.4 |
  126681.6 |####
  126852.6 |
  127023.8 |
  127194.9 |
  127365.9 |
  127537.1 |
  127708.2 |
  127879.2 |
  128050.4 |#################
  128221.5 |#################
  128392.6 |####
  (4 below, 2 above range)

```

## Diagnostics

- **bitpack-carrier-d16-control**: autocorrelation=0.51 (measurement drift or warm-up artifact)
- **bitpack-carrier-d64**: autocorrelation=0.79 (measurement drift or warm-up artifact)
- **bitpack-carrier-packed-simd**: autocorrelation=0.66 (measurement drift or warm-up artifact)
