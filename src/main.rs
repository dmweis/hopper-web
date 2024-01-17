use bevy::{
    ecs::{entity, query},
    prelude::*,
    render::camera::ScalingMode,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(bevy::log::LogPlugin { ..default() }),
            bevy_stl::StlPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate, rotate_tibias, rotate_femurs, rotate_coxas))
        .run();
}

#[derive(Debug, Clone, Copy)]
enum LegFlag {
    LeftFront,
    LeftMiddle,
    LeftRear,
    RightFront,
    RightMiddle,
    RightRear,
}

impl LegFlag {
    fn is_left(&self) -> bool {
        matches!(
            self,
            LegFlag::LeftFront | LegFlag::LeftMiddle | LegFlag::LeftRear
        )
    }

    fn is_right(&self) -> bool {
        !self.is_left()
    }
}

#[derive(Component)]
struct MainBody;

#[derive(Component)]
struct Coxa(LegFlag);

#[derive(Component)]
struct Femur(LegFlag);

#[derive(Component)]
struct Tibia(LegFlag);

const MM_TO_METER_SCALE: Vec3 = Vec3::new(0.001, 0.001, 0.001);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,

    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let center_cylinder_handle = meshes.add(Mesh::from(shape::Cylinder {
        height: 0.2,
        radius: 0.005,
        ..Default::default()
    }));

    let center_sphere_handle = meshes.add(
        Mesh::try_from(shape::Icosphere {
            radius: 0.005,
            subdivisions: 7,
        })
        .unwrap(),
    );

    let red_material = materials.add(StandardMaterial {
        base_color: Color::rgb(1.0, 0., 0.),
        ..default()
    });

    let hopper_material_blue = materials.add(Color::rgba(0.0, 0.0, 1.0, 0.3).into());
    let hopper_material_purple = materials.add(Color::rgba(1., 0., 1., 0.3).into());
    let hopper_material_cyan = materials.add(Color::rgba(0., 1., 1., 0.3).into());
    let hopper_material_red = materials.add(Color::rgba(1., 0., 0., 0.3).into());

    commands.spawn(PbrBundle {
        mesh: center_cylinder_handle.clone(),
        material: red_material.clone(),
        transform: Transform::from_xyz(0.0, 0.1, 0.0),
        ..default()
    });

    let hopper_main_body_mesh = asset_server.load("hopper-main-body-with-parts.stl");
    let hopper_coxa_mesh = asset_server.load("hopper-coxa.stl");
    let hopper_femur_mesh = asset_server.load("hopper-femur.stl");
    let hopper_tibia_mesh = asset_server.load("hopper-tibia.stl");

    let mm_to_meter_scale: Vec3 = (0.001, 0.001, 0.001).into();

    let red_sphere = PbrBundle {
        mesh: center_sphere_handle.clone(),
        material: red_material.clone(),
        transform: Transform::default(),
        ..Default::default()
    };

    let main_body = commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            MainBody,
        ))
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh: hopper_main_body_mesh,
                material: hopper_material_blue.clone(),
                // main body transforms
                transform: Transform::from_xyz(-0.045, 0., -0.270 / 2.0)
                    .with_scale(mm_to_meter_scale),
                ..Default::default()
            });

            parent.spawn(red_sphere.clone());
        })
        .id();

    let coxa_offsets = [
        (0.063, -0.115, 90_f32.to_radians(), LegFlag::LeftFront),
        (0.103, 0., 90_f32.to_radians(), LegFlag::LeftMiddle),
        (0.063, 0.115, 90_f32.to_radians(), LegFlag::LeftRear),
        (-0.063, -0.115, -90_f32.to_radians(), LegFlag::RightFront),
        (-0.103, 0.0, -90_f32.to_radians(), LegFlag::RightMiddle),
        (-0.063, 0.115, -90_f32.to_radians(), LegFlag::RightRear),
    ];

    let mut coxas = vec![];

    for (x, z, rotation, flag) in coxa_offsets.iter().cloned() {
        commands.entity(main_body).with_children(|parent| {
            let coxa_id = parent
                .spawn((
                    SpatialBundle {
                        transform: Transform::from_xyz(x, 0.0, z)
                            .with_rotation(Quat::from_axis_angle(Vec3::Y, rotation)),
                        ..default()
                    },
                    Coxa(flag),
                ))
                .with_children(|parent| {
                    parent.spawn(build_coxa(
                        hopper_coxa_mesh.clone(),
                        hopper_material_purple.clone(),
                    ));
                    parent.spawn(red_sphere.clone());
                })
                .id();
            coxas.push(coxa_id);
        });
    }

    let mut femurs = vec![];
    for (coxa, (_, _, _, flag)) in coxas.iter().zip(coxa_offsets) {
        commands.entity(*coxa).with_children(|parent| {
            let femur_id = parent
                .spawn((
                    SpatialBundle {
                        transform: Transform::from_xyz(0., 0., 0.044),
                        ..default()
                    },
                    Femur(flag),
                ))
                .with_children(|parent| {
                    parent.spawn(build_femur(
                        hopper_femur_mesh.clone(),
                        hopper_material_cyan.clone(),
                    ));
                    parent.spawn(red_sphere.clone());
                })
                .id();
            femurs.push(femur_id);
        });
    }

    for (femur, (_, _, _, flag)) in femurs.iter().zip(coxa_offsets) {
        commands.entity(*femur).with_children(|parent| {
            parent
                .spawn((
                    PbrBundle {
                        transform: Transform::from_xyz(0.0, 0.0, 0.07),
                        ..default()
                    },
                    Tibia(flag),
                ))
                .with_children(|parent| {
                    parent.spawn(build_tibia(
                        hopper_tibia_mesh.clone(),
                        hopper_material_red.clone(),
                    ));
                    parent.spawn(red_sphere.clone());
                });
        });
    }

    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 5.0, -4.0),
        ..default()
    });

    // commands.insert_resource(AmbientLight {
    //     color: Color::WHITE,
    //     brightness: 1.0,
    // });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 1., 1.).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
        ..default()
    });

    // commands.spawn(Camera3dBundle {
    //     transform: Transform::from_xyz(0., 1., 1.).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
    //     projection: Projection::Orthographic(OrthographicProjection {
    //         scale: 1.0,
    //         scaling_mode: ScalingMode::FixedVertical(1.0),
    //         ..Default::default()
    //     }),
    //     ..default()
    // });

    // commands.spawn(Camera3dBundle {
    //     transform: Transform::from_xyz(0., 0., 2.0).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
    //     projection: Projection::Perspective(PerspectiveProjection {
    //         fov: 100_f32.to_radians(),
    //         ..Default::default()
    //     }),
    //     ..default()
    // });
}

