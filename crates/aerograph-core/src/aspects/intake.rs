pub struct AirIntakeAspect {
    pub filter_clog_ratio: f64,
}

impl AirIntakeAspect {
    pub fn get_restriction_factor(&self) -> f64 {
        1.0 - (0.85 * self.filter_clog_ratio.powf(1.8))
    }
}
