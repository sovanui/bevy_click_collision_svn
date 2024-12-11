use bevy::prelude::*;
use bevy_click_collision_svn::plugin::{ClickCollisionEvent, ClickCollisionPlugin};
use bevy_rapier3d::prelude::{Collider, NoUserData, RapierPhysicsPlugin, Sensor};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            ClickCollisionPlugin,
        ))
        .add_systems(Startup, setup_clickable_object)
        .add_systems(Update, handle_click_collision_event)
        .run();
}

// #[derive(Bundle)]
// pub struct ClickableObject {
//     pbr_bundle: PbrBundle,
//     collider: Collider,
//     sensor: Sensor,
// }

#[derive(Component)]
#[require(Collider, Sensor)]
pub struct ClickableObject;

fn setup_clickable_object(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let plane_size = 20.0;
    let click_collider_size = plane_size / 2.0;

    commands.spawn((
        ClickableObject,
        Mesh3d(meshes.add(Plane3d::default().mesh().size(plane_size, plane_size))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Collider::cuboid(click_collider_size, 0., click_collider_size),
    ));

    // Add Camera and light

    let player_camera_y_offset: f32 = 20.0;
    let player_camera_z_offset: f32 = 10.0;

    // Spawn Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, player_camera_y_offset, player_camera_z_offset)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Add global light
    commands.insert_resource(AmbientLight {
        color: Default::default(),
        brightness: 1000.0,
    });
}

fn handle_click_collision_event(mut click_collision_event: EventReader<ClickCollisionEvent>) {
    for event in click_collision_event.read() {
        println!("{:?}", event.position);
    }
}
