# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary deliberately misaligned

2 variants, 40 samples per variant.
Baseline: **bitpack-write-dense**

## Highlights

Baseline for all deltas below: **bitpack-write-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-write-dense dominates: 693% faster than the next best (bitpack-write-guarded)

bitpack-write-dense (458.60 us) leads bitpack-write-guarded (3.64 ms) by 693%, a clear separation rather than a photo finish. CV 9.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-dense is fastest but the noisiest (CV 9.7%)

bitpack-write-dense wins on median (458.60 us) yet has the highest variance (CV 9.7%), while bitpack-write-guarded is the steadiest (CV 0.3%, 3.64 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bitpack-write-dense shows alternating (throttle bounce) (autocorr -0.79)

bitpack-write-dense's per-pass series has lag-1 autocorrelation -0.79, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (bitpack-write-dense)

The baseline bitpack-write-dense is the fastest (458.60 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 7.9x the fastest

Fastest bitpack-write-dense (458.60 us) to slowest bitpack-write-guarded (3.64 ms): 7.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (bitpack-write-dense) is the fastest** at 458601.6 ns median
- 1 variant significantly slower than baseline
- Spread: 7.93x (fastest 458601.6 ns, slowest 3638841.8 ns)

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
| bitpack-write-dense | 434053ns | 458962ns | 387025ns | 432139ns | 486824ns | base |
| bitpack-write-guarded | 3642339ns | 3639169ns | 3635424ns | 3639016ns | 3659224ns | +739.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-dense | 433758ns | 386768ns | 486544ns | base | 4.835 |
| bitpack-write-guarded | 3641965ns | 3635015ns | 3658852ns | +739.63% | 0.576 |

## Performance model

- Peak throughput: **5.422 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 2097150

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-dense | 4.573 | 84.3% |
| bitpack-write-guarded | 0.576 | 10.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-dense | 434053ns | 434053ns | base |
| bitpack-write-guarded | 3642339ns | 3642339ns | +739.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-dense | 458602ns | base | --- | [388826, 464303] | --- | --- | --- | --- |
| bitpack-write-guarded | 3638842ns | +3190186.0ns (+695.6%) | [+3174842, +3247780]ns | [3637325, 3639389] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-write-dense | bitpack-write-guarded |
|---|---|---|---|
| 1 | 1 | 388516ns | +836.1% |
| 1 | 2 | 486085ns | +648.6% |
| 1 | 3 | 386988ns | +839.9% |
| 1 | 4 | 464255ns | +683.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-dense | -0.791 | HIGH- (thermal bounce) |
| bitpack-write-guarded | -0.652 | HIGH- (thermal bounce) |

**Consistency summary:**

- **bitpack-write-guarded**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-dense | 77.1ns | 433758.4ns | 0.0% |  |
| bitpack-write-guarded | 65.4ns | 3641964.8ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-dense (n=40, range 386768.1-486544.4 ns)
  386768.1 |########################################
  391756.9 |
  396745.7 |
  401734.5 |
  406723.4 |
  411712.2 |
  416701.0 |
  421689.8 |
  426678.6 |
  431667.4 |
  436656.3 |
  441645.1 |
  446633.9 |
  451622.7 |##
  456611.5 |
  461600.3 |#########################
  466589.2 |
  471578.0 |
  476566.8 |
  481555.6 |############################
  (5 below, 1 above range)

bitpack-write-guarded (n=40, range 3635015.4-3658852.3 ns)
  3635015.4 |##############################
  3636207.2 |#########################
  3637399.1 |#########################
  3638590.9 |########################################
  3639782.8 |##########
  3640974.6 |#####
  3642166.4 |##########
  3643358.3 |
  3644550.1 |
  3645742.0 |##########
  3646933.8 |
  3648125.7 |#####
  3649317.5 |
  3650509.4 |
  3651701.2 |
  3652893.1 |
  3654084.9 |#####
  3655276.8 |
  3656468.6 |#####
  3657660.4 |
  (3 below, 3 above range)

```
