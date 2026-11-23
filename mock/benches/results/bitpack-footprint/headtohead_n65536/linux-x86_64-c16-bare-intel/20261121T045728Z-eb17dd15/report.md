# Bitpacked against Dense over one column, swept from L1 to past a 12 MB L2

4 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed dominates: 13% faster than the next best (bitpack-footprint-dense-alt)

bitpack-footprint-packed (9.84 us) leads bitpack-footprint-dense-alt (11.08 us) by 13%, a clear separation rather than a photo finish. CV 28.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-footprint-packed-naive is an outlier: 4.8x slower than the field

bitpack-footprint-packed-naive (47.54 us) is 4.8x the fastest (9.84 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-footprint-packed is fastest but the noisiest (CV 28.2%)

bitpack-footprint-packed wins on median (9.84 us) yet has the highest variance (CV 28.2%), while bitpack-footprint-dense-alt is the steadiest (CV 10.0%, 11.08 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Two tiers: {bitpack-footprint-packed, bitpack-footprint-dense-alt, bitpack-footprint-dense} vs {bitpack-footprint-packed-naive} (329% apart)

The field splits into a fast tier {bitpack-footprint-packed, bitpack-footprint-dense-alt, bitpack-footprint-dense} and a slow tier {bitpack-footprint-packed-naive} with a 329% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.8x the fastest

Fastest bitpack-footprint-packed (9.84 us) to slowest bitpack-footprint-packed-naive (47.54 us): 4.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-footprint-packed is inconsistent: worst-20% is 1.6x its best-20%

bitpack-footprint-packed's best 20% of batches run at 8.05 us but its worst 20% at 13.21 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-footprint-packed** at 9841.2 ns median (-11.2% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 4.83x (fastest 9841.2 ns, slowest 47540.7 ns)

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
| Counter frequency | 2112765842 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 11168ns | 11181ns | 10043ns | 11166ns | 12299ns | base |
| bitpack-footprint-dense-alt | 11164ns | 11182ns | 10042ns | 11162ns | 12294ns | -0.03% |
| bitpack-footprint-packed | 10170ns | 9924ns | 8126ns | 9794ns | 13339ns | -8.94% |
| bitpack-footprint-packed-naive | 47306ns | 47620ns | 41689ns | 47490ns | 52371ns | +323.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 11088ns | 9976ns | 12203ns | base | 5.911 |
| bitpack-footprint-dense-alt | 11084ns | 9975ns | 12199ns | -0.03% | 5.912 |
| bitpack-footprint-packed | 10080ns | 8054ns | 13207ns | -9.09% | 6.502 |
| bitpack-footprint-packed-naive | 47225ns | 41618ns | 52275ns | +325.93% | 1.388 |

## Performance model

- Peak throughput: **8.137 Gops/s** (bitpack-footprint-packed; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 5.911 | 72.6% |
| bitpack-footprint-dense-alt | 5.913 | 72.7% |
| bitpack-footprint-packed | 6.659 | 81.8% |
| bitpack-footprint-packed-naive | 1.379 | 16.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 11168ns | 11168ns | base |
| bitpack-footprint-dense-alt | 11164ns | 11164ns | -0.03% |
| bitpack-footprint-packed | 10170ns | 10170ns | -8.94% |
| bitpack-footprint-packed-naive | 47306ns | 47306ns | +323.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 11088ns | base | --- | [9978, 12193] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 11084ns | no significant difference | [-6, +0]ns | [9978, 12184] | no | 0.2682 | 0.2682 | 0 |
| bitpack-footprint-packed | 9841ns | -1921.2ns (-17.3%) | [-2350, -1373]ns | [9766, 9844] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-footprint-packed-naive | 47541ns | +36454.8ns (+328.8%) | [+32763, +40050]ns | [42741, 52250] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|---|---|---|
| 1 | 1 | 12198ns | -0.0% | -19.9% | +328.4% |
| 1 | 2 | 9976ns | +0.0% | -19.3% | +317.2% |
| 1 | 3 | 9977ns | -0.0% | -1.3% | +328.4% |
| 1 | 4 | 12194ns | -0.1% | -19.3% | +328.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | -0.250 | moderate- |
| bitpack-footprint-dense-alt | -0.250 | moderate- |
| bitpack-footprint-packed | -0.392 | moderate- |
| bitpack-footprint-packed-naive | -0.252 | moderate- |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 0/4, lost 0/4
- **bitpack-footprint-packed**: won 4/4, lost 0/4
- **bitpack-footprint-packed-naive**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 21.4ns | 11087.5ns | 0.2% |  |
| bitpack-footprint-dense-alt | 21.4ns | 11084.4ns | 0.2% |  |
| bitpack-footprint-packed | 24.0ns | 10079.9ns | 0.2% |  |
| bitpack-footprint-packed-naive | 21.5ns | 47224.5ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 9975.7-12203.2 ns)
   9975.7 |#####################################
  10087.1 |
  10198.4 |
  10309.8 |
  10421.2 |
  10532.6 |
  10643.9 |
  10755.3 |
  10866.7 |
  10978.1 |
  11089.5 |
  11200.8 |
  11312.2 |
  11423.6 |
  11535.0 |
  11646.3 |
  11757.7 |
  11869.1 |
  11980.5 |
  12091.8 |########################################
  (4 below, 3 above range)

bitpack-footprint-dense-alt (n=40, range 9975.3-12199.2 ns)
   9975.3 |########################################
  10086.5 |
  10197.7 |
  10308.9 |
  10420.1 |
  10531.3 |
  10642.5 |
  10753.7 |
  10864.9 |
  10976.1 |
  11087.3 |
  11198.5 |
  11309.7 |
  11420.9 |
  11532.1 |
  11643.2 |
  11754.4 |
  11865.6 |
  11976.8 |
  12088.0 |########################################
  (3 below, 3 above range)

bitpack-footprint-packed (n=40, range 8054.0-13206.9 ns)
   8054.0 |#######
   8311.6 |###
   8569.3 |
   8826.9 |
   9084.5 |
   9342.2 |
   9599.8 |########################################
   9857.5 |
  10115.1 |
  10372.8 |
  10630.4 |#
  10888.1 |
  11145.7 |
  11403.3 |
  11661.0 |
  11918.6 |
  12176.3 |
  12433.9 |
  12691.6 |
  12949.2 |
  (3 below, 3 above range)

bitpack-footprint-packed-naive (n=40, range 41617.5-52275.1 ns)
  41617.5 |##########
  42150.4 |
  42683.3 |#####################
  43216.2 |
  43749.1 |
  44281.9 |
  44814.8 |
  45347.7 |
  45880.6 |
  46413.5 |
  46946.3 |
  47479.2 |
  48012.1 |
  48545.0 |
  49077.9 |
  49610.7 |
  50143.6 |
  50676.5 |
  51209.4 |
  51742.3 |########################################
  (5 below, 1 above range)

```

## Diagnostics

- **bitpack-footprint-packed**: CV=27.6% (high variance, measurements may be unstable)
