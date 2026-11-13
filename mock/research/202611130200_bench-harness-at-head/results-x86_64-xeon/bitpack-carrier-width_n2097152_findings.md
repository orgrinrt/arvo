# Packed 13-bit against u16, u32 and u64 dense carriers, swept from L1 to past a 12 MB L2

5 variants, 40 samples per variant.
Baseline: **bitpack-carrier-d16**

## Highlights

Baseline for all deltas below: **bitpack-carrier-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-carrier-packed dominates: 23% faster than the next best (bitpack-carrier-d32)

bitpack-carrier-packed (498.20 us) leads bitpack-carrier-d32 (613.57 us) by 23%, a clear separation rather than a photo finish. CV 7.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bitpack-carrier-packed beats baseline by 24% (significant)

bitpack-carrier-packed is -159.95 us (24%) faster than baseline bitpack-carrier-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-carrier-d64 shows alternating (throttle bounce) (autocorr -0.81)

bitpack-carrier-d64's per-pass series has lag-1 autocorrelation -0.81, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: bitpack-carrier-packed** at 498199.8 ns median (-24.4% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.74x (fastest 498199.8 ns, slowest 868033.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 673128ns | 659698ns | 625867ns | 660696ns | 757685ns | base |
| bitpack-carrier-d16-control | 717959ns | 690661ns | 615416ns | 693601ns | 893579ns | +6.66% |
| bitpack-carrier-d32 | 597695ns | 615626ns | 496097ns | 615727ns | 645199ns | -11.21% |
| bitpack-carrier-d64 | 882369ns | 870566ns | 794765ns | 872876ns | 998452ns | +31.08% |
| bitpack-carrier-packed | 501382ns | 499580ns | 460168ns | 497905ns | 553026ns | -25.51% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-carrier-d16 | 671882ns | 624609ns | 756358ns | base | 3.121 |
| bitpack-carrier-d16-control | 716468ns | 614285ns | 891339ns | +6.64% | 2.927 |
| bitpack-carrier-d32 | 595658ns | 494407ns | 642903ns | -11.34% | 3.521 |
| bitpack-carrier-d64 | 880001ns | 792322ns | 996101ns | +30.98% | 2.383 |
| bitpack-carrier-packed | 500026ns | 458965ns | 551471ns | -25.58% | 4.194 |

## Performance model

- Peak throughput: **4.569 Gops/s** (bitpack-carrier-packed; best 20% batches)
- Ops per call: 2097152

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-carrier-d16 | 3.184 | 69.7% |
| bitpack-carrier-d16-control | 3.042 | 66.6% |
| bitpack-carrier-d32 | 3.418 | 74.8% |
| bitpack-carrier-d64 | 2.416 | 52.9% |
| bitpack-carrier-packed | 4.209 | 92.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-carrier-d16 | 673128ns | 673128ns | base |
| bitpack-carrier-d16-control | 717959ns | 717959ns | +6.66% |
| bitpack-carrier-d32 | 597695ns | 597695ns | -11.21% |
| bitpack-carrier-d64 | 882369ns | 882369ns | +31.08% |
| bitpack-carrier-packed | 501382ns | 501382ns | -25.51% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-carrier-d16 | 658569ns | base | --- | [642246, 674107] | --- | --- | --- | --- |
| bitpack-carrier-d16-control | 689392ns | +43767.0ns (+6.6%) | [+4614, +58396]ns | [670312, 711069] | YES | 0.0385 | 0.0385 | 0 |
| bitpack-carrier-d32 | 613573ns | -44180.5ns (-6.7%) | [-83878, -28558]ns | [601995, 625646] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-d64 | 868033ns | +213966.2ns (+32.5%) | [+190334, +234810]ns | [855548, 881749] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-carrier-packed | 498200ns | -159950.0ns (-24.3%) | [-183709, -141715]ns | [487682, 503511] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-carrier-d16 | bitpack-carrier-d16-control | bitpack-carrier-d32 | bitpack-carrier-d64 | bitpack-carrier-packed |
|---|---|---|---|---|---|---|
| 1 | 1 | 629548ns | +8.2% | -2.5% | +42.0% | -22.6% |
| 1 | 2 | 636618ns | +7.9% | -5.3% | +32.3% | -20.8% |
| 1 | 3 | 672104ns | +9.3% | -28.9% | +34.4% | -28.8% |
| 1 | 4 | 707704ns | -8.4% | -10.4% | +20.6% | -28.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-carrier-d16 | 0.263 | moderate+ |
| bitpack-carrier-d16-control | -0.494 | moderate- |
| bitpack-carrier-d32 | -0.462 | moderate- |
| bitpack-carrier-d64 | -0.807 | HIGH- (thermal bounce) |
| bitpack-carrier-packed | -0.786 | HIGH- (thermal bounce) |

