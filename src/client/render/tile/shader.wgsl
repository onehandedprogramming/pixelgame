// Vertex shader

struct VertexOutput {
    @location(0) rgba: vec4<f32>,
    @builtin(position) clip_position: vec4<f32>,
};

struct InstanceInput {
    @location(0) rgba: vec4<f32>,
};

struct ViewUniform {
    pos: vec2<f32>,
    proj: vec2<f32>,
    width: u32,
};

@group(0) @binding(0)
var<uniform> view: ViewUniform;

@vertex
fn vs_main(
    @builtin(vertex_index) vi: u32,
    @builtin(instance_index) i: u32,
    in: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;

    var pos = vec2<f32>(f32(vi % 2u), f32(vi / 2u));
    pos.x += f32(i % view.width);
    pos.y += f32(i / view.width);
    pos += view.pos;
    pos *= view.proj;
    out.clip_position = vec4<f32>(pos.x, pos.y, 0.0, 1.0);
    out.rgba = in.rgba;

    return out;
}

// Fragment shader

@fragment
fn fs_main(
    in: VertexOutput
) -> @location(0) vec4<f32> {
    return in.rgba;
}
