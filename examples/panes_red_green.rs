use beacon_calculator::calculate_color_from_panes_default;

fn main() {
    let x = calculate_color_from_panes_default(&[
        "red".to_string(),
        "green".to_string(),
        "red".to_string(),
    ]);
    dbg!(x);
}
