#[derive(Debug, Clone)]
pub struct PhysicsEnvelope<T> {
    pub sequence_tick: u64,
    pub source_node_id: u64,
    pub target_node_id: u64,
    pub timestamp_emitted_ns: f64,
    pub timestamp_deliverable_ns: f64,
    pub payload: T,
}
