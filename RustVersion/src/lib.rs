// src/lib.rs

pub mod color_distance {

    use deltae::*;
    use lab::rgbs_to_labs;
    use std::fmt;

    /// Represents an RGB color with red, green, and blue components.
    #[derive(Debug, Clone, Copy)]
    pub struct RGB {
        pub red: u8,
        pub green: u8,
        pub blue: u8,
    }

    impl RGB {
        /// Creates a new RGB color.
        pub fn new(red: u8, green: u8, blue: u8) -> Self {
            RGB { red, green, blue }
        }

        pub fn to_array(&self) -> [u8; 3] {
            [self.red, self.green, self.blue]
        }

        pub fn to_f64_array(&self) -> [f64; 3] {
            [self.red as f64, self.green as f64, self.blue as f64]
        }

        pub fn new_from_array(all: [u8; 3]) -> Self {
            RGB {
                red: all[0],
                green: all[1],
                blue: all[2],
            }
        }
        
        #[inline]
        pub fn to_precise(self) -> PreciseRGB {
            self.into()
        }

        pub fn new_from_number(number: u32) -> Self {
            let red = ((number >> 16) & 0xFF) as u8;
            let green = ((number >> 8) & 0xFF) as u8;
            let blue = (number & 0xFF) as u8;

            RGB { red, green, blue }
        }
    }

    impl From<RGB> for PreciseRGB {
        #[inline]
        fn from(value: RGB) -> Self {
            PreciseRGB {
                red: value.red as f64,
                green: value.green as f64,
                blue: value.blue as f64,
            }
        }
    }

    impl fmt::Display for RGB {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "0x{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    pub struct PreciseRGB {
        pub red: f64,
        pub green: f64,
        pub blue: f64,
    }

    impl PreciseRGB {
        /// Creates a new RGB color.
        pub fn new(red: f64, green: f64, blue: f64) -> Self {
            PreciseRGB { red, green, blue }
        }

        pub fn to_array(&self) -> [f64; 3] {
            [self.red, self.green, self.blue]
        }

        pub fn to_u8_array(&self) -> [u8; 3] {
            [self.red as u8, self.green as u8, self.blue as u8]
        }
    }

    #[derive(Clone, Copy)]
    struct MyLab(f32, f32, f32);

    // Types that implement Into<LabValue> also implement the Delta trait
    impl From<MyLab> for LabValue {
        fn from(mylab: MyLab) -> Self {
            LabValue {
                l: mylab.0,
                a: mylab.1,
                b: mylab.2,
            }
        }
    }

    pub fn calculate_distance(color: PreciseRGB, target: RGB) -> f64 {
        let rgbs = &[color.to_u8_array(), target.to_array()];

        let labs = rgbs_to_labs(rgbs);

        //println!("{:?}", labs);

        let lab0 = MyLab(labs[0].l, labs[0].a, labs[0].b);
        let lab1 = MyLab(labs[1].l, labs[1].a, labs[1].b);

        let de0 = DeltaE::new(lab0, lab1, DE2000);

        *de0.value() as f64
    }
}
