//! This is a library for calculating the ideal combination of glass colors to color a Minecraft
//! beacon.
//!
//! # Features
//! - **Color Approxmation**: Find the best combination of glass colors to approximate a given
//!     color
//! - **Pane Calculation**: Calculate the color of the beacon beam with some given glass colors
//! - **Standard Colors**: Acces the standard Minecraft colors
//! ## Color Approxmation
//! `find_combination_default` and it's custom variants are used for approximating a color using
//! the limited palette of Minecraft colors.
//! ```
//! use beacon_calculator::find_combination_default;
//!
//! find_combination_default([254, 44, 84]);
//!
//! ```
//! Returns:
//! ```
//! Some(Panes {
//!     panes: [
//!         "pink",
//!         "magenta",
//!         "orange",
//!         "pink",
//!         "pink",
//!         "red",
//!     ],
//!     distance: 7.9557647705078125,
//!     color: PreciseRGB {
//!         red: 208.5,
//!         green: 89.90625,
//!         blue: 95.78125,
//!     },
//! })
//! ```
//! ### Performance Considerations
//! When using the custom variants pay attention to what limits you set, as they can get out of
//! hand rather quickly. A `depth` and `cutoff` of 7 already take longer than 30 seconds on my
//! machine for example. The accuracy doesn't get much better with values over 6 anyway.
//!
//!
//! ## Get color from panes
//! `calculate_color_from_panes_default` and it's custom counterpart are used for getting the color
//! of a `Vec<String>` representing a list of glass colors. Order is important here!
//! ```
//! use beacon_calculator::calculate_color_from_panes_default;
//!
//! calculate_color_from_panes_default(&[
//!     "red".to_string(),
//!     "green".to_string(),
//!     "red".to_string(),
//! ]);
//! ```
//! Returns:
//! ```
//! PreciseRGB {
//!     red: 155.5,
//!     green: 65.5,
//!     blue: 34.0,
//! }
//! ```
//! ## Standard Colors
//! `get_standard_colors` just gets the standard Minecraft colors in form of a `HashMap<String,
//! [u8;3]>`

use color_utils::{calculate_distance, RGB};
use core::f64;
use std::{
    cmp::Ordering,
    collections::HashMap,
    sync::mpsc::channel,
    thread::{self},
};
mod color_utils;

/// Calculates the color of some glass panes using custom colors
pub use color_utils::calculate_color_from_panes as calculate_color_from_panes_custom;

/// Represents a RGB color using f64
pub use color_utils::PreciseRGB;

/// Represents some glass panes, their DE2000 distance to the target and their calculated color
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Panes {
    panes: Vec<String>,
    distance: f64,
    color: PreciseRGB,
}

