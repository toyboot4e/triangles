struct VertexInput {
    [[location(0)]] pos: vec2<f32>;
    [[location(1)]] color: vec4<f32>;
};

struct VertexOutput {
    // clip position
    [[builtin(position)]] pos: vec4<f32>;
    [[location(0)]] color: vec4<f32>;
};

[[stage(vertex)]]
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.pos = vec4<f32>(model.pos, 0.0, 1.0);
    out.color = model.color;
    return out;
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(in.color);
}
