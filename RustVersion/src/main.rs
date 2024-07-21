use beacon::color_distance::{calculate_distance, PreciseRGB, RGB};
use crossbeam::thread;
use indicatif::ProgressBar;
use std::collections::{HashMap, HashSet};
use std::env;
use std::time::Duration;

struct Results<'a> {
    distance: f64,
    final_color: PreciseRGB,
    panes: Vec<&'a str>,
}

impl<'a> Results<'a> {
    pub fn new(distance: f64, final_color: PreciseRGB, panes: Vec<&'a str>) -> Self {
        Results {
            distance,
            final_color,
            panes,
        }
    }

    pub fn print(&self) {
        println!("Distance: {}", self.distance);
        println!("Calculated Color: {:?}", self.final_color);
        println!("Final Panes: {:?}", self.panes);
    }
}

fn main() {
    let colors: HashMap<&str, RGB> = [
        ("white", RGB::new_from_array([249, 255, 254])),
        ("light_gray", RGB::new_from_array([157, 157, 151])),
        ("gray", RGB::new_from_array([71, 79, 82])),
        ("black", RGB::new_from_array([29, 29, 33])),
        ("brown", RGB::new_from_array([131, 84, 50])),
        ("red", RGB::new_from_array([176, 46, 38])),
        ("orange", RGB::new_from_array([249, 128, 29])),
        ("yellow", RGB::new_from_array([254, 216, 61])),
        ("lime", RGB::new_from_array([128, 199, 31])),
        ("green", RGB::new_from_array([94, 124, 22])),
        ("cyan", RGB::new_from_array([22, 156, 156])),
        ("light_blue", RGB::new_from_array([58, 179, 218])),
        ("blue", RGB::new_from_array([60, 68, 170])),
        ("purple", RGB::new_from_array([137, 50, 184])),
        ("magenta", RGB::new_from_array([199, 78, 189])),
        ("pink", RGB::new_from_array([243, 139, 170])),
    ]
    .iter()
    .cloned()
    .collect();

    let red_colors = colors
        .values()
        .map(|color| color.red as f64)
        .collect::<Vec<_>>();
    let green_colors = colors
        .values()
        .map(|color| color.green as f64)
        .collect::<Vec<_>>();
    let blue_colors = colors
        .values()
        .map(|color| color.blue as f64)
        .collect::<Vec<_>>();

    let args: Vec<String> = env::args().collect();

    let without_prefix = args[1].trim_start_matches("0x");
    let z = u32::from_str_radix(without_prefix, 16).unwrap();

    let target_color = RGB::new_from_number(z);

    let only_colors = colors.values().collect::<Vec<_>>();
    let mut pruned_colors = HashSet::<&str>::new();
    while pruned_colors.len() < 7 {
        let init_colors = [
            only_colors[fastrand::usize(0..colors.len())].to_precise(),
            only_colors[fastrand::usize(0..colors.len())].to_precise(),
            only_colors[fastrand::usize(0..colors.len())].to_precise(),
            only_colors[fastrand::usize(0..colors.len())].to_precise(),
            only_colors[fastrand::usize(0..colors.len())].to_precise(),
            only_colors[fastrand::usize(0..colors.len())].to_precise(),
            only_colors[fastrand::usize(0..colors.len())].to_precise(),
            // only_colors[fastrand::usize(0..colors.len())].to_precise(),
        ];
        let mut trainable_colors = init_colors;

        let mut target_color_pre = target_color.to_precise();
        target_color_pre.red /= 255.;
        target_color_pre.green /= 255.;
        target_color_pre.blue /= 255.;


        let penalty = |w, colors: &[f64]| {
            let color_diff = colors
                .into_iter()
                .map(|color| (w as f64 - color))
                .fold(f64::INFINITY, |a, b| a.min(b))
                * (1. / 255.)
                * 0.;
            let x = color_diff;
            // println!("pen x: {x}");
            let w = w / 255.;
            if w < 0. {
                -1. + x
            } else if w > 1. {
                1. + x
            } else {
                x
            }
        };

        for _ in 0..10000 {
            let mut predicted = forward(&trainable_colors);

            // let loss = calculate_distance(predicted, target_color).powf(2.);
            // let loss_diff = 2. * calculate_distance(predicted, target_color);

            predicted.red /= 255.;
            predicted.green /= 255.;
            predicted.blue /= 255.;
            // println!("pred: {predicted:?}");

            // let loss_r = loss;
            // let loss_g = loss;
            // let loss_b = loss;

            // let loss_b_diff = loss_diff;

            let loss_r = (predicted.red - target_color_pre.red as f64).powf(2.);
            let loss_g = (predicted.green - target_color_pre.green as f64).powf(2.);
            let loss_b = (predicted.blue - target_color_pre.blue as f64).powf(2.);

            let loss_r_diff = (predicted.red - target_color_pre.red as f64) * 2.;
            let loss_g_diff = (predicted.green - target_color_pre.green as f64) * 2.;
            let loss_b_diff = (predicted.blue - target_color_pre.blue as f64) * 2.;

            let mut grad_colors = vec![PreciseRGB::default(); trainable_colors.len()];
            for (i, _color) in trainable_colors.iter().enumerate().skip(1) {
                let weight = 2u32.pow((i - 1) as u32) as f64;
                grad_colors[i].red +=
                    weight * (loss_r_diff + penalty(trainable_colors[i].red, &red_colors));
                grad_colors[i].green +=
                    weight * (loss_g_diff + penalty(trainable_colors[i].green, &green_colors));
                grad_colors[i].blue +=
                    weight * (loss_b_diff + penalty(trainable_colors[i].blue, &blue_colors));
            }
            grad_colors[0].red += loss_r_diff + penalty(trainable_colors[0].red, &red_colors);
            grad_colors[0].green += loss_g_diff + penalty(trainable_colors[0].green, &green_colors);
            grad_colors[0].blue += loss_b_diff + penalty(trainable_colors[0].blue, &blue_colors);

            let lr = 0.1;
            for (color, grad_color) in trainable_colors.iter_mut().zip(grad_colors) {
                color.red -= grad_color.red * lr;
                color.green -= grad_color.green * lr;
                color.blue -= grad_color.blue * lr;
            }
            // println!("loss: {}", loss_r + loss_g + loss_b);
            // println!("dist: {}, loss: {}", loss, loss_r + loss_g + loss_b);
            // println!("init colors: {init_colors:?}")
        }

        // println!("init colors: {trainable_colors:?}");
        // let preds = forward(&trainable_colors);
        // println!("preds: {preds:?}, targets: {target_color:?}");

        let mut results = vec![];
        for color in trainable_colors {
            let mut min_dist_color = None;
            let mut min_dist = f64::INFINITY;
            for (name, target) in &colors {
                let dist = calculate_distance(color, *target);
                if dist < min_dist {
                    min_dist = dist;
                    min_dist_color = Some((name, target));
                }
            }
            // println!("color: {:?}", min_dist_color);
            results.push(*min_dist_color.unwrap().0);
        }

        let out_color = convert_panes_to_rgb(results.clone(), colors.clone());
        let dist = calculate_distance(out_color, target_color);

        // if dist <= 11. / init_colors.len() as f64 {
        //     pruned_colors.extend(&results);
        // }

        // dynamisch anpassen lassen 
        if dist <= 6.  {
            for result in results {
                pruned_colors.insert(result);
                if pruned_colors.len() == 7 {
                    break;
                }
            }
        }

        // println!("out color: {out_color:?}, dist: {dist:?}");
    }
    println!("pruned_colors: {pruned_colors:?}, pc len: {}, all colors: {}", pruned_colors.len(), colors.len());
    // return;

    let colors = pruned_colors.iter().map(|color| (*color, colors.get(color).copied().unwrap())).collect::<HashMap<_, _>>();
    let results = find_closest_panes(target_color, colors);
    Results::print(&results)
}

