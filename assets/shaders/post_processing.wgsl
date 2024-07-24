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
struct PostProcessUniform {
    time: f32,
    target_height: f32,
    target_width: f32,
    height: f32,
    width: f32,

    daytime: f32,
    day_color: vec4,
    night_color: vec4,

    vignette_strength: f32,

    wave_strength: f32,


//ifdef SIXTEEN_BYTE_ALIGNMENT
//   // WebGL2 structs must be 16 byte aligned.
//   _webgl2_padding: vec3<f32>
//endif
}
@group(0) @binding(2) var<uniform> settings: PostProcessUniform;




@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let time = settings.time;
    let target_aspect = settings.target_width / settings.target_height;
    let aspect = settings.width / settings.height;
    var tw = settings.target_width;
    var th = settings.target_height;

    if aspect > target_aspect {
        // keep height
        tw =  settings.target_height / settings.height * settings.width;
    } else {
        // keep width
        th =  settings.target_width / settings.width * settings.height;
    }

    let coords = in.uv * vec2(settings.width, settings.height);
    var px_coords = in.uv * vec2(tw, th);
    let px_center = vec2(tw, th) * 0.5;
    // target_aspect

    // Sample each color channel with an arbitrary shift
    //let b = vec4<f32>(
    //    textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(offset_strength, -offset_strength)).r,
    //    textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(-offset_strength, 0.0)).g,
    //    textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(0.0, offset_strength)).b,
    //    1.0
    //);
//    var c = in.position;
//    var src_col = textureSample(screen_texture, texture_sampler, in.uv);
//    src_col = src_col * 8.;
//    var col = vec4(
//        round(src_col.r),
//        round(src_col.g),
//        round(src_col.b),
//        round(src_col.a)
//    );
//    col = col / 2.;
//
//
//    return col;
    var col = textureSample(screen_texture, texture_sampler, in.uv);



    let centered_uv = (in.uv - 0.5) * (settings.width/settings.height) * 2.0;
    let rf = sqrt(dot(centered_uv, centered_uv)) * 0.5;
    let rf2_1 = rf * rf + 1.0;
    let vignette = 1.0 / (rf2_1 * rf2_1 * rf2_1);
    //return col * vec4(vec3(vignette), 1.0);



    //round(px.x) + round(px.y) % 2 == 0
    //if (round(px_coords.x) + round(px_coords.y)) % 2 == 0 {col = col * 0.2;}
    let day = vec4(1.5, 1.1, 0.6, 1.);
    let night = vec4(0.005, 0.01, 0.03, 1.);
    var sign = 1.;
    var daytime = time * 0.1;
    if sin(daytime) < 0. {sign = -1.;}
    daytime = (sqrt(abs(sin(daytime)))) * sign * 0.5 + 0.5;
    let modulate = mix(day, night, daytime);



    let pix_uv = vec2(floor(in.uv.y * (th * 0.5)) / (th * 0.5), floor(in.uv.x * (tw * 0.5)) / (tw * 0.5));
    let wave = sin(time + pix_uv * vec2(10., 30.)) * settings.wave_strength * 0.001;
    let waved_pos = in.uv + wave;
    var waved = textureSample(screen_texture, texture_sampler, waved_pos);
    if waved_pos.x < 0. || waved_pos.x > 1. {
        waved = vec4(0., 0., 0., 1.);
    }



    return col * day;
    //return waved;
    //return vec4(1., 1., 1., 1.) * col;
}
