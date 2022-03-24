use crate::constants::{FONT_PATH, LOGO_PATH};
use bevy::{
    prelude::{AssetServer, FromWorld, Handle, Image, World},
    text::Font,
};

pub struct Global {
    pub font: Handle<Font>,
    pub logo: Handle<Image>,
}
impl FromWorld for Global {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let assets = world.get_resource_mut::<AssetServer>().unwrap();
        Global {
            font: assets.load(FONT_PATH),
            logo: assets.load(LOGO_PATH),
        }
    }
}
