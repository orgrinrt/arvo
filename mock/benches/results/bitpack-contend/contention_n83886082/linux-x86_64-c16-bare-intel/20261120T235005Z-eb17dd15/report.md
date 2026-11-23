# Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-packed beats baseline by 21% (significant)

bitpack-contend-packed is -183.12 us (21%) faster than baseline bitpack-contend-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-contend-d64 is an outlier: 2.7x slower than the field

bitpack-contend-d64 (1.90 ms) is 2.7x the fastest (704.61 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-contend-d64 shows alternating (throttle bounce) (autocorr -0.52)

bitpack-contend-d64's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-contend-packed, bitpack-contend-packed-simd, bitpack-contend-d16, bitpack-contend-d16-control, bitpack-contend-d32} vs {bitpack-contend-d64} (79% apart)

The field splits into a fast tier {bitpack-contend-packed, bitpack-contend-packed-simd, bitpack-contend-d16, bitpack-contend-d16-control, bitpack-contend-d32} and a slow tier {bitpack-contend-d64} with a 79% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-contend-packed** at 704608.9 ns median (-20.2% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.69x (fastest 704608.9 ns, slowest 1896430.8 ns)

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
| Counter frequency | 2112542308 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 869693ns | 883898ns | 741463ns | 878938ns | 970187ns | base |
| bitpack-contend-d16-control | 897998ns | 891315ns | 841368ns | 898995ns | 951638ns | +3.25% |
| bitpack-contend-d32 | 1105026ns | 1063391ns | 1025824ns | 1089145ns | 1231871ns | +27.06% |
| bitpack-contend-d64 | 1925376ns | 1898313ns | 1885815ns | 1905832ns | 2023568ns | +121.39% |
| bitpack-contend-packed | 726720ns | 705074ns | 693434ns | 712684ns | 802114ns | -16.44% |
| bitpack-contend-packed-simd | 749257ns | 762904ns | 700556ns | 746606ns | 805911ns | -13.85% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 868892ns | 740660ns | 969386ns | base | 9.654 |
| bitpack-contend-d16-control | 897196ns | 840637ns | 950802ns | +3.26% | 9.350 |
| bitpack-contend-d32 | 1103631ns | 1024489ns | 1230564ns | +27.02% | 7.601 |
| bitpack-contend-d64 | 1923702ns | 1884398ns | 2021984ns | +121.40% | 4.361 |
| bitpack-contend-packed | 726197ns | 692966ns | 801475ns | -16.42% | 11.551 |
| bitpack-contend-packed-simd | 748818ns | 700151ns | 805451ns | -13.82% | 11.202 |

## Performance model

- Peak throughput: **12.105 Gops/s** (bitpack-contend-packed; best 20% batches)
- Ops per call: 8388608

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 9.498 | 78.5% |
| bitpack-contend-d16-control | 9.420 | 77.8% |
| bitpack-contend-d32 | 7.898 | 65.2% |
| bitpack-contend-d64 | 4.423 | 36.5% |
| bitpack-contend-packed | 11.905 | 98.3% |
| bitpack-contend-packed-simd | 11.006 | 90.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 869693ns | 869693ns | base |
| bitpack-contend-d16-control | 897998ns | 897998ns | +3.25% |
| bitpack-contend-d32 | 1105026ns | 1105026ns | +27.06% |
| bitpack-contend-d64 | 1925376ns | 1925376ns | +121.39% |
| bitpack-contend-packed | 726720ns | 726720ns | -16.44% |
| bitpack-contend-packed-simd | 749257ns | 749257ns | -13.85% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 883202ns | base | --- | [868587, 919487] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 890483ns | no significant difference | [-39393, +29392]ns | [885750, 914091] | no | 0.4296 | 0.4296 | 0 |
| bitpack-contend-d32 | 1062056ns | +182499.8ns (+20.7%) | [+147421, +238860]ns | [1058082, 1073301] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-d64 | 1896431ns | +1039143.1ns (+117.7%) | [+1008456, +1097366]ns | [1894612, 1916428] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed | 704609ns | -183122.0ns (-20.7%) | [-209790, -72317]ns | [698438, 711668] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed-simd | 762218ns | -156266.3ns (-17.7%) | [-169221, -125609]ns | [707556, 777446] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-d32 | bitpack-contend-d64 | bitpack-contend-packed | bitpack-contend-packed-simd |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 885530ns | +0.5% | +15.7% | +114.0% | -21.1% | -20.2% |
| 1 | 2 | 827126ns | +6.9% | +27.7% | +129.1% | -3.2% | -15.2% |
| 1 | 3 | 756414ns | +22.5% | +61.1% | +153.6% | -8.4% | +5.5% |
| 1 | 4 | 928288ns | -4.4% | +15.3% | +103.3% | -23.5% | -16.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | -0.364 | moderate- |
| bitpack-contend-d16-control | -0.480 | moderate- |
| bitpack-contend-d32 | -0.219 | moderate- |
| bitpack-contend-d64 | -0.524 | HIGH- (thermal bounce) |
| bitpack-contend-packed | -0.516 | HIGH- (thermal bounce) |
| bitpack-contend-packed-simd | 0.140 | ok |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 1/4, lost 3/4
- **bitpack-contend-d32**: won 0/4, lost 4/4
- **bitpack-contend-d64**: won 0/4, lost 4/4
- **bitpack-contend-packed**: won 4/4, lost 0/4
- **bitpack-contend-packed-simd**: won 3/4, lost 1/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 91.9ns | 868891.8ns | 0.0% |  |
| bitpack-contend-d16-control | 104.0ns | 897196.4ns | 0.0% |  |
| bitpack-contend-d32 | 215.6ns | 1103631.3ns | 0.0% |  |
| bitpack-contend-d64 | 225.1ns | 1923702.2ns | 0.0% |  |
| bitpack-contend-packed | 57.9ns | 726197.3ns | 0.0% |  |
| bitpack-contend-packed-simd | 64.4ns | 748818.1ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 740660.1-969386.3 ns)
  740660.1 |##########
  752096.4 |###############
  763532.8 |
  774969.1 |
  786405.4 |#####
  797841.7 |
  809278.0 |
  820714.3 |
  832150.6 |
  843586.9 |
  855023.2 |#####
  866459.5 |####################
  877895.8 |########################################
  889332.1 |#####
  900768.4 |
  912204.7 |###############
  923641.0 |##############################
  935077.4 |
  946513.7 |
  957950.0 |##########
  (5 below, 4 above range)

bitpack-contend-d16-control (n=40, range 840637.2-950801.8 ns)
  840637.2 |
  846145.4 |
  851653.6 |
  857161.9 |
  862670.1 |
  868178.3 |
  873686.6 |##########
  879194.8 |########################################
  884703.0 |#########################
  890211.3 |####################
  895719.5 |#####
  901227.7 |
  906736.0 |##########
  912244.2 |
  917752.4 |###############
  923260.7 |##############################
  928768.9 |##########
  934277.1 |
  939785.4 |
  945293.6 |#####
  (4 below, 2 above range)

bitpack-contend-d32 (n=40, range 1024489.4-1230563.6 ns)
  1024489.4 |#################
  1034793.1 |
  1045096.8 |#################
  1055400.5 |########################################
  1065704.2 |###############################
  1076007.9 |
  1086311.6 |
  1096615.4 |
  1106919.1 |
  1117222.8 |
  1127526.5 |
  1137830.2 |
  1148133.9 |
  1158437.6 |
  1168741.3 |
  1179045.1 |
  1189348.8 |
  1199652.5 |####
  1209956.2 |###############################
  1220259.9 |####
  (4 below, 3 above range)

bitpack-contend-d64 (n=40, range 1884398.2-2021984.4 ns)
  1884398.2 |################################
  1891277.5 |########################################
  1898156.8 |####
  1905036.1 |########
  1911915.4 |####################
  1918794.8 |########
  1925674.1 |####
  1932553.4 |####
  1939432.7 |
  1946312.0 |
  1953191.3 |
  1960070.6 |
  1966949.9 |####
  1973829.2 |
  1980708.5 |
  1987587.8 |####
  1994467.1 |
  2001346.5 |
  2008225.8 |
  2015105.1 |
  (3 below, 5 above range)

bitpack-contend-packed (n=40, range 692966.2-801474.6 ns)
  692966.2 |########################################
  698391.6 |###############################
  703817.0 |########
  709242.4 |###############################
  714667.8 |####
  720093.3 |
  725518.7 |
  730944.1 |
  736369.5 |
  741794.9 |
  747220.4 |
  752645.8 |
  758071.2 |
  763496.6 |
  768922.0 |
  774347.5 |
  779772.9 |
  785198.3 |
  790623.7 |
  796049.1 |##########################
  (4 below, 4 above range)

bitpack-contend-packed-simd (n=40, range 700151.1-805450.7 ns)
  700151.1 |###################################
  705416.0 |##############################
  710681.0 |#####
  715946.0 |
  721211.0 |
  726476.0 |#####
  731740.9 |
  737005.9 |
  742270.9 |
  747535.9 |
  752800.9 |
  758065.9 |#####
  763330.8 |##########
  768595.8 |##########
  773860.8 |###############
  779125.8 |##########
  784390.8 |
  789655.8 |#####
  794920.7 |########################################
  800185.7 |
  (4 below, 2 above range)

```
