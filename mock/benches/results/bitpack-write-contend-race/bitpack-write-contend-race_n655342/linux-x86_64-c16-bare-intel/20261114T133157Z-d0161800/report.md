# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary deliberately misaligned

2 variants, 40 samples per variant.
Baseline: **bitpack-write-dense**

## Highlights

Baseline for all deltas below: **bitpack-write-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-write-dense dominates: 822% faster than the next best (bitpack-write-guarded)

bitpack-write-dense (19.97 us) leads bitpack-write-guarded (184.06 us) by 822%, a clear separation rather than a photo finish. CV 22.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-dense shows alternating (throttle bounce) (autocorr -0.55)

bitpack-write-dense's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (bitpack-write-dense)

The baseline bitpack-write-dense is the fastest (19.97 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 9.2x the fastest

Fastest bitpack-write-dense (19.97 us) to slowest bitpack-write-guarded (184.06 us): 9.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-write-guarded is inconsistent: worst-20% is 2.2x its best-20%

bitpack-write-guarded's best 20% of batches run at 161.92 us but its worst 20% at 350.58 us (2.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (bitpack-write-dense) is the fastest** at 19973.5 ns median
- 1 variant significantly slower than baseline
- Spread: 9.22x (fastest 19973.5 ns, slowest 184056.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-write-dense | 19213ns | 20048ns | 12192ns | 19615ns | 25027ns | base |
| bitpack-write-guarded | 213471ns | 184135ns | 162004ns | 184865ns | 350756ns | +1011.08% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-dense | 19134ns | 12117ns | 24952ns | base | 3.425 |
| bitpack-write-guarded | 213363ns | 161918ns | 350585ns | +1015.08% | 0.307 |

## Performance model

- Peak throughput: **5.408 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 65534

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-dense | 3.281 | 60.7% |
| bitpack-write-guarded | 0.356 | 6.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-dense | 19213ns | 19213ns | base |
| bitpack-write-guarded | 213471ns | 213471ns | +1011.08% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-dense | 19974ns | base | --- | [19734, 20196] | --- | --- | --- | --- |
| bitpack-write-guarded | 184056ns | +159159.5ns (+796.9%) | [+157872, +178702]ns | [177604, 190804] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-write-dense | bitpack-write-guarded |
|---|---|---|---|
| 1 | 1 | 24933ns | +638.2% |
| 1 | 2 | 12118ns | +1504.0% |
| 1 | 3 | 20196ns | +842.6% |
| 1 | 4 | 19758ns | +795.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-dense | -0.555 | HIGH- (thermal bounce) |
| bitpack-write-guarded | -0.143 | ok |

**Consistency summary:**

- **bitpack-write-guarded**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-dense | 21.0ns | 19134.3ns | 0.1% |  |
| bitpack-write-guarded | 26.9ns | 213362.6ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-dense (n=40, range 12117.1-24951.8 ns)
  12117.1 |##############
  12758.8 |
  13400.6 |
  14042.3 |
  14684.0 |
  15325.8 |
  15967.5 |
  16609.3 |
  17251.0 |
  17892.7 |
  18534.5 |
  19176.2 |#################
  19817.9 |########################################
  20459.7 |##
  21101.4 |
  21743.1 |
  22384.9 |
  23026.6 |
  23668.4 |
  24310.1 |##############
  (5 below, 4 above range)

bitpack-write-guarded (n=40, range 161918.0-350584.9 ns)
  161918.0 |
  171351.4 |####################################
  180784.7 |########################################
  190218.1 |#####################
  199651.4 |
  209084.8 |
  218518.1 |#######
  227951.4 |
  237384.8 |
  246818.1 |
  256251.5 |
  265684.8 |
  275118.2 |
  284551.5 |
  293984.8 |
  303418.2 |
  312851.5 |
  322284.9 |###
  331718.2 |
  341151.6 |
  (5 below, 5 above range)

```

## Diagnostics

- **bitpack-write-dense**: CV=23.5% (high variance, measurements may be unstable)
- **bitpack-write-guarded**: CV=37.6% (high variance, measurements may be unstable)
