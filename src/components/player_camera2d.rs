#![allow(unused)]
use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Camera;

#[derive(Debug, Component, Deref, DerefMut)]
pub struct CameraSensitivity(Vec2);
