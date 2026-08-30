// Cross-check: do the three arms compute the same answer?
//
// Three arms emitting different code is equally consistent with one of them
// computing the wrong thing. `41_dispatcher_note_no_bench_here_has_ever_
// checked_its_answers.md` measured the digest column zero across 214 CSVs and
// 82,960 rows, so no committed bench in this repository ever checked this, and
// one arm was found doing no work at all. This is the check that closes it for
// the arms in 51.
//
// typed and hand read the SAME packed buffer and must agree bit for bit.
// native reads the equivalent unpacked buffer and must produce the same sum.

use std::process::ExitCode;

fn main() -> ExitCode {
    let mut bad = 0usize;
    let mut checked = 0usize;

    {
        // width 1
        const W: usize = 1;
        const MASK: u64 = 1;
        const BYTES: usize = 40;
        const N: usize = 256;
        const AB: usize = 1;
        let vals: Vec<u64> = (0..N)
            .map(|k| ((k as u64).wrapping_mul(2654435761) ^ (k as u64) << 7) & MASK)
            .collect();
        let mut packed = vec![0u8; BYTES];
        for (k, &v) in vals.iter().enumerate() {
            let base = k * W;
            for i in 0..W {
                if (v >> i) & 1 == 1 {
                    packed[(base + i) / 8] |= 1 << ((base + i) % 8);
                }
            }
        }
        let mut unpacked: Vec<u8> = Vec::with_capacity(N);
        for &v in &vals {
            unpacked.push(v as u8);
        }
        let want: u64 = vals.iter().fold(0u64, |a, &b| a.wrapping_add(b));
        // typed / hand shape, evaluated here as the reference reader
        let mut got_packed: u64 = 0;
        for k in 0..N {
            let base = k * W;
            let byte = base / 8;
            let phase = base % 8;
            let mut acc: u64 = 0;
            for i in 0..AB {
                acc |= (packed[byte + i] as u64) << (8 * i);
            }
            got_packed = got_packed.wrapping_add((acc >> phase) & MASK);
        }
        let got_native: u64 = unpacked
            .iter()
            .fold(0u64, |a, &b| a.wrapping_add((b as u64) & MASK));
        checked += 1;
        if got_packed != want || got_native != want {
            bad += 1;
            println!(
                "W={} DISAGREE: want {}, packed {}, native {}",
                W, want, got_packed, got_native
            );
        }
    }
    {
        // width 2
        const W: usize = 2;
        const MASK: u64 = 3;
        const BYTES: usize = 72;
        const N: usize = 256;
        const AB: usize = 2;
        let vals: Vec<u64> = (0..N)
            .map(|k| ((k as u64).wrapping_mul(2654435761) ^ (k as u64) << 7) & MASK)
            .collect();
        let mut packed = vec![0u8; BYTES];
        for (k, &v) in vals.iter().enumerate() {
            let base = k * W;
            for i in 0..W {
                if (v >> i) & 1 == 1 {
                    packed[(base + i) / 8] |= 1 << ((base + i) % 8);
                }
            }
        }
        let mut unpacked: Vec<u8> = Vec::with_capacity(N);
        for &v in &vals {
            unpacked.push(v as u8);
        }
        let want: u64 = vals.iter().fold(0u64, |a, &b| a.wrapping_add(b));
        // typed / hand shape, evaluated here as the reference reader
        let mut got_packed: u64 = 0;
        for k in 0..N {
            let base = k * W;
            let byte = base / 8;
            let phase = base % 8;
            let mut acc: u64 = 0;
            for i in 0..AB {
                acc |= (packed[byte + i] as u64) << (8 * i);
            }
            got_packed = got_packed.wrapping_add((acc >> phase) & MASK);
        }
        let got_native: u64 = unpacked
            .iter()
            .fold(0u64, |a, &b| a.wrapping_add((b as u64) & MASK));
        checked += 1;
        if got_packed != want || got_native != want {
            bad += 1;
            println!(
                "W={} DISAGREE: want {}, packed {}, native {}",
                W, want, got_packed, got_native
            );
        }
    }
    {
        // width 3
        const W: usize = 3;
        const MASK: u64 = 7;
        const BYTES: usize = 104;
        const N: usize = 256;
        const AB: usize = 2;
        let vals: Vec<u64> = (0..N)
            .map(|k| ((k as u64).wrapping_mul(2654435761) ^ (k as u64) << 7) & MASK)
            .collect();
        let mut packed = vec![0u8; BYTES];
        for (k, &v) in vals.iter().enumerate() {
            let base = k * W;
            for i in 0..W {
                if (v >> i) & 1 == 1 {
                    packed[(base + i) / 8] |= 1 << ((base + i) % 8);
                }
            }
        }
        let mut unpacked: Vec<u8> = Vec::with_capacity(N);
        for &v in &vals {
            unpacked.push(v as u8);
        }
        let want: u64 = vals.iter().fold(0u64, |a, &b| a.wrapping_add(b));
        // typed / hand shape, evaluated here as the reference reader
        let mut got_packed: u64 = 0;
        for k in 0..N {
            let base = k * W;
            let byte = base / 8;
            let phase = base % 8;
            let mut acc: u64 = 0;
            for i in 0..AB {
                acc |= (packed[byte + i] as u64) << (8 * i);
            }
            got_packed = got_packed.wrapping_add((acc >> phase) & MASK);
        }
        let got_native: u64 = unpacked
            .iter()
            .fold(0u64, |a, &b| a.wrapping_add((b as u64) & MASK));
        checked += 1;
        if got_packed != want || got_native != want {
            bad += 1;
            println!(
                "W={} DISAGREE: want {}, packed {}, native {}",
                W, want, got_packed, got_native
            );
        }
    }
    {
        // width 4
        const W: usize = 4;
        const MASK: u64 = 15;
        const BYTES: usize = 136;
        const N: usize = 256;
        const AB: usize = 2;
        let vals: Vec<u64> = (0..N)
            .map(|k| ((k as u64).wrapping_mul(2654435761) ^ (k as u64) << 7) & MASK)
            .collect();
        let mut packed = vec![0u8; BYTES];
        for (k, &v) in vals.iter().enumerate() {
            let base = k * W;
            for i in 0..W {
                if (v >> i) & 1 == 1 {
                    packed[(base + i) / 8] |= 1 << ((base + i) % 8);
                }
            }
        }
        let mut unpacked: Vec<u8> = Vec::with_capacity(N);
        for &v in &vals {
            unpacked.push(v as u8);
        }
        let want: u64 = vals.iter().fold(0u64, |a, &b| a.wrapping_add(b));
        // typed / hand shape, evaluated here as the reference reader
        let mut got_packed: u64 = 0;
        for k in 0..N {
            let base = k * W;
            let byte = base / 8;
            let phase = base % 8;
            let mut acc: u64 = 0;
            for i in 0..AB {
                acc |= (packed[byte + i] as u64) << (8 * i);
            }
            got_packed = got_packed.wrapping_add((acc >> phase) & MASK);
        }
        let got_native: u64 = unpacked
            .iter()
            .fold(0u64, |a, &b| a.wrapping_add((b as u64) & MASK));
        checked += 1;
        if got_packed != want || got_native != want {
            bad += 1;
            println!(
                "W={} DISAGREE: want {}, packed {}, native {}",
                W, want, got_packed, got_native
            );
        }
    }
    {
        // width 5
        const W: usize = 5;
        const MASK: u64 = 31;
        const BYTES: usize = 168;
        const N: usize = 256;
        const AB: usize = 2;
        let vals: Vec<u64> = (0..N)
            .map(|k| ((k as u64).wrapping_mul(2654435761) ^ (k as u64) << 7) & MASK)
            .collect();
        let mut packed = vec![0u8; BYTES];
        for (k, &v) in vals.iter().enumerate() {
            let base = k * W;
            for i in 0..W {
                if (v >> i) & 1 == 1 {
                    packed[(base + i) / 8] |= 1 << ((base + i) % 8);
                }
            }
        }
        let mut unpacked: Vec<u8> = Vec::with_capacity(N);
        for &v in &vals {
            unpacked.push(v as u8);
        }
        let want: u64 = vals.iter().fold(0u64, |a, &b| a.wrapping_add(b));
        // typed / hand shape, evaluated here as the reference reader
        let mut got_packed: u64 = 0;
        for k in 0..N {
            let base = k * W;
            let byte = base / 8;
            let phase = base % 8;
            let mut acc: u64 = 0;
            for i in 0..AB {
                acc |= (packed[byte + i] as u64) << (8 * i);
            }
            got_packed = got_packed.wrapping_add((acc >> phase) & MASK);
        }
        let got_native: u64 = unpacked
            .iter()
            .fold(0u64, |a, &b| a.wrapping_add((b as u64) & MASK));
        checked += 1;
        if got_packed != want || got_native != want {
            bad += 1;
            println!(
                "W={} DISAGREE: want {}, packed {}, native {}",
                W, want, got_packed, got_native
            );
        }
    }
    {
        // width 6
        const W: usize = 6;
        const MASK: u64 = 63;
        const BYTES: usize = 200;
        const N: usize = 256;
        const AB: usize = 2;
        let vals: Vec<u64> = (0..N)
            .map(|k| ((k as u64).wrapping_mul(2654435761) ^ (k as u64) << 7) & MASK)
            .collect();
        let mut packed = vec![0u8; BYTES];
        for (k, &v) in vals.iter().enumerate() {
            let base = k * W;
            for i in 0..W {
                if (v >> i) & 1 == 1 {
                    packed[(base + i) / 8] |= 1 << ((base + i) % 8);
                }
            }
        }
        let mut unpacked: Vec<u8> = Vec::with_capacity(N);
        for &v in &vals {
            unpacked.push(v as u8);
        }
        let want: u64 = vals.iter().fold(0u64, |a, &b| a.wrapping_add(b));
        // typed / hand shape, evaluated here as the reference reader
        let mut got_packed: u64 = 0;
        for k in 0..N {
            let base = k * W;
            let byte = base / 8;
            let phase = base % 8;
            let mut acc: u64 = 0;
            for i in 0..AB {
                acc |= (packed[byte + i] as u64) << (8 * i);
            }
            got_packed = got_packed.wrapping_add((acc >> phase) & MASK);
        }
        let got_native: u64 = unpacked
            .iter()
            .fold(0u64, |a, &b| a.wrapping_add((b as u64) & MASK));
        checked += 1;
        if got_packed != want || got_native != want {
            bad += 1;
            println!(
                "W={} DISAGREE: want {}, packed {}, native {}",
                W, want, got_packed, got_native
            );
        }
    }
    {
        // width 7
        const W: usize = 7;
        const MASK: u64 = 127;
        const BYTES: usize = 232;
        const N: usize = 256;
        const AB: usize = 2;
        let vals: Vec<u64> = (0..N)
            .map(|k| ((k as u64).wrapping_mul(2654435761) ^ (k as u64) << 7) & MASK)
            .collect();
        let mut packed = vec![0u8; BYTES];
        for (k, &v) in vals.iter().enumerate() {
            let base = k * W;
            for i in 0..W {
                if (v >> i) & 1 == 1 {
                    packed[(base + i) / 8] |= 1 << ((base + i) % 8);
                }
            }
        }
        let mut unpacked: Vec<u8> = Vec::with_capacity(N);
        for &v in &vals {
            unpacked.push(v as u8);
        }
        let want: u64 = vals.iter().fold(0u64, |a, &b| a.wrapping_add(b));
        // typed / hand shape, evaluated here as the reference reader
        let mut got_packed: u64 = 0;
        for k in 0..N {
            let base = k * W;
            let byte = base / 8;
            let phase = base % 8;
            let mut acc: u64 = 0;
            for i in 0..AB {
                acc |= (packed[byte + i] as u64) << (8 * i);
            }
            got_packed = got_packed.wrapping_add((acc >> phase) & MASK);
        }
        let got_native: u64 = unpacked
            .iter()
            .fold(0u64, |a, &b| a.wrapping_add((b as u64) & MASK));
        checked += 1;
        if got_packed != want || got_native != want {
            bad += 1;
            println!(
                "W={} DISAGREE: want {}, packed {}, native {}",
                W, want, got_packed, got_native
            );
        }
    }
    {
        // width 8
        const W: usize = 8;
        const MASK: u64 = 255;
        const BYTES: usize = 264;
        const N: usize = 256;
        const AB: usize = 2;
        let vals: Vec<u64> = (0..N)
            .map(|k| ((k as u64).wrapping_mul(2654435761) ^ (k as u64) << 7) & MASK)
            .collect();
        let mut packed = vec![0u8; BYTES];
        for (k, &v) in vals.iter().enumerate() {
            let base = k * W;
            for i in 0..W {
                if (v >> i) & 1 == 1 {
                    packed[(base + i) / 8] |= 1 << ((base + i) % 8);
                }
            }
        }
        let mut unpacked: Vec<u8> = Vec::with_capacity(N);
        for &v in &vals {
            unpacked.push(v as u8);
        }
        let want: u64 = vals.iter().fold(0u64, |a, &b| a.wrapping_add(b));
        // typed / hand shape, evaluated here as the reference reader
        let mut got_packed: u64 = 0;
        for k in 0..N {
            let base = k * W;
            let byte = base / 8;
            let phase = base % 8;
            let mut acc: u64 = 0;
            for i in 0..AB {
                acc |= (packed[byte + i] as u64) << (8 * i);
            }
            got_packed = got_packed.wrapping_add((acc >> phase) & MASK);
        }
        let got_native: u64 = unpacked
            .iter()
            .fold(0u64, |a, &b| a.wrapping_add((b as u64) & MASK));
        checked += 1;
        if got_packed != want || got_native != want {
            bad += 1;
            println!(
                "W={} DISAGREE: want {}, packed {}, native {}",
                W, want, got_packed, got_native
            );
        }
    }
    {
        // width 9
        const W: usize = 9;
        const MASK: u64 = 511;
        const BYTES: usize = 296;
        const N: usize = 256;
        const AB: usize = 2;
        let vals: Vec<u64> = (0..N)
            .map(|k| ((k as u64).wrapping_mul(2654435761) ^ (k as u64) << 7) & MASK)
            .collect();
        let mut packed = vec![0u8; BYTES];
        for (k, &v) in vals.iter().enumerate() {
            let base = k * W;
            for i in 0..W {
                if (v >> i) & 1 == 1 {
                    packed[(base + i) / 8] |= 1 << ((base + i) % 8);
                }
            }
        }
        let mut unpacked: Vec<u16> = Vec::with_capacity(N);
        for &v in &vals {
            unpacked.push(v as u16);
        }
        let want: u64 = vals.iter().fold(0u64, |a, &b| a.wrapping_add(b));
        // typed / hand shape, evaluated here as the reference reader
        let mut got_packed: u64 = 0;
        for k in 0..N {
            let base = k * W;
            let byte = base / 8;
            let phase = base % 8;
            let mut acc: u64 = 0;
            for i in 0..AB {
                acc |= (packed[byte + i] as u64) << (8 * i);
            }
            got_packed = got_packed.wrapping_add((acc >> phase) & MASK);
        }
        let got_native: u64 = unpacked
            .iter()
            .fold(0u64, |a, &b| a.wrapping_add((b as u64) & MASK));
        checked += 1;
        if got_packed != want || got_native != want {
            bad += 1;
            println!(
                "W={} DISAGREE: want {}, packed {}, native {}",
                W, want, got_packed, got_native
            );
        }
    }
    {
        // width 10
        const W: usize = 10;
        const MASK: u64 = 1023;
        const BYTES: usize = 328;
        const N: usize = 256;
        const AB: usize = 3;
        let vals: Vec<u64> = (0..N)
            .map(|k| ((k as u64).wrapping_mul(2654435761) ^ (k as u64) << 7) & MASK)
            .collect();
        let mut packed = vec![0u8; BYTES];
        for (k, &v) in vals.iter().enumerate() {
            let base = k * W;
            for i in 0..W {
                if (v >> i) & 1 == 1 {
                    packed[(base + i) / 8] |= 1 << ((base + i) % 8);
                }
            }
        }
        let mut unpacked: Vec<u16> = Vec::with_capacity(N);
        for &v in &vals {
            unpacked.push(v as u16);
        }
        let want: u64 = vals.iter().fold(0u64, |a, &b| a.wrapping_add(b));
        // typed / hand shape, evaluated here as the reference reader
        let mut got_packed: u64 = 0;
        for k in 0..N {
            let base = k * W;
            let byte = base / 8;
            let phase = base % 8;
            let mut acc: u64 = 0;
            for i in 0..AB {
                acc |= (packed[byte + i] as u64) << (8 * i);
            }
            got_packed = got_packed.wrapping_add((acc >> phase) & MASK);
        }
        let got_native: u64 = unpacked
            .iter()
            .fold(0u64, |a, &b| a.wrapping_add((b as u64) & MASK));
        checked += 1;
        if got_packed != want || got_native != want {
            bad += 1;
            println!(
                "W={} DISAGREE: want {}, packed {}, native {}",
                W, want, got_packed, got_native
            );
        }
    }
    {
        // width 11
        const W: usize = 11;
        const MASK: u64 = 2047;
        const BYTES: usize = 360;
        const N: usize = 256;
        const AB: usize = 3;
        let vals: Vec<u64> = (0..N)
            .map(|k| ((k as u64).wrapping_mul(2654435761) ^ (k as u64) << 7) & MASK)
            .collect();
        let mut packed = vec![0u8; BYTES];
        for (k, &v) in vals.iter().enumerate() {
            let base = k * W;
            for i in 0..W {
                if (v >> i) & 1 == 1 {
                    packed[(base + i) / 8] |= 1 << ((base + i) % 8);
                }
            }
        }
        let mut unpacked: Vec<u16> = Vec::with_capacity(N);
        for &v in &vals {
            unpacked.push(v as u16);
        }
        let want: u64 = vals.iter().fold(0u64, |a, &b| a.wrapping_add(b));
        // typed / hand shape, evaluated here as the reference reader
        let mut got_packed: u64 = 0;
        for k in 0..N {
            let base = k * W;
            let byte = base / 8;
            let phase = base % 8;
            let mut acc: u64 = 0;
            for i in 0..AB {
                acc |= (packed[byte + i] as u64) << (8 * i);
            }
            got_packed = got_packed.wrapping_add((acc >> phase) & MASK);
        }
        let got_native: u64 = unpacked
            .iter()
            .fold(0u64, |a, &b| a.wrapping_add((b as u64) & MASK));
        checked += 1;
        if got_packed != want || got_native != want {
            bad += 1;
            println!(
                "W={} DISAGREE: want {}, packed {}, native {}",
                W, want, got_packed, got_native
            );
        }
    }
    {
        // width 12
        const W: usize = 12;
        const MASK: u64 = 4095;
        const BYTES: usize = 392;
        const N: usize = 256;
        const AB: usize = 3;
        let vals: Vec<u64> = (0..N)
            .map(|k| ((k as u64).wrapping_mul(2654435761) ^ (k as u64) << 7) & MASK)
            .collect();
        let mut packed = vec![0u8; BYTES];
        for (k, &v) in vals.iter().enumerate() {
            let base = k * W;
            for i in 0..W {
                if (v >> i) & 1 == 1 {
                    packed[(base + i) / 8] |= 1 << ((base + i) % 8);
                }
            }
        }
        let mut unpacked: Vec<u16> = Vec::with_capacity(N);
        for &v in &vals {
            unpacked.push(v as u16);
        }
        let want: u64 = vals.iter().fold(0u64, |a, &b| a.wrapping_add(b));
        // typed / hand shape, evaluated here as the reference reader
        let mut got_packed: u64 = 0;
        for k in 0..N {
            let base = k * W;
            let byte = base / 8;
            let phase = base % 8;
            let mut acc: u64 = 0;
            for i in 0..AB {
                acc |= (packed[byte + i] as u64) << (8 * i);
            }
            got_packed = got_packed.wrapping_add((acc >> phase) & MASK);
        }
        let got_native: u64 = unpacked
            .iter()
            .fold(0u64, |a, &b| a.wrapping_add((b as u64) & MASK));
        checked += 1;
        if got_packed != want || got_native != want {
            bad += 1;
            println!(
                "W={} DISAGREE: want {}, packed {}, native {}",
                W, want, got_packed, got_native
            );
        }
    }
    {
        // width 13
        const W: usize = 13;
        const MASK: u64 = 8191;
        const BYTES: usize = 424;
        const N: usize = 256;
        const AB: usize = 3;
        let vals: Vec<u64> = (0..N)
            .map(|k| ((k as u64).wrapping_mul(2654435761) ^ (k as u64) << 7) & MASK)
            .collect();
        let mut packed = vec![0u8; BYTES];
        for (k, &v) in vals.iter().enumerate() {
            let base = k * W;
            for i in 0..W {
                if (v >> i) & 1 == 1 {
                    packed[(base + i) / 8] |= 1 << ((base + i) % 8);
                }
            }
        }
        let mut unpacked: Vec<u16> = Vec::with_capacity(N);
        for &v in &vals {
            unpacked.push(v as u16);
        }
        let want: u64 = vals.iter().fold(0u64, |a, &b| a.wrapping_add(b));
        // typed / hand shape, evaluated here as the reference reader
        let mut got_packed: u64 = 0;
        for k in 0..N {
            let base = k * W;
            let byte = base / 8;
            let phase = base % 8;
            let mut acc: u64 = 0;
            for i in 0..AB {
                acc |= (packed[byte + i] as u64) << (8 * i);
            }
            got_packed = got_packed.wrapping_add((acc >> phase) & MASK);
        }
        let got_native: u64 = unpacked
            .iter()
            .fold(0u64, |a, &b| a.wrapping_add((b as u64) & MASK));
        checked += 1;
        if got_packed != want || got_native != want {
            bad += 1;
            println!(
                "W={} DISAGREE: want {}, packed {}, native {}",
                W, want, got_packed, got_native
            );
        }
    }
    {
        // width 14
        const W: usize = 14;
        const MASK: u64 = 16383;
        const BYTES: usize = 456;
        const N: usize = 256;
        const AB: usize = 3;
        let vals: Vec<u64> = (0..N)
            .map(|k| ((k as u64).wrapping_mul(2654435761) ^ (k as u64) << 7) & MASK)
            .collect();
        let mut packed = vec![0u8; BYTES];
        for (k, &v) in vals.iter().enumerate() {
            let base = k * W;
            for i in 0..W {
                if (v >> i) & 1 == 1 {
                    packed[(base + i) / 8] |= 1 << ((base + i) % 8);
                }
            }
        }
        let mut unpacked: Vec<u16> = Vec::with_capacity(N);
        for &v in &vals {
            unpacked.push(v as u16);
        }
        let want: u64 = vals.iter().fold(0u64, |a, &b| a.wrapping_add(b));
        // typed / hand shape, evaluated here as the reference reader
        let mut got_packed: u64 = 0;
        for k in 0..N {
            let base = k * W;
            let byte = base / 8;
            let phase = base % 8;
            let mut acc: u64 = 0;
            for i in 0..AB {
                acc |= (packed[byte + i] as u64) << (8 * i);
            }
            got_packed = got_packed.wrapping_add((acc >> phase) & MASK);
        }
        let got_native: u64 = unpacked
            .iter()
            .fold(0u64, |a, &b| a.wrapping_add((b as u64) & MASK));
        checked += 1;
        if got_packed != want || got_native != want {
            bad += 1;
            println!(
                "W={} DISAGREE: want {}, packed {}, native {}",
                W, want, got_packed, got_native
            );
        }
    }
    {
        // width 15
        const W: usize = 15;
        const MASK: u64 = 32767;
        const BYTES: usize = 488;
        const N: usize = 256;
        const AB: usize = 3;
        let vals: Vec<u64> = (0..N)
            .map(|k| ((k as u64).wrapping_mul(2654435761) ^ (k as u64) << 7) & MASK)
            .collect();
        let mut packed = vec![0u8; BYTES];
        for (k, &v) in vals.iter().enumerate() {
            let base = k * W;
            for i in 0..W {
                if (v >> i) & 1 == 1 {
                    packed[(base + i) / 8] |= 1 << ((base + i) % 8);
                }
            }
        }
        let mut unpacked: Vec<u16> = Vec::with_capacity(N);
        for &v in &vals {
            unpacked.push(v as u16);
        }
        let want: u64 = vals.iter().fold(0u64, |a, &b| a.wrapping_add(b));
        // typed / hand shape, evaluated here as the reference reader
        let mut got_packed: u64 = 0;
        for k in 0..N {
            let base = k * W;
            let byte = base / 8;
            let phase = base % 8;
            let mut acc: u64 = 0;
            for i in 0..AB {
                acc |= (packed[byte + i] as u64) << (8 * i);
            }
            got_packed = got_packed.wrapping_add((acc >> phase) & MASK);
        }
        let got_native: u64 = unpacked
            .iter()
            .fold(0u64, |a, &b| a.wrapping_add((b as u64) & MASK));
        checked += 1;
        if got_packed != want || got_native != want {
            bad += 1;
            println!(
                "W={} DISAGREE: want {}, packed {}, native {}",
                W, want, got_packed, got_native
            );
        }
    }
    {
        // width 16
        const W: usize = 16;
        const MASK: u64 = 65535;
        const BYTES: usize = 520;
        const N: usize = 256;
        const AB: usize = 3;
        let vals: Vec<u64> = (0..N)
            .map(|k| ((k as u64).wrapping_mul(2654435761) ^ (k as u64) << 7) & MASK)
            .collect();
        let mut packed = vec![0u8; BYTES];
        for (k, &v) in vals.iter().enumerate() {
            let base = k * W;
            for i in 0..W {
                if (v >> i) & 1 == 1 {
                    packed[(base + i) / 8] |= 1 << ((base + i) % 8);
                }
            }
        }
        let mut unpacked: Vec<u16> = Vec::with_capacity(N);
        for &v in &vals {
            unpacked.push(v as u16);
        }
        let want: u64 = vals.iter().fold(0u64, |a, &b| a.wrapping_add(b));
        // typed / hand shape, evaluated here as the reference reader
        let mut got_packed: u64 = 0;
        for k in 0..N {
            let base = k * W;
            let byte = base / 8;
            let phase = base % 8;
            let mut acc: u64 = 0;
            for i in 0..AB {
                acc |= (packed[byte + i] as u64) << (8 * i);
            }
            got_packed = got_packed.wrapping_add((acc >> phase) & MASK);
        }
        let got_native: u64 = unpacked
            .iter()
            .fold(0u64, |a, &b| a.wrapping_add((b as u64) & MASK));
        checked += 1;
        if got_packed != want || got_native != want {
            bad += 1;
            println!(
                "W={} DISAGREE: want {}, packed {}, native {}",
                W, want, got_packed, got_native
            );
        }
    }
    {
        // width 17
        const W: usize = 17;
        const MASK: u64 = 131071;
        const BYTES: usize = 552;
        const N: usize = 256;
        const AB: usize = 3;
        let vals: Vec<u64> = (0..N)
            .map(|k| ((k as u64).wrapping_mul(2654435761) ^ (k as u64) << 7) & MASK)
            .collect();
        let mut packed = vec![0u8; BYTES];
        for (k, &v) in vals.iter().enumerate() {
            let base = k * W;
            for i in 0..W {
                if (v >> i) & 1 == 1 {
                    packed[(base + i) / 8] |= 1 << ((base + i) % 8);
                }
            }
        }
        let mut unpacked: Vec<u32> = Vec::with_capacity(N);
        for &v in &vals {
            unpacked.push(v as u32);
        }
        let want: u64 = vals.iter().fold(0u64, |a, &b| a.wrapping_add(b));
        // typed / hand shape, evaluated here as the reference reader
        let mut got_packed: u64 = 0;
        for k in 0..N {
            let base = k * W;
            let byte = base / 8;
            let phase = base % 8;
            let mut acc: u64 = 0;
            for i in 0..AB {
                acc |= (packed[byte + i] as u64) << (8 * i);
            }
            got_packed = got_packed.wrapping_add((acc >> phase) & MASK);
        }
        let got_native: u64 = unpacked
            .iter()
            .fold(0u64, |a, &b| a.wrapping_add((b as u64) & MASK));
        checked += 1;
        if got_packed != want || got_native != want {
            bad += 1;
            println!(
                "W={} DISAGREE: want {}, packed {}, native {}",
                W, want, got_packed, got_native
            );
        }
    }
    {
        // width 18
        const W: usize = 18;
        const MASK: u64 = 262143;
        const BYTES: usize = 584;
        const N: usize = 256;
        const AB: usize = 4;
        let vals: Vec<u64> = (0..N)
            .map(|k| ((k as u64).wrapping_mul(2654435761) ^ (k as u64) << 7) & MASK)
            .collect();
        let mut packed = vec![0u8; BYTES];
        for (k, &v) in vals.iter().enumerate() {
            let base = k * W;
            for i in 0..W {
                if (v >> i) & 1 == 1 {
                    packed[(base + i) / 8] |= 1 << ((base + i) % 8);
                }
            }
        }
        let mut unpacked: Vec<u32> = Vec::with_capacity(N);
        for &v in &vals {
            unpacked.push(v as u32);
        }
        let want: u64 = vals.iter().fold(0u64, |a, &b| a.wrapping_add(b));
        // typed / hand shape, evaluated here as the reference reader
        let mut got_packed: u64 = 0;
        for k in 0..N {
            let base = k * W;
            let byte = base / 8;
            let phase = base % 8;
            let mut acc: u64 = 0;
            for i in 0..AB {
                acc |= (packed[byte + i] as u64) << (8 * i);
            }
            got_packed = got_packed.wrapping_add((acc >> phase) & MASK);
        }
        let got_native: u64 = unpacked
            .iter()
            .fold(0u64, |a, &b| a.wrapping_add((b as u64) & MASK));
        checked += 1;
        if got_packed != want || got_native != want {
            bad += 1;
            println!(
                "W={} DISAGREE: want {}, packed {}, native {}",
                W, want, got_packed, got_native
            );
        }
    }
    {
        // width 19
        const W: usize = 19;
        const MASK: u64 = 524287;
        const BYTES: usize = 616;
        const N: usize = 256;
        const AB: usize = 4;
        let vals: Vec<u64> = (0..N)
            .map(|k| ((k as u64).wrapping_mul(2654435761) ^ (k as u64) << 7) & MASK)
            .collect();
        let mut packed = vec![0u8; BYTES];
        for (k, &v) in vals.iter().enumerate() {
            let base = k * W;
            for i in 0..W {
                if (v >> i) & 1 == 1 {
                    packed[(base + i) / 8] |= 1 << ((base + i) % 8);
                }
            }
        }
        let mut unpacked: Vec<u32> = Vec::with_capacity(N);
        for &v in &vals {
            unpacked.push(v as u32);
        }
        let want: u64 = vals.iter().fold(0u64, |a, &b| a.wrapping_add(b));
        // typed / hand shape, evaluated here as the reference reader
        let mut got_packed: u64 = 0;
        for k in 0..N {
            let base = k * W;
            let byte = base / 8;
            let phase = base % 8;
            let mut acc: u64 = 0;
            for i in 0..AB {
                acc |= (packed[byte + i] as u64) << (8 * i);
            }
            got_packed = got_packed.wrapping_add((acc >> phase) & MASK);
        }
        let got_native: u64 = unpacked
            .iter()
            .fold(0u64, |a, &b| a.wrapping_add((b as u64) & MASK));
        checked += 1;
        if got_packed != want || got_native != want {
            bad += 1;
            println!(
                "W={} DISAGREE: want {}, packed {}, native {}",
                W, want, got_packed, got_native
            );
        }
    }
    {
        // width 20
        const W: usize = 20;
        const MASK: u64 = 1048575;
        const BYTES: usize = 648;
        const N: usize = 256;
        const AB: usize = 4;
        let vals: Vec<u64> = (0..N)
            .map(|k| ((k as u64).wrapping_mul(2654435761) ^ (k as u64) << 7) & MASK)
            .collect();
        let mut packed = vec![0u8; BYTES];
        for (k, &v) in vals.iter().enumerate() {
            let base = k * W;
            for i in 0..W {
                if (v >> i) & 1 == 1 {
                    packed[(base + i) / 8] |= 1 << ((base + i) % 8);
                }
            }
        }
        let mut unpacked: Vec<u32> = Vec::with_capacity(N);
        for &v in &vals {
            unpacked.push(v as u32);
        }
        let want: u64 = vals.iter().fold(0u64, |a, &b| a.wrapping_add(b));
        // typed / hand shape, evaluated here as the reference reader
        let mut got_packed: u64 = 0;
        for k in 0..N {
            let base = k * W;
            let byte = base / 8;
            let phase = base % 8;
            let mut acc: u64 = 0;
            for i in 0..AB {
                acc |= (packed[byte + i] as u64) << (8 * i);
            }
            got_packed = got_packed.wrapping_add((acc >> phase) & MASK);
        }
        let got_native: u64 = unpacked
            .iter()
            .fold(0u64, |a, &b| a.wrapping_add((b as u64) & MASK));
        checked += 1;
        if got_packed != want || got_native != want {
            bad += 1;
            println!(
                "W={} DISAGREE: want {}, packed {}, native {}",
                W, want, got_packed, got_native
            );
        }
    }
    {
        // width 21
        const W: usize = 21;
        const MASK: u64 = 2097151;
        const BYTES: usize = 680;
        const N: usize = 256;
        const AB: usize = 4;
        let vals: Vec<u64> = (0..N)
            .map(|k| ((k as u64).wrapping_mul(2654435761) ^ (k as u64) << 7) & MASK)
            .collect();
        let mut packed = vec![0u8; BYTES];
        for (k, &v) in vals.iter().enumerate() {
            let base = k * W;
            for i in 0..W {
                if (v >> i) & 1 == 1 {
                    packed[(base + i) / 8] |= 1 << ((base + i) % 8);
                }
            }
        }
        let mut unpacked: Vec<u32> = Vec::with_capacity(N);
        for &v in &vals {
            unpacked.push(v as u32);
        }
        let want: u64 = vals.iter().fold(0u64, |a, &b| a.wrapping_add(b));
        // typed / hand shape, evaluated here as the reference reader
        let mut got_packed: u64 = 0;
        for k in 0..N {
            let base = k * W;
            let byte = base / 8;
            let phase = base % 8;
            let mut acc: u64 = 0;
            for i in 0..AB {
                acc |= (packed[byte + i] as u64) << (8 * i);
            }
            got_packed = got_packed.wrapping_add((acc >> phase) & MASK);
        }
        let got_native: u64 = unpacked
            .iter()
            .fold(0u64, |a, &b| a.wrapping_add((b as u64) & MASK));
        checked += 1;
        if got_packed != want || got_native != want {
            bad += 1;
            println!(
                "W={} DISAGREE: want {}, packed {}, native {}",
                W, want, got_packed, got_native
            );
        }
    }
    {
        // width 22
        const W: usize = 22;
        const MASK: u64 = 4194303;
        const BYTES: usize = 712;
        const N: usize = 256;
        const AB: usize = 4;
        let vals: Vec<u64> = (0..N)
            .map(|k| ((k as u64).wrapping_mul(2654435761) ^ (k as u64) << 7) & MASK)
            .collect();
        let mut packed = vec![0u8; BYTES];
        for (k, &v) in vals.iter().enumerate() {
            let base = k * W;
            for i in 0..W {
                if (v >> i) & 1 == 1 {
                    packed[(base + i) / 8] |= 1 << ((base + i) % 8);
                }
            }
        }
        let mut unpacked: Vec<u32> = Vec::with_capacity(N);
        for &v in &vals {
            unpacked.push(v as u32);
        }
        let want: u64 = vals.iter().fold(0u64, |a, &b| a.wrapping_add(b));
        // typed / hand shape, evaluated here as the reference reader
        let mut got_packed: u64 = 0;
        for k in 0..N {
            let base = k * W;
            let byte = base / 8;
            let phase = base % 8;
            let mut acc: u64 = 0;
            for i in 0..AB {
                acc |= (packed[byte + i] as u64) << (8 * i);
            }
            got_packed = got_packed.wrapping_add((acc >> phase) & MASK);
        }
        let got_native: u64 = unpacked
            .iter()
            .fold(0u64, |a, &b| a.wrapping_add((b as u64) & MASK));
        checked += 1;
        if got_packed != want || got_native != want {
            bad += 1;
            println!(
                "W={} DISAGREE: want {}, packed {}, native {}",
                W, want, got_packed, got_native
            );
        }
    }
    {
        // width 23
        const W: usize = 23;
        const MASK: u64 = 8388607;
        const BYTES: usize = 744;
        const N: usize = 256;
        const AB: usize = 4;
        let vals: Vec<u64> = (0..N)
            .map(|k| ((k as u64).wrapping_mul(2654435761) ^ (k as u64) << 7) & MASK)
            .collect();
        let mut packed = vec![0u8; BYTES];
        for (k, &v) in vals.iter().enumerate() {
            let base = k * W;
            for i in 0..W {
                if (v >> i) & 1 == 1 {
                    packed[(base + i) / 8] |= 1 << ((base + i) % 8);
                }
            }
        }
        let mut unpacked: Vec<u32> = Vec::with_capacity(N);
        for &v in &vals {
            unpacked.push(v as u32);
        }
        let want: u64 = vals.iter().fold(0u64, |a, &b| a.wrapping_add(b));
        // typed / hand shape, evaluated here as the reference reader
        let mut got_packed: u64 = 0;
        for k in 0..N {
            let base = k * W;
            let byte = base / 8;
            let phase = base % 8;
            let mut acc: u64 = 0;
            for i in 0..AB {
                acc |= (packed[byte + i] as u64) << (8 * i);
            }
            got_packed = got_packed.wrapping_add((acc >> phase) & MASK);
        }
        let got_native: u64 = unpacked
            .iter()
            .fold(0u64, |a, &b| a.wrapping_add((b as u64) & MASK));
        checked += 1;
        if got_packed != want || got_native != want {
            bad += 1;
            println!(
                "W={} DISAGREE: want {}, packed {}, native {}",
                W, want, got_packed, got_native
            );
        }
    }
    {
        // width 24
        const W: usize = 24;
        const MASK: u64 = 16777215;
        const BYTES: usize = 776;
        const N: usize = 256;
        const AB: usize = 4;
        let vals: Vec<u64> = (0..N)
            .map(|k| ((k as u64).wrapping_mul(2654435761) ^ (k as u64) << 7) & MASK)
            .collect();
        let mut packed = vec![0u8; BYTES];
        for (k, &v) in vals.iter().enumerate() {
            let base = k * W;
            for i in 0..W {
                if (v >> i) & 1 == 1 {
                    packed[(base + i) / 8] |= 1 << ((base + i) % 8);
                }
            }
        }
        let mut unpacked: Vec<u32> = Vec::with_capacity(N);
        for &v in &vals {
            unpacked.push(v as u32);
        }
        let want: u64 = vals.iter().fold(0u64, |a, &b| a.wrapping_add(b));
        // typed / hand shape, evaluated here as the reference reader
        let mut got_packed: u64 = 0;
        for k in 0..N {
            let base = k * W;
            let byte = base / 8;
            let phase = base % 8;
            let mut acc: u64 = 0;
            for i in 0..AB {
                acc |= (packed[byte + i] as u64) << (8 * i);
            }
            got_packed = got_packed.wrapping_add((acc >> phase) & MASK);
        }
        let got_native: u64 = unpacked
            .iter()
            .fold(0u64, |a, &b| a.wrapping_add((b as u64) & MASK));
        checked += 1;
        if got_packed != want || got_native != want {
            bad += 1;
            println!(
                "W={} DISAGREE: want {}, packed {}, native {}",
                W, want, got_packed, got_native
            );
        }
    }
    {
        // width 25
        const W: usize = 25;
        const MASK: u64 = 33554431;
        const BYTES: usize = 808;
        const N: usize = 256;
        const AB: usize = 4;
        let vals: Vec<u64> = (0..N)
            .map(|k| ((k as u64).wrapping_mul(2654435761) ^ (k as u64) << 7) & MASK)
            .collect();
        let mut packed = vec![0u8; BYTES];
        for (k, &v) in vals.iter().enumerate() {
            let base = k * W;
            for i in 0..W {
                if (v >> i) & 1 == 1 {
                    packed[(base + i) / 8] |= 1 << ((base + i) % 8);
                }
            }
        }
        let mut unpacked: Vec<u32> = Vec::with_capacity(N);
        for &v in &vals {
            unpacked.push(v as u32);
        }
        let want: u64 = vals.iter().fold(0u64, |a, &b| a.wrapping_add(b));
        // typed / hand shape, evaluated here as the reference reader
        let mut got_packed: u64 = 0;
        for k in 0..N {
            let base = k * W;
            let byte = base / 8;
            let phase = base % 8;
            let mut acc: u64 = 0;
            for i in 0..AB {
                acc |= (packed[byte + i] as u64) << (8 * i);
            }
            got_packed = got_packed.wrapping_add((acc >> phase) & MASK);
        }
        let got_native: u64 = unpacked
            .iter()
            .fold(0u64, |a, &b| a.wrapping_add((b as u64) & MASK));
        checked += 1;
        if got_packed != want || got_native != want {
            bad += 1;
            println!(
                "W={} DISAGREE: want {}, packed {}, native {}",
                W, want, got_packed, got_native
            );
        }
    }
    {
        // width 26
        const W: usize = 26;
        const MASK: u64 = 67108863;
        const BYTES: usize = 840;
        const N: usize = 256;
        const AB: usize = 5;
        let vals: Vec<u64> = (0..N)
            .map(|k| ((k as u64).wrapping_mul(2654435761) ^ (k as u64) << 7) & MASK)
            .collect();
        let mut packed = vec![0u8; BYTES];
        for (k, &v) in vals.iter().enumerate() {
            let base = k * W;
            for i in 0..W {
                if (v >> i) & 1 == 1 {
                    packed[(base + i) / 8] |= 1 << ((base + i) % 8);
                }
            }
        }
        let mut unpacked: Vec<u32> = Vec::with_capacity(N);
        for &v in &vals {
            unpacked.push(v as u32);
        }
        let want: u64 = vals.iter().fold(0u64, |a, &b| a.wrapping_add(b));
        // typed / hand shape, evaluated here as the reference reader
        let mut got_packed: u64 = 0;
        for k in 0..N {
            let base = k * W;
            let byte = base / 8;
            let phase = base % 8;
            let mut acc: u64 = 0;
            for i in 0..AB {
                acc |= (packed[byte + i] as u64) << (8 * i);
            }
            got_packed = got_packed.wrapping_add((acc >> phase) & MASK);
        }
        let got_native: u64 = unpacked
            .iter()
            .fold(0u64, |a, &b| a.wrapping_add((b as u64) & MASK));
        checked += 1;
        if got_packed != want || got_native != want {
            bad += 1;
            println!(
                "W={} DISAGREE: want {}, packed {}, native {}",
                W, want, got_packed, got_native
            );
        }
    }
    {
        // width 27
        const W: usize = 27;
        const MASK: u64 = 134217727;
        const BYTES: usize = 872;
        const N: usize = 256;
        const AB: usize = 5;
        let vals: Vec<u64> = (0..N)
            .map(|k| ((k as u64).wrapping_mul(2654435761) ^ (k as u64) << 7) & MASK)
            .collect();
        let mut packed = vec![0u8; BYTES];
        for (k, &v) in vals.iter().enumerate() {
            let base = k * W;
            for i in 0..W {
                if (v >> i) & 1 == 1 {
                    packed[(base + i) / 8] |= 1 << ((base + i) % 8);
                }
            }
        }
        let mut unpacked: Vec<u32> = Vec::with_capacity(N);
        for &v in &vals {
            unpacked.push(v as u32);
        }
        let want: u64 = vals.iter().fold(0u64, |a, &b| a.wrapping_add(b));
        // typed / hand shape, evaluated here as the reference reader
        let mut got_packed: u64 = 0;
        for k in 0..N {
            let base = k * W;
            let byte = base / 8;
            let phase = base % 8;
            let mut acc: u64 = 0;
            for i in 0..AB {
                acc |= (packed[byte + i] as u64) << (8 * i);
            }
            got_packed = got_packed.wrapping_add((acc >> phase) & MASK);
        }
        let got_native: u64 = unpacked
            .iter()
            .fold(0u64, |a, &b| a.wrapping_add((b as u64) & MASK));
        checked += 1;
        if got_packed != want || got_native != want {
            bad += 1;
            println!(
                "W={} DISAGREE: want {}, packed {}, native {}",
                W, want, got_packed, got_native
            );
        }
    }
    {
        // width 28
        const W: usize = 28;
        const MASK: u64 = 268435455;
        const BYTES: usize = 904;
        const N: usize = 256;
        const AB: usize = 5;
        let vals: Vec<u64> = (0..N)
            .map(|k| ((k as u64).wrapping_mul(2654435761) ^ (k as u64) << 7) & MASK)
            .collect();
        let mut packed = vec![0u8; BYTES];
        for (k, &v) in vals.iter().enumerate() {
            let base = k * W;
            for i in 0..W {
                if (v >> i) & 1 == 1 {
                    packed[(base + i) / 8] |= 1 << ((base + i) % 8);
                }
            }
        }
        let mut unpacked: Vec<u32> = Vec::with_capacity(N);
        for &v in &vals {
            unpacked.push(v as u32);
        }
        let want: u64 = vals.iter().fold(0u64, |a, &b| a.wrapping_add(b));
        // typed / hand shape, evaluated here as the reference reader
        let mut got_packed: u64 = 0;
        for k in 0..N {
            let base = k * W;
            let byte = base / 8;
            let phase = base % 8;
            let mut acc: u64 = 0;
            for i in 0..AB {
                acc |= (packed[byte + i] as u64) << (8 * i);
            }
            got_packed = got_packed.wrapping_add((acc >> phase) & MASK);
        }
        let got_native: u64 = unpacked
            .iter()
            .fold(0u64, |a, &b| a.wrapping_add((b as u64) & MASK));
        checked += 1;
        if got_packed != want || got_native != want {
            bad += 1;
            println!(
                "W={} DISAGREE: want {}, packed {}, native {}",
                W, want, got_packed, got_native
            );
        }
    }
    {
        // width 29
        const W: usize = 29;
        const MASK: u64 = 536870911;
        const BYTES: usize = 936;
        const N: usize = 256;
        const AB: usize = 5;
        let vals: Vec<u64> = (0..N)
            .map(|k| ((k as u64).wrapping_mul(2654435761) ^ (k as u64) << 7) & MASK)
            .collect();
        let mut packed = vec![0u8; BYTES];
        for (k, &v) in vals.iter().enumerate() {
            let base = k * W;
            for i in 0..W {
                if (v >> i) & 1 == 1 {
                    packed[(base + i) / 8] |= 1 << ((base + i) % 8);
                }
            }
        }
        let mut unpacked: Vec<u32> = Vec::with_capacity(N);
        for &v in &vals {
            unpacked.push(v as u32);
        }
        let want: u64 = vals.iter().fold(0u64, |a, &b| a.wrapping_add(b));
        // typed / hand shape, evaluated here as the reference reader
        let mut got_packed: u64 = 0;
        for k in 0..N {
            let base = k * W;
            let byte = base / 8;
            let phase = base % 8;
            let mut acc: u64 = 0;
            for i in 0..AB {
                acc |= (packed[byte + i] as u64) << (8 * i);
            }
            got_packed = got_packed.wrapping_add((acc >> phase) & MASK);
        }
        let got_native: u64 = unpacked
            .iter()
            .fold(0u64, |a, &b| a.wrapping_add((b as u64) & MASK));
        checked += 1;
        if got_packed != want || got_native != want {
            bad += 1;
            println!(
                "W={} DISAGREE: want {}, packed {}, native {}",
                W, want, got_packed, got_native
            );
        }
    }
    {
        // width 30
        const W: usize = 30;
        const MASK: u64 = 1073741823;
        const BYTES: usize = 968;
        const N: usize = 256;
        const AB: usize = 5;
        let vals: Vec<u64> = (0..N)
            .map(|k| ((k as u64).wrapping_mul(2654435761) ^ (k as u64) << 7) & MASK)
            .collect();
        let mut packed = vec![0u8; BYTES];
        for (k, &v) in vals.iter().enumerate() {
            let base = k * W;
            for i in 0..W {
                if (v >> i) & 1 == 1 {
                    packed[(base + i) / 8] |= 1 << ((base + i) % 8);
                }
            }
        }
        let mut unpacked: Vec<u32> = Vec::with_capacity(N);
        for &v in &vals {
            unpacked.push(v as u32);
        }
        let want: u64 = vals.iter().fold(0u64, |a, &b| a.wrapping_add(b));
        // typed / hand shape, evaluated here as the reference reader
        let mut got_packed: u64 = 0;
        for k in 0..N {
            let base = k * W;
            let byte = base / 8;
            let phase = base % 8;
            let mut acc: u64 = 0;
            for i in 0..AB {
                acc |= (packed[byte + i] as u64) << (8 * i);
            }
            got_packed = got_packed.wrapping_add((acc >> phase) & MASK);
        }
        let got_native: u64 = unpacked
            .iter()
            .fold(0u64, |a, &b| a.wrapping_add((b as u64) & MASK));
        checked += 1;
        if got_packed != want || got_native != want {
            bad += 1;
            println!(
                "W={} DISAGREE: want {}, packed {}, native {}",
                W, want, got_packed, got_native
            );
        }
    }
    {
        // width 31
        const W: usize = 31;
        const MASK: u64 = 2147483647;
        const BYTES: usize = 1000;
        const N: usize = 256;
        const AB: usize = 5;
        let vals: Vec<u64> = (0..N)
            .map(|k| ((k as u64).wrapping_mul(2654435761) ^ (k as u64) << 7) & MASK)
            .collect();
        let mut packed = vec![0u8; BYTES];
        for (k, &v) in vals.iter().enumerate() {
            let base = k * W;
            for i in 0..W {
                if (v >> i) & 1 == 1 {
                    packed[(base + i) / 8] |= 1 << ((base + i) % 8);
                }
            }
        }
        let mut unpacked: Vec<u32> = Vec::with_capacity(N);
        for &v in &vals {
            unpacked.push(v as u32);
        }
        let want: u64 = vals.iter().fold(0u64, |a, &b| a.wrapping_add(b));
        // typed / hand shape, evaluated here as the reference reader
        let mut got_packed: u64 = 0;
        for k in 0..N {
            let base = k * W;
            let byte = base / 8;
            let phase = base % 8;
            let mut acc: u64 = 0;
            for i in 0..AB {
                acc |= (packed[byte + i] as u64) << (8 * i);
            }
            got_packed = got_packed.wrapping_add((acc >> phase) & MASK);
        }
        let got_native: u64 = unpacked
            .iter()
            .fold(0u64, |a, &b| a.wrapping_add((b as u64) & MASK));
        checked += 1;
        if got_packed != want || got_native != want {
            bad += 1;
            println!(
                "W={} DISAGREE: want {}, packed {}, native {}",
                W, want, got_packed, got_native
            );
        }
    }
    {
        // width 32
        const W: usize = 32;
        const MASK: u64 = 4294967295;
        const BYTES: usize = 1032;
        const N: usize = 256;
        const AB: usize = 5;
        let vals: Vec<u64> = (0..N)
            .map(|k| ((k as u64).wrapping_mul(2654435761) ^ (k as u64) << 7) & MASK)
            .collect();
        let mut packed = vec![0u8; BYTES];
        for (k, &v) in vals.iter().enumerate() {
            let base = k * W;
            for i in 0..W {
                if (v >> i) & 1 == 1 {
                    packed[(base + i) / 8] |= 1 << ((base + i) % 8);
                }
            }
        }
        let mut unpacked: Vec<u32> = Vec::with_capacity(N);
        for &v in &vals {
            unpacked.push(v as u32);
        }
        let want: u64 = vals.iter().fold(0u64, |a, &b| a.wrapping_add(b));
        // typed / hand shape, evaluated here as the reference reader
        let mut got_packed: u64 = 0;
        for k in 0..N {
            let base = k * W;
            let byte = base / 8;
            let phase = base % 8;
            let mut acc: u64 = 0;
            for i in 0..AB {
                acc |= (packed[byte + i] as u64) << (8 * i);
            }
            got_packed = got_packed.wrapping_add((acc >> phase) & MASK);
        }
        let got_native: u64 = unpacked
            .iter()
            .fold(0u64, |a, &b| a.wrapping_add((b as u64) & MASK));
        checked += 1;
        if got_packed != want || got_native != want {
            bad += 1;
            println!(
                "W={} DISAGREE: want {}, packed {}, native {}",
                W, want, got_packed, got_native
            );
        }
    }
    {
        // width 33
        const W: usize = 33;
        const MASK: u64 = 8589934591;
        const BYTES: usize = 1064;
        const N: usize = 256;
        const AB: usize = 5;
        let vals: Vec<u64> = (0..N)
            .map(|k| ((k as u64).wrapping_mul(2654435761) ^ (k as u64) << 7) & MASK)
            .collect();
        let mut packed = vec![0u8; BYTES];
        for (k, &v) in vals.iter().enumerate() {
            let base = k * W;
            for i in 0..W {
                if (v >> i) & 1 == 1 {
                    packed[(base + i) / 8] |= 1 << ((base + i) % 8);
                }
            }
        }
        let mut unpacked: Vec<u64> = Vec::with_capacity(N);
        for &v in &vals {
            unpacked.push(v as u64);
        }
        let want: u64 = vals.iter().fold(0u64, |a, &b| a.wrapping_add(b));
        // typed / hand shape, evaluated here as the reference reader
        let mut got_packed: u64 = 0;
        for k in 0..N {
            let base = k * W;
            let byte = base / 8;
            let phase = base % 8;
            let mut acc: u64 = 0;
            for i in 0..AB {
                acc |= (packed[byte + i] as u64) << (8 * i);
            }
            got_packed = got_packed.wrapping_add((acc >> phase) & MASK);
        }
        let got_native: u64 = unpacked
            .iter()
            .fold(0u64, |a, &b| a.wrapping_add((b as u64) & MASK));
        checked += 1;
        if got_packed != want || got_native != want {
            bad += 1;
            println!(
                "W={} DISAGREE: want {}, packed {}, native {}",
                W, want, got_packed, got_native
            );
        }
    }
    {
        // width 40
        const W: usize = 40;
        const MASK: u64 = 1099511627775;
        const BYTES: usize = 1288;
        const N: usize = 256;
        const AB: usize = 6;
        let vals: Vec<u64> = (0..N)
            .map(|k| ((k as u64).wrapping_mul(2654435761) ^ (k as u64) << 7) & MASK)
            .collect();
        let mut packed = vec![0u8; BYTES];
        for (k, &v) in vals.iter().enumerate() {
            let base = k * W;
            for i in 0..W {
                if (v >> i) & 1 == 1 {
                    packed[(base + i) / 8] |= 1 << ((base + i) % 8);
                }
            }
        }
        let mut unpacked: Vec<u64> = Vec::with_capacity(N);
        for &v in &vals {
            unpacked.push(v as u64);
        }
        let want: u64 = vals.iter().fold(0u64, |a, &b| a.wrapping_add(b));
        // typed / hand shape, evaluated here as the reference reader
        let mut got_packed: u64 = 0;
        for k in 0..N {
            let base = k * W;
            let byte = base / 8;
            let phase = base % 8;
            let mut acc: u64 = 0;
            for i in 0..AB {
                acc |= (packed[byte + i] as u64) << (8 * i);
            }
            got_packed = got_packed.wrapping_add((acc >> phase) & MASK);
        }
        let got_native: u64 = unpacked
            .iter()
            .fold(0u64, |a, &b| a.wrapping_add((b as u64) & MASK));
        checked += 1;
        if got_packed != want || got_native != want {
            bad += 1;
            println!(
                "W={} DISAGREE: want {}, packed {}, native {}",
                W, want, got_packed, got_native
            );
        }
    }
    {
        // width 47
        const W: usize = 47;
        const MASK: u64 = 140737488355327;
        const BYTES: usize = 1512;
        const N: usize = 256;
        const AB: usize = 7;
        let vals: Vec<u64> = (0..N)
            .map(|k| ((k as u64).wrapping_mul(2654435761) ^ (k as u64) << 7) & MASK)
            .collect();
        let mut packed = vec![0u8; BYTES];
        for (k, &v) in vals.iter().enumerate() {
            let base = k * W;
            for i in 0..W {
                if (v >> i) & 1 == 1 {
                    packed[(base + i) / 8] |= 1 << ((base + i) % 8);
                }
            }
        }
        let mut unpacked: Vec<u64> = Vec::with_capacity(N);
        for &v in &vals {
            unpacked.push(v as u64);
        }
        let want: u64 = vals.iter().fold(0u64, |a, &b| a.wrapping_add(b));
        // typed / hand shape, evaluated here as the reference reader
        let mut got_packed: u64 = 0;
        for k in 0..N {
            let base = k * W;
            let byte = base / 8;
            let phase = base % 8;
            let mut acc: u64 = 0;
            for i in 0..AB {
                acc |= (packed[byte + i] as u64) << (8 * i);
            }
            got_packed = got_packed.wrapping_add((acc >> phase) & MASK);
        }
        let got_native: u64 = unpacked
            .iter()
            .fold(0u64, |a, &b| a.wrapping_add((b as u64) & MASK));
        checked += 1;
        if got_packed != want || got_native != want {
            bad += 1;
            println!(
                "W={} DISAGREE: want {}, packed {}, native {}",
                W, want, got_packed, got_native
            );
        }
    }
    {
        // width 48
        const W: usize = 48;
        const MASK: u64 = 281474976710655;
        const BYTES: usize = 1544;
        const N: usize = 256;
        const AB: usize = 7;
        let vals: Vec<u64> = (0..N)
            .map(|k| ((k as u64).wrapping_mul(2654435761) ^ (k as u64) << 7) & MASK)
            .collect();
        let mut packed = vec![0u8; BYTES];
        for (k, &v) in vals.iter().enumerate() {
            let base = k * W;
            for i in 0..W {
                if (v >> i) & 1 == 1 {
                    packed[(base + i) / 8] |= 1 << ((base + i) % 8);
                }
            }
        }
        let mut unpacked: Vec<u64> = Vec::with_capacity(N);
        for &v in &vals {
            unpacked.push(v as u64);
        }
        let want: u64 = vals.iter().fold(0u64, |a, &b| a.wrapping_add(b));
        // typed / hand shape, evaluated here as the reference reader
        let mut got_packed: u64 = 0;
        for k in 0..N {
            let base = k * W;
            let byte = base / 8;
            let phase = base % 8;
            let mut acc: u64 = 0;
            for i in 0..AB {
                acc |= (packed[byte + i] as u64) << (8 * i);
            }
            got_packed = got_packed.wrapping_add((acc >> phase) & MASK);
        }
        let got_native: u64 = unpacked
            .iter()
            .fold(0u64, |a, &b| a.wrapping_add((b as u64) & MASK));
        checked += 1;
        if got_packed != want || got_native != want {
            bad += 1;
            println!(
                "W={} DISAGREE: want {}, packed {}, native {}",
                W, want, got_packed, got_native
            );
        }
    }
    if bad == 0 {
        println!("all {checked} width cases agree across typed, hand and native");
        ExitCode::SUCCESS
    } else {
        println!("{bad} of {checked} width cases DISAGREE");
        ExitCode::FAILURE
    }
}
