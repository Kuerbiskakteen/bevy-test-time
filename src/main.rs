use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::sprite::Material2d;
use bevy::sprite::Material2dPlugin;
use bevy::sprite::MaterialMesh2dBundle;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<CustomMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(MaterialMesh2dBundle {
        material: materials.add(CustomMaterial {}),
        mesh: meshes.add(Rectangle::default()).into(),
        transform: Transform::from_scale(Vec3::new(250f32, 250f32, 250f32)),
        ..default()
    });
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/test.wgsl".into()
    }
}
