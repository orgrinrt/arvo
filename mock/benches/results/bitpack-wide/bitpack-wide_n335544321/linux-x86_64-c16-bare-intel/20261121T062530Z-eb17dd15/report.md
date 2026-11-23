# Packed 13-bit against the u16 carrier with both columns several times past a 12 MB L2, at one and four threads

4 variants, 40 samples per variant.
Baseline: **bitpack-wide-d16**

## Highlights

Baseline for all deltas below: **bitpack-wide-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-wide-d16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-wide-d16 has the worst median (5.92 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-wide-pipe4 at 4.76 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-wide-pipe4 dominates: 24% faster than the next best (bitpack-wide-d16-padal)

bitpack-wide-pipe4 (4.76 ms) leads bitpack-wide-d16-padal (5.91 ms) by 24%, a clear separation rather than a photo finish. CV 0.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-wide-d16-control shows alternating (throttle bounce) (autocorr -0.66)

bitpack-wide-d16-control's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: bitpack-wide-pipe4** at 4758918.1 ns median (-19.6% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.24x (fastest 4758918.1 ns, slowest 5917665.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Packed 13-bit against the u16 carrier with both columns several times past a 12 MB L2, at one and four threads |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112815569 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-wide-d16 | 6172554ns | 5918693ns | 5903928ns | 6001153ns | 6955385ns | base |
| bitpack-wide-d16-control | 5917781ns | 5911893ns | 5898668ns | 5912118ns | 5953883ns | -4.13% |
| bitpack-wide-d16-padal | 5920626ns | 5910715ns | 5903106ns | 5910323ns | 5969057ns | -4.08% |
| bitpack-wide-pipe4 | 4753519ns | 4759805ns | 4698185ns | 4760103ns | 4789100ns | -22.99% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-wide-d16 | 6171619ns | 5902973ns | 6954431ns | base | 5.437 |
| bitpack-wide-d16-control | 5916869ns | 5897766ns | 5952983ns | -4.13% | 5.671 |
| bitpack-wide-d16-padal | 5919637ns | 5902178ns | 5967969ns | -4.08% | 5.668 |
| bitpack-wide-pipe4 | 4752667ns | 4697386ns | 4788218ns | -22.99% | 7.060 |

## Performance model

- Peak throughput: **7.143 Gops/s** (bitpack-wide-pipe4; best 20% batches)
- Ops per call: 33554432

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-wide-d16 | 5.670 | 79.4% |
| bitpack-wide-d16-control | 5.677 | 79.5% |
| bitpack-wide-d16-padal | 5.678 | 79.5% |
| bitpack-wide-pipe4 | 7.051 | 98.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-wide-d16 | 6172554ns | 6172554ns | base |
| bitpack-wide-d16-control | 5917781ns | 5917781ns | -4.13% |
| bitpack-wide-d16-padal | 5920626ns | 5920626ns | -4.08% |
| bitpack-wide-pipe4 | 4753519ns | 4753519ns | -22.99% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-wide-d16 | 5917666ns | base | --- | [5911899, 5923382] | --- | --- | --- | --- |
| bitpack-wide-d16-control | 5911001ns | -7394.7ns (-0.1%) | [-20025, -1590]ns | [5907794, 5914094] | YES | 0.0166 | 0.0166 | 0 |
| bitpack-wide-d16-padal | 5909832ns | -7762.9ns (-0.1%) | [-18132, -2761]ns | [5906381, 5910979] | YES | 0.0166 | 0.0166 | 0 |
| bitpack-wide-pipe4 | 4758918ns | -1155370.6ns (-19.5%) | [-1228884, -1149394]ns | [4756282, 4761731] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-wide-d16 | bitpack-wide-d16-control | bitpack-wide-d16-padal | bitpack-wide-pipe4 |
|---|---|---|---|---|---|
| 1 | 1 | 5906558ns | +0.1% | +0.1% | -19.4% |
| 1 | 2 | 5914293ns | -0.3% | -0.2% | -19.5% |
| 1 | 3 | 6911668ns | -14.4% | -14.5% | -31.2% |
| 1 | 4 | 5918629ns | -0.2% | -0.1% | -20.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-wide-d16 | -0.408 | moderate- |
| bitpack-wide-d16-control | -0.665 | HIGH- (thermal bounce) |
| bitpack-wide-d16-padal | -0.211 | moderate- |
| bitpack-wide-pipe4 | 0.030 | ok |

**Consistency summary:**

- **bitpack-wide-d16-control**: won 3/4, lost 1/4
- **bitpack-wide-d16-padal**: won 3/4, lost 0/4
- **bitpack-wide-pipe4**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-wide-d16 | 136.6ns | 6171618.6ns | 0.0% |  |
| bitpack-wide-d16-control | 143.1ns | 5916868.5ns | 0.0% |  |
| bitpack-wide-d16-padal | 142.9ns | 5919637.1ns | 0.0% |  |
| bitpack-wide-pipe4 | 107.1ns | 4752667.4ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-wide-d16 (n=40, range 5902972.6-6954430.7 ns)
  5902972.6 |########################################
  5955545.5 |
  6008118.4 |
  6060691.3 |
  6113264.2 |
  6165837.1 |
  6218410.0 |
  6270982.9 |
  6323555.8 |
  6376128.7 |
  6428701.6 |
  6481274.5 |
  6533847.4 |
  6586420.4 |
  6638993.3 |
  6691566.2 |
  6744139.1 |
  6796712.0 |
  6849284.9 |
  6901857.8 |#############
  (3 below, 1 above range)

bitpack-wide-d16-control (n=40, range 5897765.9-5952983.1 ns)
  5897765.9 |######
  5900526.7 |#############
  5903287.6 |####################
  5906048.5 |#################################
  5908809.3 |########################################
  5911570.2 |#################################
  5914331.0 |#############
  5917091.9 |####################
  5919852.8 |##########################
  5922613.6 |
  5925374.5 |
  5928135.4 |
  5930896.2 |
  5933657.1 |######
  5936418.0 |
  5939178.8 |
  5941939.7 |
  5944700.5 |
  5947461.4 |
  5950222.3 |
  (5 below, 3 above range)

bitpack-wide-d16-padal (n=40, range 5902177.8-5967968.9 ns)
  5902177.8 |####################################
  5905467.3 |########################
  5908756.9 |########################################
  5912046.5 |############
  5915336.0 |############
  5918625.6 |####
  5921915.1 |####
  5925204.7 |
  5928494.2 |
  5931783.8 |############
  5935073.4 |
  5938362.9 |
  5941652.5 |
  5944942.0 |
  5948231.6 |
  5951521.2 |
  5954810.7 |
  5958100.3 |
  5961389.8 |
  5964679.4 |
  (3 below, 1 above range)

bitpack-wide-pipe4 (n=40, range 4697385.7-4788218.2 ns)
  4697385.7 |
  4701927.3 |
  4706468.9 |
  4711010.6 |
  4715552.2 |
  4720093.8 |
  4724635.4 |
  4729177.1 |
  4733718.7 |
  4738260.3 |
  4742802.0 |
  4747343.6 |############
  4751885.2 |############################
  4756426.8 |########################################
  4760968.5 |####################
  4765510.1 |################
  4770051.7 |####
  4774593.3 |
  4779135.0 |####
  4783676.6 |
  (5 below, 4 above range)

```
