use bevy::{prelude::Res, window::Windows};

// 3840 = 288pt = 200.0%;
// 1920 = 144pt = 100.0%;
// 1280 =  96pt = 66.66%;
//  640 =  48pt = 33.33%;
pub fn normalize(windows: &Res<Windows>, font_size: f32) -> f32 {
    windows.get_primary().unwrap().width() * font_size / 1920.0
}
