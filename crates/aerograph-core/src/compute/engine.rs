use crate::compute::cpu_rayon::{compute_fields_rayon, ObserverNode, SourceNode};

pub enum HardwareBackend {
    CpuFallback,
}

pub struct FieldComputeEngine {
    backend: HardwareBackend,
}

impl FieldComputeEngine {
    pub fn new() -> Self {
        Self {
            backend: HardwareBackend::CpuFallback,
        }
    }

    pub fn evaluate_fields(&self, sources: &[SourceNode], observers: &[ObserverNode]) -> Vec<f32> {
        match self.backend {
            HardwareBackend::CpuFallback => compute_fields_rayon(sources, observers),
        }
    }
}
