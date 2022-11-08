use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection)]
pub struct UIAssets {
    #[asset(path = "fonts/Endor.ttf")]
    pub font: Handle<Font>,
    #[asset(path = "images/logo.png")]
    pub logo: Handle<Image>,
    #[asset(path = "models/shiba.glb#Scene0")]
    pub player: Handle<Scene>,
}
