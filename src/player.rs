use crate::assets::UIAssets;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Jumper {
    jumping: bool,
    jump_time: f32,
    grounded: bool,
    buttom_time: f32,
}

pub fn spawn(mut commands: Commands, resources: Res<UIAssets>) {
    commands
        .spawn_bundle(SceneBundle {
            scene: resources.player.clone(),
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            ..default()
        })
        .insert_bundle((
            Jumper {
                jumping: false,
                jump_time: 0.0,
                grounded: false,
                buttom_time: 0.2,
            },
            Player,
            RigidBody::Dynamic,
            // Collider::capsule_y(0.05, 0.7),
            Collider::cuboid(0.5, 0.5, 0.5),
            ColliderMassProperties::default(),
            LockedAxes::ROTATION_LOCKED,
            KinematicCharacterController::default(),
        ));
}

pub fn handle_move(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<
        (
            &mut Transform,
            &mut Jumper,
            &mut KinematicCharacterController,
            Option<&KinematicCharacterControllerOutput>,
        ),
        With<Player>,
    >,
) {
    let (mut transform, mut jumper, mut controller, controller_output) = query.single_mut();
    let mut desired_movement = Vec3::ZERO;
    let mut speed = 0.1;
    let x = Vec3::new(1.0, 0.0, -1.0);
    let z = Vec3::new(1.0, 0.0, 1.0);

    for key in input.get_pressed() {
        match *key {
            KeyCode::D => {
                desired_movement += x;
            }
            KeyCode::A => {
                desired_movement -= x;
            }
            KeyCode::W => {
                desired_movement -= z;
            }
            KeyCode::S => {
                desired_movement += z;
            }
            KeyCode::Space => {
                if jumper.grounded {
                    jumper.jumping = true;
                    jumper.jump_time = 0.0;
                }
            }
            KeyCode::LShift => {
                speed /= 10.0;
            }
            _ => {}
        }
    }

    if jumper.jumping {
        desired_movement.y = 2.0;
        jumper.jump_time += time.delta_seconds();

        if jumper.jump_time > jumper.buttom_time {
            jumper.jumping = false;
        }
    }

    if let Some(output) = controller_output {
        jumper.grounded = output.grounded;

        if output.effective_translation.z != 0.0 || output.effective_translation.x != 0.0 {
            let angle = (-output.effective_translation.z).atan2(output.effective_translation.x);
            transform.rotation = transform.rotation.lerp(Quat::from_rotation_y(angle), 0.2);
        }
    }

    desired_movement *= speed;
    controller.translation = Some(desired_movement);
}
