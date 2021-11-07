use crate::lib::{controller::*, events::*};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub const BODY_TO_VELOCITY_SYSTEM: &str = "body_to_velocity";
pub const CONTROLLER_TO_RAPIER_DYNAMIC_FORCE_SYSTEM: &str = "controller_to_rapier_dynamic_force";
pub const CREATE_MASS_FROM_RAPIER_SYSTEM: &str = "create_mass_from_rapier";

pub struct RapierDynamicForceControllerPlugin;

impl Plugin for RapierDynamicForceControllerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(ControllerPlugin)
            .add_system(
                create_mass_from_rapier
                    .system()
                    .label(CREATE_MASS_FROM_RAPIER_SYSTEM),
            )
            .add_system(body_to_velocity.system().label(BODY_TO_VELOCITY_SYSTEM))
            .add_system(
                controller_to_rapier_dynamic_force
                    .system()
                    .label(CONTROLLER_TO_RAPIER_DYNAMIC_FORCE_SYSTEM)
                    .after(BODY_TO_VELOCITY_SYSTEM),
            )
            .add_system(controller_to_yaw.system())
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

pub fn controller_to_rapier_dynamic_force(
    mut forces: EventReader<ForceEvent>,
    mut query: Query<(&mut RigidBodyForces, &mut RigidBodyActivation), With<BodyTag>>,
) {
    let mut force = Vec3::ZERO;
    for event in forces.iter() {
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
    mut translations: EventReader<TranslationEvent>,
    mut query: Query<(&mut Transform, &mut Controller), With<BodyTag>>,
) {
    for (mut transform, mut controller) in query.iter_mut() {
        for translation in translations.iter() {
            transform.translation += **translation;
        }
        if transform.translation.y < 0.0 {
            transform.translation.y = 0.0;
            controller.jumping = false;
        }
    }
}
