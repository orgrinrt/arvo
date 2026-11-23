# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary deliberately misaligned

2 variants, 40 samples per variant.
Baseline: **bitpack-write-dense**

## Highlights

Baseline for all deltas below: **bitpack-write-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-write-dense dominates: 960% faster than the next best (bitpack-write-guarded)

bitpack-write-dense (17.49 us) leads bitpack-write-guarded (185.32 us) by 960%, a clear separation rather than a photo finish. CV 19.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-dense is fastest but the noisiest (CV 19.1%)

bitpack-write-dense wins on median (17.49 us) yet has the highest variance (CV 19.1%), while bitpack-write-guarded is the steadiest (CV 4.1%, 185.32 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bitpack-write-dense shows alternating (throttle bounce) (autocorr -0.60)

bitpack-write-dense's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (bitpack-write-dense)

The baseline bitpack-write-dense is the fastest (17.49 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 10.6x the fastest

Fastest bitpack-write-dense (17.49 us) to slowest bitpack-write-guarded (185.32 us): 10.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-write-dense is inconsistent: worst-20% is 1.6x its best-20%

bitpack-write-dense's best 20% of batches run at 12.34 us but its worst 20% at 20.29 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (bitpack-write-dense) is the fastest** at 17490.2 ns median
- 1 variant significantly slower than baseline
- Spread: 10.60x (fastest 17490.2 ns, slowest 185323.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary deliberately misaligned |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef2 |
| Counter frequency | 2112756716 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-write-dense | 16999ns | 17568ns | 12420ns | 17401ns | 20371ns | base |
| bitpack-write-guarded | 181780ns | 185401ns | 168518ns | 184282ns | 187538ns | +969.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-dense | 16919ns | 12335ns | 20288ns | base | 3.873 |
| bitpack-write-guarded | 181692ns | 168433ns | 187444ns | +973.89% | 0.361 |

## Performance model

- Peak throughput: **5.313 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 65534

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-dense | 3.747 | 70.5% |
| bitpack-write-guarded | 0.354 | 6.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-dense | 16999ns | 16999ns | base |
| bitpack-write-guarded | 181780ns | 181780ns | +969.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-dense | 17490ns | base | --- | [15035, 20022] | --- | --- | --- | --- |
| bitpack-write-guarded | 185323ns | +169965.8ns (+971.8%) | [+165807, +170244]ns | [185069, 185825] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-write-dense | bitpack-write-guarded |
|---|---|---|---|
| 1 | 1 | 12339ns | +1409.4% |
| 1 | 2 | 20196ns | +734.7% |
| 1 | 3 | 15046ns | +1131.5% |
| 1 | 4 | 20084ns | +825.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-dense | -0.598 | HIGH- (thermal bounce) |
| bitpack-write-guarded | -0.423 | moderate- |

**Consistency summary:**

- **bitpack-write-guarded**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-dense | 20.2ns | 16919.2ns | 0.1% |  |
| bitpack-write-guarded | 20.7ns | 181692.4ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-dense (n=40, range 12335.4-20288.3 ns)
  12335.4 |################
  12733.1 |
  13130.7 |
  13528.4 |
  13926.0 |
  14323.7 |
  14721.3 |########################
  15118.9 |##
  15516.6 |
  15914.2 |
  16311.9 |
  16709.5 |
  17107.1 |
  17504.8 |
  17902.4 |
  18300.1 |
  18697.7 |
  19095.4 |
  19493.0 |########
  19890.6 |########################################
  (4 below, 2 above range)

bitpack-write-guarded (n=40, range 168433.4-187444.3 ns)
  168433.4 |#############
  169383.9 |##
  170334.5 |
  171285.0 |
  172235.5 |
  173186.1 |
  174136.6 |
  175087.2 |
  176037.7 |
  176988.3 |
  177938.8 |
  178889.4 |
  179839.9 |
  180790.5 |
  181741.0 |
  182691.6 |
  183642.1 |
  184592.6 |################################
  185543.2 |########################################
  186493.7 |
  (4 below, 3 above range)

```
