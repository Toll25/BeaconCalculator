use color_utils::{calculate_distance, convert_panes_to_rgb, PreciseRGB, RGB};
use core::f64;
use std::{
    cmp::Ordering,
    collections::HashMap,
    sync::mpsc::channel,
    thread::{self},
};
mod color_utils;

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
        let color = convert_panes_to_rgb(panes, available_colors);
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

pub fn convert_panes_to_rgb_default(panes: &[String]) -> PreciseRGB {
    convert_panes_to_rgb(panes, &convert_colors(&get_standard_colors()))
}

#[allow(clippy::implicit_hasher)]
pub fn calculate_color_custom_colors(
    color: [u8; 3],
    colors: &HashMap<String, [u8; 3]>,
    depth: u8,
    cutoff: u8,
) {
    calculate_color(color, &convert_colors(colors), depth, cutoff);
}

pub fn calculate_color_custom(color: [u8; 3], depth: u8, cutoff: u8) {
    calculate_color(
        color,
        &convert_colors(&get_standard_colors()),
        depth,
        cutoff,
    );
}

pub fn calculate_color_default(color: [u8; 3]) -> Option<Panes> {
    let depth = 6;
    let cutoff = 6;
    calculate_color(
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
fn calculate_color(
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
    let mut all = recursive(colors, 0, depth, color, cutoff, &Vec::new());
    all.sort();
    Some(all[0].clone())
}

fn recursive(
    available_colors: &HashMap<String, RGB>,
    depth: u8,
    target_depth: u8,
    target: RGB,
    cutoff: u8,
    base_panes: &[String],
) -> Vec<Panes> {
    if depth == target_depth {
        return vec![Panes::from_panes_vec(base_panes, available_colors, target)];
    }

    let mut end_dists = Vec::new();

    let dists = get_dists(base_panes, available_colors, target);
    let new_dists = drop_entries(&dists, cutoff);
    for dis in new_dists {
        let mut temp: Vec<String> = base_panes.to_vec();
        temp.push(dis.0);
        end_dists.push(temp);
    }

    let mut collection = Vec::new();
    for posi in end_dists {
        collection.extend(recursive(
            available_colors,
            depth + 1,
            target_depth,
            target,
            cutoff,
            &posi,
        ));
    }
    collection
}

fn drop_entries(entries: &[(String, f64)], limit: u8) -> Vec<(String, f64)> {
    let mut entries = entries.to_owned();
    entries.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
    entries.into_iter().take(usize::from(limit)).collect()
}

fn get_dists(
    panes: &[String],
    available_colors: &HashMap<String, RGB>,
    target: RGB,
) -> Vec<(String, f64)> {
    let (tx, rx) = channel();
    thread::scope(|x| {
        let mut panes = panes.to_vec();
        let mut handles = Vec::new();
        for color in available_colors {
            let tx1 = tx.clone();
            panes.push(color.0.clone());
            let new_panes = panes.clone();
            handles.push(x.spawn(move || {
                let _ = tx1.send((
                    color.0.clone(),
                    calculate_distance(convert_panes_to_rgb(&new_panes, available_colors), target),
                ));
            }));
            panes.pop();
        }
        drop(tx);
        for handle in handles {
            let _ = handle.join();
        }
    });
    let mut dists = Vec::new();
    for message in rx {
        dists.push(message);
    }
    dists
}
