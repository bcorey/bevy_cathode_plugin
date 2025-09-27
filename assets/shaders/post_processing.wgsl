// This shader computes the chromatic aberration effect

// Since post processing is a fullscreen effect, we use the fullscreen vertex shader provided by bevy.
// This will import a vertex shader that renders a single fullscreen triangle.
//
// A fullscreen triangle is a single triangle that covers the entire screen.
// The box in the top left in that diagram is the screen. The 4 x are the corner of the screen
//
// Y axis
//  1 |  x-----x......
//  0 |  |  s  |  . ´
// -1 |  x_____x´
// -2 |  :  .´
// -3 |  :´
//    +---------------  X axis
//      -1  0  1  2  3
//
// As you can see, the triangle ends up bigger than the screen.
//
// You don't need to worry about this too much since bevy will compute the correct UVs for you.
#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct PostProcessSettings {
    intensity: f32,
#ifdef SIXTEEN_BYTE_ALIGNMENT
    // WebGL2 structs must be 16 byte aligned.
    _webgl2_padding: vec3<f32>
#endif
}
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    // Chromatic aberration strength
    let offset_strength = settings.intensity;

    // Sample each color channel with an arbitrary shift
    return vec4<f32>(
        textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(offset_strength, -offset_strength)).r,
        textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(-offset_strength, 0.0)).g,
        textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(0.0, offset_strength)).b,
        1.0
    );
}

// struct PostProcessSettings {
//   time: f32,
//   debugStep: f32,
// }

// fn inverseLerp(val: f32, minVal: f32, maxVal: f32) -> f32 {
//   return (val - minVal) / (maxVal - minVal);
// }

// fn remap(
//   val: f32,
//   inputMin: f32,
//   inputMax: f32,
//   outputMin: f32,
//   outputMax: f32,
// ) -> f32 {
//   var t: f32 = inverseLerp(val, inputMin, inputMax);
//   return mix(outputMin, outputMax, t);
// }

// @fragment
// fn fragmentMain(input: VertexOutput) -> @location(0) vec4<f32> {

//   let t1 = remap(
//     sin(input.v_uv.y * 400.0 + uniforms.time * 10.0),
//     -1.0,
//     1.0,
//     0.9,
//     1.0
//   );

//   let t2 = remap(
//     sin(input.v_uv.y * 200.0 - uniforms.time * 20.0),
//     -1.0,
//     1.0,
//     0.95,
//     1.0
//   );
  
//   var color = textureSample(
//     diffuse,
//     image_sampler,
//     input.v_uv
//   ).xyz * t1 * t2;

//   return vec4<f32>(color, 1.0);
// }