#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput
#import bevy_render::view::View
#import bevy_light_2d::types::{LightOccluder2d,Quaternion}
#import bevy_light_2d::view_transformations::{frag_coord_to_ndc, ndc_to_world};

// We're currently only using a single uniform binding for occluders in
// WebGL2, which is limited to 4kb in BatchedUniformBuffer, so we need to
// ensure our occluders can fit in 4kb.
//
// As each occluder is 32 bytes, we can fit 4096 / 32 = 256 occluders.
const MAX_OCCLUDERS: u32 = 128u;

@group(0) @binding(0)
var<uniform> view: View;

// WebGL2 does not support storage buffers, so we fall back to a fixed length
// array in a uniform buffer.
#if AVAILABLE_STORAGE_BUFFER_BINDINGS >= 6
    @group(0) @binding(1)
    var<storage> occluders: array<LightOccluder2d>;
#else
    @group(0) @binding(1)
    var<uniform> occluders: array<LightOccluder2d, MAX_OCCLUDERS>;
#endif

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let pos = ndc_to_world(frag_coord_to_ndc(in.position.xy));

    var sdf = occluder_sd(pos, occluders[0]);

    // WebGL2 does not support storage buffers (or runtime sized arrays), so we
    // need to use a fixed number of occluders.
#if AVAILABLE_STORAGE_BUFFER_BINDINGS >= 6
    let occluder_count = arrayLength(&occluders);
#else
    let occluder_count = MAX_OCCLUDERS;
#endif

    for (var i = 1u; i < occluder_count; i++) {
        sdf = min(sdf, occluder_sd(pos, occluders[i]));
    }

    return vec4(sdf, 0.0, 0.0, 1.0);
}

fn occluder_sd(p: vec2f, occluder: LightOccluder2d) -> f32 {
  let local_pos = quat_mul(occluder.rotation, vec3<f32>(occluder.center - p, 0.0)).xy;
  let d = abs(local_pos) - occluder.half_size;

  return length(max(d, vec2f(0.))) + min(max(d.x, d.y), 0.);
}

// [Drobot2014a] Low Level Optimizations for GCN
fn fast_sqrt(x: f32) -> f32 {
    var bits = bitcast<u32>(x);
        bits = bits >> 1u;
        bits = bits + 0x1fbd1df5u;
    return bitcast<f32>(bits);
}

fn fast_distance_2d(a: vec2<f32>, b: vec2<f32>) -> f32 {
    let d = a - b;
    return fast_sqrt(d.x * d.x + d.y * d.y);
}

fn fast_length_2d(a: vec2<f32>) -> f32 {
    return fast_sqrt(a.x * a.x + a.y * a.y);
}

fn fast_normalize_2d(a: vec2<f32>) -> vec2<f32> {
    return a / fast_length_2d(a);
}

fn fast_distance_3d(a: vec3<f32>, b: vec3<f32>) -> f32 {
    let d = a - b;
    return fast_sqrt(d.x * d.x + d.y * d.y + d.z * d.z);
}

fn fast_length_3d(a: vec3<f32>) -> f32 {
    return fast_sqrt(a.x * a.x + a.y * a.y + a.z * a.z);
}

fn distance_squared(a: vec2<f32>, b: vec2<f32>) -> f32 {
    let c = a - b;
    return dot(c, c);
}

fn hash(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(11.9898, 78.233))) * 43758.5453);
}

/// Quaternion Inverse
fn quat_inv(quat: Quaternion) -> Quaternion {
    let q = quat.data;
    // assume it's a unit quaternion, so just Conjugate
    return Quaternion(vec4<f32>( -q.xyz, q.w ));
}

/// Quaternion multiplication
fn quat_dot(quat1: Quaternion, quat2: Quaternion) -> Quaternion {
    let q1 = quat1.data;
    let q2 = quat2.data;
    let scalar = q1.w * q2.w - dot(q1.xyz, q2.xyz);
    let v = cross(q1.xyz, q2.xyz) + q1.w * q2.xyz + q2.w * q1.xyz;
    return Quaternion(vec4<f32>(v, scalar));
}

/// Apply unit quaternion to vector (rotate vector)
fn quat_mul(q: Quaternion, v: vec3<f32>) -> vec3<f32> {
    let r = quat_dot(q, quat_dot(Quaternion(vec4<f32>(v, 0.0)), quat_inv(q)));
    return r.data.xyz;
}