impl PartialEq for Panes {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for Panes {}

impl PartialOrd for Panes {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Panes {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.partial_cmp(&other.distance).unwrap()
    }
}
impl Panes {
    fn from_panes_vec(
        panes: &[String],
        available_colors: &HashMap<String, RGB>,
        target: RGB,
    ) -> Self {
        let color = calculate_color_from_panes_custom(panes, available_colors);
        let dist = calculate_distance(color, target);
        Self {
            panes: panes.to_vec(),
            distance: dist,
            color,
        }
    }
}
/// Gets the standard Minecraft colors
pub fn get_standard_colors() -> HashMap<String, [u8; 3]> {
    HashMap::from([
        ("white".to_string(), [249, 255, 254]),
        ("orange".to_string(), [249, 128, 29]),
        ("magenta".to_string(), [199, 78, 189]),
        ("light_blue".to_string(), [58, 179, 218]),
        ("yellow".to_string(), [254, 216, 61]),
        ("lime".to_string(), [128, 199, 31]),
        ("pink".to_string(), [243, 139, 170]),
        ("gray".to_string(), [71, 79, 82]),
        ("light_gray".to_string(), [157, 157, 151]),
        ("cyan".to_string(), [22, 156, 156]),
        ("purple".to_string(), [137, 50, 184]),
        ("blue".to_string(), [60, 68, 170]),
        ("brown".to_string(), [131, 84, 50]),
        ("green".to_string(), [94, 124, 22]),
        ("red".to_string(), [176, 46, 38]),
        ("black".to_string(), [29, 29, 33]),
    ])
}

/// Calculates the color of some glass panes using the default Minecraft colors
pub fn calculate_color_from_panes_default(panes: &[String]) -> PreciseRGB {
    calculate_color_from_panes_custom(panes, &convert_colors(&get_standard_colors()))
}

/// Calculates the most precise combination with the given values (incl. custom Colors)
#[allow(clippy::implicit_hasher)]
pub fn find_combination_custom_colors(
    color: [u8; 3],
    colors: &HashMap<String, [u8; 3]>,
    depth: u8,
    cutoff: u8,
) -> Option<Panes> {
    find_combination(color, &convert_colors(colors), depth, cutoff)
}

/// Calculates the most precise combination with the given values (excl. custom Colors)
pub fn find_combination_custom(color: [u8; 3], depth: u8, cutoff: u8) -> Option<Panes> {
    find_combination(
        color,
        &convert_colors(&get_standard_colors()),
        depth,
        cutoff,
    )
}

/// Calculates the most precise combination with the default values
pub fn find_combination_default(color: [u8; 3]) -> Option<Panes> {
    let depth = 6;
    let cutoff = 6;
    find_combination(
        color,
        &convert_colors(&get_standard_colors()),
        depth,
        cutoff,
    )
}

fn convert_colors(colors: &HashMap<String, [u8; 3]>) -> HashMap<String, RGB> {
    let mut new_colors = HashMap::new();

    for x in colors {
        new_colors.insert(x.0.clone(), RGB::new(*x.1));
    }
    new_colors
}

#[allow(clippy::implicit_hasher)]
fn find_combination(
    color: [u8; 3],
    colors: &HashMap<String, RGB>,
    depth: u8,
    cutoff: u8,
) -> Option<Panes> {
    let color = RGB::new(color);
    if depth == 0 {
        return None;
    }
    if usize::from(cutoff) >= colors.len() {
        return None;
    }
    let mut all = calculate_combinations_recursively(colors, 0, depth, color, cutoff, &Vec::new());
    all.sort();
    //dbg!(&all[0]);
    Some(all[0].clone())
}

fn calculate_combinations_recursively(
    colors: &HashMap<String, RGB>,
    depth: u8,
    max_depth: u8,
    target: RGB,
    cutoff: u8,
    base_panes: &[String],
) -> Vec<Panes> {
    if depth == max_depth {
        return vec![Panes::from_panes_vec(base_panes, colors, target)];
    }

    let mut possibilities = Vec::new();

    let dists = get_distances(base_panes, colors, target);
    let trimmed_dists = drop_entries(&dists, cutoff);
    for dist in trimmed_dists {
        let mut possibility: Vec<String> = base_panes.to_vec();
        possibility.push(dist.0);
        possibilities.push(possibility);
    }

    let mut collection = Vec::new();
    for possibility in possibilities {
        collection.extend(calculate_combinations_recursively(
            colors,
            depth + 1,
            max_depth,
            target,
            cutoff,
            &possibility,
        ));
    }
    collection
}

fn drop_entries(distance_pairs: &[(String, f64)], cutoff: u8) -> Vec<(String, f64)> {
    let mut distance_pairs = distance_pairs.to_owned();
    distance_pairs.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
    distance_pairs
        .into_iter()
        .take(usize::from(cutoff))
        .collect()
}

fn get_distances(
    panes: &[String],
    available_colors: &HashMap<String, RGB>,
    target: RGB,
) -> Vec<(String, f64)> {
    let (sender, reciever) = channel();
    thread::scope(|x| {
        let mut panes = panes.to_vec();
        for color in available_colors {
            let sender_clone = sender.clone();
            panes.push(color.0.clone());
            let new_panes = panes.clone();
            x.spawn(move || {
                let _ = sender_clone.send((
                    color.0.clone(),
                    calculate_distance(
                        calculate_color_from_panes_custom(&new_panes, available_colors),
                        target,
                    ),
                ));
            });
            panes.pop();
        }
        drop(sender);
    });
    let mut distance_pairs = Vec::new();
    for message in reciever {
        distance_pairs.push(message);
    }
    distance_pairs
}
