// Probe 1: file 72's original digest probe (probe_4, part a) modelled the Hot preset's exact
// widths (13-bit fields, 16-bit container, statement P vacuous since W_F == W_S). File 83
// relabelled that model's padding as statement-C territory, not statement-P territory, and file
// 86 applied the same relabel to 72's digest chapter without building a case where BOTH tiers
// are real. This probe builds that case (the ratified Warm/Precise row: W_F=13, W_S=26, W_C=32)
// and shows the datum-keyed digest projection is a single mask to W_F, which undoes both tiers
// in one step regardless of what garbage occupies either of them, because those bits are
// discarded rather than read.

fn fnv1a(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

const W_F: u32 = 13;
const FIELDS_MASK: u32 = (1 << W_F) - 1; // 0x1FFF

// --- Scenario (a): Hot preset shape. W_F = W_S = 13, W_C = 16. Reproduces 72_probes/probe_4's
// own numbers exactly (`DATUM_MASK: u16 = 0x1FFF`). Statement P is vacuous here (no bits between
// 13 and 13); what the original probe called "padding" is bits [13, 16), which is the container
// level, not the carrier level.
mod hot_shape {
    use super::*;
    const W_S: u32 = 13;
    const W_C: u32 = 16;
    const STORED_MASK: u32 = (1 << W_S) - 1; // == FIELDS_MASK here: statement P is vacuous

    pub fn digest_raw_container(c: u32) -> u64 {
        fnv1a(&(c as u16).to_le_bytes())
    }
    pub fn digest_datum(c: u32) -> u64 {
        fnv1a(&((c & FIELDS_MASK) as u16).to_le_bytes())
    }

    pub fn run() {
        let clean: u32 = 0x1A5C & FIELDS_MASK;
        let c_dirty_container: u32 = clean | 0xE000; // dirties [13,16), the container tier only

        assert_eq!(
            clean & FIELDS_MASK,
            c_dirty_container & FIELDS_MASK,
            "same datum"
        );
        assert_eq!(
            clean & STORED_MASK,
            c_dirty_container & STORED_MASK,
            "statement P vacuous: masking to W_S changes nothing beyond masking to W_F here"
        );
        assert_ne!(
            digest_raw_container(clean),
            digest_raw_container(c_dirty_container),
            "raw digest separates same-datum values: the container-tier dirt is visible to a raw hash"
        );
        assert_eq!(
            digest_datum(clean),
            digest_datum(c_dirty_container),
            "datum-keyed digest is immune: it never reads [W_F, W_C)"
        );
        let _ = W_C;
        println!("hot_shape: reproduces 72_probes/probe_4 part (a) exactly; the dirtied region is [13,16), the container tier, not statement P");
    }
}

// --- Scenario (b): Warm/Precise preset shape, per 83:135-141's own table row. W_F = 13,
// W_S = 26 (doubled), W_C = 32 (u32 container). Statement P governs [13, 26), 13 real bits;
// statement C governs [26, 32), 6 real bits. Both tiers exist and are dirtied independently.
mod warm_shape {
    use super::*;
    const W_S: u32 = 26;
    const W_C: u32 = 32;
    const STORED_MASK: u32 = (1 << W_S) - 1; // 0x3FFFFFF

    pub fn digest_raw_container(c: u32) -> u64 {
        fnv1a(&c.to_le_bytes())
    }
    // undoes statement C only: masks to W_S, leaves statement P's region (real here) untouched
    pub fn digest_carrier(c: u32) -> u64 {
        fnv1a(&(c & STORED_MASK).to_le_bytes())
    }
    // undoes both tiers in one mask: the datum-keyed digest projection
    pub fn digest_datum(c: u32) -> u64 {
        fnv1a(&(c & FIELDS_MASK).to_le_bytes())
    }

    pub fn run() {
        let clean: u32 = 0x1A5C & FIELDS_MASK; // the datum, W_F = 13 bits, nothing above it set
        let dirty_p_only: u32 = clean | (0x1234 << W_F) & STORED_MASK; // dirty [13,26) only
        let dirty_c_only: u32 = clean | (0x2F << W_S); // dirty [26,32) only
        let dirty_both: u32 = dirty_p_only | (0x2F << W_S);

        for &dirty in &[dirty_p_only, dirty_c_only, dirty_both] {
            assert_eq!(
                clean & FIELDS_MASK,
                dirty & FIELDS_MASK,
                "same datum in every case"
            );
            assert_ne!(
                digest_raw_container(clean),
                digest_raw_container(dirty),
                "raw container digest separates same-datum values under any of the three dirt patterns"
            );
            assert_eq!(
                digest_datum(clean),
                digest_datum(dirty),
                "datum-keyed digest (mask straight to W_F) is immune to all three dirt patterns in one step"
            );
        }

        // statement P's own tier, isolated: dirtying only [26,32) must not change a carrier-level
        // (statement-P-vacuous-checked) compare, but dirtying [13,26) must.
        assert_eq!(
            digest_carrier(clean),
            digest_carrier(dirty_c_only),
            "carrier-level digest (undoes statement C only) is immune to container-tier dirt alone"
        );
        assert_ne!(
            digest_carrier(clean),
            digest_carrier(dirty_p_only),
            "carrier-level digest is NOT immune to statement-P dirt: undoing statement C alone is not enough to reach the datum"
        );

        println!("warm_shape: both tiers real (13-bit statement P, 6-bit statement C); masking to W_F undoes both in one step, independent of the intermediate carrier-level digest which only undoes statement C");
    }
}

fn main() {
    hot_shape::run();
    warm_shape::run();
}
