use physi_rs::units::*;

fn main() {
    let mass = 3.0 * KILOGRAM;
    let acceleration = 1.0 * METER / (SECOND * SECOND);
    let force = mass * acceleration;
    println!("f = {mass} * {acceleration} = {}", force);
}