fn rotate_tibias(mut query: Query<&mut Transform, With<Tibia>>, time: Res<Time>) {
    for mut transform in &mut query {
        *transform = transform.with_rotation(Quat::from_axis_angle(
            Vec3::X,
            time.elapsed_seconds().sin() / 2.,
        ));
    }
}

fn rotate_femurs(mut query: Query<&mut Transform, With<Femur>>, time: Res<Time>) {
    for mut transform in &mut query {
        *transform = transform.with_rotation(Quat::from_axis_angle(
            Vec3::X,
            time.elapsed_seconds().sin() / 2.,
        ));
    }
}

fn rotate_coxas(mut query: Query<(&mut Transform, &Coxa)>, time: Res<Time>) {
    for (mut transform, coxa) in &mut query {
        let offset = if coxa.0.is_left() {
            90_f32.to_radians()
        } else {
            -90_f32.to_radians()
        };
        *transform = transform.with_rotation(Quat::from_axis_angle(
            Vec3::Y,
            time.elapsed_seconds().sin() / 2. + offset,
        ));
    }
}

fn build_coxa(
    hopper_coxa_mesh: Handle<Mesh>,
    hopper_material: Handle<StandardMaterial>,
) -> impl Bundle {
    PbrBundle {
        mesh: hopper_coxa_mesh,
        material: hopper_material,
        transform: Transform::from_xyz(0., 0., 0.026)
            .with_rotation(Quat::from_axis_angle(Vec3::X, -90_f32.to_radians()))
            .with_scale(MM_TO_METER_SCALE),
        ..Default::default()
    }
}

fn build_femur(
    hopper_femur_mesh: Handle<Mesh>,
    hopper_material: Handle<StandardMaterial>,
) -> impl Bundle {
    PbrBundle {
        mesh: hopper_femur_mesh,
        material: hopper_material,
        transform: Transform::from_xyz(-0.024, 0.01, 0.0315)
            .with_rotation(Quat::from_axis_angle(Vec3::Z, -90_f32.to_radians()))
            .with_scale(MM_TO_METER_SCALE),
        ..Default::default()
    }
}

fn build_tibia(
    hopper_tibia_mesh: Handle<Mesh>,
    hopper_material: Handle<StandardMaterial>,
) -> impl Bundle {
    PbrBundle {
        mesh: hopper_tibia_mesh,
        material: hopper_material,
        transform: Transform::from_xyz(-0.02, 0.01, 0.0315)
            .with_rotation(Quat::from_axis_angle(Vec3::Z, -90_f32.to_radians()))
            .with_scale(MM_TO_METER_SCALE),
        ..Default::default()
    }
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

fn rotate_camera(
    mut query: Query<&mut Transform, With<Camera3d>>,
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
