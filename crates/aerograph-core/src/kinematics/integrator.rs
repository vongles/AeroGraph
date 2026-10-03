use crate::kinematics::rigid_body::RigidBodyState;

pub fn step_symplectic_euler(state: &mut RigidBodyState, delta_t: f32) {
    state.linear_velocity += state.linear_accel * delta_t;
    state.position += state.linear_velocity * delta_t;
    state.angular_velocity += state.angular_accel * delta_t;
}
