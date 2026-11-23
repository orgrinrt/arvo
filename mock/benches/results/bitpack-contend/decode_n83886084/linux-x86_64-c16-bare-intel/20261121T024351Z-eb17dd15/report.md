# The packed decode with one, two and four accumulators, against the u16 carrier, at one and four threads

5 variants, 40 samples per variant.
Baseline: **bitpack-contend-d16**

## Highlights

Baseline for all deltas below: **bitpack-contend-d16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bitpack-contend-packed-simd beats baseline by 22% (significant)

bitpack-contend-packed-simd is -127.22 us (22%) faster than baseline bitpack-contend-d16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### bitpack-contend-pipe2 is fastest but the noisiest (CV 7.1%)

bitpack-contend-pipe2 wins on median (428.97 us) yet has the highest variance (CV 7.1%), while bitpack-contend-pipe4 is the steadiest (CV 3.3%, 467.14 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### bitpack-contend-d16-control shows alternating (throttle bounce) (autocorr -0.80)

bitpack-contend-d16-control's per-pass series has lag-1 autocorrelation -0.80, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader bitpack-contend-pipe2 vs stability leader bitpack-contend-pipe4 (+9% speed for 2.1x steadier)

bitpack-contend-pipe2 is fastest (428.97 us, CV 7.1%); bitpack-contend-pipe4 gives up 8.9% median for 2.1x lower variance (CV 3.3%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: bitpack-contend-pipe2** at 428967.2 ns median (-25.1% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.39x (fastest 428967.2 ns, slowest 595670.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | The packed decode with one, two and four accumulators, against the u16 carrier, at one and four threads |
| Passes | 4 |
| Runs per pass | 1000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112791945 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 571301ns | 573401ns | 528613ns | 568180ns | 623354ns | base |
| bitpack-contend-d16-control | 570888ns | 596461ns | 521206ns | 572614ns | 615393ns | -0.07% |
| bitpack-contend-packed-simd | 452480ns | 464044ns | 392638ns | 462015ns | 483720ns | -20.80% |
| bitpack-contend-pipe2 | 441242ns | 429461ns | 410203ns | 436346ns | 486970ns | -22.77% |
| bitpack-contend-pipe4 | 465481ns | 467601ns | 445326ns | 465732ns | 484885ns | -18.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bitpack-contend-d16 | 570619ns | 527972ns | 622554ns | base | 14.701 |
| bitpack-contend-d16-control | 570134ns | 520391ns | 614673ns | -0.09% | 14.713 |
| bitpack-contend-packed-simd | 451965ns | 392192ns | 483125ns | -20.79% | 18.560 |
| bitpack-contend-pipe2 | 440662ns | 409599ns | 486336ns | -22.77% | 19.036 |
| bitpack-contend-pipe4 | 464941ns | 444791ns | 484225ns | -18.52% | 18.042 |

## Performance model

- Peak throughput: **21.389 Gops/s** (bitpack-contend-packed-simd; best 20% batches)
- Ops per call: 8388608

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bitpack-contend-d16 | 14.647 | 68.5% |
| bitpack-contend-d16-control | 14.083 | 65.8% |
| bitpack-contend-packed-simd | 18.096 | 84.6% |
| bitpack-contend-pipe2 | 19.555 | 91.4% |
| bitpack-contend-pipe4 | 17.957 | 84.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bitpack-contend-d16 | 571301ns | 571301ns | base |
| bitpack-contend-d16-control | 570888ns | 570888ns | -0.07% |
| bitpack-contend-packed-simd | 452480ns | 452480ns | -20.80% |
| bitpack-contend-pipe2 | 441242ns | 441242ns | -22.77% |
| bitpack-contend-pipe4 | 465481ns | 465481ns | -18.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bitpack-contend-d16 | 572713ns | base | --- | [534705, 594713] | --- | --- | --- | --- |
| bitpack-contend-d16-control | 595670ns | no significant difference | [-9207, +7390]ns | [534700, 596307] | no | 0.8746 | 0.8746 | 0 |
| bitpack-contend-packed-simd | 463561ns | -127224.6ns (-22.2%) | [-134931, -106703]ns | [463149, 463864] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-pipe2 | 428967ns | -126430.4ns (-22.1%) | [-154845, -106673]ns | [427642, 437852] | YES | 0.0000 | 0.0000 | 0 |
| bitpack-contend-pipe4 | 467140ns | -112226.0ns (-19.6%) | [-127900, -77647]ns | [462033, 468031] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bitpack-contend-d16 | bitpack-contend-d16-control | bitpack-contend-packed-simd | bitpack-contend-pipe2 | bitpack-contend-pipe4 |
|---|---|---|---|---|---|---|
| 1 | 1 | 531754ns | +3.3% | -26.2% | -19.6% | -13.2% |
| 1 | 2 | 603160ns | -0.3% | -23.2% | -28.9% | -22.3% |
| 1 | 3 | 534705ns | -1.9% | -13.3% | -13.0% | -12.6% |
| 1 | 4 | 562296ns | +6.0% | -16.6% | -23.9% | -18.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bitpack-contend-d16 | -0.710 | HIGH- (thermal bounce) |
| bitpack-contend-d16-control | -0.796 | HIGH- (thermal bounce) |
| bitpack-contend-packed-simd | -0.067 | ok |
| bitpack-contend-pipe2 | -0.403 | moderate- |
| bitpack-contend-pipe4 | -0.212 | moderate- |

**Consistency summary:**

- **bitpack-contend-d16-control**: won 2/4, lost 2/4
- **bitpack-contend-packed-simd**: won 4/4, lost 0/4
- **bitpack-contend-pipe2**: won 4/4, lost 0/4
- **bitpack-contend-pipe4**: won 4/4, lost 0/4

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bitpack-contend-d16 | 79.7ns | 570619.4ns | 0.0% |  |
| bitpack-contend-d16-control | 68.7ns | 570134.3ns | 0.0% |  |
| bitpack-contend-packed-simd | 73.4ns | 451964.7ns | 0.0% |  |
| bitpack-contend-pipe2 | 42.6ns | 440662.2ns | 0.0% |  |
| bitpack-contend-pipe4 | 38.8ns | 464941.4ns | 0.0% |  |

## Distribution (algo ns)

```
bitpack-contend-d16 (n=40, range 527972.5-622554.3 ns)
  527972.5 |#################
  532701.6 |########################################
  537430.6 |
  542159.7 |
  546888.8 |
  551617.9 |####
  556347.0 |####
  561076.1 |
  565805.2 |
  570534.3 |
  575263.4 |
  579992.5 |
  584721.5 |########
  589450.6 |#################
  594179.7 |####
  598908.8 |#############
  603637.9 |#############
  608367.0 |####
  613096.1 |########
  617825.2 |
  (5 below, 4 above range)

bitpack-contend-d16-control (n=40, range 520390.6-614672.9 ns)
  520390.6 |####################
  525104.7 |##############################
  529818.9 |##########
  534533.0 |#####
  539247.1 |
  543961.2 |##########
  548675.3 |#####
  553389.4 |
  558103.5 |
  562817.7 |
  567531.8 |
  572245.9 |
  576960.0 |
  581674.1 |
  586388.2 |
  591102.3 |####################
  595816.5 |########################################
  600530.6 |##############################
  605244.7 |#####
  609958.8 |
  (2 below, 3 above range)

bitpack-contend-packed-simd (n=40, range 392191.9-483125.3 ns)
  392191.9 |###########
  396738.5 |
  401285.2 |
  405831.9 |
  410378.6 |
  414925.2 |
  419471.9 |
  424018.6 |####
  428565.2 |
  433111.9 |
  437658.6 |
  442205.2 |
  446751.9 |
  451298.6 |
  455845.2 |
  460391.9 |########################################
  464938.6 |################
  469485.3 |##
  474031.9 |
  478578.6 |####
  (3 below, 3 above range)

bitpack-contend-pipe2 (n=40, range 409599.1-486335.6 ns)
  409599.1 |
  413436.0 |
  417272.8 |##
  421109.6 |
  424946.4 |########################################
  428783.2 |###############
  432620.1 |##
  436456.9 |
  440293.7 |##
  444130.5 |
  447967.3 |##
  451804.2 |##
  455641.0 |
  459477.8 |
  463314.6 |##########
  467151.5 |#####
  470988.3 |
  474825.1 |
  478661.9 |
  482498.7 |
  (2 below, 5 above range)

bitpack-contend-pipe4 (n=40, range 444791.2-484224.7 ns)
  444791.2 |###
  446762.9 |
  448734.6 |
  450706.2 |
  452677.9 |
  454649.6 |
  456621.3 |
  458592.9 |#####################
  460564.6 |#####################
  462536.3 |#######
  464507.9 |
  466479.6 |########################################
  468451.3 |##################
  470423.0 |
  472394.6 |###
  474366.3 |
  476338.0 |
  478309.6 |###
  480281.3 |
  482253.0 |
  (3 below, 4 above range)

```
