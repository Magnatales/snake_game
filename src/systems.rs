use crate::components::{Food, Position, Size, SnakeHead, Direction, SnakeSegment, SnakeSegments, GrowthEvent, LastTailPosition, GameOverEvent};
use crate::cons;
use bevy::prelude::*;
use rand::random;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub fn spawn_snake(mut commands: Commands, mut segments: ResMut<SnakeSegments>) {
    *segments = SnakeSegments(vec![
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: cons::SNAKE_HEAD_COLOR,
                    ..default()
                },
                ..default()
            })
            .insert(SnakeHead {
                direction: Direction::Up,
            })
            .insert(SnakeSegment)
            .insert(Position { x: 3, y: 3 })
            .insert(Size::square(0.8))
            .id(),
        spawn_segment(commands, Position { x: 3, y: 2 })]);
}

pub fn snake_movement_input(keyboard_input: Res<Input<KeyCode>>,
                            mut heads: Query<&mut SnakeHead>) {
    if let Some(mut head) = heads.iter_mut().next() {
        let dir: Direction = if keyboard_input.pressed(KeyCode::Left) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::Down) {
            Direction::Down
        } else if keyboard_input.pressed(KeyCode::Up) {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::Right) {
            Direction::Right
        } else {
            head.direction
        };
        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}

pub fn snake_movement(mut last_tail_position: ResMut<LastTailPosition>, 
                      mut game_over_writer: EventWriter<GameOverEvent>, 
                      segments: ResMut<SnakeSegments>, 
                      mut heads: Query<(Entity, &SnakeHead)>, 
                      mut positions: Query<&mut Position>) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let segment_positions = segments
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();
        let mut head_pos = positions.get_mut(head_entity).unwrap();
        match &head.direction {
            Direction::Left => {
                head_pos.x -= 1;
            }
            Direction::Right => {
                head_pos.x += 1;
            }
            Direction::Up => {
                head_pos.y += 1;
            }
            Direction::Down => {
                head_pos.y -= 1;
            }
        };
        if head_pos.x < 0 || head_pos.y < 0 || head_pos.x as u32 >= cons::ARENA_WIDTH || head_pos.y as u32 >= cons::ARENA_HEIGHT {
            game_over_writer.send(GameOverEvent);
        }
        if segment_positions.contains(&head_pos) {
            game_over_writer.send(GameOverEvent);
        }
        segment_positions
            .iter()
            .zip(segments.iter().skip(1))
            .for_each(|(pos, segment)| { *positions.get_mut(*segment).unwrap() = *pos; });
        *last_tail_position = LastTailPosition(Some(*segment_positions.last().unwrap()));
    }
}

pub fn game_over(mut commands: Commands, 
                 mut reader: EventReader<GameOverEvent>, 
                 segments_res: ResMut<SnakeSegments>, 
                 food: Query<Entity, With<Food>>, 
                 segments: Query<Entity, With<SnakeSegment>>) {
    if reader.iter().next().is_some() {
        for ent in food.iter().chain(segments.iter()) {
            commands.entity(ent).despawn();
        }
        spawn_snake(commands, segments_res);
    }
}

pub fn snake_growth(commands: Commands, last_tail_position: Res<LastTailPosition>, mut segments: ResMut<SnakeSegments>, mut growth_reader: EventReader<GrowthEvent>) {
    if growth_reader.iter().next().is_some() {
        segments.push(spawn_segment(commands, last_tail_position.0.unwrap()));
    }
}

pub fn snake_eating(mut commands: Commands, 
                    mut growth_writer: EventWriter<GrowthEvent>, 
                    food_positions: Query<(Entity, &Position), With<Food>>, 
                    head_positions: Query<&Position, With<SnakeHead>>) {
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if food_pos == head_pos {
                commands.entity(ent).despawn();
                growth_writer.send(GrowthEvent);
            }
        }
    }
}

pub fn spawn_segment(mut commands: Commands, position: Position) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: cons::SNAKE_SEGMENT_COLOR,
                ..default()
            },
            ..default() })
        .insert(SnakeSegment)
        .insert(position)
        .insert(Size::square(0.65))
        .id()
}

pub fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / cons::ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / cons::ARENA_HEIGHT as f32 * window.height() as f32,
            1.0, ); }
}

pub fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(
                pos.x as f32,
                window.width() as f32,
                cons::ARENA_WIDTH as f32, ),
            convert(
                pos.y as f32,
                window.height() as f32,
                cons::ARENA_HEIGHT as f32, ),
            0.0, );
    }
}

pub fn food_spawner(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: cons::FOOD_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Food)
        .insert(Position {
            x: (random::<f32>() * cons::ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * cons::ARENA_HEIGHT as f32) as i32,
        })
        .insert(Size::square(0.8));
}
