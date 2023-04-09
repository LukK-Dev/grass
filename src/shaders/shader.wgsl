struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
}

struct VertexOuput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOuput {
    var out: VertexOuput;
    out.clip_position = vec4<f32>(in.position, 1.0);
    out.color = vec3<f32>(in.color);
    return out;
}

@fragment
fn fs_main(in: VertexOuput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
