#![allow(dead_code)]
use bevy::prelude::Color;

pub const APP_NAME: &str = "Singularity";
pub const FONT_PATH: &str = "fonts/Endor.ttf";
pub const LOGO_PATH: &str = "images/logo.png";
pub const ZOON_MIN: f32 = 5.0;
pub const ZOON_MAX: f32 = 10.0;
pub const ZOOM_SENSITIVITY: f32 = 0.1;
pub const PLAYER: u32 = 0b1;
pub const GROUND: u32 = 0b10;
pub struct Theme;
impl Theme {
    pub const WHITE: Color = Color::WHITE;
    pub const BLACK: Color = Color::BLACK;
    pub const BLACK_TRANSPARENT: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);
    // #140023 rgb(20, 0, 35)
    pub const BACKGROUND: Color = Color::rgb(0.07843, 0.00000, 0.13725);
    pub const BACKGROUND_TRANSPARENT: Color = Color::rgba(0.07843, 0.00000, 0.13725, 0.0);
    // #EECF8C rgb(238, 207, 140)
    pub const FOREGROUND_PRIMARY: Color = Color::rgb(0.93333, 0.81176, 0.54902);
    // #BB9865 rgb(187, 152, 101)
    pub const FOREGROUND_SECONDARY: Color = Color::rgb(0.73333, 0.59608, 0.39608);
    // #30323E rgb(48, 50, 62)
    pub const INACTIVE: Color = Color::rgb(0.18824, 0.19608, 0.24314);
    // #2489A9 rgb(36, 137, 169)
    pub const INFO: Color = Color::rgb(0.14118, 0.53725, 0.66275);
    // #22B299 rgb(34, 178, 153)
    pub const SUCCESS: Color = Color::rgb(0.13333, 0.69804, 0.60000);
    // #EB5E5E rgb(235, 94, 94)
    pub const ERROR: Color = Color::rgb(0.92157, 0.36863, 0.36863);
}
