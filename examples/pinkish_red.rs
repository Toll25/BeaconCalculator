use beacon_calculator::find_combination_default;

fn main() {
    let x = find_combination_default([254, 44, 84]).unwrap();
    dbg!(x);
}
