# Packed 13-bit against u16, u32 and u64 dense carriers, swept from L1 to past a 12 MB L2

6 variants, 40 samples per variant.
Baseline: **bitpack-carrier-d16**

## Highlights

Baseline for all deltas below: **bitpack-carrier-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-carrier-packed-simd beats baseline by 21% (significant)

bitpack-carrier-packed-simd is -69.25 us (21%) faster than baseline bitpack-carrier-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Top two (bitpack-carrier-packed-simd, bitpack-carrier-packed) are a dead heat (<1%)

bitpack-carrier-packed-simd (257.31 us) and bitpack-carrier-packed (257.78 us) differ by 0.18%, inside the noise, even though the wider field spreads 97.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### bitpack-carrier-d32 shows alternating (throttle bounce) (autocorr -0.75)

bitpack-carrier-d32's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-carrier-packed-simd, bitpack-carrier-packed, bitpack-carrier-d32, bitpack-carrier-d16, bitpack-carrier-d16-control} vs {bitpack-carrier-d64} (55% apart)

The field splits into a fast tier {bitpack-carrier-packed-simd, bitpack-carrier-packed, bitpack-carrier-d32, bitpack-carrier-d16, bitpack-carrier-d16-control} and a slow tier {bitpack-carrier-d64} with a 55% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### bitpack-carrier-d32 is inconsistent: worst-20% is 1.7x its best-20%

bitpack-carrier-d32's best 20% of batches run at 246.99 us but its worst 20% at 414.20 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-carrier-packed-simd** at 257309.8 ns median (-21.3% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.97x (fastest 257309.8 ns, slowest 506961.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Packed 13-bit against u16, u32 and u64 dense carriers, swept from L1 to past a 12 MB L2 |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112721357 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 324349ns | 326916ns | 318957ns | 324762ns | 328499ns | base |
| bitpack-carrier-d16-control | 328347ns | 327371ns | 327067ns | 327405ns | 332455ns | +1.23% |
| bitpack-carrier-d32 | 322147ns | 314439ns | 247263ns | 316236ns | 414765ns | -0.68% |
| bitpack-carrier-d64 | 532075ns | 507497ns | 487220ns | 512916ns | 634408ns | +64.04% |
| bitpack-carrier-packed | 260865ns | 258051ns | 257605ns | 260332ns | 265726ns | -19.57% |
| bitpack-carrier-packed-simd | 258046ns | 257497ns | 257092ns | 257498ns | 260644ns | -20.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-carrier-d16 | 324163ns | 318811ns | 328225ns | base | 6.469 |
| bitpack-carrier-d16-control | 328150ns | 326885ns | 332229ns | +1.23% | 6.391 |
| bitpack-carrier-d32 | 321788ns | 246995ns | 414202ns | -0.73% | 6.517 |
| bitpack-carrier-d64 | 531432ns | 486658ns | 633709ns | +63.94% | 3.946 |
| bitpack-carrier-packed | 260675ns | 257434ns | 265547ns | -19.59% | 8.045 |
| bitpack-carrier-packed-simd | 257858ns | 256900ns | 260473ns | -20.45% | 8.133 |

## Performance model

- Peak throughput: **8.491 Gops/s** (bitpack-carrier-d32; best 20% batches)
- Ops per call: 2097152

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-carrier-d16 | 6.418 | 75.6% |
| bitpack-carrier-d16-control | 6.409 | 75.5% |
| bitpack-carrier-d32 | 6.675 | 78.6% |
| bitpack-carrier-d64 | 4.137 | 48.7% |
| bitpack-carrier-packed | 8.136 | 95.8% |
| bitpack-carrier-packed-simd | 8.150 | 96.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-carrier-d16 | 324349ns | 324349ns | base |
| bitpack-carrier-d16-control | 328347ns | 328347ns | +1.23% |
| bitpack-carrier-d32 | 322147ns | 322147ns | -0.68% |
| bitpack-carrier-d64 | 532075ns | 532075ns | +64.04% |
| bitpack-carrier-packed | 260865ns | 260865ns | -19.57% |
| bitpack-carrier-packed-simd | 258046ns | 258046ns | -20.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 326771ns | base | --- | [319303, 327334] | --- | --- | --- | --- |
| bitpack-carrier-d16-control | 327207ns | +1194.0ns (+0.4%) | [+110, +8048]ns | [327051, 327333] | YES | 0.0080 | 0.0064 | 0 |
| bitpack-carrier-d32 | 314192ns | no significant difference | [-64500, +46282]ns | [262900, 366374] | no | 1.0000 | 1.0000 | 0 |
| bitpack-carrier-d64 | 506962ns | +180438.1ns (+55.2%) | [+175222, +200515]ns | [496318, 523192] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-packed | 257775ns | -61989.5ns (-19.0%) | [-67646, -61591]ns | [257660, 265170] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-packed-simd | 257310ns | -69246.9ns (-21.2%) | [-69696, -62131]ns | [257220, 257359] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-carrier-d16 | bitpack-carrier-d16-control | bitpack-carrier-d32 | bitpack-carrier-d64 | bitpack-carrier-packed | bitpack-carrier-packed-simd |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 326757ns | +0.2% | -24.4% | +51.3% | -21.2% | -21.3% |
| 1 | 2 | 327341ns | +0.1% | +18.3% | +78.8% | -21.3% | -21.4% |
| 1 | 3 | 327133ns | -0.0% | -19.6% | +60.0% | -19.4% | -21.3% |
| 1 | 4 | 323075ns | +1.2% | +13.3% | +50.6% | -17.8% | -20.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-carrier-d16 | -0.080 | ok |
| bitpack-carrier-d16-control | -0.043 | ok |
| bitpack-carrier-d32 | -0.749 | HIGH- (thermal bounce) |
| bitpack-carrier-d64 | -0.289 | moderate- |
| bitpack-carrier-packed | 0.290 | moderate+ |
| bitpack-carrier-packed-simd | 0.243 | moderate+ |

