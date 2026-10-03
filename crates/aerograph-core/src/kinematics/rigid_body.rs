use glam::{Vec3A, Quat};

#[derive(Debug, Clone, Copy)]
pub struct RigidBodyState {
    pub position: Vec3A,
    pub linear_velocity: Vec3A,
    pub linear_accel: Vec3A,
    pub orientation: Quat,
    pub angular_velocity: Vec3A,
    pub angular_accel: Vec3A,
}

impl Default for RigidBodyState {
    fn default() -> Self {
        Self {
            position: Vec3A::ZERO,
            linear_velocity: Vec3A::ZERO,
            linear_accel: Vec3A::ZERO,
            orientation: Quat::IDENTITY,
            angular_velocity: Vec3A::ZERO,
            angular_accel: Vec3A::ZERO,
        }
    }
}
