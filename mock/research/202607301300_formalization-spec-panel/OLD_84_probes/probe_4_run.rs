#[path = "probe_4_where_a_partial_operation_pays.rs"]
mod p;
fn main() {
    println!("size_of i64          = {}", p::SIZE_PLAIN);
    println!("size_of bottom (i64) = {}", p::SIZE_BOTTOM);
    println!("size_of Refusing     = {}", p::SIZE_CARRIER);
    println!("size_of Nz           = {}", p::SIZE_NZ);
    println!(
        "column of 64: plain={}B carrier={}B",
        64 * p::SIZE_PLAIN,
        64 * p::SIZE_CARRIER
    );
}
