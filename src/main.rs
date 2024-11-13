//! Shows how to render to a texture. Useful for mirrors, UI, or exporting images.

use bevy::{
    prelude::*,
    render::{
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        texture::ImageSampler,
        view::RenderLayers,
    },
};

const RENDER_TEXTURE_WIDTH: u32 = 640;
const RENDER_TEXTURE_HEIGHT: u32 = 360;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, cube_rotator_system)
        .run();
}

#[derive(Component)]
struct Cube;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    //
    // Create the render texture
    //

    let size = Extent3d {
        width: RENDER_TEXTURE_WIDTH,
        height: RENDER_TEXTURE_HEIGHT,
        ..default()
    };

    let mut render_texture = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        sampler: ImageSampler::nearest(), // For pixelated look
        ..default()
    };

    // fill image.data with zeroes
    render_texture.resize(size);

    let render_texture_handle = images.add(render_texture);

    //
    // Everything that needs to be rendered, needs to be assigned to render layer #1
    //

    let cube_handle = meshes.add(Cuboid::new(4.0, 4.0, 4.0));
    let cube_material_handle = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.7, 0.6),
        reflectance: 0.02,
        unlit: false,
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh: cube_handle,
            material: cube_material_handle,
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..default()
        },
        Cube,
        RenderLayers::layer(1),
    ));

    commands.spawn((
        PointLightBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
            ..default()
        },
        RenderLayers::layer(1),
    ));

    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                // render before the "main pass" camera
                order: -1,
                target: render_texture_handle.clone().into(),
                clear_color: Color::WHITE.into(),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        RenderLayers::layer(1),
    ));

    //
    // A sprite and orthographic camera is used to present the render texture.
    // These are on render layer #0
    //
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(
                    RENDER_TEXTURE_WIDTH as f32,
                    RENDER_TEXTURE_HEIGHT as f32,
                )),
                ..default()
            },
            texture: render_texture_handle,
            transform: Transform::from_scale(Vec3::splat(4.0)),
            ..default()
        },
        RenderLayers::layer(0),
    ));

    commands.spawn((Camera2dBundle::default(), RenderLayers::layer(0)));
}

fn cube_rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Cube>>) {
    for mut transform in &mut query {
        transform.rotate_x(1.0 * time.delta_seconds());
        transform.rotate_y(0.7 * time.delta_seconds());
    }
}
