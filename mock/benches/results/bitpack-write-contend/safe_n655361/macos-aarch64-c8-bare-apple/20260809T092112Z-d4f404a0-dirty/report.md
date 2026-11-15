# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary period-aligned

3 variants, 40 samples per variant.
Baseline: **bitpack-write-aligned**

## Highlights

Baseline for all deltas below: **bitpack-write-aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-write-aligned) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-write-aligned has the worst median (215.39 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-write-dense at 9.78 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-write-dense dominates: 1229% faster than the next best (bitpack-write-windowed)

bitpack-write-dense (9.78 us) leads bitpack-write-windowed (130.01 us) by 1229%, a clear separation rather than a photo finish. CV 12.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-dense beats baseline by 92% (significant)

bitpack-write-dense is -197.92 us (92%) faster than baseline bitpack-write-aligned, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-write-aligned is an outlier: 22.0x slower than the field

bitpack-write-aligned (215.39 us) is 22.0x the fastest (9.78 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-write-windowed shows warm-up / thermal drift (autocorr +0.78)

bitpack-write-windowed's per-pass series has lag-1 autocorrelation +0.78, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 22.0x the fastest

Fastest bitpack-write-dense (9.78 us) to slowest bitpack-write-aligned (215.39 us): 22.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-write-windowed is inconsistent: worst-20% is 1.8x its best-20%

bitpack-write-windowed's best 20% of batches run at 116.93 us but its worst 20% at 214.50 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-write-dense** at 9781.2 ns median (-95.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 22.02x (fastest 9781.2 ns, slowest 215386.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-write-aligned | 225741ns | 215641ns | 201437ns | 216824ns | 276793ns | base |
| bitpack-write-dense | 10334ns | 9892ns | 8948ns | 10180ns | 12184ns | -95.42% |
| bitpack-write-windowed | 146967ns | 130279ns | 117140ns | 134173ns | 215177ns | -34.90% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-aligned | 225190ns | 201188ns | 275260ns | base | 0.291 |
| bitpack-write-dense | 10204ns | 8854ns | 11991ns | -95.47% | 6.423 |
| bitpack-write-windowed | 146597ns | 116926ns | 214502ns | -34.90% | 0.447 |

## Performance model

- Peak throughput: **7.402 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-aligned | 0.304 | 4.1% |
| bitpack-write-dense | 6.700 | 90.5% |
| bitpack-write-windowed | 0.504 | 6.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-aligned | 225741ns | 225741ns | base |
| bitpack-write-dense | 10334ns | 10334ns | -95.42% |
| bitpack-write-windowed | 146967ns | 146967ns | -34.90% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-aligned | 215387ns | base | --- | [210333, 223283] | --- | --- | --- | --- |
| bitpack-write-dense | 9781ns | -205726.6ns (-95.5%) | [-212756, -200804]ns | [9732, 10441] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-write-windowed | 130007ns | -85984.6ns (-39.9%) | [-92745, -69681]ns | [125882, 135803] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-write-aligned | bitpack-write-dense | bitpack-write-windowed |
|---|---|---|---|
| 1 | 208748ns | -95.3% | -43.0% |
| 2 | 230148ns | -95.8% | -49.1% |
| 3 | 214813ns | -95.5% | -44.1% |
| 4 | 207216ns | -95.3% | -41.7% |
| 5 | 204645ns | -95.2% | -43.4% |
| 6 | 204589ns | -94.9% | -41.8% |
| 7 | 218090ns | -94.8% | -46.7% |
| 8 | 213517ns | -95.1% | -45.2% |
| 9 | 231689ns | -95.5% | -49.3% |
| 10 | 210856ns | -95.0% | -45.8% |
| 11 | 209810ns | -95.9% | -39.0% |
| 12 | 199188ns | -95.7% | -31.7% |
| 13 | 200364ns | -95.7% | -32.2% |
| 14 | 201533ns | -95.7% | -31.3% |
| 15 | 217161ns | -96.0% | -41.1% |
| 16 | 223431ns | -96.1% | -41.8% |
| 17 | 234352ns | -96.0% | -44.8% |
| 18 | 215960ns | -95.5% | -42.6% |
| 19 | 213067ns | -95.5% | -44.3% |
| 20 | 205037ns | -95.3% | -36.9% |
| 21 | 479270ns | -97.1% | -63.1% |
| 22 | 244487ns | -95.6% | -14.9% |
| 23 | 252745ns | -94.8% | -26.8% |
| 24 | 279382ns | -96.0% | -31.0% |
| 25 | 245480ns | -95.4% | -4.7% |
| 26 | 213200ns | -94.7% | -8.8% |
| 27 | 219149ns | -94.9% | -2.4% |
| 28 | 223208ns | -95.0% | +8.2% |
| 29 | 229424ns | -94.9% | -15.8% |
| 30 | 223200ns | -94.8% | +6.7% |
| 31 | 227148ns | -95.7% | -41.5% |
| 32 | 212978ns | -95.4% | -36.4% |
| 33 | 204412ns | -95.2% | -33.6% |
| 34 | 200454ns | -95.1% | -32.2% |
| 35 | 200811ns | -95.1% | -35.2% |
| 36 | 223358ns | -95.6% | -44.6% |
| 37 | 234678ns | -95.9% | -47.4% |
| 38 | 227043ns | -95.7% | -42.0% |
| 39 | 204820ns | -95.1% | -36.8% |
| 40 | 198148ns | -94.7% | -32.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-aligned | 0.102 | ok |
| bitpack-write-dense | 0.533 | HIGH+ (drift/warm-up) |
| bitpack-write-windowed | 0.775 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **bitpack-write-dense**: won 40/40, lost 0/40
- **bitpack-write-windowed**: won 38/40, lost 2/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-aligned | 16.8ns | 225190.2ns | 0.0% |  |
| bitpack-write-dense | 5.3ns | 10204.0ns | 0.1% |  |
| bitpack-write-windowed | 19.0ns | 146597.2ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-aligned (n=40, range 201187.5-275260.3 ns)
  201187.5 |#################################
  204891.1 |#############
  208594.8 |####################
  212298.4 |########################################
  216002.1 |####################
  219705.7 |####################
  223409.3 |#############
  227113.0 |####################
  230816.6 |#############
  234520.3 |######
  238223.9 |
  241927.5 |#############
  245631.2 |
  249334.8 |######
  253038.5 |
  256742.1 |
  260445.8 |
  264149.4 |
  267853.0 |
  271556.7 |
  (5 below, 2 above range)

bitpack-write-dense (n=40, range 8853.7-11991.0 ns)
   8853.7 |
   9010.6 |
   9167.4 |
   9324.3 |###
   9481.2 |##########
   9638.0 |########################################
   9794.9 |###
   9951.8 |###
  10108.6 |
  10265.5 |###
  10422.4 |#############
  10579.2 |
  10736.1 |###
  10893.0 |
  11049.8 |
  11206.7 |################
  11363.6 |###
  11520.4 |###
  11677.3 |###
  11834.2 |
  (6 below, 2 above range)

bitpack-write-windowed (n=40, range 116926.0-214502.0 ns)
  116926.0 |########################################
  121804.8 |###############
  126683.6 |###################################
  131562.4 |########################################
  136441.2 |#####
  141320.0 |
  146198.8 |
  151077.6 |
  155956.4 |
  160835.2 |
  165714.0 |
  170592.8 |
  175471.6 |#####
  180350.4 |#####
  185229.2 |
  190108.0 |###############
  194986.8 |
  199865.6 |
  204744.4 |#####
  209623.2 |#####
  (3 below, 3 above range)

```

## Diagnostics

- **bitpack-write-dense**: autocorrelation=0.53 (measurement drift or warm-up artifact)
- **bitpack-write-windowed**: CV=25.6% (high variance, measurements may be unstable)
- **bitpack-write-windowed**: autocorrelation=0.78 (measurement drift or warm-up artifact)
