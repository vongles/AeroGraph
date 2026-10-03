pub struct IgnitionAspect {
    pub primary_voltage: f64,
    pub boot_leakage_ratio: f64,
}

impl IgnitionAspect {
    pub fn calculate_spark_emi_v_m(&self, rpm: f64) -> f64 {
        let spark_rate_hz = rpm / 60.0;
        let base_field = (self.primary_voltage / 1000.0) * self.boot_leakage_ratio;
        base_field * (1.0 + 0.1 * (spark_rate_hz + 1.0).log10())
    }
}
