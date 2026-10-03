pub struct DigitalBusModel {
    pub baud_rate: u32,
}

impl DigitalBusModel {
    pub fn calculate_latency_s(&self, byte_count: usize) -> f64 {
        (byte_count * 10) as f64 / self.baud_rate as f64
    }
}
