# Packed 13-bit write against a dense u16 write, column split 1, 2 and 4 ways, every internal boundary deliberately misaligned

3 variants, 40 samples per variant.
Baseline: **bitpack-write-dense**

## Highlights

Baseline for all deltas below: **bitpack-write-dense**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-write-dense dominates: 1434% faster than the next best (bitpack-write-unsound)

bitpack-write-dense (409.49 us) leads bitpack-write-unsound (6.28 ms) by 1434%, a clear separation rather than a photo finish. CV 8.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-write-guarded is an outlier: 17.8x slower than the field

bitpack-write-guarded (7.27 ms) is 17.8x the fastest (409.49 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### bitpack-write-dense is fastest but the noisiest (CV 8.2%)

bitpack-write-dense wins on median (409.49 us) yet has the highest variance (CV 8.2%), while bitpack-write-guarded is the steadiest (CV 2.0%, 7.27 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bitpack-write-dense shows alternating (throttle bounce) (autocorr -0.77)

bitpack-write-dense's per-pass series has lag-1 autocorrelation -0.77, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (bitpack-write-dense)

The baseline bitpack-write-dense is the fastest (409.49 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 17.8x the fastest

Fastest bitpack-write-dense (409.49 us) to slowest bitpack-write-guarded (7.27 ms): 17.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (bitpack-write-dense) is the fastest** at 409488.8 ns median
- 2 variants significantly slower than baseline
- Spread: 17.76x (fastest 409488.8 ns, slowest 7272811.0 ns)

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
| bitpack-write-dense | 432948ns | 409693ns | 406770ns | 422559ns | 490293ns | base |
| bitpack-write-guarded | 7274730ns | 7273065ns | 7118874ns | 7258929ns | 7477989ns | +1580.28% |
| bitpack-write-unsound | 6395248ns | 6283879ns | 6274539ns | 6304747ns | 6787461ns | +1377.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-write-dense | 432665ns | 406534ns | 489957ns | base | 4.847 |
| bitpack-write-guarded | 7274448ns | 7118603ns | 7477714ns | +1581.31% | 0.288 |
| bitpack-write-unsound | 6394953ns | 6274227ns | 6787157ns | +1378.04% | 0.328 |

## Performance model

- Peak throughput: **5.159 Gops/s** (bitpack-write-dense; best 20% batches)
- Ops per call: 2097150

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-write-dense | 5.121 | 99.3% |
| bitpack-write-guarded | 0.288 | 5.6% |
| bitpack-write-unsound | 0.334 | 6.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-write-dense | 432948ns | 432948ns | base |
| bitpack-write-guarded | 7274730ns | 7274730ns | +1580.28% |
| bitpack-write-unsound | 6395248ns | 6395248ns | +1377.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-write-dense | 409489ns | base | --- | [408419, 450926] | --- | --- | --- | --- |
| bitpack-write-guarded | 7272811ns | +6841485.8ns (+1670.7%) | [+6762434, +6901380]ns | [7178293, 7318733] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-write-unsound | 6283525ns | +5871342.6ns (+1433.8%) | [+5831891, +5879277]ns | [6278340, 6288686] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-write-dense | bitpack-write-guarded | bitpack-write-unsound |
|---|---|---|---|---|
| 1 | 1 | 407987ns | +1645.7% | +1440.5% |
| 1 | 2 | 457594ns | +1458.1% | +1272.8% |
| 1 | 3 | 409053ns | +1683.6% | +1537.4% |
| 1 | 4 | 452121ns | +1531.1% | +1287.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-write-dense | -0.769 | HIGH- (thermal bounce) |
| bitpack-write-guarded | 0.293 | moderate+ |
| bitpack-write-unsound | -0.432 | moderate- |

**Consistency summary:**

- **bitpack-write-guarded**: won 0/4, lost 4/4
- **bitpack-write-unsound**: won 0/4, lost 4/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-write-dense | 50.4ns | 432665.5ns | 0.0% |  |
| bitpack-write-guarded | 68.6ns | 7274448.3ns | 0.0% |  |
| bitpack-write-unsound | 64.4ns | 6394953.0ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-write-dense (n=40, range 406533.9-489956.9 ns)
  406533.9 |########################################
  410705.0 |
  414876.2 |##
  419047.3 |##
  423218.5 |
  427389.6 |
  431560.8 |
  435731.9 |
  439903.1 |
  444074.2 |
  448245.4 |############
  452416.5 |##
  456587.7 |##
  460758.8 |
  464930.0 |
  469101.2 |
  473272.3 |
  477443.5 |
  481614.6 |##
  485785.8 |##
  (4 below, 5 above range)

bitpack-write-guarded (n=40, range 7118603.1-7477713.6 ns)
  7118603.1 |########################################
  7136558.6 |
  7154514.1 |
  7172469.7 |
  7190425.2 |
  7208380.7 |####
  7226336.2 |####
  7244291.8 |################
  7262247.3 |
  7280202.8 |############
  7298158.3 |########
  7316113.9 |########
  7334069.4 |
  7352024.9 |########
  7369980.4 |################
  7387936.0 |####
  7405891.5 |########
  7423847.0 |####
  7441802.5 |
  7459758.1 |
  (4 below, 3 above range)

bitpack-write-unsound (n=40, range 6274227.5-6787157.2 ns)
  6274227.5 |########################################
  6299874.0 |#
  6325520.5 |
  6351166.9 |
  6376813.4 |#
  6402459.9 |
  6428106.4 |#
  6453752.9 |
  6479399.4 |#
  6505045.9 |
  6530692.3 |
  6556338.8 |#
  6581985.3 |
  6607631.8 |
  6633278.3 |###
  6658924.8 |
  6684571.3 |
  6710217.8 |
  6735864.2 |#
  6761510.7 |#
  (5 below, 3 above range)

```
