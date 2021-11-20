use crate::constants::{Color, FONT_PATH, LOGO_PATH};
use bevy::{
    prelude::{AssetServer, Assets, ColorMaterial, FromWorld, Handle, World},
    text::Font,
};

pub struct Colors {
    pub white: Handle<ColorMaterial>,
    pub black: Handle<ColorMaterial>,
    pub black_transparent: Handle<ColorMaterial>,
    pub background: Handle<ColorMaterial>,
    pub background_transparent: Handle<ColorMaterial>,
    pub foreground_primary: Handle<ColorMaterial>,
    pub foreground_secondary: Handle<ColorMaterial>,
    pub inactive: Handle<ColorMaterial>,
    pub info: Handle<ColorMaterial>,
    pub success: Handle<ColorMaterial>,
    pub error: Handle<ColorMaterial>,
}
pub struct Global {
    pub font: Handle<Font>,
    pub logo: Handle<ColorMaterial>,
    pub colors: Colors,
}
impl FromWorld for Global {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        let assets = world.get_resource_mut::<AssetServer>().unwrap();
        Global {
            font: assets.load(FONT_PATH),
            logo: materials.add(assets.load(LOGO_PATH).into()),
            colors: Colors {
                white: materials.add(Color::WHITE.into()),
                black: materials.add(Color::BLACK.into()),
                black_transparent: materials.add(Color::BLACK_TRANSPARENT.into()),
                background: materials.add(Color::BACKGROUND.into()),
                background_transparent: materials.add(Color::BACKGROUND_TRANSPARENT.into()),
                foreground_primary: materials.add(Color::FOREGROUND_PRIMARY.into()),
                foreground_secondary: materials.add(Color::FOREGROUND_SECONDARY.into()),
                inactive: materials.add(Color::INACTIVE.into()),
                info: materials.add(Color::INFO.into()),
                success: materials.add(Color::SUCCESS.into()),
                error: materials.add(Color::ERROR.into()),
            },
        }
    }
}
