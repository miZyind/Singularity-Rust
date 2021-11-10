use bevy::prelude::*;
use std::ops::Deref;

#[derive(Default)]
pub struct LookEvent {
    pub position: Vec3,
}

impl LookEvent {
    pub fn new(other: &Vec3) -> Self {
        Self { position: *other }
    }
}

impl Deref for LookEvent {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.position
    }
}

#[derive(Default)]
pub struct TranslationEvent {
    translation: Vec3,
}

impl TranslationEvent {
    pub fn new(other: &Vec3) -> Self {
        Self {
            translation: *other,
        }
    }
}

impl Deref for TranslationEvent {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.translation
    }
}

#[derive(Default)]
pub struct ForceEvent {
    force: Vec3,
}

impl ForceEvent {
    pub fn new(other: &Vec3) -> Self {
        Self { force: *other }
    }
}

impl Deref for ForceEvent {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.force
    }
}
