use glam::Vec3A;
use rayon::prelude::*;

pub struct SourceNode {
    pub position: Vec3A,
    pub intensity: f32,
}

pub struct ObserverNode {
    pub position: Vec3A,
}

pub fn compute_fields_rayon(sources: &[SourceNode], observers: &[ObserverNode]) -> Vec<f32> {
    observers
        .par_iter()
        .map(|obs| {
            let mut total_field = 0.0f32;
            for src in sources {
                let diff = obs.position - src.position;
                let mut dist_sq = diff.length_squared();
                if dist_sq < 0.0001 {
                    dist_sq = 0.0001;
                }
                total_field += src.intensity / dist_sq;
            }
            total_field
        })
        .collect()
}
