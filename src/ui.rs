#![allow(dead_code)]

use bevy::{math::Vec4, prelude::Color as BevyColor};

pub struct Color;
impl Color {
    // #140023 rgb(20, 0, 35)
    pub const BACKGROUND: BevyColor = BevyColor::rgb(0.07843, 0.00000, 0.13725);
    // #EECF8C rgb(238, 207, 140)
    pub const FOREGROUND_PRIMARY: BevyColor = BevyColor::rgb(0.93333, 0.81176, 0.54902);
    // #BB9865 rgb(187, 152, 101)
    pub const FOREGROUND_SECONDARY: BevyColor = BevyColor::rgb(0.73333, 0.59608, 0.39608);
    // Linear Interpolation
    pub fn lerp(origin: BevyColor, target: BevyColor, timing: Timing) -> BevyColor {
        BevyColor::from(Vec4::from(origin).lerp(Vec4::from(target), Timing::calc(timing)))
    }
}

pub enum Timing {
    Linear(f32),
    EaseIn(f32),
    EaseOut(f32),
    EaseInOut(f32),
    CubicIn(f32),
    CubicOut(f32),
    CubicInOut(f32),
}
impl Timing {
    pub fn calc(timing: Timing) -> f32 {
        match timing {
            Timing::Linear(t) => t,
            Timing::EaseIn(t) => t * t,
            Timing::EaseOut(t) => -t * (t - 2.0),
            Timing::EaseInOut(t) => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    (-2.0 * t * t) + (4.0 * t) - 1.0
                }
            }
            Timing::CubicIn(t) => t * t * t,
            Timing::CubicOut(t) => {
                let f = t - 1.0;
                f * f * f + 1.0
            }
            Timing::CubicInOut(t) => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    let f = (2.0 * t) - 2.0;
                    0.5 * f * f * f + 1.0
                }
            }
        }
    }
}
