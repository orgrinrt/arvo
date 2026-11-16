# Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways

6 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-d32 beats baseline by 22% (significant)

bitpack-contend-d32 is -36.87 us (22%) faster than baseline bitpack-contend-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Top two (bitpack-contend-d32, bitpack-contend-packed) are a dead heat (<1%)

bitpack-contend-d32 (129.79 us) and bitpack-contend-packed (130.91 us) differ by 0.86%, inside the noise, even though the wider field spreads 66.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### bitpack-contend-packed shows alternating (throttle bounce) (autocorr -0.76)

bitpack-contend-packed's per-pass series has lag-1 autocorrelation -0.76, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### bitpack-contend-d64 is inconsistent: worst-20% is 1.7x its best-20%

bitpack-contend-d64's best 20% of batches run at 187.60 us but its worst 20% at 328.24 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: bitpack-contend-d32** at 129787.3 ns median (-22.1% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.66x (fastest 129787.3 ns, slowest 216024.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Packed 13-bit against u16, u32 and u64 carriers with one column split 1, 2 and 4 ways |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112749291 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 166304ns | 166746ns | 164104ns | 166763ns | 167127ns | base |
| bitpack-contend-d16-control | 164889ns | 166684ns | 159745ns | 165811ns | 167265ns | -0.85% |
| bitpack-contend-d32 | 129851ns | 129945ns | 129463ns | 129849ns | 130244ns | -21.92% |
| bitpack-contend-d64 | 233966ns | 216510ns | 187880ns | 217735ns | 328744ns | +40.69% |
| bitpack-contend-packed | 130931ns | 131099ns | 128956ns | 130869ns | 133092ns | -21.27% |
| bitpack-contend-packed-simd | 132492ns | 132683ns | 129108ns | 132580ns | 135614ns | -20.33% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 166112ns | 163903ns | 166869ns | base | 6.312 |
| bitpack-contend-d16-control | 164723ns | 159578ns | 167103ns | -0.84% | 6.366 |
| bitpack-contend-d32 | 129675ns | 129305ns | 130055ns | -21.94% | 8.086 |
| bitpack-contend-d64 | 233587ns | 187602ns | 328243ns | +40.62% | 4.489 |
| bitpack-contend-packed | 130727ns | 128785ns | 132777ns | -21.30% | 8.021 |
| bitpack-contend-packed-simd | 132315ns | 128954ns | 135375ns | -20.35% | 7.925 |

## Performance model

- Peak throughput: **8.142 Gops/s** (bitpack-contend-packed; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 6.295 | 77.3% |
| bitpack-contend-d16-control | 6.297 | 77.3% |
| bitpack-contend-d32 | 8.079 | 99.2% |
| bitpack-contend-d64 | 4.854 | 59.6% |
| bitpack-contend-packed | 8.010 | 98.4% |
| bitpack-contend-packed-simd | 7.912 | 97.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 166304ns | 166304ns | base |
| bitpack-contend-d16-control | 164889ns | 164889ns | -0.85% |
| bitpack-contend-d32 | 129851ns | 129851ns | -21.92% |
| bitpack-contend-d64 | 233966ns | 233966ns | +40.69% |
| bitpack-contend-packed | 130931ns | 130931ns | -21.27% |
| bitpack-contend-packed-simd | 132492ns | 132492ns | -20.33% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 166582ns | base | --- | [166503, 166682] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 166521ns | no significant difference | [-182, +138]ns | [166474, 166666] | no | 0.6358 | 0.6358 | 0 |
| bitpack-contend-d32 | 129787ns | -36865.3ns (-22.1%) | [-37053, -36626]ns | [129454, 129884] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-d64 | 216025ns | +49412.2ns (+29.7%) | [+34150, +53659]ns | [200765, 219903] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed | 130909ns | -34212.4ns (-20.5%) | [-37687, -34057]ns | [128890, 132431] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-packed-simd | 132522ns | -34036.8ns (-20.4%) | [-34198, -33952]ns | [132447, 132557] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-d32 | bitpack-contend-d64 | bitpack-contend-packed | bitpack-contend-packed-simd |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 166780ns | -4.3% | -22.5% | +57.7% | -22.7% | -20.4% |
| 1 | 2 | 166699ns | -0.1% | -22.1% | +53.3% | -20.4% | -20.5% |
| 1 | 3 | 166598ns | +0.3% | -22.3% | +22.5% | -22.6% | -22.6% |
| 1 | 4 | 166475ns | +0.0% | -22.0% | +18.3% | -20.5% | -20.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | 0.246 | moderate+ |
| bitpack-contend-d16-control | -0.046 | ok |
| bitpack-contend-d32 | -0.654 | HIGH- (thermal bounce) |
| bitpack-contend-d64 | 0.296 | moderate+ |
| bitpack-contend-packed | -0.763 | HIGH- (thermal bounce) |
| bitpack-contend-packed-simd | -0.355 | moderate- |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 2/4, lost 1/4
- **bitpack-contend-d32**: won 4/4, lost 0/4
- **bitpack-contend-d64**: won 0/4, lost 4/4
- **bitpack-contend-packed**: won 4/4, lost 0/4
- **bitpack-contend-packed-simd**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 28.8ns | 166111.9ns | 0.0% |  |
| bitpack-contend-d16-control | 32.6ns | 164722.8ns | 0.0% |  |
| bitpack-contend-d32 | 29.4ns | 129674.7ns | 0.0% |  |
| bitpack-contend-d64 | 38.3ns | 233587.5ns | 0.0% |  |
| bitpack-contend-packed | 29.1ns | 130727.0ns | 0.0% |  |
| bitpack-contend-packed-simd | 32.7ns | 132315.3ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 163902.9-166868.6 ns)
  163902.9 |
  164051.2 |
  164199.5 |
  164347.8 |
  164496.0 |
  164644.3 |
  164792.6 |
  164940.9 |
  165089.2 |
  165237.5 |
  165385.8 |
  165534.0 |
  165682.3 |
  165830.6 |
  165978.9 |
  166127.2 |
  166275.5 |
  166423.8 |########################################
  166572.1 |######################
  166720.3 |####################
  (3 below, 4 above range)

