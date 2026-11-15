# Bitpacked against Dense over one column, swept from L1 to past a 12 MB L2

4 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed dominates: 23% faster than the next best (bitpack-footprint-dense-alt)

bitpack-footprint-packed (9.92 us) leads bitpack-footprint-dense-alt (12.20 us) by 23%, a clear separation rather than a photo finish. CV 7.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-footprint-packed-naive is an outlier: 5.2x slower than the field

bitpack-footprint-packed-naive (51.57 us) is 5.2x the fastest (9.92 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {bitpack-footprint-packed, bitpack-footprint-dense-alt, bitpack-footprint-dense} vs {bitpack-footprint-packed-naive} (323% apart)

The field splits into a fast tier {bitpack-footprint-packed, bitpack-footprint-dense-alt, bitpack-footprint-dense} and a slow tier {bitpack-footprint-packed-naive} with a 323% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.2x the fastest

Fastest bitpack-footprint-packed (9.92 us) to slowest bitpack-footprint-packed-naive (51.57 us): 5.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-footprint-packed** at 9915.1 ns median (-18.7% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.20x (fastest 9915.1 ns, slowest 51567.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 11731ns | 12281ns | 10048ns | 12098ns | 12311ns | base |
| bitpack-footprint-dense-alt | 11728ns | 12280ns | 10048ns | 12096ns | 12305ns | -0.02% |
| bitpack-footprint-packed | 9548ns | 9997ns | 8178ns | 9847ns | 10021ns | -18.61% |
| bitpack-footprint-packed-naive | 49621ns | 51657ns | 42773ns | 50975ns | 52405ns | +322.99% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 11647ns | 9980ns | 12211ns | base | 5.627 |
| bitpack-footprint-dense-alt | 11646ns | 9981ns | 12209ns | -0.01% | 5.627 |
| bitpack-footprint-packed | 9464ns | 8111ns | 9917ns | -18.74% | 6.925 |
| bitpack-footprint-packed-naive | 49533ns | 42704ns | 52306ns | +325.29% | 1.323 |

## Performance model

- Peak throughput: **8.080 Gops/s** (bitpack-footprint-packed; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 5.372 | 66.5% |
| bitpack-footprint-dense-alt | 5.373 | 66.5% |
| bitpack-footprint-packed | 6.610 | 81.8% |
| bitpack-footprint-packed-naive | 1.271 | 15.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 11731ns | 11731ns | base |
| bitpack-footprint-dense-alt | 11728ns | 11728ns | -0.02% |
| bitpack-footprint-packed | 9548ns | 9548ns | -18.61% |
| bitpack-footprint-packed-naive | 49621ns | 49621ns | +322.99% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 12199ns | base | --- | [12198, 12199] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 12198ns | no significant difference | [-12, +4]ns | [12198, 12199] | no | 0.6358 | 0.6358 | 0 |
| bitpack-footprint-packed | 9915ns | -2283.6ns (-18.7%) | [-2294, -2282]ns | [9915, 9915] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-footprint-packed-naive | 51568ns | +40046.3ns (+328.3%) | [+40015, +40112]ns | [50888, 52226] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|---|---|---|
| 1 | 1 | 9980ns | +22.2% | -0.7% | +409.9% |
| 1 | 2 | 12203ns | -0.0% | -33.5% | +249.9% |
| 1 | 3 | 12199ns | -0.0% | -18.7% | +328.3% |
| 1 | 4 | 12198ns | -18.2% | -18.7% | +328.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | -0.085 | ok |
| bitpack-footprint-dense-alt | -0.083 | ok |
| bitpack-footprint-packed | -0.417 | moderate- |
| bitpack-footprint-packed-naive | -0.324 | moderate- |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 1/4, lost 1/4
- **bitpack-footprint-packed**: won 4/4, lost 0/4
- **bitpack-footprint-packed-naive**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 22.5ns | 11646.9ns | 0.2% |  |
| bitpack-footprint-dense-alt | 22.8ns | 11646.1ns | 0.2% |  |
| bitpack-footprint-packed | 22.9ns | 9464.1ns | 0.2% |  |
| bitpack-footprint-packed-naive | 23.3ns | 49532.8ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 9980.2-12210.9 ns)
   9980.2 |########
  10091.8 |
  10203.3 |
  10314.8 |
  10426.4 |
  10537.9 |
  10649.4 |
  10761.0 |
  10872.5 |
  10984.0 |
  11095.6 |
  11207.1 |
  11318.6 |
  11430.2 |
  11541.7 |
  11653.2 |
  11764.8 |
  11876.3 |
  11987.8 |
  12099.4 |########################################
  (4 below, 2 above range)

bitpack-footprint-dense-alt (n=40, range 9981.4-12209.0 ns)
   9981.4 |##########
  10092.7 |
  10204.1 |
  10315.5 |
  10426.9 |
  10538.3 |
  10649.6 |
  10761.0 |
  10872.4 |
  10983.8 |
  11095.2 |
  11206.6 |
  11317.9 |
  11429.3 |
  11540.7 |
  11652.1 |
  11763.5 |
  11874.9 |
  11986.2 |
  12097.6 |########################################
  (4 below, 6 above range)

bitpack-footprint-packed (n=40, range 8111.1-9917.1 ns)
   8111.1 |#############
   8201.4 |
   8291.7 |
   8382.0 |
   8472.3 |
   8562.6 |
   8652.9 |
   8743.2 |
   8833.5 |
   8923.8 |
   9014.1 |
   9104.4 |
   9194.7 |
   9285.0 |
   9375.3 |
   9465.6 |
   9555.9 |
   9646.2 |
   9736.5 |
   9826.8 |########################################
  (1 below, 3 above range)

bitpack-footprint-packed-naive (n=40, range 42704.3-52306.0 ns)
  42704.3 |##########
  43184.4 |
  43664.5 |
  44144.6 |
  44624.7 |
  45104.8 |
  45584.9 |
  46064.9 |
  46545.0 |
  47025.1 |
  47505.2 |
  47985.3 |
  48465.4 |
  48945.4 |
  49425.5 |
  49905.6 |
  50385.7 |
  50865.8 |#########################
  51345.9 |
  51825.9 |########################################
  (6 below, 4 above range)

```
