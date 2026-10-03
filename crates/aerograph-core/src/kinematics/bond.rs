pub struct StructuralBondAspect {
    pub cross_sectional_area_m2: f64,
    pub ultimate_shear_stress_mpa: f64,
    pub structural_integrity: f64,
}

impl StructuralBondAspect {
    pub fn new(area_m2: f64, ultimate_shear_mpa: f64) -> Self {
        Self {
            cross_sectional_area_m2: area_m2,
            ultimate_shear_stress_mpa: ultimate_shear_mpa,
            structural_integrity: 1.0,
        }
    }

    pub fn evaluate_force_transfer(&mut self, input_force_n: f64) -> f64 {
        if self.structural_integrity <= 0.0 {
            return 0.0;
        }

        let stress_mpa = (input_force_n / self.cross_sectional_area_m2) / 1e6;

        if stress_mpa >= self.ultimate_shear_stress_mpa {
            self.structural_integrity = 0.0;
        } else if stress_mpa > self.ultimate_shear_stress_mpa * 0.85 {
            self.structural_integrity -= 0.02;
        }

        input_force_n * self.structural_integrity
    }
}