**Consistency summary:**

- **bitpack-carrier-d16-control**: won 1/4, lost 3/4
- **bitpack-carrier-d32**: won 4/4, lost 0/4
- **bitpack-carrier-d64**: won 0/4, lost 4/4
- **bitpack-carrier-packed**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-carrier-d16 | 350.4ns | 671882.4ns | 0.1% |  |
| bitpack-carrier-d16-control | 398.3ns | 716468.0ns | 0.1% |  |
| bitpack-carrier-d32 | 651.2ns | 595657.7ns | 0.1% |  |
| bitpack-carrier-d64 | 776.1ns | 880000.7ns | 0.1% |  |
| bitpack-carrier-packed | 414.5ns | 500026.2ns | 0.1% |  |

## Distribution (algo ns)

```
bitpack-carrier-d16 (n=40, range 624608.5-756357.6 ns)
  624608.5 |########################################
  631196.0 |################
  637783.4 |########################
  644370.9 |########################
  650958.3 |########################
  657545.8 |################
  664133.2 |########################
  670720.7 |################
  677308.1 |########
  683895.6 |########################
  690483.1 |########
  697070.5 |########
  703658.0 |########
  710245.4 |########
  716832.9 |########
  723420.3 |########
  730007.8 |
  736595.2 |
  743182.7 |
  749770.1 |########
  (4 below, 2 above range)

bitpack-carrier-d16-control (n=40, range 614285.1-891339.3 ns)
  614285.1 |######
  628137.8 |##########################
  641990.5 |####################
  655843.2 |####################
  669695.9 |#############
  683548.6 |########################################
  697401.3 |##########################
  711254.0 |##########################
  725106.8 |
  738959.5 |######
  752812.2 |#############
  766664.9 |
  780517.6 |
  794370.3 |
  808223.0 |
  822075.7 |######
  835928.4 |####################
  849781.1 |######
  863633.8 |
  877486.6 |
  (3 below, 2 above range)

bitpack-carrier-d32 (n=40, range 494407.4-642902.9 ns)
  494407.4 |
  501832.2 |
  509257.0 |
  516681.7 |######
  524106.5 |
  531531.3 |
  538956.1 |
  546380.8 |
  553805.6 |
  561230.4 |
  568655.1 |
  576079.9 |
  583504.7 |
  590929.5 |##########################
  598354.2 |########################################
  605779.0 |####################
  613203.8 |##########################
  620628.5 |#################################
  628053.3 |#################################
  635478.1 |#############
  (6 below, 4 above range)

bitpack-carrier-d64 (n=40, range 792321.8-996101.3 ns)
  792321.8 |
  802510.8 |
  812699.8 |
  822888.7 |######
  833077.7 |########################################
  843266.7 |##########################
  853455.7 |########################################
  863644.6 |####################
  873833.6 |#################################
  884022.6 |######
  894211.6 |######
  904400.5 |##########################
  914589.5 |
  924778.5 |
  934967.5 |#############
  945156.4 |
  955345.4 |
  965534.4 |
  975723.4 |######
  985912.3 |######
  (2 below, 3 above range)

bitpack-carrier-packed (n=40, range 458965.4-551471.0 ns)
  458965.4 |
  463590.6 |######
  468215.9 |
  472841.2 |##########################
  477466.5 |####################
  482091.8 |##########################
  486717.1 |######
  491342.3 |######
  495967.6 |########################################
  500592.9 |########################################
  505218.2 |####################
  509843.5 |####################
  514468.8 |#############
  519094.0 |######
  523719.3 |
  528344.6 |
  532969.9 |
  537595.2 |
  542220.5 |
  546845.7 |
  (2 below, 3 above range)

```
