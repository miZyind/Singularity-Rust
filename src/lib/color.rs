use super::easing::Function;
use bevy::{math::Vec4, prelude::Color};

// Linear Interpolation
pub fn lerp(origin: Color, target: Color, function: Function) -> Color {
    Color::from(Vec4::from(origin).lerp(Vec4::from(target), Function::apply(function)))
}
