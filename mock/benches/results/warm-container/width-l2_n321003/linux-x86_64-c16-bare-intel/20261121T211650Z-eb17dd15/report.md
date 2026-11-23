# Container fork, declared-width sweep, 1048576 elements (3 ops/element, wrapping)

4 variants, 40 samples per variant.
Baseline: **warm-container-headroom**

## Highlights

Baseline for all deltas below: **warm-container-headroom**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (warm-container-headroom) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline warm-container-headroom has the worst median (281.16 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest warm-container-minimum at 108.87 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### warm-container-minimum beats baseline by 63% (significant)

warm-container-minimum is -177.49 us (63%) faster than baseline warm-container-headroom, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### warm-container-headroom is an outlier: 2.6x slower than the field

warm-container-headroom (281.16 us) is 2.6x the fastest (108.87 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (warm-container-minimum, warm-container-native) are a dead heat (<1%)

warm-container-minimum (108.87 us) and warm-container-native (108.89 us) differ by 0.02%, inside the noise, even though the wider field spreads 158.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### warm-container-minimum shows alternating (throttle bounce) (autocorr -0.57)

warm-container-minimum's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {warm-container-minimum, warm-container-native} vs {warm-container-plusone, warm-container-headroom} (112% apart)

The field splits into a fast tier {warm-container-minimum, warm-container-native} and a slow tier {warm-container-plusone, warm-container-headroom} with a 112% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader warm-container-minimum vs stability leader warm-container-native (+0% speed for 1.5x steadier)

warm-container-minimum is fastest (108.87 us, CV 3.4%); warm-container-native gives up 0.0% median for 1.5x lower variance (CV 2.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### warm-container-plusone is inconsistent: worst-20% is 1.6x its best-20%

warm-container-plusone's best 20% of batches run at 180.88 us but its worst 20% at 289.66 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: warm-container-minimum** at 108869.0 ns median (-61.3% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.58x (fastest 108869.0 ns, slowest 281162.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Container fork, declared-width sweep, 1048576 elements (3 ops/element, wrapping) |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112751190 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| warm-container-headroom | 289399ns | 281450ns | 257574ns | 287147ns | 327978ns | base |
| warm-container-minimum | 107362ns | 109022ns | 102370ns | 107764ns | 111148ns | -62.90% |
| warm-container-native | 107765ns | 109051ns | 103299ns | 108599ns | 109731ns | -62.76% |
| warm-container-plusone | 232386ns | 230980ns | 181038ns | 230304ns | 289978ns | -19.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| warm-container-headroom | 289122ns | 257330ns | 327659ns | base | 14.507 |
| warm-container-minimum | 107198ns | 102197ns | 110972ns | -62.92% | 39.127 |
| warm-container-native | 107583ns | 103125ns | 109507ns | -62.79% | 38.987 |
| warm-container-plusone | 232138ns | 180878ns | 289655ns | -19.71% | 18.068 |

## Performance model

- Peak throughput: **41.041 Gops/s** (warm-container-minimum; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| warm-container-headroom | 14.918 | 36.3% |
| warm-container-minimum | 38.526 | 93.9% |
| warm-container-native | 38.517 | 93.8% |
| warm-container-plusone | 18.179 | 44.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| warm-container-headroom | 289399ns | 289399ns | base |
| warm-container-minimum | 107362ns | 107362ns | -62.90% |
| warm-container-native | 107765ns | 107765ns | -62.76% |
| warm-container-plusone | 232386ns | 232386ns | -19.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| warm-container-headroom | 281163ns | base | --- | [273351, 302899] | --- | --- | --- | --- |
| warm-container-minimum | 108869ns | -177486.9ns (-63.1%) | [-193127, -169769]ns | [104077, 110648] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-native | 108895ns | -174077.2ns (-61.9%) | [-193629, -169450]ns | [108860, 108923] | YES | 0.0000 | 0.0000 | 0 |
| warm-container-plusone | 230719ns | -49926.4ns (-17.8%) | [-78390, -44745]ns | [205378, 258696] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | warm-container-headroom | warm-container-minimum | warm-container-native | warm-container-plusone |
|---|---|---|---|---|---|
| 1 | 1 | 292724ns | -62.8% | -62.8% | -38.2% |
| 1 | 2 | 300732ns | -64.8% | -65.7% | -13.4% |
| 1 | 3 | 284520ns | -61.1% | -61.7% | -9.0% |
| 1 | 4 | 270614ns | -62.2% | -59.8% | -24.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| warm-container-headroom | 0.168 | ok |
| warm-container-minimum | -0.569 | HIGH- (thermal bounce) |
| warm-container-native | -0.415 | moderate- |
| warm-container-plusone | -0.238 | moderate- |

**Consistency summary:**

- **warm-container-minimum**: won 4/4, lost 0/4
- **warm-container-native**: won 4/4, lost 0/4
- **warm-container-plusone**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| warm-container-headroom | 22.4ns | 289122.0ns | 0.0% |  |
| warm-container-minimum | 21.7ns | 107197.6ns | 0.0% |  |
| warm-container-native | 22.1ns | 107582.5ns | 0.0% |  |
| warm-container-plusone | 24.9ns | 232138.4ns | 0.0% |  |

## Distribution (algo ns)

```
warm-container-headroom (n=40, range 257330.5-327659.1 ns)
  257330.5 |##########################
  260846.9 |#############
  264363.3 |######
  267879.8 |######
  271396.2 |####################
  274912.6 |####################
  278429.0 |#############
  281945.5 |######
  285461.9 |
  288978.3 |######
  292494.8 |####################
  296011.2 |
  299527.6 |######
  303044.1 |####################
  306560.5 |#############
  310076.9 |
  313593.4 |
  317109.8 |
  320626.2 |
  324142.6 |########################################
  (4 below, 3 above range)

warm-container-minimum (n=40, range 102197.5-110972.4 ns)
  102197.5 |################################
  102636.2 |###
  103075.0 |###
  103513.7 |
  103952.5 |
  104391.2 |###
  104830.0 |
  105268.7 |
  105707.5 |
  106146.2 |
  106584.9 |
  107023.7 |###
  107462.4 |
  107901.2 |
  108339.9 |
  108778.7 |################################
  109217.4 |
  109656.2 |
  110094.9 |
  110533.7 |########################################
  (3 below, 4 above range)

warm-container-native (n=40, range 103124.5-109506.9 ns)
  103124.5 |############
  103443.7 |
  103762.8 |
  104081.9 |
  104401.0 |
  104720.1 |
  105039.2 |
  105358.4 |
  105677.5 |
  105996.6 |
  106315.7 |
  106634.8 |
  106953.9 |
  107273.1 |
  107592.2 |
  107911.3 |
  108230.4 |
  108549.5 |############
  108868.7 |########################################
  109187.8 |##
  (4 below, 3 above range)

warm-container-plusone (n=40, range 180878.4-289655.0 ns)
  180878.4 |############
  186317.2 |
  191756.1 |
  197194.9 |
  202633.7 |##############################
  208072.5 |
  213511.4 |
  218950.2 |
  224389.0 |
  229827.9 |
  235266.7 |
  240705.5 |
  246144.4 |
  251583.2 |######
  257022.0 |########################################
  262460.9 |###
  267899.7 |
  273338.5 |
  278777.4 |###
  284216.2 |
  (6 below, 3 above range)

```
