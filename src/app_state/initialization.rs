use super::AppState;
use crate::constants::{COLOR, PATH};
use bevy::{asset::LoadState, prelude::*};
use bevy_ui_animation::*;

pub struct State;
impl Plugin for State {
    fn build(&self, app: &mut App) {
        app.init_resource::<Handles>()
            .add_state(AppState::Initialization)
            .add_system_set(SystemSet::on_enter(AppState::Initialization).with_system(enter))
            .add_system_set(SystemSet::on_update(AppState::Initialization).with_system(update))
            .add_system_set(SystemSet::on_exit(AppState::Initialization).with_system(exit));
    }
}

#[derive(Default)]
struct Handles(Vec<HandleUntyped>);
struct Background(Entity);

fn enter(mut commands: Commands, assets: Res<AssetServer>, mut hs: ResMut<Handles>) {
    hs.0.push((assets.load(PATH::FONT) as Handle<Font>).clone_untyped());
    hs.0.push((assets.load(PATH::LOGO) as Handle<Image>).clone_untyped());
    commands.spawn_bundle(UiCameraBundle::default());
    let entity = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            color: COLOR::BACKGROUND_TRANSPARENT.into(),
            ..Default::default()
        })
        .insert(Animation::new(Vars {
            color: Some(UiColor(COLOR::BACKGROUND)),
            duration: 2.0,
            ease: Ease::PowerIn,
            paused: true,
            ..Default::default()
        }))
        .id();
    commands.insert_resource(Background(entity));
}

fn update(
    assets: Res<AssetServer>,
    handles: Res<Handles>,
    mut state: ResMut<bevy::ecs::schedule::State<AppState>>,
    mut query: Query<&mut Animation>,
    mut complete_events: EventReader<CompleteEvent>,
) {
    if let LoadState::Loaded = assets.get_group_load_state(handles.0.iter().map(|handle| handle.id))
    {
        if let Ok(mut animation) = query.get_single_mut() {
            if animation.paused() {
                animation.play();
            }
        }
        for _ in complete_events.iter() {
            state.set(AppState::Loading).unwrap();
        }
    }
}

fn exit(mut commands: Commands, data: Res<Background>) {
    commands.entity(data.0).despawn_recursive();
    commands.remove_resource::<Handles>();
    commands.remove_resource::<Background>();
}
