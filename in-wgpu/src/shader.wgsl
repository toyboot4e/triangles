struct VertexInput {
    [[location(0)]] pos: vec2<f32>;
    [[location(1)]] color: vec4<f32>;
    [[location(2)]] uv: vec2<f32>;
};

struct VertexOutput {
    // clip position
    [[builtin(position)]] pos: vec4<f32>;
    [[location(0)]] color: vec4<f32>;
    [[location(1)]] uv: vec2<f32>;
};

[[stage(vertex)]]
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.pos = vec4<f32>(model.pos, 0.0, 1.0);
    out.color = model.color;
    out.uv = model.uv;
    return out;
}

[[group(0), binding(0)]]
var t_diffuse: texture_2d<f32>;
[[group(0), binding(1)]]
var s_diffuse: sampler;

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    let color = textureSample(t_diffuse, s_diffuse, in.uv);
    // return color * in.color;
    return color;
}
