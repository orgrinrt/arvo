# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary deliberately misaligned

2 variants, 40 samples per variant.
Baseline: **bitpack-write-dense**

## Highlights

Baseline for all deltas below: **bitpack-write-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-write-dense dominates: 1052% faster than the next best (bitpack-write-guarded)

bitpack-write-dense (431.47 us) leads bitpack-write-guarded (4.97 ms) by 1052%, a clear separation rather than a photo finish. CV 5.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (bitpack-write-dense)

The baseline bitpack-write-dense is the fastest (431.47 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 11.5x the fastest

Fastest bitpack-write-dense (431.47 us) to slowest bitpack-write-guarded (4.97 ms): 11.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (bitpack-write-dense) is the fastest** at 431467.8 ns median
- 1 variant significantly slower than baseline
- Spread: 11.52x (fastest 431467.8 ns, slowest 4972251.1 ns)

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
| Counter frequency | 2112761742 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-write-dense | 421024ns | 431839ns | 377975ns | 428231ns | 442453ns | base |
| bitpack-write-guarded | 4940281ns | 4972837ns | 4380565ns | 4921583ns | 5556091ns | +1073.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-dense | 420651ns | 377592ns | 442107ns | base | 4.985 |
| bitpack-write-guarded | 4939791ns | 4380119ns | 5555568ns | +1074.32% | 0.425 |

## Performance model

- Peak throughput: **5.554 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 2097150

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-dense | 4.861 | 87.5% |
| bitpack-write-guarded | 0.422 | 7.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-dense | 421024ns | 421024ns | base |
| bitpack-write-guarded | 4940281ns | 4940281ns | +1073.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-dense | 431468ns | base | --- | [423053, 434234] | --- | --- | --- | --- |
| bitpack-write-guarded | 4972251ns | +4542441.8ns (+1052.8%) | [+4286380, +4678562]ns | [4709974, 5115921] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-write-dense | bitpack-write-guarded |
|---|---|---|---|
| 1 | 1 | 379950ns | +1051.9% |
| 1 | 2 | 432482ns | +1068.4% |
| 1 | 3 | 440301ns | +1093.6% |
| 1 | 4 | 423284ns | +1025.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-dense | -0.068 | ok |
| bitpack-write-guarded | -0.130 | ok |

**Consistency summary:**

- **bitpack-write-guarded**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-dense | 73.4ns | 420651.1ns | 0.0% |  |
| bitpack-write-guarded | 78.7ns | 4939791.3ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-dense (n=40, range 377592.1-442106.8 ns)
  377592.1 |
  380817.8 |
  384043.6 |#####
  387269.3 |###############
  390495.0 |
  393720.8 |
  396946.5 |
  400172.3 |#####
  403398.0 |
  406623.7 |
  409849.5 |
  413075.2 |
  416300.9 |
  419526.7 |##########
  422752.4 |#########################
  425978.1 |
  429203.9 |#########################
  432429.6 |#########################
  435655.4 |###############
  438881.1 |########################################
  (5 below, 2 above range)

bitpack-write-guarded (n=40, range 4380119.2-5555567.5 ns)
  4380119.2 |########################
  4438891.6 |################
  4497664.0 |
  4556436.4 |
  4615208.8 |################
  4673981.3 |########################
  4732753.7 |################
  4791526.1 |
  4850298.5 |########
  4909070.9 |########
  4967843.3 |########################
  5026615.8 |################
  5085388.2 |########################################
  5144160.6 |
  5202933.0 |########################
  5261705.4 |########
  5320477.8 |
  5379250.3 |################
  5438022.7 |########
  5496795.1 |########
  (5 below, 3 above range)

```
