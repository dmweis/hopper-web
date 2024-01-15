use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(bevy::log::LogPlugin { ..default() }),
            bevy_stl::StlPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate)
        .run();
}

#[derive(Component)]
struct MainBody;

#[derive(Component)]
struct Coxa;

#[derive(Component)]
struct Femur;

#[derive(Component)]
struct Tibia;

#[derive(Component)]
enum LegFlag {
    LeftFront,
    LeftMiddle,
    LeftRead,
    RightFront,
    RightMiddle,
    RightRear,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,

    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // commands.insert_resource(AmbientLight {
    //     color: Color::WHITE,
    //     brightness: 1.0,
    // });

    let center_cylinder_handle = meshes.add(Mesh::from(shape::Cylinder {
        height: 0.2,
        radius: 0.005,
        ..Default::default()
    }));
    let cylinder_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(1.0, 0., 0.),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: center_cylinder_handle.clone(),
        material: cylinder_material_handle.clone(),
        transform: Transform::from_xyz(0.0, 0.1, 0.0),
        ..default()
    });

    commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            MainBody,
        ))
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh: asset_server.load("hopper-main-body-with-parts.stl"),
                material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
                // main body transforms
                transform: Transform::from_xyz(-0.045, 0., -0.270 / 2.0)
                    .with_scale((0.001, 0.001, 0.001).into()),
                ..Default::default()
            });

            parent.spawn(PbrBundle {
                mesh: center_cylinder_handle.clone(),
                material: cylinder_material_handle.clone(),
                transform: Transform::from_xyz(0.0, 0.1, 0.0),
                ..Default::default()
            });
        });

    commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_xyz(0.5, 0.0, 0.5),
                ..default()
            },
            Coxa,
        ))
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh: asset_server.load("hopper-coxa.stl"),
                material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
                transform: Transform::from_xyz(0., 0., 0.026)
                    .with_rotation(Quat::from_axis_angle(Vec3::X, -90_f32.to_radians()))
                    .with_scale((0.001, 0.001, 0.001).into()),
                ..Default::default()
            });

            parent.spawn(PbrBundle {
                mesh: center_cylinder_handle.clone(),
                material: cylinder_material_handle.clone(),
                transform: Transform::from_xyz(0.0, 0.1, 0.0),
                ..Default::default()
            });
        });

    commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_xyz(-0.5, 0., 0.5),
                ..default()
            },
            Femur,
        ))
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh: asset_server.load("hopper-femur.stl"),
                material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
                transform: Transform::from_xyz(-0.024, 0.01, 0.0315)
                    .with_rotation(Quat::from_axis_angle(Vec3::Z, -90_f32.to_radians()))
                    .with_scale((0.001, 0.001, 0.001).into()),
                ..Default::default()
            });

            parent.spawn(PbrBundle {
                mesh: center_cylinder_handle.clone(),
                material: cylinder_material_handle.clone(),
                transform: Transform::from_xyz(0.0, 0.1, 0.0),
                ..Default::default()
            });
        });

    commands
        .spawn((
            PbrBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.5),
                ..default()
            },
            Tibia,
        ))
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh: asset_server.load("hopper-tibia.stl"),
                material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
                transform: Transform::from_xyz(-0.02, 0.01, 0.0315)
                    .with_rotation(Quat::from_axis_angle(Vec3::Z, -90_f32.to_radians()))
                    .with_scale((0.001, 0.001, 0.001).into()),
                ..Default::default()
            });

            parent.spawn(PbrBundle {
                mesh: center_cylinder_handle.clone(),
                material: cylinder_material_handle.clone(),
                transform: Transform::from_xyz(0.0, 0.1, 0.0),
                ..Default::default()
            });
        });

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 5.0, -4.0),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 1., 1.0).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
        ..default()
    });
}

fn rotate(
    mut query: Query<&mut Transform, With<MainBody>>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
) {
    for mut transform in &mut query {
        let mut direction = Vec3::ZERO;
        if input.pressed(KeyCode::W) {
            direction.z -= 1.0
        }
        if input.pressed(KeyCode::S) {
            direction.z += 1.0;
        }
        if input.pressed(KeyCode::A) {
            direction.x -= 1.0;
        }
        if input.pressed(KeyCode::D) {
            direction.x += 1.0;
        }

        transform.translation += time.delta_seconds() * 0.5 * direction;

        // rotation

        let mut rotation = 0.0;
        if input.pressed(KeyCode::Q) {
            rotation += 90_f32.to_radians();
        }
        if input.pressed(KeyCode::E) {
            rotation -= 90_f32.to_radians();
        }
        transform.rotate_y(rotation * time.delta_seconds());

        if input.just_pressed(KeyCode::Space) {
            transform.translation = Vec3::ZERO;
            transform.rotation = Quat::IDENTITY;
        }
    }
}
