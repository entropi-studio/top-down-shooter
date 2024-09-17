#define_import_path bevy_light_2d::types

struct Quaternion {
    data: vec4<f32>,
}

struct AmbientLight2d {
    color: vec4<f32>,
}

struct LightOccluder2d {
    center: vec2<f32>,
    rotation: Quaternion,
    half_size: vec2<f32>,
}

struct PointLight2d {
    center: vec2f,
    radius: f32,
    color: vec4<f32>,
    intensity: f32,
    falloff: f32,
    angles: array<f32, 2>,
    cast_shadows: u32
}

struct PointLightMeta {
    count: u32,
    // WebGL2 structs must be 16 byte aligned.
    _padding: vec3<u32>
}
