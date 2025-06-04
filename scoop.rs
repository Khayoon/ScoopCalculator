use crate::shapes::Volumeable;

#[derive(Debug)]
pub struct Scoop {
    pub base: Box<dyn Volumeable>,
    pub mound: Box<dyn Volumeable>,
}

impl Scoop {
    pub fn new(base: Box<dyn Volumeable>, mound: Box<dyn Volumeable>) -> Self {
        Scoop { base, mound }
    }

    pub fn total_volume_in_cubic_inches(&self) -> f64 {
        self.base.volume_in_cubic_inches() + self.mound.volume_in_cubic_inches()
    }

    pub fn mass_in_grams(&self, density_g_per_cuin: f64) -> f64 {
        self.total_volume_in_cubic_inches() * density_g_per_cuin
    }
}
