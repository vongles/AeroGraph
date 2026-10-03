pub struct MaterialWaveModel {
    pub speed_of_sound_m_s: f64,
}

impl MaterialWaveModel {
    pub fn calculate_propagation_delay_s(&self, distance_m: f64) -> f64 {
        distance_m / self.speed_of_sound_m_s
    }
}
