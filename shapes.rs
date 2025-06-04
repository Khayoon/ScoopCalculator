

use std::f64::consts::PI;

pub trait Volumeable {
    fn volume_in_cubic_inches(&self) -> f64;
}

#[derive(Debug)]
pub struct RectangularPrism {
    pub length: f64,
    pub width: f64,
    pub height: f64,
}

impl RectangularPrism {
    pub fn new(length: f64, width: f64, height: f64) -> Self {
        RectangularPrism {
            length,
            width,
            height,
        }
    }
}

impl Volumeable for RectangularPrism {
    fn volume_in_cubic_inches(&self) -> f64 {
        self.length * self.width * self.height
    }
}

#[derive(Debug)]
pub struct HalfEllipsoid {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl HalfEllipsoid {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        HalfEllipsoid { a, b, c }
    }
}

impl Volumeable for HalfEllipsoid {
    fn volume_in_cubic_inches(&self) -> f64 {
        0.5 * (4.0 / 3.0) * PI * self.a * self.b * self.c
    }
}

#[derive(Debug)]
pub struct Cone {
    pub radius: f64,
    pub height: f64,
}

impl Cone {
    pub fn new(radius: f64, height: f64) -> Self {
        Cone { radius, height }
    }
}

impl Volumeable for Cone {
    fn volume_in_cubic_inches(&self) -> f64 {
        (1.0 / 3.0) * PI * self.radius * self.radius * self.height
    }
}
