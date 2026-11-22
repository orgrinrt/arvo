# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary period-aligned

3 variants, 40 samples per variant.
Baseline: **bitpack-write-aligned**

## Highlights

Baseline for all deltas below: **bitpack-write-aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-write-aligned) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-write-aligned has the worst median (162.51 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-write-dense at 28.62 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-write-dense dominates: 308% faster than the next best (bitpack-write-windowed)

bitpack-write-dense (28.62 us) leads bitpack-write-windowed (116.70 us) by 308%, a clear separation rather than a photo finish. CV 25.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-dense beats baseline by 83% (significant)

bitpack-write-dense is -134.12 us (83%) faster than baseline bitpack-write-aligned, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-write-aligned is an outlier: 5.7x slower than the field

bitpack-write-aligned (162.51 us) is 5.7x the fastest (28.62 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-write-aligned shows alternating (throttle bounce) (autocorr -0.51)

bitpack-write-aligned's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 5.7x the fastest

Fastest bitpack-write-dense (28.62 us) to slowest bitpack-write-aligned (162.51 us): 5.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-write-windowed is inconsistent: worst-20% is 2.8x its best-20%

bitpack-write-windowed's best 20% of batches run at 90.89 us but its worst 20% at 256.03 us (2.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-write-dense** at 28621.2 ns median (-82.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 5.68x (fastest 28621.2 ns, slowest 162510.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary period-aligned |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef1 |
| Counter frequency | 2112697486 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-write-aligned | 157235ns | 162617ns | 124993ns | 156331ns | 192188ns | base |
| bitpack-write-dense | 25319ns | 28699ns | 12730ns | 27602ns | 31062ns | -83.90% |
| bitpack-write-windowed | 143959ns | 116767ns | 91033ns | 124154ns | 256301ns | -8.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-aligned | 157116ns | 124885ns | 192059ns | base | 0.417 |
| bitpack-write-dense | 25234ns | 12647ns | 30963ns | -83.94% | 2.597 |
| bitpack-write-windowed | 143806ns | 90888ns | 256032ns | -8.47% | 0.456 |

## Performance model

- Peak throughput: **5.182 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-aligned | 0.403 | 7.8% |
| bitpack-write-dense | 2.290 | 44.2% |
| bitpack-write-windowed | 0.562 | 10.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-aligned | 157235ns | 157235ns | base |
| bitpack-write-dense | 25319ns | 25319ns | -83.90% |
| bitpack-write-windowed | 143959ns | 143959ns | -8.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-aligned | 162510ns | base | --- | [138656, 169223] | --- | --- | --- | --- |
| bitpack-write-dense | 28621ns | -134116.5ns (-82.5%) | [-143760, -117617]ns | [28361, 29009] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-write-windowed | 116698ns | -34321.2ns (-21.1%) | [-52807, -30296]ns | [97996, 119188] | YES | 0.0022 | 0.0022 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-write-aligned | bitpack-write-dense | bitpack-write-windowed |
|---|---|---|---|---|
| 1 | 1 | 126327ns | -90.0% | -27.2% |
| 1 | 2 | 162733ns | -82.5% | +27.7% |
| 1 | 3 | 137753ns | -79.2% | +18.0% |
| 1 | 4 | 175456ns | -82.5% | -33.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-aligned | -0.507 | HIGH- (thermal bounce) |
| bitpack-write-dense | -0.052 | ok |
| bitpack-write-windowed | -0.344 | moderate- |

**Consistency summary:**

- **bitpack-write-dense**: won 4/4, lost 0/4
- **bitpack-write-windowed**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-aligned | 26.1ns | 157116.3ns | 0.0% |  |
| bitpack-write-dense | 21.6ns | 25233.8ns | 0.1% |  |
| bitpack-write-windowed | 43.0ns | 143805.6ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-aligned (n=40, range 124885.3-192059.0 ns)
  124885.3 |####################################
  128244.0 |
  131602.7 |
  134961.3 |
  138320.0 |
  141678.7 |
  145037.4 |
  148396.1 |####
  151754.8 |
  155113.5 |
  158472.1 |####
  161830.8 |########################################
  165189.5 |
  168548.2 |
  171906.9 |############
  175265.6 |############################
  178624.3 |####
  181983.0 |
  185341.6 |
  188700.3 |
  (5 below, 3 above range)

bitpack-write-dense (n=40, range 12647.1-30963.0 ns)
  12647.1 |###############
  13562.9 |
  14478.7 |
  15394.5 |
  16310.3 |
  17226.1 |
  18141.9 |
  19057.7 |
  19973.5 |
  20889.3 |
  21805.1 |
  22720.8 |
  23636.6 |
  24552.4 |
  25468.2 |
  26384.0 |
  27299.8 |##
  28215.6 |########################################
  29131.4 |#######
  30047.2 |###############
  (4 below, 4 above range)

bitpack-write-windowed (n=40, range 90888.3-256032.2 ns)
  90888.3 |########################################
  99145.5 |
  107402.7 |########
  115659.9 |########################
  123917.1 |
  132174.3 |
  140431.5 |
  148688.7 |
  156945.8 |
  165203.0 |
  173460.2 |#####
  181717.4 |
  189974.6 |
  198231.8 |
  206489.0 |
  214746.2 |
  223003.4 |########
  231260.6 |#####
  239517.8 |#####
  247775.0 |#####
  (1 below, 1 above range)

```

## Diagnostics

- **bitpack-write-dense**: CV=29.0% (high variance, measurements may be unstable)
- **bitpack-write-windowed**: CV=46.5% (high variance, measurements may be unstable)
