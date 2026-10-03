use crate::actor::node::ActorNode;

pub struct DAGPhysicsScheduler {
    nodes: Vec<Box<dyn ActorNode>>,
}

impl DAGPhysicsScheduler {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn register_node(&mut self, node: Box<dyn ActorNode>) {
        self.nodes.push(node);
    }

    pub fn step_simulation(&mut self, delta_t_s: f64) {
        for node in self.nodes.iter_mut() {
            node.step(delta_t_s);
        }
    }
}
