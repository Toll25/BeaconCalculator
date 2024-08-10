use std::collections::HashMap;

pub struct Approximation {
    distance: f64,
    final_color: [f64; 3],
    colors: Vec<String>,
}

#[allow(clippy::implicit_hasher)]
pub fn calculate_color(
    color: [u8; 3],
    colors: Option<&HashMap<&str, [u8; 3]>>,
    depth: Option<u8>,
) -> Approximation {
    let colors = colors.unwrap_or(&HashMap::from([
        ("white", [249, 255, 254]),
        ("light_gray", [157, 157, 151]),
        ("gray", [71, 79, 82]),
        ("black", [29, 29, 33]),
        ("brown", [131, 84, 50]),
        ("red", [176, 46, 38]),
        ("orange", [249, 128, 29]),
        ("yellow", [254, 216, 61]),
        ("lime", [128, 199, 31]),
        ("green", [94, 124, 22]),
        ("cyan", [22, 156, 156]),
        ("light_blue", [58, 179, 218]),
        ("blue", [60, 68, 170]),
        ("purple", [137, 50, 184]),
        ("magenta", [199, 78, 189]),
        ("pink", [243, 139, 170]),
    ]));
    let depth = depth.unwrap_or(6);

    Approximation {
        distance: 0.0,
        final_color: [0.0, 0.0, 0.0],
        colors: vec![String::new()],
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
