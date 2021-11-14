use crate::lib::{controller::*, events::*};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub const BODY_TO_VELOCITY_SYSTEM: &str = "body_to_velocity";
pub const BODY_TO_ROTATION_SYSTEM: &str = "body_to_rotation";
pub const CONTROLLER_TO_RAPIER_DYNAMIC_FORCE_SYSTEM: &str = "controller_to_rapier_dynamic_force";
pub const CREATE_MASS_FROM_RAPIER_SYSTEM: &str = "create_mass_from_rapier";

pub struct RapierPlugin;

impl Plugin for RapierPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(ControllerPlugin)
            .add_system(
                create_mass_from_rapier
                    .system()
                    .label(CREATE_MASS_FROM_RAPIER_SYSTEM),
            )
            .add_system(body_to_rotation.system().label(BODY_TO_ROTATION_SYSTEM))
            .add_system(
                body_to_velocity
                    .system()
                    .label(BODY_TO_VELOCITY_SYSTEM)
                    .after(BODY_TO_ROTATION_SYSTEM),
            )
            .add_system(
                controller_to_rapier_dynamic_force
                    .system()
                    .label(CONTROLLER_TO_RAPIER_DYNAMIC_FORCE_SYSTEM)
                    .after(BODY_TO_VELOCITY_SYSTEM),
            )
            .add_system(controller_to_kinematic.system());
    }
}

pub fn create_mass_from_rapier(
    mut commands: Commands,
    query: Query<(Entity, &RigidBodyMassProps), Without<Mass>>,
) {
    for (entity, mass_props) in query.iter() {
        let mass = 1.0 / mass_props.effective_inv_mass;
        commands.entity(entity).insert(Mass::new(mass));
    }
}

pub fn body_to_velocity(mut query: Query<(&RigidBodyVelocity, &mut Controller), With<BodyTag>>) {
    for (velocity, mut controller) in query.iter_mut() {
        controller.velocity = velocity.linvel.into();
    }
}

pub fn body_to_rotation(
    mut look_reader: EventReader<LookEvent>,
    mut query: Query<(&Transform, &mut RigidBodyPosition), With<BodyTag>>,
) {
    for event in look_reader.iter() {
        for (transform, mut positions) in query.iter_mut() {
            let y = transform.translation.y;
            let up = Vec3::Y;
            let forward = Vec3::normalize(
                Vec3::new(event.position.x, y, event.position.z) - transform.translation,
            );
            let right = up.cross(forward).normalize();
            let up = forward.cross(right);
            let rotation = Quat::from_rotation_mat3(&Mat3::from_cols(right, up, forward));
            if !Quat::is_nan(rotation) {
                positions.position.rotation =
                    Quat::from_rotation_mat3(&Mat3::from_cols(right, up, forward)).into();
            }
        }
    }
}

pub fn controller_to_rapier_dynamic_force(
    mut force_reader: EventReader<ForceEvent>,
    mut query: Query<(&mut RigidBodyForces, &mut RigidBodyActivation), With<BodyTag>>,
) {
    let mut force = Vec3::ZERO;
    for event in force_reader.iter() {
        force += **event;
    }

    if force.length_squared() > 1E-6 {
        for (mut forces, mut activation) in query.iter_mut() {
            forces.force = force.into();
            activation.wake_up(true);
        }
    }
}

// TODO: Interaction Test
pub fn controller_to_kinematic(
    mut translation_reader: EventReader<TranslationEvent>,
    mut query: Query<(&mut Transform, &mut Controller), With<BodyTag>>,
) {
    for (mut transform, mut controller) in query.iter_mut() {
        for translation in translation_reader.iter() {
            transform.translation += **translation;
        }
        if transform.translation.y < 0.0 {
            transform.translation.y = 0.0;
            controller.jumping = false;
        }
    }
}

// pub fn ground_intersect(
//     // mut query: Query<(&mut Player, Entity, &RigidBodyVelocity)>,
//     mut query: Query<(&mut Controller, &RigidBodyVelocity), With<BodyTag>>,
//     grounds: Res<Grounds>,
//     mut psc_event: EventWriter<PlayerStateChangeEvent>,
//     mut intersection_events: EventReader<IntersectionEvent>,
// ) {
//     if let Ok((mut controller, rb_vel)) = query.single_mut() {
//         for intersection_event in intersection_events.iter() {
//             match intersection_event {
//                 IntersectionEvent {
//                     collider1,
//                     collider2,
//                     intersecting: true,
//                     ..
//                 } => {
//                     if !grounds.contains(&collider1.entity())
//                         && !grounds.contains(&collider2.entity())
//                     {
//                         return;
//                     }
//                     if collider1.entity() == player_entity || collider2.entity() == player_entity {
//                         if rb_vel
//                             .linvel
//                             .data
//                             .0
//                             .get(0)
//                             .map(|x| x[0] == 0.)
//                             .unwrap_or_default()
//                         {
//                             player.state = PlayerState::Wait;
//                             psc_event.send(PlayerStateChangeEvent {
//                                 state: PlayerState::Wait,
//                             });
//                         } else {
//                             player.state = PlayerState::Walk(0);
//                             psc_event.send(PlayerStateChangeEvent {
//                                 state: PlayerState::Walk(0),
//                             });
//                         };
//                     }
//                 }
//                 IntersectionEvent {
//                     collider1,
//                     collider2,
//                     intersecting: false,
//                 } => {
//                     if !grounds.contains(&collider1.entity())
//                         && !grounds.contains(&collider2.entity())
//                     {
//                         return;
//                     }
//                     if collider1.entity() == player_entity || collider2.entity() == player_entity {
//                         player.state = PlayerState::Jump;
//                         psc_event.send(PlayerStateChangeEvent {
//                             state: PlayerState::Jump,
//                         });
//                     }
//                 }
//             }
//         }
//     }
// }
