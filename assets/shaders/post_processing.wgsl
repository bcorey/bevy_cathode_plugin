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
    canvasWidth: f32,
    canvasHeight: f32,
    cellOffset: f32,
    cellSize: f32,
    borderMask: f32,
    time: f32,
    debugStep: f32,
    screenCurvature: f32,
    zoom: f32,
    pulseIntensity: f32,
    pulseWidth: f32,
    pulseRate: f32,
#ifdef SIXTEEN_BYTE_ALIGNMENT
    // WebGL2 structs must be 16 byte aligned.
    _webgl2_padding: vec3<f32>
#endif
}
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;

@fragment
fn fragment(input: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    //Zoom effect hack, doesn't work past certain screen curvatures
  //Add select statement at -1.0 to show what would happen if -1.0 was - 0.0
    var uv = input.uv * (settings.zoom + (dot(input.uv, input.uv) - select(
        1.0, 0.0, settings.debugStep >= 6 && settings.debugStep <= 7
    )) * settings.screenCurvature);
    var pixel = (uv * 0.5 + 0.5) * vec2<f32>(
        settings.canvasWidth,
        settings.canvasHeight
    );

    var coord = pixel / settings.cellSize;
    var subcoord = coord * vec2<f32>(select(settings.cellSize, 3.0, settings.cellSize >= 6.0), 1);

    var cell_offset = vec2<f32>(0, fract(floor(coord.x) * settings.cellOffset));

    var mask_coord = floor(coord + cell_offset) * settings.cellSize;

    var samplePoint = mask_coord / vec2<f32>(settings.canvasWidth, settings.canvasHeight);

    var abberation = textureSample(
        screen_texture,
        texture_sampler,
        samplePoint
    ).xyz;

    var color = abberation;

  //current implementation does not give an even amount of space to each r, g, b unit of a cell
  //Fix/hack this by multiplying subCoord.x by cellSize at cellSizes below 6
    var ind = floor(subcoord.x) % 3;

    var mask_color = vec3<f32>(
        f32(ind == 0.0),
        f32(ind == 1.0),
        f32(ind == 2.0)
    ) * 3.0;

    var cell_uv = fract(subcoord + cell_offset) * 2.0 - 1.0;
    var border: vec2<f32> = 1.0 - cell_uv * cell_uv * settings.borderMask;

    mask_color *= vec3f(clamp(border.x, 0.0, 1.0) * clamp(border.y, 0.0, 1.0));

    color *= vec3f(1.0 + (mask_color - 1.0) * 1.0);

    color.r *= 1.0 + settings.pulseIntensity * sin(pixel.y / settings.pulseWidth + settings.time * settings.pulseRate);
    color.b *= 1.0 + settings.pulseIntensity * sin(pixel.y / settings.pulseWidth + settings.time * settings.pulseRate);
    color.g *= 1.0 + settings.pulseIntensity * sin(pixel.y / settings.pulseWidth + settings.time * settings.pulseRate);

    return vec4<f32>(color, 1.0);
}
