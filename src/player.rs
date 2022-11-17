use crate::assets::UIAssets;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct Player {
    move_speed: f32,
    rotate_speed: f32,
    grounded: bool,
    jumping: bool,
    jump_power: f32,
    jump_time: f32,
    jump_time_max: f32,
}

pub fn spawn(mut commands: Commands, resources: Res<UIAssets>) {
    commands
        .spawn_bundle(SceneBundle {
            scene: resources.player.clone(),
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            ..default()
        })
        .insert_bundle((
            Player {
                move_speed: 0.1,
                rotate_speed: 0.2,
                grounded: false,
                jumping: false,
                jump_power: 2.0,
                jump_time: 0.0,
                jump_time_max: 0.2,
            },
            RigidBody::Dynamic,
            Collider::cuboid(0.5, 0.5, 0.5), // Collider::capsule_y(0.05, 0.7),
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
            &mut Player,
            &mut KinematicCharacterController,
            Option<&KinematicCharacterControllerOutput>,
        ),
        With<Player>,
    >,
) {
    let (mut transform, mut player, mut controller, controller_output) = query.single_mut();
    let mut desired_movement = Vec3::ZERO;
    let mut speed = player.move_speed;
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
                if player.grounded {
                    player.jumping = true;
                    player.jump_time = 0.0;
                }
            }
            KeyCode::LShift => {
                speed /= 10.0;
            }
            _ => {}
        }
    }

    if player.jumping {
        desired_movement.y = player.jump_power;
        player.jump_time += time.delta_seconds();

        if player.jump_time > player.jump_time_max {
            player.jumping = false;
        }
    }

    if let Some(&KinematicCharacterControllerOutput {
        grounded,
        effective_translation,
        ..
    }) = controller_output
    {
        player.grounded = grounded;

        if effective_translation.x != 0.0 || effective_translation.z != 0.0 {
            let angle = (-effective_translation.z).atan2(effective_translation.x);
            transform.rotation = transform
                .rotation
                .lerp(Quat::from_rotation_y(angle), player.rotate_speed);
        }
    }

    desired_movement *= speed;
    controller.translation = Some(desired_movement);
}
