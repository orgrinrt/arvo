# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary period-aligned

3 variants, 40 samples per variant.
Baseline: **bitpack-write-aligned**

## Highlights

Baseline for all deltas below: **bitpack-write-aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-write-dense dominates: 446% faster than the next best (bitpack-write-aligned)

bitpack-write-dense (22.54 us) leads bitpack-write-aligned (123.10 us) by 446%, a clear separation rather than a photo finish. CV 31.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-dense beats baseline by 82% (significant)

bitpack-write-dense is -100.38 us (82%) faster than baseline bitpack-write-aligned, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-write-windowed is an outlier: 5.7x slower than the field

bitpack-write-windowed (127.51 us) is 5.7x the fastest (22.54 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-write-dense is fastest but the noisiest (CV 31.8%)

bitpack-write-dense wins on median (22.54 us) yet has the highest variance (CV 31.8%), while bitpack-write-windowed is the steadiest (CV 12.6%, 127.51 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bitpack-write-dense shows alternating (throttle bounce) (autocorr -0.81)

bitpack-write-dense's per-pass series has lag-1 autocorrelation -0.81, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 5.7x the fastest

Fastest bitpack-write-dense (22.54 us) to slowest bitpack-write-windowed (127.51 us): 5.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### bitpack-write-dense is inconsistent: worst-20% is 2.3x its best-20%

bitpack-write-dense's best 20% of batches run at 13.33 us but its worst 20% at 30.37 us (2.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-write-dense** at 22539.4 ns median (-81.7% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 5.66x (fastest 22539.4 ns, slowest 127512.6 ns)

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
| Counter frequency | 2112601419 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-write-aligned | 129105ns | 123194ns | 116203ns | 122707ns | 161200ns | base |
| bitpack-write-dense | 22408ns | 22651ns | 13406ns | 22722ns | 30466ns | -82.64% |
| bitpack-write-windowed | 119481ns | 127618ns | 97939ns | 120801ns | 137062ns | -7.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-aligned | 129005ns | 116109ns | 161090ns | base | 0.508 |
| bitpack-write-dense | 22314ns | 13326ns | 30367ns | -82.70% | 2.937 |
| bitpack-write-windowed | 119374ns | 97843ns | 136966ns | -7.47% | 0.549 |

## Performance model

- Peak throughput: **4.918 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-aligned | 0.532 | 10.8% |
| bitpack-write-dense | 2.908 | 59.1% |
| bitpack-write-windowed | 0.514 | 10.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-aligned | 129105ns | 129105ns | base |
| bitpack-write-dense | 22408ns | 22408ns | -82.64% |
| bitpack-write-windowed | 119481ns | 119481ns | -7.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-aligned | 123096ns | base | --- | [118014, 124579] | --- | --- | --- | --- |
| bitpack-write-dense | 22539ns | -100380.1ns (-81.5%) | [-107411, -93399]ns | [16960, 28456] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-write-windowed | 127513ns | no significant difference | [-17610, +5571]ns | [109005, 133528] | no | 0.2682 | 0.2682 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-write-aligned | bitpack-write-dense | bitpack-write-windowed |
|---|---|---|---|---|
| 1 | 1 | 116404ns | -75.6% | -15.8% |
| 1 | 2 | 124579ns | -89.3% | -12.5% |
| 1 | 3 | 123298ns | -75.4% | +7.4% |
| 1 | 4 | 124756ns | -86.4% | +9.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-aligned | -0.183 | ok |
| bitpack-write-dense | -0.805 | HIGH- (thermal bounce) |
| bitpack-write-windowed | 0.302 | moderate+ |

**Consistency summary:**

- **bitpack-write-dense**: won 4/4, lost 0/4
- **bitpack-write-windowed**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-aligned | 24.5ns | 129004.6ns | 0.0% |  |
| bitpack-write-dense | 23.9ns | 22313.9ns | 0.1% |  |
| bitpack-write-windowed | 25.7ns | 119374.2ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-aligned (n=40, range 116109.1-161090.3 ns)
  116109.1 |########################################
  118358.2 |
  120607.2 |################
  122856.3 |####################################
  125105.4 |####
  127354.4 |####
  129603.5 |########
  131852.5 |
  134101.6 |####
  136350.7 |
  138599.7 |
  140848.8 |
  143097.8 |
  145346.9 |
  147596.0 |
  149845.0 |
  152094.1 |
  154343.1 |
  156592.2 |
  158841.3 |
  (5 below, 7 above range)

bitpack-write-dense (n=40, range 13325.7-30366.9 ns)
  13325.7 |################
  14177.8 |########
  15029.8 |
  15881.9 |
  16733.9 |########################################
  17586.0 |
  18438.1 |
  19290.1 |
  20142.2 |
  20994.2 |
  21846.3 |
  22698.4 |
  23550.4 |
  24402.5 |
  25254.6 |
  26106.6 |
  26958.7 |
  27810.8 |####################################
  28662.8 |####
  29514.9 |####################################
  (4 below, 1 above range)

bitpack-write-windowed (n=40, range 97842.6-136966.2 ns)
  97842.6 |####################
  99798.8 |
  101754.9 |
  103711.1 |
  105667.3 |###
  107623.5 |##########################
  109579.7 |
  111535.8 |
  113492.0 |
  115448.2 |
  117404.4 |
  119360.6 |
  121316.7 |
  123272.9 |
  125229.1 |###
  127185.3 |
  129141.5 |################
  131097.6 |###
  133053.8 |###
  135010.0 |########################################
  (4 below, 1 above range)

```

## Diagnostics

- **bitpack-write-dense**: CV=32.1% (high variance, measurements may be unstable)
