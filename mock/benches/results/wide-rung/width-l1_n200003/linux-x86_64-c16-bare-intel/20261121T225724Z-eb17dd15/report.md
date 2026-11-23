# Wide rung, payload-shape sweep, cache-resident (2048 elements, 3 ops/element)

5 variants, 40 samples per variant.
Baseline: **wide-rung-align16**

## Highlights

Baseline for all deltas below: **wide-rung-align16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wide-rung-wordround-alias is inconsistent: worst-20% is 11.1x its best-20%

wide-rung-wordround-alias's best 20% of batches run at 5.07 us but its worst 20% at 56.13 us (11.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: wide-rung-ragged** at 5240.2 ns median (-6.2% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 1.20x (fastest 5240.2 ns, slowest 6306.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Wide rung, payload-shape sweep, cache-resident (2048 elements, 3 ops/element) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112636733 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide-rung-align16 | 5669ns | 5686ns | 5014ns | 5593ns | 6552ns | base |
| wide-rung-ragged | 5581ns | 5310ns | 5291ns | 5403ns | 6405ns | -1.54% |
| wide-rung-ragged-overread | 5905ns | 5922ns | 5295ns | 5901ns | 6527ns | +4.17% |
| wide-rung-wordround | 5991ns | 6119ns | 5197ns | 6122ns | 6391ns | +5.68% |
| wide-rung-wordround-alias | 17094ns | 6389ns | 5135ns | 7828ns | 56854ns | +201.55% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide-rung-align16 | 5589ns | 4948ns | 6450ns | base | 1.466 |
| wide-rung-ragged | 5504ns | 5221ns | 6316ns | -1.51% | 1.488 |
| wide-rung-ragged-overread | 5826ns | 5230ns | 6431ns | +4.25% | 1.406 |
| wide-rung-wordround | 5907ns | 5129ns | 6309ns | +5.70% | 1.387 |
| wide-rung-wordround-alias | 16870ns | 5068ns | 56134ns | +201.87% | 0.486 |

## Performance model

