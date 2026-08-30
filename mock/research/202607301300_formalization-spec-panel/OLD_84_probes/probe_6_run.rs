#[path = "probe_6_the_refusing_carrier_for_free.rs"]
mod p;
fn main() {
    println!(
        "Biased={} Option<Biased>={} Result<Biased,()>={}",
        p::SIZE_BIASED,
        p::SIZE_OPT_BIASED,
        p::SIZE_RES_BIASED
    );
    println!(
        "Plain={} Option<Plain>={}",
        p::SIZE_PLAIN,
        p::SIZE_OPT_PLAIN
    );
    println!(
        "column of 64: biased-refusing={}B plain-refusing={}B",
        p::COLUMN_BIASED,
        p::COLUMN_PLAIN
    );
}