bitpack-contend-d16-control (n=40, range 159578.4-167103.2 ns)
  159578.4 |###############
  159954.6 |
  160330.9 |
  160707.1 |
  161083.4 |
  161459.6 |
  161835.8 |
  162212.1 |
  162588.3 |
  162964.5 |
  163340.8 |##
  163717.0 |
  164093.2 |
  164469.5 |
  164845.7 |
  165222.0 |
  165598.2 |
  165974.4 |
  166350.7 |########################################
  166726.9 |######################
  (5 below, 3 above range)

bitpack-contend-d32 (n=40, range 129305.3-130055.0 ns)
  129305.3 |####################
  129342.8 |#####
  129380.3 |
  129417.8 |##
  129455.2 |##
  129492.7 |##
  129530.2 |#####
  129567.7 |##
  129605.2 |
  129642.7 |
  129680.1 |
  129717.6 |##
  129755.1 |
  129792.6 |
  129830.1 |##
  129867.5 |########################################
  129905.0 |##
  129942.5 |##
  129980.0 |
  130017.5 |
  (4 below, 3 above range)

bitpack-contend-d64 (n=40, range 187601.8-328243.4 ns)
  187601.8 |
  194633.9 |########################################
  201666.0 |####################
  208698.1 |####
  215730.1 |########################################
  222762.2 |
  229794.3 |
  236826.4 |
  243858.5 |
  250890.5 |
  257922.6 |
  264954.7 |
  271986.8 |
  279018.9 |
  286050.9 |####
  293083.0 |
  300115.1 |####
  307147.2 |################
  314179.3 |
  321211.3 |
  (4 below, 4 above range)

bitpack-contend-packed (n=40, range 128785.3-132776.9 ns)
  128785.3 |########################################
  128984.9 |###
  129184.4 |
  129384.0 |###
  129583.6 |
  129783.2 |
  129982.8 |
  130182.3 |
  130381.9 |
  130581.5 |
  130781.1 |
  130980.7 |
  131180.2 |
  131379.8 |
  131579.4 |
  131779.0 |
  131978.6 |
  132178.2 |###
  132377.7 |####################################
  132577.3 |###############
  (5 below, 2 above range)

bitpack-contend-packed-simd (n=40, range 128954.4-135374.9 ns)
  128954.4 |#######
  129275.4 |
  129596.4 |##
  129917.5 |
  130238.5 |
  130559.5 |
  130880.5 |
  131201.6 |
  131522.6 |
  131843.6 |
  132164.6 |##################
  132485.7 |########################################
  132806.7 |##
  133127.7 |##
  133448.7 |
  133769.8 |
  134090.8 |
  134411.8 |
  134732.8 |##
  135053.9 |
  (5 below, 3 above range)

```

## Diagnostics

- **bitpack-contend-d64**: CV=23.0% (high variance, measurements may be unstable)
