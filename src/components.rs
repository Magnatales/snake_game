use bevy::prelude::*;
use crate::Component;

#[derive(Component)]
pub struct SnakeHead {
   pub direction: Direction,
}
#[derive(Component)]
pub struct SnakeSegment;

#[derive(Default)]
pub struct LastTailPosition(pub Option<Position>);

#[derive(Default, Deref, DerefMut)]
pub struct SnakeSegments(pub Vec<Entity>);


#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct GrowthEvent;

#[derive(Component)]
pub struct GameOverEvent;

#[derive(Component)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(Component)]
pub struct Food;

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}