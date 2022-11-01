use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection)]
pub struct UIAssets {
    #[asset(path = "fonts/Endor.ttf")]
    pub font: Handle<Font>,
    #[asset(path = "images/logo.png")]
    pub logo: Handle<Image>,
}