fn forward(inputs: &[PreciseRGB]) -> PreciseRGB {
    let n = inputs.len();
    let mut sum_colors = PreciseRGB::new(0.0, 0.0, 0.0);
    for (i, color) in inputs.iter().enumerate().skip(1) {
        let weight = 2u32.pow((i - 1) as u32) as f64;
        sum_colors.red += weight * color.red;
        sum_colors.green += weight * color.green;
        sum_colors.blue += weight * color.blue;
    }

    let first_color = inputs[0];
    sum_colors.red += first_color.red;
    sum_colors.green += first_color.green;
    sum_colors.blue += first_color.blue;

    let scaling_factor = 1.0 / (2u32.pow((n - 1) as u32) as f64);

    PreciseRGB::new(
        scaling_factor * sum_colors.red,
        scaling_factor * sum_colors.green,
        scaling_factor * sum_colors.blue,
    )
}

fn find_closest_panes(target_color: RGB, available_colors: HashMap<&str, RGB>) -> Results {
    let mut results = vec![];

    // Create a progress bar with the total number of tasks
    let bar = ProgressBar::new_spinner();
    bar.enable_steady_tick(Duration::from_millis(10));
    bar.set_message(format!("Starting processing {}", target_color));

    thread::scope(|s| {
        for starting_color in available_colors.keys() {
            let available_colors = available_colors.clone(); // Clone to move ownership into closure

            // Spawn a thread for each item to process concurrently
            let handle = s.spawn(move |_| {
                generate_combinations(
                    1,
                    5,
                    &mut f64::INFINITY,
                    &mut vec![starting_color],
                    available_colors.clone(),
                    &mut vec![],
                    target_color,
                )
            });

            // Collect the thread's result
            results.push(handle.join().unwrap()); // unwrap() here assumes no thread panics
        }
    })
    .unwrap();

    bar.finish_with_message(format!("Finished processing {}", target_color));

    let mut min_tuple: (f64, Vec<&str>) = (0.0, vec![]);
    let mut min_value = f64::INFINITY;

    for tuple in &results {
        let (value, _) = tuple;
        if *value < min_value {
            min_value = *value;
            min_tuple = tuple.clone();
        }
    }

    let (_, final_panes) = min_tuple;

    Results::new(
        min_value,
        convert_panes_to_rgb(final_panes.clone(), available_colors),
        final_panes,
    )
}

