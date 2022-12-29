const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let mut missiles = STARTING_MISSILES;
    let ready: i32 = READY_AMOUNT;

    println!("Firing {} of my {} missiles...", ready, missiles);

    missiles = missiles - ready;
    println!("Firing {} missiles left", missiles);

    // ! is a macro with undefined type; NB only macro support this
    println!("mul result {}", do_stuff(3.0, 4.0));
}


fn do_stuff(qty: f64, oz: f64) -> f64 {
    qty * oz
}
