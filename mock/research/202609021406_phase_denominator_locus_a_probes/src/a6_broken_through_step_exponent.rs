// A6. The offending declaration through `smallest_step_exponent`, the derived
// quantity the subnormal story turns on. Also does not force the obligation.
mod shared;
use arvo_format::format::smallest_step_exponent;

fn main() {
    println!(
        "A6 broken/smallest_step_exponent = {}",
        smallest_step_exponent::<shared::Broken>().power()
    );
}