- Peak throughput: **1.656 Gops/s** (wide-rung-align16; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide-rung-align16 | 1.466 | 88.5% |
| wide-rung-ragged | 1.563 | 94.4% |
| wide-rung-ragged-overread | 1.406 | 84.9% |
| wide-rung-wordround | 1.357 | 82.0% |
| wide-rung-wordround-alias | 1.299 | 78.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide-rung-align16 | 5669ns | 5669ns | base |
| wide-rung-ragged | 5581ns | 5581ns | -1.54% |
| wide-rung-ragged-overread | 5905ns | 5905ns | +4.17% |
| wide-rung-wordround | 5991ns | 5991ns | +5.68% |
| wide-rung-wordround-alias | 17094ns | 17094ns | +201.55% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide-rung-align16 | 5588ns | base | --- | [4958, 6026] | --- | --- | --- | --- |
| wide-rung-ragged | 5240ns | +257.6ns (+4.6%) | [+37, +270]ns | [5234, 5245] | YES | 0.0221 | 0.0166 | 0 |
| wide-rung-ragged-overread | 5828ns | +293.7ns (+5.3%) | [+277, +324]ns | [5247, 6402] | YES | 0.0000 | 0.0000 | 0 |
| wide-rung-wordround | 6036ns | +214.9ns (+3.8%) | [+194, +284]ns | [6025, 6213] | YES | 0.0221 | 0.0166 | 0 |
| wide-rung-wordround-alias | 6306ns | no significant difference | [-6, +1360]ns | [6239, 6313] | no | 0.0807 | 0.0807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide-rung-align16 | wide-rung-ragged | wide-rung-ragged-overread | wide-rung-wordround | wide-rung-wordround-alias |
|---|---|---|---|---|---|---|
| 1 | 1 | 6025ns | -13.0% | +6.4% | +4.6% | -14.5% |
| 1 | 2 | 6168ns | +2.4% | +3.9% | -2.2% | +1.2% |
| 1 | 3 | 4954ns | +5.5% | +5.9% | +4.1% | +27.3% |
| 1 | 4 | 4957ns | +5.6% | +5.5% | +23.1% | +1047.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide-rung-align16 | 0.212 | moderate+ |
| wide-rung-ragged | -0.417 | moderate- |
| wide-rung-ragged-overread | 0.253 | moderate+ |
| wide-rung-wordround | -0.254 | moderate- |
| wide-rung-wordround-alias | -0.078 | ok |

**Consistency summary:**

- **wide-rung-ragged**: won 1/4, lost 3/4
- **wide-rung-ragged-overread**: won 0/4, lost 4/4
- **wide-rung-wordround**: won 1/4, lost 3/4
- **wide-rung-wordround-alias**: won 1/4, lost 3/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide-rung-align16 | 22.0ns | 5588.6ns | 0.4% |  |
| wide-rung-ragged | 21.1ns | 5504.2ns | 0.4% |  |
| wide-rung-ragged-overread | 21.8ns | 5826.3ns | 0.4% |  |
| wide-rung-wordround | 22.9ns | 5907.3ns | 0.4% |  |
| wide-rung-wordround-alias | 64.5ns | 16870.2ns | 0.4% |  |

## Distribution (algo ns)

```
wide-rung-align16 (n=40, range 4948.1-6450.4 ns)
   4948.1 |##################################
   5023.3 |##
   5098.4 |#####
   5173.5 |
   5248.6 |
   5323.7 |
   5398.8 |
   5473.9 |
   5549.1 |
   5624.2 |
   5699.3 |
   5774.4 |
   5849.5 |
   5924.6 |
   5999.7 |########################################
   6074.9 |
   6150.0 |
   6225.1 |########
   6300.2 |##
   6375.3 |
  (5 below, 2 above range)

wide-rung-ragged (n=40, range 5221.4-6315.6 ns)
   5221.4 |########################################
   5276.1 |
   5330.8 |
   5385.6 |
   5440.3 |
   5495.0 |
   5549.7 |
   5604.4 |
   5659.1 |
   5713.8 |
   5768.5 |
   5823.2 |
   5877.9 |
   5932.7 |
   5987.4 |
   6042.1 |
   6096.8 |
   6151.5 |
   6206.2 |
   6260.9 |######
  (5 below, 6 above range)

wide-rung-ragged-overread (n=40, range 5229.7-6430.8 ns)
   5229.7 |########################################
   5289.8 |
   5349.8 |
   5409.9 |
   5470.0 |
   5530.0 |
   5590.1 |
   5650.1 |
   5710.2 |
   5770.2 |
   5830.3 |
   5890.3 |
   5950.4 |
   6010.4 |
   6070.5 |
   6130.5 |
   6190.6 |
   6250.7 |
   6310.7 |
   6370.8 |########################################
  (5 below, 5 above range)

wide-rung-wordround (n=40, range 5129.4-6309.1 ns)
   5129.4 |#####################
   5188.4 |##
   5247.4 |
   5306.4 |
   5365.3 |
   5424.3 |
   5483.3 |
   5542.3 |
   5601.3 |
   5660.3 |
   5719.2 |
   5778.2 |
   5837.2 |
   5896.2 |
   5955.2 |
   6014.2 |########################################
   6073.1 |
   6132.1 |##
   6191.1 |
   6250.1 |#####################
  (1 below, 6 above range)

wide-rung-wordround-alias (n=40, range 5068.3-56133.9 ns)
   5068.3 |########################################
   7621.6 |#
  10174.9 |
  12728.2 |#
  15281.4 |
  17834.7 |
  20388.0 |
  22941.3 |
  25494.6 |
  28047.8 |
  30601.1 |#
  33154.4 |
  35707.7 |
  38260.9 |
  40814.2 |
  43367.5 |
  45920.8 |
  48474.0 |
  51027.3 |###
  53580.6 |
  (3 below, 6 above range)

```

## Diagnostics

- **wide-rung-wordround-alias**: CV=119.2% (high variance, measurements may be unstable)
- **wide-rung-wordround-alias**: worst_20/best_20 = 11.1x (possible bimodal distribution)
