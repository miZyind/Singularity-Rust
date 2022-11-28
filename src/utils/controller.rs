use super::{events::*, inputs::*, look::*};
use bevy::prelude::*;

#[derive(Component)]
pub struct BodyTag;
pub struct ControllerPlugin;
pub const INPUT_TO_EVENTS_SYSTEM: &str = "input_to_events";
pub const INPUT_TO_LOOK_SYSTEM: &str = "input_to_look";

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LookEvent>()
            .add_event::<TranslationEvent>()
            .add_event::<ForceEvent>()
            .init_resource::<MouseSettings>()
            .add_system_to_stage(
                CoreStage::PreUpdate,
                input_to_events.label(INPUT_TO_EVENTS_SYSTEM),
            )
            .add_system_to_stage(
                CoreStage::PreUpdate,
                input_to_look.label(INPUT_TO_LOOK_SYSTEM),
            );
    }
}

#[derive(Default)]
pub struct InputState {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub jump: bool,
    pub run: bool,
}

#[derive(Component)]
pub struct Controller {
    pub inputs: Inputs,
    pub walk_speed: f32,
    pub run_speed: f32,
    pub jump_speed: f32,
    pub velocity: Vec3,
    pub jumping: bool,
    pub dt: f32,
    pub sim_to_render: f32,
    pub input_state: InputState,
}

impl Default for Controller {
    fn default() -> Self {
        Self {
            inputs: Inputs::default(),
            walk_speed: 5.0,
            run_speed: 8.0,
            jump_speed: 6.0,
            velocity: Vec3::ZERO,
            jumping: false,
            dt: 1.0 / 60.0,
            sim_to_render: 0.0,
            input_state: InputState::default(),
        }
    }
}
#[derive(Component)]
pub struct Mass {
    pub mass: f32,
}

pub fn input_to_events(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut translation_events: EventWriter<TranslationEvent>,
    mut force_events: EventWriter<ForceEvent>,
    mut query: Query<(&Mass, &Transform, &mut Controller)>,
) {
    let xz = Vec3::new(1.0, 0.0, 1.0);
    for (mass, transform, mut controller) in query.iter_mut() {
        controller.sim_to_render += time.delta_seconds();

        if keyboard_input.pressed(controller.inputs.key_forward) {
            controller.input_state.forward = true;
        }
        if keyboard_input.pressed(controller.inputs.key_backward) {
            controller.input_state.backward = true;
        }
        if keyboard_input.pressed(controller.inputs.key_right) {
            controller.input_state.right = true;
        }
        if keyboard_input.pressed(controller.inputs.key_left) {
            controller.input_state.left = true;
        }
        if keyboard_input.pressed(controller.inputs.key_run) {
            controller.input_state.run = true;
        }
        if keyboard_input.just_pressed(controller.inputs.key_jump) {
            controller.input_state.jump = true;
        }
        if controller.sim_to_render < controller.dt {
            continue;
        }
        // Calculate the remaining simulation to render time after all simulation steps were taken
        controller.sim_to_render %= controller.dt;
        let (forward, right) = (
            transform.local_z().normalize(),
            Vec3::cross(transform.local_z().normalize(), Vec3::Y).normalize(),
        );
        // Calculate the desired velocity based on input
        let mut desired_velocity = Vec3::ZERO;
        if controller.input_state.forward {
            desired_velocity += forward;
        }
        if controller.input_state.backward {
            desired_velocity -= forward;
        }
        if controller.input_state.right {
            desired_velocity += right;
        }
        if controller.input_state.left {
            desired_velocity -= right;
        }
        // Limit x/z velocity to walk/run speed
        let speed = if controller.input_state.run {
            controller.run_speed
        } else {
            controller.walk_speed
        };
        desired_velocity = if desired_velocity.length_squared() > 1E-6 {
            desired_velocity.normalize() * speed
        } else {
            // No input - apply damping to the x/z of the current velocity
            controller.velocity * 0.5 * xz
        };
        // Handle jumping
        desired_velocity.y = if controller.input_state.jump && !controller.jumping {
            controller.jumping = true;
            controller.jump_speed
        } else {
            0.0
        };
        // Calculate force - the desired rate of change of momentum for the time period
        let delta_velocity = desired_velocity - controller.velocity * xz;
        let impulse = delta_velocity * mass.mass;
        let force = impulse / controller.dt;
        if force.length_squared() > 1E-6 {
            force_events.send(ForceEvent::new(&force));
        }
        controller.velocity.x = desired_velocity.x;
        controller.velocity.z = desired_velocity.z;
        controller.velocity.y = if controller.jumping {
            // Apply gravity for kinematic simulation
            (-9.81f32).mul_add(controller.dt, controller.velocity.y)
        } else {
            desired_velocity.y
        };
        let translation = controller.velocity * controller.dt;
        if translation.length_squared() > 1E-6 {
            translation_events.send(TranslationEvent::new(&translation));
        }
        controller.input_state = InputState::default();
    }
}
