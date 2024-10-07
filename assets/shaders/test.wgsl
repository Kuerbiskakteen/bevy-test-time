#import bevy_pbr::mesh_view_bindings::globals;

@fragment
fn fragment() -> @location(0) vec4<f32> {
    return vec4<f32>(globals.time, globals.time, globals.time, globals.time);
//    return vec4<f32>(0.0, 0.0, 0.0, 0.0);
}
