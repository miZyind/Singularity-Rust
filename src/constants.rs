#![allow(dead_code)]
use bevy::prelude::Color as BevyColor;

pub const APP_NAME: &str = "Singularity";
pub const ZOON_MIN: f32 = 5.0;
pub const ZOON_MAX: f32 = 10.0;
pub const ZOOM_SENSITIVITY: f32 = 0.1;
pub const PLAYER: u32 = 0b1;
pub const GROUND: u32 = 0b10;
pub struct Color;
impl Color {
    pub const WHITE: BevyColor = BevyColor::WHITE;
    pub const BLACK: BevyColor = BevyColor::BLACK;
    pub const BLACK_TRANSPARENT: BevyColor = BevyColor::rgba(0.0, 0.0, 0.0, 0.0);
    pub const BACKGROUND: BevyColor = BevyColor::rgb(0.07843, 0.00000, 0.13725); // #140023 rgb(20, 0, 35)
    pub const BACKGROUND_TRANSPARENT: BevyColor = BevyColor::rgba(0.07843, 0.00000, 0.13725, 0.0);
    pub const FOREGROUND_PRIMARY: BevyColor = BevyColor::rgb(0.93333, 0.81176, 0.54902); // #EECF8C rgb(238, 207, 140)
    pub const FOREGROUND_SECONDARY: BevyColor = BevyColor::rgb(0.73333, 0.59608, 0.39608); // #BB9865 rgb(187, 152, 101)
    pub const INACTIVE: BevyColor = BevyColor::rgb(0.18824, 0.19608, 0.24314); // #30323E rgb(48, 50, 62)
    pub const INFO: BevyColor = BevyColor::rgb(0.14118, 0.53725, 0.66275); // #2489A9 rgb(36, 137, 169)
    pub const SUCCESS: BevyColor = BevyColor::rgb(0.13333, 0.69804, 0.60000); // #22B299 rgb(34, 178, 153)
    pub const ERROR: BevyColor = BevyColor::rgb(0.92157, 0.36863, 0.36863); // #EB5E5E rgb(235, 94, 94)
    pub const HOVERED_BUTTON: BevyColor = BevyColor::rgba(0.25, 0.25, 0.25, 0.5);
    pub const PRESSED_BUTTON: BevyColor = BevyColor::rgba(0.25, 0.25, 0.25, 1.0);
}
