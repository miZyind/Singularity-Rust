use crate::{
    constants::{Color, APP_NAME},
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

pub fn spawn_logo_name<'a>(
    parent: &mut ChildBuilder,
    resources: &Res<Global>,
    windows: &Res<Windows>,
    size: f32,
) {
    parent.spawn_bundle(TextBundle {
        text: Text::with_section(
            APP_NAME,
            TextStyle {
                font: resources.font.clone(),
                font_size: normalize(windows, size),
                color: Color::FOREGROUND_SECONDARY,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        ..Default::default()
    });
    parent.spawn_bundle(ImageBundle {
        style: Style {
            margin: Rect {
                bottom: Val::Percent(1.0),
                ..Default::default()
            },
            size: Size::new(Val::Auto, Val::Percent(size / 4.8)),
            ..Default::default()
        },
        material: resources.logo.clone(),
        ..Default::default()
    });
}
