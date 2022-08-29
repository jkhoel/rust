const STARTING_MISSILES: u8 = 8;
const READY_AMOUNT: u8 = 2;

fn main() {
    let mut missiles: u8 = STARTING_MISSILES;
    let ready: u8 = READY_AMOUNT;

    println!("Firing {} of my {} missiles...", ready, missiles);

    missiles -= ready;

    println!("{} missiles left", missiles);
}