fn generate_combinations<'a>(
    current_depth: usize,
    max_depth: usize,
    distance: &mut f64,
    current_combination: &mut Vec<&'a str>, // Change to Vec<&'a str>
    available_colors: HashMap<&'a str, beacon::color_distance::RGB>,
    most_similar: &mut Vec<&'a str>,
    target_color: RGB,
) -> (f64, Vec<&'a str>) {
    if current_depth == max_depth {
        // Calculate current color and its distance from target
        let current_color = convert_panes_to_rgb(current_combination.clone(), available_colors);
        let dist = calculate_distance(current_color, target_color);

        // Update most_similar and distance if current combination is closer
        if dist < *distance {
            *distance = dist;
            most_similar.clone_from(current_combination)
        }

        // Return the current distance for comparison
        return (dist, most_similar.to_vec());
    }

    let mut min_distance = f64::INFINITY;

    for (key, _) in available_colors.clone().iter() {
        // Append the current possibility to the combination
        current_combination.push(key);

        // Recursive call to generate combinations at the next depth
        let (dist, _) = generate_combinations(
            current_depth + 1,
            max_depth,
            distance,
            current_combination,
            available_colors.clone(),
            most_similar,
            target_color,
        );

        // Track the minimum distance found in the recursive calls
        if dist < min_distance {
            min_distance = dist;
        }

        // Backtrack: Remove the last added possibility to try the next one
        current_combination.pop();
    }

    (min_distance, most_similar.to_vec()) // Return the minimum distance found in this depth level
}

fn convert_panes_to_rgb(panes: Vec<&str>, available_colors: HashMap<&str, RGB>) -> PreciseRGB {
    let n = panes.len();

    if n == 0 {
        return PreciseRGB::new(0.0, 0.0, 0.0);
    }

    let mut sum_colors = PreciseRGB::new(0.0, 0.0, 0.0);

    for (i, pane) in panes.iter().enumerate().skip(1) {
        let weight = 2u32.pow((i - 1) as u32) as f64;
        let color = available_colors.get(pane).unwrap();
        sum_colors.red += weight * color.red as f64;
        sum_colors.green += weight * color.green as f64;
        sum_colors.blue += weight * color.blue as f64;
    }

    let first_color = available_colors.get(&panes[0]).unwrap();
    sum_colors.red += first_color.red as f64;
    sum_colors.green += first_color.green as f64;
    sum_colors.blue += first_color.blue as f64;

    let scaling_factor = 1.0 / (2u32.pow((n - 1) as u32) as f64);

    PreciseRGB::new(
        scaling_factor * sum_colors.red,
        scaling_factor * sum_colors.green,
        scaling_factor * sum_colors.blue,
    )
}
