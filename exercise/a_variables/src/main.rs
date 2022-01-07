const STARTING_MISSILES : i32 = 8;
const READY_AMOUNT : i32 = 2;

fn main() {
    let (missles, ready) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missles...", ready, missles);
    println!("Missles left: {}", missles - ready);
}
