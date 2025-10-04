use bevy::{prelude::*, window::PrimaryWindow};
use bevy_crt::{CathodePlugin, CathodeSettings};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CathodePlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate, update_settings))
        .run();
}

/// Set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let cube_location = vec3(0.0, 1.0, 0.0);

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(3.0, 3.0, 8.0)).looking_at(cube_location, Vec3::Y),
        Camera {
            clear_color: Color::WHITE.into(),
            ..default()
        },
        // Add the setting to the camera.
        // This component is also used to determine on which camera to run the post processing effect.
        CathodeSettings {
            crt_width: window.physical_width() as f32,
            crt_height: window.physical_height() as f32,
            border_mask: 0.0,
            ..default()
        },
    ));

    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid {
            half_size: Vec3::new(0.3, 0.3, 0.3),
        })),
        MeshMaterial3d(materials.add(Color::srgb(1., 1., 1.))),
        Transform::from_xyz(cube_location.x, cube_location.y, cube_location.z),
        Rotates,
    ));

    commands.spawn((
        SceneRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/memphis_arches.glb")),
        ),
        Transform::from_xyz(1., 0., 1.),
    ));
    commands.spawn((
        SceneRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/memphis_arches.glb")),
        ),
        Transform::from_xyz(5., 0., 1.),
    ));
    commands.spawn((
        SceneRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/memphis_arches.glb")),
        ),
        Transform::from_xyz(-3., 0., 1.),
    ));
    commands.spawn((
        SceneRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/memphis_arches.glb")),
        ),
        Transform::from_xyz(-7., 0., 1.),
    ));
    // light
    commands.spawn(DirectionalLight {
        illuminance: 1_000.,
        ..default()
    });

    // gui
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::VMax(30.),
                height: Val::VMax(5.),
                bottom: Val::Px(20.),
                left: Val::Px(20.),
                ..Default::default()
            },
            BackgroundColor(Color::WHITE),
        ))
        .with_child((
            Node {
                position_type: PositionType::Relative,
                min_width: Val::VMax(15.0),
                height: Val::Percent(95.),
                margin: UiRect::all(Val::VMax(0.125)),
                ..Default::default()
            },
            BackgroundColor(Color::linear_rgb(0., 0., 1.)),
        ));

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::VMax(30.),
                height: Val::VMax(5.),
                bottom: Val::Px(20.),
                left: Val::Px(500.),
                ..Default::default()
            },
            BackgroundColor(Color::WHITE),
        ))
        .with_child((
            Node {
                position_type: PositionType::Relative,
                min_width: Val::VMax(15.0),
                height: Val::Percent(95.),
                margin: UiRect::all(Val::VMax(0.125)),
                ..Default::default()
            },
            BackgroundColor(Color::linear_rgb(0., 1., 0.)),
        ));
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::VMax(30.),
                height: Val::VMax(5.),
                bottom: Val::Px(20.),
                left: Val::Px(950.),
                ..Default::default()
            },
            BackgroundColor(Color::WHITE),
        ))
        .with_child((
            Node {
                position_type: PositionType::Relative,
                min_width: Val::VMax(15.0),
                height: Val::Percent(95.),
                margin: UiRect::all(Val::VMax(0.125)),
                ..Default::default()
            },
            BackgroundColor(Color::linear_rgb(1., 0., 0.)),
        ));

    commands
        .spawn((Node {
            position_type: PositionType::Absolute,
            width: Val::VMax(30.),
            height: Val::VMax(7.),
            top: Val::Px(20.),
            left: Val::Px(20.),
            ..Default::default()
        },))
        .with_child((
            Text::new("HELLO_WORLD"),
            TextFont {
                font_size: 50.,
                ..default()
            },
            TextColor(Color::WHITE),
            TextBackgroundColor(Color::linear_rgb(0.0, 0., 1.)),
        ));
}

#[derive(Component)]
struct Rotates;

/// Rotates any entity around the x and y axis
fn rotate(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in &mut query {
        transform.rotate_x(0.55 * time.delta_secs());
        transform.rotate_z(0.15 * time.delta_secs());
    }
}

// Change the intensity over time to show that the effect is controlled from the main world
fn update_settings(
    mut settings: Query<&mut CathodeSettings>,
    time: Res<Time>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    for mut setting in &mut settings {
        setting.time = time.elapsed_secs();
        setting.crt_width = window.physical_width() as f32;
        setting.crt_height = window.physical_height() as f32;
    }
}
