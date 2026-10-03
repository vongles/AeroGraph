use std::any::Any;

pub trait ActorNode: Send + Sync {
    fn id(&self) -> u64;
    fn step(&mut self, delta_t_s: f64);
    fn as_any(&self) -> &dyn Any;
}
