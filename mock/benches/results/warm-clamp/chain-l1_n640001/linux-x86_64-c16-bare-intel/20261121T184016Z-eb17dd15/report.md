# Elementwise clamping chain of four steps, width swept: what the doubled container costs when no fold accumulator is involved

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-clamp-acc64) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-clamp-acc64 has the worst median (362 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-clamp-minimum at 293 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-clamp-minimum beats baseline by 25% (significant)

warm-clamp-minimum is -91 ns (25%) faster than baseline warm-clamp-acc64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-clamp-minimum shows alternating (throttle bounce) (autocorr -0.77)

warm-clamp-minimum's per-pass series has lag-1 autocorrelation -0.77, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### warm-clamp-accfit-dyn is inconsistent: worst-20% is 1.6x its best-20%

warm-clamp-accfit-dyn's best 20% of batches run at 253 ns but its worst 20% at 394 ns (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-minimum** at 292.8 ns median (-19.0% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 1.24x (fastest 292.8 ns, slowest 361.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Elementwise clamping chain of four steps, width swept: what the doubled container costs when no fold accumulator is involved |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112832026 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 657ns | 665ns | 534ns | 647ns | 812ns | base |
| warm-clamp-accfit | 513ns | 529ns | 421ns | 515ns | 597ns | -22.02% |
| warm-clamp-accfit-dyn | 585ns | 588ns | 440ns | 566ns | 787ns | -10.94% |
| warm-clamp-head | 609ns | 630ns | 485ns | 597ns | 770ns | -7.30% |
| warm-clamp-min-lanes | 544ns | 532ns | 408ns | 550ns | 663ns | -17.23% |
| warm-clamp-minimum | 528ns | 531ns | 462ns | 519ns | 618ns | -19.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 368ns | 307ns | 439ns | base | 22.288 |
| warm-clamp-accfit | 290ns | 241ns | 327ns | -21.22% | 28.290 |
| warm-clamp-accfit-dyn | 315ns | 253ns | 394ns | -14.20% | 25.977 |
| warm-clamp-head | 341ns | 278ns | 420ns | -7.28% | 24.038 |
| warm-clamp-min-lanes | 305ns | 234ns | 357ns | -17.10% | 26.885 |
| warm-clamp-minimum | 295ns | 266ns | 328ns | -19.84% | 27.802 |

## Performance model

- Peak throughput: **35.014 Gops/s** (warm-clamp-min-lanes; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 22.649 | 64.7% |
| warm-clamp-accfit | 27.059 | 77.3% |
| warm-clamp-accfit-dyn | 26.714 | 76.3% |
| warm-clamp-head | 25.280 | 72.2% |
| warm-clamp-min-lanes | 26.930 | 76.9% |
| warm-clamp-minimum | 27.978 | 79.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 657ns | 657ns | base |
| warm-clamp-accfit | 513ns | 513ns | -22.02% |
| warm-clamp-accfit-dyn | 585ns | 585ns | -10.94% |
| warm-clamp-head | 609ns | 609ns | -7.30% |
| warm-clamp-min-lanes | 544ns | 544ns | -17.23% |
| warm-clamp-minimum | 528ns | 528ns | -19.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 362ns | base | --- | [341, 384] | --- | --- | --- | --- |
| warm-clamp-accfit | 303ns | -65.8ns (-18.2%) | [-134, -42]ns | [276, 303] | YES | 0.0000 | 0.0000 | 0 |
| warm-clamp-accfit-dyn | 307ns | -54.3ns (-15.0%) | [-92, -54]ns | [275, 346] | YES | 0.0002 | 0.0002 | 0 |
| warm-clamp-head | 324ns | no significant difference | [-27, +1]ns | [307, 383] | no | 0.8746 | 0.8746 | 0 |
| warm-clamp-min-lanes | 304ns | -80.2ns (-22.2%) | [-101, -78]ns | [304, 338] | YES | 0.0001 | 0.0000 | 0 |
| warm-clamp-minimum | 293ns | -91.0ns (-25.1%) | [-103, -63]ns | [279, 307] | YES | 0.0000 | 0.0000 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 341ns | -19.2% | +11.1% | +23.5% | -31.4% | -19.0% |
| 1 | 2 | 439ns | -30.9% | -16.6% | -36.5% | -23.0% | -26.3% |
| 1 | 3 | 384ns | -35.7% | -28.2% | -0.1% | -20.7% | -27.3% |
| 1 | 4 | 307ns | -1.2% | -17.7% | +0.1% | +5.9% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.177 | ok |
| warm-clamp-accfit | -0.746 | HIGH- (thermal bounce) |
| warm-clamp-accfit-dyn | 0.303 | moderate+ |
| warm-clamp-head | -0.687 | HIGH- (thermal bounce) |
| warm-clamp-min-lanes | -0.351 | moderate- |
| warm-clamp-minimum | -0.773 | HIGH- (thermal bounce) |

**Consistency summary:**

- **warm-clamp-accfit**: won 4/4, lost 0/4
- **warm-clamp-accfit-dyn**: won 3/4, lost 1/4
- **warm-clamp-head**: won 1/4, lost 1/4
- **warm-clamp-min-lanes**: won 3/4, lost 1/4
- **warm-clamp-minimum**: won 3/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 107.4ns | 367.6ns | 29.2% | HIGH |
| warm-clamp-accfit | 80.7ns | 289.6ns | 27.9% | HIGH |
| warm-clamp-accfit-dyn | 96.6ns | 315.4ns | 30.6% | HIGH |
| warm-clamp-head | 98.1ns | 340.8ns | 28.8% | HIGH |
| warm-clamp-min-lanes | 88.0ns | 304.7ns | 28.9% | HIGH |
| warm-clamp-minimum | 82.7ns | 294.7ns | 28.1% | HIGH |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 306.7-439.1 ns)
    306.7 |########################
    313.3 |
    320.0 |
    326.6 |
    333.2 |
    339.8 |########################################
    346.4 |
    353.1 |
    359.7 |
    366.3 |
    372.9 |
    379.5 |########################################
    386.2 |
    392.8 |
    399.4 |
    406.0 |
    412.6 |
    419.3 |
    425.9 |
    432.5 |####################
  (4 below, 5 above range)

warm-clamp-accfit (n=40, range 241.3-326.6 ns)
    241.3 |
    245.6 |
    249.9 |#######
    254.1 |
    258.4 |
    262.6 |
    266.9 |
    271.2 |#######
    275.4 |###
    279.7 |
    283.9 |#
    288.2 |
    292.5 |
    296.7 |
    301.0 |########################################
    305.2 |
    309.5 |#
    313.8 |
    318.0 |
    322.3 |
  (5 below, 2 above range)

warm-clamp-accfit-dyn (n=40, range 252.6-393.7 ns)
    252.6 |########################################
    259.6 |
    266.7 |####
    273.7 |############################
    280.8 |
    287.9 |
    294.9 |
    302.0 |
    309.0 |
    316.1 |
    323.1 |
    330.2 |############
    337.2 |
    344.3 |####################
    351.4 |####
    358.4 |
    365.5 |
    372.5 |########################
    379.6 |################
    386.6 |
  (2 below, 1 above range)

warm-clamp-head (n=40, range 278.3-420.1 ns)
    278.3 |####################################
    285.4 |###
    292.5 |
    299.6 |
    306.7 |#########################
    313.8 |
    320.8 |
    327.9 |
    335.0 |#######
    342.1 |
    349.2 |
    356.3 |###
    363.4 |
    370.5 |
    377.6 |########################################
    384.7 |
    391.8 |
    398.9 |###
    406.0 |
    413.0 |
  (2 below, 5 above range)

warm-clamp-min-lanes (n=40, range 234.0-357.2 ns)
    234.0 |###############
    240.1 |
    246.3 |
    252.5 |###
    258.6 |
    264.8 |
    270.9 |
    277.1 |
    283.3 |
    289.4 |
    295.6 |
    301.8 |########################################
    307.9 |###
    314.1 |
    320.2 |
    326.4 |
    332.6 |####################################
    338.7 |###
    344.9 |
    351.1 |
  (4 below, 3 above range)

warm-clamp-minimum (n=40, range 266.4-328.3 ns)
    266.4 |
    269.5 |
    272.6 |####
    275.6 |##########################
    278.8 |########################################
    281.9 |
    284.9 |
    288.0 |
    291.1 |
    294.2 |
    297.3 |
    300.4 |
    303.5 |##########################
    306.6 |########################################
    309.7 |
    312.8 |
    315.9 |
    319.0 |
    322.1 |
    325.2 |
  (4 below, 5 above range)

```

## Diagnostics

- **warm-clamp-acc64**: bridge=29.5% of algo (FFI overhead may distort results)
- **warm-clamp-accfit**: bridge=28.3% of algo (FFI overhead may distort results)
- **warm-clamp-accfit-dyn**: bridge=30.9% of algo (FFI overhead may distort results)
- **warm-clamp-head**: bridge=29.4% of algo (FFI overhead may distort results)
- **warm-clamp-min-lanes**: bridge=28.3% of algo (FFI overhead may distort results)
- **warm-clamp-minimum**: bridge=28.2% of algo (FFI overhead may distort results)
