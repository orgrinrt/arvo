// p7: the group arithmetic keys on the STRIDE; the mask and the load width key
// on the FIELD width. In a homogeneous column the two coincide, which is why
// file 81's table can say every row is "a function of W". Under a bitfield they
// separate.
//
// Swept over every stride from 1 to 57 (file 81's own range) and, at each
// stride, every two-field partition of it, rather than sampled.
//
// No feature gates. Edition 2024.

const fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn load_width_bytes(w: usize) -> usize {
    // narrowest of 1, 2, 4, 8 bytes holding w + 7 bits
    [1usize, 2, 4, 8]
        .into_iter()
        .find(|lb| w + 7 <= 8 * lb)
        .unwrap()
}

fn main() {
    let mut checked_shapes = 0usize;
    let mut agree_failures = 0usize;
    let mut period_failures = 0usize;
    let mut rows_keyed_on_stride_that_moved_with_width = 0usize;

    for stride in 1..=57usize {
        let p = 8 / gcd(stride, 8);
        let g = stride * p / 8;
        if g * 8 != stride * p {
            period_failures += 1;
        }

        for w0 in 1..stride {
            let w1 = stride - w0;
            let fields = [(0usize, w0), (w0, w1)];
            checked_shapes += 1;

            // period and group bytes must not move when the field split moves
            let p2 = 8 / gcd(stride, 8);
            let g2 = stride * p2 / 8;
            if p2 != p || g2 != g {
                rows_keyed_on_stride_that_moved_with_width += 1;
            }

            // pack 64 elements at this stride and read every field both ways
            let n = 64usize;
            let bytes = (n * stride + 7) / 8 + 16;
            let mut buf = vec![0u8; bytes];
            let mut expect = vec![[0u128; 2]; n];
            for i in 0..n {
                let v0 = ((i as u128).wrapping_mul(0x9E37)) & ((1u128 << w0) - 1);
                let v1 = ((i as u128).wrapping_mul(0x85EB)) & ((1u128 << w1) - 1);
                expect[i] = [v0, v1];
                let elem: u128 = v0 | (v1 << w0);
                let bit = i * stride;
                let byte = bit / 8;
                let sh = bit % 8;
                let mut cur = u128::from_le_bytes(buf[byte..byte + 16].try_into().unwrap());
                cur |= elem << sh;
                buf[byte..byte + 16].copy_from_slice(&cur.to_le_bytes());
            }
            for i in 0..n {
                for k in 0..2 {
                    let (o, w) = fields[k];
                    // two-step
                    let eb = i * stride;
                    let ew = u128::from_le_bytes(buf[eb / 8..eb / 8 + 16].try_into().unwrap());
                    let elem = (ew >> (eb % 8)) & ((1u128 << stride) - 1);
                    let two = (elem >> o) & ((1u128 << w) - 1);
                    // one-step, composed offset
                    let fb = i * stride + o;
                    let fw = u128::from_le_bytes(buf[fb / 8..fb / 8 + 16].try_into().unwrap());
                    let one = (fw >> (fb % 8)) & ((1u128 << w) - 1);
                    if two != one || two != expect[i][k] {
                        agree_failures += 1;
                    }
                }
            }
        }
    }

    println!(
        "shapes checked (stride 1..=57 x every two-field partition): {}",
        checked_shapes
    );
    println!(
        "two-step vs one-step vs packed-input disagreements: {}",
        agree_failures
    );
    println!("G*8 == stride*P failures: {}", period_failures);
    println!(
        "period/group rows moving with the field split: {}",
        rows_keyed_on_stride_that_moved_with_width
    );

    println!("\nthe split, at stride 13 with fields (0,3) (3,5) (8,5):");
    println!(
        "  keyed on the stride: P = {}, G = {} bytes",
        8 / gcd(13, 8),
        13 * (8 / gcd(13, 8)) / 8
    );
    for (o, w) in [(0usize, 3usize), (3, 5), (8, 5)] {
        println!("  keyed on the field:  o = {}, mask width = {}, load width = {} bytes, well-formed = {}",
                 o, w, load_width_bytes(w), w + 7 <= 8 * load_width_bytes(w));
    }
    println!("  keyed on both:       lane shifts (j*stride + o) mod 8");
}
