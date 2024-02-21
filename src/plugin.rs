use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::{QueryFilter, RapierContext, Real};

pub struct ClickCollisionPlugin;

impl Plugin for ClickCollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, monitor_click_collisions)
            .add_event::<ClickCollisionEvent>();
    }
}

#[derive(Event)]
pub struct ClickCollisionEvent {
    pub position: Vec3,
    pub entity: Entity,
}

fn monitor_click_collisions(
    camera: Query<(&Camera, &GlobalTransform)>,
    window: Query<&Window, With<PrimaryWindow>>,
    rapier_context: Res<RapierContext>,
    mouse_button: Res<ButtonInput<MouseButton>>,

    mut click_collision_event_writer: EventWriter<ClickCollisionEvent>,
) {
    if mouse_button.just_pressed(MouseButton::Right) {
        let (camera, camera_global_transform) = camera.single();
        let window = window.single();

        if let Some(cursor_position) = window.cursor_position() {
            let ray = camera
                .viewport_to_world(camera_global_transform, cursor_position)
                .unwrap();

            if let Some((entity, toi)) = rapier_context.cast_ray(
                ray.origin,
                *ray.direction,
                Real::MAX,
                true,
                QueryFilter::default(),
            ) {
                click_collision_event_writer.send(ClickCollisionEvent {
                    position: (ray.origin + ray.direction * toi),
                    entity,
                });
            }
        }
    }
}
