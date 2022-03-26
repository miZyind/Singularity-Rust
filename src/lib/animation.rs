#![allow(dead_code)]
use bevy::prelude::*;
use std::f32::consts::PI;

#[derive(Clone, Copy)]
pub enum Ease {
    BackIn,
    BackInOut,
    BackOut,
    BounceIn,
    BounceInOut,
    BounceOut,
    ElasticIn,
    ElasticInOut,
    ElasticOut,
    ExpoIn,
    ExpoInOut,
    ExpoOut,
    Linear,
}

fn normalize(p: f32) -> f32 {
    match () {
        _ if p > 1.0 => 1.0,
        _ if p < 0.0 => 0.0,
        _ => p,
    }
}

trait Easer {
    fn ease(self, ease: Ease) -> Self;
}
impl Easer for f32 {
    fn ease(self, ease: Ease) -> f32 {
        match ease {
            Ease::BackIn => {
                let p = normalize(self);
                p * p * p - p * (p * PI).sin()
            }
            Ease::BackInOut => {
                let p = normalize(self);
                if p < 0.5 {
                    let f = 2.0 * p;
                    0.5 * (f * f * f - f * (f * PI).sin())
                } else {
                    let f = 1.0 - (2.0 * p - 1.0);
                    0.5 * (1.0 - (f * f * f - f * (f * PI).sin())) + 0.5
                }
            }
            Ease::BackOut => {
                let p = normalize(self);
                let f = 1.0 - p;
                1.0 - (f * f * f - f * (f * PI).sin())
            }
            Ease::BounceIn => {
                let p = normalize(self);
                1.0 - (1.0 - p).ease(Ease::BounceOut)
            }
            Ease::BounceInOut => {
                let p = normalize(self);
                if p < 0.5 {
                    0.5 * (p * 2.0).ease(Ease::BounceIn)
                } else {
                    0.5 * (p * 2.0 - 1.0).ease(Ease::BounceOut) + 0.5
                }
            }
            Ease::BounceOut => {
                let p = normalize(self);
                if p < 4.0 / 11.0 {
                    (121.0 * p * p) / 16.0
                } else if p < 8.0 / 11.0 {
                    (363.0 / 40.0 * p * p) - (99.0 / 10.0 * p) + 17.0 / 5.0
                } else if p < 9.0 / 10.0 {
                    (4356.0 / 361.0 * p * p) - (35442.0 / 1805.0 * p) + 16061.0 / 1805.0
                } else {
                    (54.0 / 5.0 * p * p) - (513.0 / 25.0 * p) + 268.0 / 25.0
                }
            }
            Ease::ElasticIn => {
                let p = normalize(self);
                (13.0 * PI * 2.0 * p).sin() * (2.0_f32).powf(10.0 * (p - 1.0))
            }
            Ease::ElasticInOut => {
                let p = normalize(self);
                if p < 0.5 {
                    0.5 * (13.0 * PI * 2.0 * (2.0 * p)).sin()
                        * (2.0_f32).powf(10.0 * ((2.0 * p) - 1.0))
                } else {
                    0.5 * ((-13.0 * PI * 2.0 * ((2.0 * p - 1.0) + 1.0)).sin()
                        * (2.0_f32).powf(-10.0 * (2.0 * p - 1.0))
                        + 2.0)
                }
            }
            Ease::ElasticOut => {
                let p = normalize(self);
                (-13.0 * PI * 2.0 * (p + 1.0)).sin() * (2.0_f32).powf(-10.0 * p) + 1.0
            }
            Ease::ExpoIn => {
                if self <= 0.0 {
                    0.0
                } else {
                    2.0_f32.powf(10.0 * (self.min(1.0) - 1.0))
                }
            }
            Ease::ExpoInOut => {
                if self <= 0.0 {
                    return 0.0;
                }
                if self >= 1.0 {
                    return 1.0;
                }
                if self < 0.5 {
                    0.5 * (2.0_f32).powf((20.0 * self) - 10.0)
                } else {
                    -0.5 * (2.0_f32).powf((-20.0 * self) + 10.0) + 1.0
                }
            }
            Ease::ExpoOut => {
                if self >= 1.0 {
                    1.0
                } else {
                    1.0 - (2.0_f32).powf(-10.0 * self.max(0.0))
                }
            }
            Ease::Linear => normalize(self),
        }
    }
}

