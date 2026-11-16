# Bitpacked against Dense over one column, swept from L1 to past a 12 MB L2

4 variants, 40 samples per variant.
Baseline: **bitpack-footprint-dense**

## Highlights

Baseline for all deltas below: **bitpack-footprint-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-footprint-packed dominates: 24% faster than the next best (bitpack-footprint-dense-alt)

bitpack-footprint-packed (2.47 us) leads bitpack-footprint-dense-alt (3.05 us) by 24%, a clear separation rather than a photo finish. CV 90.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-footprint-packed-naive is an outlier: 5.1x slower than the field

bitpack-footprint-packed-naive (12.68 us) is 5.1x the fastest (2.47 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-footprint-packed is fastest but the noisiest (CV 90.3%)

bitpack-footprint-packed wins on median (2.47 us) yet has the highest variance (CV 90.3%), while bitpack-footprint-packed-naive is the steadiest (CV 7.8%, 12.68 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Two tiers: {bitpack-footprint-packed, bitpack-footprint-dense-alt, bitpack-footprint-dense} vs {bitpack-footprint-packed-naive} (315% apart)

The field splits into a fast tier {bitpack-footprint-packed, bitpack-footprint-dense-alt, bitpack-footprint-dense} and a slow tier {bitpack-footprint-packed-naive} with a 315% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.1x the fastest

Fastest bitpack-footprint-packed (2.47 us) to slowest bitpack-footprint-packed-naive (12.68 us): 5.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-footprint-packed is inconsistent: worst-20% is 3.5x its best-20%

bitpack-footprint-packed's best 20% of batches run at 2.17 us but its worst 20% at 7.58 us (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-footprint-packed** at 2466.6 ns median (-19.3% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.14x (fastest 2466.6 ns, slowest 12679.8 ns)

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
| bitpack-footprint-dense | 4476ns | 3123ns | 2763ns | 3460ns | 9236ns | base |
| bitpack-footprint-dense-alt | 4066ns | 3118ns | 2621ns | 3146ns | 8270ns | -9.16% |
| bitpack-footprint-packed | 3933ns | 2530ns | 2233ns | 3201ns | 7831ns | -12.12% |
| bitpack-footprint-packed-naive | 12195ns | 12759ns | 10447ns | 12575ns | 12800ns | +172.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-footprint-dense | 4369ns | 2705ns | 8979ns | base | 3.750 |
| bitpack-footprint-dense-alt | 3970ns | 2567ns | 8054ns | -9.12% | 4.127 |
| bitpack-footprint-packed | 3823ns | 2175ns | 7579ns | -12.49% | 4.286 |
| bitpack-footprint-packed-naive | 12118ns | 10379ns | 12709ns | +177.39% | 1.352 |

## Performance model

- Peak throughput: **7.535 Gops/s** (bitpack-footprint-packed; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-footprint-dense | 5.360 | 71.1% |
| bitpack-footprint-dense-alt | 5.371 | 71.3% |
| bitpack-footprint-packed | 6.642 | 88.2% |
| bitpack-footprint-packed-naive | 1.292 | 17.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-footprint-dense | 4476ns | 4476ns | base |
| bitpack-footprint-dense-alt | 4066ns | 4066ns | -9.16% |
| bitpack-footprint-packed | 3933ns | 3933ns | -12.12% |
| bitpack-footprint-packed-naive | 12195ns | 12195ns | +172.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-footprint-dense | 3056ns | base | --- | [3051, 3063] | --- | --- | --- | --- |
| bitpack-footprint-dense-alt | 3051ns | no significant difference | [-2, +0]ns | [3051, 3063] | no | 0.8714 | 0.8714 | 2 |
| bitpack-footprint-packed | 2467ns | -585.2ns (-19.1%) | [-606, -571]ns | [2465, 2482] | YES | 0.0003 | 0.0002 | 0 |
| bitpack-footprint-packed-naive | 12680ns | +9622.3ns (+314.8%) | [+7876, +9625]ns | [12675, 12688] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-footprint-dense | bitpack-footprint-dense-alt | bitpack-footprint-packed | bitpack-footprint-packed-naive |
|---|---|---|---|---|---|
| 1 | 1 | 3056ns | +0.1% | -10.9% | +315.2% |
| 1 | 2 | 4066ns | -38.6% | -39.4% | +155.2% |
| 1 | 3 | 3057ns | -0.2% | -19.2% | +314.8% |
| 1 | 4 | 3056ns | -0.0% | -19.3% | +315.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-footprint-dense | -0.417 | moderate- |
| bitpack-footprint-dense-alt | -0.417 | moderate- |
| bitpack-footprint-packed | -0.100 | ok |
| bitpack-footprint-packed-naive | -0.416 | moderate- |

**Consistency summary:**

- **bitpack-footprint-dense-alt**: won 2/4, lost 0/4
- **bitpack-footprint-packed**: won 4/4, lost 0/4
- **bitpack-footprint-packed-naive**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-footprint-dense | 34.0ns | 4368.5ns | 0.8% |  |
| bitpack-footprint-dense-alt | 31.0ns | 3970.3ns | 0.8% |  |
| bitpack-footprint-packed | 36.8ns | 3822.8ns | 1.0% |  |
| bitpack-footprint-packed-naive | 23.0ns | 12117.7ns | 0.2% |  |

## Distribution (algo ns)

```
bitpack-footprint-dense (n=40, range 2705.3-8979.0 ns)
   2705.3 |
   3019.0 |########################################
   3332.6 |
   3646.3 |
   3960.0 |
   4273.7 |
   4587.4 |
   4901.1 |
   5214.8 |#
   5528.5 |#
   5842.1 |#
   6155.8 |
   6469.5 |
   6783.2 |
   7096.9 |#
   7410.6 |
   7724.3 |#
   8038.0 |###
   8351.7 |
   8665.3 |#
  (5 below, 3 above range)

bitpack-footprint-dense-alt (n=40, range 2566.5-8054.2 ns)
   2566.5 |
   2840.9 |########################################
   3115.3 |
   3389.7 |#
   3664.1 |
   3938.4 |
   4212.8 |#
   4487.2 |
   4761.6 |
   5036.0 |#
   5310.4 |
   5584.8 |
   5859.2 |
   6133.5 |
   6407.9 |
   6682.3 |
   6956.7 |
   7231.1 |
   7505.5 |
   7779.9 |
  (7 below, 6 above range)

bitpack-footprint-packed (n=40, range 2174.5-7578.5 ns)
   2174.5 |#
   2444.7 |########################################
   2714.9 |
   2985.1 |
   3255.3 |#
   3525.5 |
   3795.7 |
   4065.9 |
   4336.1 |
   4606.3 |
   4876.5 |
   5146.7 |#
   5416.9 |
   5687.1 |
   5957.3 |
   6227.5 |###
   6497.7 |#####
   6767.9 |
   7038.1 |###
   7308.3 |#
  (5 below, 3 above range)

bitpack-footprint-packed-naive (n=40, range 10379.4-12709.5 ns)
  10379.4 |######
  10495.9 |#
  10612.4 |
  10728.9 |
  10845.4 |
  10961.9 |
  11078.4 |
  11194.9 |
  11311.4 |
  11427.9 |
  11544.5 |
  11661.0 |
  11777.5 |
  11894.0 |
  12010.5 |
  12127.0 |
  12243.5 |
  12360.0 |
  12476.5 |
  12593.0 |########################################
  (5 below, 4 above range)

```

## Diagnostics

- **bitpack-footprint-dense**: CV=56.9% (high variance, measurements may be unstable)
- **bitpack-footprint-dense**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **bitpack-footprint-dense-alt**: CV=56.3% (high variance, measurements may be unstable)
- **bitpack-footprint-dense-alt**: worst_20/best_20 = 3.1x (possible bimodal distribution)
- **bitpack-footprint-packed**: CV=58.3% (high variance, measurements may be unstable)
- **bitpack-footprint-packed**: worst_20/best_20 = 3.5x (possible bimodal distribution)
