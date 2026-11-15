# Elementwise clamping chain of four steps, width swept: what the doubled container costs when no fold accumulator is involved

6 variants, 40 samples per variant.
Baseline: **warm-clamp-acc64**

## Highlights

Baseline for all deltas below: **warm-clamp-acc64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (warm-clamp-head, warm-clamp-minimum) are a dead heat (<1%)

warm-clamp-head (303 ns) and warm-clamp-minimum (303 ns) differ by 0.07%, inside the noise, even though the wider field spreads 12.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Speed leader warm-clamp-head vs stability leader warm-clamp-minimum (+0% speed for 1.1x steadier)

warm-clamp-head is fastest (303 ns, CV 6.2%); warm-clamp-minimum gives up 0.1% median for 1.1x lower variance (CV 5.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-clamp-acc64 is inconsistent: worst-20% is 2.0x its best-20%

warm-clamp-acc64's best 20% of batches run at 278 ns but its worst 20% at 567 ns (2.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-clamp-head** at 303.2 ns median (-8.4% vs baseline)
- Spread: 1.13x (fastest 303.2 ns, slowest 342.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 655ns | 636ns | 484ns | 611ns | 960ns | base |
| warm-clamp-accfit | 677ns | 591ns | 587ns | 637ns | 884ns | +3.24% |
| warm-clamp-accfit-dyn | 615ns | 534ns | 530ns | 587ns | 784ns | -6.18% |
| warm-clamp-head | 593ns | 532ns | 527ns | 572ns | 722ns | -9.46% |
| warm-clamp-min-lanes | 630ns | 604ns | 557ns | 622ns | 726ns | -3.89% |
| warm-clamp-minimum | 558ns | 530ns | 527ns | 544ns | 631ns | -14.91% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-clamp-acc64 | 376ns | 278ns | 567ns | base | 21.788 |
| warm-clamp-accfit | 378ns | 336ns | 492ns | +0.56% | 21.666 |
| warm-clamp-accfit-dyn | 344ns | 304ns | 443ns | -8.57% | 23.830 |
| warm-clamp-head | 316ns | 303ns | 347ns | -15.90% | 25.906 |
| warm-clamp-min-lanes | 356ns | 319ns | 392ns | -5.29% | 23.005 |
| warm-clamp-minimum | 314ns | 303ns | 344ns | -16.58% | 26.117 |

## Performance model

- Peak throughput: **29.415 Gops/s** (warm-clamp-acc64; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-clamp-acc64 | 24.742 | 84.1% |
| warm-clamp-accfit | 24.283 | 82.6% |
| warm-clamp-accfit-dyn | 26.890 | 91.4% |
| warm-clamp-head | 27.018 | 91.9% |
| warm-clamp-min-lanes | 23.936 | 81.4% |
| warm-clamp-minimum | 27.001 | 91.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-clamp-acc64 | 655ns | 655ns | base |
| warm-clamp-accfit | 677ns | 677ns | +3.24% |
| warm-clamp-accfit-dyn | 615ns | 615ns | -6.18% |
| warm-clamp-head | 593ns | 593ns | -9.46% |
| warm-clamp-min-lanes | 630ns | 630ns | -3.89% |
| warm-clamp-minimum | 558ns | 558ns | -14.91% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-clamp-acc64 | 331ns | base | --- | [279, 410] | --- | --- | --- | --- |
| warm-clamp-accfit | 337ns | no significant difference | [-7, +58]ns | [337, 378] | no | 1.0000 | 0.2682 | 0 |
| warm-clamp-accfit-dyn | 305ns | no significant difference | [-90, +26]ns | [304, 336] | no | 1.0000 | 1.0000 | 0 |
| warm-clamp-head | 303ns | no significant difference | [-86, +24]ns | [303, 309] | no | 1.0000 | 1.0000 | 0 |
| warm-clamp-min-lanes | 342ns | no significant difference | [-52, +59]ns | [337, 379] | no | 1.0000 | 0.8746 | 0 |
| warm-clamp-minimum | 303ns | no significant difference | [-107, +24]ns | [303, 304] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-clamp-acc64 | warm-clamp-accfit | warm-clamp-accfit-dyn | warm-clamp-head | warm-clamp-min-lanes | warm-clamp-minimum |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 279ns | +20.7% | +9.1% | +8.6% | +35.8% | +8.6% |
| 1 | 2 | 384ns | -12.3% | -17.0% | -21.0% | -1.1% | -21.0% |
| 1 | 3 | 464ns | +8.5% | -34.5% | -25.5% | -28.3% | -34.7% |
| 1 | 4 | 279ns | +31.0% | +56.0% | +8.8% | +21.1% | +20.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-clamp-acc64 | -0.284 | moderate- |
| warm-clamp-accfit | -0.304 | moderate- |
| warm-clamp-accfit-dyn | -0.155 | ok |
| warm-clamp-head | -0.419 | moderate- |
| warm-clamp-min-lanes | 0.219 | moderate+ |
| warm-clamp-minimum | -0.080 | ok |

**Consistency summary:**

- **warm-clamp-accfit**: won 1/4, lost 3/4
- **warm-clamp-accfit-dyn**: won 2/4, lost 2/4
- **warm-clamp-head**: won 2/4, lost 2/4
- **warm-clamp-min-lanes**: won 2/4, lost 2/4
- **warm-clamp-minimum**: won 2/4, lost 2/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-clamp-acc64 | 104.4ns | 376.0ns | 27.8% | HIGH |
| warm-clamp-accfit | 112.0ns | 378.1ns | 29.6% | HIGH |
| warm-clamp-accfit-dyn | 97.0ns | 343.8ns | 28.2% | HIGH |
| warm-clamp-head | 97.1ns | 316.2ns | 30.7% | HIGH |
| warm-clamp-min-lanes | 108.6ns | 356.1ns | 30.5% | HIGH |
| warm-clamp-minimum | 90.2ns | 313.7ns | 28.8% | HIGH |

## Distribution (algo ns)

```
warm-clamp-acc64 (n=40, range 278.5-566.6 ns)
    278.5 |########################################
    292.9 |
    307.3 |
    321.7 |
    336.1 |
    350.5 |
    364.9 |
    379.3 |##############
    393.7 |
    408.2 |
    422.6 |##
    437.0 |################
    451.4 |
    465.8 |
    480.2 |##
    494.6 |##
    509.0 |####
    523.4 |
    537.8 |##
    552.2 |
  (3 below, 1 above range)

warm-clamp-accfit (n=40, range 335.8-491.9 ns)
    335.8 |########################################
    343.6 |
    351.4 |##
    359.2 |
    367.0 |
    374.9 |##########
    382.7 |
    390.5 |
    398.3 |
    406.1 |
    413.9 |
    421.7 |
    429.5 |######
    437.3 |
    445.1 |
    452.9 |
    460.7 |
    468.5 |##
    476.3 |
    484.1 |
  (4 below, 6 above range)

warm-clamp-accfit-dyn (n=40, range 303.9-443.2 ns)
    303.9 |########################################
    310.9 |
    317.8 |
    324.8 |
    331.7 |#######
    338.7 |
    345.7 |
    352.6 |
    359.6 |
    366.6 |
    373.5 |#
    380.5 |
    387.5 |
    394.4 |
    401.4 |
    408.3 |
    415.3 |
    422.3 |
    429.2 |###############
    436.2 |#
  (4 below, 1 above range)

warm-clamp-head (n=40, range 302.9-347.2 ns)
    302.9 |########################################
    305.1 |
    307.4 |
    309.6 |
    311.8 |
    314.0 |#
    316.2 |
    318.4 |
    320.6 |
    322.9 |#
    325.1 |
    327.3 |
    329.5 |#
    331.7 |
    333.9 |
    336.2 |###
    338.4 |
    340.6 |
    342.8 |
    345.0 |##############
  (4 below, 1 above range)

warm-clamp-min-lanes (n=40, range 319.2-392.2 ns)
    319.2 |
    322.8 |
    326.5 |##
    330.1 |
    333.8 |##########################
    337.4 |#############
    341.1 |
    344.7 |##
    348.4 |
    352.0 |
    355.7 |
    359.3 |
    363.0 |
    366.6 |##
    370.3 |
    373.9 |##
    377.6 |########################################
    381.2 |
    384.9 |
    388.5 |
  (4 below, 2 above range)

warm-clamp-minimum (n=40, range 302.9-343.8 ns)
    302.9 |########################################
    305.0 |
    307.0 |
    309.1 |
    311.1 |
    313.1 |
    315.2 |
    317.2 |
    319.3 |
    321.3 |
    323.4 |
    325.4 |
    327.4 |#
    329.5 |
    331.5 |
    333.6 |
    335.6 |###########
    337.7 |#
    339.7 |
    341.8 |
  (5 below, 2 above range)

```

## Diagnostics

- **warm-clamp-acc64**: CV=38.2% (high variance, measurements may be unstable)
- **warm-clamp-acc64**: bridge=32.9% of algo (FFI overhead may distort results)
- **warm-clamp-accfit**: bridge=29.1% of algo (FFI overhead may distort results)
- **warm-clamp-accfit-dyn**: bridge=28.6% of algo (FFI overhead may distort results)
- **warm-clamp-head**: bridge=29.1% of algo (FFI overhead may distort results)
- **warm-clamp-min-lanes**: bridge=28.8% of algo (FFI overhead may distort results)
- **warm-clamp-minimum**: bridge=29.0% of algo (FFI overhead may distort results)
