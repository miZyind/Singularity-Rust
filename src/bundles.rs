use crate::{
    constants::{Theme, APP_NAME},
    lib::font::normalize,
    resources::Global,
};
use bevy::{
    math::{Rect, Size},
    prelude::*,
    text::{Text, TextAlignment, TextStyle},
    ui::{Style, Val},
    window::Windows,
};

fn spawn_name(
    parent: &mut ChildBuilder,
    resources: &Res<Global>,
    windows: &Res<Windows>,
    size: f32,
    margin: Rect<Val>,
) {
    parent.spawn_bundle(TextBundle {
        style: Style {
            margin,
            ..Default::default()
        },
        text: Text::with_section(
            APP_NAME,
            TextStyle {
                font: resources.font.clone(),
                font_size: normalize(windows, size),
                color: Theme::FOREGROUND_SECONDARY,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        ..Default::default()
    });
}

fn spawn_logo(parent: &mut ChildBuilder, resources: &Res<Global>, size: f32) {
    parent.spawn_bundle(ImageBundle {
        style: Style {
            margin: Rect {
                bottom: Val::Percent(1.0),
                ..Default::default()
            },
            size: Size::new(Val::Auto, Val::Percent(size / 4.8)),
            ..Default::default()
        },
        image: resources.logo.clone().into(),
        ..Default::default()
    });
}

pub fn spawn_logo_name(
    parent: &mut ChildBuilder,
    resources: &Res<Global>,
    windows: &Res<Windows>,
    size: f32,
) {
    spawn_name(parent, resources, windows, size, Rect::default());
    spawn_logo(parent, resources, size);
}

pub fn spawn_logo_name_with_margin(
    parent: &mut ChildBuilder,
    resources: &Res<Global>,
    windows: &Res<Windows>,
    size: f32,
    margin: Rect<Val>,
) {
    spawn_name(parent, resources, windows, size, margin);
    spawn_logo(parent, resources, size);
}
