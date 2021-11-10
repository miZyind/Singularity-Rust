mod camera;
mod constants;
mod lib;
mod player;
mod world;

#[cfg(feature = "native")]
use bevy::input::system::exit_on_esc_system;

use bevy::prelude::*;
use bevy_mod_raycast::{DefaultRaycastingPlugin, RayCastMethod, RayCastSource, RaycastSystem};
use bevy_rapier3d::{physics::TimestepMode, prelude::*};

use lib::{controller::BodyTag, rapier::RapierDynamicForceControllerPlugin};

pub struct MyRaycastSet;

fn main() {
    let mut app = App::build();

    app.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            title: "Singularity".to_string(),
            width: 640.0,
            height: 360.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(RapierConfiguration {
            timestep_mode: TimestepMode::InterpolatedTimestep,
            ..Default::default()
        })
        .add_plugin(RapierDynamicForceControllerPlugin)
        .add_plugin(DefaultRaycastingPlugin::<MyRaycastSet>::default());

    #[cfg(feature = "native")]
    app.add_system(exit_on_esc_system.system());

    #[cfg(feature = "web")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.add_startup_system(world::spawn.system())
        .add_startup_system(player::spawn.system())
        .add_startup_system(camera::spawn.system())
        .add_system(camera::zoom.system())
        .add_system_to_stage(CoreStage::PreUpdate, update_raycast_with_cursor.system())
        .run();
}

fn update_raycast_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RayCastSource<MyRaycastSet>>,
    mut pos_query: Query<(&Transform, &mut RigidBodyPosition), With<BodyTag>>,
) {
    for mut pick_source in &mut query.iter_mut() {
        if let Some(inters) = pick_source.intersect_list() {
            for inter in inters.iter() {
                let pos = inter.1.position();
                for (transform, mut rb_pos) in pos_query.iter_mut() {
                    let y = transform.translation.y;
                    let up = Vec3::Y;
                    let forward =
                        Vec3::normalize(Vec3::new(pos.x, y, pos.z) - transform.translation);
                    let right = up.cross(forward).normalize();
                    let up = forward.cross(right);
                    rb_pos.position.rotation =
                        Quat::from_rotation_mat3(&Mat3::from_cols(right, up, forward)).into();
                    println!("cols {:?}", Mat3::from_cols(right, up, forward));
                }
            }
        }
        // Grab the most recent cursor event if it exists:
        if let Some(cursor_latest) = cursor.iter().last() {
            pick_source.cast_method = RayCastMethod::Screenspace(cursor_latest.position);
        }
    }
}
