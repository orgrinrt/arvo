# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary period-aligned

3 variants, 40 samples per variant.
Baseline: **bitpack-write-aligned**

## Highlights

Baseline for all deltas below: **bitpack-write-aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bitpack-write-aligned) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bitpack-write-aligned has the worst median (169.22 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bitpack-write-dense at 8.53 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bitpack-write-dense dominates: 1271% faster than the next best (bitpack-write-windowed)

bitpack-write-dense (8.53 us) leads bitpack-write-windowed (116.90 us) by 1271%, a clear separation rather than a photo finish. CV 15.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-dense beats baseline by 93% (significant)

bitpack-write-dense is -158.13 us (93%) faster than baseline bitpack-write-aligned, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-write-aligned is an outlier: 19.8x slower than the field

bitpack-write-aligned (169.22 us) is 19.8x the fastest (8.53 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-write-dense is fastest but the noisiest (CV 15.2%)

bitpack-write-dense wins on median (8.53 us) yet has the highest variance (CV 15.2%), while bitpack-write-windowed is the steadiest (CV 5.5%, 116.90 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bitpack-write-dense shows warm-up / thermal drift (autocorr +0.80)

bitpack-write-dense's per-pass series has lag-1 autocorrelation +0.80, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 19.8x the fastest

Fastest bitpack-write-dense (8.53 us) to slowest bitpack-write-aligned (169.22 us): 19.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: bitpack-write-dense** at 8528.8 ns median (-95.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 19.84x (fastest 8528.8 ns, slowest 169221.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-write-aligned | 172273ns | 169516ns | 159059ns | 171427ns | 188024ns | base |
| bitpack-write-dense | 9084ns | 8639ns | 7747ns | 8785ns | 11318ns | -94.73% |
| bitpack-write-windowed | 117837ns | 117160ns | 109835ns | 117368ns | 127247ns | -31.60% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-aligned | 171955ns | 158782ns | 187651ns | base | 0.381 |
| bitpack-write-dense | 8966ns | 7650ns | 11163ns | -94.79% | 7.309 |
| bitpack-write-windowed | 117580ns | 109624ns | 126942ns | -31.62% | 0.557 |

## Performance model

- Peak throughput: **8.567 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-aligned | 0.387 | 4.5% |
| bitpack-write-dense | 7.684 | 89.7% |
| bitpack-write-windowed | 0.561 | 6.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-aligned | 172273ns | 172273ns | base |
| bitpack-write-dense | 9084ns | 9084ns | -94.73% |
| bitpack-write-windowed | 117837ns | 117837ns | -31.60% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-aligned | 169221ns | base | --- | [167228, 176388] | --- | --- | --- | --- |
| bitpack-write-dense | 8529ns | -160984.0ns (-95.1%) | [-166633, -159294]ns | [8238, 9297] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-write-windowed | 116896ns | -56579.1ns (-33.4%) | [-59003, -50667]ns | [114987, 119444] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | bitpack-write-aligned | bitpack-write-dense | bitpack-write-windowed |
|---|---|---|---|
| 1 | 155415ns | -94.3% | -17.4% |
| 2 | 154843ns | -94.3% | -19.1% |
| 3 | 158602ns | -94.1% | -21.4% |
| 4 | 157862ns | -93.9% | -18.8% |
| 5 | 162423ns | -94.1% | -23.9% |
| 6 | 164834ns | -94.2% | -25.2% |
| 7 | 158435ns | -94.0% | -22.8% |
| 8 | 160937ns | -94.1% | -25.7% |
| 9 | 161737ns | -94.6% | -26.7% |
| 10 | 167037ns | -94.7% | -29.8% |
| 11 | 190633ns | -95.7% | -40.0% |
| 12 | 211392ns | -96.1% | -46.8% |
| 13 | 172861ns | -95.2% | -35.0% |
| 14 | 176134ns | -95.3% | -33.9% |
| 15 | 181595ns | -95.5% | -38.8% |
| 16 | 177086ns | -95.4% | -35.6% |
| 17 | 171556ns | -95.2% | -33.1% |
| 18 | 168693ns | -95.1% | -28.4% |
| 19 | 169750ns | -95.1% | -34.7% |
| 20 | 167861ns | -95.1% | -31.3% |
| 21 | 177363ns | -95.0% | -30.8% |
| 22 | 179796ns | -94.9% | -32.5% |
| 23 | 178285ns | -93.5% | -35.3% |
| 24 | 178608ns | -93.5% | -34.7% |
| 25 | 184454ns | -93.7% | -35.1% |
| 26 | 172717ns | -94.0% | -30.8% |
| 27 | 176850ns | -94.5% | -32.7% |
| 28 | 176642ns | -93.3% | -32.7% |
| 29 | 167405ns | -93.1% | -29.0% |
| 30 | 166870ns | -93.4% | -16.5% |
| 31 | 167506ns | -95.4% | -36.3% |
| 32 | 174217ns | -95.6% | -36.2% |
| 33 | 183140ns | -95.8% | -39.6% |
| 34 | 167613ns | -95.4% | -34.7% |
| 35 | 167108ns | -95.4% | -30.5% |
| 36 | 167348ns | -95.4% | -33.6% |
| 37 | 191593ns | -96.0% | -43.7% |
| 38 | 178422ns | -95.7% | -33.1% |
| 39 | 166779ns | -95.4% | -30.2% |
| 40 | 165788ns | -95.4% | -34.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-aligned | 0.508 | HIGH+ (drift/warm-up) |
| bitpack-write-dense | 0.798 | HIGH+ (drift/warm-up) |
| bitpack-write-windowed | 0.285 | moderate+ |

**Consistency summary:**

- **bitpack-write-dense**: won 40/40, lost 0/40
- **bitpack-write-windowed**: won 40/40, lost 0/40

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-aligned | 9.4ns | 171954.8ns | 0.0% |  |
| bitpack-write-dense | 4.2ns | 8966.4ns | 0.0% |  |
| bitpack-write-windowed | 7.9ns | 117579.7ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-aligned (n=40, range 158781.8-187651.5 ns)
  158781.8 |
  160225.3 |######
  161668.8 |#############
  163112.3 |
  164555.8 |#############
  165999.2 |########################################
  167442.7 |##########################
  168886.2 |######
  170329.7 |######
  171773.2 |#############
  173216.6 |######
  174660.1 |
  176103.6 |#################################
  177547.1 |####################
  178990.6 |######
  180434.0 |######
  181877.5 |######
  183321.0 |######
  184764.5 |
  186208.0 |
  (5 below, 3 above range)

bitpack-write-dense (n=40, range 7649.7-11163.1 ns)
   7649.7 |####################
   7825.4 |
   8001.1 |
   8176.7 |########################################
   8352.4 |
   8528.1 |
   8703.7 |####################
   8879.4 |
   9055.1 |
   9230.7 |########
   9406.4 |################
   9582.1 |########
   9757.7 |
   9933.4 |
  10109.1 |
  10284.7 |####
  10460.4 |
  10636.1 |
  10811.7 |
  10987.4 |####
  (5 below, 5 above range)

bitpack-write-windowed (n=40, range 109624.3-126941.8 ns)
  109624.3 |
  110490.2 |########################################
  111356.1 |
  112221.9 |################
  113087.8 |
  113953.7 |########################
  114819.5 |################
  115685.4 |########################
  116551.3 |################
  117417.2 |
  118283.0 |################################
  119148.9 |################################
  120014.8 |########
  120880.7 |########
  121746.5 |########
  122612.4 |################
  123478.3 |########
  124344.1 |########
  125210.0 |########
  126075.9 |
  (4 below, 3 above range)

```

## Diagnostics

- **bitpack-write-aligned**: autocorrelation=0.51 (measurement drift or warm-up artifact)
- **bitpack-write-dense**: autocorrelation=0.80 (measurement drift or warm-up artifact)
