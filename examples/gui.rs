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
    mut materials: ResMut<Assets<StandardMaterial>>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)).looking_at(Vec3::default(), Vec3::Y),
        Camera {
            clear_color: Color::BLACK.into(),
            ..default()
        },
        // Add the setting to the camera.
        // This component is also used to determine on which camera to run the post processing effect.
        CathodeSettings {
            crt_width: window.physical_width() as f32,
            crt_height: window.physical_height() as f32,
            ..default()
        },
    ));

    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::srgb(1., 1., 1.))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Rotates,
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
fn update_settings(mut settings: Query<&mut CathodeSettings>, time: Res<Time>) {
    for mut setting in &mut settings {
        setting.time = time.elapsed_secs();
    }
}
