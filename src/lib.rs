use color_utils::{calculate_distance, PreciseRGB, RGB};
use core::f64;
use std::{
    cmp::Ordering,
    collections::HashMap,
    sync::mpsc::channel,
    thread::{self},
};
mod color_utils;
pub use color_utils::calculate_color_from_panes;

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
        let color = calculate_color_from_panes(panes, available_colors);
        let dist = calculate_distance(color, target);
        Self {
            panes: panes.to_vec(),
            distance: dist,
            color,
        }
    }
}

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

pub fn calculate_color_from_panes_default(panes: &[String]) -> PreciseRGB {
    calculate_color_from_panes(panes, &convert_colors(&get_standard_colors()))
}

#[allow(clippy::implicit_hasher)]
pub fn calculate_color_custom_colors(
    color: [u8; 3],
    colors: &HashMap<String, [u8; 3]>,
    depth: u8,
    cutoff: u8,
) {
    find_combination(color, &convert_colors(colors), depth, cutoff);
}

pub fn find_combination_custom(color: [u8; 3], depth: u8, cutoff: u8) {
    find_combination(
        color,
        &convert_colors(&get_standard_colors()),
        depth,
        cutoff,
    );
}

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
    dbg!(&all[0]);
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
                        calculate_color_from_panes(&new_panes, available_colors),
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
