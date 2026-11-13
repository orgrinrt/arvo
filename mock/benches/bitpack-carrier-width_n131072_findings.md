# Packed 13-bit against u16, u32 and u64 dense carriers, swept from L1 to past a 12 MB L2

6 variants, 40 samples per variant.
Baseline: **bitpack-carrier-d16**

## Highlights

Baseline for all deltas below: **bitpack-carrier-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-carrier-d64 shows warm-up / thermal drift (autocorr +0.85)

bitpack-carrier-d64's per-pass series has lag-1 autocorrelation +0.85, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {bitpack-carrier-d32, bitpack-carrier-d16, bitpack-carrier-d16-control, bitpack-carrier-d64} vs {bitpack-carrier-packed-simd, bitpack-carrier-packed} (26% apart)

The field splits into a fast tier {bitpack-carrier-d32, bitpack-carrier-d16, bitpack-carrier-d16-control, bitpack-carrier-d64} and a slow tier {bitpack-carrier-packed-simd, bitpack-carrier-packed} with a 26% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: bitpack-carrier-d32** at 11195.4 ns median (-1.4% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 1.51x (fastest 11195.4 ns, slowest 16865.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 11413ns | 11457ns | 11183ns | 11412ns | 11647ns | base |
| bitpack-carrier-d16-control | 11515ns | 11457ns | 11182ns | 11443ns | 12063ns | +0.89% |
| bitpack-carrier-d32 | 11255ns | 11299ns | 11033ns | 11267ns | 11443ns | -1.39% |
| bitpack-carrier-d64 | 12642ns | 12686ns | 12392ns | 12677ns | 12788ns | +10.76% |
| bitpack-carrier-packed | 17249ns | 16964ns | 16709ns | 17027ns | 18454ns | +51.13% |
| bitpack-carrier-packed-simd | 16007ns | 15972ns | 15698ns | 15936ns | 16529ns | +40.25% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-carrier-d16 | 11315ns | 11088ns | 11546ns | base | 11.584 |
| bitpack-carrier-d16-control | 11414ns | 11090ns | 11946ns | +0.88% | 11.483 |
| bitpack-carrier-d32 | 11156ns | 10940ns | 11342ns | -1.40% | 11.749 |
| bitpack-carrier-d64 | 12543ns | 12296ns | 12688ns | +10.86% | 10.450 |
| bitpack-carrier-packed | 17147ns | 16619ns | 18328ns | +51.55% | 7.644 |
| bitpack-carrier-packed-simd | 15908ns | 15606ns | 16416ns | +40.60% | 8.239 |

## Performance model

- Peak throughput: **11.981 Gops/s** (bitpack-carrier-d32; best 20% batches)
- Ops per call: 131072

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-carrier-d16 | 11.540 | 96.3% |
| bitpack-carrier-d16-control | 11.537 | 96.3% |
| bitpack-carrier-d32 | 11.708 | 97.7% |
| bitpack-carrier-d64 | 10.417 | 86.9% |
| bitpack-carrier-packed | 7.772 | 64.9% |
| bitpack-carrier-packed-simd | 8.258 | 68.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-carrier-d16 | 11413ns | 11413ns | base |
| bitpack-carrier-d16-control | 11515ns | 11515ns | +0.89% |
| bitpack-carrier-d32 | 11255ns | 11255ns | -1.39% |
| bitpack-carrier-d64 | 12642ns | 12642ns | +10.76% |
| bitpack-carrier-packed | 17249ns | 17249ns | +51.13% |
| bitpack-carrier-packed-simd | 16007ns | 16007ns | +40.25% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 11358ns | base | --- | [11234, 11388] | --- | --- | --- | --- |
| bitpack-carrier-d16-control | 11361ns | +54.0ns (+0.5%) | [+10, +183]ns | [11178, 11403] | YES | 0.0166 | 0.0166 | 0 |
| bitpack-carrier-d32 | 11195ns | -143.4ns (-1.3%) | [-163, -102]ns | [11110, 11246] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-d64 | 12582ns | +1222.9ns (+10.8%) | [+1200, +1269]ns | [12555, 12617] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-packed | 16866ns | +5560.2ns (+49.0%) | [+5525, +5753]ns | [16635, 17131] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-packed-simd | 15873ns | +4601.3ns (+40.5%) | [+4511, +4646]ns | [15670, 16003] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-carrier-d16 | bitpack-carrier-d16-control | bitpack-carrier-d32 | bitpack-carrier-d64 | bitpack-carrier-packed | bitpack-carrier-packed-simd |
|---|---|---|---|---|---|---|
| 1 | 11388ns | +2.2% | -1.5% | +10.7% | +65.4% | +40.4% |
| 2 | 11334ns | +3.3% | -0.6% | +12.1% | +58.6% | +41.7% |
| 3 | 11387ns | +3.8% | -1.2% | +10.6% | +56.3% | +45.4% |
| 4 | 11399ns | +2.9% | -1.9% | +11.5% | +57.6% | +43.4% |
| 5 | 11524ns | +4.0% | -1.0% | +9.8% | +56.4% | +44.4% |
| 6 | 11352ns | +16.0% | -0.8% | +11.1% | +63.7% | +41.1% |
| 7 | 11403ns | +2.6% | -1.4% | +11.1% | +55.1% | +40.4% |
| 8 | 11375ns | +1.7% | -1.1% | +10.9% | +59.8% | +43.2% |
| 9 | 11452ns | +2.1% | -1.7% | +10.5% | +54.4% | +44.1% |
| 10 | 11363ns | +2.9% | -0.3% | +11.6% | +69.3% | +47.2% |
| 11 | 11380ns | +0.1% | -1.3% | +10.9% | +50.6% | +41.0% |
| 12 | 11402ns | -0.3% | -0.4% | +10.7% | +49.4% | +40.2% |
| 13 | 11320ns | +0.3% | -0.1% | +11.3% | +51.3% | +42.1% |
| 14 | 11315ns | +0.5% | -0.3% | +12.0% | +51.1% | +41.2% |
| 15 | 11558ns | +0.1% | -3.2% | +9.4% | +47.7% | +38.1% |
| 16 | 11482ns | -1.2% | -2.1% | +9.6% | +50.1% | +39.0% |
| 17 | 11367ns | +0.3% | -1.3% | +11.2% | +50.1% | +40.6% |
| 18 | 11364ns | +0.3% | -0.8% | +11.7% | +51.3% | +40.9% |
| 19 | 11414ns | +1.6% | -1.8% | +11.2% | +50.1% | +41.6% |
| 20 | 11414ns | +3.1% | -1.3% | +10.1% | +49.8% | +40.7% |
| 21 | 11255ns | +0.7% | -1.1% | +10.9% | +47.7% | +38.9% |
| 22 | 11188ns | +1.8% | -0.0% | +10.0% | +48.6% | +39.5% |
| 23 | 11085ns | +1.7% | +3.1% | +10.9% | +50.2% | +41.0% |
| 24 | 11102ns | +1.2% | +2.4% | +10.7% | +49.7% | +41.5% |
| 25 | 11144ns | +2.0% | +0.1% | +10.7% | +49.2% | +40.1% |
| 26 | 11090ns | -0.0% | +0.2% | +10.8% | +50.1% | +41.4% |
| 27 | 11086ns | +0.5% | +0.2% | +10.8% | +49.9% | +40.8% |
| 28 | 11088ns | +0.8% | -0.1% | +11.1% | +49.9% | +42.4% |
| 29 | 11190ns | -0.4% | -2.1% | +9.8% | +48.7% | +39.9% |
| 30 | 11086ns | +0.0% | -1.0% | +11.0% | +49.9% | +41.7% |
| 31 | 11666ns | -4.9% | -6.1% | +7.5% | +43.1% | +33.8% |
| 32 | 11528ns | -3.7% | -4.5% | +8.9% | +44.7% | +35.4% |
| 33 | 11728ns | -5.5% | -6.7% | +7.1% | +41.8% | +33.6% |
| 34 | 11433ns | -3.0% | -4.3% | +10.0% | +45.8% | +36.5% |
| 35 | 11184ns | -0.9% | -1.8% | +12.2% | +48.7% | +40.1% |
| 36 | 11220ns | -1.0% | -2.5% | +12.1% | +48.1% | +39.3% |
| 37 | 11083ns | +0.4% | -1.3% | +13.3% | +49.9% | +40.8% |
| 38 | 11086ns | +0.8% | -1.4% | +13.4% | +50.0% | +40.9% |
| 39 | 11103ns | -0.2% | -1.2% | +13.0% | +49.7% | +40.6% |
| 40 | 11248ns | -1.1% | -2.8% | +12.7% | +47.9% | +39.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-carrier-d16 | 0.636 | HIGH+ (drift/warm-up) |
| bitpack-carrier-d16-control | 0.613 | HIGH+ (drift/warm-up) |
| bitpack-carrier-d32 | 0.806 | HIGH+ (drift/warm-up) |
| bitpack-carrier-d64 | 0.852 | HIGH+ (drift/warm-up) |
| bitpack-carrier-packed | 0.662 | HIGH+ (drift/warm-up) |
| bitpack-carrier-packed-simd | 0.767 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **bitpack-carrier-d16-control**: won 11/40, lost 25/40
- **bitpack-carrier-d32**: won 34/40, lost 5/40
- **bitpack-carrier-d64**: won 0/40, lost 40/40
- **bitpack-carrier-packed**: won 0/40, lost 40/40
- **bitpack-carrier-packed-simd**: won 0/40, lost 40/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-carrier-d16 | 2.7ns | 11314.6ns | 0.0% |  |
| bitpack-carrier-d16-control | 2.6ns | 11414.2ns | 0.0% |  |
| bitpack-carrier-d32 | 2.3ns | 11156.1ns | 0.0% |  |
| bitpack-carrier-d64 | 2.5ns | 12543.1ns | 0.0% |  |
| bitpack-carrier-packed | 2.9ns | 17146.9ns | 0.0% |  |
| bitpack-carrier-packed-simd | 2.7ns | 15908.1ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-carrier-d16 (n=40, range 11088.1-11546.4 ns)
  11088.1 |########################
  11111.0 |
  11133.9 |########
  11156.8 |
  11179.7 |########################
  11202.6 |########
  11225.5 |########
  11248.5 |########
  11271.4 |
  11294.3 |########
  11317.2 |################
  11340.1 |########
  11363.0 |########################################
  11385.9 |########################################
  11408.9 |################
  11431.8 |################
  11454.7 |
  11477.6 |########
  11500.5 |
  11523.4 |################
  (6 below, 3 above range)

bitpack-carrier-d16-control (n=40, range 11090.2-11946.4 ns)
  11090.2 |#################################
  11133.0 |####################
  11175.8 |######
  11218.6 |######
  11261.4 |######
  11304.2 |#############
  11347.1 |########################################
  11389.9 |#############
  11432.7 |
  11475.5 |
  11518.3 |
  11561.1 |####################
  11603.9 |######
  11646.7 |
  11689.5 |#################################
  11732.3 |######
  11775.1 |
  11818.0 |######
  11860.8 |
  11903.6 |
  (6 below, 2 above range)

bitpack-carrier-d32 (n=40, range 10940.2-11342.4 ns)
  10940.2 |########################
  10960.3 |################
  10980.4 |########
  11000.5 |########
  11020.6 |
  11040.7 |
  11060.9 |########
  11081.0 |
  11101.1 |################
  11121.2 |########
  11141.3 |########
  11161.4 |
  11181.5 |########################
  11201.6 |########################
  11221.7 |################
  11241.8 |########################################
  11262.0 |################################
  11282.1 |
  11302.2 |########
  11322.3 |########
  (5 below, 4 above range)

bitpack-carrier-d64 (n=40, range 12296.0-12687.6 ns)
  12296.0 |################
  12315.5 |########
  12335.1 |########
  12354.7 |
  12374.3 |
  12393.9 |
  12413.4 |
  12433.0 |
  12452.6 |
  12472.2 |########
  12491.8 |
  12511.4 |
  12530.9 |########################
  12550.5 |################################
  12570.1 |################################
  12589.7 |################
  12609.3 |########################################
  12628.8 |################
  12648.4 |################
  12668.0 |################################
  (5 below, 4 above range)

bitpack-carrier-packed (n=40, range 16619.0-18328.1 ns)
  16619.0 |########################################
  16704.4 |
  16789.9 |
  16875.3 |
  16960.8 |##
  17046.2 |############
  17131.7 |#######
  17217.2 |##
  17302.6 |
  17388.1 |
  17473.5 |
  17559.0 |
  17644.4 |#####
  17729.9 |##
  17815.4 |
  17900.8 |#####
  17986.3 |##
  18071.7 |
  18157.2 |##
  18242.7 |
  (4 below, 3 above range)

bitpack-carrier-packed-simd (n=40, range 15606.1-16415.7 ns)
  15606.1 |########################################
  15646.6 |############################
  15687.1 |###########
  15727.6 |
  15768.0 |#####
  15808.5 |
  15849.0 |
  15889.5 |
  15930.0 |###########
  15970.4 |######################
  16010.9 |######################
  16051.4 |#################
  16091.9 |
  16132.4 |#####
  16172.8 |
  16213.3 |
  16253.8 |#####
  16294.3 |
  16334.8 |#####
  16375.2 |
  (5 below, 4 above range)

```

## Diagnostics

- **bitpack-carrier-d16**: autocorrelation=0.64 (measurement drift or warm-up artifact)
- **bitpack-carrier-d16-control**: autocorrelation=0.61 (measurement drift or warm-up artifact)
- **bitpack-carrier-d32**: autocorrelation=0.81 (measurement drift or warm-up artifact)
- **bitpack-carrier-d64**: autocorrelation=0.85 (measurement drift or warm-up artifact)
- **bitpack-carrier-packed**: autocorrelation=0.66 (measurement drift or warm-up artifact)
- **bitpack-carrier-packed-simd**: autocorrelation=0.77 (measurement drift or warm-up artifact)