**Consistency summary:**

- **bitpack-carrier-d16-control**: won 0/4, lost 3/4
- **bitpack-carrier-d32**: won 2/4, lost 2/4
- **bitpack-carrier-d64**: won 0/4, lost 4/4
- **bitpack-carrier-packed**: won 4/4, lost 0/4
- **bitpack-carrier-packed-simd**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-carrier-d16 | 41.1ns | 324162.9ns | 0.0% |  |
| bitpack-carrier-d16-control | 42.6ns | 328150.4ns | 0.0% |  |
| bitpack-carrier-d32 | 97.5ns | 321787.5ns | 0.0% |  |
| bitpack-carrier-d64 | 141.5ns | 531431.9ns | 0.0% |  |
| bitpack-carrier-packed | 39.5ns | 260675.3ns | 0.0% |  |
| bitpack-carrier-packed-simd | 44.4ns | 257857.6ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-carrier-d16 (n=40, range 318810.5-328224.7 ns)
  318810.5 |##############################
  319281.2 |###
  319751.9 |
  320222.6 |
  320693.4 |
  321164.1 |
  321634.8 |
  322105.5 |
  322576.2 |
  323046.9 |
  323517.6 |
  323988.3 |
  324459.0 |###
  324929.7 |
  325400.5 |
  325871.2 |###
  326341.9 |################
  326812.6 |######
  327283.3 |########################################
  327754.0 |######
  (5 below, 2 above range)

bitpack-carrier-d16-control (n=40, range 326885.2-332229.2 ns)
  326885.2 |########################################
  327152.4 |############################
  327419.6 |########
  327686.8 |#####
  327954.0 |##
  328221.2 |
  328488.4 |
  328755.6 |
  329022.8 |
  329290.0 |
  329557.2 |
  329824.4 |
  330091.6 |
  330358.8 |
  330626.0 |
  330893.2 |
  331160.4 |
  331427.6 |
  331694.8 |
  331962.0 |
  (5 below, 5 above range)

bitpack-carrier-d32 (n=40, range 246994.7-414201.9 ns)
  246994.7 |########################
  255355.1 |########################################
  263715.4 |
  272075.8 |
  280436.1 |
  288796.5 |
  297156.9 |
  305517.2 |
  313877.6 |
  322237.9 |
  330598.3 |
  338958.7 |
  347319.0 |
  355679.4 |
  364039.7 |################################
  372400.1 |
  380760.5 |################
  389120.8 |####################
  397481.2 |
  405841.5 |####
  (4 below, 2 above range)

bitpack-carrier-d64 (n=40, range 486657.8-633709.4 ns)
  486657.8 |################
  494010.4 |########################################
  501363.0 |########
  508715.5 |####
  516068.1 |####################
  523420.7 |############
  530773.3 |####
  538125.9 |
  545478.5 |
  552831.0 |
  560183.6 |
  567536.2 |
  574888.8 |
  582241.4 |################
  589593.9 |
  596946.5 |####
  604299.1 |
  611651.7 |
  619004.3 |
  626356.9 |
  (5 below, 4 above range)

bitpack-carrier-packed (n=40, range 257434.4-265546.8 ns)
  257434.4 |########################################
  257840.0 |
  258245.6 |##
  258651.2 |
  259056.9 |
  259462.5 |
  259868.1 |
  260273.7 |
  260679.3 |
  261085.0 |
  261490.6 |
  261896.2 |
  262301.8 |##
  262707.5 |
  263113.1 |
  263518.7 |
  263924.3 |
  264329.9 |
  264735.6 |##
  265141.2 |######################
  (3 below, 3 above range)

bitpack-carrier-packed-simd (n=40, range 256899.7-260473.2 ns)
  256899.7 |##################
  257078.4 |################################
  257257.1 |########################################
  257435.7 |#####################
  257614.4 |#######
  257793.1 |
  257971.8 |
  258150.4 |
  258329.1 |###
  258507.8 |
  258686.5 |
  258865.1 |
  259043.8 |
  259222.5 |
  259401.2 |
  259579.8 |
  259758.5 |
  259937.2 |
  260115.9 |
  260294.5 |
  (3 below, 3 above range)

```

## Diagnostics

- **bitpack-carrier-d32**: CV=21.8% (high variance, measurements may be unstable)
