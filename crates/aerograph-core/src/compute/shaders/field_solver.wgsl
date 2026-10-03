struct SourceNode {
    position: vec3<f32>,
    intensity: f32,
};

struct ObserverNode {
    position: vec3<f32>,
    padding: f32,
};

@group(0) @binding(0) var<storage, read> sources: array<SourceNode>;
@group(0) @binding(1) var<storage, read> observers: array<ObserverNode>;
@group(0) @binding(2) var<storage, read_write> output_fields: array<f32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let idx = global_id.x;
    let total_observers = arrayLength(&observers);
    if (idx >= total_observers) {
        return;
    }

    let obs_pos = observers[idx].position;
    var accumulated_field: f32 = 0.0;
    let total_sources = arrayLength(&sources);

    for (var i: u32 = 0u; i < total_sources; i = i + 1u) {
        let src_pos = sources[i].position;
        let diff = obs_pos - src_pos;
        var dist_sq = dot(diff, diff);
        if (dist_sq < 0.0001) {
            dist_sq = 0.0001;
        }
        accumulated_field = accumulated_field + (sources[i].intensity / dist_sq);
    }

    output_fields[idx] = accumulated_field;
}
