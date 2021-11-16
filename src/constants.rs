use bevy::render::color::Color as BevyColor;

pub const APP_NAME: &str = "Singularity";
pub const ZOON_MIN: f32 = 5.0;
pub const ZOON_MAX: f32 = 10.0;
pub const ZOOM_SENSITIVITY: f32 = 0.1;
pub const PLAYER: u32 = 0b1;
pub const GROUND: u32 = 0b10;
pub struct Color;
impl Color {
    // #140023 rgb(20, 0, 35)
    pub const BACKGROUND: BevyColor = BevyColor::rgb(0.07843, 0.00000, 0.13725);
    // #EECF8C rgb(238, 207, 140)
    pub const FOREGROUND_PRIMARY: BevyColor = BevyColor::rgb(0.93333, 0.81176, 0.54902);
    // #BB9865 rgb(187, 152, 101)
    pub const FOREGROUND_SECONDARY: BevyColor = BevyColor::rgb(0.73333, 0.59608, 0.39608);
}
