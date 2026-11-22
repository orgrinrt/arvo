# Bitpacked against Dense over one column, swept from L1 to past a 12 MB L2

4 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed dominates: 12% faster than the next best (bitpack-footprint-dense)

bitpack-footprint-packed (9.92 us) leads bitpack-footprint-dense (11.09 us) by 12%, a clear separation rather than a photo finish. CV 8.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-footprint-packed-naive is an outlier: 4.3x slower than the field

bitpack-footprint-packed-naive (42.73 us) is 4.3x the fastest (9.92 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {bitpack-footprint-packed, bitpack-footprint-dense, bitpack-footprint-dense-alt} vs {bitpack-footprint-packed-naive} (250% apart)

The field splits into a fast tier {bitpack-footprint-packed, bitpack-footprint-dense, bitpack-footprint-dense-alt} and a slow tier {bitpack-footprint-packed-naive} with a 250% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.3x the fastest

Fastest bitpack-footprint-packed (9.92 us) to slowest bitpack-footprint-packed-naive (42.73 us): 4.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-footprint-packed** at 9915.5 ns median (-10.6% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 4.31x (fastest 9915.5 ns, slowest 42727.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Bitpacked against Dense over one column, swept from L1 to past a 12 MB L2 |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112837581 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 11178ns | 11186ns | 10035ns | 11167ns | 12351ns | base |
| bitpack-footprint-dense-alt | 12289ns | 12280ns | 12279ns | 12282ns | 12317ns | +9.94% |
| bitpack-footprint-packed | 9538ns | 9997ns | 8133ns | 9845ns | 10019ns | -14.67% |
| bitpack-footprint-packed-naive | 44920ns | 42804ns | 41687ns | 43515ns | 52366ns | +301.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 11098ns | 9969ns | 12255ns | base | 5.905 |
| bitpack-footprint-dense-alt | 12202ns | 12198ns | 12213ns | +9.95% | 5.371 |
| bitpack-footprint-packed | 9455ns | 8062ns | 9917ns | -14.81% | 6.932 |
| bitpack-footprint-packed-naive | 44841ns | 41619ns | 52273ns | +304.04% | 1.462 |

## Performance model

- Peak throughput: **8.129 Gops/s** (bitpack-footprint-packed; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 5.911 | 72.7% |
| bitpack-footprint-dense-alt | 5.372 | 66.1% |
| bitpack-footprint-packed | 6.609 | 81.3% |
| bitpack-footprint-packed-naive | 1.534 | 18.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 11178ns | 11178ns | base |
| bitpack-footprint-dense-alt | 12289ns | 12289ns | +9.94% |
| bitpack-footprint-packed | 9538ns | 9538ns | -14.67% |
| bitpack-footprint-packed-naive | 44920ns | 44920ns | +301.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 11087ns | base | --- | [9981, 12198] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 12199ns | +1113.5ns (+10.0%) | [+1, +2220]ns | [12199, 12200] | YES | 0.0007 | 0.0007 | 0 |
| bitpack-footprint-packed | 9915ns | -2103.3ns (-19.0%) | [-2282, -1913]ns | [9915, 9916] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-footprint-packed-naive | 42727ns | +31662.0ns (+285.6%) | [+30513, +32764]ns | [42710, 42774] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|---|---|---|
| 1 | 1 | 9981ns | +22.2% | -19.2% | +328.2% |
| 1 | 2 | 12199ns | -0.0% | -18.7% | +250.2% |
| 1 | 3 | 12199ns | +0.0% | -18.7% | +241.2% |
| 1 | 4 | 9969ns | +22.4% | -0.5% | +424.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | -0.250 | moderate- |
| bitpack-footprint-dense-alt | -0.130 | ok |
| bitpack-footprint-packed | -0.083 | ok |
| bitpack-footprint-packed-naive | -0.170 | ok |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 0/4, lost 2/4
- **bitpack-footprint-packed**: won 4/4, lost 0/4
- **bitpack-footprint-packed-naive**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 21.5ns | 11098.0ns | 0.2% |  |
| bitpack-footprint-dense-alt | 23.6ns | 12202.0ns | 0.2% |  |
| bitpack-footprint-packed | 22.6ns | 9454.5ns | 0.2% |  |
| bitpack-footprint-packed-naive | 21.1ns | 44840.5ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 9969.0-12254.8 ns)
   9969.0 |#################################
  10083.3 |
  10197.6 |
  10311.9 |
  10426.1 |
  10540.4 |
  10654.7 |
  10769.0 |
  10883.3 |
  10997.6 |
  11111.9 |
  11226.2 |
  11340.5 |
  11454.8 |
  11569.1 |
  11683.4 |
  11797.6 |
  11911.9 |
  12026.2 |
  12140.5 |########################################
  (4 below, 1 above range)

bitpack-footprint-dense-alt (n=40, range 12198.2-12212.5 ns)
  12198.2 |########################################
  12199.0 |#################
  12199.7 |##########
  12200.4 |
  12201.1 |
  12201.8 |
  12202.5 |##
  12203.3 |
  12204.0 |
  12204.7 |
  12205.4 |
  12206.1 |
  12206.8 |
  12207.5 |
  12208.3 |
  12209.0 |##
  12209.7 |##
  12210.4 |##
  12211.1 |#####
  12211.8 |##
  (3 below, 3 above range)

bitpack-footprint-packed (n=40, range 8062.4-9917.3 ns)
   8062.4 |#########
   8155.2 |
   8247.9 |
   8340.7 |
   8433.4 |
   8526.2 |
   8618.9 |
   8711.7 |
   8804.4 |
   8897.1 |
   8989.9 |
   9082.6 |
   9175.4 |
   9268.1 |
   9360.9 |
   9453.6 |
   9546.4 |
   9639.1 |
   9731.9 |
   9824.6 |########################################
  (4 below, 4 above range)

bitpack-footprint-packed-naive (n=40, range 41619.1-52273.2 ns)
  41619.1 |##########
  42151.8 |
  42684.5 |########################################
  43217.2 |
  43749.9 |
  44282.6 |
  44815.3 |
  45348.0 |
  45880.7 |
  46413.4 |
  46946.1 |
  47478.8 |
  48011.5 |
  48544.2 |
  49076.9 |
  49609.7 |
  50142.4 |
  50675.1 |
  51207.8 |
  51740.5 |############
  (5 below, 4 above range)

```
