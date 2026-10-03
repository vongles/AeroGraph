pub struct ShaftBalanceAspect {
    pub dynamic_imbalance_g_cm: f64,
}

impl ShaftBalanceAspect {
    pub fn calculate_vibration_g(&self, rpm: f64) -> f64 {
        let omega = (rpm / 60.0) * 2.0 * std::f64::consts::PI;
        let centrifugal_force_n = (self.dynamic_imbalance_g_cm * 0.00001) * omega.powi(2);
        centrifugal_force_n / (3.5 * 9.81)
    }
}