trait Lerp {
    fn lerp(&self, target: &Self, delta: &f32) -> Self;
}
impl Lerp for f32 {
    fn lerp(&self, target: &f32, delta: &f32) -> f32 {
        self + (target - self) * delta
    }
}
impl Lerp for Val {
    fn lerp(&self, other: &Self, delta: &f32) -> Self {
        match (self, other) {
            (Val::Percent(self_val), Val::Percent(other_val)) => {
                Val::Percent(self_val.lerp(&other_val, delta))
            }
            (Val::Px(self_val), Val::Px(other_val)) => Val::Px(self_val.lerp(&other_val, delta)),
            _ => *self,
        }
    }
}
impl Lerp for Size<Val> {
    fn lerp(&self, other: &Self, delta: &f32) -> Self {
        Size {
            width: self.width.lerp(&other.width, delta),
            height: self.height.lerp(&other.height, delta),
        }
    }
}
impl Lerp for Transform {
    fn lerp(&self, other: &Self, delta: &f32) -> Self {
        Transform {
            translation: self.translation.lerp(other.translation, *delta),
            scale: self.scale.lerp(other.scale, *delta),
            rotation: self.rotation.lerp(other.rotation, *delta),
        }
    }
}

#[derive(Debug)]
pub struct Target {
    pub style: Option<Style>,
    pub color: Option<Color>,
    pub transform: Option<Transform>,
}
impl Target {
    pub fn style(style: Style) -> Self {
        Self {
            style: Some(style),
            color: None,
            transform: None,
        }
    }
    pub fn color(color: Color) -> Self {
        Self {
            style: None,
            color: Some(color),
            transform: None,
        }
    }
    pub fn transform(transform: Transform) -> Self {
        Self {
            style: None,
            color: None,
            transform: Some(transform),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Config {
    pub delay: f32,
    pub duration: f32,
    pub ease: Ease,
    pub repeat: bool,
    pub yoyo: bool,
    pub paused: bool,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            delay: 0.0,
            duration: 0.5,
            ease: Ease::ExpoOut,
            repeat: false,
            yoyo: false,
            paused: false,
        }
    }
}

#[derive(Component)]
pub struct Animation {
    target: Target,
    config: Config,
    timer: Timer,
    delay_timer: Timer,
    direction: i16,
}
impl Animation {
    pub fn new(target: Target, config: Config) -> Self {
        Self {
            target,
            config,
            timer: Timer::from_seconds(config.duration, false),
            delay_timer: Timer::from_seconds(config.delay, false),
            direction: 1,
        }
    }
}

fn animation_system(
    time: Res<Time>,
    mut commands: Commands,
    mut source: Local<Option<Target>>,
    mut query: Query<(Entity, &mut Style, &mut UiColor, &mut Transform)>,
    mut animation_query: Query<&mut Animation>,
) {
    for (entity, mut style, mut color, mut transform) in query.iter_mut() {
        if source.is_none() {
            // TODO: Optimize
            *source = Some(Target {
                style: Some(style.clone()),
                color: Some(color.0),
                transform: Some(*transform),
            });
        }

        if let Ok(ref mut animation) = animation_query.get_mut(entity) {
            if !animation.config.paused {
                animation.delay_timer.tick(time.delta());

                if animation.delay_timer.finished() {
                    animation.timer.tick(time.delta());

                    if animation.timer.duration().as_secs_f32() != 0.0 {
                        let progress = if animation.direction.is_positive() {
                            animation.timer.percent()
                        } else {
                            animation.timer.percent_left()
                        };
                        let delta = progress.ease(animation.config.ease);

                        // TODO: Optimize
                        if let Some(ref target) = animation.target.style {
                            style.size = source
                                .as_ref()
                                .unwrap()
                                .style
                                .as_ref()
                                .unwrap()
                                .size
                                .lerp(&target.size, &delta);
                        }
                        if let Some(ref target) = animation.target.color {
                            color.0 = Color::from(
                                Vec4::from(source.as_ref().unwrap().color.unwrap())
                                    .lerp(Vec4::from(*target), delta),
                            );
                        }
                        if let Some(ref target) = animation.target.transform {
                            *transform = source
                                .as_ref()
                                .unwrap()
                                .transform
                                .unwrap()
                                .lerp(target, &delta);
                        }
                    }
                    if animation.timer.just_finished() {
                        if animation.config.repeat {
                            if animation.config.yoyo {
                                animation.direction *= -1;
                            }
                            animation.timer.reset();
                        } else {
                            commands.entity(entity).remove::<Animation>();
                        }
                    }
                }
            }
        }
    }
}

pub struct AnimationPlugin;
impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animation_system);
    }
}
