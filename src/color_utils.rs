use std::collections::HashMap;

use deltae::{DEMethod::DE2000, DeltaE, LabValue};
use lab::rgbs_to_labs;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy)]
pub struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

impl RGB {
    pub const fn to_array(self) -> [u8; 3] {
        [self.red, self.green, self.blue]
    }

    //pub fn to_f64_array(&self) -> [f64; 3] {
    //    [self.red as f64, self.green as f64, self.blue as f64]
    //}

    pub const fn new(all: [u8; 3]) -> Self {
        Self {
            red: all[0],
            green: all[1],
            blue: all[2],
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PreciseRGB {
    red: f64,
    green: f64,
    blue: f64,
}

impl PreciseRGB {
    pub const fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }

    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn to_u8_array(self) -> [u8; 3] {
        [self.red as u8, self.green as u8, self.blue as u8]
    }
}

impl From<RGB> for PreciseRGB {
    fn from(value: RGB) -> Self {
        Self {
            red: f64::from(value.red),
            green: f64::from(value.green),
            blue: f64::from(value.blue),
        }
    }
}
pub fn calculate_distance(color: PreciseRGB, target: RGB) -> f64 {
    let rgbs = &[color.to_u8_array(), target.to_array()];

    let labs = rgbs_to_labs(rgbs);

    // Convert Lab values from the lab crate to LabValue from DeltaE
    let source_lab = LabValue {
        l: labs[0].l,
        a: labs[0].a,
        b: labs[0].b,
    };

    let target_lab = LabValue {
        l: labs[1].l,
        a: labs[1].a,
        b: labs[1].b,
    };

    let de0 = DeltaE::new(source_lab, target_lab, DE2000);

    f64::from(*de0.value())
}

#[allow(clippy::implicit_hasher)]
pub fn calculate_color_from_panes(panes: &[String], colors: &HashMap<String, RGB>) -> PreciseRGB {
    let n = panes.len();

    if n == 0 {
        return PreciseRGB::new(0.0, 0.0, 0.0);
    }

    let mut sum_colors = PreciseRGB::new(0.0, 0.0, 0.0);

    for (i, pane) in panes.iter().enumerate().skip(1) {
        #[allow(clippy::cast_possible_truncation)]
        let weight = f64::from(2u32.pow((i - 1) as u32));
        let color = colors[pane];
        sum_colors.red += weight * f64::from(color.red);
        sum_colors.green += weight * f64::from(color.green);
        sum_colors.blue += weight * f64::from(color.blue);
    }

    let first_color = colors[&panes[0]];
    sum_colors.red += f64::from(first_color.red);
    sum_colors.green += f64::from(first_color.green);
    sum_colors.blue += f64::from(first_color.blue);
    #[allow(clippy::cast_possible_truncation)]
    let scaling_factor = 1.0 / f64::from(2u32.pow((n - 1) as u32));

    PreciseRGB::new(
        scaling_factor * sum_colors.red,
        scaling_factor * sum_colors.green,
        scaling_factor * sum_colors.blue,
    )
}
